//! Recipe (command) handling: chopping recipes into lines, setting the
//! automatic variables (`$@`, `$<`, `$^`, ...), running a target's
//! commands, and cleaning up half-built targets on a fatal signal.
//!
//! Port of `commands.c`.

use crate::ar::{ar_member_date, ar_name_err};
use crate::dep::DepNode;
pub use crate::ffi_types::{pid_t, sig_atomic_t, size_t, time_t, uintmax_t};
use crate::file::{
    file_timestamp_cons, lookup_file, remove_intermediates, system_time_from_unix, CommandState,
    FileId, FileNode, UpdateStatus, VarOrigin, NONEXISTENT_MTIME, ORDINARY_MTIME_MIN,
};
use crate::floc::Floc;
use crate::job::{child, job_slots_used, new_job, reap_children};
use crate::load::unload_file;
use crate::make_main::{die_cleanup, one_shell, stopchar_map, temp_stdin_unlink};
use crate::misc::make_pid;
use crate::output::{error, exit_on_err, perror_with_name, FmtArg};
use crate::posixos::{jobserver_clear, osync_clear};
use crate::recipe::{Recipe, RecipeLine, RecipeLineFlags};
use crate::remake::notice_finished_file;
use crate::variable::{define_target_variable, initialize_file_variables};

use crate::execctx::ExecContext;

use ::core::ffi::{c_char, CStr};
use ::core::ptr::null;
use ::std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use std::sync::atomic::Ordering;

use libc::{
    __errno_location, exit, kill, signal, EINTR, ENOENT, SIGHUP, SIGINT, SIGQUIT, SIGTERM, SIG_DFL,
    S_IFMT, S_IFREG,
};

pub const MAKE_TROUBLE: i32 = 1;

/// Recipe-line flag bits stored in `commands.lines_flags`.
pub const COMMANDS_RECURSE: i32 = 1;
pub const COMMANDS_SILENT: i32 = 2;
pub const COMMANDS_NOERROR: i32 = 4;

pub const FILE_LIST_SEPARATOR: u8 = b' ';

/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_BLANK: i32 = 0x0002;
const MAP_NEWLINE: i32 = 0x0004;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
}

/// The bytes a dependency goes by — the idiomatic form of C make's
/// `dep_name(d)` macro (`dep.h`: `(d)->name ? (d)->name : (d)->file->name`).
///
/// A dep's own `name` is normally populated, but `merge_intermediate`
/// (implicit.rs) clears it for an intermediate prerequisite, relying on the
/// resolved `file` handle for identity — exactly as C make stores a null
/// `name` and falls back to `file->name`. So when `name` is empty we must fall
/// back to the dep's file node's name; otherwise `$<`/`$^`/`$?`/`$+`/`$|`
/// render empty for targets reached through a pattern→pattern intermediate
/// chain (e.g. the kernel's `vdso-image-%.c: $(obj)/vdso%.so` → `%.so` rule).
fn dep_name_bytes(ctx: &ExecContext, d: &DepNode) -> Vec<u8> {
    if !d.name.is_empty() {
        return d.name.clone().into_bytes();
    }
    if let Some(fid) = d.file {
        if let Some(node) = ctx.filenodes.get(fid) {
            return node.lock().expect("file node poisoned").name.clone();
        }
    }
    Vec::new()
}

/// NUL-terminate a name list that was built into `buf` by writing `len` bytes
/// (each entry followed by a `FILE_LIST_SEPARATOR`). When at least one byte was
/// written the trailing separator at `len - 1` is overwritten with NUL;
/// otherwise the list is empty and byte 0 is set to NUL. `buf` must hold at
/// least `len + 1` bytes (the callers always allocate the extra terminator
/// slot). Pure index math, no pointer arithmetic.
#[cfg(test)]
fn finish_list(buf: &mut [u8], len: usize) {
    if len > 0 {
        buf[len - 1] = 0;
    } else {
        buf[0] = 0;
    }
}

/// Split an archive reference `lib(member)` into its library and member byte
/// slices. The member excludes the trailing `)`. Pure: indexes the byte view,
/// with no pointer arithmetic. Returns `None` when there is no `(`.
fn split_archive_ref(name: &[u8]) -> Option<(&[u8], &[u8])> {
    let paren = name.iter().position(|&b| b == b'(')?;
    let lib = &name[..paren];
    // The member runs from just after '(' up to the trailing ')'.
    let member = &name[paren + 1..name.len().saturating_sub(1)];
    Some((lib, member))
}

/// Whether dependency `d` contributes to the automatic-variable lists
/// (`$+`, `$^`, `$?`, `$|`): deps awaiting second expansion and deps that
/// explicitly opt out are skipped everywhere those lists are built.
fn dep_uses_auto_vars(d: &DepNode) -> bool {
    !d.needs_second_expansion && !d.ignore_automatic_vars
}

