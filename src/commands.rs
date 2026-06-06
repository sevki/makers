pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __pid_t, __sig_atomic_t, __syscall_slong_t, __time_t, __uid_t, pid_t, sig_atomic_t, size_t,
    time_t, uintmax_t,
};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
use crate::misc::{make_pid, xmalloc, xrealloc, xstrdup, xstrndup};
use crate::stdio::FILE;
use crate::strcache::{strcache_add, strcache_add_len};
use c2rust_bitfields;
use libc::{__errno_location, exit, printf, puts, strchr, strcmp, strstr, unlink};
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn temp_stdin_unlink();
    fn pfatal_with_name(_: *const ::core::ffi::c_char) -> !;
    fn perror_with_name(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    fn ar_name(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ar_member_date(_: *const ::core::ffi::c_char) -> time_t;
    fn unload_file(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    static mut always_make_flag: ::core::ffi::c_int;
    static mut one_shell: ::core::ffi::c_int;
    static mut cmd_prefix: ::core::ffi::c_char;
    fn hash_init(
        ht: *mut hash_table,
        size: ::core::ffi::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut *mut ::core::ffi::c_void;
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const ::core::ffi::c_void,
        slot: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn hash_free(ht: *mut hash_table, free_items: ::core::ffi::c_int);
    fn jhash_string(key: *const ::core::ffi::c_uchar) -> ::core::ffi::c_uint;
    static mut hash_deleted_item: *const ::core::ffi::c_void;
    static mut default_file: *mut file;
    fn enter_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn remove_intermediates(sig: ::core::ffi::c_int);
    fn set_command_state(file: *mut file, state: cmd_state);
    fn notice_finished_file(file: *mut file);
    fn file_timestamp_cons(
        _: *const ::core::ffi::c_char,
        _: time_t,
        _: ::core::ffi::c_long,
    ) -> uintmax_t;
    static mut children: *mut child;
    fn new_job(file: *mut file);
    fn reap_children(block: ::core::ffi::c_int, err: ::core::ffi::c_int);
    static mut job_slots_used: ::core::ffi::c_uint;
    fn jobserver_clear();
    fn osync_clear();
    fn initialize_file_variables(file: *mut file, reading: ::core::ffi::c_int);
    fn define_variable_in_set(
        name: *const ::core::ffi::c_char,
        length: size_t,
        value: *const ::core::ffi::c_char,
        origin: variable_origin,
        recursive: ::core::ffi::c_int,
        set: *mut variable_set,
        flocp: *const Floc,
    ) -> *mut variable;
}
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type file = File;
pub type cmd_state = ::core::ffi::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = ::core::ffi::c_uint;
pub type update_status_0 = u32;
pub const us_failed: update_status_0 = 3;
pub const us_question: update_status_0 = 2;
pub const us_none: update_status_0 = 1;
pub const us_success: update_status_0 = 0;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
pub type dep = Dep;
pub type commands = Commands;
use crate::floc::Floc;

pub const o_invalid: variable_origin = 7;
pub const o_automatic: variable_origin = 6;
pub const o_override: variable_origin = 5;
pub const o_command: variable_origin = 4;
pub const o_env_override: variable_origin = 3;
pub const o_file: variable_origin = 2;
pub const o_env: variable_origin = 1;
pub const o_default: variable_origin = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
    pub fileinfo: Floc,
    pub length: ::core::ffi::c_uint,
    #[bitfield(name = "recursive", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "::core::ffi::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export:
        [u8; 4],
}
pub type variable_export = ::core::ffi::c_uint;
pub const v_ifset: variable_export = 3;
pub const v_noexport: variable_export = 2;
pub const v_export: variable_export = 1;
pub const v_default: variable_export = 0;
pub type variable_origin = ::core::ffi::c_uint;
pub type variable_flavor = ::core::ffi::c_uint;
pub const f_append_value: variable_flavor = 6;
pub const f_shell: variable_flavor = 5;
pub const f_append: variable_flavor = 4;
pub const f_expand: variable_flavor = 3;
pub const f_recursive: variable_flavor = 2;
pub const f_simple: variable_flavor = 1;
pub const f_bogus: variable_flavor = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct child {
    pub cmd_name: *mut ::core::ffi::c_char,
    pub environment: *mut *mut ::core::ffi::c_char,
    pub output: output,
    pub next: *mut child,
    pub file: *mut file,
    pub sh_batch_file: *mut ::core::ffi::c_char,
    pub command_lines: *mut *mut ::core::ffi::c_char,
    pub command_ptr: *mut ::core::ffi::c_char,
    pub command_line: ::core::ffi::c_uint,
    pub pid: pid_t,
    #[bitfield(name = "remote", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "noerror", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "good_stdin", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "deleted", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "recursive", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "jobslot", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "dontcare", ty = "::core::ffi::c_uint", bits = "6..=6")]
    pub remote_noerror_good_stdin_deleted_recursive_jobslot_dontcare: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub use crate::output::output;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2;
pub const SIGTERM: ::core::ffi::c_int = 15;
pub const SIGHUP: ::core::ffi::c_int = 1;
pub const SIGQUIT: ::core::ffi::c_int = 3;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4;
pub const USHRT_MAX: ::core::ffi::c_int = __SHRT_MAX__ * 2 + 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const MAKE_TROUBLE: ::core::ffi::c_int = 1;
pub const COMMANDS_RECURSE: ::core::ffi::c_int = 1;
pub const COMMANDS_SILENT: ::core::ffi::c_int = 2;
pub const COMMANDS_NOERROR: ::core::ffi::c_int = 4;
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
pub const OLD_MTIME: ::core::ffi::c_int = 2;
pub const ORDINARY_MTIME_MIN: ::core::ffi::c_int = OLD_MTIME + 1;
pub const FILE_LIST_SEPARATOR: ::core::ffi::c_int = ' ' as i32;
#[no_mangle]
pub unsafe extern "C" fn dep_hash_1(key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let d: *const dep = key as *const dep;
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar = (if !(*d).name.is_null() {
        (*d).name
    } else {
        (*(*d).file).name
    }) as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    _result_
}
#[no_mangle]
pub unsafe extern "C" fn dep_hash_2(key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let d: *const dep = key as *const dep;
    let mut _result_: ::core::ffi::c_ulong = 0;
    if !(*d).name.is_null() {
    } else {
    };
    _result_
}
unsafe extern "C" fn dep_hash_cmp(
    x: *const ::core::ffi::c_void,
    y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let dx: *const dep = x as *const dep;
    let dy: *const dep = y as *const dep;
    strcmp(
        if !(*dx).name.is_null() {
            (*dx).name
        } else {
            (*(*dx).file).name
        },
        if !(*dy).name.is_null() {
            (*dy).name
        } else {
            (*(*dy).file).name
        },
    )
}
#[no_mangle]
pub unsafe extern "C" fn set_file_variables(file: *mut file, mut stem: *const ::core::ffi::c_char) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut d: *mut dep;
    let at: *const ::core::ffi::c_char;
    let percent: *const ::core::ffi::c_char;
    let star: *const ::core::ffi::c_char;
    let mut less: *const ::core::ffi::c_char;
    if ar_name((*file).name) != 0 {
        let len: size_t;
        let cp: *const ::core::ffi::c_char;
        let mut p: *mut ::core::ffi::c_char;
        cp = strchr((*file).name, '(' as i32);
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (cp.offset_from((*file).name) as ::core::ffi::c_long + 1) as ::core::ffi::c_ulong
                as usize,
        ));
        p = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            p as *mut ::core::ffi::c_void,
            (*file).name as *const ::core::ffi::c_void,
            cp.offset_from((*file).name) as ::core::ffi::c_long as size_t,
        );
        *p.offset(cp.offset_from((*file).name) as ::core::ffi::c_long as isize) = 0;
        at = p;
        len = strlen(cp.offset(1 as ::core::ffi::c_int as isize)) as size_t;
        alloca_allocations.push(::std::vec::from_elem(0, len as usize));
        p = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            p as *mut ::core::ffi::c_void,
            cp.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (len as size_t).wrapping_sub(1),
        );
        *p.offset(len.wrapping_sub(1) as isize) = 0;
        percent = p;
    } else {
        at = (*file).name;
        percent = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if stem.is_null() {
        let name: *const ::core::ffi::c_char;
        let len_0: size_t;
        if ar_name((*file).name) != 0 {
            name = strchr((*file).name, '(' as i32).offset(1 as ::core::ffi::c_int as isize);
            len_0 = strlen(name).wrapping_sub(1) as size_t;
        } else {
            name = (*file).name;
            len_0 = strlen(name) as size_t;
        }
        d = (*enter_file(strcache_add(
            b".SUFFIXES\0" as *const u8 as *const ::core::ffi::c_char,
        )))
        .deps;
        while !d.is_null() {
            let dn: *const ::core::ffi::c_char = if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            let slen: size_t = strlen(dn) as size_t;
            if len_0 > slen
                && memcmp(
                    dn as *const ::core::ffi::c_void,
                    name.offset(len_0.wrapping_sub(slen) as isize) as *const ::core::ffi::c_void,
                    slen as size_t,
                ) == 0
            {
                stem = strcache_add_len(name, len_0.wrapping_sub(slen));
                (*file).stem = stem;
                break;
            } else {
                d = (*d).next;
            }
        }
        if d.is_null() {
            stem = b"\0" as *const u8 as *const ::core::ffi::c_char;
            (*file).stem = stem;
        }
    }
    star = stem;
    less = b"\0" as *const u8 as *const ::core::ffi::c_char;
    d = (*file).deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0
            && (*d).ignore_automatic_vars() == 0
            && (*d).need_2nd_expansion() == 0
        {
            less = if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            break;
        } else {
            d = (*d).next;
        }
    }
    if !(*file).cmds.is_null() && (*file).cmds == (*default_file).cmds {
        less = at;
    }
    define_variable_in_set(
        b"<\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        less,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
    define_variable_in_set(
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        star,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
    define_variable_in_set(
        b"@\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        at,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
    define_variable_in_set(
        b"%\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        percent,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
    static mut plus_value: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut bar_value: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut qmark_value: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    static mut plus_max: size_t = 0;
    static mut bar_max: size_t = 0;
    static mut qmark_max: size_t = 0;
    let mut qmark_len: size_t;
    let mut plus_len: size_t;
    let mut bar_len: size_t;
    let mut cp_0: *mut ::core::ffi::c_char;
    let caret_value: *mut ::core::ffi::c_char;
    let mut qp: *mut ::core::ffi::c_char;
    let mut bp: *mut ::core::ffi::c_char;
    let mut len_1: size_t;
    let mut dep_hash: hash_table = hash_table {
        ht_vec: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        ht_hash_1: None,
        ht_hash_2: None,
        ht_compare: None,
        ht_size: 0,
        ht_capacity: 0,
        ht_fill: 0,
        ht_empty_slots: 0,
        ht_collisions: 0,
        ht_lookups: 0,
        ht_rehashes: 0,
        ht_in_map: [0; 1],
        c2rust_padding: [0; 3],
    };
    let mut slot: *mut *mut ::core::ffi::c_void;
    plus_len = 0;
    bar_len = 0;
    d = (*file).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 && (*d).ignore_automatic_vars() == 0 {
            if (*d).ignore_mtime() != 0 {
                bar_len = bar_len.wrapping_add(
                    strlen(if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    })
                    .wrapping_add(1) as size_t,
                );
            } else {
                plus_len = plus_len.wrapping_add(
                    strlen(if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    })
                    .wrapping_add(1) as size_t,
                );
            }
        }
        d = (*d).next;
    }
    if bar_len == 0 {
        bar_len = bar_len.wrapping_add(1);
    }
    if plus_len == 0 {
        plus_len = plus_len.wrapping_add(1);
    }
    if plus_len > plus_max {
        plus_max = plus_len;
        plus_value =
            xrealloc(plus_value as *mut ::core::ffi::c_void, plus_max) as *mut ::core::ffi::c_char;
    }
    cp_0 = plus_value;
    qmark_len = plus_len.wrapping_add(1);
    d = (*file).deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0
            && (*d).need_2nd_expansion() == 0
            && (*d).ignore_automatic_vars() == 0
        {
            let mut c: *const ::core::ffi::c_char = if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            if ar_name(c) != 0 {
                c = strchr(c, '(' as i32).offset(1 as ::core::ffi::c_int as isize);
                len_1 = strlen(c).wrapping_sub(1) as size_t;
            } else {
                len_1 = strlen(c) as size_t;
            }
            cp_0 = mempcpy(
                cp_0 as *mut ::core::ffi::c_void,
                c as *const ::core::ffi::c_void,
                len_1 as size_t,
            ) as *mut ::core::ffi::c_char;
            let fresh0 = cp_0;
            cp_0 = cp_0.offset(1 as ::core::ffi::c_int as isize);
            *fresh0 = FILE_LIST_SEPARATOR as ::core::ffi::c_char;
            if !((*d).changed() as ::core::ffi::c_int != 0 || always_make_flag != 0) {
                qmark_len = qmark_len.wrapping_sub(len_1.wrapping_add(1));
            }
        }
        d = (*d).next;
    }
    *cp_0.offset(
        (if cp_0 > plus_value {
            -(1 as ::core::ffi::c_int)
        } else {
            0
        }) as isize,
    ) = 0;
    define_variable_in_set(
        b"+\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        plus_value,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
    caret_value = plus_value;
    cp_0 = caret_value;
    if qmark_len > qmark_max {
        qmark_max = qmark_len;
        qmark_value = xrealloc(qmark_value as *mut ::core::ffi::c_void, qmark_max)
            as *mut ::core::ffi::c_char;
    }
    qp = qmark_value;
    if bar_len > bar_max {
        bar_max = bar_len;
        bar_value =
            xrealloc(bar_value as *mut ::core::ffi::c_void, bar_max) as *mut ::core::ffi::c_char;
    }
    bp = bar_value;
    hash_init(
        &raw mut dep_hash,
        500,
        Some(
            dep_hash_1 as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            dep_hash_2 as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            dep_hash_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    d = (*file).deps;
    while !d.is_null() {
        if !((*d).need_2nd_expansion() as ::core::ffi::c_int != 0
            || (*d).ignore_automatic_vars() as ::core::ffi::c_int != 0)
        {
            slot = hash_find_slot(&raw mut dep_hash, d as *const ::core::ffi::c_void);
            if (*slot).is_null() || *slot == hash_deleted_item as *mut ::core::ffi::c_void {
                hash_insert_at(
                    &raw mut dep_hash,
                    d as *const ::core::ffi::c_void,
                    slot as *const ::core::ffi::c_void,
                );
            } else {
                let hd: *mut dep = *slot as *mut dep;
                if (*d).ignore_mtime() as ::core::ffi::c_int
                    != (*hd).ignore_mtime() as ::core::ffi::c_int
                {
                    let rhs = {
                        (*hd).set_ignore_mtime(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*hd).ignore_mtime()
                    } as ::core::ffi::c_uint;
                    (*d).set_ignore_mtime(rhs);
                }
            }
        }
        d = (*d).next;
    }
    d = (*file).deps;
    while !d.is_null() {
        let mut c_0: *const ::core::ffi::c_char;
        if !((*d).need_2nd_expansion() as ::core::ffi::c_int != 0
            || (*d).ignore_automatic_vars() as ::core::ffi::c_int != 0
            || hash_find_item(&raw mut dep_hash, d as *const ::core::ffi::c_void)
                != d as *mut ::core::ffi::c_void)
        {
            c_0 = if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            if ar_name(c_0) != 0 {
                c_0 = strchr(c_0, '(' as i32).offset(1 as ::core::ffi::c_int as isize);
                len_1 = strlen(c_0).wrapping_sub(1) as size_t;
            } else {
                len_1 = strlen(c_0) as size_t;
            }
            if (*d).ignore_mtime() != 0 {
                bp = mempcpy(
                    bp as *mut ::core::ffi::c_void,
                    c_0 as *const ::core::ffi::c_void,
                    len_1 as size_t,
                ) as *mut ::core::ffi::c_char;
                let fresh1 = bp;
                bp = bp.offset(1 as ::core::ffi::c_int as isize);
                *fresh1 = FILE_LIST_SEPARATOR as ::core::ffi::c_char;
            } else {
                cp_0 = mempcpy(
                    cp_0 as *mut ::core::ffi::c_void,
                    c_0 as *const ::core::ffi::c_void,
                    len_1 as size_t,
                ) as *mut ::core::ffi::c_char;
                let fresh2 = cp_0;
                cp_0 = cp_0.offset(1 as ::core::ffi::c_int as isize);
                *fresh2 = FILE_LIST_SEPARATOR as ::core::ffi::c_char;
                if (*d).changed() as ::core::ffi::c_int != 0 || always_make_flag != 0 {
                    qp = mempcpy(
                        qp as *mut ::core::ffi::c_void,
                        c_0 as *const ::core::ffi::c_void,
                        len_1 as size_t,
                    ) as *mut ::core::ffi::c_char;
                    let fresh3 = qp;
                    qp = qp.offset(1 as ::core::ffi::c_int as isize);
                    *fresh3 = FILE_LIST_SEPARATOR as ::core::ffi::c_char;
                }
            }
        }
        d = (*d).next;
    }
    hash_free(&raw mut dep_hash, 0);
    *cp_0.offset(
        (if cp_0 > caret_value {
            -(1 as ::core::ffi::c_int)
        } else {
            0
        }) as isize,
    ) = 0;
    define_variable_in_set(
        b"^\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        caret_value,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
    *qp.offset(
        (if qp > qmark_value {
            -(1 as ::core::ffi::c_int)
        } else {
            0
        }) as isize,
    ) = 0;
    define_variable_in_set(
        b"?\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        qmark_value,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
    *bp.offset(
        (if bp > bar_value {
            -(1 as ::core::ffi::c_int)
        } else {
            0
        }) as isize,
    ) = 0;
    define_variable_in_set(
        b"|\0" as *const u8 as *const ::core::ffi::c_char,
        1,
        bar_value,
        o_automatic,
        0,
        (*(*file).variables).set,
        NILF,
    );
}
#[no_mangle]
pub unsafe extern "C" fn chop_commands(cmds: *mut commands) {
    let mut nlines: ::core::ffi::c_ushort;
    let mut i: ::core::ffi::c_ushort;
    let mut lines: *mut *mut ::core::ffi::c_char;
    if cmds.is_null() || !(*cmds).command_lines.is_null() {
        return;
    }
    if one_shell != 0 {
        let l: size_t = strlen((*cmds).commands) as size_t;
        nlines = 1;
        lines = xmalloc(
            (nlines as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        ) as *mut *mut ::core::ffi::c_char;
        let fresh4 = &mut (*lines.offset(0 as ::core::ffi::c_int as isize));
        *fresh4 = xstrdup((*cmds).commands);
        if l > 0
            && *(*lines.offset(0 as ::core::ffi::c_int as isize)).offset(l.wrapping_sub(1) as isize)
                as ::core::ffi::c_int
                == '\n' as i32
        {
            *(*lines.offset(0 as ::core::ffi::c_int as isize)).offset(l.wrapping_sub(1) as isize) =
                0;
        }
    } else {
        let mut p: *const ::core::ffi::c_char = (*cmds).commands;
        let mut max: size_t = 5;
        nlines = 0;
        lines =
            xmalloc(max.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t))
                as *mut *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int != 0 {
            let mut end: *const ::core::ffi::c_char = p;
            loop {
                end = strchr(end, '\n' as i32);
                if end.is_null() {
                    end = p.offset(strlen(p) as isize);
                    break;
                } else {
                    if !(end > p
                        && *end.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                            == '\\' as i32)
                    {
                        break;
                    }
                    let mut backslash: ::core::ffi::c_int = 1;
                    if end > p.offset(1 as ::core::ffi::c_int as isize) {
                        let mut b: *const ::core::ffi::c_char;
                        b = end.offset(-(2 as ::core::ffi::c_int as isize));
                        while b >= p && *b as ::core::ffi::c_int == '\\' as i32 {
                            backslash = (backslash == 0) as ::core::ffi::c_int;
                            b = b.offset(-(1 as ::core::ffi::c_int) as isize);
                        }
                    }
                    if !(backslash != 0) {
                        break;
                    }
                    end = end.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            if nlines as ::core::ffi::c_int == USHRT_MAX {
                fatal(
                    &raw mut (*cmds).fileinfo,
                    INTSTR_LENGTH,
                    b"recipe has too many lines (limit %hu)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    nlines as ::core::ffi::c_int,
                );
            }
            if nlines as size_t == max {
                max = max.wrapping_add(2);
                lines = xrealloc(
                    lines as *mut ::core::ffi::c_void,
                    max.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
                ) as *mut *mut ::core::ffi::c_char;
            }
            let fresh5 = nlines;
            nlines = nlines.wrapping_add(1);
            let fresh6 = &mut (*lines.offset(fresh5 as isize));
            *fresh6 = xstrndup(p, end.offset_from(p) as ::core::ffi::c_long as size_t);
            p = end;
            if *p as ::core::ffi::c_int != 0 {
                p = p.offset(1 as ::core::ffi::c_int as isize);
            }
        }
    }
    (*cmds).ncommand_lines = nlines;
    (*cmds).command_lines = lines;
    (*cmds).set_any_recurse(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*cmds).lines_flags = xmalloc(nlines as size_t) as *mut ::core::ffi::c_uchar;
    i = 0;
    while (i as ::core::ffi::c_int) < nlines as ::core::ffi::c_int {
        let mut flags: ::core::ffi::c_uchar = 0;
        let mut p_0: *const ::core::ffi::c_char = *lines.offset(i as isize);
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p_0 as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x2 as ::core::ffi::c_int
            != 0
            || *p_0 as ::core::ffi::c_int == '-' as i32
            || *p_0 as ::core::ffi::c_int == '@' as i32
            || *p_0 as ::core::ffi::c_int == '+' as i32
        {
            let fresh7 = p_0;
            p_0 = p_0.offset(1 as ::core::ffi::c_int as isize);
            match *fresh7 as ::core::ffi::c_int {
                43 => {
                    flags =
                        (flags as ::core::ffi::c_int | COMMANDS_RECURSE) as ::core::ffi::c_uchar;
                }
                64 => {
                    flags = (flags as ::core::ffi::c_int | COMMANDS_SILENT) as ::core::ffi::c_uchar;
                }
                45 => {
                    flags =
                        (flags as ::core::ffi::c_int | COMMANDS_NOERROR) as ::core::ffi::c_uchar;
                }
                _ => {}
            }
        }
        if !(flags as ::core::ffi::c_int & 1 != 0)
            && (!strstr(p_0, b"$(MAKE)\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                || !strstr(p_0, b"${MAKE}\0" as *const u8 as *const ::core::ffi::c_char).is_null())
        {
            flags = (flags as ::core::ffi::c_int | COMMANDS_RECURSE) as ::core::ffi::c_uchar;
        }
        *(*cmds).lines_flags.offset(i as isize) = flags;
        (*cmds).set_any_recurse(
            (*cmds).any_recurse()
                | (if flags as ::core::ffi::c_int & 1 != 0 {
                    1
                } else {
                    0
                }) as ::core::ffi::c_uint,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn execute_file_commands(file: *mut file) {
    let mut p: *const ::core::ffi::c_char;
    p = (*(*file).cmds).commands;
    while *p as ::core::ffi::c_int != 0 {
        if !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0)
            && *p as ::core::ffi::c_int != '-' as i32
            && *p as ::core::ffi::c_int != '@' as i32
            && *p as ::core::ffi::c_int != '+' as i32
        {
            break;
        }
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    if *p as ::core::ffi::c_int == 0 {
        set_command_state(file, cs_running);
        (*file).set_update_status(us_success as update_status);
        notice_finished_file(file);
        return;
    }
    initialize_file_variables(file, 0);
    set_file_variables(file, (*file).stem);
    if (*file).loaded() as ::core::ffi::c_int != 0 && unload_file((*file).name) == 0 {
        (*file).set_loaded(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*file).set_unloaded(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    new_job(file);
}
#[no_mangle]
pub static mut handling_fatal_signal: sig_atomic_t = 0 as sig_atomic_t;
#[no_mangle]
pub unsafe extern "C" fn fatal_error_signal(sig: ::core::ffi::c_int) {
    ::core::ptr::write_volatile(
        &raw mut handling_fatal_signal as *mut sig_atomic_t,
        1 as ::core::ffi::c_int as sig_atomic_t,
    );
    signal(sig, SIG_DFL);
    temp_stdin_unlink();
    osync_clear();
    jobserver_clear();
    if sig == SIGTERM {
        let mut c: *mut child;
        c = children;
        while !c.is_null() {
            if (*c).remote() == 0 && (*c).pid > 0 {
                kill((*c).pid as __pid_t, SIGTERM);
            }
            c = (*c).next;
        }
    }
    if sig == SIGTERM || sig == SIGINT || sig == SIGHUP || sig == SIGQUIT {
        let mut c_0: *mut child;
        c_0 = children;
        while !c_0.is_null() {
            if (*c_0).remote() as ::core::ffi::c_int != 0 && (*c_0).pid > 0 {
                crate::remote_stub::remote_kill((*c_0).pid, sig);
            }
            c_0 = (*c_0).next;
        }
        c_0 = children;
        while !c_0.is_null() {
            delete_child_targets(c_0);
            c_0 = (*c_0).next;
        }
        while job_slots_used > 0 {
            reap_children(1, 0);
        }
    } else {
        while job_slots_used > 0 {
            reap_children(1, 1);
        }
    }
    remove_intermediates(1);
    if sig == SIGQUIT {
        exit(MAKE_TROUBLE);
    }
    if kill(make_pid() as __pid_t, sig) < 0 {
        pfatal_with_name(b"kill\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn delete_target(file: *mut file, on_behalf_of: *const ::core::ffi::c_char) {
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
    if (*file).precious() as ::core::ffi::c_int != 0 || (*file).phony() as ::core::ffi::c_int != 0 {
        return;
    }
    if ar_name((*file).name) != 0 {
        let file_date: time_t = if (*file).last_mtime == NONEXISTENT_MTIME as uintmax_t {
            -(1 as ::core::ffi::c_int) as time_t
        } else {
            ((*file)
                .last_mtime
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) as time_t
        };
        if ar_member_date((*file).name) != file_date {
            if !on_behalf_of.is_null() {
                error(
                    ::core::ptr::null_mut::<Floc>(),
                    (strlen(on_behalf_of) as size_t).wrapping_add(strlen((*file).name) as size_t),
                    b"*** [%s] archive member '%s' may be bogus; not deleted\0" as *const u8
                        as *const ::core::ffi::c_char,
                    on_behalf_of,
                    (*file).name,
                );
            } else {
                error(
                    ::core::ptr::null_mut::<Floc>(),
                    strlen((*file).name) as size_t,
                    b"*** archive member '%s' may be bogus; not deleted\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*file).name,
                );
            }
        }
        return;
    }
    loop {
        e = stat((*file).name, &raw mut st);
        if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if e == 0
        && st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        && file_timestamp_cons(
            (*file).name,
            st.st_mtim.tv_sec as time_t,
            st.st_mtim.tv_nsec as ::core::ffi::c_long,
        ) != (*file).last_mtime
    {
        if !on_behalf_of.is_null() {
            error(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(on_behalf_of) as size_t).wrapping_add(strlen((*file).name) as size_t),
                b"*** [%s] deleting file '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                on_behalf_of,
                (*file).name,
            );
        } else {
            error(
                ::core::ptr::null_mut::<Floc>(),
                strlen((*file).name) as size_t,
                b"*** deleting file '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
        }
        if unlink((*file).name) < 0 && *__errno_location() != ENOENT {
            perror_with_name(
                b"unlink: \0" as *const u8 as *const ::core::ffi::c_char,
                (*file).name,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete_child_targets(child: *mut child) {
    let mut d: *mut dep;
    if (*child).deleted() as ::core::ffi::c_int != 0 || (*child).pid < 0 {
        return;
    }
    delete_target((*child).file, ::core::ptr::null::<::core::ffi::c_char>());
    d = (*(*child).file).also_make;
    while !d.is_null() {
        delete_target((*d).file, (*(*child).file).name);
        d = (*d).next;
    }
    (*child).set_deleted(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn print_commands(cmds: *const commands) {
    let mut s: *const ::core::ffi::c_char;
    fputs(
        b"#  recipe to execute\0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    if (*cmds).fileinfo.filenm.is_null() {
        puts(b" (built-in):\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        printf(
            b" (from '%s', line %lu):\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*cmds).fileinfo.filenm,
            (*cmds).fileinfo.lineno,
        );
    }
    s = (*cmds).commands;
    while *s as ::core::ffi::c_int != 0 {
        let mut end: *const ::core::ffi::c_char;
        let mut bs: ::core::ffi::c_int;
        end = s;
        bs = 0;
        while *end as ::core::ffi::c_int != 0 {
            if *end as ::core::ffi::c_int == '\n' as i32 && bs == 0 {
                break;
            }
            bs = if *end as ::core::ffi::c_int == '\\' as i32 {
                (bs == 0) as ::core::ffi::c_int
            } else {
                0
            };
            end = end.offset(1 as ::core::ffi::c_int as isize);
        }
        printf(
            b"%c%.*s\n\0" as *const u8 as *const ::core::ffi::c_char,
            cmd_prefix as ::core::ffi::c_int,
            end.offset_from(s) as ::core::ffi::c_long as ::core::ffi::c_int,
            s,
        );
        s = end.offset(
            (*end.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32)
                as ::core::ffi::c_int as isize,
        );
    }
}
pub const __SHRT_MAX__: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const FILE_TIMESTAMP_HI_RES: ::core::ffi::c_int = 1;
