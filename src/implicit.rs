//! Implicit (pattern) rule search: `pattern_search` walks the pattern-rule
//! database looking for a rule — or a chain of rules through intermediate
//! files — that can build a given target.
//!
//! Port of `implicit.c`.
//!
//! Slice 5 (`*mut`-to-handle): the search operates on [`FileId`] handles and the
//! owned [`Rule`] database ([`crate::execctx::ExecContext::rules`]). Targets are
//! identified by `FileId`, prerequisites are built as owned [`DepNode`]s on the
//! [`FileNode`], and candidate rules are referenced by index into the database
//! rather than by `*mut Rule`. No `*mut File`/`*mut Dep`/`*mut Commands`.

use crate::ar::ar_name_err;
use crate::dep::{DepFlags, DepNode};
use crate::dir::{file_exists_p, file_impossible, file_impossible_p};
pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{enter_file, lookup_file, FileId};
use crate::make_main::{db_level, stopchar_map};
use crate::misc::{print_spaces, skip_reference};
use crate::recipe::Recipe;
use crate::rule::{with_pattern_rules, with_pattern_rules_mut};
use crate::vpath::vpath_search;

/// `DB_IMPLICIT`: `-d` implicit-rule tracing enabled in `db_level`.
const DB_IMPLICIT: i32 = 0x8;
/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_NUL: i32 = 0x0001;
const MAP_BLANK: i32 = 0x0002;
const MAP_NEWLINE: i32 = 0x0004;
const MAP_PIPE: i32 = 0x0100;

/// `STOP_SET (c, mask)` from `makeint.h`.
fn stop_set(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
}

/// `DBS (DB_IMPLICIT, ...)`: print an indented trace line when implicit-rule
/// debugging is enabled. Takes the depth and a preformatted byte string.
fn dbs(ctx: &crate::execctx::ExecContext, depth: u32, msg: &[u8]) {
    if DB_IMPLICIT & db_level(ctx) != 0 {
        print_spaces(depth);
        crate::output::trace_out(msg);
    }
}

/// String equality on byte slices (make's `streq`).
#[cfg(test)]
fn streq(a: &[u8], b: &[u8]) -> bool {
    a == b
}

pub const PATH_MAX: usize = 4096;
pub const GET_PATH_MAX: usize = PATH_MAX;

/// A prerequisite discovered while trying a pattern rule, together with the
/// intermediate file that would build it (if any) — the pointer-free form of
/// the c2rust `patdeps` (whose `file` was `*mut File`).
#[derive(Debug, Clone)]
struct PatDep {
    /// Resolved prerequisite name (raw bytes).
    name: Vec<u8>,
    /// The matched pattern that built it, when this dep is an intermediate.
    pattern: Option<Vec<u8>>,
    /// The intermediate file's `FileId`, if this dep is built as one.
    file: Option<FileId>,
    ignore_mtime: bool,
    ignore_automatic_vars: bool,
    is_explicit: bool,
    wait_here: bool,
}

impl PatDep {
    fn new(name: Vec<u8>) -> Self {
        PatDep {
            name,
            pattern: None,
            file: None,
            ignore_mtime: false,
            ignore_automatic_vars: false,
            is_explicit: false,
            wait_here: false,
        }
    }
}

/// A candidate pattern rule recorded during the first matching pass. `rule` is
/// an index into the pattern-rule database (the former `*mut Rule`); `None`
/// marks a candidate discarded during winnowing.
#[derive(Debug, Clone, Copy)]
struct TryRule {
    rule: Option<usize>,
    stemlen: usize,
    matches: u32,
    order: u32,
    checked_lastslash: bool,
}

/// Look up a file's `name` bytes (the `name` field of its [`FileNode`]).
fn file_name(ctx: &crate::execctx::ExecContext, id: FileId) -> Vec<u8> {
    ctx.filenodes
        .get(id)
        .map(|n| n.lock().expect("file node lock poisoned").name.clone())
        .unwrap_or_default()
}

/// Build a `DepNode` carrying a resolved name.
fn dep_with_name(name: Vec<u8>) -> DepNode {
    DepNode {
        name: String::from_utf8_lossy(&name).into_owned(),
        file: None,
        shuf: None,
        stem: None,
        flags: DepFlags::empty(),
        changed: false,
        ignore_mtime: false,
        static_pattern: false,
        needs_second_expansion: false,
        ignore_automatic_vars: false,
        is_explicit: false,
        wait_here: false,
    }
}

/// Whether `name` is impossible to make (wraps the name-based `file_impossible_p`).
fn is_impossible(
    ctx: &crate::execctx::ExecContext,
    name: &[u8],
) -> Result<bool, crate::build_result::BuildError> {
    let mut buf = name.to_vec();
    buf.push(0);
    // SAFETY: NUL-terminated name; `file_impossible_p` is name-based.
    unsafe { file_impossible_p(ctx, buf.as_ptr().cast()) }.map(|r| r != 0)
}

/// Whether `name` exists on disk (wraps the name-based `file_exists_p`).
fn exists(
    ctx: &crate::execctx::ExecContext,
    name: &[u8],
) -> Result<bool, crate::build_result::BuildError> {
    let mut buf = name.to_vec();
    buf.push(0);
    // SAFETY: NUL-terminated name; `file_exists_p` is name-based.
    unsafe { file_exists_p(ctx, buf.as_ptr().cast()) }.map(|r| r != 0)
}

/// Mark `name` impossible (wraps the name-based `file_impossible`).
fn mark_impossible(
    ctx: &crate::execctx::ExecContext,
    name: &[u8],
) -> Result<(), crate::build_result::BuildError> {
    let mut buf = name.to_vec();
    buf.push(0);
    // SAFETY: NUL-terminated name; `file_impossible` is name-based.
    unsafe { file_impossible(ctx, buf.as_ptr().cast()) }
}

