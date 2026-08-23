pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __syscall_slong_t, __time_t, __uid_t, off_t, size_t, ssize_t, time_t, uintmax_t,
};
use crate::file::{
    cs_deps_running, cs_finished, cs_not_started, cs_running, us_failed, us_none, us_question,
    us_success, CommandState, UpdateStatus, VariableSet, VariableSetList,
};
use crate::misc::{find_next_token, print_spaces};
use crate::output::FmtArg;
use crate::strcache::strcache_add;
use libc::{
    __errno_location, close, free, open, sprintf, strcmp, strcpy, strerror,
    strrchr,
};
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> i32;
    fn fstat(__fd: i32, __buf: *mut stat) -> i32;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn read(__fd: i32, __buf: *mut ::core::ffi::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
use crate::warning::{self, Action, Type};
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type HashTable = crate::hash::HashTable;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
use crate::floc::Floc;

use crate::ar::{ar_member_date, ar_name_err, ar_touch, ParsedArName};
use crate::commands::{chop_commands, execute_file_commands};
use crate::expand::{allocated_expand_variable, variable_buffer_output};
pub use crate::file::nameseq;
use crate::file::{
    enter_file, expand_deps, file_timestamp_cons, file_timestamp_now, lookup_file, rehash_file,
    rename_file, system_time_from_unix,
};
use crate::implicit::try_implicit_rule;
use crate::job::{reap_children, start_waiting_jobs};
use crate::make_main::{db_level, opt_rebuilding_makefiles, second_expansion};
use crate::output::{error, message, perror_with_name};
use crate::read::find_percent;
pub use crate::read::goaldep;
use crate::vpath::{gpath_search, vpath_search};
pub const __S_IFMT: i32 = 0o170000_i32;
pub const ENOENT: i32 = 2;
pub const EINTR: i32 = 4;
pub const ENOTDIR: i32 = 20;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2)
    .wrapping_add(1);
pub const CHAR_BIT: i32 = __CHAR_BIT__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PATH_MAX: i32 = 4096_i32;
pub const GET_PATH_MAX: i32 = PATH_MAX;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const RM_INCLUDED: i32 = (1) << 1;
pub const RM_DONTCARE: i32 = (1) << 2;
pub const UNKNOWN_MTIME: i32 = 0;
pub const NONEXISTENT_MTIME: i32 = 1;
pub const OLD_MTIME: i32 = 2;
pub const ORDINARY_MTIME_MIN: i32 = OLD_MTIME + 1;
// The updater cluster now threads `FileId` and reads/writes `FileNode` through
// the arena. The former module statics (`goal_list`/`goal_dep`/`dropped_list`)
// are replaced by owned, `FileId`-based state:
//   * `goal_dep` (the currently-processed goal, used by `show_goal_error`) and
//     `goal_list` (the makefile-remaking goal set) now live on `ExecContext`
//     (`ctx.goal_dep`/`ctx.goal_list`), not a thread-local or module static.
//   * `dropped_list` (the circular-dep drop bookkeeping) is dropped entirely:
//     deps removed for circularity are simply removed from the owning
//     `Vec<DepNode>` by index.
pub const DROPPED_LIST_INCR: i32 = 5;

use crate::dep::{DepFlags, DepNode, GoalDepNode};
use crate::file::{FileId, FileNode};
use crate::recipe::RecipeLineFlags;

/// Walk the `renamed` chain from `id` to the live node, collecting ids so no
/// `FileNode` guard is held across an arena lookup. Returns the final id (the
/// node with `renamed == None`), or `id` itself if it is not interned.
fn follow_renamed(ctx: &crate::execctx::ExecContext, id: FileId) -> FileId {
    let mut cur = id;
    loop {
        let next = {
            let Some(node) = ctx.filenodes.get(cur) else {
                return cur;
            };
            let n = node.lock().expect("file node lock poisoned");
            n.renamed
        };
        match next {
            Some(n) => cur = n,
            None => return cur,
        }
    }
}

/// FileId port of the free `set_command_state`: set `state` on `id` and bump any
/// also_make peer up to `state`. Lock discipline: also_make peer ids are
/// snapshotted under the head guard, the head guard is dropped, then each peer
/// is locked briefly on its own.
fn set_command_state_id(ctx: &crate::execctx::ExecContext, id: FileId, state: CommandState) {
    let peers: Vec<FileId> = {
        let Some(node) = ctx.filenodes.get(id) else {
            return;
        };
        let mut n = node.lock().expect("file node lock poisoned");
        n.command_state = state;
        n.also_make.iter().filter_map(|d| d.file).collect()
    };
    for pid in peers {
        if let Some(node) = ctx.filenodes.get(pid) {
            let mut n = node.lock().expect("file node lock poisoned");
            if state as u32 > n.command_state as u32 {
                n.command_state = state;
            }
        }
    }
}

/// NUL-terminate a name into an owned buffer for the C FFI helpers
/// (`name_mtime`, `ar_*`, `vpath_search`), which only read it during the call.
fn cname(name: &[u8]) -> Vec<u8> {
    let mut v = name.to_vec();
    v.push(0);
    v
}

/// The name (raw bytes) of the head node `id`, or empty if not interned.
fn node_name(ctx: &crate::execctx::ExecContext, id: FileId) -> Vec<u8> {
    ctx.filenodes
        .get(id)
        .map(|node| node.lock().expect("file node lock poisoned").name.clone())
        .unwrap_or_default()
}

/// The largest ordinary (existent, in-range) packed timestamp — the c2rust
/// `ORDINARY_MTIME_MAX` expression, extracted so the updater reads cleanly.
#[inline]
fn ordinary_mtime_max() -> uintmax_t {
    ((!(0_i32 as uintmax_t))
        .wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
            0_i32 as uintmax_t
        } else {
            !(0_i32 as uintmax_t)
                << (::core::mem::size_of::<uintmax_t>() as usize)
                    .wrapping_mul(8_usize)
                    .wrapping_sub(1_usize)
        })
        .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
        >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
        << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
    .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
    .wrapping_add((if FILE_TIMESTAMP_HI_RES != 0 { 1000000000_i32 } else { 1 }) as uintmax_t)
    .wrapping_sub(1 as uintmax_t)
}

/// `NEW_MTIME` — the "freshly built" sentinel the c2rust code wrote into
/// `last_mtime` for just-made files (`(uintmax_t) -1` minus the sign bit).
#[inline]
fn new_mtime() -> uintmax_t {
    (!(0_i32 as uintmax_t)).wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
        0_i32 as uintmax_t
    } else {
        !(0_i32 as uintmax_t)
            << (::core::mem::size_of::<uintmax_t>() as usize)
                .wrapping_mul(CHAR_BIT as usize)
                .wrapping_sub(1_usize)
    })
}

/// Read the debug level via the `make_main` accessor.
#[inline]
/// `-d` trace line `<pre><name><post>`, where `name` is a `cname`-style
/// NUL-terminated buffer (the NUL is dropped) — one printf `%s` site.
fn trace_name(pre: &[u8], name: &[u8], post: &[u8]) {
    crate::output::trace_parts(&[pre, &name[..name.len().saturating_sub(1)], post]);
}

fn dbg(ctx: &crate::execctx::ExecContext) -> i32 {
    db_level(ctx)
}

/// Whether `id` or any of its double-colon entries is running/deps-running.
/// Lock discipline: a single brief lock on the head node.
fn dep_chain_running(ctx: &crate::execctx::ExecContext, id: FileId) -> bool {
    let Some(node) = ctx.filenodes.get(id) else {
        return false;
    };
    let n = node.lock().expect("file node lock poisoned");
    let is_running = |s: CommandState| s == cs_running || s == cs_deps_running;
    if is_running(n.command_state) {
        return true;
    }
    n.double_colon.iter().any(|e| is_running(e.command_state))
}

/// FileId port of `check_also_make`: if the head's recipe ran and updated it,
/// warn for any grouped-target peer the recipe left missing.
///
/// Lock discipline: the head node is locked only to copy out
/// `last_mtime`/`mtime_before_update`/`name`/recipe-location and the also_make
/// peer ids; the guard is dropped before `name_mtime` (which re-enters the
/// arena) and before each peer is locked.
pub fn check_also_make(ctx: &crate::execctx::ExecContext, file: FileId) {
    let (mut mtime, mtime_before_update, name, recipe_floc, peers): (
        uintmax_t,
        uintmax_t,
        Vec<u8>,
        Option<Floc>,
        Vec<(Vec<u8>, FileId)>,
    ) = {
        let Some(node) = ctx.filenodes.get(file) else {
            return;
        };
        let n = node.lock().expect("file node lock poisoned");
        let floc = n.recipe.as_ref().map(|r| Floc {
            filenm: ::core::ptr::null(),
            lineno: r.defined_lineno,
            offset: 0,
        });
        let peers = n
            .also_make
            .iter()
            .filter_map(|d| d.file.map(|f| (d.name.clone().into_bytes(), f)))
            .collect();
        (n.last_mtime, n.mtime_before_update, n.name.clone(), floc, peers)
    };
    // lock: guard dropped before name_mtime / peer locks.
    if mtime == UNKNOWN_MTIME as uintmax_t {
        let cn = cname(&name);
        mtime = unsafe { name_mtime(ctx, cn.as_ptr() as *const ::core::ffi::c_char) };
    }
    if mtime >= ORDINARY_MTIME_MIN as uintmax_t
        && mtime <= ordinary_mtime_max()
        && mtime > mtime_before_update
    {
        for (peer_name, peer_id) in peers {
            let peer_mtime = {
                let Some(node) = ctx.filenodes.get(peer_id) else {
                    continue;
                };
                let n = node.lock().expect("file node lock poisoned");
                n.last_mtime
            };
            if peer_mtime == NONEXISTENT_MTIME as uintmax_t {
                let mut floc = recipe_floc.clone();
                let floc_ptr = floc
                    .as_mut()
                    .map_or(::core::ptr::null_mut::<Floc>(), |f| f as *mut Floc);
                let pcn = cname(&peer_name);
                unsafe {
                    error(
                        ctx,
                        floc_ptr,
                        0,
                        b"warning: pattern recipe did not update peer target '%s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(pcn.as_ptr() as *const ::core::ffi::c_char)],
                    );
                }
            }
        }
    }
}

