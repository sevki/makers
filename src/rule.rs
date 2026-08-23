//! Pattern (implicit) rule database: the context-owned list of `%`-pattern
//! rules, conversion of old-style suffix rules into pattern rules, and the
//! `print_rule_data_base` report.
//!
//! Port of `rule.c`.
//!
//! Slice 5 (`*mut`-to-handle): the rule database is now an owned, pointer-free
//! structure. The global `pattern_rules`/`last_pattern_rule` `*mut rule` linked
//! list became a `RefCell<Vec<Rule>>` on the execution context
//! ([`crate::execctx::ExecContext::rules`]); each [`Rule`] owns its
//! target patterns (`Vec<Vec<u8>>`), prerequisites (`Vec<DepNode>`) and recipe
//! (`Option<Recipe>`). `suffix_file`/`default_file` are no longer `*mut File`
//! statics — the `.SUFFIXES` file is looked up by name through
//! `lookup_file`/`enter_file` (matching `read.rs::is_suffix_file`).

use crate::dep::DepNode;
pub use crate::ffi_types::{size_t, uintmax_t};
use crate::floc::Floc;
use crate::make_main::posix_pedantic;
use crate::recipe::Recipe;

use crate::dir::dir_file_exists_p;
use crate::file::lookup_file;

pub const RECIPEPREFIX_DEFAULT: u8 = b'\t';

/// An owned, pointer-free pattern rule.
///
/// The c2rust `rule` carried `*mut *const c_char targets`, `*mut Dep deps`,
/// `*mut Commands cmds` and a `*mut rule next` link. Here a rule owns its data:
/// targets are raw-byte patterns, `suffixes[i]` is the byte index just past the
/// `%` in `targets[i]`, deps are owned [`DepNode`]s and the recipe is an owned
/// [`Recipe`]. The whole database lives on the execution context
/// ([`crate::execctx::ExecContext::rules`]) as a `Vec<Rule>`, so there is no
/// `next` pointer and no `#[repr(C)]`/`Copy`.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Target patterns (raw bytes, no NUL), one per `num`.
    pub targets: Vec<Vec<u8>>,
    /// For each target, the byte index just past its `%` (the former
    /// `suffixes[i]` pointer, which pointed one past the `%`).
    pub suffixes: Vec<usize>,
    /// Length of each target pattern (the former `lens[i]`).
    pub lens: Vec<usize>,
    /// Prerequisites of the rule.
    pub deps: Vec<DepNode>,
    /// The rule's recipe, if any (the former `*mut Commands`).
    pub cmds: Option<Recipe>,
    /// Cached printable definition (e.g. `%.o: %.c`); computed on demand.
    pub defn: Option<Vec<u8>>,
    /// Number of targets.
    pub num: u16,
    pub terminal: bool,
    pub in_use: bool,
}

/// Semantic content hash for [`Rule`], the basis of [`RuleId`]. Only the
/// fields that define what the rule *is* participate: target patterns, deps,
/// recipe, and terminal-ness. Excluded: `defn` (a lazily computed print
/// cache), `in_use` (`pattern_search` recursion scratch), and
/// `suffixes`/`lens`/`num` (all derived from `targets`) — so a rule's id is
/// stable across matching and printing.
impl crate::content_hash::ContentHash for Rule {
    fn hash(&self, state: &mut impl crate::content_hash::DigestUpdate) {
        self.targets.hash(state);
        self.deps.hash(state);
        self.cmds.hash(state);
        self.terminal.hash(state);
    }
}

// Stable identity for a pattern rule: content-hash of its semantic fields (see
// the `ContentHash` impl above). Extends the blake3 identity family
// (`FileId`/`DepId`/`GoalDepId`) to the rule database; `depgraph` keys rule
// nodes by it.
crate::id_wireformat!(RuleId[crate::file::HASH_SIZE] <- Rule);

impl Rule {
    fn new() -> Self {
        Rule {
            targets: Vec::new(),
            suffixes: Vec::new(),
            lens: Vec::new(),
            deps: Vec::new(),
            cmds: None,
            defn: None,
            num: 0,
            terminal: false,
            in_use: false,
        }
    }