/// The bytes naming a dependency as they appear in `$+`/`$^`/`$|`: for an
/// archive ref `lib(member)` only `member` (sans the trailing `)`), otherwise
/// the whole name. The returned slice borrows `name`'s storage.
fn autovar_dep_name<'a>(
    ctx: &crate::execctx::ExecContext,
    name: &'a [u8],
) -> Result<&'a [u8], crate::build_result::BuildError> {
    let mut c = name.to_vec();
    c.push(0);
    let is_ar = ar_name_err(
        ctx,
        CStr::from_bytes_with_nul(&c).expect("dep name has interior NUL"),
    )?;
    Ok(if is_ar {
        split_archive_ref(name)
            .expect("ar_name guarantees a lib(member) reference")
            .1
    } else {
        name
    })
}

/// Append a list entry `nm` to `buf`, followed by a `FILE_LIST_SEPARATOR`.
fn push_entry(buf: &mut Vec<u8>, nm: &[u8]) {
    buf.extend_from_slice(nm);
    buf.push(FILE_LIST_SEPARATOR);
}

/// Drop the trailing `FILE_LIST_SEPARATOR` left by `push_entry`, yielding the
/// idiomatic value: list entries joined by single separators, no terminator.
/// Mirrors the legacy `finish_list` (which overwrote the trailing separator
/// with a NUL string terminator).
fn trim_list(mut buf: Vec<u8>) -> Vec<u8> {
    if buf.last() == Some(&FILE_LIST_SEPARATOR) {
        buf.pop();
    }
    buf
}

/// Set the automatic variables (`$@`, `$<`, `$*`, `$%`, `$^`, `$+`, `$?`,
/// `$|`) on `file`, computing the stem first if needed. All values are built
/// as owned byte strings and attached to the `FileNode` as per-target
/// variables (no raw pointers, no `c_char`).
pub fn set_file_variables(
    ctx: &ExecContext,
    file: FileId,
    stem: Option<&[u8]>,
) -> Result<(), crate::build_result::BuildError> {
    let Some(node) = ctx.filenodes.get(file) else {
        return Ok(());
    };

    // Snapshot everything we need out of the node, then drop the guard before
    // doing any work that re-enters the arena (the `.SUFFIXES` lookup and the
    // `define_target_variable` upserts).
    let (name, deps, node_recipe_text, mut node_stem): (
        Vec<u8>,
        Vec<DepNode>,
        Option<Vec<u8>>,
        Option<Vec<u8>>,
    ) = {
        let guard = node.lock().expect("file node poisoned");
        (
            guard.name.clone(),
            guard.deps.clone(),
            guard.recipe.as_ref().map(|r| r.text.clone()),
            guard.stem.as_ref().map(|s| s.clone().into_bytes()),
        )
    };

    // For an archive member `lib(member)`, `$@` is `lib` and `$%` is `member`.
    let (at, percent): (Vec<u8>, Vec<u8>) = {
        let mut nm = name.clone();
        nm.push(0);
        let is_ar = ar_name_err(
            ctx,
            CStr::from_bytes_with_nul(&nm).expect("file name has interior NUL"),
        )?;
        if is_ar {
            let (lib, member) =
                split_archive_ref(&name).expect("ar_name guarantees a lib(member) reference");
            (lib.to_vec(), member.to_vec())
        } else {
            (name.clone(), Vec::new())
        }
    };

    // Resolve the stem: caller-provided, the node's recorded stem, or derived
    // by stripping a known suffix listed in `.SUFFIXES`.
    let star: Vec<u8> = if let Some(s) = stem {
        s.to_vec()
    } else if let Some(s) = node_stem.take() {
        s
    } else {
        let nm = autovar_dep_name(ctx, &name)?;
        let mut derived: Option<Vec<u8>> = None;
        if let Some(sid) = lookup_file(ctx, b".SUFFIXES") {
            if let Some(snode) = ctx.filenodes.get(sid) {
                let sdeps = snode.lock().expect("file node poisoned").deps.clone();
                for d in &sdeps {
                    let dn = dep_name_bytes(ctx, d);
                    if nm.len() > dn.len() && nm.ends_with(&dn) {
                        derived = Some(nm[..nm.len() - dn.len()].to_vec());
                        break;
                    }
                }
            }
        }
        let s = derived.unwrap_or_default();
        // Record the derived stem back on the node (mirrors `file.stem = stem`).
        node.lock().expect("file node poisoned").stem =
            Some(String::from_utf8_lossy(&s).into_owned());
        s
    };

    // `$<` is the first usable dependency, or `$@` when running default
    // commands (recipe identical to the `.DEFAULT` recipe).
    let mut less: Vec<u8> = Vec::new();
    for d in &deps {
        if !d.ignore_mtime && dep_uses_auto_vars(d) {
            less = dep_name_bytes(ctx, d);
            break;
        }
    }
    if let Some(ref rt) = node_recipe_text {
        let default_recipe = lookup_file(ctx, b".DEFAULT").and_then(|did| {
            ctx.filenodes.get(did).and_then(|n| {
                n.lock()
                    .expect("file node poisoned")
                    .recipe
                    .as_ref()
                    .map(|r| r.text.clone())
            })
        });
        if default_recipe.as_ref() == Some(rt) {
            less = at.clone();
        }
    }

    define_target_variable(ctx, file, b"<", &less, VarOrigin::Automatic);
    define_target_variable(ctx, file, b"*", &star, VarOrigin::Automatic);
    define_target_variable(ctx, file, b"@", &at, VarOrigin::Automatic);
    define_target_variable(ctx, file, b"%", &percent, VarOrigin::Automatic);

    // `$+`: every non-order-only usable dep, with repeats.
    let mut plus: Vec<u8> = Vec::new();
    for d in &deps {
        if !d.ignore_mtime && dep_uses_auto_vars(d) {
            push_entry(&mut plus, autovar_dep_name(ctx, &dep_name_bytes(ctx, d))?);
        }
    }
    define_target_variable(ctx, file, b"+", &trim_list(plus), VarOrigin::Automatic);

    // `$^`/`$?`/`$|` must not repeat names; dedupe through a map keyed by name,
    // promoting an order-only duplicate of a normal dep to normal on both.
    let mut canonical: FxHashMap<Box<[u8]>, usize> = FxHashMap::default();
    let mut ignore_mtime: Vec<bool> = deps.iter().map(|d| d.ignore_mtime).collect();
    for (i, d) in deps.iter().enumerate() {
        if dep_uses_auto_vars(d) {
            let key: Box<[u8]> = dep_name_bytes(ctx, d).into();
            match canonical.entry(key) {
                Entry::Vacant(slot) => {
                    slot.insert(i);
                }
                Entry::Occupied(slot) => {
                    let j = *slot.get();
                    if ignore_mtime[i] != ignore_mtime[j] {
                        ignore_mtime[i] = false;
                        ignore_mtime[j] = false;
                    }
                }
            }
        }
    }

    let mut caret: Vec<u8> = Vec::new();
    let mut qmark: Vec<u8> = Vec::new();
    let mut bar: Vec<u8> = Vec::new();
    for (i, d) in deps.iter().enumerate() {
        // Take only each name's canonical (first-inserted) dep.
        if dep_uses_auto_vars(d)
            && canonical.get(dep_name_bytes(ctx, d).as_slice()).copied() == Some(i)
        {
            let nm = autovar_dep_name(ctx, &dep_name_bytes(ctx, d))?.to_vec();
            if ignore_mtime[i] {
                push_entry(&mut bar, &nm);
            } else {
                push_entry(&mut caret, &nm);
                if d.changed || ctx.always_make_flag.get() {
                    push_entry(&mut qmark, &nm);
                }
            }
        }
    }

    define_target_variable(ctx, file, b"^", &trim_list(caret), VarOrigin::Automatic);
    define_target_variable(ctx, file, b"?", &trim_list(qmark), VarOrigin::Automatic);
    define_target_variable(ctx, file, b"|", &trim_list(bar), VarOrigin::Automatic);
    Ok(())
}

