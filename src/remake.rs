pub use crate::file::{CommandState, UpdateStatus};
pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __syscall_slong_t, __time_t, __uid_t, off_t, size_t, ssize_t, time_t, uintmax_t,
};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
use crate::misc::free_ns_chain;
use crate::misc::{copy_dep_chain, find_next_token, print_spaces, xmalloc, xrealloc};
use crate::stdio::FILE;
use crate::strcache::strcache_add;
use libc::{
    __errno_location, abort, close, free, open, printf, puts, sprintf, strcmp, strcpy, strerror,
    strrchr,
};
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    rename_file, set_command_state,
};
use crate::implicit::try_implicit_rule;
use crate::job::{reap_children, start_waiting_jobs};
use crate::make_main::{
    always_make_flag, check_symlink_flag, clock_skew_detected, command_count, db_level,
    default_file, just_print_flag, keep_going_flag, no_intermediates, question_flag,
    rebuilding_makefiles, run_silent, second_expansion, touch_flag,
};
use crate::output::{error, fatal, message, perror_with_name};
use crate::read::find_percent;
pub use crate::read::goaldep;
use crate::vpath::{gpath_search, vpath_search};
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4;
pub const ENOTDIR: ::core::ffi::c_int = 20;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2)
    .wrapping_add(1);
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const GET_PATH_MAX: ::core::ffi::c_int = PATH_MAX;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const RM_INCLUDED: ::core::ffi::c_int = (1) << 1;
pub const RM_DONTCARE: ::core::ffi::c_int = (1) << 2;
#[inline]
unsafe extern "C" fn free_ns(n: *mut nameseq) {
    free(n as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn free_dep(d: *mut dep) {
    free_ns(d as *mut nameseq);
}
#[inline]
unsafe extern "C" fn free_dep_chain(d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
pub const UNKNOWN_MTIME: ::core::ffi::c_int = 0;
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
pub const OLD_MTIME: ::core::ffi::c_int = 2;
pub const ORDINARY_MTIME_MIN: ::core::ffi::c_int = OLD_MTIME + 1;
pub static mut commands_started: ::core::ffi::c_uint = 0;
static mut goal_list: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
static mut goal_dep: *mut dep = ::core::ptr::null::<dep>() as *mut dep;
static mut considered: ::core::ffi::c_uint = 0;
static mut dropped_list: *mut *mut dep = ::core::ptr::null::<*mut dep>() as *mut *mut dep;
static mut dropped_list_len: size_t = 0;
pub const DROPPED_LIST_INCR: ::core::ffi::c_int = 5;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn check_also_make(file: *const file) {
    let mut ad: *mut dep;
    let mut mtime: uintmax_t = (*file).last_mtime;
    if mtime == UNKNOWN_MTIME as uintmax_t {
        mtime = name_mtime((*file).name);
    }
    if mtime >= ORDINARY_MTIME_MIN as uintmax_t
        && mtime
            <= ((!(0 as ::core::ffi::c_int as uintmax_t))
                .wrapping_sub(
                    if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                        0 as ::core::ffi::c_int as uintmax_t
                    } else {
                        !(0 as ::core::ffi::c_int as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8 as usize)
                                .wrapping_sub(1 as usize)
                    },
                )
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
                << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
            .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000 as ::core::ffi::c_int
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
                    if !(*file).cmds.is_null() {
                        &raw mut (*(*file).cmds).fileinfo
                    } else {
                        ::core::ptr::null_mut::<Floc>()
                    },
                    strlen((*(*ad).file).name) as size_t,
                    b"warning: pattern recipe did not update peer target '%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*(*ad).file).name,
                );
            }
            ad = (*ad).next;
        }
    }
}

