pub use crate::ffi_types::{size_t, uintmax_t};
use crate::file::{file, Commands, Dep, FileId, TargetVariable, VarExport, VarFlavor, VarOrigin};
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
    #[cfg(test)]
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

/// A scoped set of variables: a `hash_table` of `variable` records. Legacy
/// c2rust `#[repr(C)]` container; `file.rs` re-exports it for compatibility.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSet {
    pub table: hash_table,
}

/// A stack of [`VariableSet`] scopes, innermost first, linked by `next`. Legacy
/// c2rust `#[repr(C)]` container; `file.rs` re-exports it for compatibility.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSetList {
    pub next: *mut VariableSetList,
    pub set: *mut VariableSet,
    pub next_is_parent: i32,
}

pub type dep = Dep;
pub type commands = Commands;
use crate::expand::{
    allocated_expand_string_for_file, allocated_expand_variable, expanding_var,
    install_variable_buffer, recursively_expand_for_file, swap_variable_buffer, variable_buffer,
};
use crate::execctx::ExecContext;
use crate::floc::Floc;
use crate::strcache::strcache_add;
use crate::function::func_shell_base;
use crate::hash::{
    hash_delete_at, hash_deleted_item, hash_find_item, hash_find_slot, hash_free, hash_init,
    hash_insert_at, hash_map, hash_map_arg, hash_print_stats, jhash,
};
use crate::job::default_shell;
use crate::make_main::stopchar_map;
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

/// Map a c2rust `variable_flavor` discriminant to the idiomatic [`VarFlavor`].
fn flavor_from_c(f: variable_flavor) -> VarFlavor {
    match f as i32 {
        x if x == f_simple as i32 => VarFlavor::Simple,
        x if x == f_recursive as i32 => VarFlavor::Recursive,
        x if x == f_expand as i32 => VarFlavor::Expand,
        x if x == f_append as i32 => VarFlavor::Append,
        x if x == f_shell as i32 => VarFlavor::Shell,
        x if x == f_append_value as i32 => VarFlavor::AppendValue,
        _ => VarFlavor::Bogus,
    }
}

/// Map a c2rust `variable_origin` discriminant to the idiomatic [`VarOrigin`].
fn origin_from_c(o: variable_origin) -> VarOrigin {
    match o as i32 {
        x if x == o_env as i32 => VarOrigin::Environment,
        x if x == o_file as i32 => VarOrigin::File,
        x if x == o_env_override as i32 => VarOrigin::EnvOverride,
        x if x == o_command as i32 => VarOrigin::Command,
        x if x == o_override as i32 => VarOrigin::Override,
        x if x == o_automatic as i32 => VarOrigin::Automatic,
        x if x == o_invalid as i32 => VarOrigin::Invalid,
        _ => VarOrigin::Default,
    }
}

/// Map a c2rust `variable_export` discriminant to the idiomatic [`VarExport`].
fn export_from_c(e: variable_export) -> VarExport {
    match e as i32 {
        x if x == v_export as i32 => VarExport::Export,
        x if x == v_noexport as i32 => VarExport::NoExport,
        x if x == v_ifset as i32 => VarExport::IfSet,
        _ => VarExport::Default,
    }
}

/// Read the NUL-terminated bytes of a c2rust C string into an owned `Vec<u8>`
/// (without the trailing NUL). A null pointer yields an empty vector.
///
/// # Safety
/// `p` must be null or a valid NUL-terminated C string for the call.
unsafe fn c_str_to_vec(p: *const ::core::ffi::c_char) -> Vec<u8> {
    if p.is_null() {
        return Vec::new();
    }
    let len = strlen(p);
    ::core::slice::from_raw_parts(p as *const u8, len).to_vec()
}

/// Build an idiomatic [`TargetVariable`] from a c2rust `variable` record (the
/// representation held in a `pattern_var`). This is the bridge used when the
/// per-target/pattern variable store moves onto [`FileNode`]'s `Vec`s.
///
/// # Safety
/// `v` must point to a valid, fully initialized `variable`.
unsafe fn target_variable_from_c(v: *const variable) -> TargetVariable {
    // SAFETY: the caller guarantees `v` points to a valid, fully initialized
    // `variable`. Bind a checked reference so every field read below goes
    // through a provably-valid reference rather than raw pointer derefs.
    let vr = v.as_ref().expect("variable pointer is non-null");
    let defined_in = if vr.fileinfo.filenm.is_null() {
        None
    } else {
        Some(c_str_to_vec(vr.fileinfo.filenm))
    };
    TargetVariable {
        name: c_str_to_vec(vr.name),
        value: c_str_to_vec(vr.value),
        defined_in,
        defined_lineno: vr.fileinfo.lineno.wrapping_add(vr.fileinfo.offset),
        flavor: flavor_from_c(vr.flavor()),
        origin: origin_from_c(vr.origin()),
        export: export_from_c(vr.export()),
        recursive: vr.recursive() != 0,
        append: vr.append() != 0,
        conditional: vr.conditional() != 0,
        per_target: vr.per_target() != 0,
        special: vr.special() != 0,
        exportable: vr.exportable() != 0,
        private_var: vr.private_var() != 0,
    }
}

