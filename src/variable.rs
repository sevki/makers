pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{file, Commands, Dep, File, VariableSet, VariableSetList};
use crate::misc::{next_token, xcalloc, xmalloc, xrealloc, xstrdup, xstrndup};
use crate::stdio::FILE;
use c2rust_bitfields;
use libc::{abort, free, printf, putchar, puts, sprintf, strchr, strcmp, strcpy, strstr};
extern "C" {
    static mut stdout: *mut FILE;
    fn putc(__c: i32, __stream: *mut FILE) -> i32;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> i32;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> i32;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> i32;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
use crate::warning::{self, Action, Type};
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;
pub type dep = Dep;
pub type commands = Commands;
use crate::expand::{
    allocated_expand_string_for_file, allocated_expand_variable, expanding_var,
    install_variable_buffer, recursively_expand_for_file, swap_variable_buffer, variable_buffer,
};
use crate::floc::Floc;
use crate::function::func_shell_base;
use crate::hash::{
    hash_delete_at, hash_deleted_item, hash_find_item, hash_find_slot, hash_free, hash_init,
    hash_insert_at, hash_map, hash_map_arg, hash_print_stats, jhash,
};
use crate::job::default_shell;
use crate::make_main::{shell_var, stopchar_map};
use crate::misc::concat;
use crate::output::fatal;
use crate::output::msg;
use crate::posixos::jobserver_get_invalid_auth;
use crate::read::reading_file;
use crate::remote_stub::remote_description;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_var {
    pub next: *mut pattern_var,
    pub suffix: *const ::core::ffi::c_char,
    pub target: *const ::core::ffi::c_char,
    pub len: size_t,
    pub variable: variable,
}
pub type hash_map_arg_func_t = crate::hash::hash_map_arg_func_t;
pub type hash_map_func_t = crate::hash::hash_map_func_t;
pub type variable_scope = ::core::ffi::c_uint;
pub const s_pattern: variable_scope = 2;
pub const s_target: variable_scope = 1;
pub const s_global: variable_scope = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defined_vars {
    pub name: *const ::core::ffi::c_char,
    pub len: size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const MAKELEVEL_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"MAKELEVEL\0") };
pub const RECIPEPREFIX_DEFAULT: i32 = '\t' as i32;
/// Depth of the in-progress environment-variable expansion (used to detect
/// self-referential recursion). Stored in an atomic so its reads are plain
/// safe operations; all access is single-threaded, so `Relaxed` preserves the
/// original program order.
pub static ENV_RECURSION: ::std::sync::atomic::AtomicU64 = ::std::sync::atomic::AtomicU64::new(0);

/// Current environment-variable expansion recursion depth.
pub fn env_recursion() -> u64 {
    ENV_RECURSION.load(::std::sync::atomic::Ordering::Relaxed)
}
/// Monotonic counter bumped whenever the global variable set changes; used to
/// invalidate the cached `.VARIABLES` value in `lookup_special_var`. Atomic so
/// its reads/writes are plain safe ops; variable mutation is single-threaded,
/// so `Relaxed` preserves the original program order.
static VARIABLE_CHANGENUM: ::std::sync::atomic::AtomicU64 = ::std::sync::atomic::AtomicU64::new(0);

/// Reads the change counter masked to the C `unsigned long` width. The original
/// counter is a `c_ulong`, which is 32-bit on some targets (Windows, 32-bit
/// Unix); masking keeps the wraparound point — and therefore the `.VARIABLES`
/// invalidation behavior — identical to the C oracle on every target. The mask
/// is `u64::MAX` (a no-op) where `c_ulong` is already 64-bit.
fn variable_changenum() -> u64 {
    // No-op (`u64::MAX`) on 64-bit `c_ulong`; the low 32 bits where `c_ulong`
    // is 32-bit. Computed via a shift to avoid a cast that clippy flags as
    // redundant on 64-bit targets.
    let mask = u64::MAX >> (u64::BITS - ::core::ffi::c_ulong::BITS);
    VARIABLE_CHANGENUM.load(::std::sync::atomic::Ordering::Relaxed) & mask
}
static mut pattern_vars: *mut pattern_var = ::core::ptr::null::<pattern_var>() as *mut pattern_var;
static mut last_pattern_vars: [*mut pattern_var; 256] =
    [::core::ptr::null::<pattern_var>() as *mut pattern_var; 256];
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn create_pattern_var(
    target: *const ::core::ffi::c_char,
    suffix: *const ::core::ffi::c_char,
) -> *mut pattern_var {
    let len: size_t = strlen(target) as size_t;
    let p: *mut pattern_var =
        xcalloc(::core::mem::size_of::<pattern_var>() as size_t) as *mut pattern_var;
    if !pattern_vars.is_null() {
        if len < 256 && !last_pattern_vars[len as usize].is_null() {
            (*p).next = (*last_pattern_vars[len as usize]).next;
            (*last_pattern_vars[len as usize]).next = p;
        } else {
            let mut v: *mut *mut pattern_var;
            v = &raw mut pattern_vars;
            loop {
                if (*v).is_null() || (**v).len > len {
                    (*p).next = *v;
                    *v = p;
                    break;
                } else {
                    v = &raw mut (**v).next;
                }
            }
        }
    } else {
        pattern_vars = p;
        (*p).next = ::core::ptr::null_mut::<pattern_var>();
    }
    (*p).target = target;
    (*p).len = len;
    (*p).suffix = suffix.offset(1_i32 as isize);
    if len < 256 {
        last_pattern_vars[len as usize] = p;
    }
    p
}
unsafe extern "C" fn lookup_pattern_var(
    start: *mut pattern_var,
    target: *const ::core::ffi::c_char,
    targlen: size_t,
) -> *mut pattern_var {
    let mut p: *mut pattern_var;
    p = if !start.is_null() {
        (*start).next
    } else {
        pattern_vars
    };
    while !p.is_null() {
        let stem: *const ::core::ffi::c_char;
        let stemlen: size_t;
        if !((*p).len > targlen) {
            stem = target
                .offset(((*p).suffix.offset_from((*p).target) as ::core::ffi::c_long - 1) as isize);
            stemlen = targlen.wrapping_sub((*p).len).wrapping_add(1);
            if !(stem > target
                && !(strncmp(
                    (*p).target,
                    target,
                    stem.offset_from(target) as ::core::ffi::c_long as size_t,
                ) == 0))
                && *(*p).suffix as i32 == *stem.offset(stemlen as isize) as i32
                && (*(*p).suffix as i32 == 0
                    || *(*p).suffix.offset(1_i32 as isize) as i32
                        == *stem.offset(stemlen.wrapping_add(1) as isize) as i32
                        && (*(*p).suffix.offset(1_i32 as isize) as i32 == 0
                            || strcmp(
                                ((*p).suffix.offset(1_i32 as isize) as *const ::core::ffi::c_char)
                                    .offset(1_i32 as isize),
                                (stem.offset(stemlen.wrapping_add(1) as isize)
                                    as *const ::core::ffi::c_char)
                                    .offset(1_i32 as isize),
                            ) == 0))
            {
                break;
            }
        }
        p = (*p).next;
    }
    p
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn variable_hash_1(keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let key: *const variable = keyv as *const variable;
    let mut _result_: ::core::ffi::c_ulong = 0;
    let _key_: *const ::core::ffi::c_uchar = (*key).name as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash(::core::slice::from_raw_parts(
        _key_,
        (*key).length as usize,
    )) as ::core::ffi::c_ulong);
    _result_
}
/// Secondary hash for [`variable`] keys; always zero, kept for the callback
/// ABI. The raw key pointer is accepted to match the signature but never
/// inspected.
pub fn variable_hash_2(keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _key: *const variable = keyv as *const variable;
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe fn variable_hash_cmp(xv: *const ::core::ffi::c_void, yv: *const ::core::ffi::c_void) -> i32 {
    let x: *const variable = xv as *const variable;
    let y: *const variable = yv as *const variable;
    let result: i32 = (*x).length.wrapping_sub((*y).length) as i32;
    if result != 0 {
        return result;
    }
    if (*x).name == (*y).name {
        0
    } else {
        memcmp(
            (*x).name as *const ::core::ffi::c_void,
            (*y).name as *const ::core::ffi::c_void,
            (*x).length as size_t,
        )
    }
}
pub const VARIABLE_BUCKETS: i32 = 523;
pub const PERFILE_VARIABLE_BUCKETS: i32 = 23;
pub const SMALL_SCOPE_VARIABLE_BUCKETS: i32 = 13;
static mut global_variable_set: variable_set = variable_set {
    table: hash_table {
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
    },
};
static mut global_setlist: variable_set_list = variable_set_list {
    next: ::core::ptr::null::<variable_set_list>() as *mut variable_set_list,
    set: &raw const global_variable_set as *mut variable_set,
    next_is_parent: 0,
};
pub static mut current_variable_set_list: *mut variable_set_list =
    &raw const global_setlist as *mut variable_set_list;
/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_BLANK: i32 = 0x2;
const MAP_NEWLINE: i32 = 0x4;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
}

/// Emit the "invalid/undefined variable" diagnostic shared by the three
/// `check_*`/`warn_undefined` warnings. Fully safe: the message is built with
/// `format!` and routed through the safe [`msg`] wrappers (no `format`/
/// `xstrdup`/`free`/`fatal`/`error` FFI). When the warning action is `Error`
/// it fatals (which never returns); otherwise it emits a `warning:` line.
///
/// `kind` is the leading text (e.g. "invalid variable name") and `name` is the
/// offending name bytes — together they reproduce the C `"... '%.*s'"` text.
fn emit_var_name_warning(
    ctx: &crate::execctx::ExecContext,
    loc: Option<&Floc>,
    is_error: bool,
    kind: &str,
    name: &[u8],
) {
    let body = format!("{kind} '{}'", String::from_utf8_lossy(name));
    if is_error {
        msg::fatal(ctx, loc, &body);
    }
    msg::error(ctx, loc, &format!("warning: {body}"));
}