    /// The name a dep goes by inside a rule's printed definition: its own name.
    /// (Pattern-rule deps always carry a name; they are never bare file links.)
    fn dep_name(d: &DepNode) -> &[u8] {
        d.name.as_bytes()
    }

    /// Compute (and cache) the printable definition of this rule, e.g.
    /// `%.o: %.c`. Returns the cached bytes (without a trailing NUL).
    pub fn rule_defn(&mut self) -> &[u8] {
        if self.defn.is_none() {
            let mut buf: Vec<u8> = Vec::new();
            for k in 0..self.num as usize {
                if k > 0 {
                    buf.push(b' ');
                }
                buf.extend_from_slice(&self.targets[k]);
            }
            buf.push(b':');
            if self.terminal {
                buf.push(b':');
            }
            // Normal prerequisites first.
            for d in &self.deps {
                if !d.ignore_mtime {
                    if d.wait_here {
                        buf.extend_from_slice(b" .WAIT");
                    }
                    buf.push(b' ');
                    buf.extend_from_slice(Rule::dep_name(d));
                }
            }
            // Order-only prerequisites after a `|`.
            let mut sep: &[u8] = b" | ";
            for d in &self.deps {
                if d.ignore_mtime {
                    buf.extend_from_slice(sep);
                    if d.wait_here {
                        buf.extend_from_slice(b".WAIT ");
                    }
                    buf.extend_from_slice(Rule::dep_name(d));
                    sep = b" ";
                }
            }
            self.defn = Some(buf);
        }
        self.defn.as_ref().expect("just set").as_slice()
    }
}

/// Run `f` with a shared borrow of the pattern-rule database
/// ([`crate::execctx::ExecContext::rules`]).
pub fn with_pattern_rules<R>(ctx: &crate::execctx::ExecContext, f: impl FnOnce(&[Rule]) -> R) -> R {
    f(&ctx.rules.borrow())
}

/// Run `f` with a mutable borrow of the pattern-rule database
/// ([`crate::execctx::ExecContext::rules`]).
pub fn with_pattern_rules_mut<R>(
    ctx: &crate::execctx::ExecContext,
    f: impl FnOnce(&mut Vec<Rule>) -> R,
) -> R {
    f(&mut ctx.rules.borrow_mut())
}

/// The number of pattern rules currently installed.
pub fn num_rules(ctx: &crate::execctx::ExecContext) -> usize {
    ctx.rules.borrow().len()
}

/// Byte-for-byte equality of two byte slices (the C `streq` macro on names).
fn streq(a: &[u8], b: &[u8]) -> bool {
    a == b
}

