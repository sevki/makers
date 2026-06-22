//! Pattern (implicit) rule database: the global list of `%`-pattern rules,
//! conversion of old-style suffix rules into pattern rules, and the
//! `print_rule_data_base` report.
//!
//! Port of `rule.c`.

use std::sync::atomic::{AtomicU32, Ordering};

pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{Commands, Dep, File};
use crate::misc::free_ns_chain;
use crate::misc::{copy_dep_chain, xcalloc, xmalloc, xstrdup};
use crate::stdio::FILE;
use crate::strcache::strcache_add_len;
use libc::{abort, free, memcpy, printf, putchar, puts, strchr, strlen, strrchr};
extern "C" {
    static mut stdout: *mut FILE;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> i32;
}
pub type dep = Dep;
pub type commands = Commands;
use crate::floc::Floc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub next: *mut rule,
    pub targets: *mut *const ::core::ffi::c_char,
    pub lens: *mut ::core::ffi::c_uint,
    pub suffixes: *mut *const ::core::ffi::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub _defn: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_ushort,
    pub terminal: ::core::ffi::c_char,
    pub in_use: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspec {
    pub target: *const ::core::ffi::c_char,
    pub dep: *const ::core::ffi::c_char,
    pub commands: *const ::core::ffi::c_char,
}
use crate::commands::print_commands;
use crate::dir::dir_file_exists_p;
pub use crate::file::nameseq;
use crate::file::{expand_extra_prereqs, lookup_file};
use crate::make_main::{posix_pedantic, second_expansion};
use crate::output::fatal;
use crate::read::{find_percent_cached, parse_file_seq};
use crate::variable::lookup_variable;
pub const MAP_NUL: i32 = 0x1;
pub const INTSTR_LENGTH: usize = 53 * ::core::mem::size_of::<uintmax_t>() / 22 + 3;
pub const RECIPEPREFIX_DEFAULT: i32 = '\t' as i32;
pub const PARSEFS_NONE: i32 = 0;
#[inline]
unsafe fn alloc_dep() -> *mut dep {
    xcalloc(::core::mem::size_of::<dep>() as size_t) as *mut dep
}
#[inline]
unsafe fn free_dep_chain(d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
/// The name a dep goes by: its own `name` if set, otherwise its file's name.
/// Mirrors the C `dep_name` macro.
unsafe fn dep_name(d: *const dep) -> *const ::core::ffi::c_char {
    let d = d.as_ref().expect("dep_name requires a non-null dep");
    if !d.name.is_null() {
        d.name
    } else {
        d.file
            .as_ref()
            .expect("dep without a name must have a file")
            .name
    }
}
/// Byte-for-byte equality of two NUL-terminated strings (the C `streq` macro).
fn streq(a: &::core::ffi::CStr, b: &::core::ffi::CStr) -> bool {
    a == b
}
/// Append the NUL-terminated string `s` to `buf` (without the NUL).
unsafe fn push_cstr(buf: &mut Vec<u8>, s: *const ::core::ffi::c_char) {
    buf.extend_from_slice(::core::slice::from_raw_parts(s.cast::<u8>(), strlen(s)));
}
pub static mut pattern_rules: *mut rule = ::core::ptr::null_mut();
pub static mut last_pattern_rule: *mut rule = ::core::ptr::null_mut();
/// Pattern-rule limits recomputed by `count_implicit_rule_limits` and read by
/// `pattern_search` to size its scratch allocations. Atomic so the reads/writes
/// are plain safe ops; the rule database is built and searched single-threaded,
/// so `Relaxed` preserves the original program order.
pub static NUM_PATTERN_RULES: AtomicU32 = AtomicU32::new(0);
pub static MAX_PATTERN_TARGETS: AtomicU32 = AtomicU32::new(0);
pub static MAX_PATTERN_DEPS: AtomicU32 = AtomicU32::new(0);
pub static mut max_pattern_dep_length: size_t = 0;
pub static mut suffix_file: *mut File = ::core::ptr::null_mut();
/// Return (computing and caching it on first use) the printable definition of
/// rule `r`, e.g. `%.o: %.c`.
///
/// # Safety
/// `r` must point to a valid rule whose targets, lens, and dep chain are
/// valid; must run single-threaded (mutates the rule's `_defn` cache).
pub unsafe fn get_rule_defn(r: *mut rule) -> *const ::core::ffi::c_char {
    let r = r.as_mut().expect("get_rule_defn requires a non-null rule");
    if r._defn.is_null() {
        let mut buf: Vec<u8> = Vec::new();
        for k in 0..r.num as usize {
            if k > 0 {
                buf.push(b' ');
            }
            let target = r.targets.add(k).as_ref().expect("rule target slot");
            let len = r.lens.add(k).as_ref().expect("rule length slot");
            buf.extend_from_slice(::core::slice::from_raw_parts(
                (*target).cast::<u8>(),
                *len as usize,
            ));
        }
        buf.push(b':');
        if r.terminal != 0 {
            buf.push(b':');
        }
        // Normal prerequisites first; remember where the order-only ones
        // start so they can be printed after a `|`.
        let mut ood: *const dep = ::core::ptr::null();
        let mut d: *const dep = r.deps;
        while let Some(dep) = d.as_ref() {
            if dep.ignore_mtime() == 0 {
                if dep.wait_here() != 0 {
                    buf.extend_from_slice(b" .WAIT");
                }
                buf.push(b' ');
                push_cstr(&mut buf, dep_name(d));
            } else if ood.is_null() {
                ood = d;
            }
            d = dep.next;
        }
        let mut sep: &[u8] = b" | ";
        while let Some(dep) = ood.as_ref() {
            if dep.ignore_mtime() != 0 {
                buf.extend_from_slice(sep);
                if dep.wait_here() != 0 {
                    buf.extend_from_slice(b".WAIT ");
                }
                push_cstr(&mut buf, dep_name(ood));
            }
            ood = dep.next;
            sep = b" ";
        }
        buf.push(0);
        // The cache is released with free() in freerule, so it must live in
        // a malloc'd buffer rather than the Vec.
        let defn = xmalloc(buf.len() as size_t) as *mut ::core::ffi::c_char;
        memcpy(defn.cast(), buf.as_ptr().cast(), buf.len());
        r._defn = defn;
    }
    r._defn
}
/// Snap the implicit-rule database after reading all makefiles: count rules,
/// compute the various `max_pattern_*` statistics, mark deps whose directory
/// does not exist, and append `.EXTRA_PREREQS` to every rule's dep chain.
///
/// # Safety
/// The global pattern-rule list and all linked structures must be valid;
/// must run single-threaded (mutates rule-database globals).
pub unsafe fn snap_implicit_rules(ctx: &crate::execctx::ExecContext) {
    let mut dirname: Vec<u8> = Vec::new();
    let prereqs: *mut dep = expand_extra_prereqs(
        ctx,
        lookup_variable(
            ctx,
            c".EXTRA_PREREQS".as_ptr(),
            ".EXTRA_PREREQS".len() as size_t,
        ),
    );
    let mut pre_deps: ::core::ffi::c_uint = 0;
    max_pattern_dep_length = 0;
    let mut d: *mut dep = prereqs;
    while let Some(dr) = d.as_mut() {
        let mut name: *const ::core::ffi::c_char = dep_name(d);
        let mut len: size_t = strlen(name);
        if second_expansion() {
            if dr.name.is_null() {
                dr.name = xstrdup(
                    dr.file
                        .as_ref()
                        .expect("dep without a name must have a file")
                        .name,
                );
            }
            dr.set_need_2nd_expansion(1);
        }
        if dr.need_2nd_expansion() != 0 {
            // Each '%' in the name may expand to "\%\%" later; budget for it.
            loop {
                name = strchr(name, '%' as i32);
                if name.is_null() {
                    break;
                }
                len = len.wrapping_add(4);
                name = name.add(1);
            }
        }
        if len > max_pattern_dep_length {
            max_pattern_dep_length = len;
        }
        pre_deps = pre_deps.wrapping_add(1);
        d = dr.next;
    }
    NUM_PATTERN_RULES.store(0, Ordering::Relaxed);
    MAX_PATTERN_TARGETS.store(0, Ordering::Relaxed);
    MAX_PATTERN_DEPS.store(0, Ordering::Relaxed);
    let mut rule: *mut rule = pattern_rules;
    while let Some(rr) = rule.as_mut() {
        let mut ndeps: ::core::ffi::c_uint = pre_deps;
        let mut lastdep: *mut dep = ::core::ptr::null_mut();
        NUM_PATTERN_RULES.fetch_add(1, Ordering::Relaxed);
        if rr.num as ::core::ffi::c_uint > MAX_PATTERN_TARGETS.load(Ordering::Relaxed) {
            MAX_PATTERN_TARGETS.store(rr.num as ::core::ffi::c_uint, Ordering::Relaxed);
        }
        d = rr.deps;
        while let Some(dr) = d.as_mut() {
            let dname: *const ::core::ffi::c_char = dep_name(d);
            let len: size_t = strlen(dname);
            let mut p: *const ::core::ffi::c_char = strrchr(dname, '/' as i32);
            let p2: *const ::core::ffi::c_char = if !p.is_null() {
                strchr(p, '%' as i32)
            } else {
                ::core::ptr::null()
            };
            ndeps = ndeps.wrapping_add(1);
            if len > max_pattern_dep_length {
                max_pattern_dep_length = len;
            }
            if dr.next.is_null() {
                lastdep = d;
            }
            if !p2.is_null() {
                // The directory part contains '%': check whether the
                // directory prefix exists and mark the dep "changed" if not.
                if p == dname {
                    p = p.add(1);
                }
                let dirlen = p.offset_from(dname) as usize;
                dirname.clear();
                dirname
                    .extend_from_slice(::core::slice::from_raw_parts(dname.cast::<u8>(), dirlen));
                dirname.push(0);
                dr.set_changed(
                    (dir_file_exists_p(ctx, dirname.as_ptr().cast(), c"".as_ptr()) == 0)
                        as ::core::ffi::c_uint,
                );
            } else {
                dr.set_changed(0);
            }
            d = dr.next;
        }
        if !prereqs.is_null() {
            if let Some(ld) = lastdep.as_mut() {
                ld.next = copy_dep_chain(prereqs);
            } else {
                rr.deps = copy_dep_chain(prereqs);
            }
        }
        if ndeps > MAX_PATTERN_DEPS.load(Ordering::Relaxed) {
            MAX_PATTERN_DEPS.store(ndeps, Ordering::Relaxed);
        }
        rule = rr.next;
    }
    free_dep_chain(prereqs);
}
/// Build a NUL-terminated `%`-prefixed copy of the NUL-terminated string `s`
/// (e.g. `.c` becomes `%.c`).
unsafe fn percent_prefixed(s: *const ::core::ffi::c_char) -> Vec<u8> {
    let len = strlen(s);
    let mut buf = Vec::with_capacity(len + 2);
    buf.push(b'%');
    // Copy the NUL along with the bytes.
    buf.extend_from_slice(::core::slice::from_raw_parts(s.cast::<u8>(), len + 1));
    buf
}
unsafe fn convert_suffix_rule(
    target: *const ::core::ffi::c_char,
    source: *const ::core::ffi::c_char,
    cmds: *mut commands,
) {
    let names: *mut *const ::core::ffi::c_char =
        xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
            as *mut *const ::core::ffi::c_char;
    let percents: *mut *const ::core::ffi::c_char =
        xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
            as *mut *const ::core::ffi::c_char;
    let name_slot = names.as_mut().expect("xmalloc returned null");
    let percent_slot = percents.as_mut().expect("xmalloc returned null");
    if target.is_null() {
        // Special case: creating "(%.o)" from an archive-member suffix rule.
        *name_slot = strcache_add_len(c"(%.o)".as_ptr(), 5);
        *percent_slot = (*name_slot).add(1);
    } else {
        let pattern = percent_prefixed(target);
        *name_slot = strcache_add_len(pattern.as_ptr().cast(), (pattern.len() - 1) as size_t);
        *percent_slot = *name_slot;
    }
    let deps: *mut dep = if source.is_null() {
        ::core::ptr::null_mut()
    } else {
        let pattern = percent_prefixed(source);
        let d = alloc_dep();
        d.as_mut().expect("xcalloc returned null").name =
            strcache_add_len(pattern.as_ptr().cast(), (pattern.len() - 1) as size_t);
        d
    };
    create_pattern_rule(names, percents, 1, 0, deps, cmds, 0);
}
/// Decide whether a looked-up suffix-rule file (which has commands) should be
/// treated as a suffix rule, given whether it carries prerequisites. With no
/// prerequisites it always applies; with prerequisites it is skipped under
/// `--posix`, else a warning is issued (located at `fileinfo`) and it still
/// applies. Shared by the single-suffix and suffix-pair arms below.
fn suffix_rule_applies(
    ctx: &crate::execctx::ExecContext,
    fileinfo: &Floc,
    has_prereqs: bool,
) -> bool {
    if !has_prereqs {
        return true;
    }
    if posix_pedantic() {
        return false;
    }
    // The safe native error path (no variadic C-ABI, no `unsafe`); byte-for-byte
    // identical output to the old `error(...)` since `build_prefix` reproduces
    // the same `file:line:` / `prog[level]:` prefix.
    crate::error!(
        ctx,
        Some(fileinfo),
        "warning: ignoring prerequisites on suffix rule definition"
    );
    true
}

/// Convert old-style suffix rules (the prerequisites of `.SUFFIXES`) into
/// pattern rules.
///
/// # Safety
/// `suffix_file` and all linked file/dep structures must be valid; must run
/// single-threaded (mutates the rule database).
pub unsafe fn convert_to_pattern(ctx: &crate::execctx::ExecContext) {
    let suffixes: *mut dep = suffix_file
        .as_ref()
        .expect("the .SUFFIXES file must exist before convert_to_pattern")
        .deps;
    let mut maxsuffix: size_t = 0;
    let mut d: *mut dep = suffixes;
    while let Some(dr) = d.as_ref() {
        let len = strlen(dep_name(d));
        if len > maxsuffix {
            maxsuffix = len;
        }
        d = dr.next;
    }
    // Scratch buffer for a concatenated ".tgt.src" suffix-rule name.
    let mut rulename: Vec<u8> = vec![0; maxsuffix * 2 + 1];
    let rulename: *mut ::core::ffi::c_char = rulename.as_mut_ptr().cast();
    d = suffixes;
    while let Some(dr) = d.as_ref() {
        // A suffix by itself (".c") describes a rule making "%" from "%.c".
        convert_suffix_rule(dep_name(d), ::core::ptr::null(), ::core::ptr::null_mut());
        let dep_file = dr.file.as_ref().expect("suffix dep must have a file");
        if !dep_file.cmds.is_null() {
            // The suffix's own commands make "%" from "%.<suffix>".
            convert_suffix_rule(c"".as_ptr(), dep_name(d), dep_file.cmds);
        }
        let slen = strlen(dep_name(d));
        memcpy(rulename.cast(), dep_name(d).cast(), slen + 1);
        if let Some(f) = lookup_file(rulename).as_mut() {
            let has_prereqs = !f.deps.is_null();
            if let Some(cmds) = f.cmds.as_mut() {
                if suffix_rule_applies(ctx, &cmds.fileinfo, has_prereqs) {
                    f.set_suffix(1);
                }
            }
        }
        let mut d2: *mut dep = suffixes;
        while let Some(d2r) = d2.as_ref() {
            let s2len = strlen(dep_name(d2));
            // Skip the pairing of a suffix with itself.
            if !(slen == s2len
                && streq(
                    ::core::ffi::CStr::from_ptr(dep_name(d)),
                    ::core::ffi::CStr::from_ptr(dep_name(d2)),
                ))
            {
                memcpy(rulename.add(slen).cast(), dep_name(d2).cast(), s2len + 1);
                if let Some(f) = lookup_file(rulename).as_mut() {
                    let has_prereqs = !f.deps.is_null();
                    if let Some(cmds) = f.cmds.as_mut() {
                        // Under --posix, prerequisites on a suffix rule are silently
                        // ignored (skip); otherwise warn and still convert the rule.
                        if suffix_rule_applies(ctx, &cmds.fileinfo, has_prereqs) {
                            f.set_suffix(1);
                            if s2len == 2
                                && *rulename.add(slen) as u8 == b'.'
                                && *rulename.add(slen + 1) as u8 == b'a'
                            {
                                // ".X.a" also describes "(%.o): %.X".
                                convert_suffix_rule(::core::ptr::null(), dep_name(d), f.cmds);
                            }
                            convert_suffix_rule(dep_name(d2), dep_name(d), f.cmds);
                        }
                    }
                }
            }
            d2 = d2r.next;
        }
        d = dr.next;
    }
}
/// Install `rule` into the pattern-rule database, replacing any rule with
/// identical targets and deps when `override_0` is set. Returns 1 if the rule
/// was installed, 0 if it was discarded as a non-overriding duplicate.
unsafe fn new_pattern_rule(rule: *mut rule, override_0: i32) -> i32 {
    let new_rule = rule.as_mut().expect("new_pattern_rule requires a rule");
    new_rule.in_use = 0;
    new_rule.terminal = 0;
    new_rule.next = ::core::ptr::null_mut();
    let mut lastrule: *mut rule = ::core::ptr::null_mut();
    let mut r: *mut rule = pattern_rules;
    'rules: while let Some(rr) = r.as_ref() {
        for i in 0..new_rule.num as usize {
            let target_i = *new_rule.targets.add(i).as_ref().expect("rule target slot");
            let mut j = 0usize;
            while j < rr.num as usize {
                let target_j = *rr.targets.add(j).as_ref().expect("rule target slot");
                if !streq(
                    ::core::ffi::CStr::from_ptr(target_i),
                    ::core::ffi::CStr::from_ptr(target_j),
                ) {
                    break;
                }
                j += 1;
            }
            if j == rr.num as usize {
                // All targets matched; compare the dep chains too.
                let mut d: *mut dep = new_rule.deps;
                let mut d2: *mut dep = rr.deps;
                while let (Some(dr), Some(d2r)) = (d.as_ref(), d2.as_ref()) {
                    if !streq(
                        ::core::ffi::CStr::from_ptr(dep_name(d)),
                        ::core::ffi::CStr::from_ptr(dep_name(d2)),
                    ) {
                        break;
                    }
                    d = dr.next;
                    d2 = d2r.next;
                }
                if d.is_null() && d2.is_null() {
                    if override_0 != 0 {
                        freerule(r, lastrule);
                        append_to_pattern_rules(rule);
                        break 'rules;
                    } else {
                        freerule(rule, ::core::ptr::null_mut());
                        return 0;
                    }
                }
            }
        }
        lastrule = r;
        r = rr.next;
    }
    if r.is_null() {
        append_to_pattern_rules(rule);
    }
    1
}
/// Append `rule` (already detached, with a null `next`) to the global
/// pattern-rule list.
unsafe fn append_to_pattern_rules(rule: *mut rule) {
    if pattern_rules.is_null() {
        pattern_rules = rule;
    } else {
        last_pattern_rule
            .as_mut()
            .expect("a non-empty rule list must have a tail")
            .next = rule;
    }
    last_pattern_rule = rule;
}
/// Install an implicit pattern rule from a `pspec`.
///
/// # Safety
/// `p` must point to a valid `pspec` whose strings are NUL-terminated and
/// live for the program's lifetime; must run single-threaded.
pub unsafe fn install_pattern_rule(
    ctx: &crate::execctx::ExecContext,
    p: *const pspec,
    terminal: i32,
) {
    let spec = p.as_ref().expect("install_pattern_rule requires a pspec");
    let r: *mut rule = xmalloc(::core::mem::size_of::<rule>() as size_t) as *mut rule;
    let rr = r.as_mut().expect("xmalloc returned null");
    rr.num = 1;
    rr.targets = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    rr.suffixes = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    rr.lens = xmalloc(::core::mem::size_of::<::core::ffi::c_uint>() as size_t)
        as *mut ::core::ffi::c_uint;
    rr._defn = ::core::ptr::null_mut();
    *rr.lens.as_mut().expect("xmalloc returned null") = strlen(spec.target) as ::core::ffi::c_uint;
    *rr.targets.as_mut().expect("xmalloc returned null") = spec.target;
    let suffix_slot = rr.suffixes.as_mut().expect("xmalloc returned null");
    *suffix_slot = find_percent_cached(rr.targets);
    assert!(
        !suffix_slot.is_null(),
        "pattern rule target must contain a '%'"
    );
    // Point past the '%' itself.
    *suffix_slot = suffix_slot.add(1);
    let mut ptr: *const ::core::ffi::c_char = spec.dep;
    rr.deps = parse_file_seq(
        ctx,
        &raw mut ptr as *mut *mut ::core::ffi::c_char,
        ::core::mem::size_of::<dep>() as size_t,
        MAP_NUL,
        ::core::ptr::null(),
        PARSEFS_NONE,
    ) as *mut dep;
    if new_pattern_rule(r, 0) != 0 {
        rr.terminal = (terminal != 0) as ::core::ffi::c_char;
        rr.cmds = xmalloc(::core::mem::size_of::<commands>() as size_t) as *mut commands;
        let cmds = rr.cmds.as_mut().expect("xmalloc returned null");
        cmds.fileinfo.filenm = ::core::ptr::null();
        cmds.fileinfo.lineno = 0;
        cmds.fileinfo.offset = 0;
        cmds.commands = xstrdup(spec.commands);
        cmds.command_lines = ::core::ptr::null_mut();
        cmds.recipe_prefix = RECIPEPREFIX_DEFAULT as ::core::ffi::c_char;
    }
}
/// Free `rule` and splice it out of the pattern-rule list; `lastrule` is the
/// node before it (or null if `rule` heads the list).
///
/// # Safety
/// `rule` must be a malloc-allocated rule on the global list and `lastrule`
/// its actual predecessor; must run single-threaded.
pub unsafe fn freerule(rule: *mut rule, lastrule: *mut rule) {
    let dead = rule.as_ref().expect("freerule requires a rule");
    let next: *mut rule = dead.next;
    free_dep_chain(dead.deps);
    free(dead.targets as *mut ::core::ffi::c_void);
    free(dead.suffixes as *mut ::core::ffi::c_void);
    free(dead.lens as *mut ::core::ffi::c_void);
    free(dead._defn as *mut ::core::ffi::c_void);
    free(rule as *mut ::core::ffi::c_void);
    if pattern_rules == rule {
        if !lastrule.is_null() {
            abort();
        } else {
            pattern_rules = next;
        }
    } else if let Some(last) = lastrule.as_mut() {
        last.next = next;
    }
    if last_pattern_rule == rule {
        last_pattern_rule = lastrule;
    }
}
/// Create a new pattern rule with `n` targets and install it.
///
/// # Safety
/// `targets`, `target_percents`, `deps`, and `commands` must be valid for
/// `n` entries and ownership transfers to the rule database; must run
/// single-threaded.
pub unsafe fn create_pattern_rule(
    targets: *mut *const ::core::ffi::c_char,
    target_percents: *mut *const ::core::ffi::c_char,
    n: ::core::ffi::c_ushort,
    terminal: i32,
    deps: *mut dep,
    commands: *mut commands,
    override_0: i32,
) {
    let r: *mut rule = xmalloc(::core::mem::size_of::<rule>() as size_t) as *mut rule;
    let rr = r.as_mut().expect("xmalloc returned null");
    rr.num = n;
    rr.cmds = commands;
    rr.deps = deps;
    rr.targets = targets;
    rr.suffixes = target_percents;
    rr.lens = xmalloc(
        (n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    rr._defn = ::core::ptr::null_mut();
    for i in 0..n as usize {
        let target = *targets.add(i).as_ref().expect("rule target slot");
        *rr.lens.add(i).as_mut().expect("rule length slot") = strlen(target) as ::core::ffi::c_uint;
        let suffix = rr.suffixes.add(i).as_mut().expect("rule suffix slot");
        assert!(!suffix.is_null(), "pattern rule target must contain a '%'");
        // Point past the '%' itself.
        *suffix = suffix.add(1);
    }
    if new_pattern_rule(r, override_0) != 0 {
        rr.terminal = (terminal != 0) as ::core::ffi::c_char;
    }
}
/// Print rule `r`'s definition and commands to stdout (for `-p`).
///
/// # Safety
/// `r` must point to a valid rule; must run single-threaded.
pub unsafe fn print_rule(r: *mut rule) {
    fputs(get_rule_defn(r), stdout);
    putchar('\n' as i32);
    let r = r.as_ref().expect("print_rule requires a rule");
    if !r.cmds.is_null() {
        print_commands(r.cmds);
    }
}
/// Print the whole implicit-rule database to stdout (for `-p`).
///
/// # Safety
/// The global pattern-rule list must be valid; must run single-threaded.
pub unsafe fn print_rule_data_base(ctx: &crate::execctx::ExecContext) {
    let mut rules: ::core::ffi::c_uint = 0;
    let mut terminal: ::core::ffi::c_uint = 0;
    puts(c"\n# Implicit Rules".as_ptr());
    let mut r: *mut rule = pattern_rules;
    while let Some(rr) = r.as_ref() {
        rules = rules.wrapping_add(1);
        putchar('\n' as i32);
        print_rule(r);
        if rr.terminal != 0 {
            terminal = terminal.wrapping_add(1);
        }
        r = rr.next;
    }
    if rules == 0 {
        puts(c"\n# No implicit rules.".as_ptr());
    } else {
        printf(
            c"\n# %u implicit rules, %u (%.1f%%) terminal.".as_ptr(),
            rules,
            terminal,
            terminal as ::core::ffi::c_double / rules as ::core::ffi::c_double * 100.0f64,
        );
    }
    let num_pattern_rules = NUM_PATTERN_RULES.load(Ordering::Relaxed);
    if num_pattern_rules != rules && num_pattern_rules != 0 {
        fatal(
            ctx,
            ::core::ptr::null_mut::<Floc>(),
            INTSTR_LENGTH.wrapping_mul(2) as size_t,
            c"INTERNAL: num_pattern_rules is wrong!  %u != %u".as_ptr(),
            num_pattern_rules,
            rules,
        );
    }
}

#[cfg(test)]
mod streq_tests {
    use super::streq;

    #[test]
    fn equal_and_unequal_strings() {
        assert!(streq(c"", c""));
        assert!(streq(c"%.o", c"%.o"));
        assert!(streq(c"target%pattern", c"target%pattern"));

        assert!(!streq(c"", c"x"));
        assert!(!streq(c"x", c""));
        assert!(!streq(c"%.o", c"%.c"));
        // Equal prefix, differing length.
        assert!(!streq(c"abc", c"abcd"));
        // Differing first byte (the C macro's fast path).
        assert!(!streq(c"a", c"b"));
    }
}