/// VPATH search for `name`; returns the resolved name bytes if found.
fn vpath_lookup(
    ctx: &crate::execctx::ExecContext,
    name: &[u8],
) -> Result<Option<Vec<u8>>, crate::build_result::BuildError> {
    let mut buf = name.to_vec();
    buf.push(0);
    // SAFETY: NUL-terminated name; the out-params are all null (we only want the
    // resolved name). `vpath_search` is name-based.
    let p = unsafe {
        vpath_search(
            ctx,
            buf.as_ptr().cast(),
            ::core::ptr::null_mut::<uintmax_t>(),
            ::core::ptr::null_mut::<::core::ffi::c_uint>(),
            ::core::ptr::null_mut::<::core::ffi::c_uint>(),
        )
    }?;
    Ok(if p.is_null() {
        None
    } else {
        // SAFETY: non-null NUL-terminated string from vpath_search.
        Some(unsafe { ::core::ffi::CStr::from_ptr(p).to_bytes().to_vec() })
    })
}

/// Search the implicit-rule database for a rule that can build `file`,
/// retrying as an archive-member reference when the plain search fails.
/// Returns `true` when a rule was found and applied to `file`.
pub fn try_implicit_rule(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    depth: u32,
) -> Result<bool, crate::build_result::BuildError> {
    let name = file_name(ctx, file);
    let mut msg = b"Looking for an implicit rule for '".to_vec();
    msg.extend_from_slice(&name);
    msg.extend_from_slice(b"'.\n");
    dbs(ctx, depth, &msg);
    if pattern_search(ctx, file, 0, depth, 0, 0)? {
        return Ok(true);
    }
    let mut cname = name.clone();
    cname.push(0);
    // SAFETY: NUL-terminated.
    // `ar_name_err` rather than `ar_name`: this frame carries a `Result` since
    // #442, so the unsupported nested `archive((member))` form travels out
    // instead of ending the process.
    let is_ar = unsafe { ar_name_err(ctx, ::core::ffi::CStr::from_ptr(cname.as_ptr().cast()))? };
    if is_ar {
        return try_archive_member_rule(ctx, file, depth, &name);
    }
    Ok(false)
}

/// The archive-member half of [`try_implicit_rule`]: announce the
/// archive-member search, run it, and announce the miss if it finds nothing.
/// Split out from the caller so each half is one search with its own tracing,
/// and so the caller's complexity does not grow with the `Result` seam.
fn try_archive_member_rule(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    depth: u32,
    name: &[u8],
) -> Result<bool, crate::build_result::BuildError> {
    let mut msg = b"Looking for archive-member implicit rule for '".to_vec();
    msg.extend_from_slice(name);
    msg.extend_from_slice(b"'.\n");
    dbs(ctx, depth, &msg);
    if pattern_search(ctx, file, 1, depth, 0, 0)? {
        return Ok(true);
    }
    let mut msg = b"No archive-member implicit rule found for '".to_vec();
    msg.extend_from_slice(name);
    msg.extend_from_slice(b"'.\n");
    dbs(ctx, depth, &msg);
    Ok(false)
}

/// Scan past leading blanks to the next word of `bytes` starting at `from`,
/// stopping at an unquoted blank, `|`, or end (skipping `$(...)` references).
/// Returns `(start, len)` of the word, or `None` at end-of-input.
fn get_next_word(bytes: &[u8], from: usize) -> Option<(usize, usize)> {
    let n = bytes.len();
    let mut i = from;
    while i < n && stop_set(bytes[i], MAP_BLANK | MAP_NEWLINE) {
        i += 1;
    }
    let beg = i;
    if i >= n {
        return None;
    }
    let mut c = bytes[i];
    i += 1;
    loop {
        match c {
            0 | b' ' | b'\t' => {
                i -= 1;
                break;
            }
            b'$' => {
                let consumed = skip_reference(&bytes[i..]);
                i += consumed;
            }
            b'|' => break,
            _ => {}
        }
        c = if i < n { bytes[i] } else { 0 };
        i += 1;
    }
    Some((beg, i - beg))
}

/// A snapshot of a candidate rule's i-th target needed for matching.
struct RuleTarget {
    /// Target pattern bytes.
    bytes: Vec<u8>,
    /// Byte index of the `%`.
    percent: usize,
}

/// Snapshot the i-th target of database rule `ri`.
fn rule_target(ctx: &crate::execctx::ExecContext, ri: usize, ti: usize) -> RuleTarget {
    with_pattern_rules(ctx, |rules| {
        let r = &rules[ri];
        let bytes = r.targets[ti].clone();
        let percent = bytes
            .iter()
            .position(|&b| b == b'%')
            .expect("pattern rule target must contain a '%'");
        RuleTarget { bytes, percent }
    })
}

/// Parse a NUL-terminated dep buffer into owned `DepNode`s with `parse_file_seq`.
fn parse_deps(
    ctx: &crate::execctx::ExecContext,
    buf: &mut Vec<u8>,
    stopmap: i32,
    prefix: *const ::core::ffi::c_char,
    flags: i32,
) -> Result<Vec<DepNode>, crate::build_result::BuildError> {
    if buf.last() != Some(&0) {
        buf.push(0);
    }
    let mut p: *mut ::core::ffi::c_char = buf.as_mut_ptr().cast();
    // SAFETY: `parse_file_seq` reads through `p` to the NUL; `buf` is
    // NUL-terminated and lives for the call.
    let parsed =
        unsafe { crate::read::parse_file_seq(ctx, &raw mut p, 0, stopmap, prefix, flags) }?;
    Ok(parsed
        .into_iter()
        .map(|pn| {
            let mut d = dep_with_name(pn.name);
            d.wait_here = pn.wait;
            d
        })
        .collect())
}

