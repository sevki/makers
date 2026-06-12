pub use crate::ffi_types::{
    __clockid_t, __off64_t, __off_t, __suseconds_t, __syscall_slong_t, __time_t, clockid_t,
    intmax_t, size_t, time_t, uintmax_t,
};
use crate::misc::free_ns_chain;
use crate::misc::{copy_dep_chain, end_of_token, xmalloc, xrealloc, xstrdup};
use crate::stdio::FILE;
use crate::strcache::{strcache_add_len, strcache_iscached};
use c2rust_bitfields;
use libc::{
    __errno_location, abort, free, printf, putchar, puts, sprintf, strchr, strcmp, strcpy, unlink,
};
use std::ffi::CStr;
use std::sync::Mutex;
extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub use crate::sys_stat::timespec;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct File {
    pub name: *const ::core::ffi::c_char,
    pub hname: *const ::core::ffi::c_char,
    pub vpath: *const ::core::ffi::c_char,
    pub deps: *mut Dep,
    pub cmds: *mut Commands,
    pub stem: *const ::core::ffi::c_char,
    pub also_make: *mut Dep,
    pub prev: *mut File,
    pub last: *mut File,
    pub renamed: *mut File,
    pub variables: *mut VariableSetList,
    pub pat_variables: *mut VariableSetList,
    pub parent: *mut File,
    pub double_colon: *mut File,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: ::core::ffi::c_uint,
    pub command_flags: ::core::ffi::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(
        name = "low_resolution_time",
        ty = "::core::ffi::c_uint",
        bits = "8..=8"
    )]
    #[bitfield(name = "tried_implicit", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "::core::ffi::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "::core::ffi::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "::core::ffi::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "::core::ffi::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "::core::ffi::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "::core::ffi::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "::core::ffi::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "::core::ffi::c_uint", bits = "24..=24")]
    #[bitfield(name = "suffix", ty = "::core::ffi::c_uint", bits = "25..=25")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
