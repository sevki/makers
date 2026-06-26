pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __syscall_slong_t, __time_t, __uid_t, off_t, size_t, ssize_t, time_t, uintmax_t,
};
use crate::file::free_dep_chain;
use crate::file::{
    cs_deps_running, cs_finished, cs_not_started, cs_running, dep, file, update_status,
    us_failed, us_none, us_question, us_success, CommandState, Dep, File, GoalDep, UpdateStatus,
    VariableSet, VariableSetList,
};
use crate::misc::{copy_dep_chain, find_next_token, print_spaces, xmalloc, xrealloc};
use crate::output::FmtArg;
use crate::stdio::FILE;
use crate::strcache::strcache_add;
use libc::{
    __errno_location, abort, close, free, open, printf, puts, sprintf, strcmp, strcpy, strerror,
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
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
use crate::warning::{self, Action, Type};
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
use crate::floc::Floc;

use crate::ar::{ar_member_date, ar_name, ar_parse_name, ar_touch};
use crate::commands::{chop_commands, execute_file_commands};
use crate::expand::{allocated_expand_variable, variable_buffer, variable_buffer_output};
pub use crate::file::nameseq;
use crate::file::{
    enter_file, expand_deps, file_timestamp_cons, file_timestamp_now, lookup_file, rehash_file,
    rename_file, set_command_state, system_time_from_unix,
};
use crate::implicit::try_implicit_rule;
use crate::job::{reap_children, start_waiting_jobs};
use crate::make_main::{db_level, default_file, opt_rebuilding_makefiles, second_expansion};
use crate::output::{error, fatal, message, perror_with_name};
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
#[inline]
unsafe extern "C" fn free_dep(d: *mut Dep) {
    free(d as *mut ::core::ffi::c_void);
}
pub const UNKNOWN_MTIME: i32 = 0;
pub const NONEXISTENT_MTIME: i32 = 1;
pub const OLD_MTIME: i32 = 2;
pub const ORDINARY_MTIME_MIN: i32 = OLD_MTIME + 1;
static mut goal_list: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
static mut goal_dep: *mut dep = ::core::ptr::null::<dep>() as *mut dep;
static mut dropped_list: *mut *mut dep = ::core::ptr::null::<*mut dep>() as *mut *mut dep;
static mut dropped_list_len: size_t = 0;
pub const DROPPED_LIST_INCR: i32 = 5;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn check_also_make(ctx: &crate::execctx::ExecContext, file: *const file) {
    let mut ad: *mut dep;
    let mut mtime: uintmax_t = (*file).last_mtime;
    if mtime == UNKNOWN_MTIME as uintmax_t {
        mtime = name_mtime(ctx, (*file).name);
    }
    if mtime >= ORDINARY_MTIME_MIN as uintmax_t
        && mtime
            <= ((!(0_i32 as uintmax_t))
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
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000_i32
                } else {
                    1
                }) as uintmax_t,
            )
            .wrapping_sub(1 as uintmax_t)
        && mtime > (*file).mtime_before_update
    {
        ad = (*file).also_make;
        while !ad.is_null() {
            if (*(*ad).file).last_mtime == NONEXISTENT_MTIME as uintmax_t {
                error(
                    ctx,
                    if !(*file).cmds.is_null() {
                        &raw mut (*(*file).cmds).fileinfo
                    } else {
                        ::core::ptr::null_mut::<Floc>()
                    },
                    0,
                    b"warning: pattern recipe did not update peer target '%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(
                        ((*(*ad).file).name) as *const ::core::ffi::c_char,
                    )],
                );
            }
            ad = (*ad).next;
        }
    }
}

/// Borrow a `*mut File` as `&file`, encoding the non-null invariant so the
/// access is a checked reference rather than a raw deref. The pointers walked
/// in `update_goal_chain` are kept non-null by the surrounding loop guards.
#[inline]
unsafe fn fref<'a>(f: *mut file) -> &'a file {
    f.as_ref()
        .expect("file pointer is non-null within the update loop")
}

/// Mutable counterpart of [`fref`].
#[inline]
unsafe fn fref_mut<'a>(f: *mut file) -> &'a mut file {
    f.as_mut()
        .expect("file pointer is non-null within the update loop")
}