/// `parse_file_seq` flags (see `dep.h`).
const PARSEFS_ONEWORD: i32 = 0x20;
const PARSEFS_WAIT: i32 = 0x40;

pub fn pattern_search(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    archive: i32,
    mut depth: u32,
    recursions: u32,
    allow_compat_rules: i32,
) -> Result<bool, crate::build_result::BuildError> {
    // The full target name, and the matching slice (inside the parens for an
    // archive member reference).
    let full_name = file_name(ctx, file);
    let (name, name_off): (Vec<u8>, usize) = if archive != 0 {
        match full_name.iter().position(|&b| b == b'(') {
            Some(pos) => (full_name[pos..].to_vec(), pos),
            None => (full_name.clone(), 0),
        }
    } else {
        (full_name.clone(), 0)
    };
    let _ = name_off;
    let namelen = name.len();

    let mut max_deps = ctx.max_pattern_deps.get();
    let mut deplist: Vec<PatDep> = Vec::with_capacity(max_deps as usize);
    let mut depname: Vec<u8> = Vec::new();
    let mut stem_off: usize;
    let mut stemlen: usize;

    let mut tryrules: Vec<TryRule> = Vec::new();
    let mut specific_rule_matched = false;
    let mut ri: usize = 0;
    let mut found_compat_rule = false;
    let mut found_rule_idx: Option<usize> = None;
    let mut pathdir: Vec<u8> = Vec::new();
    let mut stem_str: Vec<u8> = vec![0u8; PATH_MAX + 1];
    depth = depth.wrapping_add(1);

    // An archive member name has no directory part.
    let mut cname = name.clone();
    cname.push(0);
    // SAFETY: NUL-terminated.
    // `ar_name_err` rather than `ar_name`: this frame carries a `Result` since
    // #442, so the unsupported nested `archive((member))` form travels out
    // instead of ending the process.
    let is_ar = unsafe { ar_name_err(ctx, ::core::ffi::CStr::from_ptr(cname.as_ptr().cast()))? };
    let pathlen: usize = if archive != 0 || is_ar {
        0
    } else {
        name[..namelen.saturating_sub(1)]
            .iter()
            .rposition(|&b| b == b'/')
            .map_or(0, |slash| slash + 1)
    };

    // First pass: collect every pattern rule whose target matches the name.
    let nrules = with_pattern_rules(ctx, |r| r.len());
    for rule_idx in 0..nrules {
        let (has_deps_no_cmds, in_use, rnum, terminal, defn) = with_pattern_rules(ctx, |rules| {
            let r = &rules[rule_idx];
            (
                !r.deps.is_empty() && r.cmds.is_none(),
                r.in_use,
                r.num,
                r.terminal,
                None::<Vec<u8>>,
            )
        });
        let _ = defn;
        // A rule with prerequisites but no commands cannot be used directly.
        if has_deps_no_cmds {
            continue;
        }
        if in_use {
            let defn = rule_defn_of(ctx, rule_idx);
            let mut msg = b"Avoiding implicit rule recursion for rule '".to_vec();
            msg.extend_from_slice(&defn);
            msg.extend_from_slice(b"'.\n");
            dbs(ctx, depth, &msg);
            continue;
        }
        for ti in 0..rnum as usize {
            let tgt = rule_target(ctx, rule_idx, ti);
            let target = &tgt.bytes;
            let percent = tgt.percent;
            if recursions > 0 && target.len() == 1 && !terminal || target.len() > namelen {
                continue;
            }
            stem_off = percent;
            stemlen = namelen.wrapping_sub(target.len()).wrapping_add(1);
            let check_lastslash = pathlen > 0 && !target.contains(&b'/');
            if check_lastslash {
                if pathlen > stemlen {
                    continue;
                }
                stemlen -= pathlen;
                stem_off += pathlen;
            }
            let prefix_start = if check_lastslash { pathlen } else { 0 };
            if target[..percent] != name[prefix_start..stem_off] {
                continue;
            }
            if target[percent + 1..] != name[stem_off + stemlen..] {
                continue;
            }
            if target.len() > 1 {
                specific_rule_matched = true;
            }
            // A rule with neither dependencies nor commands exists solely to set
            // `specific_rule_matched` when its target matches (C implicit.c:
            // `if (rule->deps == 0 && rule->cmds == 0) continue;`). It must not
            // be recorded as a usable candidate, or it would shadow the chained
            // intermediate rule (e.g. a bare `%.out:` blocking `%.out: %.mid`).
            let deps_and_cmds_empty = with_pattern_rules(ctx, |rules| {
                let r = &rules[rule_idx];
                r.deps.is_empty() && r.cmds.is_none()
            });
            if deps_and_cmds_empty {
                continue;
            }
            tryrules.push(TryRule {
                rule: Some(rule_idx),
                matches: ti as u32,
                stemlen: stemlen + if check_lastslash { pathlen } else { 0 },
                order: tryrules.len() as u32,
                checked_lastslash: check_lastslash,
            });
        }
    }

    if tryrules.is_empty() {
        return finish_no_rule(
            ctx,
            depth,
            &full_name,
            found_compat_rule,
            file,
            archive,
            recursions,
            allow_compat_rules,
        );
    }

    // Shortest-stem (most specific) candidates first, stable by order.
    tryrules.sort_by_key(|tr| (tr.stemlen, tr.order));
    // If a specific rule matched, discard non-terminal match-anything ("%") rules.
    if specific_rule_matched {
        for tr in &mut tryrules {
            if let Some(idx) = tr.rule {
                let (terminal, has_one) = with_pattern_rules(ctx, |rules| {
                    let r = &rules[idx];
                    (r.terminal, r.lens.contains(&1))
                });
                if !terminal && has_one {
                    tr.rule = None;
                }
            }
        }
    }

    // Second pass: try each candidate.
    let mut file_vars_initialized = false;
    let mut intermed_ok: i32 = 0;
    let mut matched = false;
    'outer: while intermed_ok < 2 {
        deplist.clear();
        if intermed_ok != 0 {
            dbs(ctx, depth, b"Trying harder.\n");
        }
        ri = 0;
        while ri < tryrules.len() {
            let tr = tryrules[ri];
            let rule_idx = match tr.rule {
                Some(idx) => idx,
                None => {
                    ri += 1;
                    continue;
                }
            };
            let rule_terminal = with_pattern_rules(ctx, |r| r[rule_idx].terminal);
            if intermed_ok != 0 && rule_terminal {
                ri += 1;
                continue;
            }
            let mut failed = false;
            let mut deps_found: u32 = 0;
            let mut order_only = false;

            let tgt = rule_target(ctx, rule_idx, tr.matches as usize);
            let target = &tgt.bytes;
            let percent = tgt.percent;
            stem_off = percent;
            stemlen = namelen.wrapping_sub(target.len()).wrapping_add(1);
            let check_lastslash = tr.checked_lastslash;
            if check_lastslash {
                stem_off += pathlen;
                stemlen -= pathlen;
                if pathdir.is_empty() {
                    pathdir.extend_from_slice(&name[..pathlen]);
                    pathdir.push(0);
                }
            }
            {
                let defn = rule_defn_of(ctx, rule_idx);
                let mut msg = b"Trying pattern rule '".to_vec();
                msg.extend_from_slice(&defn);
                msg.extend_from_slice(b"' with stem '");
                msg.extend_from_slice(&name[stem_off..stem_off + stemlen]);
                msg.extend_from_slice(b"'.\n");
                dbs(ctx, depth, &msg);
            }
            if stemlen + if check_lastslash { pathlen } else { 0 } > GET_PATH_MAX {
                dbs(ctx, depth, b"Stem too long.\n");
                ri += 1;
                continue;
            }
            // Build the stem string.
            if !check_lastslash {
                stem_str[..stemlen].copy_from_slice(&name[stem_off..stem_off + stemlen]);
                stem_str[stemlen] = 0;
            } else {
                stem_str[..pathlen].copy_from_slice(&name[..pathlen]);
                stem_str[pathlen..pathlen + stemlen]
                    .copy_from_slice(&name[stem_off..stem_off + stemlen]);
                stem_str[pathlen + stemlen] = 0;
            }
            let no_deps = with_pattern_rules(ctx, |r| r[rule_idx].deps.is_empty());
            if no_deps {
                // A matching rule without prerequisites wins immediately.
                matched = true;
                found_rule_idx = Some(rule_idx);
                break 'outer;
            }
            with_pattern_rules_mut(ctx, |r| r[rule_idx].in_use = true);
            deplist.clear();

            // Snapshot the rule's deps for iteration (names + flags).
            let rule_deps = with_pattern_rules(ctx, |r| r[rule_idx].deps.clone());

            'deps: for dep in &rule_deps {
                let dep_name_bytes = dep.name.clone().into_bytes();
                let expanded: Vec<DepNode>;
                let mut is_explicit_default = 1;
                if !dep.needs_second_expansion {
                    // No second expansion: substitute the stem for '%' and parse.
                    depname.clear();
                    if let Some(cp) = dep_name_bytes.iter().position(|&b| b == b'%') {
                        if check_lastslash {
                            depname.extend_from_slice(&name[..pathlen]);
                        }
                        depname.extend_from_slice(&dep_name_bytes[..cp]);
                        depname.extend_from_slice(&name[stem_off..stem_off + stemlen]);
                        depname.extend_from_slice(&dep_name_bytes[cp + 1..]);
                        is_explicit_default = 0;
                    } else {
                        depname.extend_from_slice(&dep_name_bytes);
                    }
                    let mut buf = depname.clone();
                    let mut parsed = parse_deps(
                        ctx,
                        &mut buf,
                        MAP_NUL,
                        ::core::ptr::null(),
                        PARSEFS_ONEWORD | PARSEFS_WAIT,
                    )?;
                    for d in &mut parsed {
                        deps_found = deps_found.wrapping_add(1);
                        d.ignore_mtime = dep.ignore_mtime;
                        d.ignore_automatic_vars = dep.ignore_automatic_vars;
                        d.wait_here = d.wait_here || dep.wait_here;
                        d.is_explicit = is_explicit_default != 0;
                    }
                    expanded = parsed;
                } else {
                    // Second expansion. BOUNDARY: per-word `%`->`$*`/`$(*F)`
                    // substitution then `expand_string_for_file`/automatic-var
                    // setup require the still-legacy variable layer (slice owned
                    // by variable.rs / commands.rs). We perform the textual
                    // rewrite into `depname` here and call the pinned FileId-based
                    // forms; those callees are not yet converted, so this arm is
                    // expected not to type-check until they are.
                    expanded = second_expansion_deps(
                        ctx,
                        file,
                        dep,
                        &name,
                        stem_off,
                        stemlen,
                        check_lastslash,
                        pathlen,
                        &stem_str,
                        &pathdir,
                        &mut order_only,
                        &mut deps_found,
                        &mut file_vars_initialized,
                    )?;
                }
                if deps_found > max_deps {
                    let new_max = ctx.max_pattern_deps.get().max(deps_found);
                    ctx.max_pattern_deps.set(new_max);
                    max_deps = new_max;
                }

                // Check each expanded prerequisite for viability.
                for dr in &expanded {
                    let dr_name = dr.name.clone().into_bytes();
                    let is_rule = dr.name.as_bytes() == dep_name_bytes.as_slice();
                    if is_impossible(ctx, &dr_name)? {
                        let defn = rule_defn_of(ctx, rule_idx);
                        let mut msg = b"Rejecting rule '".to_vec();
                        msg.extend_from_slice(&defn);
                        msg.extend_from_slice(b"' due to impossible prerequisite '");
                        msg.extend_from_slice(&dr_name);
                        msg.extend_from_slice(b"'.\n");
                        dbs(ctx, depth, &msg);
                        tryrules[ri].rule = None;
                        failed = true;
                        break 'deps;
                    }
                    let mut pe = PatDep::new(dr_name.clone());
                    pe.ignore_mtime = dr.ignore_mtime;
                    pe.ignore_automatic_vars = dr.ignore_automatic_vars;
                    pe.wait_here = dr.wait_here;
                    pe.is_explicit = dr.is_explicit;
                    let _ = is_rule;

                    // Resolve the prerequisite's existing file (if any).
                    let df = lookup_file(ctx, &dr_name);
                    let mut df_is_explicit = false;
                    let mut df_is_target = false;
                    if let Some(dfid) = df {
                        if let Some(node) = ctx.filenodes.get(dfid) {
                            let mut n = node.lock().expect("file node lock poisoned");
                            df_is_explicit = n.is_explicit;
                            df_is_target = n.is_target;
                            if !n.is_explicit && !dr.is_explicit {
                                n.intermediate = true;
                            }
                        }
                    }
                    if df_is_explicit {
                        pe.is_explicit = true;
                    }

                    // "ought to exist" if it is an explicit target or a dep of
                    // our target.
                    let mut ought = df_is_target;
                    if !ought {
                        // Is it among our target's existing deps?
                        ought = ctx
                            .filenodes
                            .get(file)
                            .map(|n| {
                                n.lock()
                                    .expect("file node lock poisoned")
                                    .deps
                                    .iter()
                                    .any(|d| d.name.as_bytes() == dr_name.as_slice())
                            })
                            .unwrap_or(false);
                    }

                    if ought {
                        deplist.push(pe);
                        let mut msg = b"'".to_vec();
                        msg.extend_from_slice(&dr_name);
                        msg.extend_from_slice(b"' ought to exist.\n");
                        dbs(ctx, depth, &msg);
                    } else if exists(ctx, &dr_name)? {
                        deplist.push(pe);
                        let mut msg = b"Found '".to_vec();
                        msg.extend_from_slice(&dr_name);
                        msg.extend_from_slice(b"'.\n");
                        dbs(ctx, depth, &msg);
                    } else if df.is_some() && allow_compat_rules != 0 {
                        deplist.push(pe);
                        let mut msg = b"Using compatibility rule due to '".to_vec();
                        msg.extend_from_slice(&dr_name);
                        msg.extend_from_slice(b"'.\n");
                        dbs(ctx, depth, &msg);
                    } else {
                        if df.is_some() {
                            found_compat_rule = true;
                        }
                        if let Some(vname) = vpath_lookup(ctx, &dr_name)? {
                            let mut msg = b"Found prerequisite '".to_vec();
                            msg.extend_from_slice(&dr_name);
                            msg.extend_from_slice(b"' as VPATH '");
                            msg.extend_from_slice(&vname);
                            msg.extend_from_slice(b"'.\n");
                            dbs(ctx, depth, &msg);
                            deplist.push(pe);
                        } else {
                            // Last resort: recursively search for a rule chain
                            // that builds it as an intermediate file.
                            let mut found_intermediate = false;
                            if intermed_ok != 0 {
                                // Enter the intermediate as a real arena node and
                                // recurse on it.
                                let int_id = enter_file(ctx, &dr_name);
                                if pattern_search(
                                    ctx,
                                    int_id,
                                    0,
                                    depth,
                                    recursions.wrapping_add(1),
                                    allow_compat_rules,
                                )? {
                                    // The recursive search renamed the node to the
                                    // matched pattern; capture it as the dep's
                                    // pattern, then restore the node's concrete
                                    // name (C: `pat->pattern = int_file->name;
                                    // int_file->name = d->name;`). Without the
                                    // restore the intermediate's `$@`/`$*` would
                                    // expand to the literal `%`-pattern.
                                    let pat = file_name(ctx, int_id);
                                    pe.pattern = Some(pat);
                                    if let Some(node) = ctx.filenodes.get(int_id) {
                                        node.lock().expect("file node lock poisoned").name =
                                            dr_name.clone();
                                    }
                                    pe.file = Some(int_id);
                                    deplist.push(pe);
                                    found_intermediate = true;
                                } else {
                                    if df.is_none() {
                                        mark_impossible(ctx, &dr_name)?;
                                    }
                                }
                            }
                            if !found_intermediate {
                                let mut msg = b"Not found '".to_vec();
                                msg.extend_from_slice(&dr_name);
                                msg.extend_from_slice(b"'.\n");
                                dbs(ctx, depth, &msg);
                                failed = true;
                                break 'deps;
                            }
                        }
                    }
                }
            }
            with_pattern_rules_mut(ctx, |r| r[rule_idx].in_use = false);
            if !failed {
                matched = true;
                found_rule_idx = Some(rule_idx);
                break 'outer;
            }
            ri += 1;
        }
        intermed_ok += 1;
    }

    if !matched {
        return finish_no_rule(
            ctx,
            depth,
            &full_name,
            found_compat_rule,
            file,
            archive,
            recursions,
            allow_compat_rules,
        );
    }

    let found = found_rule_idx.expect("matched implies a found rule");
    let found_tr = tryrules[ri];

    // Recompute stem for the found rule (ri/found are aligned).
    let tgt = rule_target(ctx, found, found_tr.matches as usize);
    let target = &tgt.bytes;
    let percent = tgt.percent;
    stem_off = percent;
    stemlen = namelen.wrapping_sub(target.len()).wrapping_add(1);
    let check_lastslash = found_tr.checked_lastslash;
    if check_lastslash {
        stem_off += pathlen;
        stemlen -= pathlen;
    }

    // When recursing, give the file the matched target pattern as its name.
    if recursions > 0 {
        let pat = with_pattern_rules(ctx, |r| r[found].targets[found_tr.matches as usize].clone());
        if let Some(node) = ctx.filenodes.get(file) {
            node.lock().expect("file node lock poisoned").name = pat;
        }
    }

    // Walk recorded prerequisites in reverse, prepending each as a dep.
    while let Some(pe) = deplist.pop() {
        if let Some(int_id) = pe.file {
            // An intermediate file: merge the scratch node into the real one.
            // Since `int_id` is already a real arena node, mark its state.
            merge_intermediate(ctx, int_id, &pe);
        }
        // Build the new dep.
        let mut nd = dep_with_name(pe.name.clone());
        nd.ignore_mtime = pe.ignore_mtime;
        nd.is_explicit = pe.is_explicit;
        nd.ignore_automatic_vars = pe.ignore_automatic_vars;
        nd.wait_here = pe.wait_here;
        if recursions != 0 {
            // keep the name only.
            nd.file = None;
        } else {
            nd.file = Some(lookup_file(ctx, &pe.name).unwrap_or_else(|| enter_file(ctx, &pe.name)));
        }
        let rule_terminal = with_pattern_rules(ctx, |r| r[found].terminal);
        if pe.file.is_none() && rule_terminal {
            // A terminal rule's non-intermediate prerequisites must exist as-is.
            match nd.file {
                None => nd.changed = true,
                Some(fid) => {
                    if let Some(node) = ctx.filenodes.get(fid) {
                        node.lock().expect("file node lock poisoned").tried_implicit = true;
                    }
                }
            }
        }
        // Prepend to the target's deps.
        if let Some(node) = ctx.filenodes.get(file) {
            let mut n = node.lock().expect("file node lock poisoned");
            n.deps.insert(0, nd);
            n.was_shuffled = false;
        }
    }

    // Set the stem on the target.
    let stem_bytes: Vec<u8>;
    if !check_lastslash {
        stem_bytes = name[stem_off..stem_off + stemlen].to_vec();
    } else {
        let fullstemlen = pathlen + stemlen;
        let mut s = Vec::with_capacity(fullstemlen);
        s.extend_from_slice(&name[..pathlen]);
        s.extend_from_slice(&name[stem_off..stem_off + stemlen]);
        stem_bytes = s;
    }
    let stem_string = Some(String::from_utf8_lossy(&stem_bytes).into_owned());

    // Attach the recipe and stem, mark target, and record which rule won
    // (semantic id, so `in_use` search scratch doesn't perturb it).
    let found_recipe: Option<Recipe> = with_pattern_rules(ctx, |r| r[found].cmds.clone());
    let found_rule_id = with_pattern_rules(ctx, |r| crate::rule::RuleId::from(&r[found]));
    if let Some(node) = ctx.filenodes.get(file) {
        let mut n = node.lock().expect("file node lock poisoned");
        n.stem = stem_string.clone();
        n.recipe = found_recipe;
        n.is_target = true;
        n.matched_rule = Some(found_rule_id);
    }

    // Inherit .PRECIOUS / .NOTINTERMEDIATE from the target pattern file.
    let found_target =
        with_pattern_rules(ctx, |r| r[found].targets[found_tr.matches as usize].clone());
    let (pat_precious, pat_notint) = lookup_flags(ctx, &found_target);
    if let Some(node) = ctx.filenodes.get(file) {
        let mut n = node.lock().expect("file node lock poisoned");
        if pat_precious {
            n.precious = true;
        }
        if pat_notint || ctx.no_intermediates.get() {
            n.notintermediate = true;
        }
    }

    // A multi-target rule also makes the other targets.
    let rnum = with_pattern_rules(ctx, |r| r[found].num);
    if rnum > 1 {
        for ti in 0..rnum as usize {
            if ti == found_tr.matches as usize {
                continue;
            }
            let tgt = rule_target(ctx, found, ti);
            let mut nm: Vec<u8> = Vec::new();
            nm.extend_from_slice(&tgt.bytes[..tgt.percent]);
            nm.extend_from_slice(&stem_bytes);
            nm.extend_from_slice(&tgt.bytes[tgt.percent + 1..]);
            let other_id = enter_file(ctx, &nm);
            let (op, on) = lookup_flags(ctx, &tgt.bytes);
            if let Some(node) = ctx.filenodes.get(other_id) {
                let mut n = node.lock().expect("file node lock poisoned");
                if op {
                    n.precious = true;
                }
                if on || ctx.no_intermediates.get() {
                    n.notintermediate = true;
                }
                n.is_target = true;
                // The same match produced this peer output — provenance
                // applies to every target of a multi-target rule.
                n.matched_rule = Some(found_rule_id);
            }
            // Prepend to also_make.
            let mut nd = dep_with_name(nm);
            nd.file = Some(other_id);
            if let Some(node) = ctx.filenodes.get(file) {
                node.lock()
                    .expect("file node lock poisoned")
                    .also_make
                    .insert(0, nd);
            }
        }
    }

    depth = depth.wrapping_sub(1);
    let defn = rule_defn_of(ctx, found);
    let mut msg = b"Found implicit rule '".to_vec();
    msg.extend_from_slice(&defn);
    msg.extend_from_slice(b"' for '");
    msg.extend_from_slice(&full_name);
    msg.extend_from_slice(b"'.\n");
    dbs(ctx, depth, &msg);
    Ok(true)
}

