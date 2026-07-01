pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __pid_t, __syscall_slong_t, __time_t, __uid_t, pid_t, ptrdiff_t, size_t, ssize_t, uintmax_t,
};
use crate::file::{File, VariableSet, VariableSetList};
use crate::misc::{
    alpha_cmp, end_of_token, find_next_token, make_lltoa, next_token, xmalloc, xstrndup,
};
use crate::output::FmtArg;
use crate::stdio::FILE;
use crate::strcache::strcache_add;
use c2rust_bitfields;
use libc::{__errno_location, abort, close, free, pipe, printf, remove, sprintf, strerror, strstr};
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
extern "C" {
    fn read(__fd: i32, __buf: *mut ::core::ffi::c_void, __nbytes: size_t) -> ssize_t;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> i32;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}

/// RAII owner for a string produced by `expand_argument`.
///
/// `expand_argument` hands back a freshly `malloc`ed, NUL-terminated buffer
/// that the caller must release. Wrapping it here lets `Drop` free the buffer
/// on every exit path, replacing the hand-written `expand_argument(...) ...
/// free(...)` ownership pairs the `$(if)`/`$(or)`/`$(and)` builtins used.
struct ExpandedArg(*mut ::core::ffi::c_char);

impl ExpandedArg {
    /// Expand `arg`, stopping at `end` (or at the NUL when `end` is null).
    unsafe fn new(
        ctx: &crate::execctx::ExecContext,
        arg: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> Self {
        ExpandedArg(expand_argument(ctx, arg, end))
    }

    /// Take ownership of an already-expanded, `malloc`ed buffer (e.g. from
    /// `allocated_expand_string_for_file`).
    unsafe fn from_raw(ptr: *mut ::core::ffi::c_char) -> Self {
        ExpandedArg(ptr)
    }

    /// Borrow the underlying NUL-terminated buffer.
    fn as_ptr(&self) -> *mut ::core::ffi::c_char {
        self.0
    }
}

impl Drop for ExpandedArg {
    fn drop(&mut self) {
        unsafe { free(self.0 as *mut ::core::ffi::c_void) }
    }
}

pub type gmk_func_ptr = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        ::core::ffi::c_uint,
        *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char,
>;
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> i32>;
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
use crate::floc::Floc;

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
use crate::expand::{
    allocated_expand_string_for_file, expand_argument, expand_string_buf, expand_variable_output,
    expanding_var, install_variable_buffer, restore_variable_buffer, variable_buffer_output,
};
pub use crate::file::nameseq;
use crate::hash::{hash_find_item, hash_init, hash_insert, hash_load, jhash};
use rustc_hash::FxHashMap;
pub use crate::job::childbase;
use crate::job::{child_execute_job, construct_command_argv, free_childbase, reap_children};
use crate::make_main::{db_level, starting_directory, stopchar_map};
pub use crate::output::output;
use crate::output::{error, fatal, out_of_memory, output_context, outputs};
use crate::posixos::fd_noinherit;
use crate::read::{eval_buffer, find_percent, parse_file_seq, reading_file};
use crate::variable::{
    current_variable_set_list, define_variable_in_set, lookup_variable, pop_variable_scope,
    push_new_variable_scope, target_environment, warn_undefined,
};
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct function_table_entry {
    pub fptr: C2RustUnnamed,
    pub name: *const ::core::ffi::c_char,
    pub len: ::core::ffi::c_uchar,
    pub minimum_args: ::core::ffi::c_uchar,
    pub maximum_args: ::core::ffi::c_uchar,
    #[bitfield(name = "expand_args", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "alloc_fn", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "adds_command", ty = "::core::ffi::c_uint", bits = "2..=2")]
    pub expand_args_alloc_fn_adds_command: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub func_ptr: Option<
        unsafe fn(
            &crate::execctx::ExecContext,
            *mut ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub alloc_func_ptr: gmk_func_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_word {
    pub chain: *mut a_word,
    pub str_0: *mut ::core::ffi::c_char,
    pub length: size_t,
    pub matched: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_pattern {
    pub str_0: *mut ::core::ffi::c_char,
    pub percent: *mut ::core::ffi::c_char,
    pub length: size_t,
}
pub const EOF: i32 = -1_i32;
pub const ENOENT: i32 = 2;
pub const EINTR: i32 = 4;
pub const ERANGE: i32 = 34;
pub const PATH_MAX: i32 = 4096_i32;
pub const GET_PATH_MAX: i32 = PATH_MAX;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_NUL: i32 = 0x1_i32;
pub const MAP_BLANK: i32 = 0x2_i32;
pub const MAP_NEWLINE: i32 = 0x4_i32;
pub const MAP_VARSEP: i32 = 0x80_i32;
pub const MAP_DOT: i32 = 0x200_i32;
pub const MAP_COMMA: i32 = 0x400_i32;
pub const MAP_DIRSEP: i32 = 0x8000_i32;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
}
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const INTSTR_LENGTH: usize = 53_usize
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22_usize)
    .wrapping_add(3_usize);
pub const EXP_COUNT_BITS: i32 = 15;
pub const EXP_COUNT_MAX: i32 = ((1) << EXP_COUNT_BITS) - 1;
unsafe fn function_table_entry_hash_1(keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: ::core::ffi::c_ulong = 0;
    let _key_: *const ::core::ffi::c_uchar = (*key).name as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash(::core::slice::from_raw_parts(
        _key_,
        (*key).len as usize,
    )) as ::core::ffi::c_ulong);
    _result_
}
fn function_table_entry_hash_2(keyv: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _key: *const function_table_entry = keyv as *const function_table_entry;
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe fn function_table_entry_hash_cmp(
    xv: *const ::core::ffi::c_void,
    yv: *const ::core::ffi::c_void,
) -> i32 {
    let x: *const function_table_entry = xv as *const function_table_entry;
    let y: *const function_table_entry = yv as *const function_table_entry;
    let result: i32 = (*x).len as i32 - (*y).len as i32;
    if result != 0 {
        return result;
    }
    if (*x).name == (*y).name {
        0
    } else {
        memcmp(
            (*x).name as *const ::core::ffi::c_void,
            (*y).name as *const ::core::ffi::c_void,
            (*x).len as size_t,
        )
    }
}
static mut function_table: hash_table = hash_table {
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
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn subst_expand(
    mut o: *mut ::core::ffi::c_char,
    text: *const ::core::ffi::c_char,
    subst: *const ::core::ffi::c_char,
    replace: *const ::core::ffi::c_char,
    slen: size_t,
    rlen: size_t,
    by_word: i32,
) -> *mut ::core::ffi::c_char {
    // `text` is NUL-terminated; view it as a byte slice so the scan runs over
    // indices instead of walking raw pointers. `subst`/`replace` are emitted
    // straight to the output buffer, which is the genuine FFI boundary.
    let text_bytes = ::core::ffi::CStr::from_ptr(text).to_bytes();
    let subst_bytes = ::core::slice::from_raw_parts(subst as *const u8, slen);
    if slen == 0 && by_word == 0 {
        o = variable_buffer_output(o, text, text_bytes.len());
        if rlen > 0 {
            o = variable_buffer_output(o, replace, rlen);
        }
        return o;
    }
    // `ti` is the offset of the C cursor `t` within `text_bytes`.
    let mut ti: usize = 0;
    loop {
        // `p` is the offset of the next match (or token end, in word mode).
        let p: usize;
        if by_word != 0 && slen == 0 {
            // p = end_of_token(next_token(t)); recover its offset by address
            // difference (an accepted span computation, not pointer arithmetic).
            let t_ptr = text_bytes[ti..].as_ptr() as *const ::core::ffi::c_char;
            let nt = next_token(t_ptr);
            // `nt` points within `text_bytes`; recover its offset and scan only
            // the already-bounded remaining slice for the token end. Feeding the
            // bounded `text_bytes[nt_off..]` (which ends at the NUL) avoids a
            // fresh `strlen` over the whole suffix per token, keeping this
            // per-token loop O(n) overall instead of O(n^2).
            let nt_off = nt as usize - text_bytes.as_ptr() as usize;
            p = nt_off + end_of_token(&text_bytes[nt_off..]);
        } else {
            // p = strstr(t, subst)
            match text_bytes[ti..]
                .windows(subst_bytes.len())
                .position(|w| w == subst_bytes)
            {
                Some(rel) => p = ti + rel,
                None => {
                    o = variable_buffer_output(
                        o,
                        text_bytes[ti..].as_ptr() as *const ::core::ffi::c_char,
                        text_bytes.len() - ti,
                    );
                    return o;
                }
            }
        }
        if p > ti {
            o = variable_buffer_output(
                o,
                text_bytes[ti..].as_ptr() as *const ::core::ffi::c_char,
                p - ti,
            );
        }
        // Whole-word boundary test (word mode only): keep the original `subst`
        // when the match is not a standalone word — preceded by a non-blank
        // (and not at the start), or followed by a non-blank, non-terminator.
        let prev_breaks = p > 0 && !stop_set(text_bytes[p - 1], MAP_BLANK | MAP_NEWLINE);
        let after = text_bytes.get(p + slen).copied().unwrap_or(0);
        let after_breaks = !stop_set(after, MAP_BLANK | MAP_NEWLINE | MAP_NUL);
        if by_word != 0 && (prev_breaks || after_breaks) {
            o = variable_buffer_output(o, subst, slen);
        } else if rlen > 0 {
            o = variable_buffer_output(o, replace, rlen);
        }
        // Advance past the match; stop when the cursor reaches the NUL.
        ti = p + slen;
        if ti >= text_bytes.len() {
            break;
        }
    }
    o
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn patsubst_expand_pat(
    mut o: *mut ::core::ffi::c_char,
    mut text: *const ::core::ffi::c_char,
    pattern: *const ::core::ffi::c_char,
    replace: *const ::core::ffi::c_char,
    pattern_percent: *const ::core::ffi::c_char,
    replace_percent: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut len: size_t = 0;
    let mut doneany: i32 = 0;
    // Replacement halves around the '%'. `replace`/`replace_percent` are emitted
    // straight to the output buffer; only their split lengths are needed here.
    // The '%' offset is an address difference, not pointer arithmetic.
    let replace_prepercent_len: usize;
    let replace_postpercent_len: usize;
    if !replace_percent.is_null() {
        replace_prepercent_len = (replace_percent as usize - replace as usize) - 1;
        replace_postpercent_len = strlen(replace_percent) as usize;
    } else {
        replace_prepercent_len = strlen(replace) as usize;
        replace_postpercent_len = 0;
    }
    if pattern_percent.is_null() {
        return subst_expand(
            o,
            text,
            pattern,
            replace,
            strlen(pattern) as size_t,
            strlen(replace) as size_t,
            1,
        );
    }
    // Split the pattern into its prefix and suffix around the '%' and match
    // each token against them by slice comparison instead of walking pointers.
    let pat = ::core::ffi::CStr::from_ptr(pattern).to_bytes();
    let prepercent_len = (pattern_percent as usize - pattern as usize) - 1;
    let pat_prefix = &pat[..prepercent_len];
    let pat_suffix = &pat[prepercent_len + 1..];
    let postpercent_len = pat_suffix.len();
    loop {
        let t = find_next_token(&raw mut text, &raw mut len);
        if t.is_null() {
            break;
        }
        let tok = ::core::slice::from_raw_parts(t as *const u8, len as usize);
        // A token matches `prefix % suffix` when it is long enough and both
        // ends line up; `&&` short-circuits so the slices stay in bounds.
        let matched = tok.len() >= prepercent_len + postpercent_len
            && tok[..prepercent_len] == *pat_prefix
            && tok[tok.len() - postpercent_len..] == *pat_suffix;
        if matched {
            o = variable_buffer_output(o, replace, replace_prepercent_len);
            if !replace_percent.is_null() {
                // The stem is what '%' captured: the token minus prefix/suffix.
                let stem = &tok[prepercent_len..tok.len() - postpercent_len];
                o = variable_buffer_output(
                    o,
                    stem.as_ptr() as *const ::core::ffi::c_char,
                    stem.len(),
                );
                o = variable_buffer_output(o, replace_percent, replace_postpercent_len);
            }
        } else {
            o = variable_buffer_output(o, t, len);
        }
        if !matched
            || replace_prepercent_len > 0
            || !replace_percent.is_null() && len.wrapping_add(replace_postpercent_len) > 0
        {
            o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
            doneany = 1;
        }
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    o
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn patsubst_expand(
    o: *mut ::core::ffi::c_char,
    text: *const ::core::ffi::c_char,
    pattern: *mut ::core::ffi::c_char,
    replace: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut pattern_percent: *const ::core::ffi::c_char = find_percent(pattern);
    let mut replace_percent: *const ::core::ffi::c_char = find_percent(replace);
    if !replace_percent.is_null() {
        replace_percent = replace_percent.offset(1_i32 as isize);
    }
    if !pattern_percent.is_null() {
        pattern_percent = pattern_percent.offset(1_i32 as isize);
    }
    patsubst_expand_pat(o, text, pattern, replace, pattern_percent, replace_percent)
}
unsafe extern "C" fn lookup_function(s: *const ::core::ffi::c_char) -> *const function_table_entry {
    let mut function_table_entry_key: function_table_entry = function_table_entry {
        fptr: C2RustUnnamed { func_ptr: None },
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        len: 0,
        minimum_args: 0,
        maximum_args: 0,
        expand_args_alloc_fn_adds_command: [0; 1],
        c2rust_padding: [0; 4],
    };
    let mut e: *const ::core::ffi::c_char = s;
    while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
        .offset(*e as ::core::ffi::c_uchar as isize) as i32
        & 0x2000_i32
        != 0
    {
        e = e.offset(1_i32 as isize);
    }
    if e == s
        || !(*(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
            .offset(*e as ::core::ffi::c_uchar as isize) as i32
            & (0x1_i32 | (0x2_i32 | 0x4_i32))
            != 0)
    {
        return ::core::ptr::null::<function_table_entry>();
    }
    function_table_entry_key.name = s;
    function_table_entry_key.len = e.offset_from(s) as ::core::ffi::c_long as ::core::ffi::c_uchar;
    hash_find_item(
        &raw mut function_table,
        &raw mut function_table_entry_key as *const ::core::ffi::c_void,
    ) as *const function_table_entry
}
/// Does `s` match a `%`-pattern whose literal text before the `%` is
/// `prefix` and whose literal text after it is `suffix`? The `%` stands for
/// any (possibly empty) run, so `s` must be at least `prefix.len() +
/// suffix.len()` bytes long and bookended by the two literals. The length
/// guard matters when the literals would otherwise overlap (e.g. pattern
/// `ab%bc` must not match `abc`).
fn pattern_matches_parts(prefix: &[u8], suffix: &[u8], s: &[u8]) -> bool {
    s.len() >= prefix.len() + suffix.len() && s.starts_with(prefix) && s.ends_with(suffix)
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn pattern_matches(
    mut pattern: *const ::core::ffi::c_char,
    mut percent: *const ::core::ffi::c_char,
    str: *const ::core::ffi::c_char,
) -> i32 {
    let s = ::core::ffi::CStr::from_ptr(str).to_bytes();
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    if percent.is_null() {
        let len: size_t = (strlen(pattern) as size_t).wrapping_add(1);
        alloca_allocations.push(::std::vec::from_elem(0, len as usize));
        let new_chars: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            new_chars as *mut ::core::ffi::c_void,
            pattern as *const ::core::ffi::c_void,
            len as size_t,
        );
        // `find_percent` collapses backslash escapes in place, so it needs the
        // writable copy made above.
        percent = find_percent(new_chars);
        if percent.is_null() {
            // No wildcard: the pattern must equal `str` outright.
            return (::core::ffi::CStr::from_ptr(new_chars).to_bytes() == s) as i32;
        }
        pattern = new_chars;
    }
    // `percent` points at the `%` inside `pattern`; split the pattern's byte
    // view there (the `% ` itself is dropped) instead of forming sub-pointers.
    let pattern_bytes = ::core::ffi::CStr::from_ptr(pattern).to_bytes();
    let percent_idx = percent as usize - pattern as usize;
    let prefix = &pattern_bytes[..percent_idx];
    let suffix = &pattern_bytes[percent_idx + 1..];
    pattern_matches_parts(prefix, suffix, s) as i32
}
/// The byte offset of the next top-level `,` argument separator within
/// `bytes`, or `None` if there is none. `startparen`/`endparen` track nesting
/// so commas inside a balanced paren group are skipped; an unbalanced closing
/// paren ends the scan with `None`, matching the original argument splitter.
fn find_next_argument(startparen: u8, endparen: u8, bytes: &[u8]) -> Option<usize> {
    let mut count: i32 = 0;
    for (i, &c) in bytes.iter().enumerate() {
        // The explicit char checks are exact, so no `stop_set` structural
        // pre-filter is needed: a non-structural byte simply matches no arm.
        if c == startparen {
            count += 1;
        } else if c == endparen {
            count -= 1;
            if count < 0 {
                return None;
            }
        } else if c == b',' && count == 0 {
            return Some(i);
        }
    }
    None
}
/// Glob `line` (make's `$(wildcard ...)`) and return the matched names joined
/// by single spaces as an owned byte buffer (no trailing space or NUL).
///
/// # Safety
///
/// `line` must be valid for [`parse_file_seq`]: a writable, NUL-terminated C
/// string, which this consumes as the glob input.
pub unsafe fn string_glob(ctx: &crate::execctx::ExecContext, mut line: *mut ::core::ffi::c_char) -> Vec<u8> {
    // 0x1 = MAP_NUL stopmap; 0x1|0x10|0x8 = PARSEFS_NOSTRIP|PARSEFS_NOCACHE|PARSEFS_EXISTS
    let chain = parse_file_seq(
        ctx,
        &raw mut line,
        ::core::mem::size_of::<nameseq>() as size_t,
        0x1_i32,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x1_i32 | 0x10_i32 | 0x8_i32,
    );
    join_glob_names(&chain)
}

/// Join glob-matched names into an owned buffer, replacing the pre-conversion
/// `static mut` scratch buffer that was grown in place through raw pointer
/// arithmetic. Reproduces make's exact byte production: each name is followed by
/// a space and the final trailing space is dropped (the C code overwrote it with
/// the terminating NUL), so a leading empty name still yields a leading space.
/// `ParsedName::name` already carries the observable bytes with no NUL. Split
/// out so the behavior oracle in tests can exercise it without the filesystem.
fn join_glob_names(chain: &[crate::read::ParsedName]) -> Vec<u8> {
    let mut names = Vec::new();
    for pn in chain {
        names.extend_from_slice(&pn.name);
        names.push(b' ');
    }
    names.pop();
    names
}

#[cfg(test)]
mod string_glob_tests {
    use super::join_glob_names;
    use crate::read::ParsedName;

    /// Behavior oracle: the pre-conversion accumulation, reproduced faithfully —
    /// grow a scratch buffer, append each name followed by a space, then
    /// overwrite the final space with a NUL — returning the observable bytes (up
    /// to that NUL) that `func_wildcard` copied out. AGENTS.md: keep the original
    /// behavior as a `#[cfg(test)]` oracle and assert the safe version agrees.
    fn join_glob_names_unsafe_oracle(chain: &[ParsedName]) -> Vec<u8> {
        let mut length: usize = 100;
        let mut buf = vec![0u8; length];
        let mut idx: usize = 0;
        for pn in chain {
            let len = pn.name.len();
            if idx + len + 1 > length {
                length += (len + 1) * 2;
                buf.resize(length, 0);
            }
            buf[idx..idx + len].copy_from_slice(&pn.name);
            idx += len;
            buf[idx] = b' ';
            idx += 1;
        }
        if idx == 0 {
            buf[0] = 0;
        } else {
            buf[idx - 1] = 0;
        }
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        buf[..end].to_vec()
    }

    fn pn(name: &[u8]) -> ParsedName {
        ParsedName {
            name: name.to_vec(),
            wait: false,
        }
    }

    #[test]
    fn matches_unsafe_oracle() {
        let cases: &[Vec<ParsedName>] = &[
            vec![],
            vec![pn(b"a")],
            vec![pn(b"foo"), pn(b"bar"), pn(b"baz")],
            // Long enough to force the oracle's scratch buffer to grow.
            vec![pn(&[b'x'; 250]), pn(b"y")],
            // Empty names exercise the leading/interior-space edge cases.
            vec![pn(b""), pn(b"z")],
            vec![pn(b"a"), pn(b""), pn(b"b")],
        ];
        for chain in cases {
            assert_eq!(join_glob_names(chain), join_glob_names_unsafe_oracle(chain));
        }
        // Exact bytes for the common shapes.
        assert_eq!(join_glob_names(&[pn(b"foo"), pn(b"bar")]), b"foo bar");
        assert!(join_glob_names(&[]).is_empty());
    }
}
unsafe fn func_patsubst(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    o = patsubst_expand(
        o,
        *argv.offset(2_i32 as isize),
        *argv.offset(0_i32 as isize),
        *argv.offset(1_i32 as isize),
    );
    o
}
/// make's `$(join list1,list2)`: concatenate the i-th word of `list1` with the
/// i-th word of `list2`, for every row up to the longer list, space-separated
/// with no trailing space. A missing word on either side contributes nothing,
/// so `$(join a b,1 2 3)` is `a1 b2 3`. Pure over the two whitespace-token
/// lists, replacing the paired `find_next_token` pointer walks with `tokens`.
fn join_lists(list1: &[u8], list2: &[u8]) -> Vec<u8> {
    // Stream both token iterators once in lockstep, as the C loop did, rather
    // than materializing the word lists.
    let mut it1 = tokens(list1);
    let mut it2 = tokens(list2);
    let mut out = Vec::new();
    loop {
        let (t, p) = (it1.next(), it2.next());
        if t.is_none() && p.is_none() {
            break;
        }
        if let Some(t) = t {
            out.extend_from_slice(t);
        }
        if let Some(p) = p {
            out.extend_from_slice(p);
        }
        out.push(b' ');
    }
    // Drop the trailing separator the C loop trimmed via `o -= 1` (no-op when
    // both lists are empty and nothing was emitted).
    out.pop();
    out
}
fn func_join(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // A safe `fn` still coerces to the function table's `unsafe fn` pointer; the
    // only unsafe is the FFI at the edges. SAFETY: the dispatcher passes an
    // `argv` of at least `maximum_args` NUL-terminated C strings (`join` has
    // min = max = 2), so `argv[0]`/`argv[1]` are valid.
    let (list1, list2) = unsafe {
        (
            ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes(),
            ::core::ffi::CStr::from_ptr(*argv.offset(1_i32 as isize)).to_bytes(),
        )
    };
    let joined = join_lists(list1, list2);
    if !joined.is_empty() {
        // SAFETY: `o` is the caller's variable-buffer output cursor and `joined`
        // is a valid byte buffer of the given length.
        o = unsafe {
            variable_buffer_output(
                o,
                joined.as_ptr() as *const ::core::ffi::c_char,
                joined.len() as size_t,
            )
        };
    }
    o
}

#[cfg(test)]
mod func_join_tests {
    use super::join_lists;

    /// Tokenize `list` with the actual pre-conversion pointer walk
    /// (`misc::find_next_token`) over a NUL-terminated buffer. Using the *old*
    /// tokenizer — not `tokens` — is what makes the oracle a genuine check: the
    /// test then also proves `tokens` agrees with `find_next_token` for these
    /// inputs, not just that the pairing logic is self-consistent.
    unsafe fn tokens_via_pointer_walk(list: &[u8]) -> Vec<Vec<u8>> {
        let cbuf: Vec<u8> = list.iter().copied().chain(::core::iter::once(0)).collect();
        let mut iter: *const ::core::ffi::c_char = cbuf.as_ptr() as *const ::core::ffi::c_char;
        let mut words = Vec::new();
        loop {
            let mut len: usize = 0;
            let p = crate::misc::find_next_token(&raw mut iter, &raw mut len);
            if p.is_null() {
                break;
            }
            words.push(::core::slice::from_raw_parts(p as *const u8, len).to_vec());
        }
        words
    }

    /// Behavior oracle: the pre-conversion `find_next_token` loop, reproduced
    /// over the two lists tokenized by the old pointer walk — advance each list
    /// independently, emit this row's word1 then word2, append a space while
    /// either side produced a word, and trim the final space. Structurally
    /// mirrors the C control flow (not `join_lists`' streaming form).
    fn join_lists_oracle(list1: &[u8], list2: &[u8]) -> Vec<u8> {
        let w1 = unsafe { tokens_via_pointer_walk(list1) };
        let w2 = unsafe { tokens_via_pointer_walk(list2) };
        let mut out = Vec::new();
        let (mut i1, mut i2) = (0usize, 0usize);
        let mut doneany = false;
        loop {
            let t = w1.get(i1);
            if t.is_some() {
                i1 += 1;
            }
            let p = w2.get(i2);
            if p.is_some() {
                i2 += 1;
            }
            if let Some(t) = t {
                out.extend_from_slice(t);
            }
            if let Some(p) = p {
                out.extend_from_slice(p);
            }
            if t.is_some() || p.is_some() {
                out.push(b' ');
                doneany = true;
            } else {
                break;
            }
        }
        if doneany {
            out.pop();
        }
        out
    }

    #[test]
    fn matches_unsafe_oracle() {
        // `find_next_token` classifies whitespace through the runtime
        // `stopchar_map`; initialize it (as the read/file tests do) so the old
        // pointer walk sees the same blank/newline classes `tokens` uses.
        crate::make_main::initialize_stopchar_map();
        let cases: &[(&[u8], &[u8])] = &[
            (b"", b""),
            (b"a", b""),
            (b"", b"1"),
            (b"a b", b"1 2"),
            (b"a b", b"1 2 3"),   // list2 longer
            (b"a b c", b"1 2"),   // list1 longer
            (b"  a   b  ", b"1\t2"), // irregular whitespace
        ];
        for &(l1, l2) in cases {
            assert_eq!(join_lists(l1, l2), join_lists_oracle(l1, l2));
        }
        // Exact bytes for the documented example and the empty case.
        assert_eq!(join_lists(b"a b", b"1 2 3"), b"a1 b2 3");
        assert!(join_lists(b"", b"").is_empty());
    }
}
unsafe fn func_origin(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let v: *mut variable = lookup_variable(
        ctx,
        *argv.offset(0_i32 as isize),
        strlen(*argv.offset(0_i32 as isize)) as size_t,
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const ::core::ffi::c_char,
            9,
        );
    } else {
        match (*v).origin() as i32 {
            7 => {
                abort();
            }
            0 => {
                o = variable_buffer_output(
                    o,
                    b"default\0" as *const u8 as *const ::core::ffi::c_char,
                    7,
                );
            }
            1 => {
                o = variable_buffer_output(
                    o,
                    b"environment\0" as *const u8 as *const ::core::ffi::c_char,
                    11,
                );
            }
            2 => {
                o = variable_buffer_output(
                    o,
                    b"file\0" as *const u8 as *const ::core::ffi::c_char,
                    4,
                );
            }
            3 => {
                o = variable_buffer_output(
                    o,
                    b"environment override\0" as *const u8 as *const ::core::ffi::c_char,
                    20,
                );
            }
            4 => {
                o = variable_buffer_output(
                    o,
                    b"command line\0" as *const u8 as *const ::core::ffi::c_char,
                    12,
                );
            }
            5 => {
                o = variable_buffer_output(
                    o,
                    b"override\0" as *const u8 as *const ::core::ffi::c_char,
                    8,
                );
            }
            6 => {
                o = variable_buffer_output(
                    o,
                    b"automatic\0" as *const u8 as *const ::core::ffi::c_char,
                    9,
                );
            }
            _ => {}
        }
    }
    o
}
unsafe fn func_flavor(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let v: *mut variable = lookup_variable(
        ctx,
        *argv.offset(0_i32 as isize),
        strlen(*argv.offset(0_i32 as isize)) as size_t,
    );
    if v.is_null() {
        o = variable_buffer_output(
            o,
            b"undefined\0" as *const u8 as *const ::core::ffi::c_char,
            9,
        );
    } else if (*v).recursive() != 0 {
        o = variable_buffer_output(
            o,
            b"recursive\0" as *const u8 as *const ::core::ffi::c_char,
            9,
        );
    } else {
        o = variable_buffer_output(o, b"simple\0" as *const u8 as *const ::core::ffi::c_char, 6);
    }
    o
}
unsafe fn func_notdir_suffix(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut list_iterator: *const ::core::ffi::c_char = *argv.offset(0_i32 as isize);
    let mut p2: *const ::core::ffi::c_char;
    let mut doneany: i32 = 0;
    let mut len: size_t = 0;
    // Classify the list-trimming function (`notdir`/`suffix`) through the typed
    // AST layer instead of switching on the raw first byte of the name.
    let is_suffix: i32 = matches!(
        crate::parser::NotdirSuffix::from_funcname(::std::ffi::CStr::from_ptr(funcname).to_bytes()),
        Some(crate::parser::NotdirSuffix::Suffix)
    ) as i32;
    let is_notdir: i32 = (is_suffix == 0) as i32;
    let stop: i32 = MAP_DIRSEP | (if is_suffix != 0 { MAP_DOT } else { 0 });
    loop {
        p2 = find_next_token(&raw mut list_iterator, &raw mut len);
        if p2.is_null() {
            break;
        }
        // The token is `len` bytes at p2; scan back to the last separator
        // (or '.' for $(suffix)) by index rather than walking pointers.
        let tok = ::core::slice::from_raw_parts(p2 as *const u8, len as usize);
        match tok.iter().rposition(|&c| stop_set(c, stop)) {
            Some(pos) => {
                if is_notdir != 0 {
                    o = variable_buffer_output(
                        o,
                        tok[pos + 1..].as_ptr() as *const ::core::ffi::c_char,
                        (tok.len() - pos - 1) as size_t,
                    );
                } else if tok[pos] != b'.' {
                    continue;
                } else {
                    o = variable_buffer_output(
                        o,
                        tok[pos..].as_ptr() as *const ::core::ffi::c_char,
                        (tok.len() - pos) as size_t,
                    );
                }
                o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
                doneany = 1;
            }
            None => {
                if is_notdir != 0 {
                    o = variable_buffer_output(o, p2, len);
                    o = variable_buffer_output(
                        o,
                        b" \0" as *const u8 as *const ::core::ffi::c_char,
                        1,
                    );
                    doneany = 1;
                }
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    o
}
unsafe fn func_basename_dir(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p3: *const ::core::ffi::c_char = *argv.offset(0_i32 as isize);
    let mut p2: *const ::core::ffi::c_char;
    let mut doneany: i32 = 0;
    let mut len: size_t = 0;
    // Classify the path-component function (`basename`/`dir`) through the typed
    // AST layer instead of switching on the raw first byte of the name.
    let is_basename: i32 = matches!(
        crate::parser::BasenameDir::from_funcname(::std::ffi::CStr::from_ptr(funcname).to_bytes()),
        Some(crate::parser::BasenameDir::Basename)
    ) as i32;
    let is_dir: i32 = (is_basename == 0) as i32;
    let stop: i32 = MAP_DIRSEP | (if is_basename != 0 { MAP_DOT } else { 0 }) | MAP_NUL;
    loop {
        p2 = find_next_token(&raw mut p3, &raw mut len);
        if p2.is_null() {
            break;
        }
        // Scan the token back to the last separator (or '.' for $(basename))
        // by index instead of walking pointers.
        let tok = ::core::slice::from_raw_parts(p2 as *const u8, len as usize);
        match tok.iter().rposition(|&c| stop_set(c, stop)) {
            Some(pos) if is_dir != 0 => {
                // Keep the directory part, including the separator.
                o = variable_buffer_output(o, p2, (pos + 1) as size_t);
            }
            Some(pos) if tok[pos] == b'.' => {
                // $(basename): drop the extension from the last '.'.
                o = variable_buffer_output(o, p2, pos as size_t);
            }
            _ if is_dir != 0 => {
                o = variable_buffer_output(
                    o,
                    b"./\0" as *const u8 as *const ::core::ffi::c_char,
                    2,
                );
            }
            _ => {
                o = variable_buffer_output(o, p2, len);
            }
        }
        o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
        doneany = 1;
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    o
}
unsafe fn func_addsuffix_addprefix(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let fixlen: size_t = strlen(*argv.offset(0_i32 as isize)) as size_t;
    let mut list_iterator: *const ::core::ffi::c_char = *argv.offset(1_i32 as isize);
    let is_addprefix: i32 = (*funcname.offset(3_i32 as isize) as i32 == 'p' as i32) as i32;
    let is_addsuffix: i32 = (is_addprefix == 0) as i32;
    let mut doneany: i32 = 0;
    let mut p: *const ::core::ffi::c_char;
    let mut len: size_t = 0;
    loop {
        p = find_next_token(&raw mut list_iterator, &raw mut len);
        if p.is_null() {
            break;
        }
        if is_addprefix != 0 {
            o = variable_buffer_output(o, *argv.offset(0_i32 as isize), fixlen);
        }
        o = variable_buffer_output(o, p, len);
        if is_addsuffix != 0 {
            o = variable_buffer_output(o, *argv.offset(0_i32 as isize), fixlen);
        }
        o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
        doneany = 1;
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    o
}
unsafe fn func_subst(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    o = subst_expand(
        o,
        *argv.offset(2_i32 as isize),
        *argv.offset(0_i32 as isize),
        *argv.offset(1_i32 as isize),
        strlen(*argv.offset(0_i32 as isize)) as size_t,
        strlen(*argv.offset(1_i32 as isize)) as size_t,
        0,
    );
    o
}
/// Iterate the whitespace-separated tokens of `s`, matching make's
/// `find_next_token`/`next_token`/`end_of_token`: each token is a maximal run
/// of bytes that are not in `MAP_SPACE`. Pure: borrows the byte view and
/// yields sub-slices in order.
fn tokens(s: &[u8]) -> impl DoubleEndedIterator<Item = &[u8]> {
    s.split(|&b| is_map_space(b)).filter(|w| !w.is_empty())
}

/// Return the sub-slice of `s` spanning words `start..=stop` (1-based,
/// whitespace-separated), preserving the original separators between them —
/// the semantics of `$(wordlist start,stop,s)`. `stop` is clamped to the last
/// word. Returns `None` when `stop < start` or there are fewer than `start`
/// words. Pure: scans byte indices, no pointer arithmetic.
fn word_span(s: &[u8], start: usize, stop: usize) -> Option<&[u8]> {
    if start == 0 || stop < start {
        return None;
    }
    let mut begin = None;
    let mut end = 0;
    let mut word = 0usize;
    let mut i = 0;
    while i < s.len() {
        if is_map_space(s[i]) {
            i += 1;
            continue;
        }
        word += 1;
        let ws = i;
        while i < s.len() && !is_map_space(s[i]) {
            i += 1;
        }
        if word == start {
            begin = Some(ws);
        }
        if word >= start {
            end = i; // end of the latest word within [start, stop]
        }
        if word >= stop {
            break;
        }
    }
    begin.map(|b| &s[b..end])
}

fn func_firstword(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // A safe `fn` still coerces to the function table's `unsafe fn` pointer; the
    // only unsafe is the FFI at the edges. SAFETY: `argv[0]` is a NUL-terminated
    // C string supplied by the dispatcher (`firstword` has min = max = 1).
    let bytes = unsafe { ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes() };
    if let Some(w) = tokens(bytes).next() {
        // SAFETY: `o` is the caller's output cursor; `w` is a valid subslice.
        o = unsafe {
            variable_buffer_output(o, w.as_ptr() as *const ::core::ffi::c_char, w.len() as size_t)
        };
    }
    o
}
fn func_lastword(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // SAFETY: as `func_firstword` — `argv[0]` is a NUL-terminated C string.
    let bytes = unsafe { ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes() };
    if let Some(w) = tokens(bytes).next_back() {
        // SAFETY: `o` is the caller's output cursor; `w` is a valid subslice.
        o = unsafe {
            variable_buffer_output(o, w.as_ptr() as *const ::core::ffi::c_char, w.len() as size_t)
        };
    }
    o
}
fn func_words(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // SAFETY: as `func_firstword` — `argv[0]` is a NUL-terminated C string.
    let bytes = unsafe { ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes() };
    let i = tokens(bytes).count() as ::core::ffi::c_uint;
    // Format the count in decimal in Rust rather than through `sprintf` into a
    // stack buffer; `u32::to_string` produces the same bytes as `sprintf("%u")`.
    let s = i.to_string();
    // SAFETY: `o` is the caller's output cursor; `s` is a valid byte buffer.
    o = unsafe {
        variable_buffer_output(o, s.as_ptr() as *const ::core::ffi::c_char, s.len() as size_t)
    };
    o
}
#[cfg(test)]
mod word_family_tests {
    //! AGENTS.md rule #3: the pre-conversion `unsafe` bodies of `func_firstword`,
    //! `func_lastword` and `func_words` are preserved verbatim below as
    //! `*_unsafe_oracle` and driven through the real variable-output buffer
    //! alongside the converted safe handlers, asserting byte-identical output.
    //! For `func_words` this cross-checks `u32::to_string` against the original
    //! `sprintf("%u")`.
    use super::{func_firstword, func_lastword, func_words, size_t, tokens};
    use crate::expand::{
        initialize_variable_output, variable_buffer, VARIABLE_BUFFER_TEST_LOCK,
    };
    use crate::make_main::initialize_stopchar_map;
    use std::ffi::{c_char, CString};

    type Handler = unsafe fn(
        &crate::execctx::ExecContext,
        *mut c_char,
        *mut *mut c_char,
        *const c_char,
    ) -> *mut c_char;

    unsafe fn func_firstword_unsafe_oracle(
        _ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        let bytes = ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes();
        if let Some(w) = tokens(bytes).next() {
            o = super::variable_buffer_output(o, w.as_ptr() as *const c_char, w.len() as size_t);
        }
        o
    }

    unsafe fn func_lastword_unsafe_oracle(
        _ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        let bytes = ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes();
        if let Some(w) = tokens(bytes).next_back() {
            o = super::variable_buffer_output(o, w.as_ptr() as *const c_char, w.len() as size_t);
        }
        o
    }

    unsafe fn func_words_unsafe_oracle(
        _ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        let bytes = ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes();
        let i = tokens(bytes).count() as ::core::ffi::c_uint;
        let mut buf: [c_char; 22] = [0; 22];
        o = super::variable_buffer_output(
            o,
            &raw mut buf as *mut c_char,
            super::sprintf(
                &raw mut buf as *mut c_char,
                b"%u\0" as *const u8 as *const c_char,
                i,
            ) as size_t,
        );
        o
    }

    /// Drive `handler` with a single argument through a freshly initialized
    /// variable-output buffer and return the bytes it wrote (`[start, end)`).
    unsafe fn emit(handler: Handler, arg: &[u8]) -> Vec<u8> {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        initialize_stopchar_map();
        let cstr = CString::new(arg).unwrap();
        let mut argv: [*mut c_char; 2] = [cstr.as_ptr() as *mut c_char, ::core::ptr::null_mut()];
        let name = CString::new("f").unwrap();
        let start = initialize_variable_output();
        let end = handler(
            &crate::execctx::ExecContext::default(),
            start,
            argv.as_mut_ptr(),
            name.as_ptr(),
        );
        // `variable_buffer_output` may `xrealloc` and move the global buffer, so
        // `start` can be stale after the call. Measure the written span from the
        // current base (`variable_buffer`), which `end` is guaranteed to point
        // into, rather than the possibly-freed `start`.
        let base = variable_buffer;
        assert!(!base.is_null());
        let len = end.offset_from(base);
        assert!(len >= 0, "output cursor moved before the buffer start");
        let out = ::core::slice::from_raw_parts(base as *const u8, len as usize).to_vec();
        drop(cstr);
        out
    }

    fn assert_matches(safe: Handler, oracle: Handler, arg: &[u8]) {
        let got = unsafe { emit(safe, arg) };
        let want = unsafe { emit(oracle, arg) };
        assert_eq!(got, want, "safe vs unsafe oracle diverged for input {arg:?}");
    }

    const CASES: &[&[u8]] = &[b"", b"   ", b"a", b"a b c", b"  a   b  ", b"a\tb\nc"];

    #[test]
    fn func_firstword_matches_unsafe_oracle() {
        for &c in CASES {
            assert_matches(func_firstword, func_firstword_unsafe_oracle, c);
        }
        assert_eq!(unsafe { emit(func_firstword, b"  a b c ") }, b"a");
        assert!(unsafe { emit(func_firstword, b"   ") }.is_empty());
    }

    #[test]
    fn func_lastword_matches_unsafe_oracle() {
        for &c in CASES {
            assert_matches(func_lastword, func_lastword_unsafe_oracle, c);
        }
        assert_eq!(unsafe { emit(func_lastword, b"  a b c ") }, b"c");
        assert!(unsafe { emit(func_lastword, b"   ") }.is_empty());
    }

    #[test]
    fn func_words_matches_unsafe_oracle() {
        for &c in CASES {
            assert_matches(func_words, func_words_unsafe_oracle, c);
        }
        // Decimal count, matching sprintf("%u").
        assert_eq!(unsafe { emit(func_words, b"a b c") }, b"3");
        assert_eq!(unsafe { emit(func_words, b"   ") }, b"0");
    }
}
/// Trim whitespace from both ends of the inclusive byte span `s` that the C
/// `strip_whitespace` walks between its two cursors. `is_ws` is the whitespace
/// classifier (the wrapper passes the runtime `stopchar_map` test so any
/// locale-specific classification is honoured). Returns `(lead, trail)`: how
/// many bytes to drop from the front and from the back.
///
/// Mirrors the cursor walk exactly: the leading scan may consume the whole span
/// (`lead == s.len()` when every byte is whitespace), after which nothing
/// remains for the trailing scan (`trail == 0`).
fn trim_whitespace_span(s: &[u8], is_ws: impl Fn(u8) -> bool) -> (usize, usize) {
    let lead = s.iter().take_while(|&&c| is_ws(c)).count();
    let trail = s[lead..].iter().rev().take_while(|&&c| is_ws(c)).count();
    (lead, trail)
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn strip_whitespace(
    begpp: *mut *const ::core::ffi::c_char,
    endpp: *mut *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let beg = *begpp;
    let end = *endpp;
    // `[beg, end]` is an inclusive span; it is empty when `end < beg` (e.g. an
    // empty argument, where the caller sets `end = beg - 1`). Only touch the
    // cursors when the span is non-empty, matching the C loops' `<=`/`>=`
    // guards. The span length is an address difference (the accepted boundary
    // pattern); `c_char` is one byte so this is the element count.
    if !beg.is_null() && end >= beg {
        let len = (end as usize - beg as usize) + 1;
        let span = ::core::slice::from_raw_parts(beg as *const u8, len);
        // Classify whitespace via the runtime `stopchar_map` (MAP_BLANK |
        // MAP_NEWLINE), exactly as the C loops did, preserving any locale
        // bytes `initialize_stopchar_map` tagged from `__ctype_b_loc()`.
        let (lead, trail) = trim_whitespace_span(span, |c| stop_set(c, MAP_BLANK | MAP_NEWLINE));
        // When `lead == len` (all whitespace) `trail == 0`, so `endpp` stays put
        // while `begpp` advances one past the span — the empty-result state the
        // C loops leave behind.
        *begpp = span[lead..].as_ptr() as *const ::core::ffi::c_char;
        *endpp = span[len - 1 - trail..].as_ptr() as *const ::core::ffi::c_char;
    }
    *begpp as *mut ::core::ffi::c_char
}
/// Outcome of classifying a make integer argument; see [`classify_numeric`].
#[derive(Debug, PartialEq, Eq)]
enum NumParse {
    Ok(i64),
    Empty,
    OutOfRange,
    Invalid,
}

/// Pure, allocation-free port of the parsing half of make's `parse_numeric`.
///
/// Mirrors `strtoll(.., 10)` over the make-whitespace-trimmed token: an optional
/// `+`/`-` sign followed by decimal digits, with the digit run required to span
/// the entire trimmed token. Precedence matches the C code, where the `strtoll`
/// range check happens before the "trailing garbage" check — so an overflowing
/// value reports [`NumParse::OutOfRange`] even when it is also followed by junk.
fn classify_numeric(s: &[u8]) -> NumParse {
    // make's ISSPACE is C `isspace` in the C locale: space, \t, \n, \v, \f, \r.
    const WS: &[u8] = b" \t\n\x0b\x0c\r";
    let mut token = s;
    while let [first, rest @ ..] = token {
        if WS.contains(first) {
            token = rest;
        } else {
            break;
        }
    }
    while let [rest @ .., last] = token {
        if WS.contains(last) {
            token = rest;
        } else {
            break;
        }
    }
    if token.is_empty() {
        return NumParse::Empty;
    }
    let sign_len = usize::from(matches!(token.first(), Some(b'+' | b'-')));
    let ndigits = token[sign_len..]
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .count();
    if ndigits == 0 {
        return NumParse::Invalid; // strtoll consumed nothing (endp == beg)
    }
    let prefix = &token[..sign_len + ndigits];
    // `prefix` is a sign plus ASCII digits, so it is always valid UTF-8.
    match std::str::from_utf8(prefix).unwrap_or("").parse::<i64>() {
        Err(e)
            if matches!(
                e.kind(),
                ::core::num::IntErrorKind::PosOverflow | ::core::num::IntErrorKind::NegOverflow
            ) =>
        {
            NumParse::OutOfRange
        }
        _ if sign_len + ndigits != token.len() => NumParse::Invalid, // trailing junk (endp <= end)
        Ok(n) => NumParse::Ok(n),
        Err(_) => NumParse::Invalid,
    }
}

/// Validate the single base-10 integer in `s`, aborting via `fatal` (with the
/// `msg` context) on empty / out-of-range / otherwise-invalid input. The parsing
/// is done in safe Rust by [`classify_numeric`]; the only `unsafe` here is the
/// variadic `fatal` reporting, which still needs the C string pointers.
unsafe fn parse_numeric(
    ctx: &crate::execctx::ExecContext,
    s: &::core::ffi::CStr,
    msg: &::core::ffi::CStr,
) -> i64 {
    match classify_numeric(s.to_bytes()) {
        NumParse::Ok(n) => n,
        // `fatal` diverges (`-> !`), so these arms never produce an `i64`.
        NumParse::Empty => fatal(
        ctx,
        *expanding_var,
        msg.to_bytes().len() as size_t,
        c"%s: empty value".as_ptr(),
        &[FmtArg::Str((msg.as_ptr()) as *const ::core::ffi::c_char)],
    ),
        other => {
            let fmt = if other == NumParse::OutOfRange {
                c"%s: '%s' out of range"
            } else {
                c"%s: '%s'"
            };
            fatal(
                ctx,
                *expanding_var,
                (msg.to_bytes().len() + s.to_bytes().len()) as size_t,
                fmt.as_ptr(),
                &[
                    FmtArg::Str((msg.as_ptr()) as *const ::core::ffi::c_char),
                    FmtArg::Str((s.as_ptr()) as *const ::core::ffi::c_char),
                ],
            )
        }
    }
}
fn func_word(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // SAFETY: `argv[0]`/`argv[1]` are NUL-terminated C strings from the
    // dispatcher; `parse_numeric`/`fatal` are the c2rust FFI-edge helpers.
    let i = unsafe {
        parse_numeric(
            ctx,
            ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)),
            c"invalid first argument to 'word' function",
        )
    };
    if i < 1 {
        unsafe {
            fatal(ctx, *expanding_var, 0, c"first argument to 'word' function must be greater than 0".as_ptr(), &[]);
        }
    }
    let bytes = unsafe { ::core::ffi::CStr::from_ptr(*argv.offset(1_i32 as isize)).to_bytes() };
    // `i >= 1` here; an index too large for `usize` (only reachable on 32-bit
    // targets) scans past the end and yields the empty string, as in C make.
    if let Some(w) = usize::try_from(i - 1)
        .ok()
        .and_then(|n| tokens(bytes).nth(n))
    {
        o = unsafe {
            variable_buffer_output(o, w.as_ptr() as *const ::core::ffi::c_char, w.len() as size_t)
        };
    }
    o
}
fn func_wordlist(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut buf: [::core::ffi::c_char; 23] = [0; 23];
    let badfirst = c"invalid first argument to 'wordlist' function";
    let badsecond = c"invalid second argument to 'wordlist' function";
    // SAFETY: `argv[0..=2]` are NUL-terminated C strings from the dispatcher;
    // `parse_numeric`/`fatal`/`make_lltoa`/`strlen` are the c2rust FFI helpers.
    let start = unsafe {
        parse_numeric(
            ctx,
            ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)),
            badfirst,
        )
    };
    if start < 1 {
        unsafe {
            fatal(
                ctx,
                *expanding_var,
                (badfirst.to_bytes().len() as size_t).wrapping_add(
                    strlen(make_lltoa(start, &raw mut buf as *mut ::core::ffi::c_char)) as size_t,
                ),
                c"%s: '%s'".as_ptr(),
                &[
                    FmtArg::Str((badfirst.as_ptr()) as *const ::core::ffi::c_char),
                    FmtArg::Str(
                        (make_lltoa(start, &raw mut buf as *mut ::core::ffi::c_char))
                            as *const ::core::ffi::c_char,
                    ),
                ],
            );
        }
    }
    let stop = unsafe {
        parse_numeric(
            ctx,
            ::core::ffi::CStr::from_ptr(*argv.offset(1_i32 as isize)),
            badsecond,
        )
    };
    if stop < 0 {
        unsafe {
            fatal(
                ctx,
                *expanding_var,
                (badsecond.to_bytes().len() as size_t).wrapping_add(
                    strlen(make_lltoa(stop, &raw mut buf as *mut ::core::ffi::c_char)) as size_t,
                ),
                c"%s: '%s'".as_ptr(),
                &[
                    FmtArg::Str((badsecond.as_ptr()) as *const ::core::ffi::c_char),
                    FmtArg::Str(
                        (make_lltoa(stop, &raw mut buf as *mut ::core::ffi::c_char))
                            as *const ::core::ffi::c_char,
                    ),
                ],
            );
        }
    }
    let bytes = unsafe { ::core::ffi::CStr::from_ptr(*argv.offset(2_i32 as isize)).to_bytes() };
    // `start >= 1` and `stop >= 0` here. An index beyond `usize` (only
    // reachable on 32-bit) falls off the end; `word_span` returns `None` when
    // `stop < start`, matching the original `count > 0` guard.
    let span = usize::try_from(start)
        .ok()
        .and_then(|start| word_span(bytes, start, usize::try_from(stop).unwrap_or(usize::MAX)));
    if let Some(span) = span {
        o = unsafe {
            variable_buffer_output(
                o,
                span.as_ptr() as *const ::core::ffi::c_char,
                span.len() as size_t,
            )
        };
    }
    o
}
fn func_findstring(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // A safe `fn` still coerces to the function table's `unsafe fn` pointer; the
    // only unsafe is the FFI. SAFETY: `argv[0]`/`argv[1]` are NUL-terminated C
    // strings from the dispatcher (`findstring` has min = max = 2).
    unsafe {
        if !strstr(*argv.offset(1_i32 as isize), *argv.offset(0_i32 as isize)).is_null() {
            o = variable_buffer_output(
                o,
                *argv.offset(0_i32 as isize),
                strlen(*argv.offset(0_i32 as isize)) as size_t,
            );
        }
    }
    o
}
#[cfg(test)]
mod selection_tests {
    //! AGENTS.md rule #3: the pre-conversion `unsafe` bodies of `func_findstring`,
    //! `func_word` and `func_wordlist` are preserved verbatim below as
    //! `*_unsafe_oracle` and driven through the real variable-output buffer
    //! against the converted safe handlers over the non-fatal (valid-argument)
    //! paths — the error paths call `fatal`, which aborts and cannot be unit
    //! tested. The conversion is signature-only (moving `unsafe` from the `fn`
    //! signature into blocks), so identical output confirms no behavioral drift.
    use super::{
        expanding_var, fatal, func_findstring, func_word, func_wordlist, make_lltoa, parse_numeric,
        size_t, strlen, strstr, tokens, variable_buffer_output, word_span, FmtArg,
    };
    use crate::expand::{initialize_variable_output, variable_buffer, VARIABLE_BUFFER_TEST_LOCK};
    use crate::make_main::initialize_stopchar_map;
    use std::ffi::{c_char, CString};

    type Handler = unsafe fn(
        &crate::execctx::ExecContext,
        *mut c_char,
        *mut *mut c_char,
        *const c_char,
    ) -> *mut c_char;

    unsafe fn func_findstring_unsafe_oracle(
        _ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        if !strstr(*argv.offset(1_i32 as isize), *argv.offset(0_i32 as isize)).is_null() {
            o = variable_buffer_output(
                o,
                *argv.offset(0_i32 as isize),
                strlen(*argv.offset(0_i32 as isize)) as size_t,
            );
        }
        o
    }

    unsafe fn func_word_unsafe_oracle(
        ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        let i = parse_numeric(
            ctx,
            ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)),
            c"invalid first argument to 'word' function",
        );
        if i < 1 {
            fatal(ctx, *expanding_var, 0, c"first argument to 'word' function must be greater than 0".as_ptr(), &[]);
        }
        let bytes = ::core::ffi::CStr::from_ptr(*argv.offset(1_i32 as isize)).to_bytes();
        if let Some(w) = usize::try_from(i - 1).ok().and_then(|n| tokens(bytes).nth(n)) {
            o = variable_buffer_output(o, w.as_ptr() as *const c_char, w.len() as size_t);
        }
        o
    }

    unsafe fn func_wordlist_unsafe_oracle(
        ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        let mut buf: [c_char; 23] = [0; 23];
        let badfirst = c"invalid first argument to 'wordlist' function";
        let badsecond = c"invalid second argument to 'wordlist' function";
        let start = parse_numeric(
            ctx,
            ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)),
            badfirst,
        );
        if start < 1 {
            fatal(
                ctx,
                *expanding_var,
                (badfirst.to_bytes().len() as size_t).wrapping_add(
                    strlen(make_lltoa(start, &raw mut buf as *mut c_char)) as size_t,
                ),
                c"%s: '%s'".as_ptr(),
                &[
                    FmtArg::Str((badfirst.as_ptr()) as *const c_char),
                    FmtArg::Str((make_lltoa(start, &raw mut buf as *mut c_char)) as *const c_char),
                ],
            );
        }
        let stop = parse_numeric(
            ctx,
            ::core::ffi::CStr::from_ptr(*argv.offset(1_i32 as isize)),
            badsecond,
        );
        if stop < 0 {
            fatal(
                ctx,
                *expanding_var,
                (badsecond.to_bytes().len() as size_t).wrapping_add(
                    strlen(make_lltoa(stop, &raw mut buf as *mut c_char)) as size_t,
                ),
                c"%s: '%s'".as_ptr(),
                &[
                    FmtArg::Str((badsecond.as_ptr()) as *const c_char),
                    FmtArg::Str((make_lltoa(stop, &raw mut buf as *mut c_char)) as *const c_char),
                ],
            );
        }
        let bytes = ::core::ffi::CStr::from_ptr(*argv.offset(2_i32 as isize)).to_bytes();
        let span = usize::try_from(start)
            .ok()
            .and_then(|start| word_span(bytes, start, usize::try_from(stop).unwrap_or(usize::MAX)));
        if let Some(span) = span {
            o = variable_buffer_output(
                o,
                span.as_ptr() as *const c_char,
                span.len() as size_t,
            );
        }
        o
    }

    /// Drive `handler` with `args` through a freshly initialized variable-output
    /// buffer and return the bytes it wrote. Measures the span from the current
    /// `variable_buffer` base after the call (it may `xrealloc`), per #315.
    unsafe fn emit(handler: Handler, args: &[&[u8]]) -> Vec<u8> {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        initialize_stopchar_map();
        let cstrs: Vec<CString> = args.iter().map(|a| CString::new(*a).unwrap()).collect();
        let mut argv: Vec<*mut c_char> = cstrs.iter().map(|c| c.as_ptr() as *mut c_char).collect();
        argv.push(::core::ptr::null_mut());
        let name = CString::new("f").unwrap();
        let start = initialize_variable_output();
        let end = handler(
            &crate::execctx::ExecContext::default(),
            start,
            argv.as_mut_ptr(),
            name.as_ptr(),
        );
        let base = variable_buffer;
        assert!(!base.is_null());
        let len = end.offset_from(base);
        assert!(len >= 0, "output cursor moved before the buffer start");
        let out = ::core::slice::from_raw_parts(base as *const u8, len as usize).to_vec();
        drop(cstrs);
        out
    }

    fn assert_matches(safe: Handler, oracle: Handler, args: &[&[u8]]) {
        let got = unsafe { emit(safe, args) };
        let want = unsafe { emit(oracle, args) };
        assert_eq!(got, want, "safe vs unsafe oracle diverged for args {args:?}");
    }

    #[test]
    fn func_findstring_matches_unsafe_oracle() {
        let cases: &[&[&[u8]]] = &[
            &[b"ab", b"xaby"], // present -> echoes the needle
            &[b"z", b"xaby"],  // absent  -> empty
            &[b"", b"xaby"],   // empty needle is always found
            &[b"xaby", b"xaby"],
        ];
        for c in cases {
            assert_matches(func_findstring, func_findstring_unsafe_oracle, c);
        }
        assert_eq!(unsafe { emit(func_findstring, &[b"ab", b"xaby"]) }, b"ab");
        assert!(unsafe { emit(func_findstring, &[b"z", b"xaby"]) }.is_empty());
    }

    #[test]
    fn func_word_matches_unsafe_oracle() {
        let cases: &[&[&[u8]]] = &[
            &[b"1", b"a b c"],
            &[b"2", b"a b c"],
            &[b"3", b"a b c"],
            &[b"4", b"a b c"], // past the end -> empty
            &[b"2", b"  a   b  "],
        ];
        for c in cases {
            assert_matches(func_word, func_word_unsafe_oracle, c);
        }
        assert_eq!(unsafe { emit(func_word, &[b"2", b"a b c"]) }, b"b");
        assert!(unsafe { emit(func_word, &[b"9", b"a b c"]) }.is_empty());
    }

    #[test]
    fn func_wordlist_matches_unsafe_oracle() {
        let cases: &[&[&[u8]]] = &[
            &[b"1", b"2", b"a b c d"],
            &[b"2", b"3", b"a b c d"],
            &[b"2", b"9", b"a b c d"], // stop past the end -> clamps
            &[b"3", b"2", b"a b c d"], // stop < start -> empty
            &[b"1", b"0", b"a b c d"], // stop 0 -> empty
        ];
        for c in cases {
            assert_matches(func_wordlist, func_wordlist_unsafe_oracle, c);
        }
        assert_eq!(unsafe { emit(func_wordlist, &[b"2", b"3", b"a b c d"]) }, b"b c");
        assert!(unsafe { emit(func_wordlist, &[b"3", b"2", b"a b c d"]) }.is_empty());
    }
}
unsafe fn func_foreach(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let varname = ExpandedArg::new(
        ctx,
        *argv.offset(0_i32 as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let list = ExpandedArg::new(
        ctx,
        *argv.offset(1_i32 as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let body: *const ::core::ffi::c_char = *argv.offset(2_i32 as isize);
    let mut doneany: i32 = 0;
    let mut list_iterator: *const ::core::ffi::c_char = list.as_ptr();
    let mut p: *const ::core::ffi::c_char;
    let mut len: size_t = 0;
    let var: *mut variable;
    let vp: *mut ::core::ffi::c_char = next_token(varname.as_ptr());
    // Bridge to the safe `end_of_token`: terminate the token by writing a NUL
    // at `vp + token_len` (the offset of the first whitespace/NUL).
    let vp_eot = vp.add(end_of_token(::core::slice::from_raw_parts(
        vp as *const u8,
        strlen(vp),
    )));
    *vp_eot = 0;
    push_new_variable_scope();
    var = define_variable_in_set(
        ctx,
        vp,
        strlen(vp) as size_t,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_automatic,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    loop {
        p = find_next_token(&raw mut list_iterator, &raw mut len);
        if p.is_null() {
            break;
        }
        free((*var).value as *mut ::core::ffi::c_void);
        (*var).value = xstrndup(p, len);
        let result = ExpandedArg::from_raw(allocated_expand_string_for_file(
            ctx,
            body,
            ::core::ptr::null_mut::<File>(),
        ));
        o = variable_buffer_output(o, result.as_ptr(), strlen(result.as_ptr()) as size_t);
        o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
        doneany = 1;
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    pop_variable_scope();
    o
}
unsafe fn func_let(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let varnames = ExpandedArg::new(
        ctx,
        *argv.offset(0_i32 as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let list = ExpandedArg::new(
        ctx,
        *argv.offset(1_i32 as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let body: *const ::core::ffi::c_char = *argv.offset(2_i32 as isize);
    let mut vp: *const ::core::ffi::c_char;
    let mut vp_next: *const ::core::ffi::c_char = varnames.as_ptr();
    let mut list_iterator: *const ::core::ffi::c_char = list.as_ptr();
    let mut vlen: size_t = 0;
    push_new_variable_scope();
    vp = find_next_token(&raw mut vp_next, &raw mut vlen);
    while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
        .offset(*vp_next as ::core::ffi::c_uchar as isize) as i32
        & (0x2_i32 | 0x4_i32)
        != 0
    {
        vp_next = vp_next.offset(1_i32 as isize);
    }
    while *vp_next as i32 != 0 {
        let mut len: size_t = 0;
        let p: *mut ::core::ffi::c_char = find_next_token(&raw mut list_iterator, &raw mut len);
        if !p.is_null() && *list_iterator as i32 != 0 {
            list_iterator = list_iterator.offset(1_i32 as isize);
            *p.offset(len as isize) = 0;
        }
        define_variable_in_set(
            ctx,
            vp,
            vlen,
            if !p.is_null() {
                p as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        vp = find_next_token(&raw mut vp_next, &raw mut vlen);
        while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
            .offset(*vp_next as ::core::ffi::c_uchar as isize) as i32
            & (0x2_i32 | 0x4_i32)
            != 0
        {
            vp_next = vp_next.offset(1_i32 as isize);
        }
    }
    if !vp.is_null() {
        define_variable_in_set(
            ctx,
            vp,
            vlen,
            next_token(list_iterator),
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
    }
    o = expand_string_buf(ctx, o, body, SIZE_MAX as size_t);
    pop_variable_scope();
    o.offset(strlen(o) as isize)
}
unsafe fn func_filter_filterout(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let words: *mut a_word;
    let word_end: *mut a_word;
    let mut wp: *mut a_word;
    let patterns: *mut a_pattern;
    let pat_end: *mut a_pattern;
    let mut pp: *mut a_pattern;
    let mut pat_count: ::core::ffi::c_ulong = 0;
    let mut word_count: ::core::ffi::c_ulong = 0;
    // Word lookup table for the literal-pattern fast path, built only when
    // `hashing` (see below). Keyed by word content bytes; the value is the head
    // of a `chain` linking every word with identical content, so a matched
    // literal pattern can mark them all. Replaces the c2rust gnulib `hash_table`
    // plus the `a_word_hash_1/2/cmp` callbacks.
    let mut a_word_table: FxHashMap<Box<[u8]>, *mut a_word> = FxHashMap::default();
    let is_filter: i32 = (*funcname.offset(
        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize).wrapping_sub(1_usize)
            as isize,
    ) as i32
        == 0) as i32;
    let mut cp: *const ::core::ffi::c_char;
    let mut literals: i32 = 0;
    let hashing: i32;
    let mut p: *mut ::core::ffi::c_char;
    let mut len: size_t = 0;
    let mut doneany: i32 = 0;
    cp = *argv.offset(1_i32 as isize);
    loop {
        p = find_next_token(&raw mut cp, ::core::ptr::null_mut::<size_t>());
        if p.is_null() {
            break;
        }
        word_count = word_count.wrapping_add(1);
    }
    if word_count == 0 {
        return o;
    }
    // Owned, zero-initialized word table (was an xcalloc'd array freed at
    // the end). Fixed capacity keeps the backing pointer stable while the
    // hash table below stores pointers into it.
    let mut words_vec: Vec<a_word> = Vec::with_capacity(word_count as usize);
    words_vec.resize_with(word_count as usize, || unsafe { ::core::mem::zeroed() });
    words = words_vec.as_mut_ptr();
    word_end = words.offset(word_count as isize);
    cp = *argv.offset(0_i32 as isize);
    loop {
        p = find_next_token(&raw mut cp, ::core::ptr::null_mut::<size_t>());
        if p.is_null() {
            break;
        }
        pat_count = pat_count.wrapping_add(1);
    }
    let mut patterns_vec: Vec<a_pattern> = Vec::with_capacity(pat_count as usize);
    patterns_vec.resize_with(pat_count as usize, || unsafe { ::core::mem::zeroed() });
    patterns = patterns_vec.as_mut_ptr();
    pat_end = patterns.offset(pat_count as isize);
    cp = *argv.offset(0_i32 as isize);
    pp = patterns;
    loop {
        p = find_next_token(&raw mut cp, &raw mut len);
        if p.is_null() {
            break;
        }
        if *cp as i32 != 0 {
            cp = cp.offset(1_i32 as isize);
        }
        *p.offset(len as isize) = 0;
        (*pp).str_0 = p;
        (*pp).percent = find_percent(p);
        if (*pp).percent.is_null() {
            literals += 1;
        }
        (*pp).length = strlen((*pp).str_0) as size_t;
        pp = pp.offset(1_i32 as isize);
    }
    cp = *argv.offset(1_i32 as isize);
    wp = words;
    loop {
        p = find_next_token(&raw mut cp, &raw mut len);
        if p.is_null() {
            break;
        }
        if *cp as i32 != 0 {
            cp = cp.offset(1_i32 as isize);
        }
        *p.offset(len as isize) = 0;
        (*wp).str_0 = p;
        (*wp).length = len;
        wp = wp.offset(1_i32 as isize);
    }
    hashing =
        (literals > 1 && (literals as ::core::ffi::c_ulong).wrapping_mul(word_count) >= 10) as i32;
    if hashing != 0 {
        a_word_table.reserve(word_count as usize);
        wp = words;
        while wp < word_end {
            let key: Box<[u8]> =
                ::core::slice::from_raw_parts((*wp).str_0 as *const u8, (*wp).length).into();
            // Insert replaces any equal-content word and returns the previous
            // head, which the new word then chains to (matching the C
            // `hash_insert`: stored slot holds the latest, `chain` links back).
            if let Some(owp) = a_word_table.insert(key, wp) {
                (*wp).chain = owp;
            }
            wp = wp.offset(1_i32 as isize);
        }
    }
    pp = patterns;
    while pp < pat_end {
        if !(*pp).percent.is_null() {
            wp = words;
            while wp < word_end {
                (*wp).matched |= pattern_matches((*pp).str_0, (*pp).percent, (*wp).str_0);
                wp = wp.offset(1_i32 as isize);
            }
        } else if hashing != 0 {
            // Mark every word whose content equals this literal pattern: look up
            // the chain head by the pattern's bytes and walk the `chain`.
            let key = ::core::slice::from_raw_parts((*pp).str_0 as *const u8, (*pp).length);
            if let Some(&head) = a_word_table.get(key) {
                wp = head;
                while let Some(wpref) = wp.as_mut() {
                    wpref.matched |= 1;
                    wp = wpref.chain;
                }
            }
        } else {
            wp = words;
            while wp < word_end {
                (*wp).matched |= ((*wp).length == (*pp).length
                    && memcmp(
                        (*pp).str_0 as *const ::core::ffi::c_void,
                        (*wp).str_0 as *const ::core::ffi::c_void,
                        (*wp).length as size_t,
                    ) == 0) as i32;
                wp = wp.offset(1_i32 as isize);
            }
        }
        pp = pp.offset(1_i32 as isize);
    }
    wp = words;
    while wp < word_end {
        if if is_filter != 0 {
            (*wp).matched
        } else {
            ((*wp).matched == 0) as i32
        } != 0
        {
            o = variable_buffer_output(o, (*wp).str_0, strlen((*wp).str_0) as size_t);
            o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
            doneany = 1;
        }
        wp = wp.offset(1_i32 as isize);
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    o
}
/// Collapse `bytes` to its whitespace-separated words rejoined by single
/// spaces — the transformation `$(strip ...)` performs. Word boundaries use
/// make's `MAP_BLANK | MAP_NEWLINE` separator class (not Unicode whitespace).
/// Every emitted word is non-empty, so the append-space-then-`pop` form is
/// byte-identical to the C loop's "emit a space after each word, then `o -= 1`".
fn strip_words(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut idx = 0usize;
    while idx < bytes.len() {
        // Skip the whitespace separating words.
        while idx < bytes.len() && stop_set(bytes[idx], MAP_BLANK | MAP_NEWLINE) {
            idx += 1;
        }
        let word_start = idx;
        while idx < bytes.len() && !stop_set(bytes[idx], MAP_BLANK | MAP_NEWLINE) {
            idx += 1;
        }
        if idx == word_start {
            break;
        }
        out.extend_from_slice(&bytes[word_start..idx]);
        out.push(b' ');
    }
    // Drop the trailing separator (no-op when nothing was emitted).
    out.pop();
    out
}
fn func_strip(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // A safe `fn` still coerces to the function table's `unsafe fn` pointer; the
    // only unsafe is the FFI at the edges. SAFETY: the dispatcher passes an
    // `argv` of at least `maximum_args` NUL-terminated C strings (`strip` has
    // min = max = 1), so `argv[0]` is valid.
    let stripped =
        strip_words(unsafe { ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes() });
    if !stripped.is_empty() {
        // SAFETY: `o` is the caller's variable-buffer output cursor and
        // `stripped` is a valid byte buffer of the given length.
        o = unsafe {
            variable_buffer_output(
                o,
                stripped.as_ptr() as *const ::core::ffi::c_char,
                stripped.len() as size_t,
            )
        };
    }
    o
}
unsafe fn func_error(
    ctx: &crate::execctx::ExecContext,
    o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // Classify the diagnostic function (`error`/`warning`/`info`) through the
    // typed AST layer instead of switching on the raw first byte of the name.
    let logfn =
        crate::parser::LogFunction::from_funcname(::std::ffi::CStr::from_ptr(funcname).to_bytes());
    match logfn {
        Some(crate::parser::LogFunction::Error) => {
            fatal(
        ctx,
        reading_file,
        strlen(*argv.offset(0_i32 as isize)) as size_t,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char)],
    );
        }
        Some(crate::parser::LogFunction::Warning) => {
            error(
        ctx,
        reading_file,
        strlen(*argv.offset(0_i32 as isize)) as size_t,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((*argv.offset(0_i32 as isize)) as *const ::core::ffi::c_char)],
    );
        }
        Some(crate::parser::LogFunction::Info) => {
            // $(info ...): build "<arg>\n\0" in an owned buffer instead of a
            // malloc/memcpy/free sequence.
            let src = *argv.offset(0_i32 as isize);
            let len = strlen(src) as usize;
            let mut msg = Vec::<u8>::with_capacity(len + 2);
            msg.extend_from_slice(::core::slice::from_raw_parts(src as *const u8, len));
            msg.push(b'\n');
            msg.push(0);
            outputs(ctx, 0, msg.as_ptr() as *const ::core::ffi::c_char);
        }
        _ => {
            fatal(
        ctx,
        *expanding_var,
        strlen(funcname) as size_t,
        b"INTERNAL: func_error: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((funcname) as *const ::core::ffi::c_char)],
    );
        }
    }
    o
}
/// Alphabetically sort the whitespace-separated words of `bytes`, drop
/// duplicates, and rejoin single-space-separated — the transformation
/// `$(sort ...)` performs. Duplicates are byte-equal, hence adjacent after
/// sorting, so `dedup` removes them. Every retained word is non-empty, so the
/// append-space-then-`pop` form matches the C loop's trailing `o -= 1`.
fn sort_words(bytes: &[u8]) -> Vec<u8> {
    let mut words: Vec<&[u8]> = tokens(bytes).collect();
    let mut out = Vec::new();
    if !words.is_empty() {
        words.sort_by(|a, b| alpha_cmp(a, b));
        words.dedup();
        for w in words {
            out.extend_from_slice(w);
            out.push(b' ');
        }
        out.pop();
    }
    out
}
fn func_sort(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // A safe `fn` still coerces to the function table's `unsafe fn` pointer; the
    // only unsafe is the FFI at the edges. SAFETY: the dispatcher passes an
    // `argv` of at least `maximum_args` NUL-terminated C strings (`sort` has
    // min = max = 1), so `argv[0]` is valid.
    let sorted =
        sort_words(unsafe { ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes() });
    if !sorted.is_empty() {
        // SAFETY: `o` is the caller's variable-buffer output cursor and `sorted`
        // is a valid byte buffer of the given length.
        o = unsafe {
            variable_buffer_output(
                o,
                sorted.as_ptr() as *const ::core::ffi::c_char,
                sorted.len() as size_t,
            )
        };
    }
    o
}
#[cfg(test)]
mod strip_sort_tests {
    //! AGENTS.md rule #3: the pre-conversion `unsafe` bodies of `func_strip`
    //! and `func_sort` are preserved *verbatim* below as `*_unsafe_oracle` and
    //! driven through the real variable-output buffer alongside the converted
    //! safe handlers, asserting the emitted bytes are identical — including the
    //! per-word `variable_buffer_output` calls and the trailing `o -= 1` trim
    //! the safe versions replaced with an owned buffer + `pop()`.
    use super::{
        func_sort, func_strip, size_t, stop_set, tokens, MAP_BLANK, MAP_NEWLINE,
    };
    use crate::expand::{
        initialize_variable_output, variable_buffer, VARIABLE_BUFFER_TEST_LOCK,
    };
    use crate::make_main::initialize_stopchar_map;
    use crate::misc::alpha_cmp;
    use std::ffi::{c_char, CString};

    type Handler = unsafe fn(
        &crate::execctx::ExecContext,
        *mut c_char,
        *mut *mut c_char,
        *const c_char,
    ) -> *mut c_char;

    /// Verbatim pre-conversion `func_strip`: emit each word followed by a
    /// separator space via `variable_buffer_output`, then trim the trailing
    /// space by walking the cursor back one byte.
    unsafe fn func_strip_unsafe_oracle(
        _ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        let s: *const c_char = *argv.offset(0_i32 as isize);
        let bytes = ::core::ffi::CStr::from_ptr(s).to_bytes();
        let mut idx = 0usize;
        let mut doneany = false;
        while idx < bytes.len() {
            while idx < bytes.len() && stop_set(bytes[idx], MAP_BLANK | MAP_NEWLINE) {
                idx += 1;
            }
            let word_start = idx;
            while idx < bytes.len() && !stop_set(bytes[idx], MAP_BLANK | MAP_NEWLINE) {
                idx += 1;
            }
            let word_len = idx - word_start;
            if word_len == 0 {
                break;
            }
            o = super::variable_buffer_output(
                o,
                bytes[word_start..].as_ptr() as *const c_char,
                word_len as size_t,
            );
            o = super::variable_buffer_output(o, b" \0" as *const u8 as *const c_char, 1);
            doneany = true;
        }
        if doneany {
            o = o.offset(-1_i32 as isize);
        }
        o
    }

    /// Verbatim pre-conversion `func_sort`: sort/dedup the tokens, emit each
    /// followed by a separator space, then trim the trailing space by walking
    /// the cursor back one byte.
    unsafe fn func_sort_unsafe_oracle(
        _ctx: &crate::execctx::ExecContext,
        mut o: *mut c_char,
        argv: *mut *mut c_char,
        mut _funcname: *const c_char,
    ) -> *mut c_char {
        let bytes = ::core::ffi::CStr::from_ptr(*argv.offset(0_i32 as isize)).to_bytes();
        let mut words: Vec<&[u8]> = tokens(bytes).collect();
        if !words.is_empty() {
            words.sort_by(|a, b| alpha_cmp(a, b));
            words.dedup();
            for w in words {
                o = super::variable_buffer_output(
                    o,
                    w.as_ptr() as *const c_char,
                    w.len() as size_t,
                );
                o = super::variable_buffer_output(o, b" \0" as *const u8 as *const c_char, 1);
            }
            o = o.offset(-1_i32 as isize);
        }
        o
    }

    /// Drive `handler` with a single argument through a freshly initialized
    /// variable-output buffer and return the bytes it wrote (`[start, end)`).
    unsafe fn emit(handler: Handler, arg: &[u8]) -> Vec<u8> {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        initialize_stopchar_map();
        let cstr = CString::new(arg).unwrap();
        let mut argv: [*mut c_char; 2] = [cstr.as_ptr() as *mut c_char, ::core::ptr::null_mut()];
        let name = CString::new("f").unwrap();
        let start = initialize_variable_output();
        let end = handler(
            &crate::execctx::ExecContext::default(),
            start,
            argv.as_mut_ptr(),
            name.as_ptr(),
        );
        let len = end.offset_from(start);
        assert!(len >= 0, "output cursor moved before the buffer start");
        let out = ::core::slice::from_raw_parts(start as *const u8, len as usize).to_vec();
        // Keep `cstr` alive until after the handler has read `argv`.
        assert!(!variable_buffer.is_null());
        drop(cstr);
        out
    }

    fn assert_matches(safe: Handler, oracle: Handler, arg: &[u8]) {
        let got = unsafe { emit(safe, arg) };
        let want = unsafe { emit(oracle, arg) };
        assert_eq!(got, want, "safe vs unsafe oracle diverged for input {arg:?}");
    }

    #[test]
    fn func_strip_matches_unsafe_oracle() {
        let cases: &[&[u8]] = &[
            b"",
            b"   ",
            b"a",
            b"a b c",
            b"  a   b  ",
            b"a\tb\nc",
            b"\ta\t",
        ];
        for &c in cases {
            assert_matches(func_strip, func_strip_unsafe_oracle, c);
        }
        // Exact bytes for the documented collapse.
        assert_eq!(unsafe { emit(func_strip, b"  a   b  ") }, b"a b");
        assert!(unsafe { emit(func_strip, b"   ") }.is_empty());
    }

    #[test]
    fn func_sort_matches_unsafe_oracle() {
        let cases: &[&[u8]] = &[
            b"",
            b"foo",
            b"foo bar baz",
            b"c b a",
            b"foo foo bar bar", // duplicates collapse
            b"  b   a  ",       // irregular whitespace
            b"2 10 1",          // byte order, not numeric
        ];
        for &c in cases {
            assert_matches(func_sort, func_sort_unsafe_oracle, c);
        }
        // Exact bytes: sorted, de-duplicated, single-space-joined.
        assert_eq!(unsafe { emit(func_sort, b"foo foo bar") }, b"bar foo");
        assert!(unsafe { emit(func_sort, b"") }.is_empty());
    }
}
/// Is `c` whitespace in make's `MAP_SPACE` class (`next_token`'s skip set):
/// space, tab, newline, vertical tab, form feed, or carriage return? This is
/// the ASCII `isspace` set; it deliberately does not use Unicode whitespace,
/// which would widen make's language definition (e.g. U+00A0).
fn is_map_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// Outcome of parsing a textual integer with [`parse_textint`].
enum TextInt {
    /// The (post-whitespace) value is empty.
    Empty,
    /// The value is not a well-formed integer (no digits, or trailing junk).
    NotNumeric,
    /// A valid integer. `sign` is `-1`/`0`/`+1` for negative-nonzero / zero /
    /// positive-nonzero; `num_start..num_end` is the run of significant digits
    /// (leading zeros stripped), as offsets into the token slice.
    Parsed {
        sign: i32,
        num_start: usize,
        num_end: usize,
    },
}

/// Pure mirror of make's `parse_textint` digit/sign parsing, over the token
/// slice `t` (the bytes from `next_token(number)`, without the NUL).
fn classify_textint(t: &[u8]) -> TextInt {
    if t.is_empty() {
        return TextInt::Empty;
    }
    let negative = t[0] == b'-';
    let mut i = 0;
    if negative || t[0] == b'+' {
        i = 1;
    }
    let after_sign = i;
    while i < t.len() && t[i] == b'0' {
        i += 1;
    }
    let num_start = i;
    while i < t.len() && t[i].is_ascii_digit() {
        i += 1;
    }
    let num_end = i;
    // No digits at all after the sign, or non-whitespace trailing the number.
    let trailing_ok = t[num_end..].iter().all(|&c| is_map_space(c));
    if num_end == after_sign || !trailing_ok {
        return TextInt::NotNumeric;
    }
    let nonzero = (num_start != num_end) as i32;
    let sign = if negative { -nonzero } else { nonzero };
    TextInt::Parsed {
        sign,
        num_start,
        num_end,
    }
}

/// # Safety
///
/// C-style API operating on raw pointers; `number` and `msg` must be valid
/// NUL-terminated strings and the out-parameters must be valid for writes.
/// Aborts via [`fatal`] on an empty or non-numeric value.
unsafe fn parse_textint(
    ctx: &crate::execctx::ExecContext,
    number: *const ::core::ffi::c_char,
    msg: *const ::core::ffi::c_char,
    sign: *mut i32,
    numstart: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let p: *const ::core::ffi::c_char = next_token(number);
    let t = ::core::ffi::CStr::from_ptr(p).to_bytes();
    match classify_textint(t) {
        TextInt::Empty => fatal(
        ctx,
        *expanding_var,
        strlen(msg) as size_t,
        b"%s: empty value\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((msg) as *const ::core::ffi::c_char)],
    ),
        TextInt::NotNumeric => fatal(
        ctx,
        *expanding_var,
        (strlen(msg) as size_t).wrapping_add(strlen(number) as size_t),
        b"%s: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((msg) as *const ::core::ffi::c_char),
            FmtArg::Str((number) as *const ::core::ffi::c_char)],
    ),
        TextInt::Parsed {
            sign: s,
            num_start,
            num_end,
        } => {
            *sign = s;
            *numstart = p.add(num_start);
            p.add(num_end)
        }
    }
}
/// Compare two integers parsed by `parse_textint`, given each one's sign
/// (`-1`, `0`, or `+1`) and the byte span of its magnitude digits as the
/// parser delimited them. Returns a value `< 0`, `0`, or `> 0` following
/// `intcmp`'s rules: order by sign first, then by digit count, then by a
/// byte-wise comparison of the digits, flipping the result for negatives.
/// Pure: depends only on its arguments.
fn compare_textint(lsign: i32, ldigits: &[u8], rsign: i32, rdigits: &[u8]) -> i32 {
    let mut cmp = lsign - rsign;
    if cmp == 0 {
        cmp = (ldigits.len() > rdigits.len()) as i32 - (ldigits.len() < rdigits.len()) as i32;
        if cmp == 0 {
            cmp = match ldigits.cmp(rdigits) {
                ::core::cmp::Ordering::Less => -1,
                ::core::cmp::Ordering::Equal => 0,
                ::core::cmp::Ordering::Greater => 1,
            };
        }
        if lsign < 0 {
            cmp = -cmp;
        }
    }
    cmp
}

unsafe fn func_intcmp(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut lsign: i32 = 0;
    let mut rsign: i32 = 0;
    let mut lnum: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rnum: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let lhs_str = ExpandedArg::new(
        ctx,
        *argv.offset(0_i32 as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let rhs_str = ExpandedArg::new(
        ctx,
        *argv.offset(1_i32 as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    let llim: *const ::core::ffi::c_char = parse_textint(
        ctx,
        lhs_str.as_ptr(),
        b"non-numeric first argument to 'intcmp' function\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut lsign,
        &raw mut lnum,
    );
    let rlim: *const ::core::ffi::c_char = parse_textint(
        ctx,
        rhs_str.as_ptr(),
        b"non-numeric second argument to 'intcmp' function\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut rsign,
        &raw mut rnum,
    );
    // `parse_textint` hands back end pointers; form the digit spans once at the
    // boundary and let the pure comparator do the rest.
    let ldigits = ::core::slice::from_raw_parts(lnum as *const u8, llim.offset_from(lnum) as usize);
    let rdigits = ::core::slice::from_raw_parts(rnum as *const u8, rlim.offset_from(rnum) as usize);
    let llen: ptrdiff_t = ldigits.len() as ptrdiff_t;
    let cmp: i32 = compare_textint(lsign, ldigits, rsign, rdigits);
    argv = argv.offset(2_i32 as isize);
    if (*argv).is_null() && cmp == 0 {
        if lsign < 0 {
            o = variable_buffer_output(o, b"-\0" as *const u8 as *const ::core::ffi::c_char, 1);
        }
        o = variable_buffer_output(
            o,
            lnum.offset(-((lsign == 0) as i32 as isize)),
            (llen + (lsign == 0) as i32 as ptrdiff_t) as size_t,
        );
    }
    if !(*argv).is_null() && cmp >= 0 {
        argv = argv.offset(1_i32 as isize);
        if cmp > 0 && !(*argv).is_null() && !(*argv.offset(1_i32 as isize)).is_null() {
            argv = argv.offset(1_i32 as isize);
        }
    }
    if !(*argv).is_null() {
        let expansion = ExpandedArg::new(ctx, *argv, ::core::ptr::null::<::core::ffi::c_char>());
        o = variable_buffer_output(o, expansion.as_ptr(), strlen(expansion.as_ptr()) as size_t);
    }
    o
}
unsafe fn func_if(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut begp: *const ::core::ffi::c_char = *argv.offset(0_i32 as isize);
    let mut endp: *const ::core::ffi::c_char = begp
        .offset(strlen(*argv.offset(0_i32 as isize)) as isize)
        .offset(-(1_i32 as isize));
    let mut result: i32 = 0;
    strip_whitespace(&raw mut begp, &raw mut endp);
    if begp <= endp {
        let expansion = ExpandedArg::new(ctx, begp, endp.offset(1_i32 as isize));
        result = (*expansion.as_ptr().offset(0_i32 as isize) as i32 != 0) as i32;
    }
    argv = argv.offset((1 + (result == 0) as i32) as isize);
    if !(*argv).is_null() {
        let expansion = ExpandedArg::new(ctx, *argv, ::core::ptr::null::<::core::ffi::c_char>());
        o = variable_buffer_output(o, expansion.as_ptr(), strlen(expansion.as_ptr()) as size_t);
    }
    o
}
unsafe fn func_or(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while !(*argv).is_null() {
        let mut begp: *const ::core::ffi::c_char = *argv;
        let mut endp: *const ::core::ffi::c_char = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1_i32 as isize));
        strip_whitespace(&raw mut begp, &raw mut endp);
        if !(begp > endp) {
            let expansion = ExpandedArg::new(ctx, begp, endp.offset(1_i32 as isize));
            let result = strlen(expansion.as_ptr()) as size_t;
            if result != 0 {
                o = variable_buffer_output(o, expansion.as_ptr(), result);
                break;
            }
        }
        argv = argv.offset(1_i32 as isize);
    }
    o
}
unsafe fn func_and(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    loop {
        let mut begp: *const ::core::ffi::c_char = *argv;
        let mut endp: *const ::core::ffi::c_char = begp
            .offset(strlen(*argv) as isize)
            .offset(-(1_i32 as isize));
        strip_whitespace(&raw mut begp, &raw mut endp);
        if begp > endp {
            return o;
        }
        let expansion = ExpandedArg::new(ctx, begp, endp.offset(1_i32 as isize));
        let result = strlen(expansion.as_ptr()) as size_t;
        if result == 0 {
            break;
        }
        argv = argv.offset(1_i32 as isize);
        if (*argv).is_null() {
            o = variable_buffer_output(o, expansion.as_ptr(), result);
            break;
        }
        // More arguments remain: drop this expansion and evaluate the next.
    }
    o
}
unsafe fn func_wildcard(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let names = string_glob(ctx, *argv.offset(0_i32 as isize));
    o = variable_buffer_output(o, names.as_ptr() as *const ::core::ffi::c_char, names.len() as size_t);
    o
}
unsafe fn func_eval(
    ctx: &crate::execctx::ExecContext,
    o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    install_variable_buffer(&raw mut buf, &raw mut len);
    eval_buffer(
        ctx,
        *argv.offset(0_i32 as isize),
        ::core::ptr::null::<Floc>(),
    );
    restore_variable_buffer(buf, len);
    o
}
unsafe fn func_value(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let v: *mut variable = lookup_variable(
        ctx,
        *argv.offset(0_i32 as isize),
        strlen(*argv.offset(0_i32 as isize)) as size_t,
    );
    if !v.is_null() {
        o = variable_buffer_output(o, (*v).value, strlen((*v).value) as size_t);
    }
    o
}
/// Fold the first `buf.len()` bytes in place: each `\n` becomes a single space
/// and the `\r` of a `\r\n` pair is dropped. The write cursor never overtakes
/// the read cursor, so the rewrite is safe in place. Returns the new length.
///
/// With `trim_newlines` the result is cut immediately after the last
/// non-newline byte (dropping every trailing folded space); otherwise at most
/// one trailing space is kept. Mirrors the C `fold_newlines` exactly, including
/// its `last_nonnl == buffer - 1` "no non-newline seen" sentinel (here the
/// signed index `-1`) and the `dst - 2` trailing-space rule.
fn fold_newlines_bytes(buf: &mut [u8], trim_newlines: bool) -> usize {
    let mut dst = 0usize;
    let mut last_nonnl: isize = -1;
    let mut src = 0usize;
    while src < buf.len() && buf[src] != 0 {
        // A `\r` immediately followed by `\n` is skipped (the `\n` that follows
        // becomes the space); a trailing lone `\r` is a normal byte.
        let is_crlf = buf[src] == b'\r' && src + 1 < buf.len() && buf[src + 1] == b'\n';
        if !is_crlf {
            if buf[src] == b'\n' {
                buf[dst] = b' ';
            } else {
                last_nonnl = dst as isize;
                buf[dst] = buf[src];
            }
            dst += 1;
        }
        src += 1;
    }
    if !trim_newlines && last_nonnl < dst as isize - 2 {
        last_nonnl = dst as isize - 2;
    }
    (last_nonnl + 1) as usize
}

/// # Safety
///
/// `buffer` must be valid for reads and writes of `*length + 1` bytes — the
/// content plus the one extra slot for the terminating NUL the caller already
/// reserves; `length` must be valid.
unsafe fn fold_newlines(buffer: *mut ::core::ffi::c_char, length: *mut size_t, trim_newlines: i32) {
    let len = *length;
    // Include the caller's reserved NUL slot so the terminator is written by
    // indexing rather than raw pointer arithmetic.
    let buf = ::core::slice::from_raw_parts_mut(buffer as *mut u8, len + 1);
    let new_len = fold_newlines_bytes(&mut buf[..len], trim_newlines != 0);
    buf[new_len] = 0;
    *length = new_len as size_t;
}
/// PID of the running `$(shell)` child, or `0` when none. Written in the
/// `$(shell)` path and by the `shell_completed` reaper callback, and read by
/// `reap_children` (reached from the `SIGCHLD` handler), so it is shared with a
/// signal-adjacent reader — an atomic gives that sharing defined semantics.
pub static SHELL_FUNCTION_PID: AtomicI32 = AtomicI32::new(0);

/// Read the running `$(shell)` child's PID (`0` when none).
pub fn shell_function_pid() -> pid_t {
    SHELL_FUNCTION_PID.load(Ordering::Relaxed)
}

#[cfg(test)]
mod shell_function_pid_tests {
    use super::*;

    /// `shell_function_pid()` reflects the `SHELL_FUNCTION_PID` atomic: `0` when
    /// no `$(shell)` child is running, the PID while one is. Restores the prior
    /// value so the global stays isolated from other tests.
    #[test]
    fn accessor_tracks_atomic() {
        let saved = SHELL_FUNCTION_PID.load(Ordering::Relaxed);

        SHELL_FUNCTION_PID.store(0, Ordering::Relaxed);
        assert_eq!(shell_function_pid(), 0, "no child running");

        SHELL_FUNCTION_PID.store(4242, Ordering::Relaxed);
        assert_eq!(shell_function_pid(), 4242, "reflects the running PID");

        SHELL_FUNCTION_PID.store(saved, Ordering::Relaxed);
    }
}
/// Set by the `$(shell)` child's reaper callback ([`shell_completed`], which is
/// also reached from the `SIGCHLD` handler) and spin-waited on by `func_shell`,
/// so it is shared between a signal-adjacent writer and the main flow. An atomic
/// gives that sharing defined semantics (and keeps the spin-loop's load from
/// being hoisted) — the original `static mut` had neither.
static SHELL_FUNCTION_COMPLETED: AtomicI32 = AtomicI32::new(0);

/// Read the `$(shell)` completion flag: `0` while the child is still running,
/// `1` on success, `-1` when the shell could not be started.
fn shell_function_completed() -> i32 {
    SHELL_FUNCTION_COMPLETED.load(Ordering::Relaxed)
}

#[cfg(test)]
mod shell_function_completed_tests {
    use super::*;

    /// `shell_function_completed()` reflects the `SHELL_FUNCTION_COMPLETED`
    /// atomic across the three states the reaper callback can leave it in.
    /// Restores the prior value so the global stays isolated from other tests.
    #[test]
    fn accessor_tracks_atomic() {
        let saved = SHELL_FUNCTION_COMPLETED.load(Ordering::Relaxed);

        SHELL_FUNCTION_COMPLETED.store(0, Ordering::Relaxed);
        assert_eq!(shell_function_completed(), 0, "pending");

        SHELL_FUNCTION_COMPLETED.store(1, Ordering::Relaxed);
        assert_eq!(shell_function_completed(), 1, "completed ok");

        SHELL_FUNCTION_COMPLETED.store(-1, Ordering::Relaxed);
        assert_eq!(shell_function_completed(), -1, "failed to start");

        SHELL_FUNCTION_COMPLETED.store(saved, Ordering::Relaxed);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn shell_completed(
    ctx: &crate::execctx::ExecContext,
    mut exit_code: i32,
    exit_sig: i32,
) {
    let mut buf: [::core::ffi::c_char; 22] = [0; 22];
    SHELL_FUNCTION_PID.store(0, Ordering::Relaxed);
    if exit_sig == 0 && exit_code == 127 {
        SHELL_FUNCTION_COMPLETED.store(-1_i32, Ordering::Relaxed);
    } else {
        SHELL_FUNCTION_COMPLETED.store(1, Ordering::Relaxed);
    }
    if exit_code == 0 && exit_sig > 0 {
        exit_code = 128 + exit_sig;
    }
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        exit_code,
    );
    define_variable_in_set(
        ctx,
        b".SHELLSTATUS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
        &raw mut buf as *mut ::core::ffi::c_char,
        o_override,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
}
/// Read everything from pipe `fd` into an owned, growing buffer (was the inline
/// xmalloc/xrealloc read loop in `func_shell_base`). Retries on `EINTR`, grows
/// by 512 bytes when full, stops at EOF/error, and NUL-terminates. Returns the
/// buffer (kept fully initialized so a growth preserves bytes already read) and
/// the filled length.
///
/// # Safety
/// `fd` must be a valid readable file descriptor.
unsafe fn read_all_pipe(fd: i32) -> (Vec<u8>, size_t) {
    let mut maxlen: size_t = 200;
    let mut buffer: Vec<u8> = vec![0u8; maxlen.wrapping_add(1) as usize];
    let mut i: size_t = 0;
    loop {
        if i == maxlen {
            maxlen = maxlen.wrapping_add(512);
            buffer.resize(maxlen.wrapping_add(1) as usize, 0);
        }
        let mut cc: i32;
        loop {
            cc = read(
                fd,
                buffer.as_mut_ptr().add(i as usize) as *mut ::core::ffi::c_void,
                (maxlen as size_t).wrapping_sub(i as size_t),
            ) as i32;
            if !(cc == -1_i32 && *__errno_location() == EINTR) {
                break;
            }
        }
        if cc <= 0 {
            break;
        }
        i = i.wrapping_add(cc as size_t);
    }
    *buffer.as_mut_ptr().add(i as usize) = 0;
    (buffer, i)
}

#[cfg(test)]
mod read_all_pipe_tests {
    use super::*;

    /// Drive `read_all_pipe` through a real `pipe(2)`: write `payload` to the
    /// write end (in a thread so a payload larger than the pipe buffer cannot
    /// deadlock), close it to signal EOF, and read from the read end. Returns
    /// the `(buffer, len)` the helper produced.
    unsafe fn run(payload: Vec<u8>) -> (Vec<u8>, size_t) {
        let mut fds = [0_i32; 2];
        assert_eq!(libc::pipe(fds.as_mut_ptr()), 0, "pipe() failed");
        let (rd, wr) = (fds[0], fds[1]);

        let writer = std::thread::spawn(move || {
            let mut off = 0usize;
            while off < payload.len() {
                let n = libc::write(
                    wr,
                    payload.as_ptr().add(off) as *const ::core::ffi::c_void,
                    payload.len() - off,
                );
                if n <= 0 {
                    break;
                }
                off += n as usize;
            }
            libc::close(wr);
        });

        let (buffer, len) = read_all_pipe(rd);
        libc::close(rd);
        writer.join().unwrap();
        (buffer, len)
    }

    /// Empty pipe (immediate EOF): zero length and a NUL at offset 0.
    #[test]
    fn empty_pipe_yields_zero_length_nul_terminated() {
        unsafe {
            let (buffer, len) = run(Vec::new());
            assert_eq!(len, 0);
            assert_eq!(buffer[0], 0, "NUL-terminated at offset 0");
        }
    }

    /// A payload that fits inside the initial 200-byte buffer round-trips
    /// byte-for-byte and is NUL-terminated just past the data.
    #[test]
    fn small_payload_round_trips() {
        unsafe {
            let payload = b"hello from the pipe".to_vec();
            let (buffer, len) = run(payload.clone());
            assert_eq!(len as usize, payload.len());
            assert_eq!(&buffer[..len as usize], &payload[..]);
            assert_eq!(buffer[len as usize], 0, "NUL-terminated past the data");
        }
    }

    /// A payload well past the initial 200 bytes forces the 512-byte growth
    /// path (potentially several times) and must still round-trip exactly.
    #[test]
    fn large_payload_exercises_buffer_growth() {
        unsafe {
            let payload: Vec<u8> = (0..5000u32).map(|n| (n % 251) as u8).collect();
            let (buffer, len) = run(payload.clone());
            assert_eq!(len as usize, payload.len(), "all bytes read across growth");
            assert_eq!(&buffer[..len as usize], &payload[..], "bytes preserved");
            assert_eq!(buffer[len as usize], 0, "NUL-terminated past the data");
        }
    }
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn func_shell_base(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    trim_newlines: i32,
) -> *mut ::core::ffi::c_char {
    let mut child: childbase = childbase {
        cmd_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        environment: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        output: output {
            out: 0,
            err: 0,
            syncout: [0; 1],
            c2rust_padding: [0; 3],
        },
    };
    let mut batch_filename: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let errfd: i32;
    let command_argv: *mut *mut ::core::ffi::c_char;
    let mut pipedes: [i32; 2] = [0; 2];
    let pid: pid_t;
    command_argv = construct_command_argv(
        ctx,
        *argv.offset(0_i32 as isize),
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        None,
        0,
        &raw mut batch_filename,
    );
    if command_argv.is_null() {
        return o;
    }
    crate::output::output_start(ctx);
    errfd = if !output_context.is_null() && (*output_context).err >= 0 {
        (*output_context).err
    } else {
        fileno(stderr)
    };
    child.environment = target_environment(ctx, None, 0);
    if pipe(&raw mut pipedes as *mut i32) < 0 {
        error(
        ctx,
        reading_file,
        strlen(strerror(*__errno_location())) as size_t,
        b"pipe: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
    } else {
        fd_noinherit(pipedes[1_i32 as usize]);
        fd_noinherit(pipedes[0_i32 as usize]);
        child
            .output
            .set_syncout(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        child.output.out = pipedes[1_i32 as usize];
        child.output.err = errfd;
        pid = child_execute_job(ctx, &raw mut child, 1, command_argv);
        if pid < 0 {
            shell_completed(ctx, 127, 0);
        } else {
            SHELL_FUNCTION_PID.store(pid, Ordering::Relaxed);
            SHELL_FUNCTION_COMPLETED.store(0, Ordering::Relaxed);
            if pipedes[1_i32 as usize] >= 0 {
                close(pipedes[1_i32 as usize]);
            }
            let (mut buffer, mut i) = read_all_pipe(pipedes[0_i32 as usize]);
            close(pipedes[0_i32 as usize]);
            while shell_function_completed() == 0 {
                reap_children(ctx, 1, 0);
            }
            if !batch_filename.is_null() {
                if 0x2_i32 & db_level() != 0 {
                    printf(
                        b"Cleaning up temporary batch file %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        batch_filename,
                    );
                    fflush(stdout);
                }
                remove(batch_filename);
                free(batch_filename as *mut ::core::ffi::c_void);
            }
            SHELL_FUNCTION_PID.store(0, Ordering::Relaxed);
            fold_newlines(
                buffer.as_mut_ptr() as *mut ::core::ffi::c_char,
                &raw mut i,
                trim_newlines,
            );
            o = variable_buffer_output(o, buffer.as_mut_ptr() as *mut ::core::ffi::c_char, i);
        }
    }
    if !command_argv.is_null() {
        free(*command_argv.offset(0_i32 as isize) as *mut ::core::ffi::c_void);
        free(command_argv as *mut ::core::ffi::c_void);
    }
    free_childbase(&raw mut child);
    o
}
unsafe fn func_shell(
    ctx: &crate::execctx::ExecContext,
    o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    func_shell_base(ctx, o, argv, 1)
}
pub const ROOT_LEN: i32 = 1;
/// Normalize a path into `out`, mirroring GNU make's `abspath`.
///
/// `name` is the input path token (no trailing NUL required). `starting_dir`
/// is the process working directory used to resolve relative paths; an empty
/// slice means it is unavailable. The result (with a trailing NUL) is written
/// to `out`, which must hold at least `GET_PATH_MAX + 1` bytes.
///
/// Returns `Some(len)` where `out[..len]` is the normalized path, or `None`
/// when the input is empty, a relative path cannot be resolved, or the result
/// would not fit. This is a pure byte transform over slices with no raw
/// pointers; the only `unsafe` boundary lives in the caller that supplies the
/// argv token and the working-directory global.
fn abspath_into(name: &[u8], starting_dir: &[u8], out: &mut [u8]) -> Option<usize> {
    const ROOT: usize = ROOT_LEN as usize;
    // `apath_limit = apath + GET_PATH_MAX`: the index just past the last
    // writable byte, leaving room for the trailing NUL in `out`.
    let limit = out.len().saturating_sub(1);
    if name.is_empty() || name[0] == 0 {
        return None;
    }
    let mut dest: usize;
    // Cursor into `name`, advanced component by component.
    let mut i: usize;
    if name[0] != b'/' {
        // Relative path: seed `out` with the working directory.
        if starting_dir.is_empty() {
            return None;
        }
        let n = starting_dir.len();
        if n >= limit {
            return None;
        }
        out[..n].copy_from_slice(starting_dir);
        out[n] = 0;
        dest = n;
        i = 0;
    } else {
        // Absolute path: copy the leading root separator.
        out[..ROOT].copy_from_slice(&name[..ROOT]);
        out[ROOT] = 0;
        dest = ROOT;
        i = ROOT;
    }
    while i < name.len() && name[i] != 0 {
        // Skip directory separators (MAP_DIRSEP).
        while i < name.len() && name[i] == b'/' {
            i += 1;
        }
        // Scan one component up to the next separator or NUL.
        let start = i;
        while i < name.len() && name[i] != b'/' && name[i] != 0 {
            i += 1;
        }
        let len = i - start;
        if len == 0 {
            break;
        }
        let comp = &name[start..i];
        if len == 1 && comp[0] == b'.' {
            // "." — current directory, drop it.
        } else if len == 2 && comp[0] == b'.' && comp[1] == b'.' {
            // ".." — back up over the previous component.
            if dest > ROOT {
                dest -= 1;
                while dest > 0 && out[dest - 1] != b'/' {
                    dest -= 1;
                }
            }
        } else {
            // Ordinary component: add a separator unless one is already there.
            if !(dest > 0 && out[dest - 1] == b'/') {
                out[dest] = b'/';
                dest += 1;
            }
            if limit - dest <= len {
                return None;
            }
            out[dest..dest + len].copy_from_slice(comp);
            dest += len;
            out[dest] = 0;
        }
    }
    // Strip a trailing separator, but keep the root.
    if dest > ROOT && out[dest - 1] == b'/' {
        dest -= 1;
    }
    out[dest] = 0;
    Some(dest)
}
/// Resolve `token` to its canonical absolute path, mirroring make's
/// `$(realpath)` (libc `realpath` followed by a `stat`): it yields a path only
/// when every component exists. This is safe code with no raw pointers — the
/// argv/FFI plumbing stays in the caller. Returns the resolved bytes (no
/// trailing NUL), or `None` when resolution fails, the result no longer
/// `stat`s, or it would overflow the `PATH_MAX` buffer libc `realpath` uses.
fn realpath_token(token: &[u8]) -> Option<Vec<u8>> {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    // Retry on EINTR, mirroring the C `realpath`/`stat` loops: a signal
    // arriving mid-call must not silently drop the token.
    fn retry_eintr<T>(mut f: impl FnMut() -> std::io::Result<T>) -> std::io::Result<T> {
        loop {
            match f() {
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                other => return other,
            }
        }
    }
    let input = std::path::Path::new(std::ffi::OsStr::from_bytes(token));
    let canon = retry_eintr(|| std::fs::canonicalize(input)).ok()?;
    // C follows `realpath` with `stat(out)`; `canonicalize` already requires
    // existence, but mirror the explicit check to match the C control flow.
    retry_eintr(|| std::fs::metadata(&canon)).ok()?;
    let bytes = canon.into_os_string().into_vec();
    // libc `realpath` writes into a `PATH_MAX` buffer, so an over-long result
    // would fail there (`ENAMETOOLONG`); reject it to match that behavior.
    if bytes.len() >= GET_PATH_MAX as usize {
        return None;
    }
    Some(bytes)
}
unsafe fn func_realpath(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = *argv.offset(0_i32 as isize);
    let mut path: *const ::core::ffi::c_char;
    let mut doneany: i32 = 0;
    let mut len: size_t = 0;
    loop {
        path = find_next_token(&raw mut p, &raw mut len);
        if path.is_null() {
            break;
        }
        if len < GET_PATH_MAX as size_t {
            // Borrow the argv token at the FFI edge; resolution is safe code.
            let token = ::core::slice::from_raw_parts(path as *const u8, len as usize);
            if let Some(resolved) = realpath_token(token) {
                o = variable_buffer_output(
                    o,
                    resolved.as_ptr() as *const ::core::ffi::c_char,
                    resolved.len() as size_t,
                );
                o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
                doneany = 1;
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    o
}
unsafe fn func_file(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut fn_0: *mut ::core::ffi::c_char = *argv.offset(0_i32 as isize);
    if *fn_0.offset(0_i32 as isize) as i32 == '>' as i32 {
        let start: *const ::core::ffi::c_char;
        let nm: *mut ::core::ffi::c_char;
        let mut fp: *mut FILE;
        let mut mode: *const ::core::ffi::c_char =
            b"w\0" as *const u8 as *const ::core::ffi::c_char;
        fn_0 = fn_0.offset(1_i32 as isize);
        if *fn_0.offset(0_i32 as isize) as i32 == '>' as i32 {
            mode = b"a\0" as *const u8 as *const ::core::ffi::c_char;
            fn_0 = fn_0.offset(1_i32 as isize);
        }
        start = next_token(fn_0);
        if *start.offset(0_i32 as isize) as i32 == 0 {
            fatal(ctx, *expanding_var, 0, b"file: missing filename\0" as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        // Bridge to the safe `end_of_token`: the returned offset of the first
        // whitespace/NUL within `[start, NUL)` is exactly the token length.
        let len = end_of_token(::core::slice::from_raw_parts(
            start as *const u8,
            strlen(start),
        )) as size_t;
        alloca_allocations.push(::std::vec::from_elem(0, len.wrapping_add(1) as usize));
        nm = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            nm as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            len as size_t,
        );
        *nm.offset(len as isize) = 0;
        loop {
            *__errno_location() = 0;
            fp = fopen(nm, mode) as *mut FILE;
            if !(fp.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if fp.is_null() {
            fatal(
        ctx,
        reading_file,
        (strlen(nm) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
        b"open: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((nm) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
        }
        crate::make_main::bump_command_count();
        if !(*argv.offset(1_i32 as isize)).is_null() {
            let l: size_t = strlen(*argv.offset(1_i32 as isize)) as size_t;
            let nl: i32 = (l == 0
                || *(*argv.offset(1_i32 as isize)).offset(l.wrapping_sub(1) as isize) as i32
                    != '\n' as i32) as i32;
            if fputs(*argv.offset(1_i32 as isize), fp) == EOF
                || nl != 0 && fputc('\n' as i32, fp) == EOF
            {
                fatal(
        ctx,
        reading_file,
        (strlen(nm) as size_t)
                        .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
        b"write: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((nm) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
            }
        }
        if fclose(fp) != 0 {
            fatal(
        ctx,
        reading_file,
        (strlen(nm) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
        b"close: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((nm) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
        }
    } else if *fn_0.offset(0_i32 as isize) as i32 == '<' as i32 {
        let mut n: size_t = 0;
        let start_0: *const ::core::ffi::c_char;
        let nm_0: *mut ::core::ffi::c_char;
        let mut fp_0: *mut FILE;
        start_0 = next_token(fn_0.offset(1_i32 as isize));
        if *start_0.offset(0_i32 as isize) as i32 == 0 {
            fatal(ctx, *expanding_var, 0, b"file: missing filename\0" as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        if !(*argv.offset(1_i32 as isize)).is_null() {
            fatal(ctx, *expanding_var, 0, b"file: too many arguments\0" as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        // Bridge to the safe `end_of_token`: the returned offset of the first
        // whitespace/NUL within `[start_0, NUL)` is exactly the token length.
        let len_0 = end_of_token(::core::slice::from_raw_parts(
            start_0 as *const u8,
            strlen(start_0),
        )) as size_t;
        alloca_allocations.push(::std::vec::from_elem(0, len_0.wrapping_add(1) as usize));
        nm_0 = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        memcpy(
            nm_0 as *mut ::core::ffi::c_void,
            start_0 as *const ::core::ffi::c_void,
            len_0 as size_t,
        );
        *nm_0.offset(len_0 as isize) = 0;
        loop {
            *__errno_location() = 0;
            fp_0 = fopen(nm_0, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
            if !(fp_0.is_null() && *__errno_location() == EINTR) {
                break;
            }
        }
        if fp_0.is_null() {
            if *__errno_location() == ENOENT {
                if 0x2_i32 & db_level() != 0 {
                    printf(
                        b"file: Failed to open '%s': %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        nm_0,
                        strerror(*__errno_location()),
                    );
                    fflush(stdout);
                }
                return o;
            }
            fatal(
        ctx,
        reading_file,
        (strlen(nm_0) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
        b"open: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((nm_0) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
        }
        loop {
            let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
            let l_0: size_t = fread(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                1,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                fp_0,
            ) as size_t;
            if l_0 > 0 {
                o = variable_buffer_output(o, &raw mut buf as *mut ::core::ffi::c_char, l_0);
                n = n.wrapping_add(l_0);
            }
            if ferror(fp_0) != 0 && *__errno_location() != EINTR {
                fatal(
        ctx,
        reading_file,
        (strlen(nm_0) as size_t)
                        .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
        b"read: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((nm_0) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
            }
            if feof(fp_0) != 0 {
                break;
            }
        }
        if fclose(fp_0) != 0 {
            fatal(
        ctx,
        reading_file,
        (strlen(nm_0) as size_t)
                    .wrapping_add(strlen(strerror(*__errno_location())) as size_t),
        b"close: %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((nm_0) as *const ::core::ffi::c_char),
            FmtArg::Str((strerror(*__errno_location())) as *const ::core::ffi::c_char)],
    );
        }
        if n != 0 && *o.offset(-1_i32 as isize) as i32 == '\n' as i32 {
            o = o.offset(
                -((1 + (n > 1 && *o.offset(-2_i32 as isize) as i32 == '\r' as i32) as i32)
                    as isize),
            );
        }
    } else {
        fatal(
        ctx,
        *expanding_var,
        strlen(fn_0) as size_t,
        b"file: invalid file operation: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((fn_0) as *const ::core::ffi::c_char)],
    );
    }
    o
}
unsafe fn func_abspath(
    _ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = *argv.offset(0_i32 as isize);
    let mut path: *const ::core::ffi::c_char;
    let mut doneany: i32 = 0;
    let mut len: size_t = 0;
    // Resolve the working-directory global once, as bytes, at the FFI edge.
    let starting_dir: &[u8] = if starting_directory.is_null() {
        &[]
    } else {
        ::core::ffi::CStr::from_ptr(starting_directory).to_bytes()
    };
    loop {
        path = find_next_token(&raw mut p, &raw mut len);
        if path.is_null() {
            break;
        }
        if len < GET_PATH_MAX as size_t {
            let mut out: [u8; 4097] = [0; 4097];
            // The argv token is borrowed as a byte slice at the FFI edge; the
            // path normalization itself runs entirely in safe code.
            let name = ::core::slice::from_raw_parts(path as *const u8, len as usize);
            if let Some(out_len) = abspath_into(name, starting_dir, &mut out) {
                o = variable_buffer_output(
                    o,
                    out.as_ptr() as *mut ::core::ffi::c_char,
                    out_len as size_t,
                );
                o = variable_buffer_output(o, b" \0" as *const u8 as *const ::core::ffi::c_char, 1);
                doneany = 1;
            }
        }
    }
    if doneany != 0 {
        o = o.offset(-1_i32 as isize);
    }
    o
}
/// Build a `function_table_entry` at compile time. Replaces the c2rust-
/// translated `run_static_initializers` constructor that ran ~1000 lines of
/// runtime bitfield-setter calls. The bitfield byte layout matches
/// `function_table_entry`'s `BitfieldStruct` derive: bit 0 = `expand_args`,
/// bit 1 = `alloc_fn`, bit 2 = `adds_command`. All static-table entries set
/// only `expand_args`; `alloc_fn` and `adds_command` are zero.
const fn ft_entry(
    name: &'static [u8],
    min: ::core::ffi::c_uchar,
    max: ::core::ffi::c_uchar,
    expand: u8,
    func: unsafe fn(
        &crate::execctx::ExecContext,
        *mut ::core::ffi::c_char,
        *mut *mut ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char,
) -> function_table_entry {
    function_table_entry {
        fptr: C2RustUnnamed {
            func_ptr: Some(func),
        },
        name: name.as_ptr() as *const ::core::ffi::c_char,
        len: (name.len() - 1) as ::core::ffi::c_uchar,
        minimum_args: min,
        maximum_args: max,
        expand_args_alloc_fn_adds_command: [expand & 1],
        c2rust_padding: [0; 4],
    }
}

static mut function_table_init: [function_table_entry; 38] = [
    ft_entry(b"abspath\0", 0, 1, 1, func_abspath),
    ft_entry(b"addprefix\0", 2, 2, 1, func_addsuffix_addprefix),
    ft_entry(b"addsuffix\0", 2, 2, 1, func_addsuffix_addprefix),
    ft_entry(b"and\0", 1, 0, 0, func_and),
    ft_entry(b"basename\0", 0, 1, 1, func_basename_dir),
    ft_entry(b"call\0", 1, 0, 1, func_call),
    ft_entry(b"dir\0", 0, 1, 1, func_basename_dir),
    ft_entry(b"error\0", 0, 1, 1, func_error),
    ft_entry(b"eval\0", 0, 1, 1, func_eval),
    ft_entry(b"file\0", 1, 2, 1, func_file),
    ft_entry(b"filter\0", 2, 2, 1, func_filter_filterout),
    ft_entry(b"filter-out\0", 2, 2, 1, func_filter_filterout),
    ft_entry(b"findstring\0", 2, 2, 1, func_findstring),
    ft_entry(b"firstword\0", 0, 1, 1, func_firstword),
    ft_entry(b"flavor\0", 0, 1, 1, func_flavor),
    ft_entry(b"foreach\0", 3, 3, 0, func_foreach),
    ft_entry(b"if\0", 2, 3, 0, func_if),
    ft_entry(b"info\0", 0, 1, 1, func_error),
    ft_entry(b"intcmp\0", 2, 5, 0, func_intcmp),
    ft_entry(b"join\0", 2, 2, 1, func_join),
    ft_entry(b"lastword\0", 0, 1, 1, func_lastword),
    ft_entry(b"let\0", 3, 3, 0, func_let),
    ft_entry(b"notdir\0", 0, 1, 1, func_notdir_suffix),
    ft_entry(b"or\0", 1, 0, 0, func_or),
    ft_entry(b"origin\0", 0, 1, 1, func_origin),
    ft_entry(b"patsubst\0", 3, 3, 1, func_patsubst),
    ft_entry(b"realpath\0", 0, 1, 1, func_realpath),
    ft_entry(b"shell\0", 0, 1, 1, func_shell),
    ft_entry(b"sort\0", 0, 1, 1, func_sort),
    ft_entry(b"strip\0", 0, 1, 1, func_strip),
    ft_entry(b"subst\0", 3, 3, 1, func_subst),
    ft_entry(b"suffix\0", 0, 1, 1, func_notdir_suffix),
    ft_entry(b"value\0", 0, 1, 1, func_value),
    ft_entry(b"warning\0", 0, 1, 1, func_error),
    ft_entry(b"wildcard\0", 0, 1, 1, func_wildcard),
    ft_entry(b"word\0", 2, 2, 1, func_word),
    ft_entry(b"wordlist\0", 3, 3, 1, func_wordlist),
    ft_entry(b"words\0", 0, 1, 1, func_words),
];
/// Append an alloc-style builtin's freshly `malloc`ed result to the variable
/// buffer, releasing it via the RAII `ExpandedArg` wrapper instead of a manual
/// `free`. Kept separate so the owned buffer's `Drop` scope stays out of the
/// hot `expand_builtin_function` dispatch.
unsafe fn output_owned_result(
    o: *mut ::core::ffi::c_char,
    p: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let owned = ExpandedArg::from_raw(p);
    variable_buffer_output(o, owned.as_ptr(), strlen(owned.as_ptr()) as size_t)
}
unsafe fn expand_builtin_function(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    argc: ::core::ffi::c_uint,
    argv: *mut *mut ::core::ffi::c_char,
    entry_p: *const function_table_entry,
) -> *mut ::core::ffi::c_char {
    let p: *mut ::core::ffi::c_char;
    // SAFETY: `entry_p` is a function-table entry resolved by the caller and is
    // valid for the duration of the call. Bind a checked reference so the field
    // accesses below go through a provably-valid reference, not raw derefs.
    let entry = entry_p
        .as_ref()
        .expect("function_table_entry pointer is non-null");
    if argc < entry.minimum_args as ::core::ffi::c_uint {
        fatal(
            ctx,
            *expanding_var,
            strlen(entry.name) as size_t,
            b"insufficient number of arguments (%u) to function '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                FmtArg::Uint((argc) as u32 as u64),
                FmtArg::Str((entry.name) as *const ::core::ffi::c_char),
            ],
        );
    }
    if argc == 0 && entry.alloc_fn() == 0 {
        return o;
    }
    if entry.fptr.func_ptr.is_none() {
        fatal(
            ctx,
            *expanding_var,
            strlen(entry.name) as size_t,
            b"unimplemented on this platform: function '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[FmtArg::Str((entry.name) as *const ::core::ffi::c_char)],
        );
    }
    if entry.adds_command() != 0 {
        crate::make_main::bump_command_count();
    }
    if entry.alloc_fn() == 0 {
        return entry.fptr.func_ptr.expect("non-null function pointer")(
            ctx,
            o,
            argv,
            entry.name,
        );
    }
    p = entry
        .fptr
        .alloc_func_ptr
        .expect("non-null function pointer")(entry.name, argc, argv);
    if !p.is_null() {
        o = output_owned_result(o, p);
    }
    o
}
/// Build the owned, NUL-terminated working buffer that `handle_function` uses
/// to split a non-`expand_args` function's argument list in place.
///
/// This is the RAII replacement for the original `xmalloc(len + 1)` +
/// `mempcpy(abeg, beg, len)` + trailing `'\0'` + `free(abeg)` sequence: the
/// returned `Vec<u8>` owns exactly `src.len() + 1` bytes (the copy plus the
/// terminator) and frees itself on drop. It is a real, mutable allocation
/// (`Vec`, not `CString`) because the caller rewrites it in place while
/// carving out argument substrings.
fn copy_args_buffer(src: &[u8]) -> Vec<u8> {
    // The original `xmalloc(len + 1)` routed allocation failure through make's
    // `out_of_memory()` ("virtual memory exhausted", make's own exit status)
    // rather than aborting the process the way plain `Vec` growth does. Mirror
    // that: reserve exactly `len + 1` bytes up front and, on reservation
    // failure, route through make's `out_of_memory()`. The subsequent
    // `extend`/`push` cannot reallocate because the capacity is already
    // guaranteed.
    let total = src.len() + 1;
    let mut v: Vec<u8> = Vec::new();
    if v.try_reserve_exact(total).is_err() {
        out_of_memory();
    }
    v.extend_from_slice(src);
    v.push(0);
    v
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn handle_function(
    ctx: &crate::execctx::ExecContext,
    op: *mut *mut ::core::ffi::c_char,
    stringp: *mut *const ::core::ffi::c_char,
) -> i32 {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let entry_p: *const function_table_entry;
    let openparen: ::core::ffi::c_char = *(*stringp).offset(0_i32 as isize);
    let closeparen: ::core::ffi::c_char = (if openparen as i32 == '(' as i32 {
        ')' as i32
    } else {
        '}' as i32
    }) as ::core::ffi::c_char;
    let mut beg: *const ::core::ffi::c_char;
    let mut end: *const ::core::ffi::c_char;
    let mut count: i32 = 0;
    let argv: *mut *mut ::core::ffi::c_char;
    let mut argvp: *mut *mut ::core::ffi::c_char;
    let mut nargs: ::core::ffi::c_uint;
    beg = (*stringp).offset(1_i32 as isize);
    entry_p = lookup_function(beg);
    if entry_p.is_null() {
        return 0;
    }
    // SAFETY: `entry_p` was just checked non-null and points to a valid
    // function-table entry. Bind a checked reference so the field reads below
    // go through a provably-valid reference rather than raw pointer derefs.
    let entry = entry_p
        .as_ref()
        .expect("function_table_entry pointer is non-null");
    beg = beg.offset(entry.len as i32 as isize);
    while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
        .offset(*beg as ::core::ffi::c_uchar as isize) as i32
        & (0x2_i32 | 0x4_i32)
        != 0
    {
        beg = beg.offset(1_i32 as isize);
    }
    nargs = 1;
    end = beg;
    while *end as i32 != 0 {
        if *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
            .offset(*end as ::core::ffi::c_uchar as isize) as i32
            & (0x80_i32 | 0x400_i32)
            != 0
        {
            if *end as i32 == ',' as i32 {
                nargs = nargs.wrapping_add(1);
            } else if *end as i32 == openparen as i32 {
                count += 1;
            } else if *end as i32 == closeparen as i32 && {
                count -= 1;
                count < 0
            } {
                break;
            }
        }
        end = end.offset(1_i32 as isize);
    }
    if count >= 0 {
        fatal(
        ctx,
        *expanding_var,
        strlen(entry.name) as size_t,
        b"unterminated call to function '%s': missing '%c'\0" as *const u8
                as *const ::core::ffi::c_char,
        &[FmtArg::Str((entry.name) as *const ::core::ffi::c_char),
            FmtArg::Int((closeparen as i32) as i64)],
    );
    }
    *stringp = end;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            .wrapping_mul(nargs.wrapping_add(2) as usize) as usize,
    ));
    argv = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_char;
    argvp = argv;
    if entry.expand_args() != 0 {
        let mut p: *const ::core::ffi::c_char;
        p = beg;
        nargs = 0;
        while p <= end {
            let mut next: *const ::core::ffi::c_char;
            nargs = nargs.wrapping_add(1);
            if nargs == entry.maximum_args as ::core::ffi::c_uint || {
                let span = end as usize - p as usize;
                let bytes = ::core::slice::from_raw_parts(p as *const u8, span);
                next = match find_next_argument(openparen as u8, closeparen as u8, bytes) {
                    Some(i) => p.add(i),
                    None => ::core::ptr::null(),
                };
                next.is_null()
            } {
                next = end;
            }
            *argvp = expand_argument(ctx, p, next);
            p = next.offset(1_i32 as isize);
            argvp = argvp.offset(1_i32 as isize);
        }
    } else {
        let len: size_t = end.offset_from(beg) as ::core::ffi::c_long as size_t;
        let mut p_0: *mut ::core::ffi::c_char;
        let aend: *mut ::core::ffi::c_char;
        // Owned, genuinely-mutable copy of `[beg, end)` plus a NUL terminator
        // (was `xmalloc(len+1)` + `mempcpy` + `free(abeg)`). The arg-splitter
        // below rewrites this buffer in place (`*next_0 = 0`) and `argv` holds
        // borrows into it, so it must be a real `Vec<u8>` (mutable) -- a
        // `CString::as_ptr()` here would be UB. Pushing it onto
        // `alloca_allocations` ties its lifetime to the function, dropping it
        // (the former `free`) only after `expand_builtin_function` has consumed
        // the borrowed `argv` slices.
        alloca_allocations.push(copy_args_buffer(::core::slice::from_raw_parts(
            beg as *const u8,
            len as usize,
        )));
        let abeg: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        aend = abeg.add(len as usize);
        p_0 = abeg;
        nargs = 0;
        while p_0 <= aend {
            let mut next_0: *mut ::core::ffi::c_char;
            nargs = nargs.wrapping_add(1);
            if nargs == entry.maximum_args as ::core::ffi::c_uint || {
                let span = aend as usize - p_0 as usize;
                let bytes = ::core::slice::from_raw_parts(p_0 as *const u8, span);
                next_0 = match find_next_argument(openparen as u8, closeparen as u8, bytes) {
                    Some(i) => p_0.add(i),
                    None => ::core::ptr::null_mut(),
                };
                next_0.is_null()
            } {
                next_0 = aend;
            }
            *argvp = p_0;
            *next_0.as_mut().expect("split_args: next_0 is null") = 0;
            p_0 = next_0.offset(1_i32 as isize);
            argvp = argvp.offset(1_i32 as isize);
        }
    }
    *argvp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *op = expand_builtin_function(ctx, *op, nargs, argv, entry_p);
    if entry.expand_args() != 0 {
        argvp = argv;
        while !(*argvp).is_null() {
            free(*argvp as *mut ::core::ffi::c_void);
            argvp = argvp.offset(1_i32 as isize);
        }
    }
    // In the non-expand-args branch the former `free(abeg)` is now handled by
    // `alloca_allocations` dropping at end of scope (RAII).
    1
}
unsafe fn func_call(
    ctx: &crate::execctx::ExecContext,
    mut o: *mut ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut _funcname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // Highest `$(call)` argument index seen so far, saved/restored around the
    // recursive expansion below so nested calls clear the right `$N` automatic
    // variables. Stored in an atomic (was a `static mut`) so its reads/writes
    // are plain safe operations; all access is single-threaded, so `Relaxed`
    // preserves the original program order.
    static MAX_ARGS: AtomicU32 = AtomicU32::new(0);
    let fname: *mut ::core::ffi::c_char;
    let flen: size_t;
    let mut i: ::core::ffi::c_uint;
    let entry_p: *const function_table_entry;
    let v: *mut variable;
    fname = next_token(*argv.offset(0_i32 as isize));
    // Bridge to the safe `end_of_token`: terminate the function name by writing
    // a NUL at `fname + token_len` (offset of the first whitespace/NUL).
    let fname_eot = fname.add(end_of_token(::core::slice::from_raw_parts(
        fname as *const u8,
        strlen(fname),
    )));
    *fname_eot = 0;
    if *fname as i32 == 0 {
        return o;
    }
    entry_p = lookup_function(fname);
    if !entry_p.is_null() {
        i = 0;
        while !(*argv.offset(i.wrapping_add(1) as isize)).is_null() {
            i = i.wrapping_add(1);
        }
        return expand_builtin_function(ctx, o, i, argv.offset(1_i32 as isize), entry_p);
    }
    flen = strlen(fname) as size_t;
    v = lookup_variable(ctx, fname, flen);
    if v.is_null() {
        // SAFETY: `fname` points to `flen` valid bytes (length precomputed
        // above via `strlen`); read-only bridge to the safe `warn_undefined`.
        warn_undefined(ctx, ::core::slice::from_raw_parts(fname as *const u8, flen));
    }
    if v.is_null() || *(*v).value as i32 == 0 {
        return o;
    }
    push_new_variable_scope();
    i = 0;
    while !(*argv).is_null() {
        let mut num: [::core::ffi::c_char; 22] = [0; 22];
        define_variable_in_set(
            ctx,
            &raw mut num as *mut ::core::ffi::c_char,
            sprintf(
                &raw mut num as *mut ::core::ffi::c_char,
                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            ) as size_t,
            *argv,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        i = i.wrapping_add(1);
        argv = argv.offset(1_i32 as isize);
    }
    while i < MAX_ARGS.load(Ordering::Relaxed) {
        let mut num_0: [::core::ffi::c_char; 22] = [0; 22];
        define_variable_in_set(
            ctx,
            &raw mut num_0 as *mut ::core::ffi::c_char,
            sprintf(
                &raw mut num_0 as *mut ::core::ffi::c_char,
                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                i,
            ) as size_t,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_automatic,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
        i = i.wrapping_add(1);
    }
    (*v).set_exp_count(EXP_COUNT_MAX as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let saved_args = MAX_ARGS.load(Ordering::Relaxed) as i32;
    MAX_ARGS.store(i, Ordering::Relaxed);
    o = expand_variable_output(ctx, o, fname, flen);
    MAX_ARGS.store(saved_args as ::core::ffi::c_uint, Ordering::Relaxed);
    (*v).set_exp_count(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    pop_variable_scope();
    o.offset(strlen(o) as isize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn define_new_function(
    ctx: &crate::execctx::ExecContext,
    flocp: *const Floc,
    name: *const ::core::ffi::c_char,
    min: ::core::ffi::c_uint,
    max: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
    func: gmk_func_ptr,
) {
    let mut e: *const ::core::ffi::c_char = name;
    let mut ent: *mut function_table_entry;
    let len: size_t;
    while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
        .offset(*e as ::core::ffi::c_uchar as isize) as i32
        & 0x2000_i32
        != 0
    {
        e = e.offset(1_i32 as isize);
    }
    len = e.offset_from(name) as ::core::ffi::c_long as size_t;
    if len == 0 {
        fatal(ctx, flocp, 0, b"empty function name\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    }
    if *name as i32 == '.' as i32 || *e as i32 != 0 {
        fatal(
        ctx,
        flocp,
        strlen(name) as size_t,
        b"invalid function name: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((name) as *const ::core::ffi::c_char)],
    );
    }
    if len > 255 {
        fatal(
        ctx,
        flocp,
        strlen(name) as size_t,
        b"function name too long: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((name) as *const ::core::ffi::c_char)],
    );
    }
    if min > 255 {
        fatal(
            ctx,
            flocp,
            INTSTR_LENGTH.wrapping_add(strlen(name) as size_t),
            b"invalid minimum argument count (%u) for function %s\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                FmtArg::Uint((min) as u32 as u64),
                FmtArg::Str((name) as *const ::core::ffi::c_char),
            ],
        );
    }
    if max > 255 || max != 0 && max < min {
        fatal(
            ctx,
            flocp,
            INTSTR_LENGTH.wrapping_add(strlen(name) as size_t),
            b"invalid maximum argument count (%u) for function %s\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                FmtArg::Uint((max) as u32 as u64),
                FmtArg::Str((name) as *const ::core::ffi::c_char),
            ],
        );
    }
    ent = xmalloc(::core::mem::size_of::<function_table_entry>() as size_t)
        as *mut function_table_entry;
    (*ent).name = strcache_add(name);
    (*ent).len = len as ::core::ffi::c_uchar;
    (*ent).minimum_args = min as ::core::ffi::c_uchar;
    (*ent).maximum_args = max as ::core::ffi::c_uchar;
    (*ent).set_expand_args(
        (if flags & 0x1 as ::core::ffi::c_uint != 0 {
            0
        } else {
            1
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    (*ent).set_alloc_fn(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ent).set_adds_command(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ent).fptr.alloc_func_ptr = func;
    ent = hash_insert(&raw mut function_table, ent as *const ::core::ffi::c_void)
        as *mut function_table_entry;
    free(ent as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn hash_init_function_table() {
    hash_init(
        &raw mut function_table,
        (::core::mem::size_of::<[function_table_entry; 38]>() as ::core::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<function_table_entry>() as ::core::ffi::c_ulong)
            .wrapping_mul(2),
        Some(function_table_entry_hash_1),
        Some(function_table_entry_hash_2),
        Some(function_table_entry_hash_cmp),
    );
    hash_load(
        &raw mut function_table,
        &raw const function_table_init as *const function_table_entry as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[function_table_entry; 38]>() as ::core::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<function_table_entry>() as ::core::ffi::c_ulong),
        ::core::mem::size_of::<function_table_entry>() as ::core::ffi::c_ulong,
    );
}

#[cfg(test)]
mod ft_init_tests {
    use super::*;

    /// Verify that the bitfield byte we wrote in `ft_entry` round-trips
    /// through the `BitfieldStruct`-generated getters. This catches any
    /// drift between our hand-packed byte layout and the c2rust_bitfields
    /// crate's expected encoding.
    #[test]
    fn bitfield_byte_matches_getters() {
        let entries = unsafe {
            std::slice::from_raw_parts(
                (&raw const function_table_init).cast::<function_table_entry>(),
                38,
            )
        };
        for (i, e) in entries.iter().enumerate() {
            let byte = e.expand_args_alloc_fn_adds_command[0];
            let exp = e.expand_args();
            let alloc = e.alloc_fn();
            let adds = e.adds_command();
            let reconstructed = (exp & 1) | ((alloc & 1) << 1) | ((adds & 1) << 2);
            assert_eq!(
                byte as u32, reconstructed,
                "idx {i}: byte {byte:#04x} != getters(exp={exp}, alloc={alloc}, adds={adds})"
            );
            assert!(
                alloc == 0 && adds == 0,
                "idx {i}: alloc/adds expected zero in static table"
            );
        }
    }
}

#[cfg(test)]
mod parse_numeric_tests {
    use super::{classify_numeric, NumParse};

    #[test]
    fn classifies_valid_integers() {
        assert_eq!(classify_numeric(b"0"), NumParse::Ok(0));
        assert_eq!(classify_numeric(b"42"), NumParse::Ok(42));
        assert_eq!(classify_numeric(b"007"), NumParse::Ok(7));
        assert_eq!(classify_numeric(b"+5"), NumParse::Ok(5));
        assert_eq!(classify_numeric(b"-5"), NumParse::Ok(-5));
        // Leading/trailing make-whitespace (incl. \v and \f) is stripped.
        assert_eq!(classify_numeric(b"  12\t\n"), NumParse::Ok(12));
        assert_eq!(classify_numeric(b"\x0b\x0c9\x0c"), NumParse::Ok(9));
        assert_eq!(
            classify_numeric(&i64::MAX.to_string().into_bytes()),
            NumParse::Ok(i64::MAX)
        );
        assert_eq!(
            classify_numeric(&i64::MIN.to_string().into_bytes()),
            NumParse::Ok(i64::MIN)
        );
    }

    #[test]
    fn classifies_empty_as_empty() {
        assert_eq!(classify_numeric(b""), NumParse::Empty);
        assert_eq!(classify_numeric(b"   \t \n"), NumParse::Empty);
    }

    #[test]
    fn classifies_overflow_as_out_of_range() {
        assert_eq!(
            classify_numeric(b"9223372036854775808"),
            NumParse::OutOfRange
        ); // MAX+1
        assert_eq!(
            classify_numeric(b"-9223372036854775809"),
            NumParse::OutOfRange
        ); // MIN-1
        assert_eq!(
            classify_numeric(b"99999999999999999999999"),
            NumParse::OutOfRange
        );
        // Range check precedes the trailing-garbage check, mirroring strtoll+errno.
        assert_eq!(
            classify_numeric(b"99999999999999999999abc"),
            NumParse::OutOfRange
        );
    }

    #[test]
    fn classifies_malformed_as_invalid() {
        assert_eq!(classify_numeric(b"x"), NumParse::Invalid);
        assert_eq!(classify_numeric(b"+"), NumParse::Invalid);
        assert_eq!(classify_numeric(b"-"), NumParse::Invalid);
        assert_eq!(classify_numeric(b"12abc"), NumParse::Invalid); // trailing junk
        assert_eq!(classify_numeric(b"0x10"), NumParse::Invalid); // base 10 only
        assert_eq!(classify_numeric(b"1 2"), NumParse::Invalid); // internal whitespace
    }
}

#[cfg(test)]
mod classify_textint_tests {
    use super::{classify_textint, TextInt};

    /// Reduce a parse to `(sign, significant_digits)` for easy assertions, or
    /// a marker string for the error cases.
    fn parse(s: &str) -> Result<(i32, &str), &'static str> {
        match classify_textint(s.as_bytes()) {
            TextInt::Empty => Err("empty"),
            TextInt::NotNumeric => Err("not-numeric"),
            TextInt::Parsed {
                sign,
                num_start,
                num_end,
            } => Ok((sign, &s[num_start..num_end])),
        }
    }

    #[test]
    fn parses_signs_and_strips_leading_zeros() {
        assert_eq!(parse("12"), Ok((1, "12")));
        assert_eq!(parse("+12"), Ok((1, "12")));
        assert_eq!(parse("-12"), Ok((-1, "12")));
        assert_eq!(parse("0012"), Ok((1, "12"))); // leading zeros stripped
        assert_eq!(parse("-0012"), Ok((-1, "12")));
    }

    #[test]
    fn zero_has_sign_zero_and_no_significant_digits() {
        assert_eq!(parse("0"), Ok((0, "")));
        assert_eq!(parse("000"), Ok((0, "")));
        assert_eq!(parse("-0"), Ok((0, "")));
        assert_eq!(parse("+000"), Ok((0, "")));
    }

    #[test]
    fn surrounding_whitespace_is_allowed() {
        // The token slice begins after next_token, but trailing whitespace
        // (any MAP_SPACE byte, including vertical tab) is still permitted.
        assert_eq!(parse("12\t"), Ok((1, "12")));
        assert_eq!(parse("12\u{0b}"), Ok((1, "12"))); // vertical tab
        assert_eq!(parse("7 "), Ok((1, "7")));
    }

    #[test]
    fn empty_and_non_numeric_are_rejected() {
        assert_eq!(parse(""), Err("empty"));
        assert_eq!(parse("-"), Err("not-numeric")); // sign with no digits
        assert_eq!(parse("+"), Err("not-numeric"));
        assert_eq!(parse("abc"), Err("not-numeric"));
        assert_eq!(parse("12abc"), Err("not-numeric")); // trailing junk
        assert_eq!(parse("1 2"), Err("not-numeric")); // internal whitespace
    }
}

#[cfg(test)]
mod pattern_matches_tests {
    use super::pattern_matches_parts;

    /// Match `pattern` (containing a single `%`) against `s` using the same
    /// prefix/suffix split that `pattern_matches` computes.
    fn matches(pattern: &str, s: &str) -> bool {
        let (prefix, suffix) = pattern.split_once('%').expect("pattern needs a %");
        pattern_matches_parts(prefix.as_bytes(), suffix.as_bytes(), s.as_bytes())
    }

    #[test]
    fn percent_matches_any_run() {
        // "%" alone matches everything, including the empty string.
        assert!(matches("%", ""));
        assert!(matches("%", "anything"));
        // Prefix + suffix with the wildcard filled in.
        assert!(matches("%.o", "foo.o"));
        assert!(matches("lib%.a", "libfoo.a"));
        // The wildcard may match an empty run.
        assert!(matches("%.o", ".o"));
        assert!(matches("a%b", "ab"));
    }

    #[test]
    fn non_matches() {
        assert!(!matches("%.o", "foo.c")); // wrong suffix
        assert!(!matches("lib%.a", "foo.a")); // wrong prefix
        assert!(!matches("%.o", "o")); // suffix longer than string
                                       // Overlapping literals must not match a too-short string.
        assert!(!matches("ab%bc", "abc"));
        assert!(matches("ab%bc", "abbc"));
    }
}

#[cfg(test)]
mod compare_textint_tests {
    use super::compare_textint;

    fn sgn(x: i32) -> i32 {
        x.signum()
    }

    #[test]
    fn positive_beats_negative_and_zero() {
        assert_eq!(sgn(compare_textint(1, b"3", -1, b"3")), 1);
        assert_eq!(sgn(compare_textint(1, b"3", 0, b"0")), 1);
        assert_eq!(sgn(compare_textint(0, b"0", -1, b"3")), 1);
        assert_eq!(sgn(compare_textint(-1, b"3", 1, b"3")), -1);
    }

    #[test]
    fn equal_values_compare_equal() {
        assert_eq!(compare_textint(1, b"42", 1, b"42"), 0);
        assert_eq!(compare_textint(0, b"0", 0, b"0"), 0);
        assert_eq!(compare_textint(-1, b"7", -1, b"7"), 0);
    }

    #[test]
    fn longer_magnitude_is_larger_when_positive() {
        // 100 > 99: more digits wins before any byte compare.
        assert_eq!(sgn(compare_textint(1, b"100", 1, b"99")), 1);
        assert_eq!(sgn(compare_textint(1, b"99", 1, b"100")), -1);
    }

    #[test]
    fn equal_length_falls_back_to_digit_bytes() {
        assert_eq!(sgn(compare_textint(1, b"19", 1, b"21")), -1);
        assert_eq!(sgn(compare_textint(1, b"21", 1, b"19")), 1);
    }

    #[test]
    fn negative_magnitude_order_is_reversed() {
        // -100 < -99: the larger magnitude is the smaller number.
        assert_eq!(sgn(compare_textint(-1, b"100", -1, b"99")), -1);
        assert_eq!(sgn(compare_textint(-1, b"19", -1, b"21")), 1);
    }
}

#[cfg(test)]
mod tokens_tests {
    use super::tokens;

    fn collect(s: &[u8]) -> Vec<&[u8]> {
        tokens(s).collect()
    }

    #[test]
    fn splits_on_runs_of_whitespace() {
        assert_eq!(
            collect(b"  foo\tbar \n baz  "),
            vec![b"foo".as_slice(), b"bar".as_slice(), b"baz".as_slice()]
        );
    }

    #[test]
    fn empty_and_all_whitespace_yield_nothing() {
        assert_eq!(tokens(b"").count(), 0);
        assert_eq!(tokens(b" \t\n\x0b\x0c\r ").count(), 0);
    }

    #[test]
    fn count_first_last_and_nth_match_word_semantics() {
        let s = b"alpha beta gamma delta";
        assert_eq!(tokens(s).count(), 4);
        assert_eq!(tokens(s).next(), Some(b"alpha".as_slice())); // $(firstword ...)
        assert_eq!(tokens(s).next_back(), Some(b"delta".as_slice())); // $(lastword ...)
        assert_eq!(tokens(s).nth(2), Some(b"gamma".as_slice())); // $(word 3,...) -> nth(2)
        assert_eq!(tokens(s).nth(99), None);
    }

    #[test]
    fn all_six_map_space_bytes_separate() {
        // space, tab, newline, vtab, formfeed, carriage-return.
        assert_eq!(collect(b"a\x20b\x09c\x0ad\x0be\x0cf\x0dg").len(), 7);
    }
}

#[cfg(test)]
mod word_span_tests {
    use super::word_span;

    #[test]
    fn spans_preserve_original_separators() {
        let s = b"a  b\tc   d";
        // words 2..=3 -> "b\tc" with the original tab between them.
        assert_eq!(word_span(s, 2, 3), Some(b"b\tc".as_slice()));
        // single word.
        assert_eq!(word_span(s, 1, 1), Some(b"a".as_slice()));
        // whole list (leading/trailing spaces trimmed to word bounds).
        assert_eq!(word_span(s, 1, 4), Some(b"a  b\tc   d".as_slice()));
    }

    #[test]
    fn stop_is_clamped_to_last_word() {
        let s = b"one two three";
        assert_eq!(word_span(s, 2, 99), Some(b"two three".as_slice()));
    }

    #[test]
    fn start_past_end_yields_none() {
        assert_eq!(word_span(b"a b", 3, 5), None);
        assert_eq!(word_span(b"   ", 1, 2), None);
    }

    #[test]
    fn empty_or_inverted_range_yields_none() {
        assert_eq!(word_span(b"a b c", 3, 2), None); // stop < start
        assert_eq!(word_span(b"a b c", 0, 2), None); // 0 is not a valid 1-based index
    }
}

#[cfg(test)]
mod alpha_cmp_tests {
    use super::alpha_cmp;
    use core::cmp::Ordering;

    #[test]
    fn ascii_is_lexicographic() {
        assert_eq!(alpha_cmp(b"abc", b"abd"), Ordering::Less);
        assert_eq!(alpha_cmp(b"abc", b"abc"), Ordering::Equal);
        assert_eq!(alpha_cmp(b"ab", b"abc"), Ordering::Less); // strcmp: shorter < longer
        assert_eq!(alpha_cmp(b"B", b"a"), Ordering::Less); // 'B'(66) < 'a'(97)
    }

    #[test]
    fn differing_first_byte_follows_char_signedness() {
        // The first differing byte is promoted through `c_char`, so the order
        // of a high-bit byte vs. ASCII tracks the target's char signedness
        // (matching make's alpha_compare). Derive the expectation the same way
        // so the test holds on both signed- and unsigned-char targets.
        let hi = 0x80u8 as ::core::ffi::c_char as i32;
        let a = b'A' as i32;
        assert_eq!(alpha_cmp(&[0x80], b"A"), hi.cmp(&a));
        assert_eq!(alpha_cmp(b"A", &[0x80]), a.cmp(&hi));
        // Two high bytes order by their (equally-promoted) values.
        assert_eq!(alpha_cmp(&[0x81], &[0x82]), Ordering::Less);
    }

    #[test]
    fn equal_first_byte_falls_back_to_unsigned_rest() {
        // First bytes equal -> strcmp over the remainder (unsigned).
        assert_eq!(alpha_cmp(&[b'a', 0x80], &[b'a', 0x10]), Ordering::Greater);
    }

    #[test]
    fn sort_then_dedup_matches_word_set() {
        let mut v: Vec<&[u8]> = vec![b"b".as_slice(), b"a", b"b", b"c", b"a"];
        v.sort_by(|a, b| alpha_cmp(a, b));
        v.dedup();
        assert_eq!(v, vec![b"a".as_slice(), b"b", b"c"]);
    }
}

#[cfg(test)]
mod trim_whitespace_span_tests {
    use super::{is_map_space, trim_whitespace_span};

    /// Trim under the C-locale `stopchar_map` classification (`is_map_space`),
    /// which is what `strip_whitespace`'s runtime `stop_set` resolves to there.
    fn trim(s: &[u8]) -> (usize, usize) {
        trim_whitespace_span(s, is_map_space)
    }

    /// Trim `s` and return the surviving byte slice, mirroring what
    /// `strip_whitespace` leaves between its cursors.
    fn trimmed(s: &[u8]) -> &[u8] {
        let (lead, trail) = trim(s);
        &s[lead..s.len() - trail]
    }

    #[test]
    fn no_whitespace_is_unchanged() {
        assert_eq!(trim(b"abc"), (0, 0));
        assert_eq!(trim(b"a"), (0, 0));
    }

    #[test]
    fn trims_each_end() {
        assert_eq!(trim(b"  abc"), (2, 0));
        assert_eq!(trim(b"abc  "), (0, 2));
        assert_eq!(trim(b" abc "), (1, 1));
        assert_eq!(trimmed(b" \tabc\t "), b"abc");
    }

    #[test]
    fn all_whitespace_consumed_by_leading_scan() {
        // The leading scan eats the whole span, leaving nothing for the trailing
        // scan — exactly the empty-result state the C loops produce.
        assert_eq!(trim(b"   "), (3, 0));
        assert_eq!(trimmed(b"   "), b"");
        assert_eq!(trim(b""), (0, 0));
    }

    #[test]
    fn covers_full_isspace_set_but_not_nul() {
        // space, tab, newline, vtab, formfeed, carriage-return are whitespace;
        // NUL is not (stopchar_map()[0] is MAP_NUL only).
        assert_eq!(trim(b"\t\n a \r\x0b"), (3, 3));
        assert_eq!(trimmed(b"\t\n a \r\x0b"), b"a");
        assert_eq!(trim(b"\0"), (0, 0));
    }

    #[test]
    fn classifier_is_honoured() {
        // The helper trims whatever the injected predicate marks; here only 'x'
        // is "whitespace", proving the wrapper's runtime classifier drives it.
        assert_eq!(trim_whitespace_span(b"xxabcxx", |c| c == b'x'), (2, 2));
    }
}

#[cfg(test)]
mod fold_newlines_tests {
    use super::fold_newlines_bytes;

    /// Fold a copy of `s` and return the resulting bytes as a `String`.
    fn fold(s: &[u8], trim: bool) -> String {
        let mut buf = s.to_vec();
        let n = fold_newlines_bytes(&mut buf, trim);
        String::from_utf8(buf[..n].to_vec()).unwrap()
    }

    #[test]
    fn internal_newline_becomes_space() {
        // Internal newlines fold to single spaces; the trailing newline is
        // dropped under trim and kept-as-nothing here too ("a b" either way).
        assert_eq!(fold(b"a\nb\n", true), "a b");
        assert_eq!(fold(b"a\nb\n", false), "a b");
        assert_eq!(fold(b"abc", true), "abc");
    }

    #[test]
    fn crlf_drops_the_cr() {
        // "\r\n" folds to one space; an internal CRLF yields a single space.
        assert_eq!(fold(b"a\r\nb", true), "a b");
        assert_eq!(fold(b"\r\n", true), "");
        // A lone trailing '\r' is an ordinary (non-newline) byte, so it stays.
        assert_eq!(fold(b"a\rb", true), "a\rb");
    }

    #[test]
    fn trim_controls_trailing_spaces() {
        // trim cuts right after the last non-newline byte; without trim at most
        // one folded space survives a single trailing newline...
        assert_eq!(fold(b"a\n\n", true), "a");
        assert_eq!(fold(b"a\n\n", false), "a ");
        // ...and the C `dst - 2` rule keeps two for a longer trailing run.
        assert_eq!(fold(b"ab\n\n\n", true), "ab");
        assert_eq!(fold(b"ab\n\n\n", false), "ab  ");
    }

    #[test]
    fn empty_and_all_newlines() {
        assert_eq!(fold(b"", true), "");
        assert_eq!(fold(b"\n", true), "");
        assert_eq!(fold(b"\n", false), "");
    }

    #[test]
    fn stops_at_embedded_nul() {
        // The C loop runs `while *src`, so an embedded NUL ends processing.
        let mut buf = b"a\nb\0c\nd".to_vec();
        let n = fold_newlines_bytes(&mut buf, true);
        assert_eq!(&buf[..n], b"a b");
    }
}

#[cfg(test)]
mod abspath_tests {
    use super::abspath_into;

    /// Normalize `name` against `cwd` and return the result as a `String`.
    fn abs(name: &[u8], cwd: &[u8]) -> Option<String> {
        let mut out = [0u8; 4097];
        abspath_into(name, cwd, &mut out).map(|n| String::from_utf8(out[..n].to_vec()).unwrap())
    }

    #[test]
    fn absolute_paths_are_collapsed() {
        assert_eq!(abs(b"/usr/bin", b"/home"), Some("/usr/bin".into()));
        // Redundant separators collapse to one.
        assert_eq!(abs(b"/usr//bin", b"/home"), Some("/usr/bin".into()));
        assert_eq!(abs(b"/", b"/home"), Some("/".into()));
        // A trailing slash is stripped (but the root is preserved above).
        assert_eq!(abs(b"/usr/bin/", b"/home"), Some("/usr/bin".into()));
    }

    #[test]
    fn dot_components_are_dropped() {
        assert_eq!(abs(b"/usr/./bin", b"/home"), Some("/usr/bin".into()));
        assert_eq!(abs(b"/./usr", b"/home"), Some("/usr".into()));
    }

    #[test]
    fn dotdot_backs_up_one_component() {
        assert_eq!(abs(b"/usr/lib/../bin", b"/home"), Some("/usr/bin".into()));
        // ".." at the root stays at the root.
        assert_eq!(abs(b"/../usr", b"/home"), Some("/usr".into()));
        assert_eq!(abs(b"/..", b"/home"), Some("/".into()));
    }

    #[test]
    fn relative_paths_resolve_against_cwd() {
        assert_eq!(abs(b"bin", b"/usr"), Some("/usr/bin".into()));
        assert_eq!(abs(b"./bin", b"/usr"), Some("/usr/bin".into()));
        assert_eq!(abs(b"../lib", b"/usr/bin"), Some("/usr/lib".into()));
    }

    #[test]
    fn empty_input_or_missing_cwd_yields_none() {
        assert_eq!(abs(b"", b"/home"), None);
        assert_eq!(abs(&[0], b"/home"), None);
        // A relative path with no working directory cannot be resolved.
        assert_eq!(abs(b"bin", b""), None);
    }

    #[test]
    fn input_stops_at_embedded_nul() {
        // The token is logically NUL-terminated, mirroring the C scan.
        assert_eq!(abs(b"/usr\0/bin", b"/home"), Some("/usr".into()));
    }

    #[test]
    fn overflowing_result_yields_none() {
        // A component that would not fit before the limit returns None rather
        // than writing past the buffer.
        let mut out = [0u8; 8]; // limit == 7
        let long = b"/aaaaaaaaaa";
        assert_eq!(abspath_into(long, b"/home", &mut out), None);
    }
}

#[cfg(test)]
mod realpath_tests {
    use super::realpath_token;

    #[test]
    fn root_resolves_to_itself() {
        // "/" is canonical and always exists.
        assert_eq!(realpath_token(b"/"), Some(b"/".to_vec()));
    }

    #[test]
    fn nonexistent_path_yields_none() {
        assert_eq!(realpath_token(b"/no/such/path/xyzzy_makers_test"), None);
    }

    #[test]
    fn resolves_real_directory_and_collapses_dots() {
        // A directory that exists on every unix; "." / redundant separators are
        // collapsed by canonicalization, matching libc realpath.
        let tmp = std::env::temp_dir();
        let canon = std::fs::canonicalize(&tmp).unwrap();
        let expected = canon.as_os_str().as_encoded_bytes().to_vec();

        let mut noisy = tmp.as_os_str().as_encoded_bytes().to_vec();
        noisy.extend_from_slice(b"/./");
        assert_eq!(realpath_token(&noisy), Some(expected));
    }
}

#[cfg(test)]
mod subst_and_strip_tests {
    use super::{func_strip, subst_expand};
    use crate::expand::{
        initialize_variable_output, variable_buffer, variable_buffer_output,
        VARIABLE_BUFFER_TEST_LOCK,
    };
    use crate::make_main::initialize_stopchar_map;
    use std::ffi::{c_char, CStr, CString};

    /// Run `body` with a freshly initialized variable-output buffer, returning
    /// the bytes it wrote (`[buffer, end_cursor)`), where `body` returns the
    /// end cursor produced by the function under test.
    unsafe fn with_output<F: FnOnce(*mut c_char) -> *mut c_char>(body: F) -> Vec<u8> {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        initialize_stopchar_map();
        let start = initialize_variable_output();
        let end = body(start);
        let len = end.offset_from(start);
        assert!(len >= 0, "output cursor moved before the buffer start");
        std::slice::from_raw_parts(start as *const u8, len as usize).to_vec()
    }

    #[test]
    fn subst_expand_replaces_each_occurrence() {
        unsafe {
            let text = CString::new("a.b.c").unwrap();
            let subst = CString::new(".").unwrap();
            let replace = CString::new("-").unwrap();
            let out = with_output(|o| {
                subst_expand(o, text.as_ptr(), subst.as_ptr(), replace.as_ptr(), 1, 1, 0)
            });
            assert_eq!(out, b"a-b-c");
        }
    }

    #[test]
    fn subst_expand_empty_subst_appends_replacement() {
        // slen == 0 && by_word == 0: copy text verbatim then append replace.
        unsafe {
            let text = CString::new("xy").unwrap();
            let subst = CString::new("").unwrap();
            let replace = CString::new("Z").unwrap();
            let out = with_output(|o| {
                subst_expand(o, text.as_ptr(), subst.as_ptr(), replace.as_ptr(), 0, 1, 0)
            });
            assert_eq!(out, b"xyZ");
        }
    }

    #[test]
    fn subst_expand_by_word_only_replaces_whole_words() {
        // by_word != 0: "foo" is replaced as a standalone word, but the "foo"
        // inside "foobar" is preserved.
        unsafe {
            let text = CString::new("foo foobar foo").unwrap();
            let subst = CString::new("foo").unwrap();
            let replace = CString::new("Q").unwrap();
            let out = with_output(|o| {
                subst_expand(o, text.as_ptr(), subst.as_ptr(), replace.as_ptr(), 3, 1, 1)
            });
            assert_eq!(out, b"Q foobar Q");
        }
    }

    #[test]
    fn func_strip_collapses_internal_and_edge_whitespace() {
        unsafe {
            let arg = CString::new("  a\t b   c  ").unwrap();
            // argv is a NULL-terminated vector of arg pointers.
            let mut argv: [*mut c_char; 2] = [arg.as_ptr() as *mut c_char, std::ptr::null_mut()];
            let name = CString::new("strip").unwrap();
            let out = with_output(|o| {
                func_strip(
                    &crate::execctx::ExecContext::default(),
                    o,
                    argv.as_mut_ptr(),
                    name.as_ptr(),
                )
            });
            // Words separated by single spaces, no leading/trailing space.
            assert_eq!(out, b"a b c");
            // Keep `arg` alive until after the call.
            let _ = CStr::from_ptr(argv[0]);
        }
    }

    #[test]
    fn func_strip_all_whitespace_yields_empty() {
        unsafe {
            let arg = CString::new("   \t  ").unwrap();
            let mut argv: [*mut c_char; 2] = [arg.as_ptr() as *mut c_char, std::ptr::null_mut()];
            let name = CString::new("strip").unwrap();
            let out = with_output(|o| {
                func_strip(
                    &crate::execctx::ExecContext::default(),
                    o,
                    argv.as_mut_ptr(),
                    name.as_ptr(),
                )
            });
            assert_eq!(out, b"");
        }
    }

    // Touch the buffer-output helper directly so the import is always used even
    // if the functions above short-circuit.
    #[test]
    fn variable_buffer_output_appends_and_nul_terminates() {
        unsafe {
            let _g = VARIABLE_BUFFER_TEST_LOCK
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            let start = initialize_variable_output();
            let s = CString::new("hi").unwrap();
            let end = variable_buffer_output(start, s.as_ptr(), 2);
            assert_eq!(end.offset_from(start), 2);
            assert_eq!(*end, 0, "buffer is NUL-terminated at the cursor");
            assert!(!variable_buffer.is_null());
        }
    }
}

#[cfg(test)]
mod handle_function_abeg_unsafe_oracle {
    //! Pure-Rust expected-value test for the RAII conversion of
    //! `handle_function`'s `abeg` argument-copy buffer.
    //!
    //! The original c2rust code allocated the buffer with `xmalloc(len + 1)`,
    //! filled it with `mempcpy(abeg, beg, len)` followed by a NUL terminator,
    //! and released it with `free(abeg)`. The conversion replaces this with an
    //! owned `Vec<u8>` produced by `copy_args_buffer`. Rather than replaying the
    //! removed C functions, we compute the expected NUL-terminated bytes in safe
    //! Rust and assert `copy_args_buffer(src)` matches, across edge cases: empty
    //! input, single byte, embedded NUL, high bytes (0x80 / 0xff), a full
    //! `0..=255` sweep, and `%`/paren-laden function-call payloads.

    fn assert_copy(src: &[u8]) {
        // Expected: a verbatim copy of `src` followed by a single NUL byte.
        let mut want = src.to_vec();
        want.push(0);
        assert_eq!(
            super::copy_args_buffer(src),
            want,
            "copy_args_buffer differs from expected NUL-terminated copy for input {src:?}"
        );
    }

    #[test]
    fn byte_identical_across_edge_cases() {
        assert_copy(b"");
        assert_copy(b"x");
        assert_copy(b"foo,bar");
        // Embedded NUL: the copy is length-based, so the NUL is preserved.
        assert_copy(b"a\0b");
        // High bytes must round-trip unchanged.
        assert_copy(b"\x80\xff\x00\x7f");
        assert_copy(&[0x80u8; 16]);
        assert_copy(&[0xffu8; 16]);
        // `%` patterns and nested parens, the kind of payload the splitter sees.
        assert_copy(b"patsubst %.c,%.o,$(SRCS)");
        assert_copy(b"$(foo $(bar),baz),%qux%");
        // A full sweep spanning every byte value.
        let big: Vec<u8> = (0u16..=255).map(|b| b as u8).collect();
        assert_copy(&big);
    }
}

#[cfg(test)]
mod find_next_argument_tests {
    use super::{find_next_argument, stop_set, MAP_COMMA, MAP_VARSEP};
    use ::core::ffi::c_char;

    /// Original c2rust raw-pointer implementation, preserved verbatim as a
    /// differential oracle (returns a pointer into `[ptr, end)` or null).
    unsafe fn find_next_argument_unsafe_oracle(
        startparen: c_char,
        endparen: c_char,
        ptr: *const c_char,
        end: *const c_char,
    ) -> *mut c_char {
        let mut count: i32 = 0;
        let span = end as usize - ptr as usize;
        let bytes = ::core::slice::from_raw_parts(ptr as *const u8, span);
        for (i, &c) in bytes.iter().enumerate() {
            if stop_set(c, MAP_VARSEP | MAP_COMMA) {
                if c as i32 == startparen as i32 {
                    count += 1;
                } else if c as i32 == endparen as i32 {
                    count -= 1;
                    if count < 0 {
                        return ::core::ptr::null_mut::<c_char>();
                    }
                } else if c as i32 == ',' as i32 && count == 0 {
                    return bytes[i..].as_ptr() as *mut c_char;
                }
            }
        }
        ::core::ptr::null_mut::<c_char>()
    }

    /// The safe offset-returning form yields exactly the same split point as the
    /// original raw-pointer oracle across nesting, unbalanced parens and the
    /// no-comma case.
    #[test]
    fn matches_unsafe_oracle() {
        crate::make_main::initialize_stopchar_map();
        let cases: &[&[u8]] = &[
            b"a,b",
            b"abc",
            b"(a,b),c",
            b"a(,)b,c",
            b"))",
            b"(((",
            b",",
            b"",
            b"{a,b},c",
            b"a,b,c,d",
        ];
        for &(sp, ep) in &[(b'(', b')'), (b'{', b'}')] {
            for &bytes in cases {
                let safe = find_next_argument(sp, ep, bytes);
                let ptr = bytes.as_ptr() as *const c_char;
                let end = unsafe { ptr.add(bytes.len()) };
                let oracle =
                    unsafe { find_next_argument_unsafe_oracle(sp as c_char, ep as c_char, ptr, end) };
                let oracle_off = if oracle.is_null() {
                    None
                } else {
                    Some(oracle as usize - ptr as usize)
                };
                assert_eq!(safe, oracle_off, "input {:?}", String::from_utf8_lossy(bytes));
            }
        }
    }

    /// Spot-check the absolute offsets and nesting behaviour.
    #[test]
    fn splits_at_top_level_comma() {
        crate::make_main::initialize_stopchar_map();
        assert_eq!(find_next_argument(b'(', b')', b"a,b"), Some(1));
        // comma nested inside parens is skipped; the top-level one is at 5.
        assert_eq!(find_next_argument(b'(', b')', b"(a,b),c"), Some(5));
        // no top-level comma.
        assert_eq!(find_next_argument(b'(', b')', b"(a,b)"), None);
        // an unbalanced close paren ends the scan with no split.
        assert_eq!(find_next_argument(b'(', b')', b")a,b"), None);
        // closing an inner paren leaves nesting depth > 0; the top-level comma
        // after the outer close is still found (depth must reach 0, not just
        // decrease, before a comma counts).
        assert_eq!(find_next_argument(b'(', b')', b"((a)),b"), Some(5));
    }
}