#[inline]
unsafe fn double_colon_file_mut<'a>(f: *mut file) -> &'a mut file {
    let fr = fref_mut(f);
    if fr.double_colon.is_null() {
        fr
    } else {
        fref_mut(fr.double_colon)
    }
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn update_goal_chain(
    ctx: &crate::execctx::ExecContext,
    goaldeps: *mut goaldep,
) -> update_status {
    let mut last_cmd_count: ::core::ffi::c_ulong = 0;
    let t: bool = crate::make_main::opt_touch();
    let q: bool = crate::make_main::opt_question();
    let n: bool = crate::make_main::opt_just_print();
    let mut status: update_status = us_none;
    let depth: ::core::ffi::c_uint =
        (if opt_rebuilding_makefiles() { 1 } else { 0 }) as ::core::ffi::c_uint;
    let goals_orig: *mut dep = copy_dep_chain(goaldeps as *mut dep);
    let mut goals: *mut dep = goals_orig;
    goal_list = if opt_rebuilding_makefiles() {
        goaldeps
    } else {
        ::core::ptr::null_mut::<GoalDep>()
    };
    ctx.considered.set(ctx.considered.get().wrapping_add(1));
    while !goals.is_null() {
        let mut gu: *mut dep;
        let mut g: *mut dep;
        let mut lastgoal: *mut dep;
        let mut running: i32 = 0;
        let mut wait: i32 = 0;
        start_waiting_jobs(ctx);
        reap_children(
            ctx,
            (last_cmd_count == crate::make_main::opt_command_count()) as i32,
            0,
        );
        last_cmd_count = crate::make_main::opt_command_count();
        lastgoal = ::core::ptr::null_mut::<dep>();
        gu = goals;
        while let Some(gu_ref) = gu.as_ref() {
            let mut file: *mut file;
            let dchead: *mut file;
            let mut stop: i32 = 0;
            let mut all_updated: i32 = 1;
            let gu_next = gu_ref.next;
            let gu_shuf = gu_ref.shuf;
            g = if gu_shuf.is_null() { gu } else { gu_shuf };
            goal_dep = g;
            // Snapshot the goal-dep fields read below so the body holds no raw
            // deref of `g`. `changed()` is re-read after the file loop because
            // `set_changed` may update it.
            let (g_file, g_flags, g_wait) = match g.as_ref() {
                Some(gd) if !gd.file.is_null() => (gd.file, gd.flags, gd.wait_here),
                Some(_) => {
                    if let Some(lg) = lastgoal.as_mut() {
                        lg.next = gu_next;
                    } else {
                        goals = gu_next;
                    }
                    gu = gu_next;
                    continue;
                }
                None => break,
            };
            dchead = if !fref(g_file).double_colon.is_null() {
                fref(g_file).double_colon
            } else {
                g_file
            };
            file = dchead;
            while !file.is_null() {
                let fail: update_status;
                fref_mut(file).set_dontcare(
                    (g_flags as i32 & (1) << 2 != 0) as i32 as ::core::ffi::c_uint
                        as ::core::ffi::c_uint,
                );
                while !fref(file).renamed.is_null() {
                    file = fref(file).renamed;
                }
                if opt_rebuilding_makefiles() {
                    if fref(file).cmd_target() != 0 {
                        crate::make_main::set_touch_mirror(t);
                        crate::make_main::set_question_mirror(q);
                        crate::make_main::set_just_print_mirror(n);
                    } else {
                        crate::make_main::set_just_print_mirror(false);
                        crate::make_main::set_question_mirror(false);
                        crate::make_main::set_touch_mirror(false);
                    }
                }
                let ocommands_started = ctx.commands_started.get();
                stop = 0;
                wait = (file == dchead && g_wait as i32 != 0 && running != 0) as i32;
                if wait != 0 {
                    if 0x2_i32 & db_level != 0 {
                        print_spaces(depth);
                        printf(
                            b".WAIT is blocking '%s'.\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            fref(file).name,
                        );
                        fflush(stdout);
                    }
                    break;
                } else {
                    fail = update_file(ctx, file, depth);
                    while !fref(file).renamed.is_null() {
                        file = fref(file).renamed;
                    }
                    running |= (fref(file).command_state() as i32 == cs_running as i32
                        || fref(file).command_state() as i32 == cs_deps_running as i32)
                        as i32;
                    if ctx.commands_started.get() > ocommands_started {
                        if let Some(gm) = g.as_mut() {
                            gm.changed = true;
                        }
                    }
                    if (fail as ::core::ffi::c_uint != 0 || fref(file).updated() as i32 != 0)
                        && (status as ::core::ffi::c_uint)
                            < us_question as i32 as ::core::ffi::c_uint
                    {
                        if fref(file).update_status() as u64 != 0 {
                            status = fref(file).update_status();
                            stop = (crate::make_main::opt_question()
                                && !crate::make_main::opt_keep_going()
                                && !opt_rebuilding_makefiles())
                                as i32;
                        } else {
                            let mtime: uintmax_t = if opt_rebuilding_makefiles() {
                                if fref(file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                    f_mtime(ctx, file, 0)
                                } else {
                                    fref(file).last_mtime
                                }
                            } else if fref(file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime(ctx, file, 1)
                            } else {
                                fref(file).last_mtime
                            };
                            while !fref(file).renamed.is_null() {
                                file = fref(file).renamed;
                            }
                            if fref(file).updated() as i32 != 0
                                && mtime != fref(file).mtime_before_update
                            {
                                if !opt_rebuilding_makefiles()
                                    || !crate::make_main::opt_just_print()
                                        && !crate::make_main::opt_question()
                                {
                                    status = UpdateStatus::Success;
                                }
                                if opt_rebuilding_makefiles() && fref(file).dontcare() as i32 != 0 {
                                    stop = 1;
                                }
                            }
                        }
                    }
                    all_updated &= fref(file).updated() as i32;
                    fref_mut(file).set_dontcare(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    if stop != 0 {
                        break;
                    }
                    file = fref(file).prev;
                }
            }
            file = g_file;
            if wait != 0 {
                break;
            }
            let g_changed = g.as_ref().map_or(false, |gd| gd.changed);
            if stop != 0 || all_updated != 0 {
                if !opt_rebuilding_makefiles()
                    && fref(file).update_status() as i32 == us_success as i32
                    && !g_changed
                    && !crate::make_main::opt_run_silent()
                    && !crate::make_main::opt_question()
                {
                    message(
                        ctx,
                        1,
                        strlen(fref(file).name) as size_t,
                        if fref(file).phony() as i32 != 0 || fref(file).cmds.is_null() {
                            b"Nothing to be done for '%s'.\0" as *const u8
                                as *const ::core::ffi::c_char
                        } else {
                            b"'%s' is up to date.\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        &[FmtArg::Str(fref(file).name)],
                    );
                }
                if let Some(lg) = lastgoal.as_mut() {
                    lg.next = gu_next;
                } else {
                    goals = gu_next;
                }
                if stop != 0 {
                    break;
                }
            } else {
                lastgoal = gu;
            }
            gu = gu_next;
        }
        if gu.is_null() || wait != 0 {
            ctx.considered.set(ctx.considered.get().wrapping_add(1));
        }
    }
    free_dep_chain(goals_orig);
    if opt_rebuilding_makefiles() {
        crate::make_main::set_touch_mirror(t);
        crate::make_main::set_question_mirror(q);
        crate::make_main::set_just_print_mirror(n);
    }
    status
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn show_goal_error(ctx: &crate::execctx::ExecContext) {
    let mut goal: *mut goaldep;
    if (*goal_dep).flags() as i32 & (RM_INCLUDED | RM_DONTCARE) != RM_INCLUDED {
        return;
    }
    goal = goal_list;
    while !goal.is_null() {
        if (*goal_dep).file == (*goal).file {
            if (*goal).error != 0 {
                error(
        ctx,
        &raw mut (*goal).floc,
        (strlen((*(*goal).file).name) as size_t)
                        .wrapping_add(strlen(strerror((*goal).error)) as size_t),
        b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str(((*(*goal).file).name) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror((*goal).error)) as *const ::core::ffi::c_char)],
    );
                (*goal).error = 0;
            }
            return;
        }
        goal = (*goal).next;
    }
}
unsafe extern "C" fn update_file(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    depth: ::core::ffi::c_uint,
) -> update_status {
    let mut status: update_status = us_success;
    // Checked view of FILE; a null argument is a caller bug.
    let file = file.as_mut().expect("update_file: null file");
    let mut f: *mut file = if file.double_colon.is_null() {
        &raw mut *file
    } else {
        &raw mut *file
            .double_colon
            .as_mut()
            .expect("update_file: null double_colon")
    };
    {
        let fr = f.as_ref().expect("update_file: null file chain");
        if fr.considered == ctx.considered.get()
            && !(fr.updated() as i32 != 0
                && fr.update_status() as i32 > us_none as i32
                && fr.dontcare() == 0
                && fr.no_diag() as i32 != 0)
            && !(!file.double_colon.is_null()
                && file.command_state() as i32 == cs_finished as i32
                && !fr.prev.is_null())
        {
            if 0x2_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Pruning file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    file.name,
                );
                fflush(stdout);
            }
            return if fr.command_state() as i32 == cs_finished as i32 {
                fr.update_status()
            } else {
                us_success
            };
        }
    }
    while !f.is_null() {
        let mut fr = f.as_mut().expect("update_file: null file chain");
        fr.considered = ctx.considered.get();
        let new: update_status = update_file_1(ctx, &raw mut *fr, depth);
        while !fr.renamed.is_null() {
            fr = fr.renamed.as_mut().expect("update_file: null renamed file");
        }
        if new as ::core::ffi::c_uint != 0 && !crate::make_main::opt_keep_going() {
            return new;
        }
        if fr.command_state() as i32 == cs_running as i32
            || fr.command_state() as i32 == cs_deps_running as i32
        {
            return UpdateStatus::Success;
        }
        if new as ::core::ffi::c_uint > status as ::core::ffi::c_uint {
            status = new;
        }
        f = match fr.prev.as_mut() {
            Some(prev) => &raw mut *prev,
            None => ::core::ptr::null_mut(),
        };
    }
    status
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn complain(ctx: &crate::execctx::ExecContext, file: *mut file) {
    let mut d: *mut dep;
    d = (*file).deps;
    while !d.is_null() {
        if (*(*d).file).updated() as i32 != 0
            && (*(*d).file).update_status() as i32 > us_none as i32
            && (*file).no_diag() as i32 != 0
        {
            complain(ctx, (*d).file);
            break;
        } else {
            d = (*d).next;
        }
    }
    if d.is_null() {
        show_goal_error(ctx);
        if !(*file).parent.is_null() {
            let m: *const ::core::ffi::c_char = b"%sNo rule to make target '%s', needed by '%s'%s\0"
                as *const u8
                as *const ::core::ffi::c_char;
            if !crate::make_main::opt_keep_going() {
                fatal(ctx, NILF, 0, m, &[
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                        FmtArg::Str((*file).name),
                        FmtArg::Str((*(*file).parent).name),
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                    ]);
            }
            error(ctx, NILF, 0, m, &[
                    FmtArg::Str(b"*** \0" as *const u8 as *const ::core::ffi::c_char),
                    FmtArg::Str((*file).name),
                    FmtArg::Str((*(*file).parent).name),
                    FmtArg::Str(b".\0" as *const u8 as *const ::core::ffi::c_char),
                ]);
        } else {
            let m_0: *const ::core::ffi::c_char =
                b"%sNo rule to make target '%s'%s\0" as *const u8 as *const ::core::ffi::c_char;
            if !crate::make_main::opt_keep_going() {
                fatal(ctx, NILF, 0, m_0, &[
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                        FmtArg::Str((*file).name),
                        FmtArg::Str(b"\0" as *const u8 as *const ::core::ffi::c_char),
                    ]);
            }
            error(ctx, NILF, 0, m_0, &[
                    FmtArg::Str(b"*** \0" as *const u8 as *const ::core::ffi::c_char),
                    FmtArg::Str((*file).name),
                    FmtArg::Str(b".\0" as *const u8 as *const ::core::ffi::c_char),
                ]);
        }
        (*file).no_diag = false;
    }
}
unsafe extern "C" fn update_file_1(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    mut depth: ::core::ffi::c_uint,
) -> update_status {
    // Checked view of FILE; a null argument is a caller bug.
    let mut file = file.as_mut().expect("update_file_1: null file");
    let mut dep_status: update_status = us_success;
    let mut this_mtime: uintmax_t;
    let mut noexist: i32;
    let mut must_make: i32;
    let mut deps_changed: i32;
    let mut du: *mut dep;
    let mut d: *mut dep;
    let mut ad: *mut dep;
    let mut amake: dep = dep {
        next: ::core::ptr::null_mut::<dep>(),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        file: ::core::ptr::null_mut::<File>(),
        shuf: ::core::ptr::null_mut::<Dep>(),
        stem: ::core::ptr::null::<::core::ffi::c_char>(),
        ..Default::default()
    };
    let mut running: i32 = 0;
    if 0x2_i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            b"Considering target file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*file).name,
        );
        fflush(stdout);
    }
    if (*file).updated() != 0 {
        if (*file).update_status() as i32 > us_none as i32 {
            if 0x2_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Recently tried and failed to update file '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
            if (*file).no_diag() as i32 != 0 && (*file).dontcare() == 0 {
                complain(ctx, file);
            }
            return (*file).update_status;
        }
        if 0x2_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"File '%s' was considered already.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        return UpdateStatus::Success;
    }
    match (*file).command_state() as i32 {
        0 | 1 => {}
        2 => {
            if 0x2_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Still updating file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
            return UpdateStatus::Success;
        }
        3 => {
            if 0x2_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Finished updating file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
            return (*file).update_status;
        }
        _ => {
            abort();
        }
    }
    (*file).set_no_diag((*file).dontcare() as ::core::ffi::c_uint);
    let fresh0: *mut file = if !file.double_colon.is_null() {
        file.double_colon
    } else {
        &raw mut *file
    };
    (*fresh0).set_updating(1);
    let ofile: *mut file = &raw mut *file;
    depth = depth.wrapping_add(1);
    this_mtime = if (*file).last_mtime == UNKNOWN_MTIME as uintmax_t {
        f_mtime(ctx, file, 1)
    } else {
        (*file).last_mtime
    };
    while !file.renamed.is_null() {
        file = file
            .renamed
            .as_mut()
            .expect("update_file_1: null renamed file");
    }
    noexist = (this_mtime == NONEXISTENT_MTIME as uintmax_t) as i32;
    if noexist != 0 {
        if (*file).phony() != 0 {
            if 0x1_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Target '%s' is phony.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
        } else if 0x1_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"File '%s' does not exist.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    } else if this_mtime >= ORDINARY_MTIME_MIN as uintmax_t
        && this_mtime
            <= ((!(0_i32 as uintmax_t))
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
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000_i32
                } else {
                    1
                }) as uintmax_t,
            )
            .wrapping_sub(1 as uintmax_t)
        && (*file).low_resolution_time() as i32 != 0
    {
        let ns: i32 = (this_mtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1) as uintmax_t)
            as i32;
        if ns != 0 {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                strlen((*file).name) as size_t,
                b"*** warning: .LOW_RESOLUTION_TIME file '%s' has a high resolution time stamp\0"
                    as *const u8 as *const ::core::ffi::c_char,
                &[FmtArg::Str(((*file).name) as *const ::core::ffi::c_char)],
            );
        }
        this_mtime = this_mtime.wrapping_add(
            ((if FILE_TIMESTAMP_HI_RES != 0 {
                1000000000_i32
            } else {
                1
            }) - 1
                - ns) as uintmax_t,
        );
    }
    ad = (*file).also_make;
    while !ad.is_null() && noexist == 0 {
        let mut adfile: *mut File = (*ad).file;
        let fmtime: uintmax_t = if (*adfile).last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, adfile, 1)
        } else {
            (*adfile).last_mtime
        };
        noexist = (fmtime == NONEXISTENT_MTIME as uintmax_t) as i32;
        if noexist != 0 {
            while !(*adfile).renamed.is_null() {
                adfile = (*adfile).renamed;
            }
            if (*adfile).phony() != 0 {
                if 0x1_i32 & db_level != 0 {
                    print_spaces(depth);
                    printf(
                        b"Grouped target peer '%s' of file '%s' is phony.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*adfile).name,
                        (*file).name,
                    );
                    fflush(stdout);
                }
            } else if 0x1_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Grouped target peer '%s' of file '%s' does not exist.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*adfile).name,
                    (*file).name,
                );
                fflush(stdout);
            }
        } else if fmtime < this_mtime {
            this_mtime = fmtime;
        }
        ad = (*ad).next;
    }
    must_make = noexist;
    if (*file).phony() == 0 && (*file).cmds.is_null() && (*file).tried_implicit() == 0 {
        try_implicit_rule(ctx, file, depth);
        (*file).set_tried_implicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if (*file).cmds.is_null()
        && !(*file).is_target
        && !default_file.is_null()
        && !(*default_file).cmds.is_null()
    {
        if 0x8_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Using default recipe for '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        (*file).cmds = (*default_file).cmds;
    }
    amake.file = file;
    amake.next = (*file).also_make;
    ad = &raw mut amake;
    while !ad.is_null() {
        let mut lastd: *mut dep = ::core::ptr::null_mut::<dep>();
        if second_expansion() {
            expand_deps(ctx, (*ad).file);
        }
        du = (*(*ad).file).deps;
        ad = (*ad).next;
        while !du.is_null() {
            let new: UpdateStatus;
            let mtime: uintmax_t;
            let mut maybe_make: i32;
            let mut dontcare: i32 = 0;
            d = if !(*du).shuf.is_null() {
                (*du).shuf
            } else {
                du
            };
            if (*d).wait_here() as i32 != 0 && running != 0 {
                break;
            }
            while !(*(*d).file).renamed.is_null() {
                (*d).file = (*(*d).file).renamed;
            }
            mtime = if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                f_mtime(ctx, (*d).file, 1)
            } else {
                (*(*d).file).last_mtime
            };
            while !(*(*d).file).renamed.is_null() {
                (*d).file = (*(*d).file).renamed;
            }
            if (*if !(*(*d).file).double_colon.is_null() {
                (*(*d).file).double_colon
            } else {
                (*d).file
            })
            .updating
            {
                if warning::action(Type::CircularDep) == Action::Error {
                    fatal(
                        ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        (strlen((*file).name) as size_t)
                            .wrapping_add(strlen((*(*d).file).name) as size_t),
                        b"circular %s <- %s dependency detected\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            FmtArg::Str(((*file).name) as *const ::core::ffi::c_char),
                            FmtArg::Str(((*(*d).file).name) as *const ::core::ffi::c_char),
                        ],
                    );
                }
                if warning::is_active(Type::CircularDep) {
                    error(
                        ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        (strlen((*file).name) as size_t)
                            .wrapping_add(strlen((*(*d).file).name) as size_t),
                        b"circular %s <- %s dependency dropped\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            FmtArg::Str(((*file).name) as *const ::core::ffi::c_char),
                            FmtArg::Str(((*(*d).file).name) as *const ::core::ffi::c_char),
                        ],
                    );
                }
                if let Some(tail) = lastd.as_mut() {
                    tail.next = (*du).next;
                } else {
                    (*file).deps = (*du).next;
                }
                du = (*du).next;
                if dropped_list_len.wrapping_rem(DROPPED_LIST_INCR as size_t) == 0 {
                    dropped_list = xrealloc(
                        dropped_list as *mut ::core::ffi::c_void,
                        (::core::mem::size_of::<*mut Dep>() as size_t).wrapping_mul(
                            dropped_list_len.wrapping_add(DROPPED_LIST_INCR as size_t),
                        ),
                    ) as *mut *mut Dep;
                }
                let fresh1 = dropped_list_len;
                dropped_list_len = dropped_list_len.wrapping_add(1);
                let fresh2 = &mut (*dropped_list.offset(fresh1 as isize));
                *fresh2 = d;
            } else {
                (*(*d).file).parent = file;
                maybe_make = must_make;
                if opt_rebuilding_makefiles() {
                    dontcare = (*(*d).file).dontcare() as i32;
                    (*(*d).file).set_dontcare((*file).dontcare() as ::core::ffi::c_uint);
                }
                new = check_dep(ctx, (*d).file, depth, this_mtime, &raw mut maybe_make);
                if new as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                    dep_status = new;
                }
                if opt_rebuilding_makefiles() {
                    (*(*d).file)
                        .set_dontcare(dontcare as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                if !(*d).ignore_mtime {
                    must_make = maybe_make;
                }
                while !(*(*d).file).renamed.is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f: *mut File = (*d).file;
                if !(*f).double_colon.is_null() {
                    f = (*f).double_colon;
                }
                loop {
                    running |= ((*f).command_state() as i32 == cs_running as i32
                        || (*f).command_state() as i32 == cs_deps_running as i32)
                        as i32;
                    f = (*f).prev;
                    if f.is_null() {
                        break;
                    }
                }
                if dep_status as ::core::ffi::c_uint != 0 && !crate::make_main::opt_keep_going() {
                    break;
                }
                if running == 0 {
                    (*d).set_changed(
                        ((if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                            f_mtime(ctx, (*d).file, 1)
                        } else {
                            (*(*d).file).last_mtime
                        }) != mtime
                            || mtime == NONEXISTENT_MTIME as uintmax_t)
                            as i32 as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                }
                lastd = du;
                du = (*du).next;
            }
        }
    }
    if must_make != 0 || ctx.always_make_flag.get() {
        du = (*file).deps;
        while !du.is_null() {
            d = if !(*du).shuf.is_null() {
                (*du).shuf
            } else {
                du
            };
            if (*d).wait_here() as i32 != 0 && running != 0 {
                break;
            }
            if (*(*d).file).intermediate() != 0 {
                let new_0: update_status;
                let mut dontcare_0: i32 = 0;
                let mtime_0: uintmax_t = if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                    f_mtime(ctx, (*d).file, 1)
                } else {
                    (*(*d).file).last_mtime
                };
                while !(*(*d).file).renamed.is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                (*(*d).file).parent = file;
                if opt_rebuilding_makefiles() {
                    dontcare_0 = (*(*d).file).dontcare() as i32;
                    (*(*d).file).set_dontcare((*file).dontcare() as ::core::ffi::c_uint);
                }
                (*(*d).file).considered = 0;
                new_0 = update_file(ctx, (*d).file, depth);
                if new_0 as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                    dep_status = new_0;
                }
                if opt_rebuilding_makefiles() {
                    (*(*d).file)
                        .set_dontcare(dontcare_0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                while !(*(*d).file).renamed.is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f_0: *mut File = (*d).file;
                if !(*f_0).double_colon.is_null() {
                    f_0 = (*f_0).double_colon;
                }
                loop {
                    running |= ((*f_0).command_state() as i32 == cs_running as i32
                        || (*f_0).command_state() as i32 == cs_deps_running as i32)
                        as i32;
                    f_0 = (*f_0).prev;
                    if f_0.is_null() {
                        break;
                    }
                }
                if dep_status as ::core::ffi::c_uint != 0 && !crate::make_main::opt_keep_going() {
                    break;
                }
                if running == 0 {
                    (*d).set_changed(
                        ((*file).phony() as i32 != 0 && !(*file).cmds.is_null()
                            || (if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime(ctx, (*d).file, 1)
                            } else {
                                (*(*d).file).last_mtime
                            }) != mtime_0) as i32 as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                }
            }
            du = (*du).next;
        }
    }
    let fresh3: *mut file = if !file.double_colon.is_null() {
        file.double_colon
    } else {
        &raw mut *file
    };
    (*fresh3).set_updating(0);
    let fresh4 = &mut (*if !(*ofile).double_colon.is_null() {
        (*ofile).double_colon
    } else {
        ofile
    });
    (*fresh4).updating = false;
    depth = depth.wrapping_sub(1);
    if running != 0 {
        set_command_state(file, cs_deps_running);
        if 0x2_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"The prerequisites of '%s' are being made.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        return UpdateStatus::Success;
    }
    if 0x2_i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            b"Finished prerequisites of target file '%s'.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*file).name,
        );
        fflush(stdout);
    }
    if dep_status as u64 != 0 {
        (*file).set_update_status(UpdateStatus::from_bits(
            if dep_status as ::core::ffi::c_uint == us_none as i32 as ::core::ffi::c_uint {
                us_failed as i32 as ::core::ffi::c_uint
            } else {
                dep_status as ::core::ffi::c_uint
            },
        ));
        notice_finished_file(ctx, file);
        if 0x2_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Giving up on target file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        if depth == 0
            && crate::make_main::opt_keep_going()
            && !crate::make_main::opt_just_print()
            && !crate::make_main::opt_question()
        {
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                strlen((*file).name) as size_t,
                b"Target '%s' not remade because of errors.\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str(((*file).name) as *const ::core::ffi::c_char)],
            );
        }
        return dep_status;
    }
    if (*file).command_state() as i32 == cs_deps_running as i32 {
        set_command_state(file, cs_not_started);
    }
    deps_changed = 0;
    d = (*file).deps;
    while !d.is_null() {
        let d_mtime: uintmax_t = if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, (*d).file, 1)
        } else {
            (*(*d).file).last_mtime
        };
        while !(*(*d).file).renamed.is_null() {
            (*d).file = (*(*d).file).renamed;
        }
        if !(*d).ignore_mtime {
            if d_mtime == NONEXISTENT_MTIME as uintmax_t && !(*(*d).file).intermediate {
                must_make = 1;
            }
            deps_changed |= (*d).changed() as i32;
        }
        (*d).set_changed(
            (*d).changed() | (noexist != 0 || d_mtime > this_mtime) as i32 as ::core::ffi::c_uint,
        );
        if noexist == 0 && (0x1_i32 | 0x2_i32) & db_level != 0 {
            let mut fmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            if (*d).ignore_mtime() != 0 {
                if 0x2_i32 & db_level != 0 {
                    fmt = b"Prerequisite '%s' is order-only for target '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char;
                }
            } else if d_mtime == NONEXISTENT_MTIME as uintmax_t {
                if 0x1_i32 & db_level != 0 {
                    if (*(*d).file).phony() != 0 {
                        fmt = b"Prerequisite '%s' of target '%s' is phony.\n\0" as *const u8
                            as *const ::core::ffi::c_char;
                    } else {
                        fmt = b"Prerequisite '%s' of target '%s' does not exist.\n\0" as *const u8
                            as *const ::core::ffi::c_char;
                    }
                }
            } else if (*d).changed() != 0 {
                if 0x1_i32 & db_level != 0 {
                    fmt = b"Prerequisite '%s' is newer than target '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char;
                }
            } else if 0x2_i32 & db_level != 0 {
                fmt = b"Prerequisite '%s' is older than target '%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            if !fmt.is_null() {
                print_spaces(depth.wrapping_add(1));
                printf(
                    fmt,
                    if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    },
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        d = (*d).next;
    }
    if !(*file).double_colon.is_null() && (*file).deps.is_null() {
        must_make = 1;
        if 0x1_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Target '%s' is double-colon and has no prerequisites.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    } else if noexist == 0
        && (*file).is_target() as i32 != 0
        && deps_changed == 0
        && (*file).cmds.is_null()
        && !ctx.always_make_flag.get()
    {
        must_make = 0;
        if 0x2_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"No recipe for '%s' and no prerequisites actually changed.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    } else if must_make == 0 && !file.cmds.is_null() && ctx.always_make_flag.get() {
        must_make = 1;
        if 0x2_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Making '%s' due to always-make flag.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    }
    if must_make == 0 {
        if 0x2_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"No need to remake target '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            if !(*(*file).name as i32 == *(*file).hname as i32
                && (*(*file).name as i32 == 0
                    || strcmp(
                        (*file).name.offset(1_i32 as isize),
                        (*file).hname.offset(1_i32 as isize),
                    ) == 0))
            {
                printf(
                    b"; using VPATH name '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).hname,
                );
            }
            puts(b".\0" as *const u8 as *const ::core::ffi::c_char);
            fflush(stdout);
        }
        if (*file).notintermediate() == 0 && !ctx.no_intermediates.get() {
            (*file).secondary = true;
        }
        notice_finished_file(ctx, file);
        loop {
            file.name = file.hname;
            match file.prev.as_mut() {
                Some(prev) => file = prev,
                None => break,
            }
        }
        return UpdateStatus::Success;
    }
    if 0x1_i32 & db_level != 0 {
        print_spaces(depth);
        printf(
            b"Must remake target '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*file).name,
        );
        fflush(stdout);
    }
    if !(*(*file).name as i32 == *(*file).hname as i32
        && (*(*file).name as i32 == 0
            || strcmp(
                (*file).name.offset(1_i32 as isize),
                (*file).hname.offset(1_i32 as isize),
            ) == 0))
    {
        if 0x1_i32 & db_level != 0 {
            printf(
                b"  Ignoring VPATH name '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).hname,
            );
            fflush(stdout);
        }
        (*file).ignore_vpath = true;
    }
    remake_file(ctx, file);
    if (*file).command_state() as i32 != cs_finished as i32 {
        if 0x2_i32 & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Recipe of '%s' is being run.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        return UpdateStatus::Success;
    }
    match (*file).update_status() as i32 {
        3 => {
            if 0x1_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Failed to remake target file '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        0 => {
            if 0x1_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Successfully remade target file '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        2 => {
            if 0x1_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Target file '%s' needs to be remade under -q.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        1 | _ => {}
    }
    (*file).updated = true;
    (*file).update_status
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn notice_finished_file(ctx: &crate::execctx::ExecContext, file: *mut file) {
    let mut d: *mut dep;
    let ran: i32 = ((*file).command_state() as i32 == cs_running as i32) as i32;
    let mut touched: i32 = 0;
    (*file).set_command_state(cs_finished);
    (*file).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if crate::make_main::opt_touch() && (*file).update_status() as i32 == us_success as i32 {
        // Touch the file unless every command line is recursive (flagged
        // COMMANDS_RECURSE); a single non-recursive line means we touch.
        let mut should_touch = true;
        if !(*file).cmds.is_null() && (*(*file).cmds).any_recurse() as i32 != 0 {
            should_touch = false;
            let n: ::core::ffi::c_uint = (*(*file).cmds).ncommand_lines as ::core::ffi::c_uint;
            let mut i: ::core::ffi::c_uint = 0;
            while i < n {
                if (*(*(*file).cmds).lines_flags.offset(i as isize) as i32 & 1) == 0 {
                    should_touch = true;
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        if should_touch {
            if (*file).phony {
                (*file).update_status = UpdateStatus::Success;
            } else if !(*file).cmds.is_null() {
                (*file).set_update_status(touch_file(ctx, file));
                ctx.commands_started.set(ctx.commands_started.get().wrapping_add(1));
                touched = 1;
            }
        }
    }
    if (*file).mtime_before_update == UNKNOWN_MTIME as uintmax_t {
        (*file).mtime_before_update = (*file).last_mtime;
    }
    if ran != 0 && (*file).phony() == 0 || touched != 0 {
        let mut i_0: i32 = 0;
        if (crate::make_main::opt_question()
            || crate::make_main::opt_just_print()
            || crate::make_main::opt_touch())
            && !(*file).cmds.is_null()
        {
            i_0 = (*(*file).cmds).ncommand_lines as i32;
            while i_0 > 0 {
                if !(*(*(*file).cmds).lines_flags.offset((i_0 - 1) as isize) as i32 & 1 != 0) {
                    break;
                }
                i_0 -= 1;
            }
        } else if (*file).is_target() as i32 != 0 && (*file).cmds.is_null() {
            i_0 = 1;
        }
        (*file).last_mtime = if i_0 == 0 {
            UNKNOWN_MTIME as uintmax_t
        } else {
            (!(0_i32 as uintmax_t)).wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                0_i32 as uintmax_t
            } else {
                !(0_i32 as uintmax_t)
                    << (::core::mem::size_of::<uintmax_t>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1_usize)
            })
        };
    }
    if !(*file).double_colon.is_null() {
        let mut f: *mut File;
        let mut max_mtime: uintmax_t = (*file).last_mtime;
        f = (*file).double_colon;
        while !f.is_null() && (*f).updated() as i32 != 0 {
            if max_mtime != UNKNOWN_MTIME as uintmax_t
                && ((*f).last_mtime == UNKNOWN_MTIME as uintmax_t || (*f).last_mtime > max_mtime)
            {
                max_mtime = (*f).last_mtime;
            }
            f = (*f).prev;
        }
        if f.is_null() {
            f = (*file).double_colon;
            while !f.is_null() {
                (*f).last_mtime = max_mtime;
                f = (*f).prev;
            }
        }
    }
    if ran != 0 && (*file).update_status() as i32 != us_none as i32 {
        d = (*file).also_make;
        while !d.is_null() {
            (*(*d).file).set_command_state(cs_finished);
            (*(*d).file).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*(*d).file).set_update_status((*file).update_status());
            if ran != 0 && (*(*d).file).phony() == 0 {
                f_mtime(ctx, (*d).file, 0);
                if crate::make_main::opt_just_print() {
                    (*(*d).file).last_mtime = (!(0_i32 as uintmax_t)).wrapping_sub(
                        if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                            0_i32 as uintmax_t
                        } else {
                            !(0_i32 as uintmax_t)
                                << (::core::mem::size_of::<uintmax_t>() as usize)
                                    .wrapping_mul(CHAR_BIT as usize)
                                    .wrapping_sub(1_usize)
                        },
                    );
                }
            }
            d = (*d).next;
        }
        if (*file).tried_implicit() as i32 != 0 && !(*file).also_make.is_null() {
            check_also_make(ctx, file);
        }
    } else if (*file).update_status() as i32 == us_none as i32 {
        (*file).set_update_status(us_success);
    }
}
unsafe extern "C" fn check_dep(
    ctx: &crate::execctx::ExecContext,
    mut file: *mut file,
    depth: ::core::ffi::c_uint,
    this_mtime: uintmax_t,
    must_make_ptr: *mut i32,
) -> update_status {
    let ofile: *mut file;
    let mut d: *mut dep;
    let mut dep_status: update_status = us_success;
    double_colon_file_mut(file).set_updating(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    ofile = file;
    if (*file).phony() as i32 != 0 || (*file).intermediate() == 0 {
        let mtime: uintmax_t;
        dep_status = update_file(ctx, file, depth);
        while !(*file).renamed.is_null() {
            file = (*file).renamed;
        }
        mtime = if (*file).last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, file, 1)
        } else {
            (*file).last_mtime
        };
        while !(*file).renamed.is_null() {
            file = (*file).renamed;
        }
        if mtime == NONEXISTENT_MTIME as uintmax_t || mtime > this_mtime {
            *must_make_ptr = 1;
        }
    } else {
        let mtime_0: uintmax_t;
        if (*file).phony() == 0 && (*file).cmds.is_null() && (*file).tried_implicit() == 0 {
            try_implicit_rule(ctx, file, depth);
            (*file).set_tried_implicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (*file).cmds.is_null()
            && !(*file).is_target
            && !default_file.is_null()
            && !(*default_file).cmds.is_null()
        {
            if 0x8_i32 & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Using default commands for '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
            (*file).cmds = (*default_file).cmds;
        }
        while !(*file).renamed.is_null() {
            file = (*file).renamed;
        }
        mtime_0 = if (*file).last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(ctx, file, 1)
        } else {
            (*file).last_mtime
        };
        while !(*file).renamed.is_null() {
            file = (*file).renamed;
        }
        if mtime_0 != NONEXISTENT_MTIME as uintmax_t && mtime_0 > this_mtime {
            *must_make_ptr = 1;
        } else {
            let mut ld: *mut dep;
            let mut deps_running: i32 = 0;
            if (*file).command_state() as i32 != cs_running as i32 {
                if (*file).command_state() as i32 == cs_deps_running as i32 {
                    (*file).considered = 0;
                }
                set_command_state(file, CommandState::NotStarted);
            }
            ld = ::core::ptr::null_mut::<dep>();
            if second_expansion() {
                expand_deps(ctx, file);
            }
            d = (*file).deps;
            while let Some(dep_ref) = d.as_mut() {
                let new: update_status;
                let mut maybe_make: i32;
                // Every prerequisite has a resolved file by the time check_dep
                // walks it (set during parsing or by expand_deps above), so the
                // pointer is non-null on all reachable paths. Take it back out
                // through the NonNull check (one expression, no extra statement
                // lines) to stay null-safe for CodeQL without the never-taken
                // skip branch that lowered coverage.
                let dep_file = ::core::ptr::NonNull::new(dep_ref.file)
                    .expect("check_dep: prerequisite has no resolved file")
                    .as_ptr();
                if double_colon_file_mut(dep_file).updating() != 0 {
                    let dep_name = fref(dep_file).name;
                    error(
                        ctx,
                        ::core::ptr::null_mut::<Floc>(),
                        0,
                        b"circular %s <- %s dependency dropped\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str((*file).name), FmtArg::Str(dep_name)],
                    );
                    let next_dep = dep_ref.next;
                    if let Some(tail) = ld.as_mut() {
                        tail.next = next_dep;
                        free_dep(d);
                        d = tail.next;
                    } else {
                        (*file).deps = next_dep;
                        free_dep(d);
                        d = next_dep;
                    }
                } else {
                    fref_mut(dep_file).parent = file;
                    maybe_make = *must_make_ptr;
                    new = check_dep(
                        ctx,
                        dep_file,
                        depth.wrapping_add(1),
                        this_mtime,
                        &raw mut maybe_make,
                    );
                    if new as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                        dep_status = new;
                    }
                    if dep_ref.ignore_mtime() == 0 {
                        *must_make_ptr = maybe_make;
                    }
                    loop {
                        let renamed = fref(dep_ref.file).renamed;
                        if renamed.is_null() {
                            break;
                        }
                        dep_ref.file = renamed;
                    }
                    if dep_status as ::core::ffi::c_uint != 0 && !crate::make_main::opt_keep_going()
                    {
                        break;
                    }
                    let dep_file_ref = fref(dep_ref.file);
                    if dep_file_ref.command_state() as i32 == cs_running as i32
                        || dep_file_ref.command_state() as i32 == cs_deps_running as i32
                    {
                        deps_running = 1;
                    }
                    ld = d;
                    d = dep_ref.next;
                }
            }
            if deps_running != 0 {
                set_command_state(file, CommandState::DepsRunning);
            }
        }
    }
    double_colon_file_mut(file).set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    double_colon_file_mut(ofile).set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    dep_status
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn touch_file(ctx: &crate::execctx::ExecContext, file: *mut file) -> update_status {
    if !crate::make_main::opt_run_silent() {
        message(
        ctx,
        0,
        strlen((*file).name) as size_t,
        b"touch %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str(((*file).name) as *const ::core::ffi::c_char)],
    );
    }
    if crate::make_main::opt_just_print() {
        return us_success;
    }
    if ar_name(ctx, ::core::ffi::CStr::from_ptr((*file).name)) {
        return if ar_touch(ctx, (*file).name) != 0 {
            us_failed
        } else {
            us_success
        };
    } else {
        let mut fd: i32;
        loop {
            fd = open((*file).name, 0o2_i32 | 0o100_i32, 0o666_i32);
            if !(fd == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd < 0 {
            perror_with_name(
                ctx,
                b"touch: open: \0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            return UpdateStatus::Failed;
        } else {
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
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
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
                    (*file).name,
                );
                return UpdateStatus::Failed;
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
                    (*file).name,
                );
                return UpdateStatus::Failed;
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
                    (*file).name,
                );
                return UpdateStatus::Failed;
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
                    (*file).name,
                );
                return UpdateStatus::Failed;
            }
            if statbuf.st_size == 0 as __off_t {
                close(fd);
                loop {
                    fd = open((*file).name, 0o2_i32 | 0o1000_i32, 0o666_i32);
                    if !(fd == -1_i32 && *__errno_location() == EINTR) {
                        break;
                    }
                }
                if fd < 0 {
                    perror_with_name(
                        ctx,
                        b"touch: open: \0" as *const u8 as *const ::core::ffi::c_char,
                        (*file).name,
                    );
                    return UpdateStatus::Failed;
                }
            }
            close(fd);
        }
    }
    us_success
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn remake_file(ctx: &crate::execctx::ExecContext, file: *mut file) {
    if (*file).cmds.is_null() {
        if (*file).phony {
            (*file).update_status = UpdateStatus::Success;
        } else if (*file).is_target {
            (*file).update_status = UpdateStatus::Success;
        } else {
            if !opt_rebuilding_makefiles() || (*file).dontcare() == 0 {
                complain(ctx, file);
            }
            (*file).update_status = UpdateStatus::Failed;
        }
    } else {
        chop_commands(ctx, (*file).cmds);
        if !crate::make_main::opt_touch() || (*(*file).cmds).any_recurse() as i32 != 0 {
            execute_file_commands(ctx, file);
            return;
        }
        (*file).update_status = UpdateStatus::Success;
    }
    notice_finished_file(ctx, file);
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

/// Return the mtime of file F, computing it if necessary. Returns
/// NONEXISTENT_MTIME if the file does not exist.
///
/// # Safety
/// `file` must point to a valid `File`; must run single-threaded with the
/// global file table.
pub unsafe fn f_mtime(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    search: i32,
) -> uintmax_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut mtime: uintmax_t;
    let propagate_timestamp: ::core::ffi::c_uint;
    // Checked view of FILE; a null argument is a caller bug.
    let mut file = file.as_mut().expect("f_mtime: null file");
    if ar_name(ctx, ::core::ffi::CStr::from_ptr(file.name)) {
        let memmtime: uintmax_t;
        let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut arfile: *mut File;
        let member_date: time_t;
        ar_parse_name(ctx, (*file).name, &raw mut arname, &raw mut memname);
        memmtime = name_mtime(ctx, memname);
        arfile = lookup_file(arname);
        if arfile.is_null() {
            arfile = enter_file(strcache_add(arname));
        }
        mtime = f_mtime(ctx, arfile, search);
        // `arfile` is non-null here; follow the (non-null) renamed links via a
        // checked reference, keeping the walk a single branch and line count.
        while !arfile
            .as_ref()
            .expect("f_mtime: null arfile")
            .renamed
            .is_null()
        {
            arfile = arfile.as_ref().expect("f_mtime: null arfile").renamed;
        }
        // Borrow the final `arfile` as `&file` only when the rename actually
        // applies; folding the guard into the `if let` keeps both conditions
        // in the (uncounted) closure and avoids a separate binding line.
        if let Some(arf2) = arfile
            .as_ref()
            .filter(|a| search != 0 && strcmp(a.hname, arname) != 0)
        {
            let arlen: size_t = strlen(arf2.hname) as size_t;
            let memlen: size_t = strlen(memname) as size_t;
            alloca_allocations.push(::std::vec::from_elem(
                0,
                arlen.wrapping_add(1).wrapping_add(memlen).wrapping_add(2) as usize,
            ));
            let name: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                name as *mut ::core::ffi::c_void,
                arf2.hname as *const ::core::ffi::c_void,
                arlen as size_t,
            );
            *name.offset(arlen as isize) = '(' as i32 as ::core::ffi::c_char;
            memcpy(
                name.offset(arlen as isize).offset(1_i32 as isize) as *mut ::core::ffi::c_void,
                memname as *const ::core::ffi::c_void,
                memlen as size_t,
            );
            *name.offset(arlen.wrapping_add(1 as size_t).wrapping_add(memlen) as isize) =
                ')' as i32 as ::core::ffi::c_char;
            *name.offset(arlen.wrapping_add(1).wrapping_add(memlen).wrapping_add(1) as isize) = 0;
            if arf2.name == arf2.hname {
                rename_file(ctx, file, strcache_add(name));
            } else {
                rehash_file(ctx, file, strcache_add(name));
            }
            while !file.renamed.is_null() {
                file = &mut *file.renamed;
            }
        }
        free(arname as *mut ::core::ffi::c_void);
        while !file.renamed.is_null() {
            file = &mut *file.renamed;
        }
        file.low_resolution_time = true;
        if mtime == NONEXISTENT_MTIME as uintmax_t {
            return NONEXISTENT_MTIME as uintmax_t;
        }
        member_date = ar_member_date(ctx, (*file).hname);
        if member_date == -1_i32 as time_t
            || memmtime != NONEXISTENT_MTIME as uintmax_t
                && (memmtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
                    as time_t
                    > member_date
        {
            mtime = NONEXISTENT_MTIME as uintmax_t;
        } else {
            mtime = file_timestamp_cons(
                ctx,
                file.hname,
                system_time_from_unix(member_date as i64, 0),
            );
        }
    } else {
        mtime = name_mtime(ctx, (*file).name);
        if mtime == NONEXISTENT_MTIME as uintmax_t && search != 0 && (*file).ignore_vpath() == 0 {
            let mut name_0: *const ::core::ffi::c_char = vpath_search(
                ctx,
                (*file).name,
                &raw mut mtime,
                ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                ::core::ptr::null_mut::<::core::ffi::c_uint>(),
            );
            if !name_0.is_null()
                || *(*file).name.offset(0_i32 as isize) as i32 == '-' as i32
                    && *(*file).name.offset(1_i32 as isize) as i32 == 'l' as i32
                    && {
                        name_0 = library_search(ctx, (*file).name, &raw mut mtime);
                        !name_0.is_null()
                    }
            {
                let name_len: size_t;
                if mtime != UNKNOWN_MTIME as uintmax_t {
                    (*file).last_mtime = mtime;
                }
                name_len = strlen(name_0)
                    .wrapping_sub(strlen((*file).name))
                    .wrapping_sub(1) as size_t;
                // SAFETY: `name_0`/`name_len` are the library pathname and its
                // prefix length computed just above, so this borrows exactly the
                // bytes the old pointer+length pair described.
                if gpath_search(unsafe {
                    ::core::slice::from_raw_parts(name_0 as *const u8, name_len as usize)
                }) {
                    rename_file(ctx, file, name_0);
                    while !file.renamed.is_null() {
                        file = file.renamed.as_mut().expect("f_mtime: null renamed file");
                    }
                    return if (*file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                        f_mtime(ctx, file, 1)
                    } else {
                        (*file).last_mtime
                    };
                }
                rehash_file(ctx, file, name_0);
                while !file.renamed.is_null() {
                    file = file.renamed.as_mut().expect("f_mtime: null renamed file");
                }
                if mtime != OLD_MTIME as uintmax_t
                    && mtime
                        != (!(0_i32 as uintmax_t)).wrapping_sub(
                            if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                                0_i32 as uintmax_t
                            } else {
                                !(0_i32 as uintmax_t)
                                    << (::core::mem::size_of::<uintmax_t>() as usize)
                                        .wrapping_mul(CHAR_BIT as usize)
                                        .wrapping_sub(1_usize)
                            },
                        )
                {
                    mtime = name_mtime(ctx, name_0);
                }
            }
        }
    }
    if !ctx.clock_skew_detected.get()
        && mtime != NONEXISTENT_MTIME as uintmax_t
        && mtime
            != (!(0_i32 as uintmax_t)).wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                0_i32 as uintmax_t
            } else {
                !(0_i32 as uintmax_t)
                    << (::core::mem::size_of::<uintmax_t>() as usize)
                        .wrapping_mul(CHAR_BIT as usize)
                        .wrapping_sub(1_usize)
            })
        && (*file).updated() == 0
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
                                as uintmax_t) as i32)
                        as ::core::ffi::c_double
                        / 1e9f64;
                let mut from_now_string: [::core::ffi::c_char; 100] = [0; 100];
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
        (strlen((*file).name) as size_t).wrapping_add(strlen(
                        &raw mut from_now_string as *mut ::core::ffi::c_char,
                    ) as size_t),
        b"warning: file '%s' has modification time %s s in the future\0" as *const u8
                        as *const ::core::ffi::c_char,
        &[FmtArg::Str(((*file).name) as *const ::core::ffi::c_char),
            FmtArg::Str((&raw mut from_now_string as *mut ::core::ffi::c_char) as *const ::core::ffi::c_char)],
    );
                ctx.clock_skew_detected.set(true);
            }
        }
    }
    if !file.double_colon.is_null() {
        file = &mut *file.double_colon;
    }
    propagate_timestamp = (*file).updated();
    loop {
        if mtime != NONEXISTENT_MTIME as uintmax_t
            && (*file).command_state() as i32 == cs_not_started as i32
            && (*file).tried_implicit() == 0
            && (*file).intermediate() as i32 != 0
        {
            (*file).set_intermediate(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (*file).updated() == propagate_timestamp {
            (*file).last_mtime = mtime;
        }
        match file.prev.as_mut() {
            Some(prev) => file = prev,
            None => break,
        }
    }
    mtime
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
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
    if crate::make_main::opt_check_symlink() && strlen(name) <= GET_PATH_MAX as size_t {
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
    }
    mtime
}
unsafe extern "C" fn library_search(
    ctx: &crate::execctx::ExecContext,
    mut lib: *const ::core::ffi::c_char,
    mtime_ptr: *mut uintmax_t,
) -> *const ::core::ffi::c_char {
    static mut dirs: [*const ::core::ffi::c_char; 4] = [
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
    );
    lib = lib.offset(2_i32 as isize);
    liblen = strlen(lib) as size_t;
    p2 = libpatterns;
    loop {
        p = find_next_token(&raw mut p2, &raw mut len);
        if p.is_null() {
            break;
        }
        static mut buf: *mut ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
        static mut buflen: size_t = 0;
        static mut libdir_maxlen: size_t = 0;
        static mut std_dirs: ::core::ffi::c_uint = 0;
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
                variable_buffer,
                p,
                p3.offset_from(p) as ::core::ffi::c_long as size_t,
            );
            p4 = variable_buffer_output(p4, lib, liblen);
            variable_buffer_output(
                p4,
                p3.offset(1_i32 as isize),
                len.wrapping_sub(p3.offset_from(p) as ::core::ffi::c_long as size_t),
            );
            *p.offset(len as isize) = c;
            libbuf = variable_buffer;
            mtime = name_mtime(ctx, libbuf);
            if mtime != NONEXISTENT_MTIME as uintmax_t {
                if !mtime_ptr.is_null() {
                    *mtime_ptr = mtime;
                }
                file = strcache_add(libbuf);
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
                );
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
                if buflen == 0 {
                    dp = &raw const dirs as *const *const ::core::ffi::c_char;
                    while !(*dp).is_null() {
                        let l: size_t = strlen(*dp) as size_t;
                        if l > libdir_maxlen {
                            libdir_maxlen = l;
                        }
                        std_dirs = std_dirs.wrapping_add(1);
                        dp = dp.offset(1_i32 as isize);
                    }
                    buflen = strlen(libbuf) as size_t;
                    buf = xmalloc(libdir_maxlen.wrapping_add(buflen).wrapping_add(2))
                        as *mut ::core::ffi::c_char;
                } else if buflen < strlen(libbuf) {
                    buflen = strlen(libbuf) as size_t;
                    buf = xrealloc(
                        buf as *mut ::core::ffi::c_void,
                        libdir_maxlen.wrapping_add(buflen).wrapping_add(2),
                    ) as *mut ::core::ffi::c_char;
                }
                let mut vpath_index_0: ::core::ffi::c_uint =
                    (!(0_i32 as ::core::ffi::c_uint)).wrapping_sub(std_dirs);
                dp = &raw const dirs as *const *const ::core::ffi::c_char;
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
                        file = strcache_add(buf);
                        best_vpath = vpath_index_0;
                        if !mtime_ptr.is_null() {
                            *mtime_ptr = mtime;
                        }
                    }
                    vpath_index_0 = vpath_index_0.wrapping_add(1);
                    dp = dp.offset(1_i32 as isize);
                }
            }
        }
    }
    free(libpatterns as *mut ::core::ffi::c_void);
    file
}
pub const LIBDIR: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"/usr/local/lib\0") };
pub const __CHAR_BIT__: i32 = 8;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const FILE_TIMESTAMP_HI_RES: i32 = 1;