/// Merge an intermediate file's discovered state into the real arena node.
fn merge_intermediate(ctx: &crate::execctx::ExecContext, id: FileId, pe: &PatDep) {
    // Resolve each dep's file and mark intermediate/target state.
    let dep_names: Vec<Vec<u8>>;
    {
        let node = match ctx.filenodes.get(id) {
            Some(n) => n,
            None => return,
        };
        let mut n = node.lock().expect("file node lock poisoned");
        n.is_target = true;
        n.is_explicit = n.is_explicit || pe.is_explicit;
        n.notintermediate = n.notintermediate || ctx.no_intermediates.get();
        n.intermediate = n.intermediate || (!n.is_explicit && !n.notintermediate);
        n.tried_implicit = true;
        dep_names = n.deps.iter().map(|d| d.name.clone().into_bytes()).collect();
    }
    // Inherit precious from the matched pattern owner.
    if let Some(pat) = &pe.pattern {
        let (precious, _) = lookup_flags(ctx, pat);
        if precious {
            if let Some(node) = ctx.filenodes.get(id) {
                node.lock().expect("file node lock poisoned").precious = true;
            }
        }
    }
    // Enter each dep's file and propagate `changed` into tried_implicit.
    let changed_flags: Vec<bool> = {
        match ctx.filenodes.get(id) {
            Some(node) => node
                .lock()
                .expect("file node lock poisoned")
                .deps
                .iter()
                .map(|d| d.changed)
                .collect(),
            None => Vec::new(),
        }
    };
    for (i, dn) in dep_names.iter().enumerate() {
        let fid = enter_file(ctx, dn);
        if let Some(node) = ctx.filenodes.get(id) {
            let mut n = node.lock().expect("file node lock poisoned");
            if let Some(d) = n.deps.get_mut(i) {
                d.file = Some(fid);
                d.name = String::new();
            }
        }
        if let Some(node) = ctx.filenodes.get(fid) {
            let mut fn_ = node.lock().expect("file node lock poisoned");
            fn_.tried_implicit =
                fn_.tried_implicit || changed_flags.get(i).copied().unwrap_or(false);
        }
    }
}