/// Split `recipe.text` into individual recipe lines (respecting
/// backslash-newline continuations) and record each line's `+`/`@`/`-` prefix
/// flags, populating `recipe.lines` and `recipe.any_recurse`. Idempotent:
/// recipes are chopped lazily, so a recipe whose `lines` is already populated
/// is left untouched.
pub fn chop_commands(ctx: &ExecContext, recipe: &mut Recipe) {
    // Recipes are chopped lazily; only do it once.
    if !recipe.lines.is_empty() {
        return;
    }

    let text = &recipe.text;
    let mut raw_lines: Vec<Vec<u8>> = Vec::new();
    if one_shell(ctx) {
        // .ONESHELL: the entire recipe is a single line (sans final newline).
        let mut line = text.clone();
        if line.last() == Some(&b'\n') {
            line.pop();
        }
        raw_lines.push(line);
    } else {
        let bytes = text.as_slice();
        let mut p = 0usize;
        while p < bytes.len() {
            // Find the end of this line: an unescaped newline (count the
            // backslashes preceding it) or the end of the recipe.
            let mut end = p;
            loop {
                match bytes[end..].iter().position(|&b| b == b'\n') {
                    None => {
                        end = bytes.len();
                        break;
                    }
                    Some(off) => end += off,
                }
                if !(end > p && bytes[end - 1] == b'\\') {
                    break;
                }
                // Count the run of backslashes; an even count means the newline
                // is not escaped.
                let mut backslash = true;
                let mut b = end as isize - 2;
                while b >= p as isize && bytes[b as usize] == b'\\' {
                    backslash = !backslash;
                    b -= 1;
                }
                if !backslash {
                    break;
                }
                end += 1;
            }
            raw_lines.push(bytes[p..end].to_vec());
            p = end;
            if p < bytes.len() {
                p += 1;
            }
        }
    }

    let mut any_recurse = false;
    for raw in raw_lines {
        let mut flags = RecipeLineFlags::empty();
        let mut i = 0usize;
        while i < raw.len()
            && (stop_set(raw[i], MAP_BLANK) || raw[i] == b'-' || raw[i] == b'@' || raw[i] == b'+')
        {
            match raw[i] {
                b'+' => flags |= RecipeLineFlags::RECURSE,
                b'@' => flags |= RecipeLineFlags::SILENT,
                b'-' => flags |= RecipeLineFlags::NOERROR,
                _ => {}
            }
            i += 1;
        }
        let body = &raw[i..];
        // A line invoking $(MAKE) recurses even without a `+` prefix.
        if !flags.contains(RecipeLineFlags::RECURSE)
            && (find_subslice(body, b"$(MAKE)").is_some()
                || find_subslice(body, b"${MAKE}").is_some())
        {
            flags |= RecipeLineFlags::RECURSE;
        }
        if flags.contains(RecipeLineFlags::RECURSE) {
            any_recurse = true;
        }
        recipe.lines.push(RecipeLine { text: raw, flags });
    }
    recipe.any_recurse = any_recurse;
}