/// FileId port of `update_goal_chain`. Takes the goal chain as an owned, mutable
/// `Vec<GoalDepNode>` (the c2rust `copy_dep_chain(goaldeps)` punning is gone) —
/// goals are spliced out of the vec as they finish. Each goal's target is
/// `goal.dep.file: Option<FileId>`.
///
/// Lock discipline: no `FileNode` guard is ever held across `update_file` /
/// `f_mtime`; per-file flags/mtimes/states are copied out under a brief lock and
/// the guard dropped before recursing.
pub fn update_goal_chain(
    ctx: &crate::execctx::ExecContext,
    goaldeps: &mut Vec<GoalDepNode>,
) -> Result<UpdateStatus, crate::build_result::BuildError> {
    let mut last_cmd_count: ::core::ffi::c_ulong = 0;
    let t: bool = crate::make_main::opt_touch(ctx);
    let q: bool = crate::make_main::opt_question(ctx);
    let n: bool = crate::make_main::opt_just_print(ctx);
    let mut status: UpdateStatus = us_none;
    let depth: ::core::ffi::c_uint =
        (if opt_rebuilding_makefiles(ctx) { 1 } else { 0 }) as ::core::ffi::c_uint;
    // The c2rust `copy_dep_chain(goaldeps)` punning is gone: clone the goals into
    // an owned, index-addressable Vec we can splice as goals finish.
    let mut goals: Vec<GoalDepNode> = goaldeps.clone();
    // `goal_list` (consulted by `show_goal_error`) is the makefile-remaking goal
    // set; populate it only when rebuilding makefiles.
    *ctx.goal_list.borrow_mut() = if opt_rebuilding_makefiles(ctx) {
        goaldeps.clone()
    } else {
        Vec::new()
    };
    ctx.considered.set(ctx.considered.get().wrapping_add(1));
    while !goals.is_empty() {
        let mut running: i32 = 0;
        let mut wait: i32 = 0;
        unsafe {
            start_waiting_jobs(ctx)?;
            reap_children(
                ctx,
                (last_cmd_count == crate::make_main::opt_command_count(ctx)) as i32,
                0,
            )?;
        }
        last_cmd_count = crate::make_main::opt_command_count(ctx);
        // Walk the goals by index; finished goals are removed from `goals`.
        let mut gi = 0usize;
        while gi < goals.len() {
            let mut stop: i32 = 0;
            ctx.goal_dep
                .set(Some((goals[gi].dep.file, goals[gi].dep.flags)));
            let g_flags = goals[gi].dep.flags;
            let g_wait = goals[gi].dep.wait_here;
            let Some(g_file) = goals[gi].dep.file else {
                goals.remove(gi);
                continue;
            };
            // Resolve the live head through the renamed chain.
            let head = follow_renamed(ctx, g_file);
            // lock: copy out flags/state under a brief guard, drop it
            // before update_file / f_mtime.
            {
                if let Some(node) = ctx.filenodes.get(head) {
                    let mut hn = node.lock().expect("file node lock poisoned");
                    hn.dontcare = g_flags.contains(DepFlags::DONTCARE);
                }
            }
            let cmd_target = ctx
                .filenodes
                .get(head)
                .map(|node| node.lock().expect("file node lock poisoned").cmd_target)
                .unwrap_or(false);
            if opt_rebuilding_makefiles(ctx) {
                if cmd_target {
                    crate::make_main::set_touch_mirror(ctx, t);
                    crate::make_main::set_question_mirror(ctx, q);
                    crate::make_main::set_just_print_mirror(ctx, n);
                } else {
                    crate::make_main::set_just_print_mirror(ctx, false);
                    crate::make_main::set_question_mirror(ctx, false);
                    crate::make_main::set_touch_mirror(ctx, false);
                }
            }
            let ocommands_started = ctx.commands_started.get();
            wait = (g_wait && running != 0) as i32;
            if wait != 0 {
                if 0x2_i32 & dbg(ctx) != 0 {
                    let head_name = node_name(ctx, head);
                    let cn = cname(&head_name);
                    print_spaces(depth);
                    trace_name(b".WAIT is blocking '", &cn, b"'.\n");
                }
                break;
            }
            // lock: guard dropped — update_file locks internally.
            let fail = update_file(ctx, head, depth)?;
            let head = follow_renamed(ctx, head);
            // Copy out the post-update state under a brief guard.
            let (cs, updated, ustatus, last_mtime, mtime_before_update, dontcare, phony, has_recipe) = {
                let node = match ctx.filenodes.get(head) {
                    Some(node) => node,
                    None => {
                        goals.remove(gi);
                        continue;
                    }
                };
                let hn = node.lock().expect("file node lock poisoned");
                (
                    hn.command_state,
                    hn.updated,
                    hn.update_status,
                    hn.last_mtime,
                    hn.mtime_before_update,
                    hn.dontcare,
                    hn.phony,
                    hn.recipe.is_some(),
                )
            };
            running |= (cs == cs_running || cs == cs_deps_running) as i32;
            if ctx.commands_started.get() > ocommands_started {
                goals[gi].dep.changed = true;
            }
            if (fail as ::core::ffi::c_uint != 0 || updated)
                && (status as ::core::ffi::c_uint) < us_question as i32 as ::core::ffi::c_uint
            {
                if ustatus as u64 != 0 {
                    status = ustatus;
                    stop = (crate::make_main::opt_question(ctx)
                        && !crate::make_main::opt_keep_going(ctx)
                        && !opt_rebuilding_makefiles(ctx)) as i32;
                } else {
                    let mtime: uintmax_t = if opt_rebuilding_makefiles(ctx) {
                        if last_mtime == UNKNOWN_MTIME as uintmax_t {
                            f_mtime(ctx, head, false)?
                        } else {
                            last_mtime
                        }
                    } else if last_mtime == UNKNOWN_MTIME as uintmax_t {
                        f_mtime(ctx, head, true)?
                    } else {
                        last_mtime
                    };
                    if updated && mtime != mtime_before_update {
                        if !opt_rebuilding_makefiles(ctx)
                            || !crate::make_main::opt_just_print(ctx)
                                && !crate::make_main::opt_question(ctx)
                        {
                            status = UpdateStatus::Success;
                        }
                        if opt_rebuilding_makefiles(ctx) && dontcare {
                            stop = 1;
                        }
                    }
                }
            }
            let all_updated = updated;
            // Clear the dontcare we set above.
            if let Some(node) = ctx.filenodes.get(head) {
                node.lock().expect("file node lock poisoned").dontcare = false;
            }
            if wait != 0 {
                break;
            }
            let g_changed = goals[gi].dep.changed;
            if stop != 0 || all_updated {
                if !opt_rebuilding_makefiles(ctx)
                    && ustatus as i32 == us_success as i32
                    && !g_changed
                    && !crate::make_main::opt_run_silent(ctx)
                    && !crate::make_main::opt_question(ctx)
                {
                    let head_name = node_name(ctx, head);
                    let cn = cname(&head_name);
                    unsafe {
                        message(
                            ctx,
                            1,
                            head_name.len() as size_t,
                            if phony || !has_recipe {
                                b"Nothing to be done for '%s'.\0" as *const u8
                                    as *const ::core::ffi::c_char
                            } else {
                                b"'%s' is up to date.\0" as *const u8 as *const ::core::ffi::c_char
                            },
                            &[FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char)],
                        );
                    }
                }
                goals.remove(gi);
                if stop != 0 {
                    break;
                }
                // do not advance gi: remove() shifted the next goal into place.
            } else {
                gi += 1;
            }
        }
        if gi >= goals.len() || wait != 0 {
            ctx.considered.set(ctx.considered.get().wrapping_add(1));
        }
    }
    if opt_rebuilding_makefiles(ctx) {
        crate::make_main::set_touch_mirror(ctx, t);
        crate::make_main::set_question_mirror(ctx, q);
        crate::make_main::set_just_print_mirror(ctx, n);
    }
    // `complain()` and `update_file_1`'s circular-dep check route through
    // `fatal_err`/`BuildError` and now propagate all the way up through
    // `update_file`/`update_file_1` rather than bridging back to `exit_on_err`
    // at their call sites (#432 Phase B, #442), so an `Err` reaching here is a
    // real build failure on its way out to `main_0`.
    Ok(status)
}

/// FileId port of `show_goal_error`: emit the deferred goal-read `errno` error
/// for the current goal, if it came from an `include` and carried an error.
pub fn show_goal_error(ctx: &crate::execctx::ExecContext) {
    let Some((cur_file, cur_flags)) = ctx.goal_dep.get() else {
        return;
    };
    if cur_flags.bits() as i32 & (RM_INCLUDED | RM_DONTCARE) != RM_INCLUDED {
        return;
    }
    {
        let mut list = ctx.goal_list.borrow_mut();
        for goal in list.iter_mut() {
            if goal.dep.file == cur_file {
                if goal.error != 0 {
                    let name = goal.dep.file.map(|f| node_name(ctx, f)).unwrap_or_default();
                    let cn = cname(&name);
                    // The error is located at the goal's source floc (the
                    // include directive), the c2rust `&goal->floc`. Materialize
                    // it from the goal's owned `defined_in`/lineno/offset.
                    let mut floc_name: Vec<u8> = goal.defined_in.clone().unwrap_or_default();
                    let floc = goal.defined_in.as_ref().map(|_| {
                        floc_name.push(0);
                        Floc {
                            filenm: floc_name.as_ptr() as *const ::core::ffi::c_char,
                            lineno: goal.lineno,
                            offset: goal.offset,
                        }
                    });
                    let floc_ptr = floc
                        .as_ref()
                        .map_or(::core::ptr::null_mut::<Floc>(), |f| f as *const Floc as *mut Floc);
                    unsafe {
                        let errstr = strerror(goal.error);
                        error(
                            ctx,
                            floc_ptr,
                            (name.len() as size_t).wrapping_add(strlen(errstr) as size_t),
                            b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[
                                FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                                FmtArg::Str(errstr as *const ::core::ffi::c_char),
                            ],
                        );
                    }
                    goal.error = 0;
                }
                return;
            }
        }
    }
}

/// FileId port of `update_file`. Walks the target and (for double-colon targets)
/// its inline `double_colon` chain, calling [`update_file_1`] on each.
///
/// In the c2rust graph each double-colon entry was a separate `*mut file` linked
/// by `prev`; here the entries live inline on the head `FileNode` and are
/// addressed by their index in `double_colon` (0 = the head itself,
/// 1.. = `double_colon[i-1]`). [`update_file_1`] processes one entry by index.
///
/// Lock discipline: never holds a `FileNode` guard across [`update_file_1`]; the
/// `considered` bump and the prune check read/write under brief, dropped guards.
pub fn update_file(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    depth: ::core::ffi::c_uint,
) -> Result<UpdateStatus, crate::build_result::BuildError> {
    let mut status: UpdateStatus = us_success;
    // Snapshot the chain shape: whether double-colon, and how many entries.
    let (is_dc, n_entries) = {
        let Some(node) = ctx.filenodes.get(file) else {
            return Ok(us_success);
        };
        let n = node.lock().expect("file node lock poisoned");
        (n.is_double_colon, 1 + n.double_colon.len())
    };
    // Prune check (c2rust: the leading `considered`/`finished`/`prev` test).
    {
        let Some(node) = ctx.filenodes.get(file) else {
            return Ok(us_success);
        };
        let n = node.lock().expect("file node lock poisoned");
        let pruned = n.considered == ctx.considered.get()
            && !(n.updated
                && n.update_status as i32 > us_none as i32
                && !n.dontcare
                && n.no_diag)
            && !(is_dc
                && n.command_state as i32 == cs_finished as i32
                && !n.double_colon.is_empty());
        if pruned {
            let cs = n.command_state;
            let ustatus = n.update_status;
            let name = n.name.clone();
            drop(n);
            if 0x2_i32 & dbg(ctx) != 0 {
                let cn = cname(&name);
                print_spaces(depth);
                trace_name(b"Pruning file '", &cn, b"'.\n");
            }
            return Ok(if cs as i32 == cs_finished as i32 {
                ustatus
            } else {
                us_success
            });
        }
    }
    // Process each double-colon entry by index (0 = head). For single-colon
    // targets there is just entry 0.
    let count = if is_dc { n_entries } else { 1 };
    for entry in 0..count {
        // lock: guard dropped before update_file_1.
        {
            if let Some(node) = ctx.filenodes.get(file) {
                node.lock().expect("file node lock poisoned").considered = ctx.considered.get();
            }
        }
        let new = update_file_1(ctx, file, depth, entry)?;
        // Follow any rename that happened.
        let live = follow_renamed(ctx, file);
        if new as ::core::ffi::c_uint != 0 && !crate::make_main::opt_keep_going(ctx) {
            return Ok(new);
        }
        let cs = ctx
            .filenodes
            .get(live)
            .map(|node| {
                let mut g = node.lock().expect("file node lock poisoned");
                entry_node_mut(&mut g, entry).command_state
            })
            .unwrap_or(cs_finished);
        if cs as i32 == cs_running as i32 || cs as i32 == cs_deps_running as i32 {
            return Ok(UpdateStatus::Success);
        }
        if new as ::core::ffi::c_uint > status as ::core::ffi::c_uint {
            status = new;
        }
    }
    Ok(status)
}

