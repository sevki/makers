//! Implicit (pattern) rule search: `pattern_search` walks the pattern-rule
//! database looking for a rule — or a chain of rules through intermediate
//! files — that can build a given target.
//!
//! Port of `implicit.c`.

use std::sync::atomic::Ordering;

pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{Dep, File};
use crate::misc::free_ns_chain;
use crate::misc::{print_spaces, skip_reference, xcalloc};
use crate::stdio::FILE;
use crate::strcache::{strcache_add, strcache_add_len};
use c2rust_bitfields;
use libc::{printf, strchr, strlen};
extern "C" {
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
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
    get_rule_defn, max_pattern_dep_length, pattern_rules, MAX_PATTERN_DEPS, MAX_PATTERN_TARGETS,
    NUM_PATTERN_RULES,
};
use crate::variable::o_automatic;
use crate::variable::{
    define_variable_in_set, free_variable_set, initialize_file_variables, merge_variable_set_lists,
};
use crate::vpath::vpath_search;

/// `DB_IMPLICIT`: `-d` implicit-rule tracing enabled in `db_level`.
const DB_IMPLICIT: i32 = 0x8;
/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_NUL: i32 = 0x0001;
const MAP_BLANK: i32 = 0x0002;
const MAP_NEWLINE: i32 = 0x0004;
const MAP_PIPE: i32 = 0x0100;
/// `parse_file_seq` flags (see `dep.h`).
const PARSEFS_ONEWORD: i32 = 0x20;
const PARSEFS_WAIT: i32 = 0x40;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
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

/// Borrow a NUL-terminated C string as a byte slice (without the NUL).
unsafe fn cstr_bytes<'a>(s: *const ::core::ffi::c_char) -> &'a [u8] {
    ::core::slice::from_raw_parts(s.cast::<u8>(), strlen(s))
}