/// Find the first occurrence of `needle` in `haystack` (substring search).
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

/// Run `file`'s commands: set up its variables and start a job, or mark it
/// finished immediately when the recipe is effectively empty.
/// Resolve a (possibly double-colon) entry within a locked head node: `0` is
/// the head itself, `i>=1` is `double_colon[i-1]`.
fn entry_node(guard: &mut FileNode, entry: usize) -> &mut FileNode {
    if entry == 0 {
        guard
    } else {
        &mut guard.double_colon[entry - 1]
    }
}

pub fn execute_file_commands(
    ctx: &ExecContext,
    file: FileId,
    entry: usize,
) -> Result<(), crate::build_result::BuildError> {
    let Some(node) = ctx.filenodes.get(file) else {
        return Ok(());
    };

    // A recipe of nothing but whitespace and `-`/`@`/`+` prefixes means there
    // is nothing to execute. Snapshot the entry's recipe text, loaded flag and
    // name (`entry` 0 = head, i>=1 = double_colon[i-1]).
    let (recipe_text, loaded, name): (Vec<u8>, bool, Vec<u8>) = {
        let mut guard = node.lock().expect("file node poisoned");
        let loaded = guard.loaded;
        let nm = guard.name.clone();
        let en = entry_node(&mut guard, entry);
        let text = en
            .recipe
            .as_ref()
            .expect("execute_file_commands requires a recipe")
            .text
            .clone();
        (text, loaded, nm)
    };

    let empty = recipe_text
        .iter()
        .all(|&c| stop_set(c, MAP_BLANK | MAP_NEWLINE) || c == b'-' || c == b'@' || c == b'+');
    if empty {
        {
            let mut guard = node.lock().expect("file node poisoned");
            let en = entry_node(&mut guard, entry);
            en.command_state = CommandState::Running;
            en.update_status = UpdateStatus::Success;
        }
        notice_finished_file(ctx, file, entry)?;
        return Ok(());
    }

    initialize_file_variables(ctx, file, 0)?;
    let stem = {
        let mut guard = node.lock().expect("file node poisoned");
        entry_node(&mut guard, entry)
            .stem
            .as_ref()
            .map(|s| s.clone().into_bytes())
    };
    set_file_variables(ctx, file, stem.as_deref())?;

    // A loaded dynamic object being rebuilt must be unloaded first.
    if loaded {
        let mut nm = name.clone();
        nm.push(0);
        // SAFETY: `unload_file` still takes a `*const c_char`; the buffer lives
        // for the call.
        if unload_file(ctx, nm.as_ptr() as *const c_char) == 0 {
            let mut guard = node.lock().expect("file node poisoned");
            guard.loaded = false;
            guard.unloaded = true;
        }
    }

    // SAFETY: `new_job` enters the job machinery, which is still the c2rust
    // pointer-based scheduler; `file` is a valid arena handle.
    unsafe { new_job(ctx, file, entry) }
}

/// Read whether a fatal signal is currently being handled; checked by code
/// that must not re-enter (e.g. output sync teardown). Was a c2rust
/// `static mut sig_atomic_t`, then a process-wide atomic; now lives on
/// `ExecContext` (see [`crate::execctx::ExecContext::handling_fatal_signal`])
/// so a future multi-tenant host keeps each session's fatal-signal state
/// separate. `Relaxed` matches the original ordering (one write from the
/// signal handler, plain reads elsewhere, never reset since the process is
/// about to die) with no synchronization cost.
#[inline]
pub fn handling_fatal_signal(ctx: &ExecContext) -> bool {
    ctx.handling_fatal_signal.0.load(Ordering::Relaxed)
}

/// Copy the live context's fatal-signal mask onto `ctx`, the throwaway
/// context `fatal_error_signal` hands its cleanup helpers. The mask is NOT
/// cosmetic: `reap_children` blocks the trapped fatal signals around its
/// child-list mutations, and a default context's empty set would block
/// nothing (Codex review on #467). The live mask — built by
/// `install_fatal_signal` during startup — is reached through the `CTX_PTR`
/// borrow channel, like the handler's temp-stdin and intermediates cleanup;
/// in bare unit tests with no installed context the empty set stands.
fn adopt_live_fatal_signal_mask(ctx: &ExecContext) {
    if let Some(mask) =
        crate::make_main::try_with_exec_context(|live_ctx| live_ctx.fatal_signal_set.0.get())
    {
        ctx.fatal_signal_set.0.set(mask);
    }
}

