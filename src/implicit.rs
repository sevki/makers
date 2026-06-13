//! Implicit (pattern) rule search: `pattern_search` walks the pattern-rule
//! database looking for a rule — or a chain of rules through intermediate
//! files — that can build a given target.
//!
//! Port of `implicit.c`.

pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{Dep, File};
use crate::misc::free_ns_chain;
use crate::misc::{lindex, print_spaces, skip_reference, xcalloc, xmalloc, xrealloc};
use crate::stdio::FILE;
use crate::strcache::{strcache_add, strcache_add_len};
use c2rust_bitfields;
use libc::{free, memcpy, memrchr, memset, printf, qsort, strchr, strcmp, strcpy, strlen, strncmp};
extern "C" {
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type file = File;
pub type dep = Dep;
use crate::ar::ar_name;
use crate::commands::set_file_variables;
use crate::dir::{file_exists_p, file_impossible, file_impossible_p};
use crate::expand::expand_string_for_file;
pub use crate::file::nameseq;
use crate::file::{enter_file, lookup_file};
use crate::make_main::{db_level, no_intermediates, stopchar_map};
use crate::read::parse_file_seq;
pub use crate::rule::rule;
use crate::rule::{
    get_rule_defn, max_pattern_dep_length, max_pattern_deps, max_pattern_targets,
    num_pattern_rules, pattern_rules,
};
use crate::variable::o_automatic;
use crate::variable::{
    define_variable_in_set, free_variable_set, initialize_file_variables, merge_variable_set_lists,
};
use crate::vpath::vpath_search;

/// `DB_IMPLICIT`: `-d` implicit-rule tracing enabled in `db_level`.
const DB_IMPLICIT: ::core::ffi::c_int = 0x8;
/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_NUL: ::core::ffi::c_int = 0x0001;
const MAP_BLANK: ::core::ffi::c_int = 0x0002;
const MAP_NEWLINE: ::core::ffi::c_int = 0x0004;
const MAP_PIPE: ::core::ffi::c_int = 0x0100;
/// `parse_file_seq` flags (see `dep.h`).
const PARSEFS_ONEWORD: ::core::ffi::c_int = 0x20;
const PARSEFS_WAIT: ::core::ffi::c_int = 0x40;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
unsafe fn stop_set(c: ::core::ffi::c_char, mask: ::core::ffi::c_int) -> bool {
    stopchar_map[c as u8 as usize] as ::core::ffi::c_int & mask != 0
}

/// `DBS (DB_IMPLICIT, ...)` from the C original: print an indented trace
/// line when implicit-rule debugging is enabled.
macro_rules! dbs {
    ($depth:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        if DB_IMPLICIT & db_level != 0 {
            print_spaces($depth);
            printf($fmt $(, $arg)*);
            fflush(stdout);
        }
    };
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

/// String equality via the C `streq` macro's shape: compare the first bytes,
/// then fall back to `strcmp` on the remainder.
unsafe fn streq(a: *const ::core::ffi::c_char, b: *const ::core::ffi::c_char) -> bool {
    let a0 = *a.as_ref().expect("streq requires non-null strings");
    let b0 = *b.as_ref().expect("streq requires non-null strings");
    a0 == b0 && (a0 == 0 || strcmp(a.add(1), b.add(1)) == 0)
}

/// A prerequisite discovered while trying a pattern rule, together with the
/// intermediate file that would build it (if any).
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct patdeps {
    pub name: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
    pub file: *mut file,
    #[bitfield(name = "ignore_mtime", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(
        name = "ignore_automatic_vars",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "wait_here", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub ignore_mtime_ignore_automatic_vars_is_explicit_wait_here: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
/// A candidate pattern rule recorded during the first matching pass.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tryrule {
    pub rule: *mut rule,
    pub stemlen: size_t,
    pub matches: ::core::ffi::c_uint,
    pub order: ::core::ffi::c_uint,
    pub checked_lastslash: ::core::ffi::c_char,
}
pub const PATH_MAX: ::core::ffi::c_int = 4096;
pub const GET_PATH_MAX: ::core::ffi::c_int = PATH_MAX;
/// # Safety
///
/// Must run single-threaded; returns a zeroed malloc'd dep owned by the
/// caller.
pub unsafe fn alloc_dep() -> *mut dep {
    xcalloc(::core::mem::size_of::<dep>() as size_t) as *mut dep
}
#[inline]
unsafe fn free_dep_chain(d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
/// Search the implicit-rule database for a rule that can build `file`,
/// retrying as an archive-member reference when the plain search fails.
/// Returns 1 when a rule was found and applied to `file`.
///
/// # Safety
/// `file` must point to a valid file entry; the rule database and all linked
/// structures must be valid; must run single-threaded.
pub unsafe fn try_implicit_rule(file: *mut file, depth: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    dbs!(
        depth,
        c"Looking for an implicit rule for '%s'.\n".as_ptr(),
        (*file).name
    );
    if pattern_search(file, 0, depth, 0, 0) != 0 {
        return 1;
    }
    if ar_name((*file).name) != 0 {
        dbs!(
            depth,
            c"Looking for archive-member implicit rule for '%s'.\n".as_ptr(),
            (*file).name
        );
        if pattern_search(file, 1, depth, 0, 0) != 0 {
            return 1;
        }
        dbs!(
            depth,
            c"No archive-member implicit rule found for '%s'.\n".as_ptr(),
            (*file).name
        );
    }
    0
}
/// Scan past leading blanks to the next word of `buffer`, stopping at an
/// unquoted blank, `|`, or NUL (skipping over `$(...)` references). Returns
/// the word's start (storing its length through `length`), or null at EOL.
unsafe fn get_next_word(
    buffer: *const ::core::ffi::c_char,
    length: *mut size_t,
) -> *const ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = buffer;
    while stop_set(*p, MAP_BLANK | MAP_NEWLINE) {
        p = p.add(1);
    }
    let beg: *const ::core::ffi::c_char = p;
    let mut c: ::core::ffi::c_char = *p;
    p = p.add(1);
    if c == 0 {
        return ::core::ptr::null();
    }
    loop {
        match c as u8 {
            0 | b' ' | b'\t' => {
                // Back up over the terminating whitespace/NUL.
                p = p.sub(1);
                break;
            }
            b'$' => {
                p = skip_reference(p);
            }
            b'|' => {
                break;
            }
            _ => {}
        }
        c = *p;
        p = p.add(1);
    }
    if let Some(len) = length.as_mut() {
        *len = p.offset_from(beg) as size_t;
    }
    beg
}
/// qsort comparator ordering candidate rules by stem length, then by the
/// order they appear in the database.
///
/// # Safety
/// Both arguments must point to valid `tryrule`s (guaranteed by qsort).
pub unsafe extern "C" fn stemlen_compare(
    v1: *const ::core::ffi::c_void,
    v2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let r1 = (v1 as *const tryrule)
        .as_ref()
        .expect("qsort passes valid elements");
    let r2 = (v2 as *const tryrule)
        .as_ref()
        .expect("qsort passes valid elements");
    let r = r1.stemlen.wrapping_sub(r2.stemlen) as ::core::ffi::c_int;
    if r != 0 {
        r
    } else {
        r1.order.wrapping_sub(r2.order) as ::core::ffi::c_int
    }
}
unsafe fn pattern_search(
    file: *mut file,
    archive: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_uint,
    recursions: ::core::ffi::c_uint,
    allow_compat_rules: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    // The target name (inside the parens for an archive member reference).
    let filename: *const ::core::ffi::c_char = if archive != 0 {
        strchr((*file).name, '(' as i32)
    } else {
        (*file).name
    };
    let namelen: size_t = strlen(filename);
    let mut int_file: *mut file = ::core::ptr::null_mut();
    // Backing storage for the "intermediate file" scratch entries; entries
    // may be linked into the patdeps list, so they must live until return.
    let mut int_file_storage: Vec<Box<file>> = Vec::new();
    let mut max_deps: ::core::ffi::c_uint = max_pattern_deps;
    let mut deplist: *mut patdeps =
        xmalloc((max_deps as size_t).wrapping_mul(::core::mem::size_of::<patdeps>() as size_t))
            as *mut patdeps;
    let mut pat: *mut patdeps = deplist;
    // Buffer big enough for any rule prerequisite with the stem substituted.
    let deplen: size_t = namelen.wrapping_add(max_pattern_dep_length).wrapping_add(4);
    let mut depname_buf: Vec<u8> = vec![0; deplen];
    let depname: *mut ::core::ffi::c_char = depname_buf.as_mut_ptr().cast();
    let dend: *mut ::core::ffi::c_char = depname.add(deplen);
    let mut stem: *const ::core::ffi::c_char = ::core::ptr::null();
    let mut stemlen: size_t = 0;
    let fullstemlen: size_t;
    let tryrules: *mut tryrule = xmalloc(
        (num_pattern_rules.wrapping_mul(max_pattern_targets) as size_t)
            .wrapping_mul(::core::mem::size_of::<tryrule>() as size_t),
    ) as *mut tryrule;
    let mut nrules: ::core::ffi::c_uint = 0;
    let foundrule: ::core::ffi::c_uint;
    let mut file_vars_initialized: ::core::ffi::c_int = 0;
    let mut specific_rule_matched: ::core::ffi::c_int = 0;
    let mut ri: ::core::ffi::c_uint = 0;
    let mut found_compat_rule: ::core::ffi::c_int = 0;
    let mut rule: *mut rule;
    let mut pathdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
    let mut pathdir_buf: Vec<u8> = Vec::new();
    let mut stem_str: [::core::ffi::c_char; (PATH_MAX + 1) as usize] = [0; (PATH_MAX + 1) as usize];
    let stem_str_ptr: *mut ::core::ffi::c_char = stem_str.as_mut_ptr();
    depth = depth.wrapping_add(1);
    // An archive member name has no directory part.
    let lastslash: *const ::core::ffi::c_char = if archive != 0 || ar_name(filename) != 0 {
        ::core::ptr::null()
    } else {
        memrchr(filename.cast(), '/' as i32, namelen.wrapping_sub(1)).cast()
    };
    let pathlen: size_t = if !lastslash.is_null() {
        (lastslash.offset_from(filename) + 1) as size_t
    } else {
        0
    };
    // First pass: collect every pattern rule whose target matches the name.
    rule = pattern_rules;
    while let Some(r) = rule.as_ref() {
        // A rule with prerequisites but no commands cannot be used directly.
        if !(!r.deps.is_null() && r.cmds.is_null()) {
            if r.in_use != 0 {
                dbs!(
                    depth,
                    c"Avoiding implicit rule recursion for rule '%s'.\n".as_ptr(),
                    get_rule_defn(rule)
                );
            } else {
                for ti in 0..r.num as usize {
                    let target: *const ::core::ffi::c_char = *r.targets.add(ti);
                    let suffix: *const ::core::ffi::c_char = *r.suffixes.add(ti);
                    // When recursing, only terminal rules may match "%" alone;
                    // and the rule's fixed text must fit in the name.
                    if recursions > 0 && *target.add(1) == 0 && r.terminal == 0
                        || *r.lens.add(ti) as size_t > namelen
                    {
                        continue;
                    }
                    stem = filename.offset(suffix.offset_from(target) - 1);
                    stemlen = namelen
                        .wrapping_sub(*r.lens.add(ti) as size_t)
                        .wrapping_add(1);
                    // A target pattern without a slash matches only the part
                    // of the name after its last slash.
                    let check_lastslash: ::core::ffi::c_char = if !lastslash.is_null() {
                        strchr(target, '/' as i32).is_null() as ::core::ffi::c_char
                    } else {
                        0
                    };
                    if check_lastslash != 0 {
                        if pathlen > stemlen {
                            continue;
                        }
                        stemlen = stemlen.wrapping_sub(pathlen);
                        stem = stem.add(pathlen);
                    }
                    // The target text before the stem must match the name
                    // (relative to the last slash when the target has none).
                    if check_lastslash != 0 {
                        if stem > lastslash.add(1)
                            && strncmp(
                                target,
                                lastslash.add(1),
                                (stem.offset_from(lastslash) - 1) as size_t,
                            ) != 0
                        {
                            continue;
                        }
                    } else if stem > filename
                        && strncmp(target, filename, stem.offset_from(filename) as size_t) != 0
                    {
                        continue;
                    }
                    // The text after the stem (the suffix) must also match.
                    let suffix_matches = *suffix == *stem.add(stemlen)
                        && (*suffix == 0 || streq(suffix.add(1), stem.add(stemlen + 1)));
                    if !suffix_matches {
                        continue;
                    }
                    // A target with anything besides '%' is a specific rule.
                    if *target.add(1) != 0 {
                        specific_rule_matched = 1;
                    }
                    if !(r.deps.is_null() && r.cmds.is_null()) {
                        let tr = tryrules
                            .add(nrules as usize)
                            .as_mut()
                            .expect("tryrules allocation");
                        tr.rule = rule;
                        tr.matches = ti as ::core::ffi::c_uint;
                        tr.stemlen =
                            stemlen.wrapping_add(if check_lastslash != 0 { pathlen } else { 0 });
                        tr.order = nrules;
                        tr.checked_lastslash = check_lastslash;
                        nrules = nrules.wrapping_add(1);
                    }
                }
            }
        }
        rule = r.next;
    }
    if nrules != 0 {
        // Shortest-stem (most specific) candidates first, stable by order.
        if nrules > 1 {
            qsort(
                tryrules.cast(),
                nrules as size_t,
                ::core::mem::size_of::<tryrule>() as size_t,
                Some(stemlen_compare),
            );
        }
        // If a specific rule matched, discard non-terminal match-anything
        // ("%") rules.
        if specific_rule_matched != 0 {
            for ri in 0..nrules as usize {
                let tr = tryrules.add(ri).as_mut().expect("tryrules allocation");
                let r = tr.rule.as_ref().expect("collected rules are non-null");
                if r.terminal == 0 {
                    for j in 0..r.num as usize {
                        if *(*r.targets.add(j)).add(1) == 0 {
                            tr.rule = ::core::ptr::null_mut();
                            break;
                        }
                    }
                }
            }
        }
        // Second pass: try each candidate; the first round requires every
        // prerequisite to exist or "ought to exist", the second round
        // ("trying harder") also accepts buildable intermediate files.
        let mut intermed_ok: ::core::ffi::c_int = 0;
        while intermed_ok < 2 {
            pat = deplist;
            if intermed_ok != 0 {
                dbs!(depth, c"Trying harder.\n".as_ptr());
            }
            ri = 0;
            while ri < nrules {
                let mut failed: ::core::ffi::c_uint = 0;
                let mut file_variables_set: ::core::ffi::c_int = 0;
                let mut deps_found: ::core::ffi::c_uint = 0;
                let mut order_only: ::core::ffi::c_int = 0;
                let tr = *tryrules.add(ri as usize);
                rule = tr.rule;
                if !rule.is_null() && !(intermed_ok != 0 && (*rule).terminal != 0) {
                    let matches = tr.matches as usize;
                    stem = filename
                        .offset(
                            (*(*rule).suffixes.add(matches))
                                .offset_from(*(*rule).targets.add(matches)),
                        )
                        .sub(1);
                    stemlen = namelen
                        .wrapping_sub(*(*rule).lens.add(matches) as size_t)
                        .wrapping_add(1);
                    let check_lastslash: ::core::ffi::c_char = tr.checked_lastslash;
                    if check_lastslash != 0 {
                        stem = stem.add(pathlen);
                        stemlen = stemlen.wrapping_sub(pathlen);
                        if pathdir.is_null() {
                            // NUL-terminated copy of the directory prefix.
                            pathdir_buf.resize(pathlen + 1, 0);
                            pathdir = pathdir_buf.as_mut_ptr().cast();
                            memcpy(pathdir.cast(), filename.cast(), pathlen);
                            *pathdir.add(pathlen) = 0;
                        }
                    }
                    dbs!(
                        depth,
                        c"Trying pattern rule '%s' with stem '%.*s'.\n".as_ptr(),
                        get_rule_defn(rule),
                        stemlen as ::core::ffi::c_int,
                        stem
                    );
                    if stemlen.wrapping_add(if check_lastslash != 0 { pathlen } else { 0 })
                        > GET_PATH_MAX as size_t
                    {
                        dbs!(
                            depth,
                            c"Stem too long: '%s%.*s'.\n".as_ptr(),
                            if check_lastslash != 0 {
                                pathdir as *const ::core::ffi::c_char
                            } else {
                                c"".as_ptr()
                            },
                            stemlen as ::core::ffi::c_int,
                            stem
                        );
                    } else {
                        if check_lastslash == 0 {
                            memcpy(stem_str_ptr.cast(), stem.cast(), stemlen);
                            *stem_str_ptr.add(stemlen) = 0;
                        } else {
                            memcpy(stem_str_ptr.cast(), filename.cast(), pathlen);
                            memcpy(stem_str_ptr.add(pathlen).cast(), stem.cast(), stemlen);
                            *stem_str_ptr.add(pathlen.wrapping_add(stemlen)) = 0;
                        }
                        if (*rule).deps.is_null() {
                            // A matching rule without prerequisites wins
                            // immediately.
                            break;
                        }
                        (*rule).in_use = 1;
                        pat = deplist;
                        let mut dep: *mut dep = (*rule).deps;
                        let mut nptr: *const ::core::ffi::c_char = dep_name(dep);
                        loop {
                            let mut dl: *mut dep = ::core::ptr::null_mut();
                            let mut d: *mut dep;
                            if nptr.is_null() {
                                // This dep is exhausted; move to the next.
                                dep = (*dep).next;
                                if dep.is_null() {
                                    break;
                                }
                                nptr = dep_name(dep);
                            }
                            if (*dep).need_2nd_expansion() == 0 {
                                // No second expansion: substitute the stem
                                // for '%' and parse the whole name at once.
                                let mut is_explicit: ::core::ffi::c_int = 1;
                                let cp: *const ::core::ffi::c_char = strchr(nptr, '%' as i32);
                                if cp.is_null() {
                                    strcpy(depname, nptr);
                                } else {
                                    let mut o: *mut ::core::ffi::c_char = depname;
                                    if check_lastslash != 0 {
                                        o = mempcpy(o.cast(), filename.cast(), pathlen).cast();
                                    }
                                    o = mempcpy(
                                        o.cast(),
                                        nptr.cast(),
                                        cp.offset_from(nptr) as size_t,
                                    )
                                    .cast();
                                    o = mempcpy(o.cast(), stem.cast(), stemlen).cast();
                                    strcpy(o, cp.add(1));
                                    is_explicit = 0;
                                }
                                let mut p: *mut ::core::ffi::c_char = depname;
                                dl = parse_file_seq(
                                    &raw mut p,
                                    ::core::mem::size_of::<dep>() as size_t,
                                    MAP_NUL,
                                    ::core::ptr::null(),
                                    PARSEFS_ONEWORD | PARSEFS_WAIT,
                                ) as *mut dep;
                                d = dl;
                                while !d.is_null() {
                                    deps_found = deps_found.wrapping_add(1);
                                    (*d).set_ignore_mtime((*dep).ignore_mtime());
                                    (*d).set_ignore_automatic_vars((*dep).ignore_automatic_vars());
                                    (*d).set_wait_here((*d).wait_here() | (*dep).wait_here());
                                    (*d).set_is_explicit(is_explicit as ::core::ffi::c_uint);
                                    d = (*d).next;
                                }
                                nptr = ::core::ptr::null();
                            } else {
                                // Second expansion: take one word at a time,
                                // replace '%' with $* (or $(*F)), expand, and
                                // parse the result.
                                let mut add_dir: ::core::ffi::c_int = 0;
                                let mut len: size_t = 0;
                                nptr = get_next_word(nptr, &raw mut len);
                                if nptr.is_null() {
                                    continue;
                                }
                                let end: *const ::core::ffi::c_char = nptr.add(len);
                                if order_only == 0
                                    && len == 1
                                    && *nptr == '|' as ::core::ffi::c_char
                                {
                                    order_only = 1;
                                    nptr = end;
                                    continue;
                                }
                                let is_explicit: ::core::ffi::c_int;
                                let mut cp: *const ::core::ffi::c_char =
                                    lindex(nptr, end, '%' as i32);
                                if cp.is_null() {
                                    memcpy(depname.cast(), nptr.cast(), len);
                                    *depname.add(len) = 0;
                                    is_explicit = 1;
                                } else {
                                    let mut o: *mut ::core::ffi::c_char = depname;
                                    is_explicit = 0;
                                    loop {
                                        let i: size_t = cp.offset_from(nptr) as size_t;
                                        assert!(o.add(i) < dend, "dep name buffer overflow");
                                        o = mempcpy(o.cast(), nptr.cast(), i).cast();
                                        if check_lastslash != 0 {
                                            add_dir = 1;
                                            assert!(o.add(5) < dend, "dep name buffer overflow");
                                            o = mempcpy(o.cast(), c"$(*F)".as_ptr().cast(), 5)
                                                .cast();
                                        } else {
                                            assert!(o.add(2) < dend, "dep name buffer overflow");
                                            o = mempcpy(o.cast(), c"$*".as_ptr().cast(), 2).cast();
                                        }
                                        assert!(o < dend, "dep name buffer overflow");
                                        cp = cp.add(1);
                                        assert!(cp <= end, "dep name scan overran the word");
                                        nptr = cp;
                                        if nptr == end {
                                            break;
                                        }
                                        // Skip over a variable reference so a
                                        // '%' inside one is not substituted.
                                        while cp < end
                                            && !stop_set(*cp, MAP_BLANK | MAP_NEWLINE | MAP_NUL)
                                        {
                                            cp = cp.add(1);
                                        }
                                        cp = lindex(cp, end, '%' as i32);
                                        if cp.is_null() {
                                            break;
                                        }
                                    }
                                    len = end.offset_from(nptr) as size_t;
                                    memcpy(o.cast(), nptr.cast(), len);
                                    *o.add(len) = 0;
                                }
                                nptr = end;
                                // The automatic variables ($*, $@, ...) must
                                // be in place before expanding the dep.
                                if file_vars_initialized == 0 {
                                    initialize_file_variables(file, 0);
                                    set_file_variables(file, stem_str_ptr);
                                    file_vars_initialized = 1;
                                } else if file_variables_set == 0 {
                                    define_variable_in_set(
                                        c"*".as_ptr(),
                                        1,
                                        stem_str_ptr,
                                        o_automatic,
                                        0,
                                        (*(*file).variables).set,
                                        ::core::ptr::null_mut(),
                                    );
                                    file_variables_set = 1;
                                }
                                let mut p: *mut ::core::ffi::c_char =
                                    expand_string_for_file(depname, file);
                                let mut dptr: *mut *mut dep = &raw mut dl;
                                loop {
                                    let dp: *mut dep = parse_file_seq(
                                        &raw mut p,
                                        ::core::mem::size_of::<dep>() as size_t,
                                        if order_only != 0 { MAP_NUL } else { MAP_PIPE },
                                        if add_dir != 0 {
                                            pathdir
                                        } else {
                                            ::core::ptr::null_mut()
                                        },
                                        PARSEFS_WAIT,
                                    )
                                        as *mut dep;
                                    *dptr = dp;
                                    d = dp;
                                    while !d.is_null() {
                                        deps_found = deps_found.wrapping_add(1);
                                        if order_only != 0 {
                                            (*d).set_ignore_mtime(1);
                                        }
                                        (*d).set_is_explicit(is_explicit as ::core::ffi::c_uint);
                                        dptr = &raw mut (*d).next;
                                        d = (*d).next;
                                    }
                                    if *p == '|' as ::core::ffi::c_char {
                                        order_only = 1;
                                        p = p.add(1);
                                    }
                                    if *p == 0 {
                                        break;
                                    }
                                }
                            }
                            // Grow the patdeps list if this rule produced
                            // more deps than any rule seen before.
                            if deps_found > max_deps {
                                let l: size_t = pat.offset_from(deplist) as size_t;
                                max_pattern_deps = max_pattern_deps.max(deps_found);
                                max_deps = max_pattern_deps;
                                deplist = xrealloc(
                                    deplist.cast(),
                                    (max_deps as size_t)
                                        .wrapping_mul(::core::mem::size_of::<patdeps>() as size_t),
                                ) as *mut patdeps;
                                pat = deplist.add(l);
                            }
                            // Check each expanded prerequisite for viability.
                            d = dl;
                            while !d.is_null() {
                                let is_rule: ::core::ffi::c_int =
                                    ((*d).name == dep_name(dep)) as ::core::ffi::c_int;
                                let mut explicit: ::core::ffi::c_int = 0;
                                let mut dp: *mut dep = ::core::ptr::null_mut();
                                if file_impossible_p((*d).name) != 0 {
                                    dbs!(
                                        depth,
                                        if is_rule != 0 {
                                            c"Rejecting rule '%s' due to impossible rule prerequisite '%s'.\n".as_ptr()
                                        } else {
                                            c"Rejecting rule '%s' due to impossible implicit prerequisite '%s'.\n".as_ptr()
                                        },
                                        get_rule_defn(rule),
                                        (*d).name
                                    );
                                    tryrules
                                        .add(ri as usize)
                                        .as_mut()
                                        .expect("tryrules allocation")
                                        .rule = ::core::ptr::null_mut();
                                    failed = 1;
                                    break;
                                }
                                memset(pat.cast(), 0, ::core::mem::size_of::<patdeps>() as size_t);
                                (*pat).set_ignore_mtime((*d).ignore_mtime());
                                (*pat).set_ignore_automatic_vars((*d).ignore_automatic_vars());
                                (*pat).set_wait_here((*d).wait_here());
                                (*pat).set_is_explicit((*d).is_explicit());
                                dbs!(
                                    depth,
                                    if is_rule != 0 {
                                        c"Trying rule prerequisite '%s'.\n".as_ptr()
                                    } else {
                                        c"Trying implicit prerequisite '%s'.\n".as_ptr()
                                    },
                                    (*d).name
                                );
                                let df: *mut file = lookup_file((*d).name);
                                if !df.is_null() && (*df).is_explicit() != 0 {
                                    (*pat).set_is_explicit(1);
                                }
                                if !df.is_null()
                                    && (*df).is_explicit() == 0
                                    && (*d).is_explicit() == 0
                                {
                                    (*df).set_intermediate(1);
                                }
                                // A prerequisite "ought to exist" if it is an
                                // explicit target or a dep of our target.
                                if !df.is_null() && (*df).is_target() != 0 {
                                    explicit = 1;
                                } else {
                                    dp = (*file).deps;
                                    while !dp.is_null() {
                                        if streq((*d).name, dep_name(dp)) {
                                            break;
                                        }
                                        dp = (*dp).next;
                                    }
                                }
                                if explicit != 0 || !dp.is_null() {
                                    (*pat).name = (*d).name;
                                    pat = pat.add(1);
                                    dbs!(depth, c"'%s' ought to exist.\n".as_ptr(), (*d).name);
                                } else if file_exists_p((*d).name) != 0 {
                                    (*pat).name = (*d).name;
                                    pat = pat.add(1);
                                    dbs!(depth, c"Found '%s'.\n".as_ptr(), (*d).name);
                                } else if !df.is_null() && allow_compat_rules != 0 {
                                    (*pat).name = (*d).name;
                                    pat = pat.add(1);
                                    dbs!(
                                        depth,
                                        c"Using compatibility rule '%s' due to '%s'.\n".as_ptr(),
                                        get_rule_defn(rule),
                                        (*d).name
                                    );
                                } else {
                                    if !df.is_null() {
                                        dbs!(
                                            depth,
                                            c"Prerequisite '%s' of rule '%s' does not qualify as ought to exist.\n".as_ptr(),
                                            (*d).name,
                                            get_rule_defn(rule)
                                        );
                                        found_compat_rule = 1;
                                    }
                                    let vname: *const ::core::ffi::c_char = vpath_search(
                                        (*d).name,
                                        ::core::ptr::null_mut::<uintmax_t>(),
                                        ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                        ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                    );
                                    if !vname.is_null() {
                                        dbs!(
                                            depth,
                                            c"Found prerequisite '%s' as VPATH '%s'.\n".as_ptr(),
                                            (*d).name,
                                            vname
                                        );
                                        (*pat).name = (*d).name;
                                        pat = pat.add(1);
                                    } else {
                                        // Last resort: recursively search for
                                        // a rule chain that builds it as an
                                        // intermediate file.
                                        let mut found_intermediate = false;
                                        if intermed_ok != 0 {
                                            dbs!(
                                                depth,
                                                if (*d).is_explicit() != 0
                                                    || !df.is_null() && (*df).is_explicit() != 0
                                                {
                                                    c"Looking for a rule with explicit file '%s'.\n"
                                                        .as_ptr()
                                                } else {
                                                    c"Looking for a rule with intermediate file '%s'.\n".as_ptr()
                                                },
                                                (*d).name
                                            );
                                            if int_file.is_null() {
                                                int_file_storage
                                                    .push(Box::new(::core::mem::zeroed::<file>()));
                                                int_file = &raw mut **int_file_storage
                                                    .last_mut()
                                                    .expect("just pushed");
                                            }
                                            ::core::ptr::write_bytes(int_file, 0, 1);
                                            (*int_file).name = (*d).name;
                                            if pattern_search(
                                                int_file,
                                                0,
                                                depth,
                                                recursions.wrapping_add(1),
                                                allow_compat_rules,
                                            ) != 0
                                            {
                                                (*pat).pattern = (*int_file).name;
                                                (*int_file).name = (*d).name;
                                                (*pat).file = int_file;
                                                int_file = ::core::ptr::null_mut();
                                                (*pat).name = (*d).name;
                                                pat = pat.add(1);
                                                found_intermediate = true;
                                            } else {
                                                if !(*int_file).variables.is_null() {
                                                    free_variable_set((*int_file).variables);
                                                }
                                                if !(*int_file).pat_variables.is_null() {
                                                    free_variable_set((*int_file).pat_variables);
                                                }
                                                if df.is_null() {
                                                    file_impossible((*d).name);
                                                }
                                            }
                                        }
                                        if !found_intermediate {
                                            if intermed_ok != 0 {
                                                dbs!(
                                                    depth,
                                                    c"Rejecting rule '%s' due to impossible prerequisite '%s'.\n".as_ptr(),
                                                    get_rule_defn(rule),
                                                    (*d).name
                                                );
                                            } else {
                                                dbs!(
                                                    depth,
                                                    c"Not found '%s'.\n".as_ptr(),
                                                    (*d).name
                                                );
                                            }
                                            failed = 1;
                                            break;
                                        }
                                    }
                                }
                                d = (*d).next;
                            }
                            free_dep_chain(dl);
                            if failed != 0 {
                                break;
                            }
                        }
                        (*rule).in_use = 0;
                        if failed == 0 {
                            // Every prerequisite checked out: use this rule.
                            break;
                        }
                    }
                }
                ri = ri.wrapping_add(1);
            }
            if ri < nrules {
                break;
            }
            rule = ::core::ptr::null_mut();
            intermed_ok += 1;
        }
        if !rule.is_null() {
            foundrule = ri;
            let found_tr = *tryrules.add(foundrule as usize);
            // When recursing, give the file the matched target pattern as its
            // name; the caller uses it to build the real name from the stem.
            if recursions > 0 {
                (*file).name = *(*rule).targets.add(found_tr.matches as usize);
            }
            // Walk the recorded prerequisites backwards, entering each one as
            // a dep of the target (so the final list is in rule order).
            while pat > deplist {
                pat = pat.sub(1);
                if !(*pat).file.is_null() {
                    // An intermediate file: merge the scratch entry into the
                    // real file table.
                    let mut imf: *mut file = (*pat).file;
                    let mut f: *mut file = lookup_file((*imf).name);
                    if f.is_null() {
                        f = enter_file((*imf).name);
                    }
                    (*f).deps = (*imf).deps;
                    (*f).cmds = (*imf).cmds;
                    (*f).stem = (*imf).stem;
                    merge_variable_set_lists(&raw mut (*f).variables, (*imf).variables);
                    (*f).pat_variables = (*imf).pat_variables;
                    (*f).set_pat_searched((*imf).pat_searched());
                    (*f).also_make = (*imf).also_make;
                    (*f).set_is_target(1);
                    (*f).set_is_explicit(
                        (*f).is_explicit()
                            | ((*imf).is_explicit() != 0 || (*pat).is_explicit() != 0)
                                as ::core::ffi::c_uint,
                    );
                    (*f).set_notintermediate(
                        (*f).notintermediate()
                            | ((*imf).notintermediate() != 0 || no_intermediates != 0)
                                as ::core::ffi::c_uint,
                    );
                    (*f).set_intermediate(
                        (*f).intermediate()
                            | ((*f).is_explicit() == 0 && (*f).notintermediate() == 0)
                                as ::core::ffi::c_uint,
                    );
                    (*f).set_tried_implicit(1);
                    imf = lookup_file((*pat).pattern);
                    if !imf.is_null() && (*imf).precious() != 0 {
                        (*f).set_precious(1);
                    }
                    let mut d: *mut dep = (*f).deps;
                    while !d.is_null() {
                        (*d).file = enter_file((*d).name);
                        (*d).name = ::core::ptr::null();
                        (*(*d).file)
                            .set_tried_implicit((*(*d).file).tried_implicit() | (*d).changed());
                        d = (*d).next;
                    }
                }
                let new_dep: *mut dep = alloc_dep();
                (*new_dep).set_ignore_mtime((*pat).ignore_mtime());
                (*new_dep).set_is_explicit((*pat).is_explicit());
                (*new_dep).set_ignore_automatic_vars((*pat).ignore_automatic_vars());
                (*new_dep).set_wait_here((*pat).wait_here());
                let s: *const ::core::ffi::c_char = strcache_add((*pat).name);
                if recursions != 0 {
                    (*new_dep).name = s;
                } else {
                    (*new_dep).file = lookup_file(s);
                    if (*new_dep).file.is_null() {
                        (*new_dep).file = enter_file(s);
                    }
                }
                if (*pat).file.is_null() && (*found_tr.rule).terminal != 0 {
                    // A terminal rule's non-intermediate prerequisites must
                    // exist as-is; mark them so they are not built.
                    if (*new_dep).file.is_null() {
                        (*new_dep).set_changed(1);
                    } else {
                        (*(*new_dep).file).set_tried_implicit(1);
                    }
                }
                (*new_dep).next = (*file).deps;
                (*file).deps = new_dep;
                (*file).set_was_shuffled(0);
            }
            if (*file).was_shuffled() == 0 {
                crate::shuffle::shuffle_deps_recursive((*file).deps);
            }
            if found_tr.checked_lastslash == 0 {
                (*file).stem = strcache_add_len(stem, stemlen);
                fullstemlen = stemlen;
            } else {
                // The rule matched only the basename: the stem includes the
                // directory part.
                fullstemlen = pathlen.wrapping_add(stemlen);
                memcpy(stem_str_ptr.cast(), filename.cast(), pathlen);
                memcpy(stem_str_ptr.add(pathlen).cast(), stem.cast(), stemlen);
                *stem_str_ptr.add(fullstemlen) = 0;
                (*file).stem = strcache_add(stem_str_ptr);
            }
            (*file).cmds = (*rule).cmds;
            (*file).set_is_target(1);
            // Inherit .PRECIOUS and .NOTINTERMEDIATE from the target pattern.
            let pattern_file: *mut file =
                lookup_file(*(*rule).targets.add(found_tr.matches as usize));
            if !pattern_file.is_null() {
                if (*pattern_file).precious() != 0 {
                    (*file).set_precious(1);
                }
                if (*pattern_file).notintermediate() != 0 || no_intermediates != 0 {
                    (*file).set_notintermediate(1);
                }
            }
            // A multi-target rule also makes the other targets (with the same
            // stem substituted).
            if (*rule).num > 1 {
                for ti in 0..(*rule).num as usize {
                    if ti == found_tr.matches as usize {
                        continue;
                    }
                    let target: *const ::core::ffi::c_char = *(*rule).targets.add(ti);
                    let suffix: *const ::core::ffi::c_char = *(*rule).suffixes.add(ti);
                    let target_len = *(*rule).lens.add(ti) as size_t;
                    let mut nm: Vec<u8> = vec![0; target_len.wrapping_add(fullstemlen) + 1];
                    let mut p: *mut ::core::ffi::c_char = nm.as_mut_ptr().cast();
                    let new_dep: *mut dep = alloc_dep();
                    p = mempcpy(
                        p.cast(),
                        target.cast(),
                        (suffix.offset_from(target) - 1) as size_t,
                    )
                    .cast();
                    p = mempcpy(p.cast(), (*file).stem.cast(), fullstemlen).cast();
                    memcpy(
                        p.cast(),
                        suffix.cast(),
                        (target_len as isize - suffix.offset_from(target) + 1) as size_t,
                    );
                    (*new_dep).name = strcache_add(nm.as_ptr().cast());
                    (*new_dep).file = enter_file((*new_dep).name);
                    (*new_dep).next = (*file).also_make;
                    let other: *mut file = lookup_file(target);
                    if !other.is_null() {
                        if (*other).precious() != 0 {
                            (*(*new_dep).file).set_precious(1);
                        }
                        if (*other).notintermediate() != 0 || no_intermediates != 0 {
                            (*(*new_dep).file).set_notintermediate(1);
                        }
                    }
                    (*(*new_dep).file).set_is_target(1);
                    (*file).also_make = new_dep;
                }
            }
        }
    } else {
        rule = ::core::ptr::null_mut();
    }
    free(tryrules.cast());
    free(deplist.cast());
    depth = depth.wrapping_sub(1);
    if !rule.is_null() {
        dbs!(
            depth,
            c"Found implicit rule '%s' for '%s'.\n".as_ptr(),
            get_rule_defn(rule),
            filename
        );
        return 1;
    }
    if found_compat_rule != 0 {
        dbs!(
            depth,
            c"Searching for a compatibility rule for '%s'.\n".as_ptr(),
            filename
        );
        assert!(
            allow_compat_rules == 0,
            "compatibility-rule retry must not recurse"
        );
        return pattern_search(file, archive, depth, recursions, 1);
    }
    dbs!(
        depth,
        c"No implicit rule found for '%s'.\n".as_ptr(),
        filename
    );
    0
}