/// Look up `(precious, notintermediate)` of a file by name (default false).
fn lookup_flags(ctx: &crate::execctx::ExecContext, name: &[u8]) -> (bool, bool) {
    lookup_file(ctx, name)
        .and_then(|id| ctx.filenodes.get(id))
        .map(|n| {
            let g = n.lock().expect("file node lock poisoned");
            (g.precious, g.notintermediate)
        })
        .unwrap_or((false, false))
}

/// The cached printable definition of database rule `idx`.
fn rule_defn_of(ctx: &crate::execctx::ExecContext, idx: usize) -> Vec<u8> {
    with_pattern_rules_mut(ctx, |rules| rules[idx].rule_defn().to_vec())
}

/// Emit the "no rule found" trace, retrying once for a compatibility rule.
#[allow(clippy::too_many_arguments)]
fn finish_no_rule(
    ctx: &crate::execctx::ExecContext,
    depth: u32,
    full_name: &[u8],
    found_compat_rule: bool,
    file: FileId,
    archive: i32,
    recursions: u32,
    allow_compat_rules: i32,
) -> Result<bool, crate::build_result::BuildError> {
    let depth = depth.wrapping_sub(1);
    if found_compat_rule {
        let mut msg = b"Searching for a compatibility rule for '".to_vec();
        msg.extend_from_slice(full_name);
        msg.extend_from_slice(b"'.\n");
        dbs(ctx, depth, &msg);
        assert!(
            allow_compat_rules == 0,
            "compatibility-rule retry must not recurse"
        );
        return pattern_search(ctx, file, archive, depth, recursions, 1);
    }
    let mut msg = b"No implicit rule found for '".to_vec();
    msg.extend_from_slice(full_name);
    msg.extend_from_slice(b"'.\n");
    dbs(ctx, depth, &msg);
    Ok(false)
}