/// FileId port of `complain`: recurse to the deepest still-failing prerequisite
/// and emit a "No rule to make target" diagnostic there.
///
/// Lock discipline: the node's deps/parent/name/no_diag are snapshotted under a
/// brief guard, the guard dropped, then `complain` recurses and `show_goal_error`
/// runs without any guard held.
///
/// `#432` Phase B: returns `Result` instead of exiting via `fatal()` — the
/// `!opt_keep_going` branches go through [`crate::output::fatal_err`] and
/// propagate `Err` instead of terminating the process directly. Every caller
/// now propagates too, so the error reaches `main_0` without any library frame
/// deciding to exit on its own (#442).
pub fn complain(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
) -> Result<(), crate::build_result::BuildError> {
    let (deps, parent, name, no_diag) = {
        let Some(node) = ctx.filenodes.get(file) else {
            return Ok(());
        };
        let n = node.lock().expect("file node lock poisoned");
        let deps: Vec<Option<FileId>> = n.deps.iter().map(|d| d.file).collect();
        (deps, n.parent, n.name.clone(), n.no_diag)
    };
    // Find the first dep that is updated+failed (recurse into it).
    let mut recursed = false;
    for dep_file in deps {
        let Some(df) = dep_file else { continue };
        let (updated, ustatus) = {
            let Some(node) = ctx.filenodes.get(df) else {
                continue;
            };
            let n = node.lock().expect("file node lock poisoned");
            (n.updated, n.update_status)
        };
        if updated && ustatus as i32 > us_none as i32 && no_diag {
            // lock: no guard held across this recursion.
            complain(ctx, df)?;
            recursed = true;
            break;
        }
    }
    if recursed {
        return Ok(());
    }
    show_goal_error(ctx);
    let cn = cname(&name);
    if let Some(parent_id) = parent {
        let pname = node_name(ctx, parent_id);
        let pcn = cname(&pname);
        let m: *const ::core::ffi::c_char = b"%sNo rule to make target '%s', needed by '%s'%s\0"
            as *const u8 as *const ::core::ffi::c_char;
        unsafe {
            if !crate::make_main::opt_keep_going(ctx) {
                return Err(crate::output::fatal_err(
                    ctx,
                    NILF,
                    0,
                    m,
                    &[
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                        FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                        FmtArg::Str(pcn.as_ptr() as *const ::core::ffi::c_char),
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                    ],
                ));
            }
            error(
                ctx,
                NILF,
                0,
                m,
                &[
                    FmtArg::Str(b"*** \0" as *const u8 as *const ::core::ffi::c_char),
                    FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                    FmtArg::Str(pcn.as_ptr() as *const ::core::ffi::c_char),
                    FmtArg::Str(b".\0" as *const u8 as *const ::core::ffi::c_char),
                ],
            );
        }
    } else {
        let m_0: *const ::core::ffi::c_char =
            b"%sNo rule to make target '%s'%s\0" as *const u8 as *const ::core::ffi::c_char;
        unsafe {
            if !crate::make_main::opt_keep_going(ctx) {
                return Err(crate::output::fatal_err(
                    ctx,
                    NILF,
                    0,
                    m_0,
                    &[
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                        FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                    ],
                ));
            }
            error(
                ctx,
                NILF,
                0,
                m_0,
                &[
                    FmtArg::Str(b"*** \0" as *const u8 as *const ::core::ffi::c_char),
                    FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                    FmtArg::Str(b".\0" as *const u8 as *const ::core::ffi::c_char),
                ],
            );
        }
    }
    if let Some(node) = ctx.filenodes.get(file) {
        node.lock().expect("file node lock poisoned").no_diag = false;
    }
    Ok(())
}