/// Handle a fatal signal: kill children, delete half-built targets, then
/// re-raise the signal with the default disposition.
///
/// # Safety
///
/// Only callable as a signal handler (or from one); touches global job
/// state.
pub unsafe extern "C" fn fatal_error_signal(sig: i32) {
    crate::make_main::try_with_exec_context(|live_ctx| {
        live_ctx
            .handling_fatal_signal
            .0
            .store(true, Ordering::Relaxed)
    });
    signal(sig, SIG_DFL);
    // This is a kernel-invoked signal handler: it cannot be passed the owned
    // `ExecContext`, so every stateful step below — the children chain, the
    // file table the target cleanup consults, temp-stdin, reaping — reaches
    // `main_0`'s live context through the `CTX_PTR` borrow channel. The
    // default (throwaway) context built here serves only the prefix-free
    // `kill` failure printer at the end; its own tables are empty (#468).
    let ctx = crate::execctx::ExecContext::default();
    adopt_live_fatal_signal_mask(&ctx);
    // The temp-stdin name lives on the *live* context (it is per-run cleanup
    // state, not part of the default/throwaway one) — reach it through the
    // CTX_PTR borrow channel like `remove_intermediates` below.
    crate::make_main::with_exec_context(|live_ctx| temp_stdin_unlink(live_ctx));
    osync_clear();
    jobserver_clear();

    let live_children = crate::make_main::with_exec_context(|live_ctx| live_ctx.children.0.get());

    if sig == SIGTERM {
        // Pass SIGTERM on to children right away so they die with us.
        let mut c = live_children;
        while !c.is_null() {
            if (*c).remote() == 0 && (*c).pid > 0 {
                kill((*c).pid, SIGTERM);
            }
            c = (*c).next;
        }
    }

    if sig == SIGTERM || sig == SIGINT || sig == SIGHUP || sig == SIGQUIT {
        let mut c = live_children;
        while !c.is_null() {
            if (*c).remote() != 0 && (*c).pid > 0 {
                crate::make_main::with_exec_context(|live_ctx| {
                    live_ctx.remote_backend.0.kill((*c).pid, sig)
                });
            }
            c = (*c).next;
        }
        // Delete the partially built targets on the *live* context: its file
        // table is the one the interrupted children were recorded in, so
        // `delete_child_targets` sees real mtimes to compare (#468).
        let mut c = live_children;
        while !c.is_null() {
            // Same boundary as the reaps below: this is a kernel-invoked
            // signal handler with no Rust frame to carry a `Result`.
            crate::make_main::with_exec_context(|live_ctx| delete_child_targets(live_ctx, c))
                .unwrap_or_else(|e| exit_on_err(e));
            c = (*c).next;
        }
        // Wait for them all to die before cleaning up. Reaping walks the live
        // children chain, so it too runs on the live context.
        //
        // `fatal_error_signal` is a signal handler: there is no Rust frame
        // between here and the interrupted code to carry a `Result`, and the
        // handler is already committed to tearing the run down, so a reap
        // failure bridges through `exit_on_err` rather than propagating
        // (#432 Phase B, #441).
        while crate::make_main::with_exec_context(job_slots_used) > 0 {
            crate::make_main::with_exec_context(|live_ctx| reap_children(live_ctx, 1, 0))
                .unwrap_or_else(|e| exit_on_err(e));
        }
    } else {
        while crate::make_main::with_exec_context(job_slots_used) > 0 {
            crate::make_main::with_exec_context(|live_ctx| reap_children(live_ctx, 1, 1))
                .unwrap_or_else(|e| exit_on_err(e));
        }
    }

    // Intermediate cleanup must consult the *live* file table, not this default
    // context. Reach `main_0`'s `ExecContext` through the `CTX_PTR` borrow
    // channel; `remove_intermediates` `try_borrow`s the table so an async signal
    // that interrupted a `borrow_mut` skips cleanup rather than panicking.
    crate::make_main::with_exec_context(|live_ctx| remove_intermediates(live_ctx, 1));

    if sig == SIGQUIT {
        exit(MAKE_TROUBLE);
    }

    // Re-raise with the default handler so our exit status reflects the
    // signal.
    if kill(make_pid(), sig) < 0 {
        // Prefix-free error path: we are in a kernel-invoked signal handler and
        // must not route through a `ctx`-taking printer. Write the bare
        // diagnostic (no `make[N]:` prefix) straight to stderr, then die.
        let err = libc::strerror(*__errno_location());
        let msg = format!(
            "kill: {}\n",
            std::ffi::CStr::from_ptr(err).to_string_lossy()
        );
        let mut bytes = msg.into_bytes();
        bytes.push(0);
        crate::output::outputs(&ctx, 1, bytes.as_ptr() as *const ::core::ffi::c_char);
        // The end-of-run cleanup reaps the children chain and unwinds run
        // state, so it must run on the live context (the throwaway one has an
        // empty chain and would spin waiting for job slots that never free).
        // This is a kernel-invoked signal handler with nowhere to propagate a
        // `Result` to, so it bridges through `exit_on_err` — the sanctioned
        // stand-in for the retired diverging `die` (#432 Phase B, #440).
        crate::make_main::with_exec_context(|live_ctx| die_cleanup(live_ctx, MAKE_TROUBLE));
        exit_on_err(crate::build_result::BuildError::Trouble);
    }
}