/// Materialize a transient c2rust `variable` from a [`TargetVariable`] so the
/// existing pointer-based printers (`print_variable`) can render it. The
/// returned `variable` borrows the NUL-terminated buffers handed back in the
/// tuple's second/third/fourth slots; keep those alive for the call.
///
/// Returns `(variable, name_buf, value_buf, file_buf)`. The bitfields are set
/// through the c2rust setters so the rendered output matches the legacy path.
fn c_variable_from_target(tv: &TargetVariable) -> (variable, Vec<u8>, Vec<u8>, Option<Vec<u8>>) {
    let mut name_buf = tv.name.clone();
    name_buf.push(0);
    let mut value_buf = tv.value.clone();
    value_buf.push(0);
    let file_buf = tv.defined_in.as_ref().map(|f| {
        let mut b = f.clone();
        b.push(0);
        b
    });
    let mut v: variable = variable {
        name: name_buf.as_ptr() as *mut ::core::ffi::c_char,
        value: value_buf.as_ptr() as *mut ::core::ffi::c_char,
        fileinfo: Floc {
            filenm: file_buf
                .as_ref()
                .map_or(::core::ptr::null(), |b| b.as_ptr() as *const ::core::ffi::c_char),
            lineno: tv.defined_lineno,
            offset: 0,
        },
        length: tv.name.len() as ::core::ffi::c_uint,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    v.set_flavor(tv.flavor as i32 as variable_flavor);
    v.set_origin(tv.origin as i32 as variable_origin);
    v.set_export(tv.export as i32 as variable_export);
    v.set_recursive(tv.recursive as ::core::ffi::c_uint);
    v.set_append(tv.append as ::core::ffi::c_uint);
    v.set_conditional(tv.conditional as ::core::ffi::c_uint);
    v.set_per_target(tv.per_target as ::core::ffi::c_uint);
    v.set_special(tv.special as ::core::ffi::c_uint);
    v.set_exportable(tv.exportable as ::core::ffi::c_uint);
    v.set_private_var(tv.private_var as ::core::ffi::c_uint);
    (v, name_buf, value_buf, file_buf)
}

/// Define (or replace) a per-target variable directly on a [`FileNode`]'s
/// `variables` vec — the idiomatic stand-in for `define_variable_in_set` into a
/// file's own `variable_set`. Used for the automatic variables (`$@`, `$<`, …)
/// that `set_file_variables` attaches to a target. An existing entry of the same
/// name is overwritten in place (matching the hash-table upsert); otherwise the
/// new definition is appended. No raw pointers, no `c_char`.
pub fn define_target_variable(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    name: &[u8],
    value: &[u8],
    origin: VarOrigin,
) {
    let Some(node) = ctx.filenodes.get(file) else {
        return;
    };
    let mut guard = node.lock().expect("file node poisoned");
    let tv = TargetVariable {
        name: name.to_vec(),
        value: value.to_vec(),
        defined_in: None,
        defined_lineno: 0,
        flavor: VarFlavor::Recursive,
        origin,
        export: VarExport::Default,
        recursive: true,
        append: false,
        conditional: false,
        per_target: true,
        special: false,
        exportable: false,
        private_var: false,
    };
    if let Some(slot) = guard.variables.iter_mut().find(|v| v.name == name) {
        *slot = tv;
    } else {
        guard.variables.push(tv);
    }
}

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
/// Order two variable names the way the variable hash table expects: shorter
/// names sort first, and names of equal length sort by their raw bytes.
///
/// This is the safe, pointer-free core of [`variable_hash_cmp`]. The C original
/// compared `length` first and only fell back to a `memcmp` of `length` bytes
/// when the lengths matched; that is exactly `len`-then-bytes ordering, since a
/// `memcmp` over equal-length buffers ranks them by their first differing byte.
fn variable_cmp(x_name: &[u8], y_name: &[u8]) -> ::core::cmp::Ordering {
    x_name
        .len()
        .cmp(&y_name.len())
        .then_with(|| x_name.cmp(y_name))
}

unsafe fn variable_hash_cmp(xv: *const ::core::ffi::c_void, yv: *const ::core::ffi::c_void) -> i32 {
    let x: &variable = &*(xv as *const variable);
    let y: &variable = &*(yv as *const variable);
    let x_name = ::core::slice::from_raw_parts(x.name as *const u8, x.length as usize);
    let y_name = ::core::slice::from_raw_parts(y.name as *const u8, y.length as usize);
    // `Ordering` is `#[repr(i8)]` with `Less = -1`, `Equal = 0`, `Greater = 1`,
    // so the cast reproduces the C callback's tri-state result without a branch.
    variable_cmp(x_name, y_name) as i32
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
    let set: &mut variable_set = set
        .as_mut()
        .expect("variable set pointer is non-null after the null-to-global fallback");
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
        // SAFETY: `v` was just checked to be neither null nor the hash
        // deleted-item sentinel, so it points to a valid `variable`. Bind a
        // checked reference so the field accesses below go through a
        // provably-valid reference rather than raw-pointer derefs.
        let vr = v
            .as_mut()
            .expect("existing variable slot pointer is non-null");
        if crate::make_main::env_overrides() && vr.origin() as i32 == o_env as i32 {
            vr.set_origin(o_env_override as variable_origin);
        }
        if origin as i32 >= vr.origin() as i32 {
            free(vr.value as *mut ::core::ffi::c_void);
            vr.value = xstrdup(value);
            if let Some(floc) = flocp.as_ref() {
                vr.fileinfo = *floc;
            } else {
                vr.fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
            }
            vr.set_origin(origin as variable_origin);
            vr.set_recursive(recursive as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        return v;
    }
    v = xcalloc(::core::mem::size_of::<variable>() as size_t) as *mut variable;
    // SAFETY: `xcalloc` aborts on allocation failure, so `v` is non-null and
    // points to a zeroed `variable`. Bind a checked reference for the rest of
    // this block so all field accesses go through a provably-valid reference.
    let vr = v
        .as_mut()
        .expect("xcalloc returns a non-null variable pointer");
    vr.name = xstrndup(name, length);
    vr.length = length as ::core::ffi::c_uint;
    hash_insert_at(
        &raw mut (*set).table,
        v as *const ::core::ffi::c_void,
        var_slot as *const ::core::ffi::c_void,
    );
    if ::core::ptr::eq(&raw const *set, &raw const global_variable_set) {
        VARIABLE_CHANGENUM.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
    }
    vr.value = xstrdup(value);
    if let Some(floc) = flocp.as_ref() {
        vr.fileinfo = *floc;
    }
    vr.set_origin(origin as variable_origin);
    vr.set_recursive(recursive as ::core::ffi::c_uint as ::core::ffi::c_uint);
    vr.set_export(v_default as variable_export);
    vr.set_exportable(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    name = vr.name;
    if *name as i32 != '_' as i32
        && ((*name as i32) < 'A' as i32 || *name as i32 > 'Z' as i32)
        && ((*name as i32) < 'a' as i32 || *name as i32 > 'z' as i32)
    {
        vr.set_exportable(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
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
            vr.set_exportable(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
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
    // SAFETY: caller guarantees `list` and its `set` are valid. Read the set
    // pointer through a checked reference, then take the table address through
    // a checked reference (no raw-pointer field derefs).
    let set = list.as_ref().expect("variable_set_list is non-null").set;
    let table = &raw mut set.as_mut().expect("variable_set is non-null").table;
    hash_map(table, Some(free_variable_name_and_value));
    hash_free(table, 1);
    free(set as *mut ::core::ffi::c_void);
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
    let set: &mut variable_set = set
        .as_mut()
        .expect("variable set pointer is non-null after the null-to-global fallback");
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
pub fn initialize_file_variables(ctx: &ExecContext, file: FileId, reading: i32) {
    // Per-target variable storage now lives directly on the `FileNode`
    // (`variables` / `pat_variables` as `Vec<TargetVariable>`); there is no
    // per-file `variable_set_list` to allocate. The former chain wiring
    // (`(*l).next`, `next_is_parent`, the `global_setlist` link) belonged to the
    // legacy lookup machinery and is reconstructed on demand by the lookup layer
    // (see the slice5 boundary note below), so this routine's remaining job is to
    // perform the pattern-variable search and populate `FileNode.pat_variables`.
    let Some(node) = ctx.filenodes.get(file) else {
        return;
    };

    // Recurse into the parent first (matching the original ordering) without
    // holding this node's lock across the call.
    let parent = {
        let guard = node.lock().expect("file node poisoned");
        guard.parent
    };
    if let Some(parent_id) = parent {
        initialize_file_variables(ctx, parent_id, reading);
    }

    // The pattern-variable search only runs when building (not while reading)
    // and only once per file.
    let (need_search, name) = {
        let guard = node.lock().expect("file node poisoned");
        (reading == 0 && !guard.pat_searched, guard.name.clone())
    };
    if !need_search {
        return;
    }

    let mut collected: Vec<TargetVariable> = Vec::new();
    // SAFETY: the pattern-var list and the legacy definition helpers are still
    // the c2rust pointer-based machinery; only their inputs/outputs cross into
    // the idiomatic side here.
    unsafe {
        let mut name_c = name.clone();
        name_c.push(0);
        let name_ptr = name_c.as_ptr() as *const ::core::ffi::c_char;
        let targlen: size_t = name.len() as size_t;
        let mut p: *mut pattern_var =
            lookup_pattern_var(::core::ptr::null_mut::<pattern_var>(), name_ptr, targlen);
        if !p.is_null() {
            // Expand the matched pattern values inside a throwaway scope so the
            // legacy expanders behave exactly as before, then snapshot each
            // resulting `variable` into an owned `TargetVariable`.
            let global: *mut variable_set_list = current_variable_set_list;
            let scope = create_new_variable_set();
            current_variable_set_list = scope;
            loop {
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
                collected.push(target_variable_from_c(v as *const variable));
                p = lookup_pattern_var(p, name_ptr, targlen);
                if p.is_null() {
                    break;
                }
            }
            // Tear the throwaway scope back down: we own the snapshots now.
            pop_variable_scope();
            current_variable_set_list = global;
        }
    }

    // Commit the snapshot onto the FileNode's pattern-variable store.
    {
        let mut guard = node.lock().expect("file node poisoned");
        guard.pat_variables = collected;
        guard.pat_searched = true;
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
        reading_file = file_recipe_floc(file);
    }
}

/// The `reading_file` location to adopt while expanding in `file`'s context: the
/// file's recipe `fileinfo` when it has one with a real source name, else null.
/// Split out of [`install_file_context`] so its branch lives on its own.
///
/// # Safety
/// `file` must be a valid pointer for the call.
unsafe fn file_recipe_floc(file: *mut file) -> *const Floc {
    if !(*file).cmds.is_null() && !(*(*file).cmds).fileinfo.filenm.is_null() {
        &raw mut (*(*file).cmds).fileinfo
    } else {
        ::core::ptr::null::<Floc>()
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

/// Populate a freshly-created `variable_set` from a slice of [`TargetVariable`]
/// records (the FileNode's `variables` / `pat_variables`). Each entry is
/// inserted via `define_variable_in_set` and then has its full flag set copied
/// across so the resulting `variable` is faithful to the idiomatic record.
unsafe fn populate_set_from_targets(
    ctx: &crate::execctx::ExecContext,
    set: *mut variable_set,
    targets: &[TargetVariable],
) {
    for tv in targets {
        let mut name_buf = tv.name.clone();
        name_buf.push(0);
        let mut value_buf = tv.value.clone();
        value_buf.push(0);
        // For synthetic entries (no recorded source file) keep the floc fully
        // zeroed, matching the previous null-pointer path where the callee left
        // `fileinfo` untouched apart from clearing `filenm`.
        let mut floc_storage = Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        };
        let mut file_buf: Option<Vec<u8>> = None;
        if let Some(ref f) = tv.defined_in {
            let mut b = f.clone();
            b.push(0);
            floc_storage.filenm = strcache_add(ctx, b.as_ptr() as *const ::core::ffi::c_char);
            floc_storage.lineno = tv.defined_lineno;
            file_buf = Some(b);
        }
        // Always hand `define_variable_in_set` a pointer to the on-stack
        // `floc_storage`, never a null pointer. When the target variable has no
        // recorded source file, `floc_storage.filenm` is already null, so the
        // resulting `fileinfo` is equivalent to the previous null-pointer path
        // without ever flowing a null pointer into the callee's deref.
        let flocp: *const Floc = &raw const floc_storage;
        let v = define_variable_in_set(
            ctx,
            name_buf.as_ptr() as *const ::core::ffi::c_char,
            tv.name.len() as size_t,
            value_buf.as_ptr() as *const ::core::ffi::c_char,
            tv.origin as i32 as variable_origin,
            tv.recursive as i32,
            set,
            flocp,
        );
        let _ = file_buf;
        if !v.is_null() {
            (*v).set_flavor(tv.flavor as i32 as variable_flavor);
            (*v).set_export(tv.export as i32 as variable_export);
            (*v).set_append(tv.append as ::core::ffi::c_uint);
            (*v).set_conditional(tv.conditional as ::core::ffi::c_uint);
            (*v).set_per_target(tv.per_target as ::core::ffi::c_uint);
            (*v).set_special(tv.special as ::core::ffi::c_uint);
            (*v).set_exportable(tv.exportable as ::core::ffi::c_uint);
            (*v).set_private_var(tv.private_var as ::core::ffi::c_uint);
        }
    }
}

/// Build a transient C-ABI `variable_set_list` chain for a [`FileId`],
/// mirroring the per-file `variables` list the legacy `*mut file` carried.
///
/// The head set holds the file's own per-target variables (its `variables`
/// plus any pattern-specific `pat_variables`); its `next` link is the parent
/// file's chain (recursively) when the node has a `parent`, otherwise the
/// global set list — `next_is_parent` set so private variables stay private to
/// the file. The returned chain is owned by the caller and must be released
/// with [`free_file_setlist`] after use.
pub unsafe fn build_file_setlist(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
) -> *mut variable_set_list {
    let next: *mut variable_set_list;
    let (variables, pat_variables, parent) = {
        let Some(node) = ctx.filenodes.get(file) else {
            return &raw mut global_setlist;
        };
        let guard = node.lock().expect("file node poisoned");
        (
            guard.variables.clone(),
            guard.pat_variables.clone(),
            guard.parent,
        )
    };
    if let Some(parent_id) = parent {
        next = build_file_setlist(ctx, parent_id);
    } else {
        next = &raw mut global_setlist;
    }

    let set = xmalloc(::core::mem::size_of::<variable_set>() as size_t) as *mut variable_set;
    hash_init(
        &raw mut (*set).table,
        SMALL_SCOPE_VARIABLE_BUCKETS as ::core::ffi::c_ulong,
        Some(variable_hash_1),
        Some(variable_hash_2),
        Some(variable_hash_cmp),
    );
    // Pattern-specific variables first, then the explicit per-target ones (so
    // an explicit definition overrides a pattern one of the same name).
    populate_set_from_targets(ctx, set, &pat_variables);
    populate_set_from_targets(ctx, set, &variables);

    let setlist =
        xmalloc(::core::mem::size_of::<variable_set_list>() as size_t) as *mut variable_set_list;
    (*setlist).set = set;
    (*setlist).next = next;
    (*setlist).next_is_parent = 1;
    setlist
}

/// Snapshot every live variable in `set` into owned [`TargetVariable`] records
/// — the inverse of [`populate_set_from_targets`], used to write a file's
/// per-target variable definitions back onto its [`FileNode`] after the
/// pointer-based definition machinery has run.
pub unsafe fn snapshot_set_to_targets(set: *mut variable_set) -> Vec<TargetVariable> {
    let mut out: Vec<TargetVariable> = Vec::new();
    let Some(setr) = set.as_ref() else {
        return out;
    };
    let mut slot = setr.table.ht_vec as *mut *mut variable;
    let end = slot.offset(setr.table.ht_size as isize);
    while slot < end {
        let v = *slot;
        if !(v.is_null() || v as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
        {
            out.push(target_variable_from_c(v));
        }
        slot = slot.offset(1_i32 as isize);
    }
    out
}

/// Release a chain built by [`build_file_setlist`], stopping at the shared
/// `global_setlist` (which is process-wide and never freed).
pub unsafe fn free_file_setlist(mut list: *mut variable_set_list) {
    while !list.is_null() && list != &raw mut global_setlist {
        // SAFETY: `list` was just checked non-null; read `next` through a
        // checked reference before freeing the node.
        let next = list.as_ref().expect("list node is non-null").next;
        free_variable_set(list);
        list = next;
    }
}

/// FileId-based form of [`install_file_context`]: build a transient set list
/// for the target, make it current, and point `reading_file` at the recipe's
/// source location. Returns the previous list (to restore) via `oldlist`; when
/// `oldfloc` is non-null the previous `reading_file` is saved there.
pub unsafe fn install_file_context_id(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    oldlist: *mut *mut variable_set_list,
    oldfloc: *mut *const Floc,
) {
    *oldlist = current_variable_set_list;
    current_variable_set_list = build_file_setlist(ctx, file);
    if !oldfloc.is_null() {
        *oldfloc = reading_file;
        let recipe_floc: Option<(Vec<u8>, u64)> = ctx.filenodes.get(file).and_then(|node| {
            let guard = node.lock().expect("file node poisoned");
            guard
                .recipe
                .as_ref()
                .and_then(|r| r.defined_in.as_ref().map(|f| (f.clone(), r.defined_lineno)))
        });
        if let Some((mut fname, lineno)) = recipe_floc {
            fname.push(0);
            let filenm = strcache_add(ctx, fname.as_ptr() as *const ::core::ffi::c_char);
            RECIPE_READING_FLOC.with(|cell| {
                *cell.borrow_mut() = Floc {
                    filenm,
                    lineno,
                    offset: 0,
                };
                reading_file = cell.as_ptr();
            });
        } else {
            reading_file = ::core::ptr::null::<Floc>();
        }
    }
}

thread_local! {
    /// Backing storage for the `reading_file` `Floc` set by
    /// [`install_file_context_id`]; kept alive for the duration of the context.
    static RECIPE_READING_FLOC: ::std::cell::RefCell<Floc> = const {
        ::std::cell::RefCell::new(Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        })
    };
}

/// FileId-based form of [`restore_file_context`] that also frees the transient
/// set list built by [`install_file_context_id`].
pub unsafe fn restore_file_context_id(
    cur: *mut variable_set_list,
    oldlist: *mut variable_set_list,
    oldfloc: *const Floc,
) {
    free_file_setlist(cur);
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
        match remote_description(ctx) {
            None => b"\0" as *const u8 as *const ::core::ffi::c_char,
            Some(_) => b"-\0" as *const u8 as *const ::core::ffi::c_char,
        },
        match remote_description(ctx) {
            None => b"\0" as *const u8 as *const ::core::ffi::c_char,
            Some(desc) => desc.as_ptr(),
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
    file: Option<FileId>,
    recursive: i32,
) -> *mut *mut ::core::ffi::c_char {
    let set_list: *mut variable_set_list;
    // For a target context, build the transient per-file variable chain and
    // install it as current so the nested `recursively_expand_for_file` calls
    // (which expand in `current_variable_set_list`) see the file's scope. The
    // chain is freed before returning.
    let mut owned_list: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let saved_current: *mut variable_set_list = current_variable_set_list;
    if let Some(f) = file {
        owned_list = build_file_setlist(ctx, f);
        current_variable_set_list = owned_list;
    }
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
    let mut added_shell: i32 = ctx.shell_var.0.get().value.is_null() as i32;
    let mut found_makelevel: i32 = 0;
    let mut found_mflags: i32 = 0;
    let mut found_makeflags: i32 = 0;
    if file.is_none() {
        ENV_RECURSION.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
    }
    if recursive == 0 && crate::make_main::opt_jobserver_auth_present() {
        invalid = jobserver_get_invalid_auth(ctx);
    }
    if file.is_some() {
        set_list = owned_list;
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
        // SAFETY: `s` was just checked non-null; read its fields through a
        // checked reference rather than raw pointer derefs.
        let sr = s.as_ref().expect("set-list node is non-null");
        let set: *mut variable_set = sr.set;
        let islocal: i32 = (s == set_list) as i32;
        let isglobal: i32 = (set == &raw mut global_variable_set) as i32;
        // SAFETY: `set` came from a valid set-list node; read the table fields
        // through a checked reference.
        let set_ref = set.as_ref().expect("variable_set is non-null");
        v_slot = set_ref.table.ht_vec as *mut *mut variable;
        v_end = v_slot.offset(set_ref.table.ht_size as isize);
        while v_slot < v_end {
            // SAFETY: `v_slot` ranges over the table's slot array; read the slot
            // through a checked reference.
            let v: *mut variable = *v_slot.as_ref().expect("slot pointer in bounds");
            if !(v.is_null()
                || v as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
            {
                let evslot: *mut *mut variable;
                // SAFETY: the slot is occupied (checked above), so `v` is a
                // live, non-null variable; bind a checked reference.
                let vr = v.as_ref().expect("variable in occupied slot is non-null");
                if !(islocal == 0 && vr.private_var() as i32 != 0) {
                    evslot = hash_find_slot(&raw mut table, v as *const ::core::ffi::c_void)
                        as *mut *mut variable;
                    // SAFETY: `hash_find_slot` returns a valid slot pointer; read
                    // the stored entry through a checked reference.
                    let existing: *mut variable =
                        *evslot.as_ref().expect("hash slot pointer is non-null");
                    if existing.is_null()
                        || existing as *mut ::core::ffi::c_void
                            == hash_deleted_item as *mut ::core::ffi::c_void
                    {
                        // `v` is a live, non-null variable taken from an
                        // occupied hash slot just above.
                        if isglobal == 0 || should_export(vr) {
                            hash_insert_at(
                                &raw mut table,
                                v as *const ::core::ffi::c_void,
                                evslot as *const ::core::ffi::c_void,
                            );
                        }
                    } else {
                        // SAFETY: `existing` is the non-null entry already in the
                        // merged table; bind a checked reference to update it.
                        let er = existing.as_mut().expect("existing variable is non-null");
                        if er.export() as i32 == v_default as i32 {
                            er.set_export(vr.export() as variable_export);
                        }
                    }
                }
            }
            v_slot = v_slot.offset(1_i32 as isize);
        }
        s = sr.next;
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
                    // `current_variable_set_list` is already the file's scope
                    // (installed above when `file` is Some), so expand in that
                    // context with a null file pointer.
                    cp = recursively_expand_for_file(ctx, v_0, ::core::ptr::null_mut::<file>());
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
                *fresh10 = xstrdup(concat(&[(*v_0).name, b"=\0" as *const u8 as *const ::core::ffi::c_char, value]));
                free(cp as *mut ::core::ffi::c_void);
            }
        }
        v_slot = v_slot.offset(1_i32 as isize);
    }
    if added_shell == 0 {
        let shell_var = ctx.shell_var.0.get();
        let fresh11 = result;
        result = result.offset(1_i32 as isize);
        *fresh11 = xstrdup(concat(&[shell_var.name, b"=\0" as *const u8 as *const ::core::ffi::c_char, shell_var.value]));
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
    if file.is_none() {
        ENV_RECURSION.fetch_sub(1, ::std::sync::atomic::Ordering::Relaxed);
    }
    if !owned_list.is_null() {
        current_variable_set_list = saved_current;
        free_file_setlist(owned_list);
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
        // SAFETY: `define_variable_in_set` always returns a valid, non-null
        // `variable` pointer (it either upserts into the set or `xcalloc`s a new
        // entry, aborting on OOM). Bind a checked reference so these flag writes
        // go through a provably-valid reference.
        let vr = v
            .as_mut()
            .expect("define_variable_in_set returns a non-null variable pointer");
        vr.set_append(append as ::core::ffi::c_uint as ::core::ffi::c_uint);
        vr.set_conditional(conditional as ::core::ffi::c_uint as ::core::ffi::c_uint);
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
    ctx: &crate::execctx::ExecContext,
    str: *const ::core::ffi::c_char,
    var: *mut variable,
) -> *mut ::core::ffi::c_char {
    // The operator-detection state machine now lives in the typed AST layer
    // (`crate::parser`), which parses the line as a safe byte slice. Here we
    // only marshal the result back into the C-facing `struct variable`: the
    // name points into the original buffer (it is not copied or terminated),
    // and the returned pointer is the address just past the operator.
    let bytes = ::core::ffi::CStr::from_ptr(str).to_bytes();
    match crate::parser::assignment_ast(&ctx.db, bytes) {
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
    if parse_variable_definition(ctx, line, v).is_null() {
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
        fatal(ctx, &raw mut (*v).fileinfo, 0, b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char, &[]);
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
    // SAFETY: dereference `flocp` only behind the null check; `as_ref` yields a
    // checked reference so the read is provably valid.
    if let Some(floc) = flocp.as_ref() {
        v.fileinfo = *floc;
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
// Read-only table (populated once by a c2rust `.init_array` ctor and never
// mutated afterward): `const` avoids the `Sync` bound a `static` would need
// for the raw-pointer `name` fields, and drops the ctor machinery entirely —
// same treatment as job.rs's `default_shell`/`sh_chars`/`sh_cmds`.
const defined_vars: [defined_vars; 13] = [
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
/// Emit a "reference to undefined variable" warning for `name`, unless `name`
/// is one of the built-in always-defined variables in the `defined_vars`
/// table, or the warning is inactive.
pub fn warn_undefined(ctx: &crate::execctx::ExecContext, name: &[u8]) {
    if warning::is_active(Type::UndefinedVar) {
        // SAFETY: `defined_vars` is a NUL-terminated table of built-in
        // variable names. We only read it here, walking until the sentinel
        // null `name`, and compare each entry's bytes against `name`.
        let is_builtin = unsafe {
            let mut dp = defined_vars.as_ptr();
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
            dp = defined_vars.as_ptr();
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
            let mut dp = defined_vars.as_ptr();
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
            let mut dp = defined_vars.as_ptr();
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
/// Print the per-target variable set of `file` (the automatic variables), each
/// line prefixed with `"# "`. Reads from [`FileNode::variables`] rather than the
/// legacy per-file `variable_set_list`.
pub fn print_file_variables(ctx: &ExecContext, file: FileId) {
    let Some(node) = ctx.filenodes.get(file) else {
        return;
    };
    let vars = {
        let guard = node.lock().expect("file node poisoned");
        guard.variables.clone()
    };
    let prefix = b"# \0";
    vars.iter()
        .filter(|tv| tv.origin == VarOrigin::Automatic)
        .for_each(|tv| {
            // SAFETY: `cv` borrows the NUL-terminated buffers held in the tuple,
            // which outlive the `print_variable` call below.
            let (cv, _n, _v, _f) = c_variable_from_target(tv);
            unsafe {
                print_variable(
                    &raw const cv as *const ::core::ffi::c_void,
                    prefix.as_ptr() as *mut ::core::ffi::c_void,
                );
            }
        });
    // NOTE (slice5 boundary): the legacy path also emitted hash-table stats for
    // the per-file `variable_set`; the `FileNode` store has no hash table, so
    // those stats lines are intentionally dropped.
}
/// Print the per-target variable set of `file` (the non-automatic variables),
/// each line prefixed with `"<target>: "`. Reads from [`FileNode::variables`].
pub fn print_target_variables(ctx: &ExecContext, file: FileId) {
    let Some(node) = ctx.filenodes.get(file) else {
        return;
    };
    let (vars, name) = {
        let guard = node.lock().expect("file node poisoned");
        (guard.variables.clone(), guard.name.clone())
    };
    // Prefix each variable line with "<target>: ".
    let mut prefix: Vec<u8> = Vec::with_capacity(name.len() + 3);
    prefix.extend_from_slice(&name);
    prefix.extend_from_slice(b": \0");
    vars.iter()
        .filter(|tv| tv.origin != VarOrigin::Automatic)
        .for_each(|tv| {
            // SAFETY: `cv` borrows the NUL-terminated buffers held in the tuple,
            // which outlive the `print_variable` call below.
            let (cv, _n, _v, _f) = c_variable_from_target(tv);
            unsafe {
                print_variable(
                    &raw const cv as *const ::core::ffi::c_void,
                    prefix.as_mut_ptr() as *mut ::core::ffi::c_void,
                );
            }
        });
}

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
    use super::initialize_file_variables;
    use crate::file::enter_file;
    use std::sync::Mutex;

    // The pattern-var database / global sets are process-wide; serialize so
    // these tests don't race other variable-layer tests.
    static GLOBAL_VARS_LOCK: Mutex<()> = Mutex::new(());

    /// With `reading != 0` the pattern-variable scan is skipped, so the call is
    /// a no-op on a fresh node: `pat_searched` stays clear.
    #[test]
    fn reading_nonzero_skips_pattern_search() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        let f = enter_file(&ctx, b"ifv_probe_target");
        initialize_file_variables(&ctx, f, 1);
        let node = ctx.filenodes.get(f).expect("interned");
        assert!(
            !node.lock().unwrap().pat_searched,
            "reading!=0 must not run (or record) the pattern search"
        );
    }

    /// With `reading == 0`, the pattern-variable search arm runs once. A target
    /// name that matches no defined pattern variable yields no match, but the
    /// search is still recorded by latching `pat_searched`.
    #[test]
    fn reading_zero_runs_pattern_search_and_latches() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        let f = enter_file(&ctx, b"ifv_reading0_unmatched_probe");
        {
            let node = ctx.filenodes.get(f).expect("interned");
            assert!(!node.lock().unwrap().pat_searched, "starts un-searched");
        }
        initialize_file_variables(&ctx, f, 0);
        let node = ctx.filenodes.get(f).expect("interned");
        assert!(
            node.lock().unwrap().pat_searched,
            "pattern search ran and was recorded"
        );
    }

    /// A second `reading == 0` call is a no-op once `pat_searched` is set: the
    /// search is performed at most once per file.
    #[test]
    fn pattern_search_runs_at_most_once() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        let f = enter_file(&ctx, b"ifv_probe_once");
        initialize_file_variables(&ctx, f, 0);
        initialize_file_variables(&ctx, f, 0);
        let node = ctx.filenodes.get(f).expect("interned");
        assert!(node.lock().unwrap().pat_searched);
    }

    /// A file with a `parent` recurses into the parent first, latching the
    /// parent's `pat_searched` too (the non-null-`parent` arm).
    #[test]
    fn parent_chains_into_parent_scope() {
        let _g = GLOBAL_VARS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        let parent = enter_file(&ctx, b"ifv_parent_probe");
        let child = enter_file(&ctx, b"ifv_child_probe");
        ctx.filenodes
            .get(child)
            .unwrap()
            .lock()
            .unwrap()
            .parent = Some(parent);

        initialize_file_variables(&ctx, child, 0);

        assert!(
            ctx.filenodes.get(parent).unwrap().lock().unwrap().pat_searched,
            "parent's pattern search ran via recursion"
        );
        assert!(
            ctx.filenodes.get(child).unwrap().lock().unwrap().pat_searched,
            "child's pattern search ran"
        );
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

#[cfg(test)]
mod variable_cmp_tests {
    use super::*;
    use ::core::cmp::Ordering;

    /// Verbatim preservation of the original raw-pointer `variable_hash_cmp`
    /// callback, retained as a differential oracle per the project rule for
    /// swapping an unsafe implementation for a safe one.
    ///
    /// SAFETY: callers pass pointers to live `variable` values whose `name`
    /// fields address `length` readable bytes — the same contract the hash
    /// table handed the original callback.
    unsafe fn variable_hash_cmp_unsafe_oracle(
        xv: *const ::core::ffi::c_void,
        yv: *const ::core::ffi::c_void,
    ) -> i32 {
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

    /// Build a throwaway `variable` whose `name`/`length` denote `bytes`.
    fn var_for(bytes: &[u8]) -> variable {
        // SAFETY: a zeroed `variable` is a valid (inert) value; the tests only
        // read its `name`/`length` fields.
        let mut v: variable = unsafe { ::core::mem::zeroed() };
        v.name = bytes.as_ptr() as *mut ::core::ffi::c_char;
        v.length = bytes.len() as ::core::ffi::c_uint;
        v
    }

    /// Drive the production callback with two name slices.
    fn callback(x: &[u8], y: &[u8]) -> i32 {
        let vx = var_for(x);
        let vy = var_for(y);
        // SAFETY: both variables address live byte slices of the stated length.
        unsafe {
            variable_hash_cmp(
                &raw const vx as *const ::core::ffi::c_void,
                &raw const vy as *const ::core::ffi::c_void,
            )
        }
    }

    /// Drive the verbatim oracle with two name slices.
    fn oracle(x: &[u8], y: &[u8]) -> i32 {
        let vx = var_for(x);
        let vy = var_for(y);
        // SAFETY: both variables address live byte slices of the stated length.
        unsafe {
            variable_hash_cmp_unsafe_oracle(
                &raw const vx as *const ::core::ffi::c_void,
                &raw const vy as *const ::core::ffi::c_void,
            )
        }
    }

    const SAMPLES: &[&[u8]] = &[
        b"",
        b"a",
        b"b",
        b"A",
        b"ab",
        b"ba",
        b"abc",
        b"abd",
        b"CC",
        b"CXX",
        b"foo",
        b"foobar",
        b"\xff",
        b"\x00",
    ];

    #[test]
    fn callback_matches_oracle_in_sign() {
        for &x in SAMPLES {
            for &y in SAMPLES {
                assert_eq!(
                    callback(x, y).signum(),
                    oracle(x, y).signum(),
                    "sign mismatch for {x:?} vs {y:?}",
                );
            }
        }
    }

    #[test]
    fn pure_core_matches_callback_sign() {
        for &x in SAMPLES {
            for &y in SAMPLES {
                let expected = match variable_cmp(x, y) {
                    Ordering::Less => -1,
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                };
                assert_eq!(callback(x, y).signum(), expected, "{x:?} vs {y:?}");
            }
        }
    }

    #[test]
    fn orders_by_length_then_bytes() {
        // Shorter names sort first, whatever their bytes.
        assert_eq!(variable_cmp(b"zzz", b"aaaa"), Ordering::Less);
        // Equal length falls back to lexicographic byte order.
        assert_eq!(variable_cmp(b"abc", b"abd"), Ordering::Less);
        assert_eq!(variable_cmp(b"abd", b"abc"), Ordering::Greater);
        assert_eq!(variable_cmp(b"abc", b"abc"), Ordering::Equal);
    }

    #[test]
    fn is_antisymmetric() {
        for &x in SAMPLES {
            for &y in SAMPLES {
                assert_eq!(
                    variable_cmp(x, y),
                    variable_cmp(y, x).reverse(),
                    "{x:?} vs {y:?}",
                );
            }
        }
    }
}

#[cfg(test)]
mod file_context_coverage_tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    // `current_variable_set_list`/`reading_file` are process globals; serialize
    // this test against itself and save/restore them so it can't perturb others.
    static CTX_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    /// Install a (default) file's context — swapping `current_variable_set_list`
    /// and `reading_file` — then restore the saved values. Exercises both
    /// `install_file_context` and `restore_file_context`.
    #[test]
    fn install_then_restore_file_context_roundtrips_globals() {
        let _g = CTX_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            let saved_list = current_variable_set_list;
            let saved_reading = crate::read::reading_file;

            let mut f = crate::file::File::default();
            let mut oldlist: *mut variable_set_list = ::core::ptr::null_mut();
            let mut oldfloc: *const Floc = ::core::ptr::null();
            install_file_context(&raw mut f, &raw mut oldlist, &raw mut oldfloc);
            // The saved-out list is whatever was active before the swap.
            assert_eq!(oldlist, saved_list);
            restore_file_context(oldlist, oldfloc);
            let current_list = current_variable_set_list;
            assert_eq!(current_list, saved_list);

            // Fully restore the globals regardless of intermediate state.
            current_variable_set_list = saved_list;
            crate::read::reading_file = saved_reading;
        }
    }
}