unsafe fn check_valid_name(
    ctx: &crate::execctx::ExecContext,
    flocp: *const Floc,
    name: *const ::core::ffi::c_char,
    length: size_t,
) {
    if !(warning::is_active(Type::InvalidVar)) {
        return;
    }
    // The name is valid unless it contains an unquoted blank or newline.
    let name_bytes = ::core::slice::from_raw_parts(name as *const u8, length);
    if !name_bytes
        .iter()
        .any(|&c| stop_set(c, MAP_BLANK | MAP_NEWLINE))
    {
        return;
    }
    if warning::is_active(Type::InvalidVar) {
        emit_var_name_warning(
            ctx,
            flocp.as_ref(),
            warning::action(Type::InvalidVar) == Action::Error,
            "invalid variable name",
            name_bytes,
        );
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn init_hash_global_variable_set() {
    hash_init(
        &raw mut global_variable_set.table,
        VARIABLE_BUCKETS as ::core::ffi::c_ulong,
        Some(variable_hash_1),
        Some(variable_hash_2),
        Some(variable_hash_cmp),
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[allow(clippy::too_many_arguments)]
pub unsafe fn define_variable_in_set(
    ctx: &crate::execctx::ExecContext,
    mut name: *const ::core::ffi::c_char,
    length: size_t,
    value: *const ::core::ffi::c_char,
    mut origin: variable_origin,
    recursive: i32,
    set: *mut variable_set,
    flocp: *const Floc,
) -> *mut variable {
    let mut v: *mut variable;
    let var_slot: *mut *mut variable;
    let mut var_key: variable = variable {
        name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        value: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        fileinfo: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    check_valid_name(ctx, flocp, name, length);
    // Route SET through a checked reference; null means the global set.
    let set = if set.is_null() {
        &raw mut global_variable_set
    } else {
        set
    };
    let set: &mut variable_set = &mut *set;
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    var_slot = hash_find_slot(
        &raw mut set.table,
        &raw mut var_key as *const ::core::ffi::c_void,
    ) as *mut *mut variable;
    v = *var_slot;
    if crate::make_main::env_overrides()
        && origin as ::core::ffi::c_uint == o_env as i32 as ::core::ffi::c_uint
    {
        origin = o_env_override;
    }
    if !(v.is_null()
        || v as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
    {
        if crate::make_main::env_overrides() && (*v).origin() as i32 == o_env as i32 {
            (*v).set_origin(o_env_override as variable_origin);
        }
        if origin as i32 >= (*v).origin() as i32 {
            free((*v).value as *mut ::core::ffi::c_void);
            (*v).value = xstrdup(value);
            if !flocp.is_null() {
                (*v).fileinfo = *flocp;
            } else {
                (*v).fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
            }
            (*v).set_origin(origin as variable_origin);
            (*v).set_recursive(recursive as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        return v;
    }
    v = xcalloc(::core::mem::size_of::<variable>() as size_t) as *mut variable;
    (*v).name = xstrndup(name, length);
    (*v).length = length as ::core::ffi::c_uint;
    hash_insert_at(
        &raw mut (*set).table,
        v as *const ::core::ffi::c_void,
        var_slot as *const ::core::ffi::c_void,
    );
    if ::core::ptr::eq(&raw const *set, &raw const global_variable_set) {
        VARIABLE_CHANGENUM.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
    }
    (*v).value = xstrdup(value);
    if !flocp.is_null() {
        (*v).fileinfo = *flocp;
    }
    (*v).set_origin(origin as variable_origin);
    (*v).set_recursive(recursive as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*v).set_export(v_default as variable_export);
    (*v).set_exportable(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    name = (*v).name;
    if *name as i32 != '_' as i32
        && ((*name as i32) < 'A' as i32 || *name as i32 > 'Z' as i32)
        && ((*name as i32) < 'a' as i32 || *name as i32 > 'z' as i32)
    {
        (*v).set_exportable(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        name = name.offset(1_i32 as isize);
        while *name as i32 != 0 {
            if *name as i32 != '_' as i32
                && ((*name as i32) < 'a' as i32 || *name as i32 > 'z' as i32)
                && ((*name as i32) < 'A' as i32 || *name as i32 > 'Z' as i32)
                && !((*name as ::core::ffi::c_uint).wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                    <= 9)
            {
                break;
            }
            name = name.offset(1_i32 as isize);
        }
        if *name as i32 != 0 {
            (*v).set_exportable(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    v
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_variable_name_and_value(item: *const ::core::ffi::c_void) {
    let v: *mut variable = item as *mut variable;
    free((*v).name as *mut ::core::ffi::c_void);
    free((*v).value as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_variable_set(list: *mut variable_set_list) {
    hash_map(
        &raw mut (*(*list).set).table,
        Some(free_variable_name_and_value),
    );
    hash_free(&raw mut (*(*list).set).table, 1);
    free((*list).set as *mut ::core::ffi::c_void);
    free(list as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn undefine_variable_in_set(
    ctx: &crate::execctx::ExecContext,
    flocp: *const Floc,
    name: *const ::core::ffi::c_char,
    length: size_t,
    mut origin: variable_origin,
    set: *mut variable_set,
) {
    let v: *mut variable;
    let var_slot: *mut *mut variable;
    let mut var_key: variable = variable {
        name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        value: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        fileinfo: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    check_valid_name(ctx, flocp, name, length);
    // Route SET through a checked reference; null means the global set.
    let set = if set.is_null() {
        &raw mut global_variable_set
    } else {
        set
    };
    let set: &mut variable_set = &mut *set;
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    var_slot = hash_find_slot(
        &raw mut set.table,
        &raw mut var_key as *const ::core::ffi::c_void,
    ) as *mut *mut variable;
    if crate::make_main::env_overrides()
        && origin as ::core::ffi::c_uint == o_env as i32 as ::core::ffi::c_uint
    {
        origin = o_env_override;
    }
    v = *var_slot;
    if !(v.is_null()
        || v as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
    {
        if crate::make_main::env_overrides() && (*v).origin() as i32 == o_env as i32 {
            (*v).set_origin(o_env_override as variable_origin);
        }
        if origin as i32 >= (*v).origin() as i32 {
            hash_delete_at(&raw mut set.table, var_slot as *const ::core::ffi::c_void);
            free_variable_name_and_value(v as *const ::core::ffi::c_void);
            free(v as *mut ::core::ffi::c_void);
            if ::core::ptr::eq(&raw const *set, &raw const global_variable_set) {
                VARIABLE_CHANGENUM.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
            }
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn lookup_special_var(var: *mut variable) -> *mut variable {
    // Memoizes the variable-set change number at which `.VARIABLES` was last
    // rebuilt. Function-local atomic so the read/write are plain safe ops;
    // access is single-threaded, so `Relaxed` preserves the original order.
    static LAST_CHANGENUM: ::std::sync::atomic::AtomicU64 = ::std::sync::atomic::AtomicU64::new(0);
    if variable_changenum() != LAST_CHANGENUM.load(::std::sync::atomic::Ordering::Relaxed)
        && (*(*var).name as i32
            == *(b".VARIABLES\0" as *const u8 as *const ::core::ffi::c_char) as i32
            && (*(*var).name as i32 == 0
                || strcmp(
                    (*var).name.offset(1_i32 as isize),
                    (b".VARIABLES\0" as *const u8 as *const ::core::ffi::c_char)
                        .offset(1_i32 as isize),
                ) == 0))
    {
        let mut max: size_t = (strlen((*var).value) as size_t)
            .wrapping_div(500)
            .wrapping_add(1)
            .wrapping_mul(500);
        let mut len: size_t;
        let mut p: *mut ::core::ffi::c_char;
        let mut vp: *mut *mut variable = global_variable_set.table.ht_vec as *mut *mut variable;
        let end: *mut *mut variable =
            vp.offset(global_variable_set.table.ht_size as isize) as *mut *mut variable;
        (*var).value =
            xrealloc((*var).value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
        p = (*var).value;
        len = 0;
        while vp < end {
            if !((*vp).is_null()
                || *vp as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
            {
                let v: *mut variable = *vp;
                let l: i32 = (*v).length as i32;
                len = len.wrapping_add((l + 1) as size_t);
                if len > max {
                    let off: size_t = p.offset_from((*var).value) as ::core::ffi::c_long as size_t;
                    max = max.wrapping_add((((l + 1) / 500 + 1) * 500) as size_t);
                    (*var).value = xrealloc((*var).value as *mut ::core::ffi::c_void, max)
                        as *mut ::core::ffi::c_char;
                    p = (*var).value.offset(off as isize) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    (*v).name as *const ::core::ffi::c_void,
                    l as size_t,
                ) as *mut ::core::ffi::c_char;
                let fresh4 = p;
                p = p.offset(1_i32 as isize);
                *fresh4 = ' ' as i32 as ::core::ffi::c_char;
            }
            vp = vp.offset(1_i32 as isize);
        }
        *p.offset(-(1_i32 as isize)) = 0;
        LAST_CHANGENUM.store(variable_changenum(), ::std::sync::atomic::Ordering::Relaxed);
    }
    var
}
unsafe fn check_variable_reference(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
) {
    if !(warning::is_active(Type::InvalidRef)) {
        return;
    }
    // The reference is valid unless it contains an unquoted blank or newline.
    let name_bytes = ::core::slice::from_raw_parts(name as *const u8, length);
    if !name_bytes
        .iter()
        .any(|&c| stop_set(c, MAP_BLANK | MAP_NEWLINE))
    {
        return;
    }
    if warning::is_active(Type::InvalidRef) {
        emit_var_name_warning(
            ctx,
            (*expanding_var).as_ref(),
            warning::action(Type::InvalidRef) == Action::Error,
            "invalid variable reference",
            name_bytes,
        );
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn lookup_variable(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut variable {
    let mut setlist: *const variable_set_list;
    let mut var_key: variable = variable {
        name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        value: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        fileinfo: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut is_parent: i32 = 0;
    check_variable_reference(ctx, name, length);
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    setlist = current_variable_set_list;
    while !setlist.is_null() {
        let set: *const variable_set = (*setlist).set;
        let v: *mut variable;
        v = hash_find_item(
            &raw const (*set).table as *mut hash_table,
            &raw mut var_key as *const ::core::ffi::c_void,
        ) as *mut variable;
        if !v.is_null() && (is_parent == 0 || (*v).private_var() == 0) {
            return if (*v).special() as i32 != 0 {
                lookup_special_var(v)
            } else {
                v
            };
        }
        is_parent |= (*setlist).next_is_parent;
        setlist = (*setlist).next;
    }
    ::core::ptr::null_mut::<variable>()
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn lookup_variable_for_file(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
    file: *mut File,
) -> *mut variable {
    let var: *mut variable;
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    if file.is_null() {
        return lookup_variable(ctx, name, length);
    }
    install_file_context(file, &raw mut savev, ::core::ptr::null_mut::<*const Floc>());
    var = lookup_variable(ctx, name, length);
    restore_file_context(savev, ::core::ptr::null::<Floc>());
    var
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn lookup_variable_in_set(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
    set: *const variable_set,
) -> *mut variable {
    let mut var_key: variable = variable {
        name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        value: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        fileinfo: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    check_variable_reference(ctx, name, length);
    var_key.name = name as *mut ::core::ffi::c_char;
    var_key.length = length as ::core::ffi::c_uint;
    let Some(setr) = set.as_ref() else {
        return ::core::ptr::null_mut::<variable>();
    };
    hash_find_item(
        &raw const setr.table as *mut hash_table,
        &raw mut var_key as *const ::core::ffi::c_void,
    ) as *mut variable
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn initialize_file_variables(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    reading: i32,
) {
    let mut l: *mut variable_set_list = (*file).variables;
    if l.is_null() {
        l = xmalloc(::core::mem::size_of::<variable_set_list>() as size_t)
            as *mut variable_set_list;
        (*l).set = xmalloc(::core::mem::size_of::<variable_set>() as size_t) as *mut variable_set;
        hash_init(
            &raw mut (*(*l).set).table,
            PERFILE_VARIABLE_BUCKETS as ::core::ffi::c_ulong,
            Some(variable_hash_1),
            Some(variable_hash_2),
            Some(variable_hash_cmp),
        );
        (*file).variables = l;
    }
    if !(*file).double_colon.is_null() && (*file).double_colon != file {
        initialize_file_variables(ctx, (*file).double_colon, reading);
        (*l).next = (*(*file).double_colon).variables;
        (*l).next_is_parent = 0;
        return;
    }
    if (*file).parent.is_null() {
        (*l).next = &raw mut global_setlist;
    } else {
        initialize_file_variables(ctx, (*file).parent, reading);
        (*l).next = (*(*file).parent).variables;
    }
    (*l).next_is_parent = 1;
    if reading == 0 && !(*file).pat_searched {
        let mut p: *mut pattern_var;
        let targlen: size_t = strlen((*file).name) as size_t;
        p = lookup_pattern_var(
            ::core::ptr::null_mut::<pattern_var>(),
            (*file).name,
            targlen,
        );
        if !p.is_null() {
            let global: *mut variable_set_list = current_variable_set_list;
            (*file).pat_variables = create_new_variable_set();
            current_variable_set_list = (*file).pat_variables;
            loop {
                // Both definition paths return a live variable; bind it once as
                // a checked `&mut` produced by the if/else so every field write
                // below is a reference access (no raw deref for CodeQL) and the
                // `f_simple` flavor write reuses that same binding — no extra
                // statement line and no added branch.
                let v = if (*p).variable.flavor() as i32 == f_simple as i32 {
                    let v = define_variable_in_set(
                        ctx,
                        (*p).variable.name,
                        strlen((*p).variable.name) as size_t,
                        (*p).variable.value,
                        (*p).variable.origin(),
                        0,
                        (*current_variable_set_list).set,
                        &raw mut (*p).variable.fileinfo,
                    )
                    .as_mut()
                    .expect("define_variable_in_set returned null");
                    v.set_flavor(f_simple as variable_flavor);
                    v
                } else {
                    do_variable_definition(
                        ctx,
                        &raw mut (*p).variable.fileinfo,
                        (*p).variable.name,
                        (*p).variable.value,
                        (*p).variable.origin(),
                        (*p).variable.flavor(),
                        (*p).variable.conditional() as i32,
                        s_pattern,
                    )
                    .as_mut()
                    .expect("do_variable_definition returned null")
                };
                v.set_per_target((*p).variable.per_target() as ::core::ffi::c_uint);
                v.set_export((*p).variable.export() as variable_export);
                v.set_private_var((*p).variable.private_var() as ::core::ffi::c_uint);
                p = lookup_pattern_var(p, (*file).name, targlen);
                if p.is_null() {
                    break;
                }
            }
            current_variable_set_list = global;
        }
        (*file).pat_searched = true;
    }
    if !(*file).pat_variables.is_null() {
        (*(*file).pat_variables).next = (*l).next;
        (*(*file).pat_variables).next_is_parent = (*l).next_is_parent;
        (*l).next = (*file).pat_variables;
        (*l).next_is_parent = 0;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn create_new_variable_set() -> *mut variable_set_list {
    let setlist: *mut variable_set_list;
    let set: *mut variable_set;
    set = xmalloc(::core::mem::size_of::<variable_set>() as size_t) as *mut variable_set;
    hash_init(
        &raw mut (*set).table,
        SMALL_SCOPE_VARIABLE_BUCKETS as ::core::ffi::c_ulong,
        Some(variable_hash_1),
        Some(variable_hash_2),
        Some(variable_hash_cmp),
    );
    setlist =
        xmalloc(::core::mem::size_of::<variable_set_list>() as size_t) as *mut variable_set_list;
    (*setlist).set = set;
    (*setlist).next = current_variable_set_list;
    (*setlist).next_is_parent = 0;
    setlist
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn push_new_variable_scope() -> *mut variable_set_list {
    current_variable_set_list = create_new_variable_set();
    if (*current_variable_set_list).next == &raw mut global_setlist {
        std::ptr::swap(
            &raw mut (*current_variable_set_list).set,
            &raw mut global_setlist.set,
        );
        (*current_variable_set_list).next = global_setlist.next;
        global_setlist.next = current_variable_set_list;
        current_variable_set_list = &raw mut global_setlist;
    }
    current_variable_set_list
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn pop_variable_scope() {
    let setlist: *mut variable_set_list;
    let set: *mut variable_set;
    if !(*current_variable_set_list).next.is_null() {
    } else {
        panic!("assertion failed: current_variable_set_list->next != NULL");
    };
    if current_variable_set_list != &raw mut global_setlist {
        setlist = current_variable_set_list;
        set = (*setlist).set;
        current_variable_set_list = (*setlist).next;
    } else {
        setlist = global_setlist.next;
        set = global_setlist.set;
        global_setlist.set = (*setlist).set;
        global_setlist.next = (*setlist).next;
        global_setlist.next_is_parent = (*setlist).next_is_parent;
    }
    free(setlist as *mut ::core::ffi::c_void);
    hash_map(&raw mut (*set).table, Some(free_variable_name_and_value));
    hash_free(&raw mut (*set).table, 1);
    free(set as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn install_file_context(
    file: *mut file,
    oldlist: *mut *mut variable_set_list,
    oldfloc: *mut *const Floc,
) {
    *oldlist = current_variable_set_list;
    current_variable_set_list = (*file).variables;
    if !oldfloc.is_null() {
        *oldfloc = reading_file;
        if !(*file).cmds.is_null() && !(*(*file).cmds).fileinfo.filenm.is_null() {
            reading_file = &raw mut (*(*file).cmds).fileinfo;
        } else {
            reading_file = ::core::ptr::null::<Floc>();
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn restore_file_context(oldlist: *mut variable_set_list, oldfloc: *const Floc) {
    current_variable_set_list = oldlist;
    if !oldfloc.is_null() {
        reading_file = oldfloc;
    }
}
unsafe extern "C" fn merge_variable_sets(to_set: *mut variable_set, from_set: *mut variable_set) {
    let Some(from_set_ref) = from_set.as_ref() else {
        return;
    };
    let mut from_var_slot: *mut *mut variable = from_set_ref.table.ht_vec as *mut *mut variable;
    let from_var_end: *mut *mut variable =
        from_var_slot.offset(from_set_ref.table.ht_size as isize);
    let inc: i32 = if to_set == &raw mut global_variable_set {
        1
    } else {
        0
    };
    while from_var_slot < from_var_end {
        if let Some(&from_var) = from_var_slot.as_ref().filter(|slot| {
            !slot.is_null()
                && !::core::ptr::eq(**slot as *const ::core::ffi::c_void, hash_deleted_item)
        }) {
            let to_var_slot: *mut *mut variable = hash_find_slot(
                &raw mut (*to_set).table,
                from_var as *const ::core::ffi::c_void,
            ) as *mut *mut variable;
            if (*to_var_slot).is_null()
                || *to_var_slot as *mut ::core::ffi::c_void
                    == hash_deleted_item as *mut ::core::ffi::c_void
            {
                hash_insert_at(
                    &raw mut (*to_set).table,
                    from_var as *const ::core::ffi::c_void,
                    to_var_slot as *const ::core::ffi::c_void,
                );
                VARIABLE_CHANGENUM.fetch_add(inc as u64, ::std::sync::atomic::Ordering::Relaxed);
            } else {
                if let Some(fv) = from_var.as_ref() {
                    free(fv.value as *mut ::core::ffi::c_void);
                }
                free(from_var as *mut ::core::ffi::c_void);
            }
        }
        from_var_slot = from_var_slot.offset(1_i32 as isize);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn merge_variable_set_lists(
    setlist0: *mut *mut variable_set_list,
    mut setlist1: *mut variable_set_list,
) {
    let mut to: *mut variable_set_list = *setlist0;
    let mut last0: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    if setlist1.is_null() || setlist1 == &raw mut global_setlist {
        return;
    }
    if !to.is_null() {
        while to != &raw mut global_setlist {
            if to == setlist1 {
                return;
            }
            to = (*to).next;
        }
        to = *setlist0;
        while setlist1 != &raw mut global_setlist && to != &raw mut global_setlist {
            // Both pointers are non-null inside this loop: `setlist1` was
            // null-checked at the top, and `to` came through the `!to.is_null()`
            // guard. Read them via checked references (keeps CodeQL satisfied)
            // without adding a branch.
            let fromr = setlist1.as_ref().expect("setlist1 non-null in merge loop");
            let tor = to.as_ref().expect("to non-null in merge loop");
            setlist1 = fromr.next;
            merge_variable_sets(tor.set, fromr.set);
            last0 = to;
            to = tor.next;
        }
    }
    if setlist1 != &raw mut global_setlist {
        if let Some(last0r) = last0.as_mut() {
            last0r.next = setlist1;
        } else {
            *setlist0 = setlist1;
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn define_automatic_variables(ctx: &crate::execctx::ExecContext) {
    let mut v: *mut variable;
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
        ctx.makelevel(),
    );
    define_variable_in_set(
        ctx,
        b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        &raw mut buf as *mut ::core::ffi::c_char,
        o_env,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        crate::version::version_string(),
        if remote_description.is_null() || *remote_description.offset(0_i32 as isize) as i32 == 0 {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"-\0" as *const u8 as *const ::core::ffi::c_char
        },
        if remote_description.is_null() || *remote_description.offset(0_i32 as isize) as i32 == 0 {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            remote_description as *const ::core::ffi::c_char
        },
    );
    define_variable_in_set(
        ctx,
        b"MAKE_VERSION\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        &raw mut buf as *mut ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"MAKE_HOST\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        crate::version::make_host(),
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    v = define_variable_in_set(
        ctx,
        b"SHELL\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        default_shell,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    if *(*v).value as i32 == 0
        || (*v).origin() as i32 == o_env as i32
        || (*v).origin() as i32 == o_env_override as i32
    {
        free((*v).value as *mut ::core::ffi::c_void);
        (*v).set_origin(o_file as variable_origin);
        (*v).value = xstrdup(default_shell);
    }
    v = define_variable_in_set(
        ctx,
        b"MAKEFILES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    (*v).set_export(v_ifset as variable_export);
    define_variable_in_set(
        ctx,
        b"@D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $@))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"%D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $%))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"*D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $*))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"<D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $<))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"?D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $?))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"^D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $^))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"+D\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(patsubst %/,%,$(dir $+))\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"@F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $@)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"%F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $%)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"*F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $*)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"<F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $<)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"?F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $?)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"^F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $^)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
    define_variable_in_set(
        ctx,
        b"+F\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
        b"$(notdir $+)\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        1,
        (*current_variable_set_list).set,
        NILF,
    );
}
/// Pure decision behind [`should_export`]: given a variable's export mode,
/// origin, whether it is exportable, and the `export_all_variables` flag,
/// decide whether the variable belongs in a child process's environment.
fn should_export_decision(
    export: variable_export,
    origin: variable_origin,
    exportable: bool,
    export_all: bool,
) -> bool {
    match export {
        // v_noexport: never export.
        2 => false,
        // v_ifset: export only if the variable was actually set somewhere.
        3 => origin != o_default,
        // v_default: export an exportable variable unless its origin forbids
        // it (default/automatic vars, or non-command/env vars when
        // `export_all_variables` is off).
        0 => {
            origin != o_default
                && origin != o_automatic
                && exportable
                && (export_all
                    || origin == o_command
                    || origin == o_env
                    || origin == o_env_override)
        }
        // v_export (1) and any unexpected value: export.
        _ => true,
    }
}

/// Should variable `v` be placed in a child process's environment?
///
/// Safe wrapper over [`should_export_decision`]: it borrows the variable and
/// returns a plain `bool`. The `export_all_variables` flag is read from the
/// owned `Options` through the `with_options` borrow channel
/// ([`crate::make_main::opt_export_all_variables`]), so this is fully safe —
/// no `unsafe` and no global remain.
pub fn should_export(v: &variable) -> bool {
    let export_all = crate::make_main::opt_export_all_variables();
    should_export_decision(v.export(), v.origin(), v.exportable() != 0, export_all)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn target_environment(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    recursive: i32,
) -> *mut *mut ::core::ffi::c_char {
    let set_list: *mut variable_set_list;
    let mut s: *mut variable_set_list;
    let mut table: hash_table = hash_table {
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
    let mut v_slot: *mut *mut variable;
    let mut v_end: *mut *mut variable;
    let result_0: *mut *mut ::core::ffi::c_char;
    let mut result: *mut *mut ::core::ffi::c_char;
    let mut invalid: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut added_shell: i32 = shell_var.value.is_null() as i32;
    let mut found_makelevel: i32 = 0;
    let mut found_mflags: i32 = 0;
    let mut found_makeflags: i32 = 0;
    if file.is_null() {
        ENV_RECURSION.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
    }
    if recursive == 0 && crate::make_main::opt_jobserver_auth_present() {
        invalid = jobserver_get_invalid_auth();
    }
    if !file.is_null() {
        set_list = (*file).variables;
    } else {
        set_list = current_variable_set_list;
    }
    hash_init(
        &raw mut table,
        VARIABLE_BUCKETS as ::core::ffi::c_ulong,
        Some(variable_hash_1),
        Some(variable_hash_2),
        Some(variable_hash_cmp),
    );
    s = set_list;
    while !s.is_null() {
        let set: *mut variable_set = (*s).set;
        let islocal: i32 = (s == set_list) as i32;
        let isglobal: i32 = (set == &raw mut global_variable_set) as i32;
        v_slot = (*set).table.ht_vec as *mut *mut variable;
        v_end = v_slot.offset((*set).table.ht_size as isize);
        while v_slot < v_end {
            if !((*v_slot).is_null()
                || *v_slot as *mut ::core::ffi::c_void
                    == hash_deleted_item as *mut ::core::ffi::c_void)
            {
                let evslot: *mut *mut variable;
                let v: *mut variable = *v_slot;
                if !(islocal == 0 && (*v).private_var() as i32 != 0) {
                    evslot = hash_find_slot(&raw mut table, v as *const ::core::ffi::c_void)
                        as *mut *mut variable;
                    if (*evslot).is_null()
                        || *evslot as *mut ::core::ffi::c_void
                            == hash_deleted_item as *mut ::core::ffi::c_void
                    {
                        // `v` is a live, non-null variable taken from an
                        // occupied hash slot just above.
                        if isglobal == 0 || should_export(&*v) {
                            hash_insert_at(
                                &raw mut table,
                                v as *const ::core::ffi::c_void,
                                evslot as *const ::core::ffi::c_void,
                            );
                        }
                    } else if (**evslot).export() as i32 == v_default as i32 {
                        (**evslot).set_export((*v).export() as variable_export);
                    }
                }
            }
            v_slot = v_slot.offset(1_i32 as isize);
        }
        s = (*s).next;
    }
    result_0 = xmalloc(
        (table.ht_fill as size_t)
            .wrapping_add(3)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    result = result_0;
    v_slot = table.ht_vec as *mut *mut variable;
    v_end = v_slot.offset(table.ht_size as isize);
    while v_slot < v_end {
        if !((*v_slot).is_null()
            || *v_slot as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
        {
            let v_0: *mut variable = *v_slot;
            let mut value: *mut ::core::ffi::c_char = (*v_0).value;
            let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            // `v_0` is a live, non-null variable taken from an
            // occupied hash slot just above.
            if should_export(&*v_0) {
                if (*v_0).recursive() as i32 != 0
                    && ((*v_0).origin() as i32 != o_env as i32
                        && (*v_0).origin() as i32 != o_env_override as i32
                        || *(*v_0).name as i32
                            == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char) as i32
                            && (*(*v_0).name as i32 == 0
                                || strcmp(
                                    (*v_0).name.offset(1_i32 as isize),
                                    (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                        .offset(1_i32 as isize),
                                ) == 0))
                {
                    cp = recursively_expand_for_file(ctx, v_0, file);
                    value = cp;
                }
                if added_shell == 0
                    && (*(*v_0).name as i32
                        == *(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char) as i32
                        && (*(*v_0).name as i32 == 0
                            || strcmp(
                                (*v_0).name.offset(1_i32 as isize),
                                (b"SHELL\0" as *const u8 as *const ::core::ffi::c_char)
                                    .offset(1_i32 as isize),
                            ) == 0))
                {
                    added_shell = 1;
                } else if found_makelevel == 0
                    && (*(*v_0).name as i32
                        == *(b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char) as i32
                        && (*(*v_0).name as i32 == 0
                            || strcmp(
                                (*v_0).name.offset(1_i32 as isize),
                                (b"MAKELEVEL\0" as *const u8 as *const ::core::ffi::c_char)
                                    .offset(1_i32 as isize),
                            ) == 0))
                {
                    let mut val: [::core::ffi::c_char; 23] = [0; 23];
                    sprintf(
                        &raw mut val as *mut ::core::ffi::c_char,
                        b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                        ctx.makelevel().wrapping_add(1),
                    );
                    free(cp as *mut ::core::ffi::c_void);
                    cp = xstrdup(&raw mut val as *mut ::core::ffi::c_char);
                    value = cp;
                    found_makelevel = 1;
                } else if !invalid.is_null() {
                    if found_makeflags == 0
                        && (*(*v_0).name as i32
                            == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char) as i32
                            && (*(*v_0).name as i32 == 0
                                || strcmp(
                                    (*v_0).name.offset(1_i32 as isize),
                                    (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                        .offset(1_i32 as isize),
                                ) == 0))
                    {
                        let mf: *mut ::core::ffi::c_char;
                        let vars: *mut ::core::ffi::c_char;
                        found_makeflags = 1;
                        if !strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                        {
                            vars =
                                strstr(value, b" -- \0" as *const u8 as *const ::core::ffi::c_char);
                            if vars.is_null() {
                                mf = xstrdup(concat(&[value, invalid]));
                            } else {
                                let lf: size_t =
                                    vars.offset_from(value) as ::core::ffi::c_long as size_t;
                                let li: size_t = strlen(invalid) as size_t;
                                mf = xmalloc(
                                    (strlen(value) as size_t).wrapping_add(li).wrapping_add(1),
                                ) as *mut ::core::ffi::c_char;
                                strcpy(
                                    mempcpy(
                                        mempcpy(
                                            mf as *mut ::core::ffi::c_void,
                                            value as *const ::core::ffi::c_void,
                                            lf as size_t,
                                        ),
                                        invalid as *const ::core::ffi::c_void,
                                        li as size_t,
                                    )
                                        as *mut ::core::ffi::c_char,
                                    vars,
                                );
                            }
                            free(cp as *mut ::core::ffi::c_void);
                            cp = mf;
                            value = cp;
                            if found_mflags != 0 {
                                invalid = ::core::ptr::null::<::core::ffi::c_char>();
                            }
                        }
                    } else if found_mflags == 0
                        && (*(*v_0).name as i32
                            == *(b"MFLAGS\0" as *const u8 as *const ::core::ffi::c_char) as i32
                            && (*(*v_0).name as i32 == 0
                                || strcmp(
                                    (*v_0).name.offset(1_i32 as isize),
                                    (b"MFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                        .offset(1_i32 as isize),
                                ) == 0))
                    {
                        let mf_0: *const ::core::ffi::c_char;
                        found_mflags = 1;
                        if !strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                            && !((*v_0).origin() as i32 != o_env as i32)
                        {
                            mf_0 = concat(&[value, invalid]);
                            free(cp as *mut ::core::ffi::c_void);
                            cp = xstrdup(mf_0);
                            value = cp;
                            if found_makeflags != 0 {
                                invalid = ::core::ptr::null::<::core::ffi::c_char>();
                            }
                        }
                    }
                }
                let fresh10 = result;
                result = result.offset(1_i32 as isize);
                *fresh10 = xstrdup(concat(
                    3,
                    (*v_0).name,
                    b"=\0" as *const u8 as *const ::core::ffi::c_char,
                    value,
                ));
                free(cp as *mut ::core::ffi::c_void);
            }
        }
        v_slot = v_slot.offset(1_i32 as isize);
    }
    if added_shell == 0 {
        let fresh11 = result;
        result = result.offset(1_i32 as isize);
        *fresh11 = xstrdup(concat(
            3,
            shell_var.name,
            b"=\0" as *const u8 as *const ::core::ffi::c_char,
            shell_var.value,
        ));
    }
    if found_makelevel == 0 {
        let mut val_0: [::core::ffi::c_char; 33] = [0; 33];
        sprintf(
            &raw mut val_0 as *mut ::core::ffi::c_char,
            b"%s=%u\0" as *const u8 as *const ::core::ffi::c_char,
            MAKELEVEL_NAME.as_ptr(),
            ctx.makelevel().wrapping_add(1),
        );
        let fresh12 = result;
        result = result.offset(1_i32 as isize);
        *fresh12 = xstrdup(&raw mut val_0 as *mut ::core::ffi::c_char);
    }
    *result = ::core::ptr::null_mut::<::core::ffi::c_char>();
    hash_free(&raw mut table, 0);
    if file.is_null() {
        ENV_RECURSION.fetch_sub(1, ::std::sync::atomic::Ordering::Relaxed);
    }
    result_0
}
unsafe fn set_special_var(
    ctx: &crate::execctx::ExecContext,
    var: *mut variable,
    origin: variable_origin,
) -> *mut variable {
    let Some(varr) = var.as_ref() else {
        return var;
    };
    let vname: *const ::core::ffi::c_char = varr.name;
    let vn0: i32 = vname.as_ref().map_or(-1, |c| *c as i32);
    if vn0 == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char) as i32
        && (vn0 == 0
            || strcmp(
                vname.offset(1_i32 as isize),
                (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char).offset(1_i32 as isize),
            ) == 0)
    {
        crate::make_main::reset_makeflags_special(ctx, origin);
    } else if vn0 == *(b".RECIPEPREFIX\0" as *const u8 as *const ::core::ffi::c_char) as i32
        && (vn0 == 0
            || strcmp(
                vname.offset(1_i32 as isize),
                (b".RECIPEPREFIX\0" as *const u8 as *const ::core::ffi::c_char)
                    .offset(1_i32 as isize),
            ) == 0)
    {
        let new_prefix = (if *varr.value.offset(0_i32 as isize) as i32 == 0 {
            RECIPEPREFIX_DEFAULT
        } else {
            *varr.value.offset(0_i32 as isize) as i32
        }) as ::core::ffi::c_char;
        crate::make_main::with_options(|o| o.cmd_prefix.set(new_prefix));
    } else if vn0 == *(b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char) as i32
        && (vn0 == 0
            || strcmp(
                vname.offset(1_i32 as isize),
                (b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char).offset(1_i32 as isize),
            ) == 0)
    {
        let actions: *mut ::core::ffi::c_char = allocated_expand_variable(
            ctx,
            b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        );
        let arg = ::core::ffi::CStr::from_ptr(actions).to_str().unwrap_or("");
        warning::decode_actions(ctx, arg, Some(&varr.fileinfo));
        free(actions as *mut ::core::ffi::c_void);
    }
    var
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn shell_result(
    ctx: &crate::execctx::ExecContext,
    p: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    let mut args: [*mut ::core::ffi::c_char; 2] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
    install_variable_buffer(&raw mut buf, &raw mut len);
    args[0_i32 as usize] = p as *mut ::core::ffi::c_char;
    args[1_i32 as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    func_shell_base(
        ctx,
        variable_buffer,
        &raw mut args as *mut *mut ::core::ffi::c_char,
        0,
    );
    swap_variable_buffer(buf, len)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
#[allow(clippy::too_many_arguments)]
pub unsafe fn do_variable_definition(
    ctx: &crate::execctx::ExecContext,
    flocp: *const Floc,
    varname: *const ::core::ffi::c_char,
    value: *const ::core::ffi::c_char,
    origin: variable_origin,
    mut flavor: variable_flavor,
    conditional: i32,
    scope: variable_scope,
) -> *mut variable {
    // Set to false by the one branch that must keep the existing value
    // (appending an empty string); every other branch defines the variable.
    let mut do_define = true;
    let mut newval: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut alloc_value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: *mut variable = ::core::ptr::null_mut::<variable>();
    let mut append: i32 = 0;
    if conditional != 0 {
        v = lookup_variable(ctx, varname, strlen(varname) as size_t);
        if !v.is_null() {
            return v;
        }
    }
    match flavor as ::core::ffi::c_uint {
        1 => {
            alloc_value =
                allocated_expand_string_for_file(ctx, value, ::core::ptr::null_mut::<file>());
            newval = alloc_value;
        }
        3 => {
            let t: *mut ::core::ffi::c_char =
                allocated_expand_string_for_file(ctx, value, ::core::ptr::null_mut::<file>());
            alloc_value = xmalloc((strlen(t) as size_t).wrapping_mul(2).wrapping_add(1))
                as *mut ::core::ffi::c_char;
            let mut np: *mut ::core::ffi::c_char = alloc_value;
            let mut op: *mut ::core::ffi::c_char = t;
            while *op.offset(0_i32 as isize) as i32 != 0 {
                if *op.offset(0_i32 as isize) as i32 == '$' as i32 {
                    let fresh0 = np;
                    np = np.offset(1_i32 as isize);
                    *fresh0 = '$' as i32 as ::core::ffi::c_char;
                }
                let fresh1 = op;
                op = op.offset(1_i32 as isize);
                let fresh2 = np;
                np = np.offset(1_i32 as isize);
                *fresh2 = *fresh1;
            }
            *np = 0;
            free(t as *mut ::core::ffi::c_void);
            newval = alloc_value;
        }
        5 => {
            let q: *mut ::core::ffi::c_char =
                allocated_expand_string_for_file(ctx, value, ::core::ptr::null_mut::<file>());
            alloc_value = shell_result(ctx, q);
            free(q as *mut ::core::ffi::c_void);
            flavor = f_recursive;
            newval = alloc_value;
        }
        2 => {
            newval = value;
        }
        4 | 6 => {
            let mut override_0: i32 = 0;
            if scope as ::core::ffi::c_uint == s_global as i32 as ::core::ffi::c_uint {
                v = lookup_variable(ctx, varname, strlen(varname) as size_t);
            } else {
                append = 1;
                v = lookup_variable_in_set(
                    ctx,
                    varname,
                    strlen(varname) as size_t,
                    (*current_variable_set_list).set,
                );
                if let Some(vr) = v.as_ref() {
                    if vr.append() == 0 {
                        append = 0;
                    }
                    if scope as ::core::ffi::c_uint == s_pattern as i32 as ::core::ffi::c_uint
                        && (vr.origin() as i32 == o_env_override as i32
                            || vr.origin() as i32 == o_command as i32)
                    {
                        override_0 = 1;
                        append = 1;
                    }
                }
            }
            if v.is_null() {
                newval = value;
                flavor = f_recursive;
            } else if override_0 != 0 {
                newval = value;
                flavor = f_recursive;
            } else if let Some(vr) = v.as_ref() {
                let oldlen: size_t;
                let vallen: size_t;
                let alloclen: size_t;
                let mut val: *const ::core::ffi::c_char;
                let mut cp: *mut ::core::ffi::c_char;
                let mut tp: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                val = value;
                if vr.recursive() != 0 {
                    flavor = f_recursive;
                } else if flavor as ::core::ffi::c_uint
                    != f_append_value as i32 as ::core::ffi::c_uint
                {
                    tp =
                        allocated_expand_string_for_file(ctx, val, ::core::ptr::null_mut::<file>());
                    val = tp;
                }
                vallen = strlen(val) as size_t;
                if vallen == 0 {
                    alloc_value = tp;
                    do_define = false;
                } else {
                    oldlen = strlen(vr.value) as size_t;
                    alloclen = oldlen.wrapping_add(1).wrapping_add(vallen).wrapping_add(1);
                    alloc_value = xmalloc(alloclen) as *mut ::core::ffi::c_char;
                    cp = alloc_value;
                    if oldlen != 0 {
                        let s: *mut ::core::ffi::c_char;
                        if varname.as_ref().is_some_and(|vn| {
                            *vn as i32
                                == *(b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                    as i32
                                && (*vn as i32 == 0
                                    || strcmp(
                                        varname.offset(1_i32 as isize),
                                        (b"MAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char)
                                            .offset(1_i32 as isize),
                                    ) == 0)
                        }) && {
                            s = strstr(
                                vr.value,
                                b" -- \0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            !s.is_null()
                        } {
                            cp = mempcpy(
                                cp as *mut ::core::ffi::c_void,
                                vr.value as *const ::core::ffi::c_void,
                                s.offset_from(vr.value) as ::core::ffi::c_long as size_t,
                            ) as *mut ::core::ffi::c_char;
                        } else {
                            cp = mempcpy(
                                cp as *mut ::core::ffi::c_void,
                                vr.value as *const ::core::ffi::c_void,
                                oldlen as size_t,
                            ) as *mut ::core::ffi::c_char;
                        }
                        let fresh3 = cp;
                        cp = cp.offset(1_i32 as isize);
                        *fresh3 = ' ' as i32 as ::core::ffi::c_char;
                    }
                    memcpy(
                        cp as *mut ::core::ffi::c_void,
                        val as *const ::core::ffi::c_void,
                        (vallen as size_t).wrapping_add(1),
                    );
                    free(tp as *mut ::core::ffi::c_void);
                    newval = alloc_value;
                }
            }
        }
        0 | _ => {
            abort();
        }
    }
    if do_define {
        if newval.is_null() {
            panic!("assertion failed: newval");
        }
        v = define_variable_in_set(
            ctx,
            varname,
            strlen(varname) as size_t,
            newval,
            origin,
            (flavor as ::core::ffi::c_uint == f_recursive as i32 as ::core::ffi::c_uint
                || flavor as ::core::ffi::c_uint == f_expand as i32 as ::core::ffi::c_uint)
                as i32,
            if scope as ::core::ffi::c_uint == s_global as i32 as ::core::ffi::c_uint {
                ::core::ptr::null_mut::<variable_set>()
            } else {
                (*current_variable_set_list).set
            },
            flocp,
        );
        (*v).set_append(append as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*v).set_conditional(conditional as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    free(alloc_value as *mut ::core::ffi::c_void);
    match v.as_mut() {
        Some(vr) if vr.special() as i32 != 0 => set_special_var(ctx, vr as *mut variable, origin),
        _ => v,
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn parse_variable_definition(
    str: *const ::core::ffi::c_char,
    var: *mut variable,
) -> *mut ::core::ffi::c_char {
    // The operator-detection state machine now lives in the typed AST layer
    // (`crate::parser`), which parses the line as a safe byte slice. Here we
    // only marshal the result back into the C-facing `struct variable`: the
    // name points into the original buffer (it is not copied or terminated),
    // and the returned pointer is the address just past the operator.
    let bytes = ::core::ffi::CStr::from_ptr(str).to_bytes();
    match crate::parser::assignment_ast(bytes) {
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
        Some(a) => {
            (*var).name = str.add(a.name_start) as *mut ::core::ffi::c_char;
            (*var).length = a.name_len as ::core::ffi::c_uint;
            (*var).set_conditional(a.conditional as ::core::ffi::c_uint);
            (*var).set_flavor(a.flavor.to_variable_flavor());
            (*var).value = str.add(a.value_start) as *mut ::core::ffi::c_char;
            str.add(a.op_end) as *mut ::core::ffi::c_char
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn assign_variable_definition(
    ctx: &crate::execctx::ExecContext,
    v: *mut variable,
    line: *const ::core::ffi::c_char,
) -> *mut variable {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let name: *mut ::core::ffi::c_char;
    if parse_variable_definition(line, v).is_null() {
        return ::core::ptr::null_mut::<variable>();
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (*v).length.wrapping_add(1) as ::core::ffi::c_ulong as usize,
    ));
    name = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    memcpy(
        name as *mut ::core::ffi::c_void,
        (*v).name as *const ::core::ffi::c_void,
        (*v).length as size_t,
    );
    *name.offset((*v).length as isize) = 0;
    (*v).name = allocated_expand_string_for_file(ctx, name, ::core::ptr::null_mut::<file>());
    fatal_on_empty_variable_name(ctx, v);
    v
}

/// Abort with "empty variable name" when `v`'s (already expanded) name is the
/// empty string. Split out of `assign_variable_definition` so that function
/// stays a flat two-branch sequence; this guard is a never-returning error path
/// (the makefile must name the variable being defined), so it is exercised only
/// by the error-handling integration cases, not the unit tests.
///
/// # Safety
///
/// `v` must be a valid `variable` whose `name` points at a live NUL-terminated
/// string, and `ctx` must be valid for diagnostic reporting.
unsafe fn fatal_on_empty_variable_name(ctx: &crate::execctx::ExecContext, v: *mut variable) {
    if *(*v).name.offset(0_i32 as isize) as i32 == 0 {
        fatal(ctx, &raw mut (*v).fileinfo, 0, 0, b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn try_variable_definition(
    ctx: &crate::execctx::ExecContext,
    flocp: *const Floc,
    line: *const ::core::ffi::c_char,
    origin: variable_origin,
    scope: variable_scope,
) -> *mut variable {
    let mut v: variable = variable {
        name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        value: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        fileinfo: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let vp: *mut variable;
    if !flocp.is_null() {
        v.fileinfo = *flocp;
    } else {
        v.fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if assign_variable_definition(ctx, &raw mut v, line).is_null() {
        return ::core::ptr::null_mut::<variable>();
    }
    vp = do_variable_definition(
        ctx,
        flocp,
        v.name,
        v.value,
        origin,
        v.flavor(),
        v.conditional() as i32,
        scope,
    );
    free(v.name as *mut ::core::ffi::c_void);
    vp
}
static mut defined_vars: [defined_vars; 13] = [defined_vars {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    len: 0,
}; 13];
/// Emit a "reference to undefined variable" warning for `name`, unless `name`
/// is one of the built-in always-defined variables in the `defined_vars`
/// table, or the warning is inactive.
pub fn warn_undefined(ctx: &crate::execctx::ExecContext, name: &[u8]) {
    if warning::is_active(Type::UndefinedVar) {
        // SAFETY: `defined_vars` is a process-wide, NUL-terminated table of
        // built-in variable names, populated once during startup and never
        // mutated afterwards. We only read it here, walking until the
        // sentinel null `name`, and compare each entry's bytes against `name`.
        let is_builtin = unsafe {
            let mut dp = &raw const defined_vars as *const defined_vars;
            let mut found = false;
            while !(*dp).name.is_null() {
                if (*dp).len == name.len()
                    && ::core::slice::from_raw_parts((*dp).name as *const u8, (*dp).len) == name
                {
                    found = true;
                    break;
                }
                dp = dp.offset(1);
            }
            found
        };
        if is_builtin {
            return;
        }
        if warning::is_active(Type::UndefinedVar) {
            emit_var_name_warning(
                ctx,
                // SAFETY: `reading_file` is a process-wide pointer to the
                // current Floc, set during makefile evaluation; read-only here.
                unsafe { reading_file.as_ref() },
                warning::action(Type::UndefinedVar) == Action::Error,
                "reference to undefined variable",
                name,
            );
        }
    }
}

#[cfg(test)]
mod warn_undefined_unsafe_oracle {
    use super::*;

    /// Verbatim pre-conversion implementation, preserved as a differential
    /// test oracle.
    unsafe fn warn_undefined(
        ctx: &crate::execctx::ExecContext,
        name: *const ::core::ffi::c_char,
        len: size_t,
    ) {
        if warning::is_active(Type::UndefinedVar) {
            let mut dp: *const defined_vars;
            dp = &raw const defined_vars as *const defined_vars;
            while !(*dp).name.is_null() {
                if (*dp).len == len
                    && memcmp(
                        (*dp).name as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        len as size_t,
                    ) == 0
                {
                    return;
                }
                dp = dp.offset(1 as i32 as isize);
            }
            if warning::is_active(Type::UndefinedVar) {
                emit_var_name_warning(
                    ctx,
                    reading_file.as_ref(),
                    warning::action(Type::UndefinedVar) == Action::Error,
                    "reference to undefined variable",
                    ::core::slice::from_raw_parts(name as *const u8, len),
                );
            }
        }
    }

    /// Membership decision mirroring the safe `warn_undefined`'s scan.
    fn is_builtin_safe(name: &[u8]) -> bool {
        // SAFETY: read-only walk of the process-wide built-in table; see
        // `warn_undefined`.
        unsafe {
            let mut dp = &raw const defined_vars as *const defined_vars;
            while !(*dp).name.is_null() {
                if (*dp).len == name.len()
                    && ::core::slice::from_raw_parts((*dp).name as *const u8, (*dp).len) == name
                {
                    return true;
                }
                dp = dp.offset(1);
            }
            false
        }
    }

    /// Membership decision mirroring the oracle's `memcmp` scan.
    fn is_builtin_oracle(name: *const ::core::ffi::c_char, len: size_t) -> bool {
        // SAFETY: read-only walk of the process-wide built-in table.
        unsafe {
            let mut dp = &raw const defined_vars as *const defined_vars;
            while !(*dp).name.is_null() {
                if (*dp).len == len
                    && memcmp(
                        (*dp).name as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        len as size_t,
                    ) == 0
                {
                    return true;
                }
                dp = dp.offset(1 as i32 as isize);
            }
            false
        }
    }

    /// The observable side effect (whether a warning is emitted) is gated on
    /// the global warning state, which we do not perturb. What this pins down
    /// is the membership decision against `defined_vars`: the safe version must
    /// classify a name as built-in iff the oracle's `memcmp`-based scan would
    /// have early-returned. Both sides read identical bytes (cast through
    /// platform `c_char`), and we assert agreement across representative inputs
    /// (empty, embedded NUL, high bytes 0x80/0xff, and known built-in names).
    #[test]
    fn differential_membership() {
        let cases: &[&[u8]] = &[
            b"",
            b"\0",
            b"a\0b",
            b".VARIABLES",
            b"MAKE",
            b"MAKEFILE_LIST",
            b"definitely-not-a-builtin",
            &[0x80, 0xff],
            &[b'M', b'A', b'K', b'E', 0xff],
        ];
        for &c in cases {
            let safe = is_builtin_safe(c);
            let oracle = is_builtin_oracle(c.as_ptr() as *const ::core::ffi::c_char, c.len());
            assert_eq!(safe, oracle, "membership mismatch for {c:?}");
        }
        // Exercise the original oracle entry point. Warning state is inactive
        // in unit tests, so this is a no-op beyond confirming it runs.
        unsafe {
            let ctx = crate::execctx::ExecContext::default();
            warn_undefined(&ctx, b"MAKE\0".as_ptr() as *const ::core::ffi::c_char, 4);
        }
    }
}
unsafe fn set_env_override(item: *const ::core::ffi::c_void, mut _arg: *mut ::core::ffi::c_void) {
    let v: *mut variable = item as *mut variable;
    let old: variable_origin = (if crate::make_main::env_overrides() {
        o_env as i32
    } else {
        o_env_override as i32
    }) as variable_origin;
    let new: variable_origin = (if crate::make_main::env_overrides() {
        o_env_override as i32
    } else {
        o_env as i32
    }) as variable_origin;
    if (*v).origin() as ::core::ffi::c_uint == old as ::core::ffi::c_uint {
        (*v).set_origin(new as variable_origin);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn reset_env_override() {
    hash_map_arg(
        &raw mut global_variable_set.table,
        Some(set_env_override),
        NULL,
    );
}
unsafe fn print_variable(item: *const ::core::ffi::c_void, arg: *mut ::core::ffi::c_void) {
    let v: *const variable = item as *const variable;
    let prefix: *const ::core::ffi::c_char = arg as *const ::core::ffi::c_char;
    let mut origin: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    match (*v).origin() as i32 {
        6 => {
            origin = b"automatic\0" as *const u8 as *const ::core::ffi::c_char;
        }
        0 => {
            origin = b"default\0" as *const u8 as *const ::core::ffi::c_char;
        }
        1 => {
            origin = b"environment\0" as *const u8 as *const ::core::ffi::c_char;
        }
        2 => {
            origin = b"makefile\0" as *const u8 as *const ::core::ffi::c_char;
        }
        3 => {
            origin = b"environment under -e\0" as *const u8 as *const ::core::ffi::c_char;
        }
        4 => {
            origin = b"command line\0" as *const u8 as *const ::core::ffi::c_char;
        }
        5 => {
            origin = b"'override' directive\0" as *const u8 as *const ::core::ffi::c_char;
        }
        7 => {
            abort();
        }
        _ => {}
    }
    fputs(b"# \0" as *const u8 as *const ::core::ffi::c_char, stdout);
    fputs(origin, stdout);
    if (*v).private_var() != 0 {
        fputs(
            b" private\0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
    }
    if !(*v).fileinfo.filenm.is_null() {
        printf(
            b" (from '%s', line %lu)\0" as *const u8 as *const ::core::ffi::c_char,
            (*v).fileinfo.filenm,
            (*v).fileinfo.lineno.wrapping_add((*v).fileinfo.offset),
        );
    }
    putchar('\n' as i32);
    fputs(prefix, stdout);
    if (*v).recursive() as i32 != 0 && !strchr((*v).value, '\n' as i32).is_null() {
        printf(
            b"define %s\n%s\nendef\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*v).name,
            (*v).value,
        );
    } else {
        let mut p: *mut ::core::ffi::c_char;
        printf(
            b"%s %s= \0" as *const u8 as *const ::core::ffi::c_char,
            (*v).name,
            if (*v).recursive() as i32 != 0 {
                if (*v).append() as i32 != 0 {
                    b"+\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                }
            } else {
                b":\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        p = next_token((*v).value);
        if p != (*v).value && *p as i32 == 0 {
            printf(
                b"$(subst ,,%s)\0" as *const u8 as *const ::core::ffi::c_char,
                (*v).value,
            );
        } else if (*v).recursive() != 0 {
            fputs((*v).value, stdout);
        } else {
            p = (*v).value;
            while *p as i32 != 0 {
                if *p as i32 == '$' as i32 {
                    putchar('$' as i32);
                }
                putchar(*p as i32);
                p = p.offset(1_i32 as isize);
            }
        }
        putchar('\n' as i32);
    };
}
unsafe fn print_auto_variable(item: *const ::core::ffi::c_void, arg: *mut ::core::ffi::c_void) {
    let v: *const variable = item as *const variable;
    if (*v).origin() as i32 == o_automatic as i32 {
        print_variable(item, arg);
    }
}
unsafe fn print_noauto_variable(item: *const ::core::ffi::c_void, arg: *mut ::core::ffi::c_void) {
    let v: *const variable = item as *const variable;
    if (*v).origin() as i32 != o_automatic as i32 {
        print_variable(item, arg);
    }
}
unsafe extern "C" fn print_variable_set(
    set: *mut variable_set,
    prefix: *const ::core::ffi::c_char,
    pauto: i32,
) {
    hash_map_arg(
        &raw mut (*set).table,
        if pauto != 0 {
            Some(print_auto_variable)
        } else {
            Some(print_variable)
        },
        prefix as *mut ::core::ffi::c_void,
    );
    fputs(
        b"# variable set hash-table stats:\n\0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    fputs(b"# \0" as *const u8 as *const ::core::ffi::c_char, stdout);
    hash_print_stats(&raw mut (*set).table, stdout);
    putc('\n' as i32, stdout);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_variable_data_base() {
    puts(b"\n# Variables\n\0" as *const u8 as *const ::core::ffi::c_char);
    print_variable_set(
        &raw mut global_variable_set,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        0,
    );
    puts(b"\n# Pattern-specific Variable Values\0" as *const u8 as *const ::core::ffi::c_char);
    let mut p: *mut pattern_var;
    let mut rules: ::core::ffi::c_uint = 0;
    p = pattern_vars;
    while !p.is_null() {
        rules = rules.wrapping_add(1);
        printf(
            b"\n%s :\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).target,
        );
        print_variable(
            &raw mut (*p).variable as *const ::core::ffi::c_void,
            b"# \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
        p = (*p).next;
    }
    if rules == 0 {
        puts(
            b"\n# No pattern-specific variable values.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        printf(
            b"\n# %u pattern-specific variable values\0" as *const u8 as *const ::core::ffi::c_char,
            rules,
        );
    };
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_file_variables(file: *const file) {
    let file = file.as_ref().expect("print_file_variables requires a file");
    if let Some(file_vars) = file.variables.as_ref() {
        print_variable_set(
            file_vars.set,
            b"# \0" as *const u8 as *const ::core::ffi::c_char,
            1,
        );
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_target_variables(file: *const file) {
    let file = file
        .as_ref()
        .expect("print_target_variables requires a file");
    if let Some(file_vars) = file.variables.as_ref() {
        // Prefix each variable line with "<target>: ".
        let name = ::core::slice::from_raw_parts(file.name.cast::<u8>(), strlen(file.name));
        let mut prefix: Vec<u8> = Vec::with_capacity(name.len() + 3);
        prefix.extend_from_slice(name);
        prefix.extend_from_slice(b": \0");
        let set = file_vars
            .set
            .as_mut()
            .expect("a variable set list always has a set");
        hash_map_arg(
            &raw mut set.table,
            Some(print_noauto_variable),
            prefix.as_mut_ptr() as *mut ::core::ffi::c_void,
        );
    }
}
unsafe extern "C" fn run_static_initializers() {
    defined_vars = [
        defined_vars {
            name: b"MAKECMDGOALS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKE_RESTARTS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKE_TERMOUT\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKE_TERMERR\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"MAKEOVERRIDES\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b".DEFAULT\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"-*-command-variables-*-\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 24]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"-*-eval-flags-*-\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 17]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"VPATH\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"GPATH\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b".WARNINGS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: b"GNUMAKEFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            len: (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        },
        defined_vars {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            len: 0,
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

#[cfg(test)]
mod should_export_tests {
    use super::{
        o_automatic, o_command, o_default, o_env, o_env_override, should_export_decision,
        v_default, v_export, v_ifset, v_noexport,
    };

    #[test]
    fn noexport_never_exports() {
        // v_noexport wins regardless of origin / exportable / export_all.
        assert!(!should_export_decision(v_noexport, o_command, true, true));
        assert!(!should_export_decision(v_noexport, o_env, true, false));
    }

    #[test]
    fn export_always_exports() {
        assert!(should_export_decision(v_export, o_default, false, false));
        assert!(should_export_decision(v_export, o_automatic, false, false));
    }

    #[test]
    fn ifset_exports_unless_default_origin() {
        assert!(!should_export_decision(v_ifset, o_default, true, true));
        assert!(should_export_decision(v_ifset, o_command, false, false));
        assert!(should_export_decision(v_ifset, o_env, false, false));
    }

    #[test]
    fn default_mode_respects_origin_and_export_all() {
        // Not exportable -> never.
        assert!(!should_export_decision(v_default, o_command, false, true));
        // default / automatic origins -> never, even if exportable & export_all.
        assert!(!should_export_decision(v_default, o_default, true, true));
        assert!(!should_export_decision(v_default, o_automatic, true, true));
        // command / env / env_override always export when exportable.
        assert!(should_export_decision(v_default, o_command, true, false));
        assert!(should_export_decision(v_default, o_env, true, false));
        assert!(should_export_decision(
            v_default,
            o_env_override,
            true,
            false
        ));
        // Other origins (e.g. file/makefile) only export when export_all is set.
        let o_file = 2; // some non-command/env origin
        assert!(!should_export_decision(v_default, o_file, true, false));
        assert!(should_export_decision(v_default, o_file, true, true));
    }
}

/// Differential test: the new safe [`should_export`] must agree byte-for-byte
/// with the original c2rust unsafe implementation across every relevant input.
#[cfg(test)]
mod should_export_unsafe_oracle {
    use super::{
        o_automatic, o_command, o_default, o_env, o_env_override, o_file, o_invalid, o_override,
        should_export, should_export_decision, v_default, v_export, v_ifset, v_noexport, variable,
        variable_export, variable_origin,
    };
    use crate::make_main::{install_default_options_for_test, with_options};

    /// Original c2rust implementation, preserved as the behavioral oracle (raw
    /// `*const variable`, `i32` result). The `export_all_variables` flag now
    /// lives on the owned `Options`, read through the `with_options` channel.
    unsafe fn should_export_oracle(v: *const variable) -> i32 {
        let v = v
            .as_ref()
            .expect("should_export requires a non-null variable");
        should_export_decision(
            v.export(),
            v.origin(),
            v.exportable() != 0,
            with_options(|o| o.export_all_variables.get()),
        ) as i32
    }

    /// Build a zeroed `variable` carrying just the fields `should_export`
    /// inspects, so both implementations see identical state.
    fn make_var(export: variable_export, origin: variable_origin, exportable: bool) -> variable {
        // SAFETY: `variable` is `#[repr(C)]` and all-zeroes is a valid bit
        // pattern (null pointers, cleared bitfields); only the export/origin/
        // exportable fields are then set.
        let mut v: variable = unsafe { ::core::mem::zeroed() };
        v.set_export(export);
        v.set_origin(origin);
        v.set_exportable(exportable as ::core::ffi::c_uint);
        v
    }

    #[test]
    fn safe_matches_oracle_over_full_cross_product() {
        let exports = [v_default, v_export, v_noexport, v_ifset];
        let origins = [
            o_default,
            o_env,
            o_file,
            o_env_override,
            o_command,
            o_override,
            o_automatic,
            o_invalid,
        ];
        // Toggle the owned `Options` flag both ways through the borrow channel,
        // restoring it afterwards so the test leaves no residue in shared state.
        install_default_options_for_test();
        let saved = with_options(|o| o.export_all_variables.get());
        for &export_all in &[false, true] {
            with_options(|o| o.export_all_variables.set(export_all));
            for &export in &exports {
                for &origin in &origins {
                    for &exportable in &[false, true] {
                        let v = make_var(export, origin, exportable);
                        // SAFETY: `v` is a live local.
                        let oracle = unsafe { should_export_oracle(&raw const v) };
                        let safe = should_export(&v) as i32;
                        assert_eq!(
                            safe, oracle,
                            "mismatch: export={export} origin={origin} \
                             exportable={exportable} export_all={export_all}"
                        );
                    }
                }
            }
        }
        // Restore the previous value through the borrow channel.
        with_options(|o| o.export_all_variables.set(saved));
    }
}

#[cfg(test)]
mod env_recursion_tests {
    use super::{env_recursion, ENV_RECURSION};
    use std::sync::atomic::Ordering;

    /// `env_recursion()` is a plain load of the `ENV_RECURSION` counter, so it
    /// agrees with a direct load. Read-only to avoid disturbing the shared
    /// production global, which the enter/leave paths mutate — keeping this
    /// test safe under the parallel test harness.
    #[test]
    fn env_recursion_reflects_the_counter() {
        assert_eq!(env_recursion(), ENV_RECURSION.load(Ordering::Relaxed));
    }

    /// The fetch_add/fetch_sub the enter/leave paths use round-trip back to the
    /// starting value. Exercised on a local atomic so it never touches the
    /// shared production counter.
    #[test]
    fn env_recursion_add_sub_round_trips() {
        let counter = std::sync::atomic::AtomicU64::new(0);
        counter.fetch_add(1, Ordering::Relaxed);
        assert_eq!(counter.load(Ordering::Relaxed), 1);
        counter.fetch_sub(1, Ordering::Relaxed);
        assert_eq!(counter.load(Ordering::Relaxed), 0);
    }
}

#[cfg(test)]
mod initialize_file_variables_tests {
    use super::{global_setlist, initialize_file_variables};
    use crate::file::File;
    use crate::strcache::strcache_add;
    use std::sync::Mutex;

    // `global_setlist` is process-wide; serialize so these tests don't race.
    static GLOBAL_VARS_LOCK: Mutex<()> = Mutex::new(());

    /// For a fresh file (no per-target set, no parent, no double-colon),
    /// `initialize_file_variables` allocates the file's variable set and links
    /// it to the global set list as a parent scope. With `reading != 0` the
    /// pattern-variable scan is skipped, keeping the call self-contained.
    #[test]
    fn allocates_set_and_links_global_parent() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let name = strcache_add(c"ifv_probe_target".as_ptr());
            let mut f = File::default();
            f.name = name;
            f.hname = name;

            assert!(f.variables.is_null(), "starts without a variable set");
            let ctx = crate::execctx::ExecContext::default();
            initialize_file_variables(&ctx, &raw mut f, 1);

            let l = f.variables;
            assert!(!l.is_null(), "a variable set list is allocated");
            assert!(!(*l).set.is_null(), "the set itself is allocated");
            assert_eq!(
                (*l).next,
                &raw mut global_setlist,
                "parent scope is the global set list"
            );
            assert_eq!((*l).next_is_parent, 1);
        }
    }

    /// Calling it again when the file already has a variable set reuses that
    /// set (the allocation branch is skipped) and re-links the global parent.
    #[test]
    fn reuses_existing_set() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let name = strcache_add(c"ifv_probe_reuse".as_ptr());
            let mut f = File::default();
            f.name = name;
            f.hname = name;

            let ctx = crate::execctx::ExecContext::default();
            initialize_file_variables(&ctx, &raw mut f, 1);
            let first = f.variables;
            assert!(!first.is_null());

            initialize_file_variables(&ctx, &raw mut f, 1);
            assert_eq!(f.variables, first, "the existing set is reused");
        }
    }

    /// With `reading == 0`, the pattern-variable search arm runs. A target name
    /// that matches no defined pattern variable yields no match, so the search
    /// loop is skipped and `pat_searched` is set. Drives that branch without
    /// requiring any pattern variables to be installed.
    #[test]
    fn reading_zero_runs_pattern_search() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let name = strcache_add(c"ifv_reading0_unmatched_probe".as_ptr());
            let mut f = File::default();
            f.name = name;
            f.hname = name;

            assert_eq!(f.pat_searched(), 0, "starts un-searched");
            let ctx = crate::execctx::ExecContext::default();
            initialize_file_variables(&ctx, &raw mut f, 0);

            assert_eq!(f.pat_searched(), 1, "pattern search ran and was recorded");
            assert!(!f.variables.is_null(), "variable set still allocated");
        }
    }

    /// A file with a `parent` recurses into the parent first, then links the
    /// parent's variable set as the next (parent) scope. Drives the
    /// non-null-`parent` arm (the recursion + parent-scope link).
    #[test]
    fn parent_chains_into_parent_scope() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let pname = strcache_add(c"ifv_parent_probe".as_ptr());
            let cname = strcache_add(c"ifv_child_probe".as_ptr());
            let mut parent = File::default();
            parent.name = pname;
            parent.hname = pname;
            let mut child = File::default();
            child.name = cname;
            child.hname = cname;
            child.parent = &raw mut parent;

            let ctx = crate::execctx::ExecContext::default();
            initialize_file_variables(&ctx, &raw mut child, 1);

            assert!(!parent.variables.is_null(), "parent set is initialized");
            let l = child.variables;
            assert!(!l.is_null(), "child set is allocated");
            assert_eq!(
                (*l).next,
                parent.variables,
                "child's next scope is the parent's variable set"
            );
            assert_eq!((*l).next_is_parent, 1);
        }
    }

    /// A file whose `double_colon` points at a distinct file recurses into that
    /// entry and links its variable set as a (non-parent) sibling scope before
    /// returning early. Drives the `double_colon` arm.
    #[test]
    fn double_colon_links_sibling_scope() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let dname = strcache_add(c"ifv_dcolon_probe".as_ptr());
            let mname = strcache_add(c"ifv_main_probe".as_ptr());
            let mut dc = File::default();
            dc.name = dname;
            dc.hname = dname;
            let mut f = File::default();
            f.name = mname;
            f.hname = mname;
            f.double_colon = &raw mut dc;

            let ctx = crate::execctx::ExecContext::default();
            initialize_file_variables(&ctx, &raw mut f, 1);

            assert!(!dc.variables.is_null(), "double-colon set is initialized");
            let l = f.variables;
            assert!(!l.is_null());
            assert_eq!(
                (*l).next,
                dc.variables,
                "next scope is the double-colon entry's set"
            );
            assert_eq!(
                (*l).next_is_parent,
                0,
                "double-colon scope is a sibling, not a parent"
            );
        }
    }
}

#[cfg(test)]
mod assign_variable_definition_tests {
    use super::{assign_variable_definition, variable};

    /// When the line is not a variable definition, `parse_variable_definition`
    /// fails and `assign_variable_definition` returns null up front — before any
    /// name allocation or expansion. This early-return arm is the self-contained
    /// half of the function (no global expansion bootstrap needed); the success
    /// arm is driven by the makefile differential suite through a fully
    /// initialized runtime.
    #[test]
    fn rejects_non_definition_line() {
        // The assignment parser keys off the global stopchar map to find the
        // line terminator; initialize it first (idempotent) so the scan stops.
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        // SAFETY: `assign_variable_definition` is the c2rust raw-pointer API. We
        // pass a freshly zeroed `variable` and a valid NUL-terminated line,
        // matching how `try_variable_definition` calls it. A bare word is not an
        // assignment, so parsing fails and the call returns null without
        // allocating or expanding anything.
        unsafe {
            let mut v: variable = ::core::mem::zeroed();
            let not_def = c"just_a_bare_word";
            let r = assign_variable_definition(&ctx, &raw mut v, not_def.as_ptr());
            assert!(r.is_null(), "a non-definition line yields null");
            assert!(v.name.is_null(), "no name is allocated on the reject path");
        }
    }
}