/// Delete `file` if it exists and was modified since make last recorded its
/// timestamp (i.e. it is a half-finished build product). `on_behalf_of` is the
/// sibling target whose rule also builds this one (`None` for the direct
/// target).
fn delete_target(
    ctx: &ExecContext,
    file: FileId,
    on_behalf_of: Option<&[u8]>,
) -> Result<(), crate::build_result::BuildError> {
    let Some(node) = ctx.filenodes.get(file) else {
        return Ok(());
    };
    let (name, precious, phony, last_mtime) = {
        let guard = node.lock().expect("file node poisoned");
        (
            guard.name.clone(),
            guard.precious,
            guard.phony,
            guard.last_mtime,
        )
    };
    if precious || phony {
        return Ok(());
    }

    // NUL-terminated buffers for the libc/printf calls.
    let mut name_c = name.clone();
    name_c.push(0);
    let name_ptr = name_c.as_ptr() as *const c_char;
    let behalf_c = on_behalf_of.map(|b| {
        let mut v = b.to_vec();
        v.push(0);
        v
    });
    let behalf_ptr = behalf_c
        .as_ref()
        .map_or(null::<c_char>(), |v| v.as_ptr() as *const c_char);

    // SAFETY: all pointers below come from buffers that outlive the calls; the
    // libc stat/unlink and the `error`/`ar_*` printers are still pointer-based.
    unsafe {
        // An archive member can't be unlinked; just warn if it looks touched.
        if ar_name_err(
            ctx,
            CStr::from_bytes_with_nul(&name_c).expect("file name has interior NUL"),
        )? {
            let file_date: time_t = if last_mtime == NONEXISTENT_MTIME as uintmax_t {
                -1
            } else {
                (last_mtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    >> if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }) as time_t
            };
            if ar_member_date(ctx, name_ptr)? != file_date {
                if !behalf_ptr.is_null() {
                    error(
                        ctx,
                        null::<Floc>(),
                        0,
                        c"*** [%s] archive member '%s' may be bogus; not deleted".as_ptr(),
                        &[FmtArg::Str(behalf_ptr), FmtArg::Str(name_ptr)],
                    );
                } else {
                    error(
                        ctx,
                        null::<Floc>(),
                        0,
                        c"*** archive member '%s' may be bogus; not deleted".as_ptr(),
                        &[FmtArg::Str(name_ptr)],
                    );
                }
            }
            return Ok(());
        }

        let mut st: libc::stat = ::core::mem::zeroed();
        let mut e: i32;
        loop {
            e = libc::stat(name_ptr, &mut st);
            if !(e == -1 && *__errno_location() == EINTR) {
                break;
            }
        }
        if e == 0
            && st.st_mode & S_IFMT == S_IFREG
            && file_timestamp_cons(
                ctx,
                name_ptr,
                system_time_from_unix(st.st_mtime as i64, st.st_mtime_nsec as u32),
            ) != last_mtime
        {
            if !behalf_ptr.is_null() {
                error(
                    ctx,
                    null::<Floc>(),
                    0,
                    c"*** [%s] deleting file '%s'".as_ptr(),
                    &[FmtArg::Str(behalf_ptr), FmtArg::Str(name_ptr)],
                );
            } else {
                error(
                    ctx,
                    null::<Floc>(),
                    0,
                    c"*** deleting file '%s'".as_ptr(),
                    &[FmtArg::Str(name_ptr)],
                );
            }
            if crate::misc::unlink_c(name_ptr) < 0 && *__errno_location() != ENOENT {
                perror_with_name(ctx, c"unlink: ".as_ptr(), name_ptr);
            }
        }
        Ok(())
    }
}

/// Delete the targets of `child` (and everything its rule also makes) if they
/// might be incompletely built.
///
/// # Safety
///
/// `child` must be a valid child record.
pub unsafe fn delete_child_targets(
    ctx: &ExecContext,
    child: *mut child,
) -> Result<(), crate::build_result::BuildError> {
    if (*child).deleted() != 0 || (*child).pid < 0 {
        return Ok(());
    }
    let cf = (*child).file;
    delete_target(ctx, cf, None)?;
    // Each sibling the rule also makes is deleted on behalf of this target.
    let (cf_name, also) = {
        match ctx.filenodes.get(cf) {
            Some(node) => {
                let g = node.lock().expect("file node poisoned");
                (g.name.clone(), g.also_make.clone())
            }
            None => (Vec::new(), Vec::new()),
        }
    };
    for d in &also {
        if let Some(did) = d.file {
            delete_target(ctx, did, Some(&cf_name))?;
        }
    }
    (*child).set_deleted(1);
    Ok(())
}