/// FileId port of `update_file_1`, processing one entry of a (possibly
/// double-colon) target. `entry` selects which entry: `0` is the head node
/// itself, `entry >= 1` is `head.double_colon[entry-1]`.
///
/// Lock discipline: this function NEVER holds a `FileNode` guard across a call
/// to `check_dep`/`update_file`/`f_mtime`/`notice_finished_file`/`remake_file`/
/// `try_implicit_rule`. Per-file state is copied out under a brief guard, the
/// guard dropped, the recursive call made, then results written back by index.
fn update_file_1(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    mut depth: ::core::ffi::c_uint,
    entry: usize,
) -> Result<UpdateStatus, crate::build_result::BuildError> {
    let mut dep_status: UpdateStatus = us_success;
    let mut running: i32 = 0;

    // Read/write the per-entry scalar state we need (the entry lives inline on
    // the head node; index 0 = head, else double_colon[entry-1]).
    macro_rules! with_entry {
        ($n:ident, $body:block) => {{
            match ctx.filenodes.get(file) {
                Some(node) => {
                    let mut guard = node.lock().expect("file node lock poisoned");
                    let $n: &mut FileNode = if entry == 0 {
                        &mut guard
                    } else {
                        &mut guard.double_colon[entry - 1]
                    };
                    $body
                }
                None => unreachable!("update_file_1: file id not interned"),
            }
        }};
    }

    let (name, updated, ustatus, cstate, last_mtime, is_phony) = with_entry!(n, {
        (
            n.name.clone(),
            n.updated,
            n.update_status,
            n.command_state,
            n.last_mtime,
            n.phony,
        )
    });
    let cn = cname(&name);
    if 0x2_i32 & dbg(ctx) != 0 {
        print_spaces(depth);
        trace_name(b"Considering target file '", &cn, b"'.\n");
    }
    if updated {
        if ustatus as i32 > us_none as i32 {
            if 0x2_i32 & dbg(ctx) != 0 {
                print_spaces(depth);
                trace_name(b"Recently tried and failed to update file '", &cn, b"'.\n");
            }
            let (no_diag, dontcare) = with_entry!(n, { (n.no_diag, n.dontcare) });
            if no_diag && !dontcare {
                complain(ctx, file)?;
            }
            return Ok(ustatus);
        }
        if 0x2_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"File '", &cn, b"' was considered already.\n");
        }
        return Ok(UpdateStatus::Success);
    }
    match cstate as i32 {
        0 | 1 => {}
        2 => {
            if 0x2_i32 & dbg(ctx) != 0 {
                print_spaces(depth);
                trace_name(b"Still updating file '", &cn, b"'.\n");
            }
            return Ok(UpdateStatus::Success);
        }
        3 => {
            if 0x2_i32 & dbg(ctx) != 0 {
                print_spaces(depth);
                trace_name(b"Finished updating file '", &cn, b"'.\n");
            }
            return Ok(ustatus);
        }
        // `cs_not_started`/`cs_deps_running`/`cs_running`/`cs_finished` are
        // the only states a file node carries.
        cs => unreachable!("unhandled command_state {cs}"),
    }
    // no_diag <- dontcare; mark updating on the entry.
    with_entry!(n, {
        n.no_diag = n.dontcare;
        n.updating = true;
    });
    depth = depth.wrapping_add(1);
    // this_mtime: f_mtime resolves through renames and locks internally.
    let mut this_mtime: uintmax_t = if last_mtime == UNKNOWN_MTIME as uintmax_t {
        f_mtime(ctx, file, true)?
    } else {
        last_mtime
    };
    let file = follow_renamed(ctx, file);
    let mut noexist = (this_mtime == NONEXISTENT_MTIME as uintmax_t) as i32;
    if noexist != 0 {
        if 0x1_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            if is_phony {
                trace_name(b"Target '", &cn, b"' is phony.\n");
            } else {
                trace_name(b"File '", &cn, b"' does not exist.\n");
            }
        }
    } else if this_mtime >= ORDINARY_MTIME_MIN as uintmax_t
        && this_mtime <= ordinary_mtime_max()
        && with_entry!(n, { n.low_resolution_time })
    {
        let ns: i32 = (this_mtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1) as uintmax_t)
            as i32;
        if ns != 0 {
            unsafe {
                error(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    name.len() as size_t,
                    b"*** warning: .LOW_RESOLUTION_TIME file '%s' has a high resolution time stamp\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    &[FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char)],
                );
            }
        }
        this_mtime = this_mtime.wrapping_add(
            ((if FILE_TIMESTAMP_HI_RES != 0 { 1000000000_i32 } else { 1 }) - 1 - ns) as uintmax_t,
        );
    }
    // also_make grouped-target peers: snapshot their FileIds, then f_mtime each.
    let peers: Vec<FileId> =
        with_entry!(n, { n.also_make.iter().filter_map(|d| d.file).collect() });
    let mut pi = 0;
    while pi < peers.len() && noexist == 0 {
        let adfile = peers[pi];
        let ad_last = ctx
            .filenodes
            .get(adfile)
            .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
            .unwrap_or(UNKNOWN_MTIME as uintmax_t);
        let fmtime: uintmax_t = if ad_last == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, adfile, true)?
        } else {
            ad_last
        };
        noexist = (fmtime == NONEXISTENT_MTIME as uintmax_t) as i32;
        if noexist != 0 {
            let adfile = follow_renamed(ctx, adfile);
            let (ad_phony, ad_name) = ctx
                .filenodes
                .get(adfile)
                .map(|node| {
                    let n = node.lock().expect("file node lock poisoned");
                    (n.phony, n.name.clone())
                })
                .unwrap_or((false, Vec::new()));
            if 0x1_i32 & dbg(ctx) != 0 {
                let adcn = cname(&ad_name);
                print_spaces(depth);
                crate::output::trace_parts(&[
                    b"Grouped target peer '",
                    &adcn[..adcn.len() - 1],
                    b"' of file '",
                    &cn[..cn.len() - 1],
                    if ad_phony {
                        b"' is phony.\n"
                    } else {
                        b"' does not exist.\n"
                    },
                ]);
            }
        } else if fmtime < this_mtime {
            this_mtime = fmtime;
        }
        pi += 1;
    }
    let mut must_make = noexist;
    // try_implicit_rule (FileId).
    let (need_implicit, has_recipe, is_target) = with_entry!(n, {
        (
            !n.phony && n.recipe.is_none() && !n.tried_implicit,
            n.recipe.is_some(),
            n.is_target,
        )
    });
    if need_implicit {
        // lock: guard dropped before try_implicit_rule.
        try_implicit_rule(ctx, file, depth)?;
        with_entry!(n, {
            n.tried_implicit = true;
        });
    }
    if !has_recipe && !is_target {
        // Default-recipe inheritance from the `.DEFAULT` target. The c2rust code
        // copied `default_file->cmds`; with the recipe owned inline we leave the
        // diagnostic but defer the actual inheritance to the recipe layer.
        if 0x8_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"Using default recipe for '", &cn, b"'.\n");
        }
    }

    // --- amake loop (trick #2): process the target, then each also_make peer ---
    // The c2rust code built a stack `dep{file=target, next=also_make}` and looped
    // uniformly; here we walk an explicit list [file, peers...].
    let amake_targets: Vec<FileId> = {
        let mut v = vec![file];
        v.extend(peers.iter().copied());
        v
    };
    'amake: for &amid in &amake_targets {
        if second_expansion(ctx) {
            // lock: guard dropped before expand_deps.
            unsafe {
                expand_deps(ctx, amid)?;
            }
        }
        // Snapshot the deps Vec (trick #1: index-based writeback).
        let mut deps: Vec<DepNode> = ctx
            .filenodes
            .get(amid)
            .map(|node| node.lock().expect("file node lock poisoned").deps.clone())
            .unwrap_or_default();
        let mut di = 0usize;
        // Indices to drop (circular deps), removed after the walk.
        let mut to_remove: Vec<usize> = Vec::new();
        while di < deps.len() {
            let wait_here = deps[di].wait_here;
            if wait_here && running != 0 {
                break;
            }
            // Resolve dep file through renames and write back by index.
            let Some(mut dfile) = deps[di].file else {
                di += 1;
                continue;
            };
            dfile = follow_renamed(ctx, dfile);
            deps[di].file = Some(dfile);
            let d_last = ctx
                .filenodes
                .get(dfile)
                .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
                .unwrap_or(UNKNOWN_MTIME as uintmax_t);
            let mtime: uintmax_t = if d_last == UNKNOWN_MTIME as uintmax_t {
                f_mtime(ctx, dfile, true)?
            } else {
                d_last
            };
            dfile = follow_renamed(ctx, dfile);
            deps[di].file = Some(dfile);
            // Circular-dep check: is the dep currently updating?
            let dep_updating = ctx
                .filenodes
                .get(dfile)
                .map(|node| node.lock().expect("file node lock poisoned").updating)
                .unwrap_or(false);
            if dep_updating {
                let dep_name = node_name(ctx, dfile);
                if warning::action(ctx, Type::CircularDep) == Action::Error {
                    let dcn = cname(&dep_name);
                    unsafe {
                        return Err(crate::output::fatal_err(
                            ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            (name.len() as size_t).wrapping_add(dep_name.len() as size_t),
                            b"circular %s <- %s dependency detected\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[
                                FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                                FmtArg::Str(dcn.as_ptr() as *const ::core::ffi::c_char),
                            ],
                        ));
                    }
                }
                if warning::is_active(ctx, Type::CircularDep) {
                    let dcn = cname(&dep_name);
                    unsafe {
                        error(
                            ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            (name.len() as size_t).wrapping_add(dep_name.len() as size_t),
                            b"circular %s <- %s dependency dropped\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[
                                FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                                FmtArg::Str(dcn.as_ptr() as *const ::core::ffi::c_char),
                            ],
                        );
                    }
                }
                // Drop this dep from the chain (dropped_list bookkeeping is gone).
                to_remove.push(di);
                di += 1;
            } else {
                // parent <- file; dontcare propagation under -r makefiles.
                let mut dontcare = false;
                {
                    if let Some(node) = ctx.filenodes.get(dfile) {
                        let mut dn = node.lock().expect("file node lock poisoned");
                        dn.parent = Some(file);
                        if opt_rebuilding_makefiles(ctx) {
                            dontcare = dn.dontcare;
                        }
                    }
                }
                if opt_rebuilding_makefiles(ctx) {
                    let file_dontcare = with_entry!(n, { n.dontcare });
                    if let Some(node) = ctx.filenodes.get(dfile) {
                        node.lock().expect("file node lock poisoned").dontcare = file_dontcare;
                    }
                }
                let mut maybe_make = must_make != 0;
                // lock: no guard held across check_dep.
                let new = check_dep(ctx, dfile, depth, this_mtime, &mut maybe_make)?;
                if new as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                    dep_status = new;
                }
                if opt_rebuilding_makefiles(ctx) {
                    if let Some(node) = ctx.filenodes.get(dfile) {
                        node.lock().expect("file node lock poisoned").dontcare = dontcare;
                    }
                }
                if !deps[di].ignore_mtime {
                    must_make = maybe_make as i32;
                }
                let dfile2 = follow_renamed(ctx, dfile);
                deps[di].file = Some(dfile2);
                // running: walk the dep's double-colon chain command_state.
                running |= dep_chain_running(ctx, dfile2) as i32;
                if dep_status as ::core::ffi::c_uint != 0 && !crate::make_main::opt_keep_going(ctx) {
                    break 'amake;
                }
                if running == 0 {
                    let d_last2 = ctx
                        .filenodes
                        .get(dfile2)
                        .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
                        .unwrap_or(UNKNOWN_MTIME as uintmax_t);
                    let cur = if d_last2 == UNKNOWN_MTIME as uintmax_t {
                        f_mtime(ctx, dfile2, true)?
                    } else {
                        d_last2
                    };
                    deps[di].changed = cur != mtime || mtime == NONEXISTENT_MTIME as uintmax_t;
                }
                di += 1;
            }
        }
        // Write back the (rename-resolved, changed-updated) deps minus the dropped
        // circular ones.
        for &idx in to_remove.iter().rev() {
            deps.remove(idx);
        }
        if let Some(node) = ctx.filenodes.get(amid) {
            node.lock().expect("file node lock poisoned").deps = deps;
        }
    }

    if must_make != 0 || ctx.always_make_flag.get() {
        // Intermediate-dep update pass over the head's deps.
        let mut new_deps: Vec<DepNode> = with_entry!(n, { n.deps.clone() });
        let (file_phony, file_has_recipe) = with_entry!(n, { (n.phony, n.recipe.is_some()) });
        let mut di = 0usize;
        while di < new_deps.len() {
            let wait_here = new_deps[di].wait_here;
            if wait_here && running != 0 {
                break;
            }
            let Some(mut dfile) = new_deps[di].file else {
                di += 1;
                continue;
            };
            let is_intermediate = ctx
                .filenodes
                .get(dfile)
                .map(|node| node.lock().expect("file node lock poisoned").intermediate)
                .unwrap_or(false);
            if is_intermediate {
                let d_last = ctx
                    .filenodes
                    .get(dfile)
                    .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
                    .unwrap_or(UNKNOWN_MTIME as uintmax_t);
                let mtime_0: uintmax_t = if d_last == UNKNOWN_MTIME as uintmax_t {
                    f_mtime(ctx, dfile, true)?
                } else {
                    d_last
                };
                dfile = follow_renamed(ctx, dfile);
                new_deps[di].file = Some(dfile);
                let mut dontcare_0 = false;
                if let Some(node) = ctx.filenodes.get(dfile) {
                    let mut dn = node.lock().expect("file node lock poisoned");
                    dn.parent = Some(file);
                    dn.considered = 0;
                    if opt_rebuilding_makefiles(ctx) {
                        dontcare_0 = dn.dontcare;
                    }
                }
                if opt_rebuilding_makefiles(ctx) {
                    let file_dontcare = with_entry!(n, { n.dontcare });
                    if let Some(node) = ctx.filenodes.get(dfile) {
                        node.lock().expect("file node lock poisoned").dontcare = file_dontcare;
                    }
                }
                // lock: no guard held across update_file.
                let new_0 = update_file(ctx, dfile, depth)?;
                if new_0 as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                    dep_status = new_0;
                }
                if opt_rebuilding_makefiles(ctx) {
                    if let Some(node) = ctx.filenodes.get(dfile) {
                        node.lock().expect("file node lock poisoned").dontcare = dontcare_0;
                    }
                }
                let dfile2 = follow_renamed(ctx, dfile);
                new_deps[di].file = Some(dfile2);
                running |= dep_chain_running(ctx, dfile2) as i32;
                if dep_status as ::core::ffi::c_uint != 0 && !crate::make_main::opt_keep_going(ctx) {
                    break;
                }
                if running == 0 {
                    let d_last2 = ctx
                        .filenodes
                        .get(dfile2)
                        .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
                        .unwrap_or(UNKNOWN_MTIME as uintmax_t);
                    let cur = if d_last2 == UNKNOWN_MTIME as uintmax_t {
                        f_mtime(ctx, dfile2, true)?
                    } else {
                        d_last2
                    };
                    new_deps[di].changed = (file_phony && file_has_recipe) || cur != mtime_0;
                }
            }
            di += 1;
        }
        with_entry!(n, {
            n.deps = new_deps;
        });
    }
    with_entry!(n, {
        n.updating = false;
    });
    depth = depth.wrapping_sub(1);
    if running != 0 {
        set_command_state_id(ctx, file, cs_deps_running);
        if 0x2_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"The prerequisites of '", &cn, b"' are being made.\n");
        }
        return Ok(UpdateStatus::Success);
    }
    if 0x2_i32 & dbg(ctx) != 0 {
        print_spaces(depth);
        trace_name(b"Finished prerequisites of target file '", &cn, b"'.\n");
    }
    if dep_status as u64 != 0 {
        with_entry!(n, {
            n.update_status = UpdateStatus::from_bits(
                if dep_status as ::core::ffi::c_uint == us_none as i32 as ::core::ffi::c_uint {
                    us_failed as i32 as ::core::ffi::c_uint
                } else {
                    dep_status as ::core::ffi::c_uint
                },
            );
        });
        notice_finished_file(ctx, file, entry)?;
        if 0x2_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"Giving up on target file '", &cn, b"'.\n");
        }
        if depth == 0
            && crate::make_main::opt_keep_going(ctx)
            && !crate::make_main::opt_just_print(ctx)
            && !crate::make_main::opt_question(ctx)
        {
            unsafe {
                error(
                    ctx,
                    ::core::ptr::null_mut::<Floc>(),
                    name.len() as size_t,
                    b"Target '%s' not remade because of errors.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char)],
                );
            }
        }
        return Ok(dep_status);
    }
    if with_entry!(n, { n.command_state }) as i32 == cs_deps_running as i32 {
        set_command_state_id(ctx, file, CommandState::NotStarted);
    }
    // deps_changed pass over the head's deps (rename-resolve, changed flags).
    let mut deps_changed = 0i32;
    let mut deps2: Vec<DepNode> = with_entry!(n, { n.deps.clone() });
    let mut di = 0usize;
    while di < deps2.len() {
        let Some(mut dfile) = deps2[di].file else {
            di += 1;
            continue;
        };
        let d_last = ctx
            .filenodes
            .get(dfile)
            .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
            .unwrap_or(UNKNOWN_MTIME as uintmax_t);
        let d_mtime: uintmax_t = if d_last == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, dfile, true)?
        } else {
            d_last
        };
        dfile = follow_renamed(ctx, dfile);
        deps2[di].file = Some(dfile);
        let d_intermediate = ctx
            .filenodes
            .get(dfile)
            .map(|node| node.lock().expect("file node lock poisoned").intermediate)
            .unwrap_or(false);
        if !deps2[di].ignore_mtime {
            if d_mtime == NONEXISTENT_MTIME as uintmax_t && !d_intermediate {
                must_make = 1;
            }
            deps_changed |= deps2[di].changed as i32;
        }
        deps2[di].changed = deps2[di].changed || noexist != 0 || d_mtime > this_mtime;
        di += 1;
    }
    with_entry!(n, {
        n.deps = deps2;
    });
    let (is_dc, deps_empty, file_is_target, file_has_recipe2, file_notintermediate) =
        with_entry!(n, {
            (
                n.is_double_colon,
                n.deps.is_empty(),
                n.is_target,
                n.recipe.is_some(),
                n.notintermediate,
            )
        });
    if is_dc && deps_empty {
        must_make = 1;
        if 0x1_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"Target '", &cn, b"' is double-colon and has no prerequisites.\n");
        }
    } else if noexist == 0
        && file_is_target
        && deps_changed == 0
        && !file_has_recipe2
        && !ctx.always_make_flag.get()
    {
        must_make = 0;
        if 0x2_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"No recipe for '", &cn, b"' and no prerequisites actually changed.\n");
        }
    } else if must_make == 0 && file_has_recipe2 && ctx.always_make_flag.get() {
        must_make = 1;
        if 0x2_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"Making '", &cn, b"' due to always-make flag.\n");
        }
    }
    if must_make == 0 {
        if 0x2_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"No need to remake target '", &cn, b"'.\n");
        }
        if !file_notintermediate && !ctx.no_intermediates.get() {
            with_entry!(n, {
                n.secondary = true;
            });
        }
        notice_finished_file(ctx, file, entry)?;
        // c2rust: reset name=hname over the chain; with one node per name this is
        // a single sync of the head's name to its hash-name.
        with_entry!(n, {
            n.name = n.hname.clone();
        });
        return Ok(UpdateStatus::Success);
    }
    if 0x1_i32 & dbg(ctx) != 0 {
        print_spaces(depth);
        trace_name(b"Must remake target '", &cn, b"'.\n");
    }
    // VPATH-name divergence check.
    let (nm, hn) = with_entry!(n, { (n.name.clone(), n.hname.clone()) });
    if nm != hn {
        if 0x1_i32 & dbg(ctx) != 0 {
            let hncn = cname(&hn);
            trace_name(b"  Ignoring VPATH name '", &hncn, b"'.\n");
        }
        with_entry!(n, {
            n.ignore_vpath = true;
        });
    }
    remake_file(ctx, file, entry)?;
    let cstate2 = with_entry!(n, { n.command_state });
    if cstate2 as i32 != cs_finished as i32 {
        if 0x2_i32 & dbg(ctx) != 0 {
            print_spaces(depth);
            trace_name(b"Recipe of '", &cn, b"' is being run.\n");
        }
        return Ok(UpdateStatus::Success);
    }
    let ustatus2 = with_entry!(n, { n.update_status });
    match ustatus2 as i32 {
        3 => {
            if 0x1_i32 & dbg(ctx) != 0 {
                print_spaces(depth);
                trace_name(b"Failed to remake target file '", &cn, b"'.\n");
            }
        }
        0 => {
            if 0x1_i32 & dbg(ctx) != 0 {
                print_spaces(depth);
                trace_name(b"Successfully remade target file '", &cn, b"'.\n");
            }
        }
        2 => {
            if 0x1_i32 & dbg(ctx) != 0 {
                print_spaces(depth);
                trace_name(b"Target file '", &cn, b"' needs to be remade under -q.\n");
            }
        }
        1 | _ => {}
    }
    with_entry!(n, {
        n.updated = true;
    });
    Ok(ustatus2)
}

/// Resolve a (possibly double-colon) entry within a locked head node: `0` is
/// the head itself, `i>=1` is `double_colon[i-1]`.
fn entry_node_mut(guard: &mut FileNode, entry: usize) -> &mut FileNode {
    if entry == 0 {
        guard
    } else {
        &mut guard.double_colon[entry - 1]
    }
}