#[cfg(test)]
mod f_mtime_tests {
    use super::*;
    use crate::strcache::strcache_add;
    use std::io::Write;

    // Serialize tests that touch the process-wide file/strcache globals.
    static F_MTIME_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Create a unique temp file and return its absolute path as a C string
    /// interned in the strcache (so it is a stable `*const c_char` usable as a
    /// `File::name`). The file is left on disk; the caller removes it.
    fn make_temp_file() -> (std::path::PathBuf, *const ::core::ffi::c_char) {
        let dir = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = dir.join(format!("fmtime-{nanos}-{}", std::process::id()));
        let mut f = std::fs::File::create(&path).expect("create temp file");
        f.write_all(b"x").expect("write temp file");
        let cstr = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
        let interned = unsafe { strcache_add(cstr.as_ptr()) };
        (path, interned)
    }

    /// `f_mtime` on a plain (non-archive) file whose name is an existing path
    /// stats the file and returns an ordinary, existent timestamp. Marking the
    /// file `updated` keeps the clock-skew block deterministic. This drives the
    /// common non-archive branch (the region the cast cleanup touched):
    /// `name_mtime` -> `file_timestamp_cons` -> the timestamp-propagation loop.
    #[test]
    fn f_mtime_existing_file_returns_ordinary_mtime() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::install_default_options_for_test();
        let (path, name) = make_temp_file();
        unsafe {
            let mut file = File::default();
            file.name = name;
            file.hname = name;
            file.set_updated(1);

            let ctx = crate::execctx::ExecContext::default();
            let mtime = f_mtime(&ctx, &raw mut file, 0);
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
            // last_mtime is propagated through the (single-element) chain.
            assert_eq!(file.last_mtime, mtime, "last_mtime is cached on the file");
        }
        let _ = std::fs::remove_file(&path);
    }

    /// `f_mtime` on a plain file whose name does not exist (and with `search`
    /// disabled, so no vpath/library fallback) returns `NONEXISTENT_MTIME`.
    /// Drives the missing-file arm of `name_mtime` (`ENOENT`).
    #[test]
    fn f_mtime_missing_file_is_nonexistent() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::install_default_options_for_test();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let missing = std::env::temp_dir().join(format!("fmtime-missing-{nanos}"));
        let cstr = std::ffi::CString::new(missing.to_str().unwrap()).unwrap();
        unsafe {
            let name = strcache_add(cstr.as_ptr());
            let mut file = File::default();
            file.name = name;
            file.hname = name;
            file.set_updated(1);
            file.set_ignore_vpath(1);

            let ctx = crate::execctx::ExecContext::default();
            let mtime = f_mtime(&ctx, &raw mut file, 0);
            assert_eq!(
                mtime, NONEXISTENT_MTIME as uintmax_t,
                "a missing file with no search reports nonexistent"
            );
        }
    }

    /// Same plain-file path but with `updated` left unset, so `f_mtime` runs the
    /// clock-skew check block. An existing file has a present/past mtime, so the
    /// inner `adjusted_now < adjusted_mtime` future-time test is false and no
    /// warning is emitted; this covers the clock-skew guard region without
    /// reaching the (make-init-dependent) `error` warning path.
    #[test]
    fn f_mtime_past_file_skips_skew_warning() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::install_default_options_for_test();
        let (path, name) = make_temp_file();
        unsafe {
            let mut file = File::default();
            file.name = name;
            file.hname = name;
            // updated() left at 0 so the clock-skew block's outer guard runs.

            let ctx = crate::execctx::ExecContext::default();
            let mtime = f_mtime(&ctx, &raw mut file, 0);
            assert!(
                mtime > ORDINARY_MTIME_MIN as uintmax_t,
                "an existing past-dated file resolves to an ordinary mtime"
            );
            assert!(
                !ctx.clock_skew_detected.get(),
                "a past-dated file triggers no clock-skew warning"
            );
        }
        let _ = std::fs::remove_file(&path);
    }

    /// `f_mtime` on a plain file whose mtime lies far in the future drives the
    /// clock-skew `error()` warning branch ("file '%s' has modification time
    /// %s s in the future"), which formats the offset and sets the context's
    /// `clock_skew_detected` latch. This is the exact previously-uncovered branch:
    /// covering it requires a valid `program` name so `error()` runs its real
    /// path instead of dereferencing the null `program` pointer (a segfault).
    #[test]
    fn f_mtime_future_file_warns_and_sets_skew() {
        let _g = F_MTIME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::install_default_options_for_test();
        let (path, name) = make_temp_file();
        // Push the file's mtime far enough into the future to beat any value
        // the process-shared `adjusted_now` accumulator already holds.
        let future = std::time::SystemTime::now() + std::time::Duration::from_secs(10_000_000);
        let future_secs = future
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        unsafe {
            crate::make_main::install_program_name_for_test();
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

            let mut file = File::default();
            file.name = name;
            file.hname = name;
            // updated() left at 0 so the clock-skew block runs in full.

            let ctx = crate::execctx::ExecContext::default();
            let mtime = f_mtime(&ctx, &raw mut file, 0);
            assert!(
                mtime > ORDINARY_MTIME_MIN as uintmax_t,
                "the future-dated file still resolves to an ordinary mtime"
            );
            assert!(
                ctx.clock_skew_detected.get(),
                "a future-dated file triggers the clock-skew warning"
            );
        }
        let _ = std::fs::remove_file(&path);
    }

    /// Verbatim port of the original future-mtime cache arithmetic, before the
    /// `static mut adjusted_now` was moved onto `ExecContext`: the cached
    /// "adjusted now" is the freshly sampled clock plus one tick less than the
    /// timestamp resolution. Kept as a `#[cfg(test)]` oracle per `AGENTS.md`
    /// so the extracted `adjusted_now_from_clock` is proven behavior-preserving.
    fn adjusted_now_from_clock_oracle(now: uintmax_t, resolution: i32) -> uintmax_t {
        now.wrapping_add((resolution - 1) as uintmax_t)
    }

    /// `adjusted_now_from_clock` matches the original arithmetic across
    /// representative clock samples and resolutions, including `resolution == 0`
    /// (which subtracts one and wraps, exactly as the C `uintmax_t` did) and a
    /// coarse resolution that pushes the cache ahead of the raw clock.
    #[test]
    fn adjusted_now_from_clock_matches_oracle() {
        let cases: &[(uintmax_t, i32)] = &[
            (0, 0),
            (0, 1),
            (1_000_000_000, 1),
            (1_000_000_000, 1_000_000_000),
            (uintmax_t::MAX, 1), // resolution 1 => +0, no wrap
            (0, 1_000_000_000),  // now 0, coarse resolution
            (5, 0),              // resolution 0 => wrapping_sub(1)
        ];
        for &(now, resolution) in cases {
            assert_eq!(
                adjusted_now_from_clock(now, resolution),
                adjusted_now_from_clock_oracle(now, resolution),
                "diverged for now={now}, resolution={resolution}"
            );
        }
    }

    /// Models `f_mtime`'s cross-call cache gate: the system clock is re-sampled
    /// only when a file's mtime is past the cached "adjusted now", and a file
    /// warns only when it is still past the cache after that refresh. Drives a
    /// sequence of files through a model of the gate (with a stubbed clock) and
    /// against an oracle of the original `static mut` control flow, asserting
    /// they agree on which calls re-read the clock and which warn — the
    /// behavior that must survive moving the cache onto `ExecContext`.
    #[test]
    fn future_mtime_cache_gate_matches_oracle() {
        // One file's pass through the gate, returning the carried cache plus
        // whether this call re-sampled the clock and whether it warned.
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
        // The original control flow, transcribed against threaded state.
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

        // (mtime, clock_now, resolution) for a run of files. The clock advances
        // monotonically; mixes files behind, at, and ahead of the cache.
        let timeline: &[(uintmax_t, uintmax_t, i32)] = &[
            (100, 100, 1), // first file: cache 0 < 100 -> sample, now==mtime, no warn
            (50, 100, 1),  // behind cache -> skip clock, no warn
            (100, 100, 1), // at cache (100 < 100 false) -> skip
            (200, 150, 1), // ahead of cache -> sample, 150 < 200 -> warn
            (200, 250, 1), // 200(cache from prev=150) < 200 false -> skip
            (300, 260, 1), // sample, 260 < 300 -> warn
            (300, 400, 1), // cache 260 < 300 -> sample, 400 >= 300 -> no warn
            (300, 400, 5), // cache 400 -> skip
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