/// String equality, mirroring make's `streq` macro (`strcmp(a, b) == 0`).
fn streq(a: &::core::ffi::CStr, b: &::core::ffi::CStr) -> bool {
    a == b
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
pub struct tryrule {
    pub rule: *mut rule,
    pub stemlen: size_t,
    pub matches: ::core::ffi::c_uint,
    pub order: ::core::ffi::c_uint,
    pub checked_lastslash: bool,
}
pub const PATH_MAX: usize = 4096;
pub const GET_PATH_MAX: usize = PATH_MAX;
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
pub unsafe fn try_implicit_rule(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    depth: ::core::ffi::c_uint,
) -> i32 {
    let name = file
        .as_ref()
        .expect("try_implicit_rule requires a file")
        .name;
    dbs!(
        depth,
        c"Looking for an implicit rule for '%s'.\n".as_ptr(),
        name
    );
    if pattern_search(ctx, file, 0, depth, 0, 0) != 0 {
        return 1;
    }
    if ar_name(ctx, ::core::ffi::CStr::from_ptr(name)) {
        dbs!(
            depth,
            c"Looking for archive-member implicit rule for '%s'.\n".as_ptr(),
            name
        );
        if pattern_search(ctx, file, 1, depth, 0, 0) != 0 {
            return 1;
        }
        dbs!(
            depth,
            c"No archive-member implicit rule found for '%s'.\n".as_ptr(),
            name
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
    // View the NUL-terminated buffer as a byte slice (excluding the NUL) so the
    // scan walks indices instead of dereferencing raw pointers.
    let bytes: &[u8] = ::core::ffi::CStr::from_ptr(buffer).to_bytes();
    let n = bytes.len();
    let mut i = 0usize;
    // Skip any leading blanks or newlines.
    while i < n && stop_set(bytes[i], MAP_BLANK | MAP_NEWLINE) {
        i += 1;
    }
    let beg = i;
    if i >= n {
        // The first non-blank byte is the terminating NUL: no word remains.
        return ::core::ptr::null();
    }
    // Consume the first byte of the word.
    let mut c = bytes[i];
    i += 1;
    loop {
        match c {
            0 | b' ' | b'\t' => {
                // Back up over the terminating whitespace/NUL.
                i -= 1;
                break;
            }
            b'$' => {
                // `skip_reference` consumes a `$(...)`/`${...}` reference. It
                // takes the bytes following the `$` and returns the number of
                // bytes consumed; advance our index by that amount.
                let consumed = skip_reference(&bytes[i..]);
                i += consumed;
            }
            b'|' => {
                break;
            }
            _ => {}
        }
        // Read the next byte, treating the position past the slice as the NUL.
        c = if i < n { bytes[i] } else { 0 };
        i += 1;
    }
    if let Some(len) = length.as_mut() {
        *len = (i - beg) as size_t;
    }
    bytes[beg..].as_ptr() as *const ::core::ffi::c_char
}
/// The per-target views of a rule needed for matching: the target string,
/// its bytes, and the index of its `%`.
unsafe fn rule_target(r: &rule, ti: usize) -> (*const ::core::ffi::c_char, &[u8], usize) {
    let targets = ::core::slice::from_raw_parts(r.targets, r.num as usize);
    let lens = ::core::slice::from_raw_parts(r.lens, r.num as usize);
    let target = targets[ti];
    let bytes = ::core::slice::from_raw_parts(target.cast::<u8>(), lens[ti] as usize);
    let percent = bytes
        .iter()
        .position(|&b| b == b'%')
        .expect("pattern rule target must contain a '%'");
    (target, bytes, percent)
}
unsafe fn pattern_search(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    archive: i32,
    mut depth: ::core::ffi::c_uint,
    recursions: ::core::ffi::c_uint,
    allow_compat_rules: i32,
) -> i32 {
    let file_ref = file.as_mut().expect("pattern_search requires a file");
    // The target name (inside the parens for an archive member reference).
    let filename: *const ::core::ffi::c_char = if archive != 0 {
        strchr(file_ref.name, '(' as i32)
    } else {
        file_ref.name
    };
    let namelen: size_t = strlen(filename);
    // Byte view of the target name (without the NUL); the underlying string
    // lives in the string cache and is never mutated during the search.
    let name: &[u8] = ::core::slice::from_raw_parts(filename.cast::<u8>(), namelen);
    // Backing storage for the "intermediate file" scratch entries; entries
    // may be linked into the patdeps list, so they must live until return.
    let mut int_file_storage: Vec<Box<file>> = Vec::new();
    // A scratch entry kept for reuse when the previous intermediate search
    // failed. Holding it in an `Option` (rather than a nullable raw pointer)
    // keeps the pointer always valid storage-backed, never a null sentinel.
    let mut int_file_reuse: Option<*mut file> = None;
    let mut max_deps: ::core::ffi::c_uint = MAX_PATTERN_DEPS.load(Ordering::Relaxed);
    // The viable prerequisites recorded while trying a rule.
    let mut deplist: Vec<patdeps> = Vec::with_capacity(max_deps as usize);
    // Scratch buffer for a prerequisite name with the stem substituted.
    let mut depname: Vec<u8> =
        Vec::with_capacity(namelen.wrapping_add(max_pattern_dep_length).wrapping_add(4));
    let mut stem_off: usize = 0;
    let mut stemlen: size_t = 0;
    let fullstemlen: size_t;
    // Candidate rules whose targets match the name.
    let mut tryrules: Vec<tryrule> = Vec::with_capacity(
        NUM_PATTERN_RULES
            .load(Ordering::Relaxed)
            .wrapping_mul(MAX_PATTERN_TARGETS.load(Ordering::Relaxed)) as usize,
    );
    let foundrule: usize;
    let mut file_vars_initialized: i32 = 0;
    let mut specific_rule_matched: bool = false;
    let mut ri: usize = 0;
    let mut found_compat_rule: i32 = 0;
    let mut rule: *mut rule;
    let mut pathdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
    let mut pathdir_buf: Vec<u8> = Vec::new();
    let mut stem_str: [u8; PATH_MAX + 1] = [0; PATH_MAX + 1];
    depth = depth.wrapping_add(1);
    // An archive member name has no directory part.
    let pathlen: usize = if archive != 0 || ar_name(ctx, ::core::ffi::CStr::from_ptr(filename)) {
        0
    } else {
        name[..namelen.saturating_sub(1)]
            .iter()
            .rposition(|&b| b == b'/')
            .map_or(0, |slash| slash + 1)
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
                    let (_, target, percent) = rule_target(r, ti);
                    // When recursing, only terminal rules may match "%" alone;
                    // and the rule's fixed text must fit in the name.
                    if recursions > 0 && target.len() == 1 && r.terminal == 0
                        || target.len() > namelen
                    {
                        continue;
                    }
                    stem_off = percent;
                    stemlen = namelen.wrapping_sub(target.len()).wrapping_add(1);
                    // A target pattern without a slash matches only the part
                    // of the name after its last slash.
                    let check_lastslash = pathlen > 0 && !target.contains(&b'/');
                    if check_lastslash {
                        if pathlen > stemlen {
                            continue;
                        }
                        stemlen -= pathlen;
                        stem_off += pathlen;
                    }
                    // The target text before the stem must match the name
                    // (relative to the last slash when the target has none).
                    let prefix_start = if check_lastslash { pathlen } else { 0 };
                    if target[..percent] != name[prefix_start..stem_off] {
                        continue;
                    }
                    // The text after the stem (the suffix) must also match.
                    if target[percent + 1..] != name[stem_off + stemlen..] {
                        continue;
                    }
                    // A target with anything besides '%' is a specific rule.
                    if target.len() > 1 {
                        specific_rule_matched = true;
                    }
                    if !(r.deps.is_null() && r.cmds.is_null()) {
                        tryrules.push(tryrule {
                            rule,
                            matches: ti as ::core::ffi::c_uint,
                            stemlen: stemlen + if check_lastslash { pathlen } else { 0 },
                            order: tryrules.len() as ::core::ffi::c_uint,
                            checked_lastslash: check_lastslash,
                        });
                    }
                }
            }
        }
        rule = r.next;
    }
    if !tryrules.is_empty() {
        // Shortest-stem (most specific) candidates first, stable by order.
        tryrules.sort_by_key(|tr| (tr.stemlen, tr.order));
        // If a specific rule matched, discard non-terminal match-anything
        // ("%") rules.
        if specific_rule_matched {
            for tr in &mut tryrules {
                let r = tr.rule.as_ref().expect("collected rules are non-null");
                if r.terminal == 0 {
                    let lens = ::core::slice::from_raw_parts(r.lens, r.num as usize);
                    if lens.contains(&1) {
                        tr.rule = ::core::ptr::null_mut();
                    }
                }
            }
        }
        // Second pass: try each candidate; the first round requires every
        // prerequisite to exist or "ought to exist", the second round
        // ("trying harder") also accepts buildable intermediate files.
        let mut intermed_ok: i32 = 0;
        while intermed_ok < 2 {
            deplist.clear();
            if intermed_ok != 0 {
                dbs!(depth, c"Trying harder.\n".as_ptr());
            }
            ri = 0;
            while ri < tryrules.len() {
                let mut failed = false;
                let mut file_variables_set: i32 = 0;
                let mut deps_found: ::core::ffi::c_uint = 0;
                let mut order_only: i32 = 0;
                let tr = tryrules[ri];
                rule = tr.rule;
                let rule_terminal = rule.as_ref().map_or(0, |r| r.terminal);
                if !rule.is_null() && !(intermed_ok != 0 && rule_terminal != 0) {
                    let rule_ref = rule.as_mut().expect("checked non-null above");
                    let matches = tr.matches as usize;
                    let (_, target, percent) = rule_target(rule_ref, matches);
                    stem_off = percent;
                    stemlen = namelen.wrapping_sub(target.len()).wrapping_add(1);
                    let check_lastslash = tr.checked_lastslash;
                    if check_lastslash {
                        stem_off += pathlen;
                        stemlen -= pathlen;
                        if pathdir.is_null() {
                            // NUL-terminated copy of the directory prefix.
                            pathdir_buf.clear();
                            pathdir_buf.extend_from_slice(&name[..pathlen]);
                            pathdir_buf.push(0);
                            pathdir = pathdir_buf.as_mut_ptr().cast();
                        }
                    }
                    dbs!(
                        depth,
                        c"Trying pattern rule '%s' with stem '%.*s'.\n".as_ptr(),
                        get_rule_defn(rule),
                        stemlen as i32,
                        name[stem_off..].as_ptr()
                    );
                    if stemlen + if check_lastslash { pathlen } else { 0 } > GET_PATH_MAX {
                        dbs!(
                            depth,
                            c"Stem too long: '%s%.*s'.\n".as_ptr(),
                            if check_lastslash {
                                pathdir as *const ::core::ffi::c_char
                            } else {
                                c"".as_ptr()
                            },
                            stemlen as i32,
                            name[stem_off..].as_ptr()
                        );
                    } else {
                        if !check_lastslash {
                            stem_str[..stemlen]
                                .copy_from_slice(&name[stem_off..stem_off + stemlen]);
                            stem_str[stemlen] = 0;
                        } else {
                            stem_str[..pathlen].copy_from_slice(&name[..pathlen]);
                            stem_str[pathlen..pathlen + stemlen]
                                .copy_from_slice(&name[stem_off..stem_off + stemlen]);
                            stem_str[pathlen + stemlen] = 0;
                        }
                        if rule_ref.deps.is_null() {
                            // A matching rule without prerequisites wins
                            // immediately.
                            break;
                        }
                        rule_ref.in_use = 1;
                        deplist.clear();
                        let mut dep: *mut dep = rule_ref.deps;
                        let mut nptr: *const ::core::ffi::c_char = dep_name(dep);
                        loop {
                            let mut dl: *mut dep = ::core::ptr::null_mut();
                            let mut d: *mut dep;
                            if nptr.is_null() {
                                // This dep is exhausted; move to the next.
                                dep = dep.as_ref().expect("dep chain node").next;
                                if dep.is_null() {
                                    break;
                                }
                                nptr = dep_name(dep);
                            }
                            let dep_ref = dep.as_ref().expect("dep is non-null here");
                            if dep_ref.need_2nd_expansion() == 0 {
                                // No second expansion: substitute the stem
                                // for '%' and parse the whole name at once.
                                let mut is_explicit: i32 = 1;
                                let dep_bytes = cstr_bytes(nptr);
                                depname.clear();
                                if let Some(cp) = dep_bytes.iter().position(|&b| b == b'%') {
                                    if check_lastslash {
                                        depname.extend_from_slice(&name[..pathlen]);
                                    }
                                    depname.extend_from_slice(&dep_bytes[..cp]);
                                    depname.extend_from_slice(&name[stem_off..stem_off + stemlen]);
                                    depname.extend_from_slice(&dep_bytes[cp + 1..]);
                                    is_explicit = 0;
                                } else {
                                    depname.extend_from_slice(dep_bytes);
                                }
                                depname.push(0);
                                let mut p: *mut ::core::ffi::c_char = depname.as_mut_ptr().cast();
                                dl = parse_file_seq(
                                    ctx,
                                    &raw mut p,
                                    ::core::mem::size_of::<dep>() as size_t,
                                    MAP_NUL,
                                    ::core::ptr::null(),
                                    PARSEFS_ONEWORD | PARSEFS_WAIT,
                                ) as *mut dep;
                                d = dl;
                                while let Some(dr) = d.as_mut() {
                                    deps_found = deps_found.wrapping_add(1);
                                    dr.set_ignore_mtime(dep_ref.ignore_mtime());
                                    dr.set_ignore_automatic_vars(dep_ref.ignore_automatic_vars());
                                    dr.set_wait_here(dr.wait_here() | dep_ref.wait_here());
                                    dr.set_is_explicit(is_explicit as ::core::ffi::c_uint);
                                    d = dr.next;
                                }
                                nptr = ::core::ptr::null();
                            } else {
                                // Second expansion: take one word at a time,
                                // replace '%' with $* (or $(*F)), expand, and
                                // parse the result.
                                let mut add_dir: i32 = 0;
                                let mut len: size_t = 0;
                                nptr = get_next_word(nptr, &raw mut len);
                                if nptr.is_null() {
                                    continue;
                                }
                                let word: &[u8] =
                                    ::core::slice::from_raw_parts(nptr.cast::<u8>(), len);
                                let end: *const ::core::ffi::c_char = nptr.add(len);
                                if order_only == 0 && word == b"|" {
                                    order_only = 1;
                                    nptr = end;
                                    continue;
                                }
                                let is_explicit: i32;
                                depname.clear();
                                match word.iter().position(|&b| b == b'%') {
                                    None => {
                                        depname.extend_from_slice(word);
                                        is_explicit = 1;
                                    }
                                    Some(first_percent) => {
                                        is_explicit = 0;
                                        let mut percent = first_percent;
                                        let mut start = 0;
                                        loop {
                                            depname.extend_from_slice(&word[start..percent]);
                                            if check_lastslash {
                                                add_dir = 1;
                                                depname.extend_from_slice(b"$(*F)");
                                            } else {
                                                depname.extend_from_slice(b"$*");
                                            }
                                            start = percent + 1;
                                            if start == word.len() {
                                                break;
                                            }
                                            // Skip the rest of this token so a
                                            // '%' inside a reference is not
                                            // substituted.
                                            let mut scan = start;
                                            while scan < word.len()
                                                && !stop_set(
                                                    word[scan],
                                                    MAP_BLANK | MAP_NEWLINE | MAP_NUL,
                                                )
                                            {
                                                scan += 1;
                                            }
                                            match word[scan..].iter().position(|&b| b == b'%') {
                                                None => break,
                                                Some(k) => percent = scan + k,
                                            }
                                        }
                                        depname.extend_from_slice(&word[start..]);
                                    }
                                }
                                depname.push(0);
                                nptr = end;
                                // The automatic variables ($*, $@, ...) must
                                // be in place before expanding the dep.
                                if file_vars_initialized == 0 {
                                    initialize_file_variables(ctx, file, 0);
                                    set_file_variables(ctx, file, stem_str.as_mut_ptr().cast());
                                    file_vars_initialized = 1;
                                } else if file_variables_set == 0 {
                                    define_variable_in_set(
                                        ctx,
                                        c"*".as_ptr(),
                                        1,
                                        stem_str.as_mut_ptr().cast(),
                                        o_automatic,
                                        0,
                                        file_ref
                                            .variables
                                            .as_ref()
                                            .expect("file variables were initialized above")
                                            .set,
                                        ::core::ptr::null_mut(),
                                    );
                                    file_variables_set = 1;
                                }
                                let mut p: *mut ::core::ffi::c_char =
                                    expand_string_for_file(ctx, depname.as_mut_ptr().cast(), file);
                                let mut dptr: *mut *mut dep = &raw mut dl;
                                loop {
                                    let dp: *mut dep = parse_file_seq(
                                        ctx,
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
                                    while let Some(dr) = d.as_mut() {
                                        deps_found = deps_found.wrapping_add(1);
                                        if order_only != 0 {
                                            dr.set_ignore_mtime(1);
                                        }
                                        dr.set_is_explicit(is_explicit as ::core::ffi::c_uint);
                                        dptr = &raw mut dr.next;
                                        d = dr.next;
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
                            // Track the most deps any rule has produced (the
                            // Vec grows on its own).
                            if deps_found > max_deps {
                                let new_max =
                                    MAX_PATTERN_DEPS.load(Ordering::Relaxed).max(deps_found);
                                MAX_PATTERN_DEPS.store(new_max, Ordering::Relaxed);
                                max_deps = new_max;
                            }
                            // Check each expanded prerequisite for viability.
                            d = dl;
                            while let Some(dr) = d.as_mut() {
                                let is_rule = dr.name == dep_name(dep);
                                let mut explicit = false;
                                let mut dp: *mut dep = ::core::ptr::null_mut();
                                if file_impossible_p(ctx, dr.name) != 0 {
                                    dbs!(
                                        depth,
                                        if is_rule {
                                            c"Rejecting rule '%s' due to impossible rule prerequisite '%s'.\n".as_ptr()
                                        } else {
                                            c"Rejecting rule '%s' due to impossible implicit prerequisite '%s'.\n".as_ptr()
                                        },
                                        get_rule_defn(rule),
                                        dr.name
                                    );
                                    tryrules[ri].rule = ::core::ptr::null_mut();
                                    failed = true;
                                    break;
                                }
                                let mut pe: patdeps = ::core::mem::zeroed();
                                pe.set_ignore_mtime(dr.ignore_mtime());
                                pe.set_ignore_automatic_vars(dr.ignore_automatic_vars());
                                pe.set_wait_here(dr.wait_here());
                                pe.set_is_explicit(dr.is_explicit());
                                dbs!(
                                    depth,
                                    if is_rule {
                                        c"Trying rule prerequisite '%s'.\n".as_ptr()
                                    } else {
                                        c"Trying implicit prerequisite '%s'.\n".as_ptr()
                                    },
                                    dr.name
                                );
                                let df: *mut file = lookup_file(dr.name);
                                if let Some(dfr) = df.as_mut() {
                                    if dfr.is_explicit() != 0 {
                                        pe.set_is_explicit(1);
                                    }
                                    if dfr.is_explicit() == 0 && dr.is_explicit() == 0 {
                                        dfr.set_intermediate(1);
                                    }
                                }
                                // A prerequisite "ought to exist" if it is an
                                // explicit target or a dep of our target.
                                if df.as_ref().is_some_and(|f| f.is_target() != 0) {
                                    explicit = true;
                                } else {
                                    dp = file_ref.deps;
                                    while let Some(dpr) = dp.as_ref() {
                                        if streq(
                                            ::core::ffi::CStr::from_ptr(dr.name),
                                            ::core::ffi::CStr::from_ptr(dep_name(dp)),
                                        ) {
                                            break;
                                        }
                                        dp = dpr.next;
                                    }
                                }
                                if explicit || !dp.is_null() {
                                    pe.name = dr.name;
                                    deplist.push(pe);
                                    dbs!(depth, c"'%s' ought to exist.\n".as_ptr(), dr.name);
                                } else if file_exists_p(ctx, dr.name) != 0 {
                                    pe.name = dr.name;
                                    deplist.push(pe);
                                    dbs!(depth, c"Found '%s'.\n".as_ptr(), dr.name);
                                } else if !df.is_null() && allow_compat_rules != 0 {
                                    pe.name = dr.name;
                                    deplist.push(pe);
                                    dbs!(
                                        depth,
                                        c"Using compatibility rule '%s' due to '%s'.\n".as_ptr(),
                                        get_rule_defn(rule),
                                        dr.name
                                    );
                                } else {
                                    if !df.is_null() {
                                        dbs!(
                                            depth,
                                            c"Prerequisite '%s' of rule '%s' does not qualify as ought to exist.\n".as_ptr(),
                                            dr.name,
                                            get_rule_defn(rule)
                                        );
                                        found_compat_rule = 1;
                                    }
                                    let vname: *const ::core::ffi::c_char = vpath_search(
                                        ctx,
                                        dr.name,
                                        ::core::ptr::null_mut::<uintmax_t>(),
                                        ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                        ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                    );
                                    if !vname.is_null() {
                                        dbs!(
                                            depth,
                                            c"Found prerequisite '%s' as VPATH '%s'.\n".as_ptr(),
                                            dr.name,
                                            vname
                                        );
                                        pe.name = dr.name;
                                        deplist.push(pe);
                                    } else {
                                        // Last resort: recursively search for
                                        // a rule chain that builds it as an
                                        // intermediate file.
                                        let mut found_intermediate = false;
                                        if intermed_ok != 0 {
                                            dbs!(
                                                depth,
                                                if dr.is_explicit() != 0
                                                    || df
                                                        .as_ref()
                                                        .is_some_and(|f| f.is_explicit() != 0)
                                                {
                                                    c"Looking for a rule with explicit file '%s'.\n"
                                                        .as_ptr()
                                                } else {
                                                    c"Looking for a rule with intermediate file '%s'.\n".as_ptr()
                                                },
                                                dr.name
                                            );
                                            // Reuse the scratch entry kept from a previous failed
                                            // search, or allocate a fresh one. Either way `int_file`
                                            // comes from valid storage and is never a null sentinel.
                                            let int_file: *mut file = match int_file_reuse.take() {
                                                Some(p) => p,
                                                None => {
                                                    int_file_storage.push(Box::new(
                                                        ::core::mem::zeroed::<file>(),
                                                    ));
                                                    &raw mut **int_file_storage
                                                        .last_mut()
                                                        .expect("just pushed")
                                                }
                                            };
                                            // Reset the scratch entry to a zeroed file before
                                            // reusing it (replaces a raw `write_bytes`).
                                            {
                                                let int_ref = int_file
                                                    .as_mut()
                                                    .expect("scratch entry is storage-backed");
                                                *int_ref = ::core::mem::zeroed::<file>();
                                                int_ref.name = dr.name;
                                            }
                                            if pattern_search(
                                                ctx,
                                                int_file,
                                                0,
                                                depth,
                                                recursions.wrapping_add(1),
                                                allow_compat_rules,
                                            ) != 0
                                            {
                                                let int_ref = int_file
                                                    .as_mut()
                                                    .expect("scratch entry is storage-backed");
                                                pe.pattern = int_ref.name;
                                                int_ref.name = dr.name;
                                                pe.file = int_file;
                                                pe.name = dr.name;
                                                deplist.push(pe);
                                                found_intermediate = true;
                                            } else {
                                                let int_ref = int_file
                                                    .as_mut()
                                                    .expect("scratch entry is storage-backed");
                                                if !int_ref.variables.is_null() {
                                                    free_variable_set(int_ref.variables);
                                                }
                                                if !int_ref.pat_variables.is_null() {
                                                    free_variable_set(int_ref.pat_variables);
                                                }
                                                if df.is_null() {
                                                    file_impossible(ctx, dr.name);
                                                }
                                                // Keep this scratch entry to reuse next iteration.
                                                int_file_reuse = Some(int_file);
                                            }
                                        }
                                        if !found_intermediate {
                                            if intermed_ok != 0 {
                                                dbs!(
                                                    depth,
                                                    c"Rejecting rule '%s' due to impossible prerequisite '%s'.\n".as_ptr(),
                                                    get_rule_defn(rule),
                                                    dr.name
                                                );
                                            } else {
                                                dbs!(depth, c"Not found '%s'.\n".as_ptr(), dr.name);
                                            }
                                            failed = true;
                                            break;
                                        }
                                    }
                                }
                                d = dr.next;
                            }
                            free_dep_chain(dl);
                            if failed {
                                break;
                            }
                        }
                        rule_ref.in_use = 0;
                        if !failed {
                            // Every prerequisite checked out: use this rule.
                            break;
                        }
                    }
                }
                ri += 1;
            }
            if ri < tryrules.len() {
                break;
            }
            rule = ::core::ptr::null_mut();
            intermed_ok += 1;
        }
        if let Some(found_rule) = rule.as_ref() {
            foundrule = ri;
            let found_tr = tryrules[foundrule];
            // When recursing, give the file the matched target pattern as its
            // name; the caller uses it to build the real name from the stem.
            if recursions > 0 {
                let targets =
                    ::core::slice::from_raw_parts(found_rule.targets, found_rule.num as usize);
                file_ref.name = targets[found_tr.matches as usize];
            }
            // Walk the recorded prerequisites backwards, entering each one as
            // a dep of the target (so the final list is in rule order).
            while let Some(pe) = deplist.pop() {
                if !pe.file.is_null() {
                    // An intermediate file: merge the scratch entry into the
                    // real file table.
                    let imf = pe.file.as_mut().expect("checked non-null");
                    // Resolve the real file for this intermediate, creating it
                    // if absent. Use an explicit null check (not `as_mut`) so the
                    // looked-up pointer is treated as a validated, non-null
                    // pointer before it is dereferenced.
                    let found: *mut file = lookup_file(imf.name);
                    let f_ptr: *mut file = if found.is_null() {
                        enter_file(imf.name)
                    } else {
                        found
                    };
                    let f = f_ptr.as_mut().expect("looked up or just entered");
                    f.deps = imf.deps;
                    f.cmds = imf.cmds;
                    f.stem = imf.stem;
                    merge_variable_set_lists(&raw mut f.variables, imf.variables);
                    f.pat_variables = imf.pat_variables;
                    f.set_pat_searched(imf.pat_searched());
                    f.also_make = imf.also_make;
                    f.set_is_target(1);
                    f.set_is_explicit(
                        f.is_explicit()
                            | (imf.is_explicit() != 0 || pe.is_explicit() != 0)
                                as ::core::ffi::c_uint,
                    );
                    f.set_notintermediate(
                        f.notintermediate()
                            | (imf.notintermediate() != 0 || no_intermediates != 0)
                                as ::core::ffi::c_uint,
                    );
                    f.set_intermediate(
                        f.intermediate()
                            | (f.is_explicit() == 0 && f.notintermediate() == 0)
                                as ::core::ffi::c_uint,
                    );
                    f.set_tried_implicit(1);
                    let pattern_owner: *mut file = lookup_file(pe.pattern);
                    if pattern_owner.as_ref().is_some_and(|p| p.precious() != 0) {
                        f.set_precious(1);
                    }
                    let mut d: *mut dep = f.deps;
                    while let Some(dr) = d.as_mut() {
                        dr.file = enter_file(dr.name);
                        dr.name = ::core::ptr::null();
                        let dep_file = dr.file.as_mut().expect("just entered");
                        dep_file.set_tried_implicit(dep_file.tried_implicit() | dr.changed());
                        d = dr.next;
                    }
                }
                let new_dep: *mut dep = alloc_dep();
                let nd = new_dep.as_mut().expect("xcalloc returned null");
                nd.set_ignore_mtime(pe.ignore_mtime());
                nd.set_is_explicit(pe.is_explicit());
                nd.set_ignore_automatic_vars(pe.ignore_automatic_vars());
                nd.set_wait_here(pe.wait_here());
                let s: *const ::core::ffi::c_char = strcache_add(pe.name);
                if recursions != 0 {
                    nd.name = s;
                } else {
                    nd.file = lookup_file(s);
                    if nd.file.is_null() {
                        nd.file = enter_file(s);
                    }
                }
                if pe.file.is_null()
                    && found_tr
                        .rule
                        .as_ref()
                        .expect("found rule is non-null")
                        .terminal
                        != 0
                {
                    // A terminal rule's non-intermediate prerequisites must
                    // exist as-is; mark them so they are not built.
                    if nd.file.is_null() {
                        nd.set_changed(1);
                    } else {
                        nd.file
                            .as_mut()
                            .expect("checked non-null")
                            .set_tried_implicit(1);
                    }
                }
                nd.next = file_ref.deps;
                file_ref.deps = new_dep;
                file_ref.set_was_shuffled(0);
            }
            if file_ref.was_shuffled() == 0 {
                crate::shuffle::shuffle_deps_recursive(file_ref.deps);
            }
            if !found_tr.checked_lastslash {
                file_ref.stem = strcache_add_len(name[stem_off..].as_ptr().cast(), stemlen);
                fullstemlen = stemlen;
            } else {
                // The rule matched only the basename: the stem includes the
                // directory part.
                fullstemlen = pathlen + stemlen;
                stem_str[..pathlen].copy_from_slice(&name[..pathlen]);
                stem_str[pathlen..fullstemlen].copy_from_slice(&name[stem_off..stem_off + stemlen]);
                stem_str[fullstemlen] = 0;
                file_ref.stem = strcache_add(stem_str.as_ptr().cast());
            }
            file_ref.cmds = found_rule.cmds;
            file_ref.set_is_target(1);
            // Inherit .PRECIOUS and .NOTINTERMEDIATE from the target pattern.
            let (found_target, _, _) = rule_target(found_rule, found_tr.matches as usize);
            let pattern_file: *mut file = lookup_file(found_target);
            if let Some(pf) = pattern_file.as_ref() {
                if pf.precious() != 0 {
                    file_ref.set_precious(1);
                }
                if pf.notintermediate() != 0 || no_intermediates != 0 {
                    file_ref.set_notintermediate(1);
                }
            }
            // A multi-target rule also makes the other targets (with the same
            // stem substituted).
            if found_rule.num > 1 {
                for ti in 0..found_rule.num as usize {
                    if ti == found_tr.matches as usize {
                        continue;
                    }
                    let (target_ptr, target, percent) = rule_target(found_rule, ti);
                    let stem_bytes =
                        ::core::slice::from_raw_parts(file_ref.stem.cast::<u8>(), fullstemlen);
                    let mut nm: Vec<u8> = Vec::with_capacity(target.len() + fullstemlen + 1);
                    nm.extend_from_slice(&target[..percent]);
                    nm.extend_from_slice(stem_bytes);
                    nm.extend_from_slice(&target[percent + 1..]);
                    nm.push(0);
                    let new_dep: *mut dep = alloc_dep();
                    let nd = new_dep.as_mut().expect("xcalloc returned null");
                    nd.name = strcache_add(nm.as_ptr().cast());
                    nd.file = enter_file(nd.name);
                    nd.next = file_ref.also_make;
                    let other_file = nd.file.as_mut().expect("just entered");
                    if let Some(other) = lookup_file(target_ptr).as_ref() {
                        if other.precious() != 0 {
                            other_file.set_precious(1);
                        }
                        if other.notintermediate() != 0 || no_intermediates != 0 {
                            other_file.set_notintermediate(1);
                        }
                    }
                    other_file.set_is_target(1);
                    file_ref.also_make = new_dep;
                }
            }
        }
    } else {
        rule = ::core::ptr::null_mut();
    }
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
        return pattern_search(ctx, file, archive, depth, recursions, 1);
    }
    dbs!(
        depth,
        c"No implicit rule found for '%s'.\n".as_ptr(),
        filename
    );
    0
}

#[cfg(test)]
mod streq_tests {
    use super::streq;

    #[test]
    fn equality_matches_strcmp() {
        assert!(streq(c"foo.o", c"foo.o"));
        assert!(streq(c"", c""));
        assert!(!streq(c"foo.o", c"bar.o"));
        assert!(!streq(c"foo", c"foobar")); // prefix is not equal
        assert!(!streq(c"", c"x"));
    }
}