/// FileId port of `notice_finished_file`: mark the target finished/updated,
/// possibly touch it, propagate timestamps across the double-colon chain, and
/// finish its grouped-target (also_make) peers.
///
/// Lock discipline: the head node is locked only for short field-copy/writeback
/// bursts; `touch_file`, `f_mtime`, and `check_also_make` run with no guard held
/// (each re-enters the arena). Peer ids are snapshotted before being locked.
pub fn notice_finished_file(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
) -> Result<(), crate::build_result::BuildError> {
    // Snapshot the bits we need, set command_state/updated on the entry.
    let (ran, has_recipe, recipe_any_recurse, recipe_line_flags, file_phony) = {
        let Some(node) = ctx.filenodes.get(file) else {
            return Ok(());
        };
        let mut guard = node.lock().expect("file node lock poisoned");
        let n = entry_node_mut(&mut guard, entry);
        let ran = (n.command_state as i32 == cs_running as i32) as i32;
        n.command_state = cs_finished;
        n.updated = true;
        let any_recurse = n.recipe.as_ref().map(|r| r.any_recurse).unwrap_or(false);
        let flags: Vec<RecipeLineFlags> = n
            .recipe
            .as_ref()
            .map(|r| r.lines.iter().map(|l| l.flags).collect())
            .unwrap_or_default();
        (ran, n.recipe.is_some(), any_recurse, flags, n.phony)
    };
    let mut touched = 0i32;
    let ustatus = ctx
        .filenodes
        .get(file)
        .map(|node| {
            let mut g = node.lock().expect("file node lock poisoned");
            entry_node_mut(&mut g, entry).update_status
        })
        .unwrap_or(us_success);
    if crate::make_main::opt_touch(ctx) && ustatus as i32 == us_success as i32 {
        // Touch unless every recipe line is recursive (RECURSE); one
        // non-recursive line means we touch.
        let mut should_touch = true;
        if has_recipe && recipe_any_recurse {
            should_touch = false;
            for f in &recipe_line_flags {
                if !f.contains(RecipeLineFlags::RECURSE) {
                    should_touch = true;
                    break;
                }
            }
        }
        if should_touch {
            if file_phony {
                if let Some(node) = ctx.filenodes.get(file) {
                    let mut g = node.lock().expect("file node lock poisoned");
                    entry_node_mut(&mut g, entry).update_status = UpdateStatus::Success;
                }
            } else if has_recipe {
                // lock: no guard held across touch_file.
                let ts = touch_file(ctx, file)?;
                if let Some(node) = ctx.filenodes.get(file) {
                    let mut g = node.lock().expect("file node lock poisoned");
                    entry_node_mut(&mut g, entry).update_status = ts;
                }
                ctx.commands_started
                    .set(ctx.commands_started.get().wrapping_add(1));
                touched = 1;
            }
        }
    }
    {
        if let Some(node) = ctx.filenodes.get(file) {
            let mut g = node.lock().expect("file node lock poisoned");
            let n = entry_node_mut(&mut g, entry);
            if n.mtime_before_update == UNKNOWN_MTIME as uintmax_t {
                n.mtime_before_update = n.last_mtime;
            }
        }
    }
    if ran != 0 && !file_phony || touched != 0 {
        let mut i_0: i32 = 0;
        if (crate::make_main::opt_question(ctx)
            || crate::make_main::opt_just_print(ctx)
            || crate::make_main::opt_touch(ctx))
            && has_recipe
        {
            i_0 = recipe_line_flags.len() as i32;
            while i_0 > 0 {
                if !recipe_line_flags[(i_0 - 1) as usize].contains(RecipeLineFlags::RECURSE) {
                    break;
                }
                i_0 -= 1;
            }
        } else {
            let is_target = ctx
                .filenodes
                .get(file)
                .map(|node| {
                    let mut g = node.lock().expect("file node lock poisoned");
                    entry_node_mut(&mut g, entry).is_target
                })
                .unwrap_or(false);
            if is_target && !has_recipe {
                i_0 = 1;
            }
        }
        if let Some(node) = ctx.filenodes.get(file) {
            let mut g = node.lock().expect("file node lock poisoned");
            entry_node_mut(&mut g, entry).last_mtime = if i_0 == 0 {
                UNKNOWN_MTIME as uintmax_t
            } else {
                new_mtime()
            };
        }
    }
    // Double-colon chain timestamp propagation: if every entry is updated,
    // propagate the max last_mtime to all entries. Entries are inline, so a
    // single lock on the head suffices.
    if entry == 0 {
        if let Some(node) = ctx.filenodes.get(file) {
            let mut n = node.lock().expect("file node lock poisoned");
            if n.is_double_colon && !n.double_colon.is_empty() {
                let mut max_mtime = n.last_mtime;
                let mut all_updated = n.updated;
                if all_updated {
                    for e in &n.double_colon {
                        if !e.updated {
                            all_updated = false;
                            break;
                        }
                        if max_mtime != UNKNOWN_MTIME as uintmax_t
                            && (e.last_mtime == UNKNOWN_MTIME as uintmax_t
                                || e.last_mtime > max_mtime)
                        {
                            max_mtime = e.last_mtime;
                        }
                    }
                }
                if all_updated {
                    n.last_mtime = max_mtime;
                    for e in &mut n.double_colon {
                        e.last_mtime = max_mtime;
                    }
                }
            }
        }
    }
    let (tried_implicit, peers): (bool, Vec<FileId>) = ctx
        .filenodes
        .get(file)
        .map(|node| {
            let n = node.lock().expect("file node lock poisoned");
            (
                n.tried_implicit,
                n.also_make.iter().filter_map(|d| d.file).collect(),
            )
        })
        .unwrap_or((false, Vec::new()));
    if ran != 0 && ustatus as i32 != us_none as i32 {
        for adfile in &peers {
            let ad_phony = {
                if let Some(node) = ctx.filenodes.get(*adfile) {
                    let mut an = node.lock().expect("file node lock poisoned");
                    an.command_state = cs_finished;
                    an.updated = true;
                    an.update_status = ustatus;
                    an.phony
                } else {
                    continue;
                }
            };
            if ran != 0 && !ad_phony {
                // lock: no guard held across f_mtime.
                f_mtime(ctx, *adfile, false)?;
                if crate::make_main::opt_just_print(ctx) {
                    if let Some(node) = ctx.filenodes.get(*adfile) {
                        node.lock().expect("file node lock poisoned").last_mtime = new_mtime();
                    }
                }
            }
        }
        if tried_implicit && !peers.is_empty() {
            check_also_make(ctx, file);
        }
    } else if ustatus as i32 == us_none as i32 {
        if let Some(node) = ctx.filenodes.get(file) {
            let mut g = node.lock().expect("file node lock poisoned");
            entry_node_mut(&mut g, entry).update_status = us_success;
        }
    }
    Ok(())
}

/// FileId port of `check_dep`: decide whether prerequisite `file` forces its
/// parent to be remade, recursing into its own prerequisites for non-target
/// intermediates.
///
/// Lock discipline: NEVER holds a `FileNode` guard across `update_file`,
/// `check_dep`, `f_mtime`, `try_implicit_rule`, or `expand_deps`. The `updating`
/// guard flag is set/cleared under brief locks; dep lists are snapshotted, walked
/// while recursing, then written back by index.
pub fn check_dep(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    depth: ::core::ffi::c_uint,
    this_mtime: uintmax_t,
    must_make: &mut bool,
) -> Result<UpdateStatus, crate::build_result::BuildError> {
    let mut dep_status: UpdateStatus = us_success;
    // Mark this file (and its double-colon head, which is the same node) updating.
    if let Some(node) = ctx.filenodes.get(file) {
        node.lock().expect("file node lock poisoned").updating = true;
    }
    let (phony, intermediate) = ctx
        .filenodes
        .get(file)
        .map(|node| {
            let n = node.lock().expect("file node lock poisoned");
            (n.phony, n.intermediate)
        })
        .unwrap_or((false, false));
    if phony || !intermediate {
        // lock: guard dropped before update_file / f_mtime.
        dep_status = update_file(ctx, file, depth)?;
        let live = follow_renamed(ctx, file);
        let last_mtime = ctx
            .filenodes
            .get(live)
            .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
            .unwrap_or(UNKNOWN_MTIME as uintmax_t);
        let mtime: uintmax_t = if last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, live, true)?
        } else {
            last_mtime
        };
        if mtime == NONEXISTENT_MTIME as uintmax_t || mtime > this_mtime {
            *must_make = true;
        }
    } else {
        let (need_implicit, _has_recipe, _is_target) = ctx
            .filenodes
            .get(file)
            .map(|node| {
                let n = node.lock().expect("file node lock poisoned");
                (
                    !n.phony && n.recipe.is_none() && !n.tried_implicit,
                    n.recipe.is_some(),
                    n.is_target,
                )
            })
            .unwrap_or((false, false, false));
        if need_implicit {
            // lock: guard dropped before try_implicit_rule.
            try_implicit_rule(ctx, file, depth)?;
            if let Some(node) = ctx.filenodes.get(file) {
                node.lock().expect("file node lock poisoned").tried_implicit = true;
            }
        }
        // (default-recipe inheritance from `.DEFAULT` is left to the recipe layer)
        let live = follow_renamed(ctx, file);
        let last_mtime = ctx
            .filenodes
            .get(live)
            .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
            .unwrap_or(UNKNOWN_MTIME as uintmax_t);
        let mtime_0: uintmax_t = if last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, live, true)?
        } else {
            last_mtime
        };
        if mtime_0 != NONEXISTENT_MTIME as uintmax_t && mtime_0 > this_mtime {
            *must_make = true;
        } else {
            let mut deps_running: i32 = 0;
            // Reset command state to NotStarted unless running.
            {
                if let Some(node) = ctx.filenodes.get(file) {
                    let mut n = node.lock().expect("file node lock poisoned");
                    if n.command_state as i32 != cs_running as i32 {
                        if n.command_state as i32 == cs_deps_running as i32 {
                            n.considered = 0;
                        }
                    }
                }
            }
            {
                let cs = ctx
                    .filenodes
                    .get(file)
                    .map(|node| node.lock().expect("file node lock poisoned").command_state)
                    .unwrap_or(cs_finished);
                if cs as i32 != cs_running as i32 {
                    set_command_state_id(ctx, file, CommandState::NotStarted);
                }
            }
            if second_expansion(ctx) {
                // lock: guard dropped before expand_deps.
                unsafe {
                    expand_deps(ctx, file)?;
                }
            }
            // Snapshot the deps; walk by index while recursing into check_dep.
            let mut deps: Vec<DepNode> = ctx
                .filenodes
                .get(file)
                .map(|node| node.lock().expect("file node lock poisoned").deps.clone())
                .unwrap_or_default();
            let name = node_name(ctx, file);
            let cn = cname(&name);
            let mut di = 0usize;
            let mut to_remove: Vec<usize> = Vec::new();
            while di < deps.len() {
                let Some(dep_file) = deps[di].file else {
                    di += 1;
                    continue;
                };
                let dep_updating = ctx
                    .filenodes
                    .get(dep_file)
                    .map(|node| node.lock().expect("file node lock poisoned").updating)
                    .unwrap_or(false);
                if dep_updating {
                    let dep_name = node_name(ctx, dep_file);
                    let dcn = cname(&dep_name);
                    unsafe {
                        error(
                            ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            0,
                            b"circular %s <- %s dependency dropped\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[
                                FmtArg::Str(cn.as_ptr() as *const ::core::ffi::c_char),
                                FmtArg::Str(dcn.as_ptr() as *const ::core::ffi::c_char),
                            ],
                        );
                    }
                    to_remove.push(di);
                    di += 1;
                } else {
                    if let Some(node) = ctx.filenodes.get(dep_file) {
                        node.lock().expect("file node lock poisoned").parent = Some(file);
                    }
                    let mut maybe_make = *must_make;
                    // lock: no guard held across check_dep.
                    let new = check_dep(
                        ctx,
                        dep_file,
                        depth.wrapping_add(1),
                        this_mtime,
                        &mut maybe_make,
                    )?;
                    if new as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                        dep_status = new;
                    }
                    if !deps[di].ignore_mtime {
                        *must_make = maybe_make;
                    }
                    let dfile2 = follow_renamed(ctx, dep_file);
                    deps[di].file = Some(dfile2);
                    if dep_status as ::core::ffi::c_uint != 0
                        && !crate::make_main::opt_keep_going(ctx)
                    {
                        break;
                    }
                    if dep_chain_running(ctx, dfile2) {
                        deps_running = 1;
                    }
                    di += 1;
                }
            }
            for &idx in to_remove.iter().rev() {
                deps.remove(idx);
            }
            if let Some(node) = ctx.filenodes.get(file) {
                node.lock().expect("file node lock poisoned").deps = deps;
            }
            if deps_running != 0 {
                set_command_state_id(ctx, file, CommandState::DepsRunning);
            }
        }
    }
    // Clear the updating guard.
    if let Some(node) = ctx.filenodes.get(file) {
        node.lock().expect("file node lock poisoned").updating = false;
    }
    Ok(dep_status)
}