/// Snap the implicit-rule database after reading all makefiles: count rules,
/// compute the various `max_pattern_*` statistics, mark deps whose directory
/// does not exist, and append `.EXTRA_PREREQS` to every rule's dep chain.
pub fn snap_implicit_rules(
    ctx: &crate::execctx::ExecContext,
) -> Result<(), crate::build_result::BuildError> {
    // `.EXTRA_PREREQS` expansion is a cross-file concern (`expand_extra_prereqs`
    // is still legacy `*mut`-based). The pointer-free port collects the extra
    // prereq names here; until that callee is converted this stays empty.
    // BOUNDARY: extra-prereqs expansion not yet ported to handles.
    let pre_deps: Vec<DepNode> = Vec::new();

    let mut dirname: Vec<u8> = Vec::new();
    let mut max_dep_len: usize = 0;
    for d in &pre_deps {
        let len = d.name.len();
        if len > max_dep_len {
            max_dep_len = len;
        }
    }
    ctx.max_pattern_dep_length.set(max_dep_len as size_t);
    let pre_ndeps = pre_deps.len() as ::core::ffi::c_uint;

    ctx.num_pattern_rules.set(0);
    ctx.max_pattern_targets.set(0);
    ctx.max_pattern_deps.set(0);

    // The closure the rule store hands out returns `()`, so the walk's verdict
    // is carried out in a local and re-raised once the borrow has been
    // released — the store must not stay borrowed across a `?`.
    let mut rejected = None;
    with_pattern_rules_mut(ctx, |rules| {
        for rr in rules.iter_mut() {
            let mut ndeps: ::core::ffi::c_uint = pre_ndeps;
            ctx.num_pattern_rules
                .set(ctx.num_pattern_rules.get().wrapping_add(1));
            if rr.num as ::core::ffi::c_uint > ctx.max_pattern_targets.get() {
                ctx.max_pattern_targets.set(rr.num as ::core::ffi::c_uint);
            }
            for d in rr.deps.iter_mut() {
                let dname = d.name.clone().into_bytes();
                let len = dname.len();
                ndeps = ndeps.wrapping_add(1);
                if len as size_t > ctx.max_pattern_dep_length.get() {
                    ctx.max_pattern_dep_length.set(len as size_t);
                }
                // Directory part containing '%': mark "changed" if the directory
                // prefix does not exist.
                let slash = dname.iter().rposition(|&b| b == b'/');
                let has_pct_in_dir = slash.map(|s| dname[s..].contains(&b'%')).unwrap_or(false);
                if has_pct_in_dir {
                    let mut p = slash.expect("slash present");
                    if p == 0 {
                        p += 1;
                    }
                    dirname.clear();
                    dirname.extend_from_slice(&dname[..p]);
                    dirname.push(0);
                    // SAFETY: `dir_file_exists_p` reads two NUL-terminated C
                    // strings; `dirname` is NUL-terminated and `c""` is empty.
                    let exists = match unsafe {
                        dir_file_exists_p(ctx, dirname.as_ptr().cast(), c"".as_ptr())
                    } {
                        Ok(e) => e,
                        Err(e) => {
                            rejected = Some(e);
                            return;
                        }
                    };
                    d.changed = exists == 0;
                } else {
                    d.changed = false;
                }
            }
            // Append the (empty for now) extra prereqs.
            rr.deps.extend(pre_deps.iter().cloned());
            if ndeps > ctx.max_pattern_deps.get() {
                ctx.max_pattern_deps.set(ndeps);
            }
        }
    });
    match rejected {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

/// Build a `%`-prefixed copy of `s` (e.g. `.c` becomes `%.c`).
fn percent_prefixed(s: &[u8]) -> Vec<u8> {
    let mut buf = vec![b'%'];
    buf.extend_from_slice(s);
    buf
}

/// Convert a single old-style suffix rule into a pattern rule.
///
/// `target`/`source` are the suffixes (without `%`); `None` means "absent".
/// A `None` target builds the archive-member pattern `(%.o)`.
fn convert_suffix_rule(
    ctx: &crate::execctx::ExecContext,
    target: Option<&[u8]>,
    source: Option<&[u8]>,
    cmds: Option<Recipe>,
) {
    let (name, percent) = suffix_rule_target(target);
    let deps = suffix_rule_source_deps(source);
    let targets = vec![name];
    let percents = vec![percent];
    create_pattern_rule(ctx, targets, percents, 1, false, deps, cmds, false);
}

/// The pattern target name and `%` offset for a suffix rule. `None` is the
/// archive-member special case `(%.o)` (percent at index 1); `Some(t)` becomes
/// a `%`-prefixed pattern (percent at index 0).
fn suffix_rule_target(target: Option<&[u8]>) -> (Vec<u8>, usize) {
    match target {
        None => (b"(%.o)".to_vec(), 1),
        Some(t) => (percent_prefixed(t), 0),
    }
}

/// The prerequisite deps for a suffix rule: empty for `None`, else a single
/// `%`-prefixed dep built from `source`.
fn suffix_rule_source_deps(source: Option<&[u8]>) -> Vec<DepNode> {
    match source {
        None => Vec::new(),
        Some(s) => vec![dep_with_name(percent_prefixed(s))],
    }
}

/// A fresh `DepNode` carrying just a name (the suffix-rule prerequisite form).
fn dep_with_name(name: Vec<u8>) -> DepNode {
    DepNode {
        name: String::from_utf8_lossy(&name).into_owned(),
        file: None,
        shuf: None,
        stem: None,
        flags: crate::dep::DepFlags::empty(),
        changed: false,
        ignore_mtime: false,
        static_pattern: false,
        needs_second_expansion: false,
        ignore_automatic_vars: false,
        is_explicit: false,
        wait_here: false,
    }
}

/// Decide whether a looked-up suffix-rule file (which has commands) should be
/// treated as a suffix rule, given whether it carries prerequisites. With no
/// prerequisites it always applies; with prerequisites it is skipped under
/// `--posix`, else a warning is issued (located at `fileinfo`) and it still
/// applies.
fn suffix_rule_applies(
    ctx: &crate::execctx::ExecContext,
    fileinfo: &Floc,
    has_prereqs: bool,
) -> bool {
    if !has_prereqs {
        return true;
    }
    if posix_pedantic(ctx) {
        return false;
    }
    crate::error!(
        ctx,
        Some(fileinfo),
        "warning: ignoring prerequisites on suffix rule definition"
    );
    true
}

/// Snapshot of a `.SUFFIXES` prerequisite needed to convert suffix rules: the
/// suffix name and the recipe carried by the suffix's own file (if any).
struct SuffixSnap {
    name: Vec<u8>,
    cmds: Option<Recipe>,
}

/// Convert old-style suffix rules (the prerequisites of `.SUFFIXES`) into
/// pattern rules. Name-based: `.SUFFIXES` is found via `lookup_file`.
pub fn convert_to_pattern(ctx: &crate::execctx::ExecContext) {
    // Snapshot the suffix list (names + each suffix file's recipe) without
    // holding the arena lock across other lookups.
    let suffixes: Vec<SuffixSnap> = {
        let sid = match lookup_file(ctx, b".SUFFIXES") {
            Some(id) => id,
            None => return,
        };
        let node = match ctx.filenodes.get(sid) {
            Some(n) => n,
            None => return,
        };
        let names: Vec<Vec<u8>> = {
            let n = node.lock().expect("file node lock poisoned");
            n.deps.iter().map(|d| d.name.clone().into_bytes()).collect()
        };
        // For each suffix name, the suffix file's own recipe.
        names
            .into_iter()
            .map(|name| {
                let cmds = lookup_file(ctx, &name)
                    .and_then(|id| ctx.filenodes.get(id))
                    .and_then(|n| n.lock().expect("file node lock poisoned").recipe.clone());
                SuffixSnap { name, cmds }
            })
            .collect()
    };

    for d in &suffixes {
        // A suffix by itself (".c") describes a rule making "%" from "%.c".
        convert_suffix_rule(ctx, Some(&d.name), None, None);
        if let Some(cmds) = &d.cmds {
            // The suffix's own commands make "%" from "%.<suffix>".
            convert_suffix_rule(ctx, None, Some(&d.name), Some(cmds.clone()));
        }
        // Single-suffix file ".c": if it exists with commands, mark suffix.
        apply_suffix_mark(ctx, &d.name);

        for d2 in &suffixes {
            // Skip the pairing of a suffix with itself.
            if d.name == d2.name {
                continue;
            }
            // ".tgt.src": concatenated rule name.
            let mut rulename = d.name.clone();
            rulename.extend_from_slice(&d2.name);
            // Look up the combined suffix-rule file and read its recipe + deps.
            let (has_prereqs, recipe, finfo) = match lookup_file(ctx, &rulename) {
                Some(id) => match ctx.filenodes.get(id) {
                    Some(node) => {
                        let n = node.lock().expect("file node lock poisoned");
                        let has = !n.deps.is_empty();
                        let rec = n.recipe.clone();
                        (has, rec, finfo_of(&n))
                    }
                    None => (false, None, null_floc()),
                },
                None => (false, None, null_floc()),
            };
            if let Some(rec) = recipe {
                // Under --posix, prerequisites on a suffix rule are silently
                // ignored (skip); otherwise warn and still convert the rule.
                if suffix_rule_applies(ctx, &finfo, has_prereqs) {
                    mark_suffix(ctx, &rulename);
                    // ".X.a" also describes "(%.o): %.X".
                    if d2.name.len() == 2 && d2.name[0] == b'.' && d2.name[1] == b'a' {
                        convert_suffix_rule(ctx, None, Some(&d.name), Some(rec.clone()));
                    }
                    convert_suffix_rule(ctx, Some(&d2.name), Some(&d.name), Some(rec));
                }
            }
        }
    }
}

/// A null/zeroed `Floc` (no source location). `Floc` is a `#[repr(C)]` c2rust
/// struct without `Default`; warnings emitted with this carry no `file:line:`.
fn null_floc() -> Floc {
    Floc {
        filenm: ::core::ptr::null(),
        lineno: 0,
        offset: 0,
    }
}

/// Build a `Floc` from a recipe's `defined_lineno`, for warnings.
fn finfo_of(n: &crate::file::FileNode) -> Floc {
    match &n.recipe {
        Some(r) => Floc {
            filenm: ::core::ptr::null(),
            lineno: r.defined_lineno,
            offset: 0,
        },
        None => null_floc(),
    }
}

/// Mark the single-suffix file `name` as a suffix rule if it has commands.
fn apply_suffix_mark(ctx: &crate::execctx::ExecContext, name: &[u8]) {
    if let Some(id) = lookup_file(ctx, name) {
        if let Some(node) = ctx.filenodes.get(id) {
            let mut n = node.lock().expect("file node lock poisoned");
            if n.recipe.is_some() {
                let has_prereqs = !n.deps.is_empty();
                let finfo = finfo_of(&n);
                if suffix_rule_applies(ctx, &finfo, has_prereqs) {
                    n.suffix = true;
                }
            }
        }
    }
}

/// Set the `suffix` flag on the file `name` (no recipe re-check; caller decided).
fn mark_suffix(ctx: &crate::execctx::ExecContext, name: &[u8]) {
    if let Some(id) = lookup_file(ctx, name) {
        if let Some(node) = ctx.filenodes.get(id) {
            node.lock().expect("file node lock poisoned").suffix = true;
        }
    }
}

/// Locate the index of the `%` in `target`, asserting it exists.
fn percent_index(target: &[u8]) -> usize {
    target
        .iter()
        .position(|&b| b == b'%')
        .expect("pattern rule target must contain a '%'")
}

/// Install `rule` into the pattern-rule database, replacing any rule with
/// identical targets and deps when `override_0` is set. Returns `true` if the
/// rule was installed, `false` if discarded as a non-overriding duplicate.
fn new_pattern_rule(ctx: &crate::execctx::ExecContext, mut rule: Rule, override_0: bool) -> bool {
    rule.in_use = false;
    rule.terminal = false;
    let dup = with_pattern_rules(ctx, |rules| {
        for (idx, rr) in rules.iter().enumerate() {
            for i in 0..rule.num as usize {
                // Compare the i-th new target against every existing target.
                let mut j = 0usize;
                while j < rr.num as usize {
                    if !streq(&rule.targets[i], &rr.targets[j]) {
                        break;
                    }
                    j += 1;
                }
                if j == rr.num as usize {
                    // All targets matched; compare the dep chains too.
                    if rule.deps.len() == rr.deps.len()
                        && rule
                            .deps
                            .iter()
                            .zip(rr.deps.iter())
                            .all(|(a, b)| streq(Rule::dep_name(a), Rule::dep_name(b)))
                    {
                        return Some(idx);
                    }
                }
            }
        }
        None
    });
    match dup {
        Some(idx) => {
            if override_0 {
                with_pattern_rules_mut(ctx, |rules| {
                    rules.remove(idx);
                    rules.push(rule);
                });
                true
            } else {
                false
            }
        }
        None => {
            with_pattern_rules_mut(ctx, |rules| rules.push(rule));
            true
        }
    }
}

/// Install an implicit pattern rule from a built-in spec.
///
/// `target` and `dep` are NUL-terminated byte patterns; `commands` is the
/// recipe text. This is the pointer-free form of the c2rust
/// `install_pattern_rule(p: *const pspec, terminal)`.
pub fn install_pattern_rule(
    ctx: &crate::execctx::ExecContext,
    target: &[u8],
    dep: &[u8],
    commands: &[u8],
    terminal: bool,
) -> Result<(), crate::build_result::BuildError> {
    // `map` rather than `?`: the parse's verdict is the whole function, so
    // threading it through keeps this frame branch-free.
    parse_dep_names(ctx, dep)
        .map(|deps| install_parsed_pattern_rule(ctx, target, deps, commands, terminal))
}

/// Install a pattern rule whose prerequisites have already been parsed —
/// the half of [`install_pattern_rule`] below the `~`-expanding parse.
fn install_parsed_pattern_rule(
    ctx: &crate::execctx::ExecContext,
    target: &[u8],
    deps: Vec<DepNode>,
    commands: &[u8],
    terminal: bool,
) {
    let mut rule = Rule::new();
    rule.num = 1;
    let target_v = target.to_vec();
    let percent = percent_index(&target_v);
    rule.lens.push(target_v.len());
    rule.suffixes.push(percent + 1);
    rule.targets.push(target_v);
    rule.deps = deps;

    let installed = {
        let dup = with_pattern_rules(ctx, |rules| {
            // Mirror new_pattern_rule's duplicate detection without consuming.
            rules.iter().enumerate().find_map(|(idx, rr)| {
                for i in 0..rule.num as usize {
                    let mut j = 0usize;
                    while j < rr.num as usize {
                        if !streq(&rule.targets[i], &rr.targets[j]) {
                            break;
                        }
                        j += 1;
                    }
                    if j == rr.num as usize
                        && rule.deps.len() == rr.deps.len()
                        && rule
                            .deps
                            .iter()
                            .zip(rr.deps.iter())
                            .all(|(a, b)| streq(Rule::dep_name(a), Rule::dep_name(b)))
                    {
                        return Some(idx);
                    }
                }
                None
            })
        });
        dup.is_none()
    };

    if installed {
        rule.terminal = terminal;
        rule.cmds = Some(Recipe {
            defined_in: None,
            defined_lineno: 0,
            text: commands.to_vec(),
            lines: Vec::new(),
            recipe_prefix: RECIPEPREFIX_DEFAULT,
            any_recurse: false,
        });
        with_pattern_rules_mut(ctx, |rules| rules.push(rule));
    }
}

/// Parse a (NUL-terminated or plain) prerequisite string into owned deps.
///
/// Wraps [`parse_file_seq`](crate::read::parse_file_seq), whose `~` expansion
/// can be refused; that refusal travels out rather than ending the process.
fn parse_dep_names(
    ctx: &crate::execctx::ExecContext,
    dep: &[u8],
) -> Result<Vec<DepNode>, crate::build_result::BuildError> {
    if dep.is_empty() {
        return Ok(Vec::new());
    }
    let mut buf: Vec<u8> = dep.to_vec();
    if buf.last() != Some(&0) {
        buf.push(0);
    }
    let mut p: *mut ::core::ffi::c_char = buf.as_mut_ptr().cast();
    // SAFETY: `parse_file_seq` reads through `p` until the NUL; `buf` is
    // NUL-terminated and lives for the call. MAP_NUL=1, PARSEFS_NONE=0.
    let parsed =
        unsafe { crate::read::parse_file_seq(ctx, &raw mut p, 0, 0x1, ::core::ptr::null(), 0) }?;
    Ok(parsed
        .into_iter()
        .map(|pn| {
            let mut d = dep_with_name(pn.name);
            d.wait_here = pn.wait;
            d
        })
        .collect())
}

/// Create a new pattern rule with `n` targets and install it.
///
/// Pointer-free pinned form: `targets` are owned byte patterns, `percents[i]`
/// is the byte index of the `%` in `targets[i]`, `deps`/`commands` are owned.
/// Ownership transfers to the rule database.
pub fn create_pattern_rule(
    ctx: &crate::execctx::ExecContext,
    targets: Vec<Vec<u8>>,
    percents: Vec<usize>,
    n: u16,
    terminal: bool,
    deps: Vec<DepNode>,
    commands: Option<Recipe>,
    override_0: bool,
) {
    let mut rule = Rule::new();
    rule.num = n;
    rule.cmds = commands;
    rule.deps = deps;
    rule.lens = targets.iter().map(|t| t.len()).collect();
    // `percents[i]` is the index OF the `%`; store the index just past it,
    // matching the c2rust `*suffix = suffix.add(1)`.
    rule.suffixes = percents.iter().map(|&p| p + 1).collect();
    rule.targets = targets;
    for t in &rule.targets {
        debug_assert!(t.contains(&b'%'), "pattern rule target must contain a '%'");
    }
    let want_terminal = terminal;
    if new_pattern_rule(ctx, rule, override_0) {
        // `new_pattern_rule` clears `terminal`; set it on the installed rule.
        with_pattern_rules_mut(ctx, |rules| {
            if let Some(last) = rules.last_mut() {
                last.terminal = want_terminal;
            }
        });
    }
}

/// Print rule `r`'s definition and commands to stdout (for `-p`).
fn print_rule(buf: &mut Vec<u8>, r: &mut Rule) {
    buf.extend_from_slice(r.rule_defn());
    buf.push(b'\n');
    if let Some(cmds) = &r.cmds {
        // Mirror `print_commands`: indent each recipe line under the rule.
        cmds.text.split(|&b| b == b'\n').for_each(|line| {
            buf.push(b'\t');
            buf.extend_from_slice(line);
            buf.push(b'\n');
        });
    }
}

/// Print the whole implicit-rule database to stdout (for `-p`).
pub fn print_rule_data_base(ctx: &crate::execctx::ExecContext) {
    use std::io::Write;
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(b"\n# Implicit Rules\n");
    let (rules_count, terminal) = with_pattern_rules_mut(ctx, |rules| {
        let mut terminal: u32 = 0;
        for r in rules.iter_mut() {
            buf.push(b'\n');
            print_rule(&mut buf, r);
            if r.terminal {
                terminal += 1;
            }
        }
        (rules.len() as u32, terminal)
    });
    if rules_count == 0 {
        buf.extend_from_slice(b"\n# No implicit rules.\n");
    } else {
        let pct = terminal as f64 / rules_count as f64 * 100.0;
        buf.extend_from_slice(
            format!(
                "\n# {} implicit rules, {} ({:.1}%) terminal.",
                rules_count, terminal, pct
            )
            .as_bytes(),
        );
    }
    // Flush explicitly: the count line above has no trailing newline (matching
    // the C oracle's printf), so Rust's line-buffered stdout would otherwise
    // hold it past the libc-printf sections that follow — and lose it entirely
    // when the run ends through libc `exit()` (fatal paths), which does not
    // flush Rust's buffer.
    let mut out = std::io::stdout();
    let _ = out.write_all(&buf);
    let _ = out.flush();
    let num_pattern_rules = ctx.num_pattern_rules.get();
    if num_pattern_rules != rules_count && num_pattern_rules != 0 {
        // INTERNAL consistency check (was a `fatal`).
        eprintln!(
            "INTERNAL: num_pattern_rules is wrong!  {} != {}",
            num_pattern_rules, rules_count
        );
    }
}

#[cfg(test)]
mod streq_tests {
    use super::streq;

    #[test]
    fn equal_and_unequal_strings() {
        assert!(streq(b"", b""));
        assert!(streq(b"%.o", b"%.o"));
        assert!(streq(b"target%pattern", b"target%pattern"));

        assert!(!streq(b"", b"x"));
        assert!(!streq(b"x", b""));
        assert!(!streq(b"%.o", b"%.c"));
        assert!(!streq(b"abc", b"abcd"));
        assert!(!streq(b"a", b"b"));
    }
}

#[cfg(test)]
mod percent_prefixed_tests {
    use super::percent_prefixed;

    #[test]
    fn prefixes_with_percent() {
        assert_eq!(percent_prefixed(b""), b"%");
        assert_eq!(percent_prefixed(b".c"), b"%.c");
        assert_eq!(percent_prefixed(b".o"), b"%.o");
        assert_eq!(percent_prefixed(b"foo.o"), b"%foo.o");
    }
}

#[cfg(test)]
mod rule_defn_tests {
    use super::*;

    fn dep(name: &str, ignore_mtime: bool, wait_here: bool) -> DepNode {
        let mut d = dep_with_name(name.as_bytes().to_vec());
        d.ignore_mtime = ignore_mtime;
        d.wait_here = wait_here;
        d
    }

    #[test]
    fn simple_rule() {
        let mut r = Rule::new();
        r.num = 1;
        r.targets.push(b"%.o".to_vec());
        r.deps.push(dep("%.c", false, false));
        assert_eq!(r.rule_defn(), b"%.o: %.c");
    }

    #[test]
    fn order_only_after_pipe() {
        let mut r = Rule::new();
        r.num = 1;
        r.targets.push(b"%.o".to_vec());
        r.deps.push(dep("%.c", false, false));
        r.deps.push(dep("dir", true, false));
        assert_eq!(r.rule_defn(), b"%.o: %.c | dir");
    }
}