impl Default for File {
    fn default() -> Self {
        Self {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            hname: ::core::ptr::null::<::core::ffi::c_char>(),
            vpath: ::core::ptr::null::<::core::ffi::c_char>(),
            deps: ::core::ptr::null_mut::<Dep>(),
            cmds: ::core::ptr::null_mut::<Commands>(),
            stem: ::core::ptr::null::<::core::ffi::c_char>(),
            also_make: ::core::ptr::null_mut::<Dep>(),
            prev: ::core::ptr::null_mut::<File>(),
            last: ::core::ptr::null_mut::<File>(),
            renamed: ::core::ptr::null_mut::<File>(),
            variables: ::core::ptr::null_mut::<VariableSetList>(),
            pat_variables: ::core::ptr::null_mut::<VariableSetList>(),
            parent: ::core::ptr::null_mut::<File>(),
            double_colon: ::core::ptr::null_mut::<File>(),
            last_mtime: 0,
            mtime_before_update: 0,
            considered: 0,
            command_flags: 0,
            update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix:
                [0; 4],
            c2rust_padding: [0; 4],
        }
    }
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSetList {
    pub next: *mut VariableSetList,
    pub set: *mut VariableSet,
    pub next_is_parent: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSet {
    pub table: hash_table,
}
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Dep {
    pub next: *mut Dep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut File,
    pub shuf: *mut Dep,
    pub stem: *const ::core::ffi::c_char,
    #[bitfield(name = "flags", ty = "::core::ffi::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(
        name = "need_2nd_expansion",
        ty = "::core::ffi::c_uint",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "ignore_automatic_vars",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "::core::ffi::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Commands {
    pub fileinfo: Floc,
    pub commands: *mut ::core::ffi::c_char,
    pub command_lines: *mut *mut ::core::ffi::c_char,
    pub lines_flags: *mut ::core::ffi::c_uchar,
    pub ncommand_lines: ::core::ffi::c_ushort,
    pub recipe_prefix: ::core::ffi::c_char,
    #[bitfield(name = "any_recurse", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
use crate::commands::{print_commands, set_file_variables};
use crate::expand::{
    expand_string_buf, expand_string_for_file, variable_buffer, variable_buffer_output,
};
use crate::floc::Floc;
use crate::function::patsubst_expand_pat;
use crate::hash::{
    hash_delete, hash_deleted_item, hash_dump, hash_find_item, hash_find_slot, hash_init,
    hash_insert_at, hash_map, hash_print_stats, is_real_item, jhash_string, table_slots,
};
use crate::make_main::{
    cmd_prefix, db_level, export_all_variables, ignore_errors_flag, just_print_flag,
    no_builtin_rules_flag, no_intermediates, not_parallel, question_flag, run_silent,
    second_expansion, stopchar_map, touch_flag, verify_flag, MAP_DIRSEP,
};
use crate::output::{error, fatal, perror_with_name};
use crate::read::{find_percent, parse_file_seq};
use crate::variable::{
    initialize_file_variables, lookup_variable, lookup_variable_in_set, merge_variable_set_lists,
    print_file_variables, print_target_variables,
};

pub type file = File;
pub type dep = Dep;
pub type commands = Commands;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;

pub const o_invalid: variable_origin = 7;
pub const o_automatic: variable_origin = 6;
pub const o_override: variable_origin = 5;
pub const o_command: variable_origin = 4;
pub const o_env_override: variable_origin = 3;
pub const o_file: variable_origin = 2;
pub const o_env: variable_origin = 1;
pub const o_default: variable_origin = 0;
pub use crate::variable::variable;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
pub type hash_map_func_t = crate::hash::hash_map_func_t;
pub type qsort_cmp_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const RECIPEPREFIX_DEFAULT: ::core::ffi::c_int = '\t' as i32;
pub const COMMANDS_SILENT: ::core::ffi::c_int = 2;
pub const COMMANDS_NOERROR: ::core::ffi::c_int = 4;

impl File {
    fn new_named(name: *const ::core::ffi::c_char) -> Self {
        let mut file = Self {
            name,
            hname: name,
            ..Self::default()
        };
        file.set_update_status(us_none as update_status);
        file
    }
}

fn boxed_file(name: *const ::core::ffi::c_char) -> *mut file {
    Box::into_raw(Box::new(File::new_named(name)))
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_ns(n: *mut nameseq) {
    free(n as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_dep(d: *mut dep) {
    free_ns(d as *mut nameseq);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_dep_chain(d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
pub const UNKNOWN_MTIME: ::core::ffi::c_int = 0;
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
pub const OLD_MTIME: ::core::ffi::c_int = 2;
pub const ORDINARY_MTIME_MIN: ::core::ffi::c_int = OLD_MTIME + 1;
pub static mut snapped_deps: ::core::ffi::c_int = 0;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_hash_1(key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar =
        (*(key as *const file)).hname as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    _result_
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_hash_2(mut _key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe fn file_hash_cmp(
    x: *const ::core::ffi::c_void,
    y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if (*(x as *const file)).hname == (*(y as *const file)).hname {
        0
    } else {
        strcmp((*(x as *const file)).hname, (*(y as *const file)).hname)
    }
}
static mut files: hash_table = hash_table {
    ht_vec: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
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

#[derive(Copy, Clone)]
struct RehashedFile {
    _ptr: *mut file,
}

// These file records are process-global C objects. The mutex protects the
// side-list ownership; the records themselves are still managed by `files`.
unsafe impl Send for RehashedFile {}

static REHASHED_FILES: Mutex<Vec<RehashedFile>> = Mutex::new(Vec::new());
static mut all_secondary: ::core::ffi::c_int = 0;

unsafe fn stop_set_byte(c: u8, mask: ::core::ffi::c_int) -> bool {
    stopchar_map[c as usize] as ::core::ffi::c_int & mask != 0
}

unsafe fn normalize_lookup_name(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    assert!(!name.is_null(), "assertion failed: name != NULL");
    let bytes = CStr::from_ptr(name).to_bytes_with_nul();
    assert!(bytes[0] != 0, "assertion failed: *name != '\\0'");

    let mut pos = 0usize;
    while bytes[pos] == b'.' && stop_set_byte(bytes[pos + 1], MAP_DIRSEP) && bytes[pos + 2] != 0 {
        pos += 2;
        while stop_set_byte(bytes[pos], MAP_DIRSEP) {
            pos += 1;
        }
    }

    if bytes[pos] == 0 {
        c"./".as_ptr()
    } else {
        bytes[pos..].as_ptr().cast()
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file {
    let f: *mut file;
    let name = normalize_lookup_name(name);
    let mut file_key = File::default();
    file_key.hname = name;
    f = hash_find_item(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut file;
    f
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn enter_file(name: *const ::core::ffi::c_char) -> *mut file {
    let f: *mut file;
    let file_slot: *mut *mut file;
    let mut file_key = File::default();
    if *name as ::core::ffi::c_int != 0 {
    } else {
        panic!("assertion failed: *name != '\'");
    };
    if verify_flag == 0 || strcache_iscached(name) != 0 {
    } else {
        panic!("assertion failed: ! verify_flag || strcache_iscached (name)");
    };
    file_key.hname = name;
    file_slot = hash_find_slot(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut *mut file;
    f = *file_slot;
    if !(f.is_null()
        || f as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
        && (*f).double_colon.is_null()
    {
        (*f).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        return f;
    }
    let new = boxed_file(name);
    if f.is_null() || f as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        (*new).last = new;
        hash_insert_at(
            &raw mut files,
            new as *const ::core::ffi::c_void,
            file_slot as *const ::core::ffi::c_void,
        );
    } else {
        (*new).double_colon = f;
        (*(*f).last).prev = new;
        (*f).last = new;
    }
    new
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn rehash_file(mut from_file: *mut file, to_hname: *const ::core::ffi::c_char) {
    let mut file_key = File::default();
    let file_slot: *mut *mut file;
    let to_file: *mut file;
    let deleted_file: *mut file;
    let mut f: *mut file;
    (*from_file).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    file_key.hname = to_hname;
    if file_hash_cmp(
        from_file as *const ::core::ffi::c_void,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) == 0
    {
        return;
    }
    file_key.hname = (*from_file).hname;
    while !(*from_file).renamed.is_null() {
        from_file = (*from_file).renamed;
    }
    if file_hash_cmp(
        from_file as *const ::core::ffi::c_void,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) != 0
    {
        abort();
    }
    deleted_file =
        hash_delete(&raw mut files, from_file as *const ::core::ffi::c_void) as *mut file;
    if deleted_file != from_file {
        abort();
    }
    file_key.hname = to_hname;
    file_slot = hash_find_slot(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut *mut file;
    to_file = *file_slot;
    (*from_file).hname = to_hname;
    f = (*from_file).double_colon;
    while !f.is_null() {
        (*f).hname = to_hname;
        f = (*f).prev;
    }
    if to_file.is_null()
        || to_file as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void
    {
        hash_insert_at(
            &raw mut files,
            from_file as *const ::core::ffi::c_void,
            file_slot as *const ::core::ffi::c_void,
        );
        return;
    }
    if !(*from_file).cmds.is_null() {
        if (*to_file).cmds.is_null() {
            (*to_file).cmds = (*from_file).cmds;
        } else if (*from_file).cmds != (*to_file).cmds {
            let mut l: size_t = strlen((*from_file).name) as size_t;
            if !(*(*to_file).cmds).fileinfo.filenm.is_null() {
                error(
                    &raw mut (*(*from_file).cmds).fileinfo,
                    l.wrapping_add(strlen((*(*to_file).cmds).fileinfo.filenm) as size_t)
                        .wrapping_add(INTSTR_LENGTH),
                    b"recipe was specified for file '%s' at %s:%lu,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*from_file).name,
                    (*(*from_file).cmds).fileinfo.filenm,
                    (*(*from_file).cmds).fileinfo.lineno,
                );
            } else {
                error(
                    &raw mut (*(*from_file).cmds).fileinfo,
                    l,
                    b"recipe for file '%s' was found by implicit rule search,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*from_file).name,
                );
            }
            l = l.wrapping_add(strlen(to_hname) as size_t);
            error(
                &raw mut (*(*from_file).cmds).fileinfo,
                l,
                b"but '%s' is now considered the same file as '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*from_file).name,
                to_hname,
            );
            error(
                &raw mut (*(*from_file).cmds).fileinfo,
                l,
                b"recipe for '%s' will be ignored in favor of the one for '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*from_file).name,
                to_hname,
            );
        }
    }
    if (*to_file).deps.is_null() {
        (*to_file).deps = (*from_file).deps;
    } else {
        let mut deps: *mut dep = (*to_file).deps;
        while !(*deps).next.is_null() {
            deps = (*deps).next;
        }
        (*deps).next = (*from_file).deps;
    }
    merge_variable_set_lists(&raw mut (*to_file).variables, (*from_file).variables);
    if !(*to_file).double_colon.is_null()
        && (*from_file).is_target() as ::core::ffi::c_int != 0
        && (*from_file).double_colon.is_null()
    {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            (strlen((*from_file).name) as size_t).wrapping_add(strlen(to_hname) as size_t),
            b"can't rename single-colon '%s' to double-colon '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            (*from_file).name,
            to_hname,
        );
    }
    if (*to_file).double_colon.is_null() && !(*from_file).double_colon.is_null() {
        if (*to_file).is_target() != 0 {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                (strlen((*from_file).name) as size_t).wrapping_add(strlen(to_hname) as size_t),
                b"can't rename double-colon '%s' to single-colon '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*from_file).name,
                to_hname,
            );
        } else {
            (*to_file).double_colon = (*from_file).double_colon;
        }
    }
    if (*from_file).last_mtime > (*to_file).last_mtime {
        (*to_file).last_mtime = (*from_file).last_mtime;
    }
    (*to_file).mtime_before_update = (*from_file).mtime_before_update;
    (*to_file).set_precious(
        (*to_file).precious()
            | (*from_file).precious() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_loaded(
        (*to_file).loaded() | (*from_file).loaded() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_tried_implicit(
        (*to_file).tried_implicit()
            | (*from_file).tried_implicit() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_updating(
        (*to_file).updating()
            | (*from_file).updating() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_updated(
        (*to_file).updated() | (*from_file).updated() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_is_target(
        (*to_file).is_target()
            | (*from_file).is_target() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_cmd_target(
        (*to_file).cmd_target()
            | (*from_file).cmd_target() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_phony(
        (*to_file).phony() | (*from_file).phony() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_is_explicit(
        (*to_file).is_explicit()
            | (*from_file).is_explicit() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_secondary(
        (*to_file).secondary()
            | (*from_file).secondary() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_notintermediate(
        (*to_file).notintermediate()
            | (*from_file).notintermediate() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_ignore_vpath(
        (*to_file).ignore_vpath()
            | (*from_file).ignore_vpath() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_snapped(
        (*to_file).snapped() | (*from_file).snapped() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_suffix(
        (*to_file).suffix() | (*from_file).suffix() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*from_file).renamed = to_file;
    REHASHED_FILES
        .lock()
        .expect("rehashed file list lock poisoned")
        .push(RehashedFile { _ptr: from_file });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn rename_file(mut from_file: *mut file, to_hname: *const ::core::ffi::c_char) {
    rehash_file(from_file, to_hname);
    while !from_file.is_null() {
        (*from_file).name = (*from_file).hname;
        from_file = (*from_file).prev;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn remove_intermediates(sig: ::core::ffi::c_int) {
    let mut doneany: ::core::ffi::c_int = 0;
    if question_flag != 0 || touch_flag != 0 || all_secondary != 0 || no_intermediates != 0 {
        return;
    }
    if sig != 0 && just_print_flag != 0 {
        return;
    }
    for slot in table_slots(&raw const files) {
        if is_real_item(*slot) {
            let f = *slot as *mut file;
            if (*f).intermediate() as ::core::ffi::c_int != 0
                && ((*f).dontcare() as ::core::ffi::c_int != 0 || (*f).precious() == 0)
                && (*f).secondary() == 0
                && (*f).notintermediate() == 0
                && (*f).cmd_target() == 0
            {
                let status: ::core::ffi::c_int;
                if (*f).update_status() as ::core::ffi::c_int != us_none as ::core::ffi::c_int {
                    // ENOENT from unlink means the file was already gone: skip the
                    // diagnostic/bookkeeping below (the C code `continue`d here).
                    let skip: bool;
                    if just_print_flag != 0 {
                        status = 0;
                        skip = false;
                    } else {
                        status = unlink((*f).name);
                        skip = status < 0 && *__errno_location() == ENOENT;
                    }
                    if !skip && (*f).dontcare() == 0 {
                        if sig != 0 {
                            error(
                                ::core::ptr::null_mut::<Floc>(),
                                strlen((*f).name) as size_t,
                                b"*** deleting intermediate file '%s'\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*f).name,
                            );
                        } else {
                            if doneany == 0 && 0x1 as ::core::ffi::c_int & db_level != 0 {
                                printf(
                                    b"Removing intermediate files...\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                fflush(stdout);
                            }
                            if run_silent == 0 {
                                if doneany == 0 {
                                    fputs(
                                        b"rm \0" as *const u8 as *const ::core::ffi::c_char,
                                        stdout,
                                    );
                                    doneany = 1;
                                } else {
                                    putchar(' ' as i32);
                                }
                                fputs((*f).name, stdout);
                                fflush(stdout);
                            }
                        }
                        if status < 0 {
                            if doneany != 0 {
                                fputs(b"\n\0" as *const u8 as *const ::core::ffi::c_char, stdout);
                            }
                            fflush(stdout);
                            perror_with_name(
                                b"unlink: \0" as *const u8 as *const ::core::ffi::c_char,
                                (*f).name,
                            );
                            doneany = 0;
                        }
                    }
                }
            }
        }
    }
    if doneany != 0 && sig == 0 {
        putchar('\n' as i32);
        fflush(stdout);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn split_prereqs(mut p: *mut ::core::ffi::c_char) -> *mut dep {
    let mut new: *mut dep = parse_file_seq(
        &raw mut p,
        ::core::mem::size_of::<dep>() as size_t,
        0x100 as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x40 as ::core::ffi::c_int,
    ) as *mut dep;
    if *p != 0 {
        let mut ood: *mut dep;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        ood = parse_file_seq(
            &raw mut p,
            ::core::mem::size_of::<dep>() as size_t,
            0x1 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0x40 as ::core::ffi::c_int,
        ) as *mut dep;
        if new.is_null() {
            new = ood;
        } else {
            let mut dp: *mut dep;
            dp = new;
            while !(*dp).next.is_null() {
                dp = (*dp).next;
            }
            (*dp).next = ood;
        }
        while !ood.is_null() {
            (*ood).set_ignore_mtime(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            ood = (*ood).next;
        }
    }
    new
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn enter_prereqs(mut deps: *mut dep, stem: *const ::core::ffi::c_char) -> *mut dep {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut d1: *mut dep;
    if deps.is_null() {
        return ::core::ptr::null_mut::<dep>();
    }
    if !stem.is_null() {
        let pattern: *const ::core::ffi::c_char = b"%\0" as *const u8 as *const ::core::ffi::c_char;
        let mut dp: *mut dep = deps;
        let mut dl: *mut dep = ::core::ptr::null_mut::<dep>();
        while !dp.is_null() {
            let percent: *mut ::core::ffi::c_char;
            let nl: size_t = (strlen((*dp).name) as size_t).wrapping_add(1);
            alloca_allocations.push(::std::vec::from_elem(0, nl as usize));
            let nm: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                nm as *mut ::core::ffi::c_void,
                (*dp).name as *const ::core::ffi::c_void,
                nl as size_t,
            );
            percent = find_percent(nm);
            if !percent.is_null() {
                let o: *mut ::core::ffi::c_char;
                if *stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
                    memmove(
                        percent as *mut ::core::ffi::c_void,
                        percent.offset(1 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        strlen(percent),
                    );
                    o = variable_buffer_output(
                        variable_buffer,
                        nm,
                        (strlen(nm) as size_t).wrapping_add(1),
                    );
                } else {
                    o = patsubst_expand_pat(
                        variable_buffer,
                        stem,
                        pattern,
                        nm,
                        pattern.offset(1 as ::core::ffi::c_int as isize),
                        percent.offset(1 as ::core::ffi::c_int as isize),
                    );
                }
                if *variable_buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0
                {
                    let df: *mut dep = dp;
                    if dp == deps {
                        deps = (*deps).next;
                        dp = deps;
                    } else {
                        (*dl).next = (*dp).next;
                        dp = (*dl).next;
                    }
                    free_dep(df);
                    continue;
                } else {
                    (*dp).name = strcache_add_len(
                        variable_buffer,
                        o.offset_from(variable_buffer) as ::core::ffi::c_long as size_t,
                    );
                }
            }
            (*dp).stem = stem;
            (*dp).set_staticpattern(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            dl = dp;
            dp = (*dp).next;
        }
    }
    d1 = deps;
    while !d1.is_null() {
        if !((*d1).need_2nd_expansion() != 0) {
            (*d1).file = lookup_file((*d1).name);
            if (*d1).file.is_null() {
                (*d1).file = enter_file((*d1).name);
            }
            (*d1).set_staticpattern(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*d1).name = ::core::ptr::null::<::core::ffi::c_char>();
            if stem.is_null() {
                (*(*d1).file).set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        d1 = (*d1).next;
    }
    deps
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_deps(f: *mut file) {
    let mut d: *mut dep;
    let mut dp: *mut *mut dep;
    let mut fstem: *const ::core::ffi::c_char;
    let mut initialized: ::core::ffi::c_int = 0;
    let mut changed_dep: ::core::ffi::c_int = 0;
    if (*f).snapped() != 0 {
        return;
    }
    (*f).set_snapped(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    dp = &raw mut (*f).deps;
    d = (*f).deps;
    while !d.is_null() {
        let p: *mut ::core::ffi::c_char;
        let mut new: *mut dep;
        let next: *mut dep;
        if (*d).name.is_null() || (*d).need_2nd_expansion() == 0 {
            dp = &raw mut (*d).next;
            d = (*d).next;
        } else {
            if (*d).staticpattern() != 0 {
                let mut cs: *const ::core::ffi::c_char = (*d).name;
                let mut nperc: size_t = 0;
                loop {
                    cs = strchr(cs, '%' as i32);
                    if cs.is_null() {
                        break;
                    }
                    nperc = nperc.wrapping_add(1);
                    cs = cs.offset(1 as ::core::ffi::c_int as isize);
                }
                if nperc != 0 {
                    let slen: size_t = (strlen((*d).name) as size_t)
                        .wrapping_add(nperc)
                        .wrapping_add(1);
                    let mut pcs: *const ::core::ffi::c_char = (*d).name;
                    let name: *mut ::core::ffi::c_char = xmalloc(slen) as *mut ::core::ffi::c_char;
                    let mut s: *mut ::core::ffi::c_char = name;
                    cs = strchr(pcs, '%' as i32);
                    while !cs.is_null() {
                        s = mempcpy(
                            s as *mut ::core::ffi::c_void,
                            pcs as *const ::core::ffi::c_void,
                            cs.offset_from(pcs) as ::core::ffi::c_long as size_t,
                        ) as *mut ::core::ffi::c_char;
                        let fresh0 = s;
                        s = s.offset(1 as ::core::ffi::c_int as isize);
                        *fresh0 = '$' as i32 as ::core::ffi::c_char;
                        let fresh1 = s;
                        s = s.offset(1 as ::core::ffi::c_int as isize);
                        *fresh1 = '*' as i32 as ::core::ffi::c_char;
                        cs = cs.offset(1 as ::core::ffi::c_int as isize);
                        pcs = cs;
                        cs = strchr(end_of_token(cs), '%' as i32);
                    }
                    strcpy(s, pcs);
                    free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
                    (*d).name = name;
                }
            }
            if initialized == 0 {
                initialize_file_variables(f, 0);
                initialized = 1;
            }
            set_file_variables(
                f,
                if !(*d).stem.is_null() {
                    (*d).stem
                } else {
                    (*f).stem
                },
            );
            p = expand_string_for_file((*d).name, f);
            free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            new = split_prereqs(p);
            if new.is_null() {
                *dp = (*d).next;
                changed_dep = 1;
                free_dep(d);
                d = *dp;
            } else {
                fstem = (*d).stem;
                next = (*d).next;
                changed_dep = 1;
                free_dep(d);
                *dp = new;
                dp = &raw mut new;
                d = new;
                while !d.is_null() {
                    (*d).file = lookup_file((*d).name);
                    if (*d).file.is_null() {
                        (*d).file = enter_file((*d).name);
                    }
                    (*d).name = ::core::ptr::null::<::core::ffi::c_char>();
                    (*d).stem = fstem;
                    if fstem.is_null() {
                        (*(*d).file)
                            .set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    dp = &raw mut (*d).next;
                    d = (*d).next;
                }
                *dp = next;
                d = *dp;
            }
        }
    }
    if changed_dep != 0 {
        crate::shuffle::shuffle_deps_recursive((*f).deps);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_extra_prereqs(extra: *const variable) -> *mut dep {
    let mut d: *mut dep;
    let prereqs: *mut dep = if !extra.is_null() {
        split_prereqs(expand_string_buf(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            (*extra).value,
            SIZE_MAX as size_t,
        ))
    } else {
        ::core::ptr::null_mut::<dep>()
    };
    d = prereqs;
    while !d.is_null() {
        (*d).file = lookup_file((*d).name);
        if (*d).file.is_null() {
            (*d).file = enter_file((*d).name);
        }
        (*d).name = ::core::ptr::null::<::core::ffi::c_char>();
        (*d).set_ignore_automatic_vars(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        d = (*d).next;
    }
    prereqs
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn snap_file(f: *mut file, deps: *const dep) {
    let mut prereqs: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut d: *mut dep;
    if second_expansion == 0 {
        (*f).set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if all_secondary != 0 && (*f).notintermediate() == 0 {
        (*f).set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if no_intermediates != 0 && (*f).intermediate() == 0 && (*f).secondary() == 0 {
        (*f).set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if !(*f).variables.is_null() {
        prereqs = expand_extra_prereqs(lookup_variable_in_set(
            b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
            (*(*f).variables).set,
        ));
        if second_expansion != 0 {
            d = prereqs;
            while !d.is_null() {
                if (*d).name.is_null() {
                    (*d).name = xstrdup((*(*d).file).name);
                }
                (*d).set_need_2nd_expansion(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                d = (*d).next;
            }
        }
    } else if (*f).is_target() != 0 {
        prereqs = copy_dep_chain(deps);
    }
    if !prereqs.is_null() {
        d = prereqs;
        while let Some(dr) = d.as_ref() {
            let dname: *const ::core::ffi::c_char = if !dr.name.is_null() {
                dr.name
            } else {
                dr.file.as_ref().expect("expand_deps: null dep file").name
            };
            let fname = (*f).name;
            let same = match (dname.as_ref(), fname.as_ref()) {
                (Some(&db), Some(&fb)) => {
                    fb as ::core::ffi::c_int == db as ::core::ffi::c_int
                        && (fb as ::core::ffi::c_int == 0
                            || strcmp(fname.offset(1), dname.offset(1)) == 0)
                }
                _ => false,
            };
            if same {
                break;
            }
            d = dr.next;
        }
        if !d.is_null() {
            free_dep_chain(prereqs);
        } else if (*f).deps.is_null() {
            (*f).deps = prereqs;
        } else {
            d = (*f).deps;
            while !(*d).next.is_null() {
                d = (*d).next;
            }
            (*d).next = prereqs;
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn snap_deps() {
    let mut f: *mut file;
    let mut f2: *mut file;
    let mut d: *mut dep;
    snapped_deps = 1;
    f = lookup_file(b".PRECIOUS\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".LOW_RESOLUTION_TIME\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_low_resolution_time(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".PHONY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_phony(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*f2).set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*f2).last_mtime = NONEXISTENT_MTIME as uintmax_t;
                (*f2).mtime_before_update = NONEXISTENT_MTIME as uintmax_t;
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".NOTINTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        if !(*f).deps.is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            no_intermediates = 1;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".INTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                if (*f2).notintermediate() != 0 {
                    fatal(
                        ::core::ptr::null_mut::<Floc>(),
                        strlen((*f2).name) as size_t,
                        b"%s cannot be both .NOTINTERMEDIATE and .INTERMEDIATE\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*f2).name,
                    );
                } else {
                    (*f2).set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".SECONDARY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        if !(*f).deps.is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if (*f2).notintermediate() != 0 {
                        fatal(
                            ::core::ptr::null_mut::<Floc>(),
                            strlen((*f2).name) as size_t,
                            b"%s cannot be both .NOTINTERMEDIATE and .SECONDARY\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*f2).name,
                        );
                    } else {
                        let rhs = {
                            (*f2).set_secondary(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            (*f2).secondary()
                        } as ::core::ffi::c_uint;
                        (*f2).set_intermediate(rhs);
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            all_secondary = 1;
        }
        f = (*f).prev;
    }
    if no_intermediates != 0 && all_secondary != 0 {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            0,
            b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    f = lookup_file(b".EXPORT_ALL_VARIABLES\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        export_all_variables = 1;
    }
    f = lookup_file(b".IGNORE\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        if (*f).deps.is_null() {
            ignore_errors_flag = 1;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= COMMANDS_NOERROR;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".SILENT\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        if (*f).deps.is_null() {
            run_silent = 1;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= COMMANDS_SILENT;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".NOTPARALLEL\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target() as ::core::ffi::c_int != 0 {
        let mut d2: *mut dep;
        if (*f).deps.is_null() {
            not_parallel = 1;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if !(*f2).deps.is_null() {
                        d2 = (*(*f2).deps).next;
                        while !d2.is_null() {
                            (*d2).set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            d2 = (*d2).next;
                        }
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    let prereqs: *mut dep = expand_extra_prereqs(lookup_variable(
        b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
    ));
    let filedump: *mut *mut ::core::ffi::c_void = hash_dump(
        &raw mut files,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        None,
    );
    let mut filep: *mut *mut ::core::ffi::c_void = filedump;
    while !(*filep).is_null() {
        snap_file(*filep as *mut file, prereqs);
        filep = filep.offset(1 as ::core::ffi::c_int as isize);
    }
    free(filedump as *mut ::core::ffi::c_void);
    free_dep_chain(prereqs);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn set_command_state(file: *mut file, state: cmd_state) {
    let mut d: *mut dep;
    (*file).set_command_state(state as cmd_state as cmd_state);
    d = (*file).also_make;
    while !d.is_null() {
        if state as ::core::ffi::c_uint > (*(*d).file).command_state() as ::core::ffi::c_uint {
            (*(*d).file).set_command_state(state as cmd_state as cmd_state);
        }
        d = (*d).next;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_timestamp_cons(
    fname: *const ::core::ffi::c_char,
    stamp: time_t,
    ns: ::core::ffi::c_long,
) -> uintmax_t {
    let offset: ::core::ffi::c_int = (ORDINARY_MTIME_MIN as ::core::ffi::c_long
        + (if FILE_TIMESTAMP_HI_RES != 0 { ns } else { 0 }))
        as ::core::ffi::c_int;
    let s: uintmax_t = stamp as uintmax_t;
    let product: uintmax_t = s << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 });
    let mut ts: uintmax_t = product.wrapping_add(offset as uintmax_t);
    if !(s
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
            .wrapping_sub((2 + 1) as uintmax_t)
            >> (if 1 != 0 { 30 } else { 0 })
            << (if 1 != 0 { 30 } else { 0 }))
        .wrapping_add((2 + 1) as uintmax_t)
        .wrapping_add(
            (if 1 != 0 {
                1000000000 as ::core::ffi::c_int
            } else {
                1
            }) as uintmax_t,
        )
        .wrapping_sub(1 as uintmax_t)
        .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
        && product <= ts
        && ts
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
            .wrapping_sub(1 as uintmax_t))
    {
        let mut buf: [::core::ffi::c_char; 43] = [0; 43];
        let f: *const ::core::ffi::c_char = if !fname.is_null() {
            fname
        } else {
            b"Current time\0" as *const u8 as *const ::core::ffi::c_char
        };
        ts = if s <= OLD_MTIME as uintmax_t {
            ORDINARY_MTIME_MIN as uintmax_t
        } else {
            ((!(0 as ::core::ffi::c_int as uintmax_t))
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
        };
        file_timestamp_sprintf(&raw mut buf as *mut ::core::ffi::c_char, ts);
        error(
            ::core::ptr::null_mut::<Floc>(),
            (strlen(f) as size_t)
                .wrapping_add(strlen(&raw mut buf as *mut ::core::ffi::c_char) as size_t),
            b"%s: timestamp out of range: substituting %s\0" as *const u8
                as *const ::core::ffi::c_char,
            f,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    }
    ts
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_timestamp_now(resolution: *mut ::core::ffi::c_int) -> uintmax_t {
    let r: ::core::ffi::c_int;
    let s: time_t;
    let ns: ::core::ffi::c_int;
    let mut timespec: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut timespec) == 0 {
        r = 1;
        s = timespec.tv_sec as time_t;
        ns = timespec.tv_nsec as ::core::ffi::c_int;
    } else {
        let mut timeval: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        if gettimeofday(
            &raw mut timeval,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) == 0
        {
            r = 1000 as ::core::ffi::c_int;
            s = timeval.tv_sec as time_t;
            ns = (timeval.tv_usec * 1000 as __suseconds_t) as ::core::ffi::c_int;
        } else {
            r = 1000000000 as ::core::ffi::c_int;
            s = time(::core::ptr::null_mut::<time_t>());
            ns = 0;
        }
    }
    *resolution = r;
    file_timestamp_cons(
        ::core::ptr::null::<::core::ffi::c_char>(),
        s,
        ns as ::core::ffi::c_long,
    )
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_timestamp_sprintf(mut p: *mut ::core::ffi::c_char, ts: uintmax_t) {
    let mut t: time_t = (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
        >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) as time_t;
    let tm: *mut tm = localtime(&raw mut t);
    if !tm.is_null() {
        let year: intmax_t = (*tm).tm_year as intmax_t;
        p = p.offset(sprintf(
            p,
            b"%04ld-%02d-%02d %02d:%02d:%02d\0" as *const u8 as *const ::core::ffi::c_char,
            year + 1900 as intmax_t,
            (*tm).tm_mon + 1,
            (*tm).tm_mday,
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec,
        ) as isize);
    } else if t < 0 as time_t {
        p = p.offset(sprintf(
            p,
            b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
            t as intmax_t,
        ) as isize);
    } else {
        p = p.offset(sprintf(
            p,
            b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
            t as uintmax_t,
        ) as isize);
    }
    p = p.offset(
        (sprintf(
            p,
            b".%09d\0" as *const u8 as *const ::core::ffi::c_char,
            (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1) as uintmax_t)
                as ::core::ffi::c_int,
        ) - 1) as isize,
    );
    while *p as ::core::ffi::c_int == '0' as i32 {
        p = p.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    p = p.offset((*p as ::core::ffi::c_int != '.' as i32) as ::core::ffi::c_int as isize);
    *p = 0;
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_prereqs(mut deps: *const dep) {
    let mut ood: *const dep = ::core::ptr::null::<dep>();
    while !deps.is_null() {
        if (*deps).ignore_mtime() == 0 {
            printf(
                b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                if (*deps).wait_here() as ::core::ffi::c_int != 0 {
                    b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !(*deps).name.is_null() {
                    (*deps).name
                } else {
                    (*(*deps).file).name
                },
            );
        } else if ood.is_null() {
            ood = deps;
        }
        deps = (*deps).next;
    }
    if !ood.is_null() {
        printf(
            b" | %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            if (*ood).wait_here() as ::core::ffi::c_int != 0 {
                b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !(*ood).name.is_null() {
                (*ood).name
            } else {
                (*(*ood).file).name
            },
        );
        ood = (*ood).next;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                printf(
                    b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    if (*ood).wait_here() as ::core::ffi::c_int != 0 {
                        b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    if !(*ood).name.is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    },
                );
            }
            ood = (*ood).next;
        }
    }
    putchar('\n' as i32);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_file(item: *const ::core::ffi::c_void) {
    let f: *const file = item as *const file;
    if no_builtin_rules_flag != 0 && (*f).builtin() as ::core::ffi::c_int != 0 {
        return;
    }
    putchar('\n' as i32);
    if !(*f).cmds.is_null()
        && (*(*f).cmds).recipe_prefix as ::core::ffi::c_int != cmd_prefix as ::core::ffi::c_int
    {
        fputs(
            b".RECIPEPREFIX = \0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        cmd_prefix = (*(*f).cmds).recipe_prefix;
        if cmd_prefix as ::core::ffi::c_int != RECIPEPREFIX_DEFAULT {
            putchar(cmd_prefix as ::core::ffi::c_int);
        }
        putchar('\n' as i32);
    }
    if !(*f).variables.is_null() {
        print_target_variables(f);
    }
    if (*f).is_target() == 0 {
        puts(b"# Not a target:\0" as *const u8 as *const ::core::ffi::c_char);
    }
    printf(
        b"%s:%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*f).name,
        if !(*f).double_colon.is_null() {
            b":\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    print_prereqs((*f).deps);
    if (*f).precious() != 0 {
        puts(
            b"#  Precious file (prerequisite of .PRECIOUS).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).phony() != 0 {
        puts(
            b"#  Phony target (prerequisite of .PHONY).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).cmd_target() != 0 {
        puts(b"#  Command line target.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*f).dontcare() != 0 {
        puts(
            b"#  A default, MAKEFILES, or -include/sinclude makefile.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).builtin() != 0 {
        puts(b"#  Builtin rule\0" as *const u8 as *const ::core::ffi::c_char);
    }
    puts(if (*f).tried_implicit() as ::core::ffi::c_int != 0 {
        b"#  Implicit rule search has been done.\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"#  Implicit rule search has not been done.\0" as *const u8 as *const ::core::ffi::c_char
    });
    if !(*f).stem.is_null() {
        printf(
            b"#  Implicit/static pattern stem: '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).stem,
        );
    }
    if (*f).intermediate() != 0 {
        puts(
            b"#  File is an intermediate prerequisite.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).notintermediate() != 0 {
        puts(
            b"#  File is a prerequisite of .NOTINTERMEDIATE.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).secondary() != 0 {
        puts(
            b"#  File is secondary (prerequisite of .SECONDARY).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).is_explicit() != 0 {
        puts(b"#  File is explicitly mentioned.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !(*f).also_make.is_null() {
        let mut d: *const dep;
        fputs(
            b"#  Also makes:\0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        d = (*f).also_make;
        while !d.is_null() {
            printf(
                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*(*d).file).name
                },
            );
            d = (*d).next;
        }
        putchar('\n' as i32);
    }
    if (*f).last_mtime == UNKNOWN_MTIME as uintmax_t {
        puts(b"#  Modification time never checked.\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*f).last_mtime == NONEXISTENT_MTIME as uintmax_t {
        puts(b"#  File does not exist.\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*f).last_mtime == OLD_MTIME as uintmax_t {
        puts(b"#  File is very old.\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let mut buf: [::core::ffi::c_char; 43] = [0; 43];
        file_timestamp_sprintf(&raw mut buf as *mut ::core::ffi::c_char, (*f).last_mtime);
        printf(
            b"#  Last modified %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    }
    puts(if (*f).updated() as ::core::ffi::c_int != 0 {
        b"#  File has been updated.\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"#  File has not been updated.\0" as *const u8 as *const ::core::ffi::c_char
    });
    match (*f).command_state() as ::core::ffi::c_int {
        2 => {
            puts(
                b"#  Recipe currently running (THIS IS A BUG).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        1 => {
            puts(
                b"#  Dependencies recipe running (THIS IS A BUG).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        0 | 3 => match (*f).update_status() as ::core::ffi::c_int {
            0 => {
                puts(b"#  Successfully updated.\0" as *const u8 as *const ::core::ffi::c_char);
            }
            2 => {
                if question_flag != 0 {
                } else {
                    panic!("assertion failed: question_flag");
                };
                puts(
                    b"#  Needs to be updated (-q is set).\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            3 => {
                puts(b"#  Failed to be updated.\0" as *const u8 as *const ::core::ffi::c_char);
            }
            1 | _ => {}
        },
        _ => {
            puts(
                b"#  Invalid value in 'command_state' member!\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fflush(stdout);
            fflush(stderr);
            abort();
        }
    }
    if !(*f).variables.is_null() {
        print_file_variables(f);
    }
    if !(*f).cmds.is_null() {
        print_commands((*f).cmds);
    }
    if !(*f).prev.is_null() {
        print_file((*f).prev as *const ::core::ffi::c_void);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_file_data_base() {
    puts(b"\n# Files\0" as *const u8 as *const ::core::ffi::c_char);
    hash_map(&raw mut files, Some(print_file));
    fputs(
        b"\n# files hash-table stats:\n# \0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    hash_print_stats(&raw mut files, stdout);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_target(item: *const ::core::ffi::c_void) {
    let f: *const file = item as *const file;
    if (*f).is_target() == 0 || (*f).suffix() as ::core::ffi::c_int != 0 {
        return;
    }
    if *(*f).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
        && *(*__ctype_b_loc()).offset(*(*f).name.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uchar as ::core::ffi::c_int
            as isize) as ::core::ffi::c_int
            & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        let mut cp: *const ::core::ffi::c_char = (*f).name.offset(1 as ::core::ffi::c_int as isize);
        loop {
            cp = cp.offset(1 as ::core::ffi::c_int as isize);
            if !(*cp as ::core::ffi::c_int != 0) {
                break;
            }
            if *(*__ctype_b_loc())
                .offset(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            {
                break;
            }
        }
        if *cp as ::core::ffi::c_int == 0 {
            return;
        }
    }
    puts((*f).name);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_targets() {
    hash_map(&raw mut files, Some(print_target));
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file(item: *const ::core::ffi::c_void) {
    let f: *const file = item as *const file;
    let mut d: *const dep;
    if !(*f).name.is_null()
        && *(*f).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).name) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).name) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"name\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
        );
    }
    if !(*f).hname.is_null()
        && *(*f).hname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).hname) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).hname) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"hname\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).hname,
        );
    }
    if !(*f).vpath.is_null()
        && *(*f).vpath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).vpath) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).vpath) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"vpath\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).vpath,
        );
    }
    if !(*f).stem.is_null()
        && *(*f).stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).stem) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            (strlen((*f).name) as size_t)
                .wrapping_add(
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
                )
                .wrapping_add(strlen((*f).stem) as size_t),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).name,
            b"stem\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).stem,
        );
    }
    d = (*f).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0
            && !(*d).name.is_null()
            && *(*d).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strcache_iscached((*d).name) == 0
        {
            error(
                ::core::ptr::null::<Floc>(),
                (strlen((*d).name) as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                            .wrapping_sub(1),
                    )
                    .wrapping_add(strlen((*d).name) as size_t),
                b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*d).name,
                b"name\0" as *const u8 as *const ::core::ffi::c_char,
                (*d).name,
            );
        }
        if !(*d).stem.is_null()
            && *(*d).stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strcache_iscached((*d).stem) == 0
        {
            error(
                ::core::ptr::null::<Floc>(),
                (strlen((*d).name) as size_t)
                    .wrapping_add(
                        (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                            .wrapping_sub(1),
                    )
                    .wrapping_add(strlen((*d).stem) as size_t),
                b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*d).name,
                b"stem\0" as *const u8 as *const ::core::ffi::c_char,
                (*d).stem,
            );
        }
        d = (*d).next;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file_data_base() {
    hash_map(&raw mut files, Some(verify_file));
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn build_target_list(mut value: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut last_targ_count: ::core::ffi::c_ulong = 0;
    if files.ht_fill != last_targ_count {
        let mut max: size_t = (strlen(value) as size_t)
            .wrapping_div(500)
            .wrapping_add(1)
            .wrapping_mul(500);
        let mut len: size_t;
        let mut p: *mut ::core::ffi::c_char;
        value = xrealloc(value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
        p = value;
        len = 0;
        for slot in table_slots(&raw const files) {
            if is_real_item(*slot) {
                let f = *slot as *mut file;
                if (*f).is_target() == 0 {
                    continue;
                }
                let l: size_t = strlen((*f).name) as size_t;
                len = len.wrapping_add(l.wrapping_add(1));
                if len > max {
                    let off: size_t = p.offset_from(value) as ::core::ffi::c_long as size_t;
                    max = max.wrapping_add(
                        l.wrapping_add(1)
                            .wrapping_div(500)
                            .wrapping_add(1)
                            .wrapping_mul(500),
                    );
                    value = xrealloc(value as *mut ::core::ffi::c_void, max)
                        as *mut ::core::ffi::c_char;
                    p = value.offset(off as isize) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    (*f).name as *const ::core::ffi::c_void,
                    l as size_t,
                ) as *mut ::core::ffi::c_char;
                let fresh4 = p;
                p = p.offset(1 as ::core::ffi::c_int as isize);
                *fresh4 = ' ' as i32 as ::core::ffi::c_char;
            }
        }
        *p.offset(-(1 as ::core::ffi::c_int as isize)) = 0;
        last_targ_count = files.ht_fill;
    }
    value
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn init_hash_files() {
    hash_init(
        &raw mut files,
        1000 as ::core::ffi::c_ulong,
        Some(file_hash_1),
        Some(file_hash_2),
        Some(file_hash_cmp),
    );
}
pub const FILE_TIMESTAMP_HI_RES: ::core::ffi::c_int = 1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_main::initialize_stopchar_map;

    #[test]
    fn normalize_lookup_name_collapses_leading_dot_dirs() {
        unsafe {
            initialize_stopchar_map();

            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c"plain".as_ptr())).to_bytes(),
                b"plain"
            );
            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c"./".as_ptr())).to_bytes(),
                b"./"
            );
            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c".//".as_ptr())).to_bytes(),
                b"./"
            );
            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c"././src/file".as_ptr())).to_bytes(),
                b"src/file"
            );
        }
    }
}