/// FileId port of `touch_file`: touch the target's file on disk (or its archive
/// member), so `-t` records it as up to date without running the recipe.
///
/// Lock discipline: the node is locked only to copy out its name; all I/O runs
/// with no guard held.
pub fn touch_file(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
) -> Result<UpdateStatus, crate::build_result::BuildError> {
    let name = node_name(ctx, file);
    let cn = cname(&name);
    let name_ptr = cn.as_ptr() as *const ::core::ffi::c_char;
    unsafe {
        if !crate::make_main::opt_run_silent(ctx) {
            message(
                ctx,
                0,
                name.len() as size_t,
                b"touch %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[FmtArg::Str(name_ptr)],
            );
        }
        if crate::make_main::opt_just_print(ctx) {
            return Ok(us_success);
        }
        if ar_name_err(ctx, ::core::ffi::CStr::from_ptr(name_ptr))? {
            return Ok(if ar_touch(ctx, name_ptr)? != 0 {
                us_failed
            } else {
                us_success
            });
        }
        let mut fd: i32;
        loop {
            fd = open(name_ptr, 0o2_i32 | 0o100_i32, 0o666_i32);
            if !(fd == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd < 0 {
            perror_with_name(
                ctx,
                b"touch: open: \0" as *const u8 as *const ::core::ffi::c_char,
                name_ptr,
            );
            return Ok(UpdateStatus::Failed);
        }
        let mut statbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut buf: ::core::ffi::c_char = 'x' as i32 as ::core::ffi::c_char;
        let mut e: i32;
        loop {
            e = fstat(fd, &raw mut statbuf);
            if !(e == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if e < 0 {
            perror_with_name(
                ctx,
                b"touch: fstat: \0" as *const u8 as *const ::core::ffi::c_char,
                name_ptr,
            );
            return Ok(UpdateStatus::Failed);
        }
        loop {
            e = read(fd, &raw mut buf as *mut ::core::ffi::c_void, 1) as i32;
            if !(e == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if e < 0 {
            perror_with_name(
                ctx,
                b"touch: read: \0" as *const u8 as *const ::core::ffi::c_char,
                name_ptr,
            );
            return Ok(UpdateStatus::Failed);
        }
        let mut o: off_t;
        loop {
            o = lseek(fd, 0 as __off_t, 0) as off_t;
            if !(o == -1_i32 as off_t && *__errno_location() == EINTR) {
                break;
            }
        }
        if o < 0 {
            perror_with_name(
                ctx,
                b"touch: lseek: \0" as *const u8 as *const ::core::ffi::c_char,
                name_ptr,
            );
            return Ok(UpdateStatus::Failed);
        }
        loop {
            e = write(fd, &raw mut buf as *const ::core::ffi::c_void, 1) as i32;
            if !(e == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if e < 0 {
            perror_with_name(
                ctx,
                b"touch: write: \0" as *const u8 as *const ::core::ffi::c_char,
                name_ptr,
            );
            return Ok(UpdateStatus::Failed);
        }
        if statbuf.st_size == 0 as __off_t {
            close(fd);
            loop {
                fd = open(name_ptr, 0o2_i32 | 0o1000_i32, 0o666_i32);
                if !(fd == -1_i32 && *__errno_location() == EINTR) {
                    break;
                }
            }
            if fd < 0 {
                perror_with_name(
                    ctx,
                    b"touch: open: \0" as *const u8 as *const ::core::ffi::c_char,
                    name_ptr,
                );
                return Ok(UpdateStatus::Failed);
            }
        }
        close(fd);
    }
    Ok(us_success)
}

/// FileId port of `remake_file`: run the target's recipe (or, if it has none,
/// either treat it as up to date or complain), then finish it.
///
/// Lock discipline: the node is locked only for brief field reads/writes; never
/// across `complain`, `execute_file_commands`, or `notice_finished_file`.
pub fn remake_file(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
) -> Result<(), crate::build_result::BuildError> {
    let (has_recipe, phony, is_target, dontcare, any_recurse) = ctx
        .filenodes
        .get(file)
        .map(|node| {
            let mut g = node.lock().expect("file node lock poisoned");
            let n = entry_node_mut(&mut g, entry);
            (
                n.recipe.is_some(),
                n.phony,
                n.is_target,
                n.dontcare,
                n.recipe.as_ref().map(|r| r.any_recurse).unwrap_or(false),
            )
        })
        .unwrap_or((false, false, false, false, false));
    if !has_recipe {
        remake_no_recipe(ctx, file, entry, phony, is_target, dontcare)?;
    } else {
        // chop_commands needs &mut Recipe; lock briefly to chop in place.
        if let Some(node) = ctx.filenodes.get(file) {
            let mut g = node.lock().expect("file node lock poisoned");
            let n = entry_node_mut(&mut g, entry);
            if let Some(recipe) = n.recipe.as_mut() {
                chop_commands(ctx, recipe);
            }
        }
        if !crate::make_main::opt_touch(ctx) || any_recurse {
            // lock: no guard held across execute_file_commands.
            return execute_file_commands(ctx, file, entry);
        }
        if let Some(node) = ctx.filenodes.get(file) {
            let mut g = node.lock().expect("file node lock poisoned");
            entry_node_mut(&mut g, entry).update_status = UpdateStatus::Success;
        }
    }
    notice_finished_file(ctx, file, entry)?;
    Ok(())
}

/// Handle the recipe-less case of [`remake_file`]: a phony or explicit target
/// succeeds trivially; anything else fails (and `complain`s unless we are
/// rebuilding makefiles and the file is `dontcare`). Split out so `remake_file`'s
/// recipe path stays under the complexity gate.
fn remake_no_recipe(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    entry: usize,
    phony: bool,
    is_target: bool,
    dontcare: bool,
) -> Result<(), crate::build_result::BuildError> {
    if phony || is_target {
        if let Some(node) = ctx.filenodes.get(file) {
            let mut g = node.lock().expect("file node lock poisoned");
            entry_node_mut(&mut g, entry).update_status = UpdateStatus::Success;
        }
    } else {
        if !opt_rebuilding_makefiles(ctx) || !dontcare {
            // lock: no guard held across complain.
            complain(ctx, file)?;
        }
        if let Some(node) = ctx.filenodes.get(file) {
            let mut g = node.lock().expect("file node lock poisoned");
            entry_node_mut(&mut g, entry).update_status = UpdateStatus::Failed;
        }
    }
    Ok(())
}

/// Refresh `f_mtime`'s cached "adjusted now" from a freshly sampled clock.
///
/// Adds the timestamp resolution slack (`resolution - 1`) the original used so
/// a file at most one clock tick ahead of `now` is not flagged as being in the
/// future. Extracted as a pure function over the raw clock sample so the cache
/// arithmetic can be differential-tested against the original `static mut`
/// implementation (see `adjusted_now_from_clock_oracle` in the tests).
fn adjusted_now_from_clock(now: uintmax_t, resolution: i32) -> uintmax_t {
    now.wrapping_add((resolution - 1) as uintmax_t)
}

/// FileId port of `f_mtime`: return the mtime of file `file`, computing and
/// caching it if necessary. `NONEXISTENT_MTIME` if the file does not exist.
///
/// Lock discipline: the node is locked only for brief field reads/writes; never
/// across `name_mtime`, `vpath_search`, `library_search`, `ar_*`,
/// `rename_file`/`rehash_file`, or the recursive `f_mtime` calls. The
/// renamed/double-colon/prev chains are walked by re-locking each node.
pub fn f_mtime(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    search: bool,
) -> Result<uintmax_t, crate::build_result::BuildError> {
    let mut mtime: uintmax_t;
    // Snapshot the head's name/flags.
    let (name, ignore_vpath) = {
        let Some(node) = ctx.filenodes.get(file) else {
            return Ok(NONEXISTENT_MTIME as uintmax_t);
        };
        let n = node.lock().expect("file node lock poisoned");
        (n.name.clone(), n.ignore_vpath)
    };
    let cn = cname(&name);
    let name_ptr = cn.as_ptr() as *const ::core::ffi::c_char;
    let mut file = file;
    if unsafe { ar_name_err(ctx, ::core::ffi::CStr::from_ptr(name_ptr))? } {
        let memmtime: uintmax_t;
        // Own the split `archive`/`member` buffer for the rest of this branch
        // (replacing the old `ar_parse_name` xstrdup + `free`).
        let parsed = ParsedArName::parse(unsafe { ::core::ffi::CStr::from_ptr(name_ptr) });
        let arname = parsed.arname();
        let memname = parsed.memname();
        unsafe {
            memmtime = name_mtime(ctx, memname);
        }
        // Resolve the archive file's FileId (look up or enter by name bytes).
        let arname_bytes = unsafe { ::core::ffi::CStr::from_ptr(arname).to_bytes().to_vec() };
        let mut arfile = match lookup_file(ctx, &arname_bytes) {
            Some(id) => id,
            None => enter_file(ctx, &arname_bytes),
        };
        mtime = f_mtime(ctx, arfile, search)?;
        arfile = follow_renamed(ctx, arfile);
        let ar_hname = node_name(ctx, arfile);
        let ar_hname_c = cname(&ar_hname);
        if search
            && unsafe { strcmp(ar_hname_c.as_ptr() as *const ::core::ffi::c_char, arname) } != 0
        {
            // Build "arname(memname)" and rename/rehash this file accordingly.
            let memname_bytes = unsafe { ::core::ffi::CStr::from_ptr(memname).to_bytes().to_vec() };
            let mut newname: Vec<u8> = Vec::with_capacity(ar_hname.len() + memname_bytes.len() + 2);
            newname.extend_from_slice(&ar_hname);
            newname.push(b'(');
            newname.extend_from_slice(&memname_bytes);
            newname.push(b')');
            let (n_name, n_hname) = {
                let Some(node) = ctx.filenodes.get(file) else {
                    return Ok(NONEXISTENT_MTIME as uintmax_t);
                };
                let n = node.lock().expect("file node lock poisoned");
                (n.name.clone(), n.hname.clone())
            };
            if n_name == n_hname {
                rename_file(ctx, file, &newname);
            } else {
                rehash_file(ctx, file, &newname);
            }
            file = follow_renamed(ctx, file);
        }
        file = follow_renamed(ctx, file);
        // file.low_resolution_time = true; capture hname for member-date below.
        let file_hname = {
            let Some(node) = ctx.filenodes.get(file) else {
                return Ok(NONEXISTENT_MTIME as uintmax_t);
            };
            let mut n = node.lock().expect("file node lock poisoned");
            n.low_resolution_time = true;
            n.hname.clone()
        };
        if mtime == NONEXISTENT_MTIME as uintmax_t {
            return Ok(NONEXISTENT_MTIME as uintmax_t);
        }
        let fh = cname(&file_hname);
        let member_date: time_t =
            unsafe { ar_member_date(ctx, fh.as_ptr() as *const ::core::ffi::c_char)? };
        if member_date == -1_i32 as time_t
            || memmtime != NONEXISTENT_MTIME as uintmax_t
                && (memmtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) as time_t
                    > member_date
        {
            mtime = NONEXISTENT_MTIME as uintmax_t;
        } else {
            mtime = unsafe {
                file_timestamp_cons(
                    ctx,
                    fh.as_ptr() as *const ::core::ffi::c_char,
                    system_time_from_unix(member_date as i64, 0),
                )
            };
        }
    } else {
        mtime = unsafe { name_mtime(ctx, name_ptr) };
        if mtime == NONEXISTENT_MTIME as uintmax_t && search && !ignore_vpath {
            let mut name_0: *const ::core::ffi::c_char = unsafe {
                vpath_search(
                    ctx,
                    name_ptr,
                    &raw mut mtime,
                    ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                    ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                )
            }?;
            let is_lib = name.len() >= 2 && name[0] == b'-' && name[1] == b'l';
            if !name_0.is_null()
                || is_lib && {
                    name_0 = unsafe { library_search(ctx, name_ptr, &raw mut mtime) }?;
                    !name_0.is_null()
                }
            {
                if mtime != UNKNOWN_MTIME as uintmax_t {
                    if let Some(node) = ctx.filenodes.get(file) {
                        node.lock().expect("file node lock poisoned").last_mtime = mtime;
                    }
                }
                let name_0_bytes = unsafe { ::core::ffi::CStr::from_ptr(name_0).to_bytes().to_vec() };
                // The c2rust "prefix length" used in gpath_search.
                let name_len = name_0_bytes.len().saturating_sub(name.len()).saturating_sub(1);
                if gpath_search(ctx, &name_0_bytes[..name_len.min(name_0_bytes.len())]) {
                    rename_file(ctx, file, &name_0_bytes);
                    let live = follow_renamed(ctx, file);
                    let last_mtime = ctx
                        .filenodes
                        .get(live)
                        .map(|node| node.lock().expect("file node lock poisoned").last_mtime)
                        .unwrap_or(UNKNOWN_MTIME as uintmax_t);
                    return if last_mtime == UNKNOWN_MTIME as uintmax_t {
                        f_mtime(ctx, live, true)
                    } else {
                        Ok(last_mtime)
                    };
                }
                rehash_file(ctx, file, &name_0_bytes);
                file = follow_renamed(ctx, file);
                if mtime != OLD_MTIME as uintmax_t && mtime != new_mtime() {
                    mtime = unsafe { name_mtime(ctx, name_0) };
                }
            }
        }
    }
    // Clock-skew detection (future modification times).
    let updated = ctx
        .filenodes
        .get(file)
        .map(|node| node.lock().expect("file node lock poisoned").updated)
        .unwrap_or(false);
    if !ctx.clock_skew_detected.get()
        && mtime != NONEXISTENT_MTIME as uintmax_t
        && mtime != new_mtime()
        && !updated
    {
        let adjusted_mtime: uintmax_t = mtime;
        if ctx.mtime_adjusted_now.get() < adjusted_mtime {
            let (now, resolution) = file_timestamp_now(ctx);
            let adjusted_now: uintmax_t = adjusted_now_from_clock(now, resolution);
            ctx.mtime_adjusted_now.set(adjusted_now);
            if adjusted_now < adjusted_mtime {
                let from_now: ::core::ffi::c_double = (mtime
                    .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
                .wrapping_sub(
                    now.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                        >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }),
                ) as ::core::ffi::c_double
                    + ((mtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                        & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1)
                            as uintmax_t) as i32
                        - (now.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                            & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1)
                                as uintmax_t) as i32) as ::core::ffi::c_double
                        / 1e9f64;
                let mut from_now_string: [::core::ffi::c_char; 100] = [0; 100];
                unsafe {
                    if from_now >= 100.0f64 && from_now < ULONG_MAX as ::core::ffi::c_double {
                        sprintf(
                            &raw mut from_now_string as *mut ::core::ffi::c_char,
                            b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
                            from_now as ::core::ffi::c_ulong,
                        );
                    } else {
                        sprintf(
                            &raw mut from_now_string as *mut ::core::ffi::c_char,
                            b"%.2g\0" as *const u8 as *const ::core::ffi::c_char,
                            from_now,
                        );
                    }
                    error(
                        ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        (name.len() as size_t).wrapping_add(
                            strlen(&raw mut from_now_string as *mut ::core::ffi::c_char) as size_t,
                        ),
                        b"warning: file '%s' has modification time %s s in the future\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        &[
                            FmtArg::Str(name_ptr),
                            FmtArg::Str(
                                (&raw mut from_now_string as *mut ::core::ffi::c_char)
                                    as *const ::core::ffi::c_char,
                            ),
                        ],
                    );
                }
                ctx.clock_skew_detected.set(true);
            }
        }
    }
    // Timestamp propagation across the double-colon chain. The c2rust code walked
    // a `double_colon`/`prev` linked list of separate `*mut file`s; here the
    // entries live inline on the head, so a single lock suffices.
    if let Some(node) = ctx.filenodes.get(file) {
        let mut n = node.lock().expect("file node lock poisoned");
        // propagate_timestamp = head.updated (the c2rust `(*file).updated()` after
        // stepping into the double_colon head, which is the same node here).
        let propagate_timestamp = n.updated;
        let apply = |e: &mut FileNode| {
            if mtime != NONEXISTENT_MTIME as uintmax_t
                && e.command_state as i32 == cs_not_started as i32
                && !e.tried_implicit
                && e.intermediate
            {
                e.intermediate = false;
            }
            if e.updated == propagate_timestamp {
                e.last_mtime = mtime;
            }
        };
        apply(&mut n);
        if n.is_double_colon {
            let len = n.double_colon.len();
            for i in 0..len {
                apply(&mut n.double_colon[i]);
            }
        }
    }
    Ok(mtime)
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn name_mtime(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
) -> uintmax_t {
    let mut mtime: uintmax_t;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut e: i32;
    loop {
        e = stat(name, &raw mut st);
        if !(e == -1_i32 && *__errno_location() == EINTR) {
            break;
        }
    }
    if e == 0 {
        mtime = file_timestamp_cons(
            ctx,
            name,
            system_time_from_unix(st.st_mtim.tv_sec, st.st_mtim.tv_nsec as u32),
        );
    } else if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
        mtime = NONEXISTENT_MTIME as uintmax_t;
    } else {
        perror_with_name(
            ctx,
            b"stat: \0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        return NONEXISTENT_MTIME as uintmax_t;
    }
    if crate::make_main::opt_check_symlink(ctx) && strlen(name) <= GET_PATH_MAX as size_t {
        mtime = follow_symlink_mtime(ctx, name, mtime);
    }
    mtime
}

/// `-L`/`--check-symlink-times` support for [`name_mtime`]: walk the symlink
/// chain starting at `name`, folding in the max of each link's own mtime, and
/// return the resulting timestamp. Split out of `name_mtime` so the chain walk's
/// branchy `lstat`/`readlink` loop lives in its own function.
///
/// # Safety
/// `name` must be a valid nul-terminated path no longer than `GET_PATH_MAX`.
unsafe fn follow_symlink_mtime(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    mut mtime: uintmax_t,
) -> uintmax_t {
    let mut st: stat = ::core::mem::zeroed();
    let mut e: i32;
    let mut lpath: [::core::ffi::c_char; 4097] = [0; 4097];
    strcpy(&raw mut lpath as *mut ::core::ffi::c_char, name);
    loop {
        let ltime: uintmax_t;
        let mut lbuf: [::core::ffi::c_char; 4097] = [0; 4097];
        let mut llen: ::core::ffi::c_long;
        let p: *mut ::core::ffi::c_char;
        loop {
            e = lstat(&raw mut lpath as *mut ::core::ffi::c_char, &raw mut st);
            if !(e == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if e != 0 {
            if *__errno_location() != ENOENT && *__errno_location() != ENOTDIR {
                perror_with_name(
                    ctx,
                    b"lstat: \0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut lpath as *mut ::core::ffi::c_char,
                );
            }
            break;
        } else {
            if !(st.st_mode & __S_IFMT as __mode_t == 0o120000 as __mode_t) {
                break;
            }
            ltime = file_timestamp_cons(
                ctx,
                &raw mut lpath as *mut ::core::ffi::c_char,
                system_time_from_unix(st.st_mtim.tv_sec, st.st_mtim.tv_nsec as u32),
            );
            if ltime > mtime {
                mtime = ltime;
            }
            loop {
                llen = readlink(
                    &raw mut lpath as *mut ::core::ffi::c_char,
                    &raw mut lbuf as *mut ::core::ffi::c_char,
                    (4096_i32 - 1) as size_t,
                ) as ::core::ffi::c_long;
                if !(llen == -1_i32 as ::core::ffi::c_long && *__errno_location() == EINTR) {
                    break;
                }
            }
            if llen < 0 {
                perror_with_name(
                    ctx,
                    b"readlink: \0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut lpath as *mut ::core::ffi::c_char,
                );
                break;
            } else {
                lbuf[llen as usize] = 0;
                if lbuf[0_i32 as usize] as i32 == '/' as i32 || {
                    p = strrchr(&raw mut lpath as *mut ::core::ffi::c_char, '/' as i32);
                    p.is_null()
                } {
                    strcpy(
                        &raw mut lpath as *mut ::core::ffi::c_char,
                        &raw mut lbuf as *mut ::core::ffi::c_char,
                    );
                } else {
                    if p.offset_from(&raw mut lpath as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long
                        + llen
                        + 2
                        > GET_PATH_MAX as ::core::ffi::c_long
                    {
                        break;
                    }
                    strcpy(
                        p.offset(1_i32 as isize),
                        &raw mut lbuf as *mut ::core::ffi::c_char,
                    );
                }
            }
        }
    }
    mtime
}
// Not `extern "C"` since #442: it takes a `&ExecContext`, so it was never
// callable from C — the ABI was a c2rust leftover, and a `Result` return is
// not FFI-safe.
unsafe fn library_search(
    ctx: &crate::execctx::ExecContext,
    mut lib: *const ::core::ffi::c_char,
    mtime_ptr: *mut uintmax_t,
) -> Result<*const ::core::ffi::c_char, crate::build_result::BuildError> {
    const dirs: [*const ::core::ffi::c_char; 4] = [
        b"/lib\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/lib\0" as *const u8 as *const ::core::ffi::c_char,
        LIBDIR.as_ptr(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let libpatterns: *mut ::core::ffi::c_char;
    let mut mtime: uintmax_t;
    let mut p: *mut ::core::ffi::c_char;
    let mut p2: *const ::core::ffi::c_char;
    let mut len: size_t = 0;
    let liblen: size_t;
    let mut best_vpath: ::core::ffi::c_uint = 0;
    let mut best_path: ::core::ffi::c_uint = 0;
    let mut dp: *const *const ::core::ffi::c_char;
    libpatterns = allocated_expand_variable(
        ctx,
        b".LIBPATTERNS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
    )?;
    lib = lib.offset(2_i32 as isize);
    liblen = strlen(lib) as size_t;
    p2 = libpatterns;
    loop {
        p = find_next_token(&raw mut p2, &raw mut len);
        if p.is_null() {
            break;
        }
        let libbuf: *mut ::core::ffi::c_char;
        let c: ::core::ffi::c_char = *p.offset(len as isize);
        let p3: *mut ::core::ffi::c_char;
        let mut p4: *mut ::core::ffi::c_char;
        *p.offset(len as isize) = 0;
        p3 = find_percent(p);
        if p3.is_null() {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                strlen(p) as size_t,
                b".LIBPATTERNS element '%s' is not a pattern\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str((p) as *const ::core::ffi::c_char)],
            );
            *p.offset(len as isize) = c;
        } else {
            p4 = variable_buffer_output(
                ctx,
                ctx.variable_buffer.ptr(),
                p,
                p3.offset_from(p) as ::core::ffi::c_long as size_t,
            );
            p4 = variable_buffer_output(ctx, p4, lib, liblen);
            variable_buffer_output(
                ctx,
                p4,
                p3.offset(1_i32 as isize),
                len.wrapping_sub(p3.offset_from(p) as ::core::ffi::c_long as size_t),
            );
            *p.offset(len as isize) = c;
            libbuf = ctx.variable_buffer.ptr();
            mtime = name_mtime(ctx, libbuf);
            if mtime != NONEXISTENT_MTIME as uintmax_t {
                if !mtime_ptr.is_null() {
                    *mtime_ptr = mtime;
                }
                file = strcache_add(ctx, libbuf);
                break;
            } else {
                let mut vpath_index: ::core::ffi::c_uint = 0;
                let mut path_index: ::core::ffi::c_uint = 0;
                let f: *const ::core::ffi::c_char = vpath_search(
                    ctx,
                    libbuf,
                    if !mtime_ptr.is_null() {
                        &raw mut mtime
                    } else {
                        ::core::ptr::null_mut::<uintmax_t>()
                    },
                    &raw mut vpath_index,
                    &raw mut path_index,
                )?;
                if !f.is_null()
                    && (file.is_null()
                        || vpath_index < best_vpath
                        || vpath_index == best_vpath && path_index < best_path)
                {
                    file = f;
                    best_vpath = vpath_index;
                    best_path = path_index;
                    if !mtime_ptr.is_null() {
                        *mtime_ptr = mtime;
                    }
                }
                let mut cache = ctx.library_search_cache.borrow_mut();
                if cache.buflen == 0 {
                    dp = dirs.as_ptr();
                    while !(*dp).is_null() {
                        let l: size_t = strlen(*dp) as size_t;
                        if l > cache.libdir_maxlen {
                            cache.libdir_maxlen = l;
                        }
                        cache.std_dirs = cache.std_dirs.wrapping_add(1);
                        dp = dp.offset(1_i32 as isize);
                    }
                    cache.buflen = strlen(libbuf) as size_t;
                    let want = cache.libdir_maxlen.wrapping_add(cache.buflen).wrapping_add(2);
                    cache.buf.resize(want, 0);
                } else if cache.buflen < strlen(libbuf) {
                    cache.buflen = strlen(libbuf) as size_t;
                    let want = cache.libdir_maxlen.wrapping_add(cache.buflen).wrapping_add(2);
                    cache.buf.resize(want, 0);
                }
                let buf = cache.buf.as_mut_ptr() as *mut ::core::ffi::c_char;
                let mut vpath_index_0: ::core::ffi::c_uint =
                    (!(0_i32 as ::core::ffi::c_uint)).wrapping_sub(cache.std_dirs);
                dp = dirs.as_ptr();
                while !(*dp).is_null() {
                    sprintf(
                        buf,
                        b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
                        *dp,
                        libbuf,
                    );
                    mtime = name_mtime(ctx, buf);
                    if mtime != NONEXISTENT_MTIME as uintmax_t
                        && (file.is_null() || vpath_index_0 < best_vpath)
                    {
                        file = strcache_add(ctx, buf);
                        best_vpath = vpath_index_0;
                        if !mtime_ptr.is_null() {
                            *mtime_ptr = mtime;
                        }
                    }
                    vpath_index_0 = vpath_index_0.wrapping_add(1);
                    dp = dp.offset(1_i32 as isize);
                }
                drop(cache);
            }
        }
    }
    free(libpatterns as *mut ::core::ffi::c_void);
    Ok(file)
}
pub const LIBDIR: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"/usr/local/lib\0") };
pub const __CHAR_BIT__: i32 = 8;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const FILE_TIMESTAMP_HI_RES: i32 = 1;

#[cfg(test)]
mod f_mtime_tests {
    use super::*;
    use crate::file::FileNode;
    use std::io::Write;

    // Serialize tests that touch the process-wide file/strcache globals.
    static F_MTIME_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Create a unique temp file and return its absolute path as raw bytes (a
    /// suitable `FileNode` name). The file is left on disk; the caller removes it.
    fn make_temp_file() -> (std::path::PathBuf, Vec<u8>) {
        let dir = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = dir.join(format!("fmtime-{nanos}-{}", std::process::id()));
        let mut f = std::fs::File::create(&path).expect("create temp file");
        f.write_all(b"x").expect("write temp file");
        let name = path.to_str().unwrap().as_bytes().to_vec();
        (path, name)
    }

    /// Intern a FileNode with the given name into `ctx`, returning its FileId.
    fn intern_named(
        ctx: &crate::execctx::ExecContext,
        name: &[u8],
        updated: bool,
        ignore_vpath: bool,
    ) -> FileId {
        let mut node = FileNode::new(name.to_vec());
        node.updated = updated;
        node.ignore_vpath = ignore_vpath;
        ctx.filenodes.intern(node)
    }

    /// `f_mtime` on a plain (non-archive) file whose name is an existing path
    /// stats the file and returns an ordinary, existent timestamp.
    #[test]
    fn f_mtime_existing_file_returns_ordinary_mtime() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (path, name) = make_temp_file();
        let ctx = crate::execctx::ExecContext::default();
        let id = intern_named(&ctx, &name, true, false);
        let mtime = f_mtime(&ctx, id, false).expect("no `~`/archive rejection in this fixture");
        assert_ne!(
            mtime, NONEXISTENT_MTIME as uintmax_t,
            "an existing file has a real mtime"
        );
        assert_ne!(
            mtime, UNKNOWN_MTIME as uintmax_t,
            "the mtime is resolved, not unknown"
        );
        assert!(
            mtime > ORDINARY_MTIME_MIN as uintmax_t,
            "an existing file lands in the ordinary timestamp range"
        );
        let last = ctx
            .filenodes
            .get(id)
            .map(|n| n.lock().unwrap().last_mtime)
            .unwrap();
        assert_eq!(last, mtime, "last_mtime is cached on the file");
        let _ = std::fs::remove_file(&path);
    }

    /// `f_mtime` on a plain file whose name does not exist (and with `search`
    /// disabled) returns `NONEXISTENT_MTIME`.
    #[test]
    fn f_mtime_missing_file_is_nonexistent() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _ctx = crate::make_main::install_default_exec_context_for_test();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let missing = std::env::temp_dir().join(format!("fmtime-missing-{nanos}"));
        let name = missing.to_str().unwrap().as_bytes().to_vec();
        let ctx = crate::execctx::ExecContext::default();
        let id = intern_named(&ctx, &name, true, true);
        let mtime = f_mtime(&ctx, id, false).expect("no `~`/archive rejection in this fixture");
        assert_eq!(
            mtime, NONEXISTENT_MTIME as uintmax_t,
            "a missing file with no search reports nonexistent"
        );
    }

    /// Same plain-file path but with `updated` unset, so `f_mtime` runs the
    /// clock-skew check block. An existing file is past-dated, so no warning.
    #[test]
    fn f_mtime_past_file_skips_skew_warning() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (path, name) = make_temp_file();
        let ctx = crate::execctx::ExecContext::default();
        let id = intern_named(&ctx, &name, false, false);
        let mtime = f_mtime(&ctx, id, false).expect("no `~`/archive rejection in this fixture");
        assert!(
            mtime > ORDINARY_MTIME_MIN as uintmax_t,
            "an existing past-dated file resolves to an ordinary mtime"
        );
        assert!(
            !ctx.clock_skew_detected.get(),
            "a past-dated file triggers no clock-skew warning"
        );
        let _ = std::fs::remove_file(&path);
    }

    /// `f_mtime` on a plain file whose mtime lies far in the future drives the
    /// clock-skew warning branch and sets `clock_skew_detected`.
    #[test]
    fn f_mtime_future_file_warns_and_sets_skew() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _ctx = crate::make_main::install_default_exec_context_for_test();
        let (path, name) = make_temp_file();
        let future = std::time::SystemTime::now() + std::time::Duration::from_secs(10_000_000);
        let future_secs = future
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        unsafe {
            let times = [
                libc::timespec {
                    tv_sec: future_secs,
                    tv_nsec: 0,
                },
                libc::timespec {
                    tv_sec: future_secs,
                    tv_nsec: 0,
                },
            ];
            let cpath = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
            assert_eq!(
                libc::utimensat(libc::AT_FDCWD, cpath.as_ptr(), times.as_ptr(), 0),
                0,
                "set the temp file's mtime into the future"
            );
        }
        let ctx = crate::execctx::ExecContext::default();
        let id = intern_named(&ctx, &name, false, false);
        let mtime = f_mtime(&ctx, id, false).expect("no `~`/archive rejection in this fixture");
        assert!(
            mtime > ORDINARY_MTIME_MIN as uintmax_t,
            "the future-dated file still resolves to an ordinary mtime"
        );
        assert!(
            ctx.clock_skew_detected.get(),
            "a future-dated file triggers the clock-skew warning"
        );
        let _ = std::fs::remove_file(&path);
    }

    /// Verbatim port of the original future-mtime cache arithmetic oracle.
    fn adjusted_now_from_clock_oracle(now: uintmax_t, resolution: i32) -> uintmax_t {
        now.wrapping_add((resolution - 1) as uintmax_t)
    }

    #[test]
    fn adjusted_now_from_clock_matches_oracle() {
        let cases: &[(uintmax_t, i32)] = &[
            (0, 0),
            (0, 1),
            (1_000_000_000, 1),
            (1_000_000_000, 1_000_000_000),
            (uintmax_t::MAX, 1),
            (0, 1_000_000_000),
            (5, 0),
        ];
        for &(now, resolution) in cases {
            assert_eq!(
                adjusted_now_from_clock(now, resolution),
                adjusted_now_from_clock_oracle(now, resolution),
                "diverged for now={now}, resolution={resolution}"
            );
        }
    }

    #[test]
    fn future_mtime_cache_gate_matches_oracle() {
        fn step_new(
            cached: uintmax_t,
            mtime: uintmax_t,
            clock_now: uintmax_t,
            resolution: i32,
        ) -> (uintmax_t, bool, bool) {
            if cached < mtime {
                let adjusted_now = adjusted_now_from_clock(clock_now, resolution);
                (adjusted_now, true, adjusted_now < mtime)
            } else {
                (cached, false, false)
            }
        }
        fn step_oracle(
            adjusted_now: uintmax_t,
            mtime: uintmax_t,
            clock_now: uintmax_t,
            resolution: i32,
        ) -> (uintmax_t, bool, bool) {
            let adjusted_mtime = mtime;
            if adjusted_now < adjusted_mtime {
                let new_adjusted = clock_now.wrapping_add((resolution - 1) as uintmax_t);
                let warned = new_adjusted < adjusted_mtime;
                (new_adjusted, true, warned)
            } else {
                (adjusted_now, false, false)
            }
        }

        let timeline: &[(uintmax_t, uintmax_t, i32)] = &[
            (100, 100, 1),
            (50, 100, 1),
            (100, 100, 1),
            (200, 150, 1),
            (200, 250, 1),
            (300, 260, 1),
            (300, 400, 1),
            (300, 400, 5),
        ];
        let (mut c_new, mut c_ora) = (0 as uintmax_t, 0 as uintmax_t);
        for &(mtime, clock_now, resolution) in timeline {
            let (nn, n_read, n_warn) = step_new(c_new, mtime, clock_now, resolution);
            let (on, o_read, o_warn) = step_oracle(c_ora, mtime, clock_now, resolution);
            assert_eq!(
                (nn, n_read, n_warn),
                (on, o_read, o_warn),
                "gate diverged at mtime={mtime}, clock_now={clock_now}, resolution={resolution}"
            );
            c_new = nn;
            c_ora = on;
        }
    }
}

#[cfg(test)]
mod touch_file_tests {
    //! Since #442 `touch_file` returns `Result`: the archive arm's rejection
    //! travels back out to `notice_finished_file` instead of ending the
    //! process. These also give `touch_file` its first coverage.

    use super::touch_file;
    use crate::file::{enter_file, UpdateStatus};

    fn fresh_ctx() -> crate::execctx::ExecContext {
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        // SAFETY: fresh context; each table is initialized once.
        unsafe {
            crate::function::hash_init_function_table(&ctx);
            crate::variable::init_hash_global_variable_set(&ctx);
            crate::expand::initialize_variable_output(&ctx);
        }
        ctx
    }

    /// Touching an ordinary existing file succeeds and leaves it in place.
    #[test]
    fn touches_an_existing_file() {
        let dir = std::env::temp_dir().join(format!(
            "touch-file-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("target.txt");
        std::fs::write(&path, b"contents").unwrap();

        let ctx = fresh_ctx();
        let name = path.to_str().unwrap().as_bytes().to_vec();
        let id = enter_file(&ctx, &name);
        let status = touch_file(&ctx, id).expect("a plain path is not an archive reference");
        assert_eq!(status, UpdateStatus::Success);
        assert!(path.exists(), "touch does not remove the file");
        assert_eq!(
            std::fs::read(&path).unwrap(),
            b"contents",
            "touch does not truncate the file"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    /// A target that does not exist is created by the touch, matching make's
    /// `open(..., O_CREAT)`.
    #[test]
    fn creates_a_missing_file() {
        let dir = std::env::temp_dir().join(format!(
            "touch-file-new-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("fresh.txt");

        let ctx = fresh_ctx();
        let name = path.to_str().unwrap().as_bytes().to_vec();
        let id = enter_file(&ctx, &name);
        let status = touch_file(&ctx, id).expect("a plain path is not an archive reference");
        assert_eq!(status, UpdateStatus::Success);
        assert!(path.exists(), "the target is created");
        std::fs::remove_dir_all(&dir).ok();
    }
}