/// Second-expansion prerequisite expansion.
///
/// BOUNDARY: this performs the per-word `%`->`$*`/`$(*F)` textual rewrite (which
/// is pointer-free) but then needs the variable layer to install the automatic
/// variables and `expand_string_for_file` to expand the result — both still
/// legacy `*mut File`-based (owned by commands.rs/variable.rs). The calls below
/// are shaped per the pinned FileId-based signatures and will not type-check
/// until those callees are converted.
#[allow(clippy::too_many_arguments)]
fn second_expansion_deps(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    dep: &DepNode,
    _name: &[u8],
    stem_off: usize,
    stemlen: usize,
    check_lastslash: bool,
    pathlen: usize,
    stem_str: &[u8],
    pathdir: &[u8],
    order_only: &mut bool,
    deps_found: &mut u32,
    file_vars_initialized: &mut bool,
) -> Result<Vec<DepNode>, crate::build_result::BuildError> {
    let mut out: Vec<DepNode> = Vec::new();
    let bytes = dep.name.clone().into_bytes();
    let mut cursor = 0usize;
    loop {
        let (beg, len) = match get_next_word(&bytes, cursor) {
            Some(w) => w,
            None => break,
        };
        let word = &bytes[beg..beg + len];
        let end = beg + len;
        if !*order_only && word == b"|" {
            *order_only = true;
            cursor = end;
            continue;
        }
        let mut depname: Vec<u8> = Vec::new();
        let mut add_dir = false;
        let is_explicit: bool;
        match word.iter().position(|&b| b == b'%') {
            None => {
                depname.extend_from_slice(word);
                is_explicit = true;
            }
            Some(first_percent) => {
                is_explicit = false;
                let mut percent = first_percent;
                let mut start = 0usize;
                loop {
                    depname.extend_from_slice(&word[start..percent]);
                    if check_lastslash {
                        add_dir = true;
                        depname.extend_from_slice(b"$(*F)");
                    } else {
                        depname.extend_from_slice(b"$*");
                    }
                    start = percent + 1;
                    if start == word.len() {
                        break;
                    }
                    let mut scan = start;
                    while scan < word.len()
                        && !stop_set(word[scan], MAP_BLANK | MAP_NEWLINE | MAP_NUL)
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
        cursor = end;

        // The automatic variables ($*, $@, ...) must be in place before
        // expanding. BOUNDARY: the variable layer is still pointer-based.
        if !*file_vars_initialized {
            let stem_slice: &[u8] = {
                let nul = stem_str
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap_or(stem_str.len());
                &stem_str[..nul]
            };
            crate::variable::initialize_file_variables(ctx, file, 0)?;
            crate::commands::set_file_variables(ctx, file, Some(stem_slice))?;
            *file_vars_initialized = true;
        }

        // Expand the rewritten depname for this file. BOUNDARY: needs a
        // FileId-based `expand_string_for_file`.
        let mut buf = depname.clone();
        buf.push(0);
        let expanded = crate::expand::expand_string_for_file(ctx, &buf, file)?;

        // Parse the expanded result.
        let mut parsed_buf = expanded;
        if parsed_buf.last() != Some(&0) {
            parsed_buf.push(0);
        }
        let prefix: *const ::core::ffi::c_char = if add_dir {
            pathdir.as_ptr().cast()
        } else {
            ::core::ptr::null()
        };
        let stopmap = if *order_only { MAP_NUL } else { MAP_PIPE };
        let mut p: *mut ::core::ffi::c_char = parsed_buf.as_mut_ptr().cast();
        // SAFETY: NUL-terminated buffer for the call.
        let parsed = unsafe {
            crate::read::parse_file_seq(ctx, &raw mut p, 0, stopmap, prefix, PARSEFS_WAIT)
        }?;
        for pn in parsed {
            let mut d = dep_with_name(pn.name);
            d.wait_here = pn.wait;
            *deps_found = deps_found.wrapping_add(1);
            if *order_only {
                d.ignore_mtime = true;
            }
            d.is_explicit = is_explicit;
            out.push(d);
        }
        let _ = (stem_off, stemlen, pathlen);
    }
    Ok(out)
}

#[cfg(test)]
mod streq_tests {
    use super::streq;

    #[test]
    fn equality_matches_strcmp() {
        assert!(streq(b"foo.o", b"foo.o"));
        assert!(streq(b"", b""));
        assert!(!streq(b"foo.o", b"bar.o"));
        assert!(!streq(b"foo", b"foobar"));
        assert!(!streq(b"", b"x"));
    }
}

#[cfg(test)]
mod get_next_word_tests {
    use super::get_next_word;

    #[test]
    fn splits_words() {
        assert_eq!(get_next_word(b"  foo bar", 0), Some((2, 3)));
        assert_eq!(get_next_word(b"foo bar", 3), Some((4, 3)));
        assert_eq!(get_next_word(b"   ", 0), None);
        // A `|` (order-only separator) terminates the word but, matching the C
        // `get_next_word` (`case '|': goto done;` with no `--p`), the byte that
        // was already consumed is included in the returned length — so "a|" of
        // "a|b" is the word here.
        assert_eq!(get_next_word(b"a|b", 0), Some((0, 2)));
    }
}

#[cfg(test)]
mod try_implicit_rule_tests {
    //! Since #442 `try_implicit_rule` returns `Result<bool, BuildError>`: the
    //! per-target variable setup it reaches through `pattern_search` ->
    //! `second_expansion_deps` -> `initialize_file_variables` now propagates,
    //! and the `archive((member))` rejection from `ar_name_err` travels out of
    //! the archive-member arm instead of ending the process.

    use super::try_implicit_rule;
    use crate::build_result::BuildError;
    use crate::file::enter_file;
    use std::sync::Mutex;

    // The pattern-rule database and the file arena are process-wide.
    static IMPLICIT_LOCK: Mutex<()> = Mutex::new(());

    fn probe(name: &[u8]) -> Result<bool, BuildError> {
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        // SAFETY: fresh context; the global set is initialized once per probe.
        unsafe { crate::variable::init_hash_global_variable_set(&ctx) };
        let f = enter_file(&ctx, name);
        try_implicit_rule(&ctx, f, 0)
    }

    /// With no pattern rules defined, an ordinary target matches nothing. The
    /// answer is `Ok(false)`, not a bare `false` — the seam the later expander
    /// slices propagate through.
    #[test]
    fn plain_target_with_no_rules_finds_nothing() {
        let _g = IMPLICIT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!probe(b"tir_probe_plain").expect("search rejected"));
    }

    /// An `archive(member)` target takes the second, archive-member search
    /// arm. It also finds nothing here, but reaching it is what exercises the
    /// `ar_name_err(..)?` call that replaced the old exiting `ar_name`.
    #[test]
    fn archive_member_target_takes_the_archive_arm() {
        let _g = IMPLICIT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!probe(b"tir_probe_lib.a(member.o)").expect("search rejected"));
    }

    /// The nested `archive((member))` form is not supported. `ar_name` used to
    /// end the process on it; the error is now a value the caller can act on.
    #[test]
    fn nested_archive_member_is_rejected_rather_than_fatal() {
        let _g = IMPLICIT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert!(matches!(
            probe(b"tir_probe_lib.a((nested.o))"),
            Err(BuildError::Failure)
        ));
    }
}