/// Print `recipe` for `make -p`, one line per recipe line with the command
/// prefix.
pub fn print_commands(ctx: &ExecContext, recipe: &Recipe) {
    // SAFETY: stdout/printf/fputs are the C stdio handles; the format buffers
    match &recipe.defined_in {
        None => crate::output::trace_out(b"#  recipe to execute (built-in):\n"),
        Some(filenm) => crate::output::trace_parts(&[
            b"#  recipe to execute (from '",
            filenm,
            b"', line ",
            (recipe.defined_lineno as ::core::ffi::c_ulong)
                .to_string()
                .as_bytes(),
            b"):\n",
        ]),
    }

    let prefix = crate::make_main::opt_cmd_prefix(ctx) as u8;
    print_recipe_lines(recipe.text.as_slice(), prefix);
}

/// Print each recipe line prefixed with the recipe-prefix character. A line ends
/// at an unescaped newline; the raw text is walked exactly as the c2rust version
/// did so the printed output is byte-identical.
fn print_recipe_lines(bytes: &[u8], prefix: u8) {
    let mut s = 0usize;
    while s < bytes.len() {
        let mut end = s;
        let mut bs = false;
        while end < bytes.len() {
            if bytes[end] == b'\n' && !bs {
                break;
            }
            bs = if bytes[end] == b'\\' { !bs } else { false };
            end += 1;
        }
        crate::output::trace_parts(&[&[prefix], &bytes[s..end], b"\n"]);
        s = end + (end < bytes.len()) as usize;
    }
}

pub const FILE_TIMESTAMP_HI_RES: i32 = 1;

#[cfg(test)]
mod adopt_live_fatal_signal_mask_tests {
    //! `fatal_error_signal`'s throwaway context must carry the *live* fatal
    //! mask so `reap_children`'s block/unblock calls mask the real set during
    //! fatal cleanup (#467 review). Both arms: live context installed (mask
    //! copied) and absent (empty set stands).

    #[test]
    fn copies_live_mask_when_context_installed() {
        let _ctx = crate::make_main::install_default_exec_context_for_test();
        let _ctx = crate::make_main::install_default_exec_context_for_test();
        // Simulate `install_fatal_signal` adding SIGINT (bit 1 of word 0, as
        // `sigaddset(set, 2)` does on Linux) to the live context's mask.
        crate::make_main::with_exec_context(|live_ctx| {
            let mut set = live_ctx.fatal_signal_set.0.get();
            set.__val[0] |= 1 << 1;
            live_ctx.fatal_signal_set.0.set(set);
        });

        let ctx = crate::execctx::ExecContext::default();
        super::adopt_live_fatal_signal_mask(&ctx);
        assert_eq!(ctx.fatal_signal_set.0.get().__val[0] & (1 << 1), 1 << 1);
    }

    #[test]
    fn keeps_empty_mask_without_installed_context() {
        // No context installed on this test thread: the fallback arm runs and
        // the throwaway context keeps its empty set.
        let ctx = crate::execctx::ExecContext::default();
        super::adopt_live_fatal_signal_mask(&ctx);
        assert!(ctx.fatal_signal_set.0.get().__val.iter().all(|&w| w == 0));
    }
}

#[cfg(test)]
mod finish_list_unsafe_oracle {
    use ::core::ffi::c_char;

    /// Verbatim c2rust-era implementation, kept as a differential oracle.
    unsafe fn finish_list(start: *mut c_char, end: *mut c_char) {
        if end > start {
            *end.sub(1) = 0;
        } else {
            *end = 0;
        }
    }

    /// Drive both implementations over a buffer of `len` written bytes and
    /// assert the resulting byte arrays are identical.
    fn check(initial: &[u8], len: usize) {
        // Safe version: index a slice covering the written bytes plus the
        // terminator slot.
        let mut safe_buf = initial.to_vec();
        super::finish_list(&mut safe_buf, len);

        // Oracle: same buffer, pointer-walked. `end = start + len`, matching
        // each real call site's cursor span.
        let mut oracle_buf = initial.to_vec();
        unsafe {
            let start = oracle_buf.as_mut_ptr() as *mut c_char;
            // `start.add(len)` mirrors the original `cp`/`qp`/`bp` cursor: a
            // pointer `len` bytes past the buffer base.
            let end = start.add(len);
            finish_list(start, end);
        }

        assert_eq!(safe_buf, oracle_buf, "len={len} initial={initial:?}");
    }

    #[test]
    fn differential() {
        // Empty list: terminator goes at byte 0.
        check(&[0xff], 0);
        check(&[0xff, 0x80, b'x'], 0);
        // Non-empty: trailing separator at len-1 becomes NUL.
        check(b"foo \0", 4);
        check(b"a b c \0", 6);
        // High bytes / embedded NUL must survive untouched before len-1.
        check(&[0x80, 0xff, 0x00, b' ', 0xab], 4);
        check(&[b'x', 0x00, 0xff, b' '], 4);
        // Single written byte.
        check(&[b' ', 0xff], 1);
    }
}