/// Borrow a `*mut file` as `&file`, encoding the non-null invariant so the
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
pub unsafe fn update_goal_chain(goaldeps: *mut goaldep) -> update_status {
    let mut last_cmd_count: ::core::ffi::c_ulong = 0;
    let t: ::core::ffi::c_int = touch_flag;
    let q: ::core::ffi::c_int = question_flag;
    let n: ::core::ffi::c_int = just_print_flag;
    let mut status: update_status = us_none;
    let depth: ::core::ffi::c_uint =
        (if rebuilding_makefiles != 0 { 1 } else { 0 }) as ::core::ffi::c_uint;
    let goals_orig: *mut dep = copy_dep_chain(goaldeps as *mut dep);
    let mut goals: *mut dep = goals_orig;
    goal_list = if rebuilding_makefiles != 0 {
        goaldeps
    } else {
        ::core::ptr::null_mut::<goaldep>()
    };
    considered = considered.wrapping_add(1);
    while !goals.is_null() {
        let mut gu: *mut dep;
        let mut g: *mut dep;
        let mut lastgoal: *mut dep;
        let mut running: ::core::ffi::c_int = 0;
        let mut wait: ::core::ffi::c_int = 0;
        start_waiting_jobs();
        reap_children((last_cmd_count == command_count) as ::core::ffi::c_int, 0);
        last_cmd_count = command_count;
        lastgoal = ::core::ptr::null_mut::<dep>();
        gu = goals;
        while let Some(gu_ref) = gu.as_ref() {
            let mut file: *mut file;
            let dchead: *mut file;
            let mut stop: ::core::ffi::c_int = 0;
            let mut all_updated: ::core::ffi::c_int = 1;
            let gu_next = gu_ref.next;
            let gu_shuf = gu_ref.shuf;
            g = if gu_shuf.is_null() { gu } else { gu_shuf };
            goal_dep = g;
            // Snapshot the goal-dep fields read below so the body holds no raw
            // deref of `g`. `changed()` is re-read after the file loop because
            // `set_changed` may update it.
            let (g_file, g_flags, g_wait) = match g.as_ref() {
                Some(gd) if !gd.file.is_null() => (gd.file, gd.flags(), gd.wait_here()),
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
                let ocommands_started: ::core::ffi::c_uint;
                let fail: update_status;
                fref_mut(file).set_dontcare(
                    (g_flags as ::core::ffi::c_int & (1) << 2 != 0) as ::core::ffi::c_int
                        as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
                while !fref(file).renamed.is_null() {
                    file = fref(file).renamed;
                }
                if rebuilding_makefiles != 0 {
                    if fref(file).cmd_target() != 0 {
                        touch_flag = t;
                        question_flag = q;
                        just_print_flag = n;
                    } else {
                        just_print_flag = 0;
                        question_flag = just_print_flag;
                        touch_flag = question_flag;
                    }
                }
                ocommands_started = commands_started;
                stop = 0;
                wait = (file == dchead && g_wait as ::core::ffi::c_int != 0 && running != 0)
                    as ::core::ffi::c_int;
                if wait != 0 {
                    if 0x2 as ::core::ffi::c_int & db_level != 0 {
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
                    fail = update_file(file, depth);
                    while !fref(file).renamed.is_null() {
                        file = fref(file).renamed;
                    }
                    running |= (fref(file).command_state() as ::core::ffi::c_int
                        == cs_running as ::core::ffi::c_int
                        || fref(file).command_state() as ::core::ffi::c_int
                            == cs_deps_running as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    if commands_started > ocommands_started {
                        if let Some(gm) = g.as_mut() {
                            gm.set_changed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        }
                    }
                    if (fail as ::core::ffi::c_uint != 0
                        || fref(file).updated() as ::core::ffi::c_int != 0)
                        && (status as ::core::ffi::c_uint)
                            < us_question as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        if fref(file).update_status() as u64 != 0 {
                            status = fref(file).update_status() as update_status;
                            stop = (question_flag != 0
                                && keep_going_flag == 0
                                && rebuilding_makefiles == 0)
                                as ::core::ffi::c_int;
                        } else {
                            let mtime: uintmax_t = if rebuilding_makefiles != 0 {
                                if fref(file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                    f_mtime(file, 0)
                                } else {
                                    fref(file).last_mtime
                                }
                            } else if fref(file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime(file, 1)
                            } else {
                                fref(file).last_mtime
                            };
                            while !fref(file).renamed.is_null() {
                                file = fref(file).renamed;
                            }
                            if fref(file).updated() as ::core::ffi::c_int != 0
                                && mtime != fref(file).mtime_before_update
                            {
                                if rebuilding_makefiles == 0
                                    || just_print_flag == 0 && question_flag == 0
                                {
                                    status = us_success;
                                }
                                if rebuilding_makefiles != 0
                                    && fref(file).dontcare() as ::core::ffi::c_int != 0
                                {
                                    stop = 1;
                                }
                            }
                        }
                    }
                    all_updated &= fref(file).updated() as ::core::ffi::c_int;
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
            let g_changed = g.as_ref().map_or(0, |gd| gd.changed());
            if stop != 0 || all_updated != 0 {
                if rebuilding_makefiles == 0
                    && fref(file).update_status() as ::core::ffi::c_int
                        == us_success as ::core::ffi::c_int
                    && g_changed == 0
                    && run_silent == 0
                    && question_flag == 0
                {
                    message(
                        1,
                        strlen(fref(file).name) as size_t,
                        if fref(file).phony() as ::core::ffi::c_int != 0
                            || fref(file).cmds.is_null()
                        {
                            b"Nothing to be done for '%s'.\0" as *const u8
                                as *const ::core::ffi::c_char
                        } else {
                            b"'%s' is up to date.\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        fref(file).name,
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
            considered = considered.wrapping_add(1);
        }
    }
    free_dep_chain(goals_orig);
    if rebuilding_makefiles != 0 {
        touch_flag = t;
        question_flag = q;
        just_print_flag = n;
    }
    status as update_status
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn show_goal_error() {
    let mut goal: *mut goaldep;
    if (*goal_dep).flags() as ::core::ffi::c_int & (RM_INCLUDED | RM_DONTCARE) != RM_INCLUDED {
        return;
    }
    goal = goal_list;
    while !goal.is_null() {
        if (*goal_dep).file == (*goal).file {
            if (*goal).error != 0 {
                error(
                    &raw mut (*goal).floc,
                    (strlen((*(*goal).file).name) as size_t)
                        .wrapping_add(strlen(strerror((*goal).error)) as size_t),
                    b"%s: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*(*goal).file).name,
                    strerror((*goal).error),
                );
                (*goal).error = 0;
            }
            return;
        }
        goal = (*goal).next;
    }
}
unsafe extern "C" fn update_file(file: *mut file, depth: ::core::ffi::c_uint) -> update_status {
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
        if fr.considered == considered
            && !(fr.updated() as ::core::ffi::c_int != 0
                && fr.update_status() as ::core::ffi::c_int > us_none as ::core::ffi::c_int
                && fr.dontcare() == 0
                && fr.no_diag() as ::core::ffi::c_int != 0)
            && !(!file.double_colon.is_null()
                && file.command_state() as ::core::ffi::c_int == cs_finished as ::core::ffi::c_int
                && !fr.prev.is_null())
        {
            if 0x2 as ::core::ffi::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Pruning file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    file.name,
                );
                fflush(stdout);
            }
            return (if fr.command_state() as ::core::ffi::c_int == cs_finished as ::core::ffi::c_int
            {
                fr.update_status() as ::core::ffi::c_int
            } else {
                us_success as ::core::ffi::c_int
            }) as update_status;
        }
    }
    while !f.is_null() {
        let mut fr = f.as_mut().expect("update_file: null file chain");
        fr.considered = considered;
        let new: update_status = update_file_1(&raw mut *fr, depth);
        while !fr.renamed.is_null() {
            fr = fr.renamed.as_mut().expect("update_file: null renamed file");
        }
        if new as ::core::ffi::c_uint != 0 && keep_going_flag == 0 {
            return new;
        }
        if fr.command_state() as ::core::ffi::c_int == cs_running as ::core::ffi::c_int
            || fr.command_state() as ::core::ffi::c_int == cs_deps_running as ::core::ffi::c_int
        {
            return us_success;
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
pub unsafe fn complain(file: *mut file) {
    let mut d: *mut dep;
    d = (*file).deps;
    while !d.is_null() {
        if (*(*d).file).updated() as ::core::ffi::c_int != 0
            && (*(*d).file).update_status() as ::core::ffi::c_int > us_none as ::core::ffi::c_int
            && (*file).no_diag() as ::core::ffi::c_int != 0
        {
            complain((*d).file);
            break;
        } else {
            d = (*d).next;
        }
    }
    if d.is_null() {
        show_goal_error();
        if !(*file).parent.is_null() {
            let l: size_t = (strlen((*file).name) as size_t)
                .wrapping_add(strlen((*(*file).parent).name) as size_t)
                .wrapping_add(4);
            let m: *const ::core::ffi::c_char = b"%sNo rule to make target '%s', needed by '%s'%s\0"
                as *const u8
                as *const ::core::ffi::c_char;
            if keep_going_flag == 0 {
                fatal(
                    NILF,
                    l,
                    m,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                    (*(*file).parent).name,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            error(
                NILF,
                l,
                m,
                b"*** \0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
                (*(*file).parent).name,
                b".\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            let l_0: size_t = (strlen((*file).name) as size_t).wrapping_add(4);
            let m_0: *const ::core::ffi::c_char =
                b"%sNo rule to make target '%s'%s\0" as *const u8 as *const ::core::ffi::c_char;
            if keep_going_flag == 0 {
                fatal(
                    NILF,
                    l_0,
                    m_0,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            error(
                NILF,
                l_0,
                m_0,
                b"*** \0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
                b".\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*file).set_no_diag(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn update_file_1(
    file: *mut file,
    mut depth: ::core::ffi::c_uint,
) -> update_status {
    // Checked view of FILE; a null argument is a caller bug.
    let mut file = file.as_mut().expect("update_file_1: null file");
    let mut dep_status: update_status = us_success;
    let mut this_mtime: uintmax_t;
    let mut noexist: ::core::ffi::c_int;
    let mut must_make: ::core::ffi::c_int;
    let mut deps_changed: ::core::ffi::c_int;
    let mut du: *mut dep;
    let mut d: *mut dep;
    let mut ad: *mut dep;
    let mut amake: dep = dep {
        next: ::core::ptr::null_mut::<dep>(),
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        file: ::core::ptr::null_mut::<file>(),
        shuf: ::core::ptr::null_mut::<dep>(),
        stem: ::core::ptr::null::<::core::ffi::c_char>(),
        flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [0; 2],
        c2rust_padding: [0; 6],
    };
    let mut running: ::core::ffi::c_int = 0;
    if 0x2 as ::core::ffi::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            b"Considering target file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*file).name,
        );
        fflush(stdout);
    }
    if (*file).updated() != 0 {
        if (*file).update_status() as ::core::ffi::c_int > us_none as ::core::ffi::c_int {
            if 0x2 as ::core::ffi::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Recently tried and failed to update file '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
            if (*file).no_diag() as ::core::ffi::c_int != 0 && (*file).dontcare() == 0 {
                complain(file);
            }
            return (*file).update_status() as update_status;
        }
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"File '%s' was considered already.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        return us_success;
    }
    match (*file).command_state() as ::core::ffi::c_int {
        0 | 1 => {}
        2 => {
            if 0x2 as ::core::ffi::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Still updating file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
            return us_success;
        }
        3 => {
            if 0x2 as ::core::ffi::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Finished updating file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
            return (*file).update_status() as update_status;
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
        f_mtime(file, 1)
    } else {
        (*file).last_mtime
    };
    while !file.renamed.is_null() {
        file = file
            .renamed
            .as_mut()
            .expect("update_file_1: null renamed file");
    }
    noexist = (this_mtime == NONEXISTENT_MTIME as uintmax_t) as ::core::ffi::c_int;
    if noexist != 0 {
        if (*file).phony() != 0 {
            if 0x1 as ::core::ffi::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    b"Target '%s' is phony.\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                fflush(stdout);
            }
        } else if 0x1 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"File '%s' does not exist.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    } else if this_mtime >= ORDINARY_MTIME_MIN as uintmax_t
        && this_mtime
            <= ((!(0 as ::core::ffi::c_int as uintmax_t))
                .wrapping_sub(
                    if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                        0 as ::core::ffi::c_int as uintmax_t
                    } else {
                        !(0 as ::core::ffi::c_int as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8 as usize)
                                .wrapping_sub(1 as usize)
                    },
                )
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
                << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
            .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000 as ::core::ffi::c_int
                } else {
                    1
                }) as uintmax_t,
            )
            .wrapping_sub(1 as uintmax_t)
        && (*file).low_resolution_time() as ::core::ffi::c_int != 0
    {
        let ns: ::core::ffi::c_int = (this_mtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1) as uintmax_t)
            as ::core::ffi::c_int;
        if ns != 0 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen((*file).name) as size_t,
                b"*** warning: .LOW_RESOLUTION_TIME file '%s' has a high resolution time stamp\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
        }
        this_mtime = this_mtime.wrapping_add(
            ((if FILE_TIMESTAMP_HI_RES != 0 {
                1000000000 as ::core::ffi::c_int
            } else {
                1
            }) - 1
                - ns) as uintmax_t,
        );
    }
    ad = (*file).also_make;
    while !ad.is_null() && noexist == 0 {
        let mut adfile: *mut file = (*ad).file;
        let fmtime: uintmax_t = if (*adfile).last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(adfile, 1)
        } else {
            (*adfile).last_mtime
        };
        noexist = (fmtime == NONEXISTENT_MTIME as uintmax_t) as ::core::ffi::c_int;
        if noexist != 0 {
            while !(*adfile).renamed.is_null() {
                adfile = (*adfile).renamed;
            }
            if (*adfile).phony() != 0 {
                if 0x1 as ::core::ffi::c_int & db_level != 0 {
                    print_spaces(depth);
                    printf(
                        b"Grouped target peer '%s' of file '%s' is phony.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*adfile).name,
                        (*file).name,
                    );
                    fflush(stdout);
                }
            } else if 0x1 as ::core::ffi::c_int & db_level != 0 {
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
        try_implicit_rule(file, depth);
        (*file).set_tried_implicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if (*file).cmds.is_null()
        && (*file).is_target() == 0
        && !default_file.is_null()
        && !(*default_file).cmds.is_null()
    {
        if 0x8 as ::core::ffi::c_int & db_level != 0 {
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
        if second_expansion != 0 {
            expand_deps((*ad).file);
        }
        du = (*(*ad).file).deps;
        ad = (*ad).next;
        while !du.is_null() {
            let new: update_status;
            let mtime: uintmax_t;
            let mut maybe_make: ::core::ffi::c_int;
            let mut dontcare: ::core::ffi::c_int = 0;
            d = if !(*du).shuf.is_null() {
                (*du).shuf
            } else {
                du
            };
            if (*d).wait_here() as ::core::ffi::c_int != 0 && running != 0 {
                break;
            }
            while !(*(*d).file).renamed.is_null() {
                (*d).file = (*(*d).file).renamed;
            }
            mtime = if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                f_mtime((*d).file, 1)
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
            .updating()
                != 0
            {
                if warning::action(Type::CircularDep) == Action::Error {
                    fatal(
                        ::core::ptr::null_mut::<Floc>(),
                        (strlen((*file).name) as size_t)
                            .wrapping_add(strlen((*(*d).file).name) as size_t),
                        b"circular %s <- %s dependency detected\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*file).name,
                        (*(*d).file).name,
                    );
                }
                if warning::is_active(Type::CircularDep) {
                    error(
                        ::core::ptr::null_mut::<Floc>(),
                        (strlen((*file).name) as size_t)
                            .wrapping_add(strlen((*(*d).file).name) as size_t),
                        b"circular %s <- %s dependency dropped\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*file).name,
                        (*(*d).file).name,
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
                        (::core::mem::size_of::<*mut dep>() as size_t).wrapping_mul(
                            dropped_list_len.wrapping_add(DROPPED_LIST_INCR as size_t),
                        ),
                    ) as *mut *mut dep;
                }
                let fresh1 = dropped_list_len;
                dropped_list_len = dropped_list_len.wrapping_add(1);
                let fresh2 = &mut (*dropped_list.offset(fresh1 as isize));
                *fresh2 = d;
            } else {
                (*(*d).file).parent = file;
                maybe_make = must_make;
                if rebuilding_makefiles != 0 {
                    dontcare = (*(*d).file).dontcare() as ::core::ffi::c_int;
                    (*(*d).file).set_dontcare((*file).dontcare() as ::core::ffi::c_uint);
                }
                new = check_dep((*d).file, depth, this_mtime, &raw mut maybe_make);
                if new as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                    dep_status = new;
                }
                if rebuilding_makefiles != 0 {
                    (*(*d).file)
                        .set_dontcare(dontcare as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                if (*d).ignore_mtime() == 0 {
                    must_make = maybe_make;
                }
                while !(*(*d).file).renamed.is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f: *mut file = (*d).file;
                if !(*f).double_colon.is_null() {
                    f = (*f).double_colon;
                }
                loop {
                    running |= ((*f).command_state() as ::core::ffi::c_int
                        == cs_running as ::core::ffi::c_int
                        || (*f).command_state() as ::core::ffi::c_int
                            == cs_deps_running as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    f = (*f).prev;
                    if f.is_null() {
                        break;
                    }
                }
                if dep_status as ::core::ffi::c_uint != 0 && keep_going_flag == 0 {
                    break;
                }
                if running == 0 {
                    (*d).set_changed(
                        ((if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                            f_mtime((*d).file, 1)
                        } else {
                            (*(*d).file).last_mtime
                        }) != mtime
                            || mtime == NONEXISTENT_MTIME as uintmax_t)
                            as ::core::ffi::c_int as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                }
                lastd = du;
                du = (*du).next;
            }
        }
    }
    if must_make != 0 || always_make_flag != 0 {
        du = (*file).deps;
        while !du.is_null() {
            d = if !(*du).shuf.is_null() {
                (*du).shuf
            } else {
                du
            };
            if (*d).wait_here() as ::core::ffi::c_int != 0 && running != 0 {
                break;
            }
            if (*(*d).file).intermediate() != 0 {
                let new_0: update_status;
                let mut dontcare_0: ::core::ffi::c_int = 0;
                let mtime_0: uintmax_t = if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                    f_mtime((*d).file, 1)
                } else {
                    (*(*d).file).last_mtime
                };
                while !(*(*d).file).renamed.is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                (*(*d).file).parent = file;
                if rebuilding_makefiles != 0 {
                    dontcare_0 = (*(*d).file).dontcare() as ::core::ffi::c_int;
                    (*(*d).file).set_dontcare((*file).dontcare() as ::core::ffi::c_uint);
                }
                (*(*d).file).considered = 0;
                new_0 = update_file((*d).file, depth);
                if new_0 as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                    dep_status = new_0;
                }
                if rebuilding_makefiles != 0 {
                    (*(*d).file)
                        .set_dontcare(dontcare_0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                while !(*(*d).file).renamed.is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f_0: *mut file = (*d).file;
                if !(*f_0).double_colon.is_null() {
                    f_0 = (*f_0).double_colon;
                }
                loop {
                    running |= ((*f_0).command_state() as ::core::ffi::c_int
                        == cs_running as ::core::ffi::c_int
                        || (*f_0).command_state() as ::core::ffi::c_int
                            == cs_deps_running as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    f_0 = (*f_0).prev;
                    if f_0.is_null() {
                        break;
                    }
                }
                if dep_status as ::core::ffi::c_uint != 0 && keep_going_flag == 0 {
                    break;
                }
                if running == 0 {
                    (*d).set_changed(
                        ((*file).phony() as ::core::ffi::c_int != 0 && !(*file).cmds.is_null()
                            || (if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                                f_mtime((*d).file, 1)
                            } else {
                                (*(*d).file).last_mtime
                            }) != mtime_0) as ::core::ffi::c_int
                            as ::core::ffi::c_uint as ::core::ffi::c_uint,
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
    (*fresh4).set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    depth = depth.wrapping_sub(1);
    if running != 0 {
        set_command_state(file, cs_deps_running);
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"The prerequisites of '%s' are being made.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        return us_success;
    }
    if 0x2 as ::core::ffi::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            b"Finished prerequisites of target file '%s'.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*file).name,
        );
        fflush(stdout);
    }
    if dep_status as u64 != 0 {
        (*file).set_update_status(
            (if dep_status as ::core::ffi::c_uint
                == us_none as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                us_failed as ::core::ffi::c_int as ::core::ffi::c_uint
            } else {
                dep_status as ::core::ffi::c_uint
            }) as update_status as update_status,
        );
        notice_finished_file(file);
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Giving up on target file '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        if depth == 0 && keep_going_flag != 0 && just_print_flag == 0 && question_flag == 0 {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen((*file).name) as size_t,
                b"Target '%s' not remade because of errors.\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
        }
        return dep_status;
    }
    if (*file).command_state() as ::core::ffi::c_int == cs_deps_running as ::core::ffi::c_int {
        set_command_state(file, cs_not_started);
    }
    deps_changed = 0;
    d = (*file).deps;
    while !d.is_null() {
        let d_mtime: uintmax_t = if (*(*d).file).last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime((*d).file, 1)
        } else {
            (*(*d).file).last_mtime
        };
        while !(*(*d).file).renamed.is_null() {
            (*d).file = (*(*d).file).renamed;
        }
        if (*d).ignore_mtime() == 0 {
            if d_mtime == NONEXISTENT_MTIME as uintmax_t && (*(*d).file).intermediate() == 0 {
                must_make = 1;
            }
            deps_changed |= (*d).changed() as ::core::ffi::c_int;
        }
        (*d).set_changed(
            (*d).changed()
                | (noexist != 0 || d_mtime > this_mtime) as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
        );
        if noexist == 0 && (0x1 as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) & db_level != 0 {
            let mut fmt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            if (*d).ignore_mtime() != 0 {
                if 0x2 as ::core::ffi::c_int & db_level != 0 {
                    fmt = b"Prerequisite '%s' is order-only for target '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char;
                }
            } else if d_mtime == NONEXISTENT_MTIME as uintmax_t {
                if 0x1 as ::core::ffi::c_int & db_level != 0 {
                    if (*(*d).file).phony() != 0 {
                        fmt = b"Prerequisite '%s' of target '%s' is phony.\n\0" as *const u8
                            as *const ::core::ffi::c_char;
                    } else {
                        fmt = b"Prerequisite '%s' of target '%s' does not exist.\n\0" as *const u8
                            as *const ::core::ffi::c_char;
                    }
                }
            } else if (*d).changed() != 0 {
                if 0x1 as ::core::ffi::c_int & db_level != 0 {
                    fmt = b"Prerequisite '%s' is newer than target '%s'.\n\0" as *const u8
                        as *const ::core::ffi::c_char;
                }
            } else if 0x2 as ::core::ffi::c_int & db_level != 0 {
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
        if 0x1 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Target '%s' is double-colon and has no prerequisites.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    } else if noexist == 0
        && (*file).is_target() as ::core::ffi::c_int != 0
        && deps_changed == 0
        && (*file).cmds.is_null()
        && always_make_flag == 0
    {
        must_make = 0;
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"No recipe for '%s' and no prerequisites actually changed.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    } else if must_make == 0 && !(*file).cmds.is_null() && always_make_flag != 0 {
        must_make = 1;
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
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
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"No need to remake target '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            if !(*(*file).name as ::core::ffi::c_int == *(*file).hname as ::core::ffi::c_int
                && (*(*file).name as ::core::ffi::c_int == 0
                    || strcmp(
                        (*file).name.offset(1 as ::core::ffi::c_int as isize),
                        (*file).hname.offset(1 as ::core::ffi::c_int as isize),
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
        if (*file).notintermediate() == 0 && no_intermediates == 0 {
            (*file).set_secondary(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        notice_finished_file(file);
        loop {
            file.name = file.hname;
            match file.prev.as_mut() {
                Some(prev) => file = prev,
                None => break,
            }
        }
        return us_success;
    }
    if 0x1 as ::core::ffi::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            b"Must remake target '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*file).name,
        );
        fflush(stdout);
    }
    if !(*(*file).name as ::core::ffi::c_int == *(*file).hname as ::core::ffi::c_int
        && (*(*file).name as ::core::ffi::c_int == 0
            || strcmp(
                (*file).name.offset(1 as ::core::ffi::c_int as isize),
                (*file).hname.offset(1 as ::core::ffi::c_int as isize),
            ) == 0))
    {
        if 0x1 as ::core::ffi::c_int & db_level != 0 {
            printf(
                b"  Ignoring VPATH name '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).hname,
            );
            fflush(stdout);
        }
        (*file).set_ignore_vpath(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    remake_file(file);
    if (*file).command_state() as ::core::ffi::c_int != cs_finished as ::core::ffi::c_int {
        if 0x2 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Recipe of '%s' is being run.\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        return us_success;
    }
    match (*file).update_status() as ::core::ffi::c_int {
        3 => {
            if 0x1 as ::core::ffi::c_int & db_level != 0 {
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
            if 0x1 as ::core::ffi::c_int & db_level != 0 {
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
            if 0x1 as ::core::ffi::c_int & db_level != 0 {
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
    (*file).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*file).update_status() as update_status
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn notice_finished_file(file: *mut file) {
    let mut d: *mut dep;
    let ran: ::core::ffi::c_int = ((*file).command_state() as ::core::ffi::c_int
        == cs_running as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut touched: ::core::ffi::c_int = 0;
    (*file).set_command_state(cs_finished as cmd_state);
    (*file).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if touch_flag != 0
        && (*file).update_status() as ::core::ffi::c_int == us_success as ::core::ffi::c_int
    {
        // Touch the file unless every command line is recursive (flagged
        // COMMANDS_RECURSE); a single non-recursive line means we touch.
        let mut should_touch = true;
        if !(*file).cmds.is_null() && (*(*file).cmds).any_recurse() as ::core::ffi::c_int != 0 {
            should_touch = false;
            let n: ::core::ffi::c_uint = (*(*file).cmds).ncommand_lines as ::core::ffi::c_uint;
            let mut i: ::core::ffi::c_uint = 0;
            while i < n {
                if (*(*(*file).cmds).lines_flags.offset(i as isize) as ::core::ffi::c_int & 1) == 0
                {
                    should_touch = true;
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        if should_touch {
            if (*file).phony() != 0 {
                (*file).set_update_status(us_success as update_status);
            } else if !(*file).cmds.is_null() {
                (*file).set_update_status(touch_file(file) as update_status as update_status);
                commands_started = commands_started.wrapping_add(1);
                touched = 1;
            }
        }
    }
    if (*file).mtime_before_update == UNKNOWN_MTIME as uintmax_t {
        (*file).mtime_before_update = (*file).last_mtime;
    }
    if ran != 0 && (*file).phony() == 0 || touched != 0 {
        let mut i_0: ::core::ffi::c_int = 0;
        if (question_flag != 0 || just_print_flag != 0 || touch_flag != 0)
            && !(*file).cmds.is_null()
        {
            i_0 = (*(*file).cmds).ncommand_lines as ::core::ffi::c_int;
            while i_0 > 0 {
                if !(*(*(*file).cmds).lines_flags.offset((i_0 - 1) as isize) as ::core::ffi::c_int
                    & 1
                    != 0)
                {
                    break;
                }
                i_0 -= 1;
            }
        } else if (*file).is_target() as ::core::ffi::c_int != 0 && (*file).cmds.is_null() {
            i_0 = 1;
        }
        (*file).last_mtime = if i_0 == 0 {
            UNKNOWN_MTIME as uintmax_t
        } else {
            (!(0 as ::core::ffi::c_int as uintmax_t)).wrapping_sub(
                if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                    0 as ::core::ffi::c_int as uintmax_t
                } else {
                    !(0 as ::core::ffi::c_int as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1 as usize)
                },
            )
        };
    }
    if !(*file).double_colon.is_null() {
        let mut f: *mut file;
        let mut max_mtime: uintmax_t = (*file).last_mtime;
        f = (*file).double_colon;
        while !f.is_null() && (*f).updated() as ::core::ffi::c_int != 0 {
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
    if ran != 0 && (*file).update_status() as ::core::ffi::c_int != us_none as ::core::ffi::c_int {
        d = (*file).also_make;
        while !d.is_null() {
            (*(*d).file).set_command_state(cs_finished as cmd_state);
            (*(*d).file).set_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*(*d).file).set_update_status((*file).update_status() as update_status);
            if ran != 0 && (*(*d).file).phony() == 0 {
                f_mtime((*d).file, 0);
                if just_print_flag != 0 {
                    (*(*d).file).last_mtime = (!(0 as ::core::ffi::c_int as uintmax_t))
                        .wrapping_sub(
                            if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                                0 as ::core::ffi::c_int as uintmax_t
                            } else {
                                !(0 as ::core::ffi::c_int as uintmax_t)
                                    << (::core::mem::size_of::<uintmax_t>() as usize)
                                        .wrapping_mul(CHAR_BIT as usize)
                                        .wrapping_sub(1 as usize)
                            },
                        );
                }
            }
            d = (*d).next;
        }
        if (*file).tried_implicit() as ::core::ffi::c_int != 0 && !(*file).also_make.is_null() {
            check_also_make(file);
        }
    } else if (*file).update_status() as ::core::ffi::c_int == us_none as ::core::ffi::c_int {
        (*file).set_update_status(us_success as update_status);
    }
}
unsafe extern "C" fn check_dep(
    mut file: *mut file,
    depth: ::core::ffi::c_uint,
    this_mtime: uintmax_t,
    must_make_ptr: *mut ::core::ffi::c_int,
) -> update_status {
    let ofile: *mut file;
    let mut d: *mut dep;
    let mut dep_status: update_status = us_success;
    double_colon_file_mut(file).set_updating(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    ofile = file;
    if (*file).phony() as ::core::ffi::c_int != 0 || (*file).intermediate() == 0 {
        let mtime: uintmax_t;
        dep_status = update_file(file, depth);
        while !(*file).renamed.is_null() {
            file = (*file).renamed;
        }
        mtime = if (*file).last_mtime == UNKNOWN_MTIME as uintmax_t {
            f_mtime(file, 1)
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
            try_implicit_rule(file, depth);
            (*file).set_tried_implicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (*file).cmds.is_null()
            && (*file).is_target() == 0
            && !default_file.is_null()
            && !(*default_file).cmds.is_null()
        {
            if 0x8 as ::core::ffi::c_int & db_level != 0 {
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
            f_mtime(file, 1)
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
            let mut deps_running: ::core::ffi::c_int = 0;
            if (*file).command_state() as ::core::ffi::c_int != cs_running as ::core::ffi::c_int {
                if (*file).command_state() as ::core::ffi::c_int
                    == cs_deps_running as ::core::ffi::c_int
                {
                    (*file).considered = 0;
                }
                set_command_state(file, cs_not_started);
            }
            ld = ::core::ptr::null_mut::<dep>();
            if second_expansion != 0 {
                expand_deps(file);
            }
            d = (*file).deps;
            while let Some(dep_ref) = d.as_mut() {
                let new: update_status;
                let mut maybe_make: ::core::ffi::c_int;
                let dep_file = dep_ref.file;
                if double_colon_file_mut(dep_file).updating() != 0 {
                    let dep_name = fref(dep_file).name;
                    error(
                        ::core::ptr::null_mut::<Floc>(),
                        (strlen((*file).name) as size_t).wrapping_add(strlen(dep_name) as size_t),
                        b"circular %s <- %s dependency dropped\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*file).name,
                        dep_name,
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
                        dep_file,
                        depth.wrapping_add(1),
                        this_mtime,
                        &raw mut maybe_make,
                    );
                    if new as ::core::ffi::c_uint > dep_status as ::core::ffi::c_uint {
                        dep_status = new;
                    }
                    if (*d).ignore_mtime() == 0 {
                        *must_make_ptr = maybe_make;
                    }
                    loop {
                        let renamed = fref(dep_ref.file).renamed;
                        if renamed.is_null() {
                            break;
                        }
                        dep_ref.file = renamed;
                    }
                    if dep_status as ::core::ffi::c_uint != 0 && keep_going_flag == 0 {
                        break;
                    }
                    let dep_file_ref = fref(dep_ref.file);
                    if dep_file_ref.command_state() as ::core::ffi::c_int
                        == cs_running as ::core::ffi::c_int
                        || dep_file_ref.command_state() as ::core::ffi::c_int
                            == cs_deps_running as ::core::ffi::c_int
                    {
                        deps_running = 1;
                    }
                    ld = d;
                    d = dep_ref.next;
                }
            }
            if deps_running != 0 {
                set_command_state(file, cs_deps_running);
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
pub unsafe fn touch_file(file: *mut file) -> update_status {
    if run_silent == 0 {
        message(
            0,
            strlen((*file).name) as size_t,
            b"touch %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*file).name,
        );
    }
    if just_print_flag != 0 {
        return us_success;
    }
    if ar_name((*file).name) != 0 {
        return (if ar_touch((*file).name) != 0 {
            us_failed as ::core::ffi::c_int
        } else {
            us_success as ::core::ffi::c_int
        }) as update_status;
    } else {
        let mut fd: ::core::ffi::c_int;
        loop {
            fd = open(
                (*file).name,
                0o2 as ::core::ffi::c_int | 0o100 as ::core::ffi::c_int,
                0o666 as ::core::ffi::c_int,
            );
            if !(fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd < 0 {
            perror_with_name(
                b"touch: open: \0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
            return us_failed;
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
            let mut e: ::core::ffi::c_int;
            loop {
                e = fstat(fd, &raw mut statbuf);
                if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if e < 0 {
                perror_with_name(
                    b"touch: fstat: \0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            loop {
                e = read(fd, &raw mut buf as *mut ::core::ffi::c_void, 1) as ::core::ffi::c_int;
                if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if e < 0 {
                perror_with_name(
                    b"touch: read: \0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            let mut o: off_t;
            loop {
                o = lseek(fd, 0 as __off_t, 0) as off_t;
                if !(o == -(1 as ::core::ffi::c_int) as off_t && *__errno_location() == EINTR) {
                    break;
                }
            }
            if o < 0 {
                perror_with_name(
                    b"touch: lseek: \0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            loop {
                e = write(fd, &raw mut buf as *const ::core::ffi::c_void, 1) as ::core::ffi::c_int;
                if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if e < 0 {
                perror_with_name(
                    b"touch: write: \0" as *const u8 as *const ::core::ffi::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            if statbuf.st_size == 0 as __off_t {
                close(fd);
                loop {
                    fd = open(
                        (*file).name,
                        0o2 as ::core::ffi::c_int | 0o1000 as ::core::ffi::c_int,
                        0o666 as ::core::ffi::c_int,
                    );
                    if !(fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                        break;
                    }
                }
                if fd < 0 {
                    perror_with_name(
                        b"touch: open: \0" as *const u8 as *const ::core::ffi::c_char,
                        (*file).name,
                    );
                    return us_failed;
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
pub unsafe fn remake_file(file: *mut file) {
    if (*file).cmds.is_null() {
        if (*file).phony() != 0 {
            (*file).set_update_status(us_success as update_status);
        } else if (*file).is_target() != 0 {
            (*file).set_update_status(us_success as update_status);
        } else {
            if rebuilding_makefiles == 0 || (*file).dontcare() == 0 {
                complain(file);
            }
            (*file).set_update_status(us_failed as update_status);
        }
    } else {
        chop_commands((*file).cmds);
        if touch_flag == 0 || (*(*file).cmds).any_recurse() as ::core::ffi::c_int != 0 {
            execute_file_commands(file);
            return;
        }
        (*file).set_update_status(us_success as update_status);
    }
    notice_finished_file(file);
}
/// Return the mtime of file F, computing it if necessary. Returns
/// NONEXISTENT_MTIME if the file does not exist.
///
/// # Safety
/// `file` must point to a valid `File`; must run single-threaded with the
/// global file table.
pub unsafe fn f_mtime(file: *mut file, search: ::core::ffi::c_int) -> uintmax_t {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut mtime: uintmax_t;
    let propagate_timestamp: ::core::ffi::c_uint;
    // Checked view of FILE; a null argument is a caller bug.
    let mut file = file.as_mut().expect("f_mtime: null file");
    if ar_name(file.name) != 0 {
        let memmtime: uintmax_t;
        let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut arfile: *mut file;
        let member_date: time_t;
        ar_parse_name((*file).name, &raw mut arname, &raw mut memname);
        memmtime = name_mtime(memname);
        arfile = lookup_file(arname);
        if arfile.is_null() {
            arfile = enter_file(strcache_add(arname));
        }
        mtime = f_mtime(arfile, search);
        while !(*arfile).renamed.is_null() {
            arfile = (*arfile).renamed;
        }
        if search != 0 && strcmp((*arfile).hname, arname) != 0 {
            let name: *mut ::core::ffi::c_char;
            let arlen: size_t;
            let memlen: size_t;
            arlen = strlen((*arfile).hname) as size_t;
            memlen = strlen(memname) as size_t;
            alloca_allocations.push(::std::vec::from_elem(
                0,
                arlen.wrapping_add(1).wrapping_add(memlen).wrapping_add(2) as usize,
            ));
            name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                name as *mut ::core::ffi::c_void,
                (*arfile).hname as *const ::core::ffi::c_void,
                arlen as size_t,
            );
            *name.offset(arlen as isize) = '(' as i32 as ::core::ffi::c_char;
            memcpy(
                name.offset(arlen as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                memname as *const ::core::ffi::c_void,
                memlen as size_t,
            );
            *name.offset(arlen.wrapping_add(1 as size_t).wrapping_add(memlen) as isize) =
                ')' as i32 as ::core::ffi::c_char;
            *name.offset(arlen.wrapping_add(1).wrapping_add(memlen).wrapping_add(1) as isize) = 0;
            if (*arfile).name == (*arfile).hname {
                rename_file(file, strcache_add(name));
            } else {
                rehash_file(file, strcache_add(name));
            }
            while !file.renamed.is_null() {
                file = file.renamed.as_mut().expect("f_mtime: null renamed file");
            }
        }
        free(arname as *mut ::core::ffi::c_void);
        (*file).set_low_resolution_time(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if mtime == NONEXISTENT_MTIME as uintmax_t {
            return NONEXISTENT_MTIME as uintmax_t;
        }
        member_date = ar_member_date((*file).hname);
        if member_date == -(1 as ::core::ffi::c_int) as time_t
            || memmtime != NONEXISTENT_MTIME as uintmax_t
                && (memmtime.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
                    as time_t
                    > member_date
        {
            mtime = NONEXISTENT_MTIME as uintmax_t;
        } else {
            mtime = file_timestamp_cons((*file).hname, member_date, 0);
        }
    } else {
        mtime = name_mtime((*file).name);
        if mtime == NONEXISTENT_MTIME as uintmax_t && search != 0 && (*file).ignore_vpath() == 0 {
            let mut name_0: *const ::core::ffi::c_char = vpath_search(
                (*file).name,
                &raw mut mtime,
                ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                ::core::ptr::null_mut::<::core::ffi::c_uint>(),
            );
            if !name_0.is_null()
                || *(*file).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '-' as i32
                    && *(*file).name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 'l' as i32
                    && {
                        name_0 = library_search((*file).name, &raw mut mtime);
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
                if gpath_search(name_0, name_len) != 0 {
                    rename_file(file, name_0);
                    while !file.renamed.is_null() {
                        file = file.renamed.as_mut().expect("f_mtime: null renamed file");
                    }
                    return if (*file).last_mtime == UNKNOWN_MTIME as uintmax_t {
                        f_mtime(file, 1)
                    } else {
                        (*file).last_mtime
                    };
                }
                rehash_file(file, name_0);
                while !file.renamed.is_null() {
                    file = file.renamed.as_mut().expect("f_mtime: null renamed file");
                }
                if mtime != OLD_MTIME as uintmax_t
                    && mtime
                        != (!(0 as ::core::ffi::c_int as uintmax_t)).wrapping_sub(
                            if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                                0 as ::core::ffi::c_int as uintmax_t
                            } else {
                                !(0 as ::core::ffi::c_int as uintmax_t)
                                    << (::core::mem::size_of::<uintmax_t>() as usize)
                                        .wrapping_mul(CHAR_BIT as usize)
                                        .wrapping_sub(1 as usize)
                            },
                        )
                {
                    mtime = name_mtime(name_0);
                }
            }
        }
    }
    if clock_skew_detected == 0
        && mtime != NONEXISTENT_MTIME as uintmax_t
        && mtime
            != (!(0 as ::core::ffi::c_int as uintmax_t)).wrapping_sub(
                if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                    0 as ::core::ffi::c_int as uintmax_t
                } else {
                    !(0 as ::core::ffi::c_int as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(1 as usize)
                },
            )
        && (*file).updated() == 0
    {
        static mut adjusted_now: uintmax_t = 0;
        let adjusted_mtime: uintmax_t = mtime;
        if adjusted_now < adjusted_mtime {
            let mut resolution: ::core::ffi::c_int = 0;
            let now: uintmax_t = file_timestamp_now(&raw mut resolution);
            adjusted_now = now.wrapping_add((resolution - 1) as uintmax_t);
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
                            as uintmax_t) as ::core::ffi::c_int
                        - (now.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                            & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1)
                                as uintmax_t) as ::core::ffi::c_int)
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
                    ::core::ptr::null_mut::<Floc>(),
                    (strlen((*file).name) as size_t).wrapping_add(strlen(
                        &raw mut from_now_string as *mut ::core::ffi::c_char,
                    ) as size_t),
                    b"warning: file '%s' has modification time %s s in the future\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                    &raw mut from_now_string as *mut ::core::ffi::c_char,
                );
                clock_skew_detected = 1;
            }
        }
    }
    if !file.double_colon.is_null() {
        file = file
            .double_colon
            .as_mut()
            .expect("f_mtime: null double_colon");
    }
    propagate_timestamp = (*file).updated();
    loop {
        if mtime != NONEXISTENT_MTIME as uintmax_t
            && (*file).command_state() as ::core::ffi::c_int == cs_not_started as ::core::ffi::c_int
            && (*file).tried_implicit() == 0
            && (*file).intermediate() as ::core::ffi::c_int != 0
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
pub unsafe fn name_mtime(name: *const ::core::ffi::c_char) -> uintmax_t {
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
    let mut e: ::core::ffi::c_int;
    loop {
        e = stat(name, &raw mut st);
        if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if e == 0 {
        mtime = file_timestamp_cons(
            name,
            st.st_mtim.tv_sec as time_t,
            st.st_mtim.tv_nsec as ::core::ffi::c_long,
        );
    } else if *__errno_location() == ENOENT || *__errno_location() == ENOTDIR {
        mtime = NONEXISTENT_MTIME as uintmax_t;
    } else {
        perror_with_name(b"stat: \0" as *const u8 as *const ::core::ffi::c_char, name);
        return NONEXISTENT_MTIME as uintmax_t;
    }
    if check_symlink_flag != 0 && strlen(name) <= GET_PATH_MAX as size_t {
        let mut lpath: [::core::ffi::c_char; 4097] = [0; 4097];
        strcpy(&raw mut lpath as *mut ::core::ffi::c_char, name);
        loop {
            let ltime: uintmax_t;
            let mut lbuf: [::core::ffi::c_char; 4097] = [0; 4097];
            let mut llen: ::core::ffi::c_long;
            let p: *mut ::core::ffi::c_char;
            loop {
                e = lstat(&raw mut lpath as *mut ::core::ffi::c_char, &raw mut st);
                if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if e != 0 {
                if *__errno_location() != ENOENT && *__errno_location() != ENOTDIR {
                    perror_with_name(
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
                    &raw mut lpath as *mut ::core::ffi::c_char,
                    st.st_mtim.tv_sec as time_t,
                    st.st_mtim.tv_nsec as ::core::ffi::c_long,
                );
                if ltime > mtime {
                    mtime = ltime;
                }
                loop {
                    llen = readlink(
                        &raw mut lpath as *mut ::core::ffi::c_char,
                        &raw mut lbuf as *mut ::core::ffi::c_char,
                        (4096 as ::core::ffi::c_int - 1) as size_t,
                    ) as ::core::ffi::c_long;
                    if !(llen == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                        && *__errno_location() == EINTR)
                    {
                        break;
                    }
                }
                if llen < 0 {
                    perror_with_name(
                        b"readlink: \0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut lpath as *mut ::core::ffi::c_char,
                    );
                    break;
                } else {
                    lbuf[llen as usize] = 0;
                    if lbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '/' as i32
                        || {
                            p = strrchr(&raw mut lpath as *mut ::core::ffi::c_char, '/' as i32);
                            p.is_null()
                        }
                    {
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
                            p.offset(1 as ::core::ffi::c_int as isize),
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
        b".LIBPATTERNS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
    );
    lib = lib.offset(2 as ::core::ffi::c_int as isize);
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
                ::core::ptr::null_mut::<Floc>(),
                strlen(p) as size_t,
                b".LIBPATTERNS element '%s' is not a pattern\0" as *const u8
                    as *const ::core::ffi::c_char,
                p,
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
                p3.offset(1 as ::core::ffi::c_int as isize),
                len.wrapping_sub(p3.offset_from(p) as ::core::ffi::c_long as size_t),
            );
            *p.offset(len as isize) = c;
            libbuf = variable_buffer;
            mtime = name_mtime(libbuf);
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
                        dp = dp.offset(1 as ::core::ffi::c_int as isize);
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
                    (!(0 as ::core::ffi::c_int as ::core::ffi::c_uint)).wrapping_sub(std_dirs);
                dp = &raw const dirs as *const *const ::core::ffi::c_char;
                while !(*dp).is_null() {
                    sprintf(
                        buf,
                        b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
                        *dp,
                        libbuf,
                    );
                    mtime = name_mtime(buf);
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
                    dp = dp.offset(1 as ::core::ffi::c_int as isize);
                }
            }
        }
    }
    free(libpatterns as *mut ::core::ffi::c_void);
    file
}
pub const LIBDIR: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"/usr/local/lib\0") };
pub const __CHAR_BIT__: ::core::ffi::c_int = 8;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const FILE_TIMESTAMP_HI_RES: ::core::ffi::c_int = 1;
