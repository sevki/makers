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
use std::sync::atomic::{AtomicBool, Ordering};
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
    cmd_prefix, db_level, export_all_variables, no_intermediates, run_silent, second_expansion,
    stopchar_map, verify_flag, MAP_DIRSEP,
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
/// Set once `snap_deps` has run, so the reader (`eval`) knows the global
/// dependency snapshot is in place. Stored in an atomic so its reads are plain
/// safe operations; all access is single-threaded, so `Relaxed` preserves the
/// original program order.
pub static SNAPPED_DEPS: AtomicBool = AtomicBool::new(false);

/// Whether `snap_deps` has run.
pub fn snapped_deps() -> bool {
    SNAPPED_DEPS.load(Ordering::Relaxed)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_hash_1(key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    let _key_ = ::core::ffi::CStr::from_ptr((*(key as *const file)).hname);
    _result_ = _result_.wrapping_add(jhash_string(_key_.to_bytes()) as ::core::ffi::c_ulong);
    _result_
}
/// Secondary hash for file keys; always zero, kept for the callback ABI.
/// The raw key pointer is accepted to match the signature but never inspected.
pub fn file_hash_2(mut _key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe fn file_hash_cmp(
    x: *const ::core::ffi::c_void,
    y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let xh = (x as *const file)
        .as_ref()
        .map_or(::core::ptr::null(), |xf| xf.hname);
    let yh = (y as *const file)
        .as_ref()
        .map_or(::core::ptr::null(), |yf| yf.hname);
    if xh == yh {
        0
    } else {
        strcmp(xh, yh)
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
/// Set once `.SECONDARY` is declared with no prerequisites, marking every
/// target as secondary. Stored in an atomic so its reads are plain safe
/// operations; all access is single-threaded, so `Relaxed` preserves the
/// original program order.
static ALL_SECONDARY: AtomicBool = AtomicBool::new(false);

fn all_secondary() -> bool {
    ALL_SECONDARY.load(Ordering::Relaxed)
}

fn stop_set_byte(c: u8, mask: ::core::ffi::c_int) -> bool {
    stopchar_map()[c as usize] as ::core::ffi::c_int & mask != 0
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
    if name.as_ref().is_some_and(|c| *c as ::core::ffi::c_int != 0) {
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
        (*f).last
            .as_mut()
            .expect("a double-colon chain head has a last entry")
            .prev = new;
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
    // Callers always pass a live file here; bind a checked reference so the
    // initial field accesses are null-safe without adding a branch.
    let from_ref = from_file
        .as_mut()
        .expect("rehash_file called with null from_file");
    from_ref.set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    file_key.hname = to_hname;
    if file_hash_cmp(
        from_file as *const ::core::ffi::c_void,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) == 0
    {
        return;
    }
    file_key.hname = from_ref.hname;
    // `from_file` is non-null here and each followed `renamed` link is itself
    // non-null, so it stays non-null; read the link through a checked
    // reference, keeping the walk a single branch.
    while !from_file
        .as_ref()
        .expect("rehash_file: null in renamed walk")
        .renamed
        .is_null()
    {
        from_file = from_file
            .as_ref()
            .expect("rehash_file: null in renamed walk")
            .renamed;
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
    // `from_file` walked only non-null `renamed` links above, so it is still
    // live here; bind a checked reference without adding a branch.
    let fr2 = from_file
        .as_mut()
        .expect("rehash_file: from_file became null");
    fr2.hname = to_hname;
    f = fr2.double_colon;
    while let Some(fr) = f.as_mut() {
        fr.hname = to_hname;
        f = fr.prev;
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
    if !fr2.cmds.is_null() {
        if (*to_file).cmds.is_null() {
            (*to_file).cmds = fr2.cmds;
        } else if fr2.cmds != (*to_file).cmds {
            let mut l: size_t = strlen(fr2.name) as size_t;
            let from_cmds = fr2
                .cmds
                .as_mut()
                .expect("from_file recipe is non-null in this branch");
            let to_cmds = (*to_file)
                .cmds
                .as_ref()
                .expect("to_file recipe is non-null in this branch");
            let from_floc = &raw mut from_cmds.fileinfo;
            if !to_cmds.fileinfo.filenm.is_null() {
                error(
                    from_floc,
                    l.wrapping_add(strlen(to_cmds.fileinfo.filenm) as size_t)
                        .wrapping_add(INTSTR_LENGTH),
                    b"recipe was specified for file '%s' at %s:%lu,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fr2.name,
                    from_cmds.fileinfo.filenm,
                    from_cmds.fileinfo.lineno,
                );
            } else {
                error(
                    from_floc,
                    l,
                    b"recipe for file '%s' was found by implicit rule search,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fr2.name,
                );
            }
            l = l.wrapping_add(strlen(to_hname) as size_t);
            error(
                from_floc,
                l,
                b"but '%s' is now considered the same file as '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                fr2.name,
                to_hname,
            );
            error(
                from_floc,
                l,
                b"recipe for '%s' will be ignored in favor of the one for '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                fr2.name,
                to_hname,
            );
        }
    }
    if (*to_file).deps.is_null() {
        (*to_file).deps = fr2.deps;
    } else {
        let mut deps: *mut dep = (*to_file).deps;
        while !(*deps).next.is_null() {
            deps = (*deps).next;
        }
        (*deps).next = fr2.deps;
    }
    merge_variable_set_lists(&raw mut (*to_file).variables, fr2.variables);
    if !(*to_file).double_colon.is_null()
        && fr2.is_target() as ::core::ffi::c_int != 0
        && fr2.double_colon.is_null()
    {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            (strlen(fr2.name) as size_t).wrapping_add(strlen(to_hname) as size_t),
            b"can't rename single-colon '%s' to double-colon '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            fr2.name,
            to_hname,
        );
    }
    if (*to_file).double_colon.is_null() && !fr2.double_colon.is_null() {
        if (*to_file).is_target() != 0 {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                (strlen(fr2.name) as size_t).wrapping_add(strlen(to_hname) as size_t),
                b"can't rename double-colon '%s' to single-colon '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                fr2.name,
                to_hname,
            );
        } else {
            (*to_file).double_colon = fr2.double_colon;
        }
    }
    if fr2.last_mtime > (*to_file).last_mtime {
        (*to_file).last_mtime = fr2.last_mtime;
    }
    (*to_file).mtime_before_update = fr2.mtime_before_update;
    (*to_file).set_precious(
        (*to_file).precious()
            | fr2.precious() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_loaded(
        (*to_file).loaded() | fr2.loaded() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_tried_implicit(
        (*to_file).tried_implicit()
            | fr2.tried_implicit() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_updating(
        (*to_file).updating()
            | fr2.updating() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_updated(
        (*to_file).updated() | fr2.updated() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_is_target(
        (*to_file).is_target()
            | fr2.is_target() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_cmd_target(
        (*to_file).cmd_target()
            | fr2.cmd_target() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_phony(
        (*to_file).phony() | fr2.phony() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_is_explicit(
        (*to_file).is_explicit()
            | fr2.is_explicit() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_secondary(
        (*to_file).secondary()
            | fr2.secondary() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_notintermediate(
        (*to_file).notintermediate()
            | fr2.notintermediate() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_ignore_vpath(
        (*to_file).ignore_vpath()
            | fr2.ignore_vpath() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_snapped(
        (*to_file).snapped() | fr2.snapped() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_suffix(
        (*to_file).suffix() | fr2.suffix() as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    (*to_file).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    fr2.renamed = to_file;
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
    while let Some(ff) = from_file.as_mut() {
        ff.name = ff.hname;
        from_file = ff.prev;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn remove_intermediates(sig: ::core::ffi::c_int) {
    let mut doneany: ::core::ffi::c_int = 0;
    if crate::make_main::opt_question() || crate::make_main::opt_touch() || all_secondary() || no_intermediates != 0 {
        return;
    }
    if sig != 0 && crate::make_main::opt_just_print() {
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
                    if crate::make_main::opt_just_print() {
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
    if p.as_ref().is_some_and(|c| *c != 0) {
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
        while let Some(dpr) = dp.as_mut() {
            let percent: *mut ::core::ffi::c_char;
            let nl: size_t = (strlen(dpr.name) as size_t).wrapping_add(1);
            alloca_allocations.push(::std::vec::from_elem(0, nl as usize));
            let nm: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                nm as *mut ::core::ffi::c_void,
                dpr.name as *const ::core::ffi::c_void,
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
                        // `dpr` is the null-checked reference to `dp`, and here
                        // `dp == deps`, so it is also the reference to `deps`.
                        deps = dpr.next;
                        dp = deps;
                    } else {
                        // `dl` is the previous list node; it was assigned from a
                        // non-null `dp` on a prior iteration, so it is non-null.
                        // Bind a checked reference (CodeQL-safe, no extra branch).
                        let dlr = dl.as_mut().expect("enter_prereqs: null prev dep");
                        dlr.next = dpr.next;
                        dp = dlr.next;
                    }
                    free_dep(df);
                    continue;
                } else {
                    dpr.name = strcache_add_len(
                        variable_buffer,
                        o.offset_from(variable_buffer) as ::core::ffi::c_long as size_t,
                    );
                }
            }
            dpr.stem = stem;
            dpr.set_staticpattern(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            dl = dp;
            dp = dpr.next;
        }
    }
    d1 = deps;
    while let Some(d1r) = d1.as_mut() {
        if !(d1r.need_2nd_expansion() != 0) {
            d1r.file = lookup_file(d1r.name);
            if d1r.file.is_null() {
                d1r.file = enter_file(d1r.name);
            }
            d1r.set_staticpattern(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            d1r.name = ::core::ptr::null::<::core::ffi::c_char>();
            if stem.is_null() {
                d1r.file
                    .as_mut()
                    .expect("dep file was just entered above")
                    .set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        d1 = d1r.next;
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
                        // Bridge to the safe `end_of_token`: it returns the
                        // offset of the first whitespace/NUL within `[cs, NUL)`,
                        // which we add back to `cs` to recover the C pointer.
                        let eot = cs.add(end_of_token(::core::slice::from_raw_parts(
                            cs as *const u8,
                            strlen(cs),
                        )));
                        cs = strchr(eot, '%' as i32);
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
                        (*d).file
                            .as_mut()
                            .expect("dep file was just entered above")
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
    while let Some(dr) = d.as_mut() {
        dr.file = lookup_file(dr.name);
        if dr.file.is_null() {
            dr.file = enter_file(dr.name);
        }
        dr.name = ::core::ptr::null::<::core::ffi::c_char>();
        dr.set_ignore_automatic_vars(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        d = dr.next;
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
    // `snap_file` is only ever called with a non-null file (its sole caller
    // `expand_deps` filters out null slots). Bind a checked reference so the
    // derefs below are null-safe without adding a control-flow branch.
    let fr = f.as_mut().expect("snap_file called with null file");
    if !second_expansion() {
        fr.set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if all_secondary() && fr.notintermediate() == 0 {
        fr.set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if no_intermediates != 0 && fr.intermediate() == 0 && fr.secondary() == 0 {
        fr.set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if let Some(file_vars) = fr.variables.as_ref() {
        prereqs = expand_extra_prereqs(lookup_variable_in_set(
            b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
            file_vars.set,
        ));
        if second_expansion() {
            d = prereqs;
            while let Some(dr) = d.as_mut() {
                if dr.name.is_null() {
                    dr.name = xstrdup(
                        dr.file
                            .as_ref()
                            .expect("a nameless prereq has a file")
                            .name,
                    );
                }
                dr.set_need_2nd_expansion(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                d = dr.next;
            }
        }
    } else if fr.is_target() != 0 {
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
            let fname = fr.name;
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
        } else if fr.deps.is_null() {
            fr.deps = prereqs;
        } else {
            // `fr.deps` is non-null in this arm and each `.next` we follow is
            // non-null until the last node; walk to the tail through checked
            // references (single branch) and append.
            d = fr.deps;
            while !d
                .as_ref()
                .expect("snap_file: null in deps walk")
                .next
                .is_null()
            {
                d = d.as_ref().expect("snap_file: null in deps walk").next;
            }
            d.as_mut().expect("snap_file: null deps tail").next = prereqs;
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
    SNAPPED_DEPS.store(true, Ordering::Relaxed);
    f = lookup_file(b".PRECIOUS\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                f2r.set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(b".LOW_RESOLUTION_TIME\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                f2r.set_low_resolution_time(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(b".PHONY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                f2r.set_phony(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2r.set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2r.last_mtime = NONEXISTENT_MTIME as uintmax_t;
                f2r.mtime_before_update = NONEXISTENT_MTIME as uintmax_t;
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(b".NOTINTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        if !fr.deps.is_null() {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    f2r.set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        } else {
            no_intermediates = 1;
        }
        f = fr.prev;
    }
    f = lookup_file(b".INTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                if f2r.notintermediate() != 0 {
                    fatal(
                        ::core::ptr::null_mut::<Floc>(),
                        strlen(f2r.name) as size_t,
                        b"%s cannot be both .NOTINTERMEDIATE and .INTERMEDIATE\0" as *const u8
                            as *const ::core::ffi::c_char,
                        f2r.name,
                    );
                } else {
                    f2r.set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(b".SECONDARY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        if !fr.deps.is_null() {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    if f2r.notintermediate() != 0 {
                        fatal(
                            ::core::ptr::null_mut::<Floc>(),
                            strlen(f2r.name) as size_t,
                            b"%s cannot be both .NOTINTERMEDIATE and .SECONDARY\0" as *const u8
                                as *const ::core::ffi::c_char,
                            f2r.name,
                        );
                    } else {
                        let rhs = {
                            f2r.set_secondary(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            f2r.secondary()
                        } as ::core::ffi::c_uint;
                        f2r.set_intermediate(rhs);
                    }
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        } else {
            ALL_SECONDARY.store(true, Ordering::Relaxed);
        }
        f = fr.prev;
    }
    if no_intermediates != 0 && all_secondary() {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            0,
            b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    f = lookup_file(b".EXPORT_ALL_VARIABLES\0" as *const u8 as *const ::core::ffi::c_char);
    if f.as_ref()
        .is_some_and(|fr| fr.is_target() as ::core::ffi::c_int != 0)
    {
        export_all_variables = 1;
    }
    f = lookup_file(b".IGNORE\0" as *const u8 as *const ::core::ffi::c_char);
    if let Some(fr) = f
        .as_ref()
        .filter(|fr| fr.is_target() as ::core::ffi::c_int != 0)
    {
        if fr.deps.is_null() {
            crate::make_main::set_ignore_errors_mirror(true);
        } else {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    f2r.command_flags |= COMMANDS_NOERROR;
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        }
    }
    f = lookup_file(b".SILENT\0" as *const u8 as *const ::core::ffi::c_char);
    if let Some(fr) = f
        .as_ref()
        .filter(|fr| fr.is_target() as ::core::ffi::c_int != 0)
    {
        if fr.deps.is_null() {
            run_silent = 1;
        } else {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    f2r.command_flags |= COMMANDS_SILENT;
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        }
    }
    f = lookup_file(b".NOTPARALLEL\0" as *const u8 as *const ::core::ffi::c_char);
    if let Some(fr) = f
        .as_ref()
        .filter(|fr| fr.is_target() as ::core::ffi::c_int != 0)
    {
        let mut d2: *mut dep;
        if fr.deps.is_null() {
            crate::make_main::NOT_PARALLEL.store(true, ::std::sync::atomic::Ordering::Relaxed);
        } else {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    if !f2r.deps.is_null() {
                        d2 = f2r.deps.as_ref().expect("checked non-null above").next;
                        while let Some(d2r) = d2.as_mut() {
                            d2r.set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            d2 = d2r.next;
                        }
                    }
                    f2 = f2r.prev;
                }
                d = dr.next;
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
    while let Some(&fp) = filep.as_ref().filter(|p| !p.is_null()) {
        snap_file(fp as *mut file, prereqs);
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
        let dfile = (*d)
            .file
            .as_mut()
            .expect("an also_make dep always has a file");
        if state as ::core::ffi::c_uint > dfile.command_state() as ::core::ffi::c_uint {
            dfile.set_command_state(state as cmd_state as cmd_state);
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
                    (*deps)
                        .file
                        .as_ref()
                        .expect("a nameless dep has a file")
                        .name
                },
            );
        } else if ood.is_null() {
            ood = deps;
        }
        deps = (*deps).next;
    }
    if let Some(oodr) = ood.as_ref() {
        printf(
            b" | %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            if oodr.wait_here() as ::core::ffi::c_int != 0 {
                b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !oodr.name.is_null() {
                oodr.name
            } else {
                oodr
                    .file
                    .as_ref()
                    .expect("a nameless dep has a file")
                    .name
            },
        );
        ood = oodr.next;
        while let Some(oodn) = ood.as_ref() {
            if oodn.ignore_mtime() != 0 {
                printf(
                    b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    if oodn.wait_here() as ::core::ffi::c_int != 0 {
                        b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    if !oodn.name.is_null() {
                        oodn.name
                    } else {
                        oodn.file
                            .as_ref()
                            .expect("a nameless dep has a file")
                            .name
                    },
                );
            }
            ood = oodn.next;
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
    if crate::make_main::opt_no_builtin_rules() && (*f).builtin() as ::core::ffi::c_int != 0 {
        return;
    }
    putchar('\n' as i32);
    if (*f)
        .cmds
        .as_ref()
        .is_some_and(|c| c.recipe_prefix as ::core::ffi::c_int != cmd_prefix as ::core::ffi::c_int)
    {
        fputs(
            b".RECIPEPREFIX = \0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        cmd_prefix = (*f)
            .cmds
            .as_ref()
            .expect("cmds is non-null when its recipe_prefix differs")
            .recipe_prefix;
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
                    (*d).file.as_ref().expect("a nameless dep has a file").name
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
                if crate::make_main::opt_question() {
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
    // Skip built-in special targets, whose names are a dot followed by one
    // or more all-uppercase letters (e.g. `.SUFFIXES`, `.PHONY`).
    let name = ::core::ffi::CStr::from_ptr((*f).name).to_bytes();
    if name.len() >= 2 && name[0] == b'.' && name[1..].iter().all(u8::is_ascii_uppercase) {
        return;
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
    use crate::implicit::alloc_dep;
    use crate::make_main::initialize_stopchar_map;
    use crate::strcache::strcache_add;

    /// `snapped_deps()` reflects the `SNAPPED_DEPS` atomic: false before
    /// `snap_deps` runs, true after. Restores the prior value so the global
    /// stays isolated from other tests.
    #[test]
    fn snapped_deps_tracks_atomic() {
        let saved = SNAPPED_DEPS.load(Ordering::Relaxed);

        SNAPPED_DEPS.store(false, Ordering::Relaxed);
        assert!(!snapped_deps(), "not yet snapped");

        SNAPPED_DEPS.store(true, Ordering::Relaxed);
        assert!(snapped_deps(), "snapped");

        SNAPPED_DEPS.store(saved, Ordering::Relaxed);
    }

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

    /// `all_secondary()` reflects the `ALL_SECONDARY` flag: false while unset
    /// (the default), true once stored. Restores the prior value so it stays
    /// isolated from other tests.
    #[test]
    fn all_secondary_tracks_flag() {
        let saved = ALL_SECONDARY.load(Ordering::Relaxed);

        ALL_SECONDARY.store(false, Ordering::Relaxed);
        assert!(!all_secondary(), "zero is unset");

        ALL_SECONDARY.store(true, Ordering::Relaxed);
        assert!(all_secondary(), "non-zero is set");

        ALL_SECONDARY.store(saved, Ordering::Relaxed);
    }

    // Serialize the tests that touch the process-wide `files` hash table and
    // the file-graph globals so they never race each other.
    static FILE_GRAPH_LOCK: Mutex<()> = Mutex::new(());

    /// `snap_file` on a plain (non-target, variable-less) file just clears the
    /// `updating` flag and returns, since `all_secondary`/`no_intermediates`
    /// are unset by default. Drives that branch on a stack file.
    #[test]
    fn snap_file_plain_target_clears_updating() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let mut f = File::default();
            f.set_updating(1);
            snap_file(&raw mut f, ::core::ptr::null());
            assert_eq!(f.updating(), 0, "updating cleared when not 2nd-expanding");
        }
    }

    /// For a target file with no per-target variables, `snap_file` copies the
    /// `.EXTRA_PREREQS` dep chain (here a single prereq whose name matches the
    /// target, so the self-match break path runs and the copy is freed).
    #[test]
    fn snap_file_target_copies_extra_prereqs() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            let name = strcache_add(c"snapself".as_ptr());
            let mut f = File::default();
            f.name = name;
            f.hname = name;
            f.set_is_target(1);

            // A one-element prereq chain whose dep name equals the target name.
            let d = alloc_dep();
            (*d).name = name;
            (*d).next = ::core::ptr::null_mut();
            snap_file(&raw mut f, d as *const Dep);
            // The self-referential prereq is dropped, so deps stays empty.
            assert!(f.deps.is_null(), "self-prereq is not appended");
            free_dep(d);
        }
    }

    /// `enter_prereqs(deps, NULL)` resolves each prerequisite to a file via
    /// `enter_file`, nulls the dep name, and (with a null stem) marks the
    /// entered file explicit. Drives the common no-pattern path.
    #[test]
    fn enter_prereqs_resolves_files_for_plain_deps() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            init_hash_files();

            let nm = strcache_add(c"enter_prereqs_probe_target".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            let head = enter_prereqs(d, ::core::ptr::null());
            assert_eq!(head, d, "the chain head is returned unchanged");
            // Name is consumed (replaced by the resolved file) and a file exists.
            assert!((*head).name.is_null(), "resolved dep name is cleared");
            assert!(!(*head).file.is_null(), "prereq resolved to a file");
            assert!(
                !lookup_file(nm).is_null(),
                "the prerequisite file is now in the table"
            );
        }
    }

    /// `enter_prereqs(NULL, _)` is a no-op returning null.
    #[test]
    fn enter_prereqs_null_is_noop() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            assert!(enter_prereqs(::core::ptr::null_mut(), ::core::ptr::null()).is_null());
        }
    }

    /// With a non-null stem, `enter_prereqs` walks the static-pattern block. A
    /// prerequisite name with no `%` finds no percent, so it keeps its name but
    /// is tagged with the stem and `staticpattern`, then resolved to a file.
    /// This exercises the stem branch without touching the variable buffer.
    #[test]
    fn enter_prereqs_static_pattern_without_percent() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            init_hash_files();

            let nm = strcache_add(c"enter_prereqs_static_probe".as_ptr());
            let stem = strcache_add(c"thestem".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            let head = enter_prereqs(d, stem);
            assert_eq!(head, d);
            // The dep was tagged with the stem (staticpattern path ran) and then
            // resolved: name cleared, file entered, staticpattern reset to 0.
            assert_eq!((*head).stem, stem, "stem recorded on the static pattern");
            assert!((*head).name.is_null(), "resolved dep name is cleared");
            assert!(!(*head).file.is_null(), "prereq resolved to a file");
            assert_eq!(
                (*head).staticpattern(),
                0,
                "staticpattern is reset after resolution"
            );
        }
    }

    /// The pattern-substitution arm of the stem branch: a prerequisite name
    /// containing `%` is expanded against the stem via the variable buffer
    /// (e.g. `%.o` with stem `epp_stem` -> `epp_stem.o`). Holds both the
    /// file-graph and variable-buffer locks (only this test needs both).
    #[test]
    fn enter_prereqs_static_pattern_substitutes_percent() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _b = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            init_hash_files();
            crate::expand::initialize_variable_output();

            let nm = strcache_add(c"%.o".as_ptr());
            let stem = strcache_add(c"epp_stem".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            let head = enter_prereqs(d, stem);
            assert_eq!(head, d);
            // `%` expanded to the stem and the dep resolved to a file named
            // "epp_stem.o"; the dep name itself is cleared after resolution.
            assert!((*head).name.is_null(), "resolved dep name is cleared");
            assert!(
                !lookup_file(strcache_add(c"epp_stem.o".as_ptr())).is_null(),
                "the expanded prerequisite file was entered"
            );
        }
    }

    /// When a `%` prerequisite expands to the empty string (a bare `%` with an
    /// empty stem: the percent is dropped and nothing remains), `enter_prereqs`
    /// removes that prerequisite from the chain and frees it. With a single
    /// such dep the chain collapses to empty.
    #[test]
    fn enter_prereqs_drops_prereq_that_expands_empty() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _b = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            init_hash_files();
            crate::expand::initialize_variable_output();

            let nm = strcache_add(c"%".as_ptr());
            let stem = strcache_add(c"".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            // The bare `%` with an empty stem expands to "", so the dep is
            // dropped and freed; the returned chain is empty.
            let head = enter_prereqs(d, stem);
            assert!(head.is_null(), "the empty-expanding prereq is removed");
        }
    }
}