#[cfg(test)]
mod split_archive_ref_tests {
    use super::split_archive_ref;

    #[test]
    fn splits_lib_and_member() {
        assert_eq!(
            split_archive_ref(b"libfoo.a(bar.o)"),
            Some((b"libfoo.a".as_slice(), b"bar.o".as_slice()))
        );
    }

    #[test]
    fn empty_library_part() {
        // "(member)" — nothing before the paren.
        assert_eq!(
            split_archive_ref(b"(bar.o)"),
            Some((b"".as_slice(), b"bar.o".as_slice()))
        );
    }

    #[test]
    fn empty_member_part() {
        // "lib()" — the member between '(' and ')' is empty.
        assert_eq!(
            split_archive_ref(b"lib()"),
            Some((b"lib".as_slice(), b"".as_slice()))
        );
    }

    #[test]
    fn no_paren_returns_none() {
        assert_eq!(split_archive_ref(b"plainfile.o"), None);
    }
}

#[cfg(test)]
mod autovar_dep_name_unsafe_oracle {
    //! `autovar_dep_name` is now a safe `fn` returning a `&[u8]` slice that
    //! reuses `split_archive_ref`. This keeps the verbatim c2rust-era pointer
    //! implementation as a differential oracle and asserts both yield identical
    //! bytes on plain names and `lib(member)` archive refs (AGENTS rule 3).
    use super::autovar_dep_name;
    use crate::ar::ar_name;
    use crate::execctx::ExecContext;
    use crate::ffi_types::size_t;
    use ::core::ffi::{c_char, CStr};
    use libc::{strchr, strlen};

    /// Verbatim c2rust-era implementation: returns a `(ptr, len)` borrowing
    /// `c`'s storage, reaching the archive member via `strchr(c, '(')+1`.
    unsafe fn oracle(ctx: &ExecContext, c: *const c_char) -> (*const c_char, size_t) {
        if ar_name(ctx, CStr::from_ptr(c)) {
            let inner = strchr(c, '(' as i32).add(1);
            (inner, strlen(inner) - 1)
        } else {
            (c, strlen(c))
        }
    }

    /// Drive both implementations over `input` and assert identical bytes.
    fn check(input: &CStr) {
        let ctx = ExecContext::default();
        let safe = autovar_dep_name(&ctx, input.to_bytes()).expect("plain name, no nested archive");
        // SAFETY: the oracle returns a pointer/len into `input`'s live storage.
        let from_oracle = unsafe {
            let (p, len) = oracle(&ctx, input.as_ptr());
            ::core::slice::from_raw_parts(p.cast::<u8>(), len as usize)
        };
        assert_eq!(safe, from_oracle, "input={input:?}");
    }

    #[test]
    fn differential() {
        // Plain names: the whole name is used verbatim.
        check(c"foo.o");
        check(c"a");
        check(c"path/to/file.c");
        // `lib(member)` archive refs: only `member`, sans the trailing ')'.
        check(c"libfoo.a(bar.o)");
        check(c"a(b)");
        check(c"x.a(m.o)");
        // Forms that *look* like archive refs but classify as Plain (paren at
        // index 0, or an empty member), so the whole name is returned.
        check(c"(bar.o)");
        check(c"lib()");
    }
}

#[cfg(test)]
mod hash_2_tests {
    //! The secondary-hash callbacks are constant-zero and never inspect
    //! their key pointer, so they are now safe `fn`s. Exercise each across
    //! the modules touched by this pass with both a null and a non-null key
    //! to confirm the pointer is ignored and the result is 0.
    use core::ffi::c_void;
    use core::ptr;

    #[test]
    fn secondary_hashes_are_zero_and_ignore_key() {
        let dummy = 0xdead_beef_usize as *const c_void;
        for key in [ptr::null::<c_void>(), dummy] {
            assert_eq!(crate::variable::variable_hash_2(key), 0);
        }
    }
}

#[cfg(test)]
mod dep_uses_auto_vars_tests {
    use super::*;

    /// Build a default [`DepNode`] and toggle the two flags that gate
    /// automatic-variable inclusion.
    fn dep_with(need_2nd: bool, ignore_auto: bool) -> DepNode {
        DepNode {
            name: String::new(),
            file: None,
            shuf: None,
            stem: None,
            flags: crate::dep::DepFlags::empty(),
            changed: false,
            ignore_mtime: false,
            static_pattern: false,
            needs_second_expansion: need_2nd,
            ignore_automatic_vars: ignore_auto,
            is_explicit: false,
            wait_here: false,
        }
    }

    /// A dep counts toward `$+`/`$^`/`$?`/`$|` only when it is neither awaiting
    /// second expansion nor explicitly opted out — the full truth table of the
    /// two gating flags.
    #[test]
    fn only_plain_deps_count() {
        assert!(dep_uses_auto_vars(&dep_with(false, false)));
        assert!(!dep_uses_auto_vars(&dep_with(true, false)));
        assert!(!dep_uses_auto_vars(&dep_with(false, true)));
        assert!(!dep_uses_auto_vars(&dep_with(true, true)));
    }
}
