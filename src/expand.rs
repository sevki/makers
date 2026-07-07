//! Variable and string expansion: the `$`-reference scanner, recursive
//! variable expansion, and the shared grow-on-demand output buffer that
//! every expansion appends into.
//!
//! Port of `expand.c`.

pub use crate::ffi_types::size_t;
use crate::file::{File, VariableSet, VariableSetList};
use crate::misc::{lindex, xstrdup, xstrndup};
use crate::output::FmtArg;
use libc::{free, memcpy, strchr, strlen, strncmp};
extern "C" {
    static mut environ: *mut *mut ::core::ffi::c_char;
}

/// Owns a `malloc`ed C string and frees it on drop, replacing the manual
/// `xstrdup`/`expand_argument` + `free` ownership pairs in this module.
struct OwnedCStr(*mut ::core::ffi::c_char);

impl OwnedCStr {
    /// Borrow the underlying NUL-terminated buffer.
    fn as_ptr(&self) -> *mut ::core::ffi::c_char {
        self.0
    }
}

impl Drop for OwnedCStr {
    fn drop(&mut self) {
        unsafe { free(self.0 as *mut ::core::ffi::c_void) }
    }
}

/// Build a NUL-terminated `%`-prefixed copy of `s`: `b'%'`, then the bytes of
/// `s`, then a trailing NUL. Used to rewrite a `$(name:a=b)` substitution
/// reference as a `%a` -> `%b` patsubst, where the leading `%` ensures an
/// explicit percent in the makefile takes precedence over the implicit one.
fn percent_prefixed(s: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(s.len() + 2);
    buf.push(b'%');
    buf.extend_from_slice(s);
    buf.push(0);
    buf
}

pub type file = File;
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
use crate::floc::Floc;
use crate::function::{handle_function, patsubst_expand_pat};
use crate::make_main::{db_level, stopchar_map};
use crate::output::fatal;
use crate::read::find_percent;
pub use crate::variable::variable;
use crate::variable::{
    env_recursion, install_file_context, lookup_variable, lookup_variable_in_set, o_command,
    o_env_override, o_override, restore_file_context, warn_undefined,
};

/// "Whole string" length sentinel accepted by [`expand_string_buf`].
pub const SIZE_MAX: size_t = size_t::MAX;

/// `DB_VERBOSE`: `-d`-style debug output enabled in `db_level`.
const DB_VERBOSE: i32 = 0x2;

/// Character-class bits in `stopchar_map` (see `makeint.h`).
const MAP_BLANK: i32 = 0x0002;
const MAP_NEWLINE: i32 = 0x0004;

/// `STOP_SET (c, mask)` from `makeint.h`: is `c` in any of the character
/// classes selected by `mask`?
fn stop_set(c: ::core::ffi::c_char, mask: i32) -> bool {
    stopchar_map()[c as u8 as usize] as i32 & mask != 0
}
// The former `static mut expanding_var`/`reading_file` pair now lives on
// `ExecContext` as `ctx.expanding_var`/`ctx.reading_file` (see
// `execctx::ExecContext::expanding_var_floc`); every use below reads/writes
// through those owned fields instead of process-wide statics.
pub const VARIABLE_BUFFER_ZONE: i32 = 5;
/// Process-wide lock serializing tests that drive the `variable_buffer`
/// global (the output buffer for `$(...)` expansion). Tests in different
/// modules share this single lock so they never race on the buffer.
#[cfg(test)]
pub static VARIABLE_BUFFER_TEST_LOCK: ::std::sync::Mutex<()> = ::std::sync::Mutex::new(());
/// # Safety
///
/// `ptr` must be a cursor previously returned by this function (or
/// [`initialize_variable_output`]) into `ctx.variable_buffer`'s current
/// allocation.
pub unsafe fn variable_buffer_output(
    ctx: &crate::execctx::ExecContext,
    mut ptr: *mut ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let base = ctx.variable_buffer.ptr();
    let cur_len = ctx.variable_buffer.length();
    assert!(ptr >= base, "output cursor before the buffer");
    assert!(
        ptr < unsafe { base.add(cur_len) },
        "output cursor past the buffer"
    );
    let offset = unsafe { ptr.offset_from(base) as size_t };
    let newlen = length + offset;

    if newlen + VARIABLE_BUFFER_ZONE as size_t + 1 > cur_len {
        ctx.variable_buffer.ensure_len(newlen + 100);
        ptr = unsafe { ctx.variable_buffer.ptr().add(offset) };
    }
    unsafe {
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            string as *const ::core::ffi::c_void,
            length,
        );
        ptr = ptr.add(length);
        *ptr = 0;
    }
    ptr
}
/// Ensure the buffer is allocated (200 bytes, the former initial `xmalloc`
/// size) and NUL-terminate it at the start, returning the base pointer.
pub fn initialize_variable_output(ctx: &crate::execctx::ExecContext) -> *mut ::core::ffi::c_char {
    if ctx.variable_buffer.length() == 0 {
        ctx.variable_buffer.ensure_len(200);
    }
    ctx.variable_buffer.set_byte_at(0, 0);
    ctx.variable_buffer.ptr()
}
/// # Safety
///
/// `bufp`/`lenp` must be valid for writes.
pub unsafe fn install_variable_buffer(
    ctx: &crate::execctx::ExecContext,
    bufp: *mut *mut ::core::ffi::c_char,
    lenp: *mut size_t,
) {
    let (old_ptr, old_len) = ctx.variable_buffer.take_raw();
    unsafe {
        *bufp = old_ptr.map_or(::core::ptr::null_mut(), |p| p.as_ptr());
        *lenp = old_len;
    }
    initialize_variable_output(ctx);
}
/// # Safety
///
/// `buf`/`len` must be exactly a pair previously produced by
/// [`crate::execctx::VariableBuffer::take_raw`] (e.g. via
/// [`install_variable_buffer`]'s out-params).
pub unsafe fn restore_variable_buffer(
    ctx: &crate::execctx::ExecContext,
    buf: *mut ::core::ffi::c_char,
    len: size_t,
) {
    unsafe { ctx.variable_buffer.set_raw(buf, len) };
}
/// # Safety
///
/// `buf`/`len` must be exactly a pair previously produced by
/// [`crate::execctx::VariableBuffer::take_raw`] (e.g. via
/// [`install_variable_buffer`]'s out-params).
pub unsafe fn swap_variable_buffer(
    ctx: &crate::execctx::ExecContext,
    buf: *mut ::core::ffi::c_char,
    len: size_t,
) -> *mut ::core::ffi::c_char {
    // Every real caller reaches this through install_variable_buffer (which
    // runs initialize_variable_output first), so the buffer being handed out
    // here is always allocated; callers (e.g. allocated_expand_variable's
    // many call sites) dereference the result directly without a null check.
    // `take_raw_nonnull` panics rather than silently handing out a would-be
    // pointer this codebase later `free()`s that isn't a real allocation.
    let (old_ptr, _old_len) = ctx.variable_buffer.take_raw_nonnull();
    unsafe { ctx.variable_buffer.set_raw(buf, len) };
    old_ptr.as_ptr()
}
/// Read one byte from the variable expansion buffer at `off`.
///
/// Bounds-checked access into `ctx.variable_buffer` via `Vec` indexing, used
/// in place of raw-pointer dereferences of cursors derived from it (e.g.
/// pointers returned by `find_char_unquote`) so the access cannot touch a
/// stale pointer.
pub fn variable_buffer_byte(ctx: &crate::execctx::ExecContext, off: size_t) -> ::core::ffi::c_char {
    ctx.variable_buffer.byte_at(off)
}
/// Write one byte into the variable expansion buffer at `off`.
///
/// Bounds-checked counterpart to [`variable_buffer_byte`].
pub fn set_variable_buffer_byte(
    ctx: &crate::execctx::ExecContext,
    off: size_t,
    b: ::core::ffi::c_char,
) {
    ctx.variable_buffer.set_byte_at(off, b);
}
/// The `-d` verbose line for a self-referencing variable being exported to
/// a `$(shell …)` function. A missing file name renders as `(null)`, exactly
/// as glibc printf did for built-in and environment variables.
fn no_recursive_expand_msg(
    filenm: Option<&[u8]>,
    lineno: ::core::ffi::c_ulong,
    name: &[u8],
) -> Vec<u8> {
    let mut msg = Vec::with_capacity(64);
    msg.extend_from_slice(filenm.unwrap_or(b"(null)"));
    msg.extend_from_slice(b":");
    msg.extend_from_slice(lineno.to_string().as_bytes());
    msg.extend_from_slice(b": not recursively expanding ");
    msg.extend_from_slice(name);
    msg.extend_from_slice(b" to export to shell function\n");
    msg
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn recursively_expand_for_file(
    ctx: &crate::execctx::ExecContext,
    v: *mut variable,
    file: *mut File,
) -> *mut ::core::ffi::c_char {
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut set_reading: i32 = 0;
    let nl: size_t = strlen((*v).name) as size_t;
    let mut parent: *mut variable = ::core::ptr::null_mut::<variable>();
    if (*v).expanding() != 0 && env_recursion(ctx) != 0 {
        // A self-referencing variable being exported to a $(shell ...)
        // function: hand back the unexpanded environment value instead.
        if DB_VERBOSE & db_level(ctx) != 0 {
            let fnm = (*v).fileinfo.filenm;
            crate::output::trace_out(&no_recursive_expand_msg(
                if fnm.is_null() {
                    None
                } else {
                    Some(::core::ffi::CStr::from_ptr(fnm).to_bytes())
                },
                (*v).fileinfo.lineno,
                ::core::ffi::CStr::from_ptr((*v).name).to_bytes(),
            ));
        }
        let mut ep = environ;
        while !(*ep).is_null() {
            if strncmp(*ep, (*v).name, nl) == 0
                && *(*ep).add(nl as usize) == b'=' as ::core::ffi::c_char
            {
                return xstrdup((*ep).add(nl as usize + 1));
            }
            ep = ep.add(1);
        }
        return xstrdup(c"".as_ptr());
    }
    let saved_varp = ctx.expanding_var.get();
    if !(*v).fileinfo.filenm.is_null() {
        ctx.expanding_var.set(Some(&raw mut (*v).fileinfo as *const Floc));
    }
    if ctx.reading_file.0.get().is_null() {
        set_reading = 1;
        ctx.reading_file.0.set(&raw mut (*v).fileinfo);
    }
    if (*v).expanding() != 0 {
        if (*v).exp_count() == 0 {
            fatal(
        ctx,
        ctx.expanding_var_floc(),
        strlen((*v).name),
        c"recursive variable '%s' references itself (eventually)".as_ptr(),
        &[FmtArg::Str(((*v).name) as *const ::core::ffi::c_char)],
    );
        }
        (*v).set_exp_count((*v).exp_count() - 1);
    }
    if !file.is_null() {
        install_file_context(ctx, file, &raw mut savev, ::core::ptr::null_mut::<*const Floc>());
    }
    (*v).set_expanding(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*v).append() != 0 {
        let mut sl: *mut variable_set_list;
        sl = ctx.variable_globals.current_variable_set_list.get();
        while !sl.is_null() && parent.is_null() {
            let vp: *mut variable = lookup_variable_in_set(ctx, (*v).name, nl, (*sl).set);
            if !vp.is_null() && vp != v && (*vp).origin() as i32 == o_override as i32 {
                parent = vp;
            }
            sl = (*sl).next;
        }
    }
    let value: *mut ::core::ffi::c_char = if let Some(pref) = parent.as_ref() {
        if (*v).origin() == o_override {
            allocated_variable_append(ctx, v)
        } else {
            xstrdup(pref.value)
        }
    } else if (*v).origin() == o_command || (*v).origin() == o_env_override {
        allocated_expand_string_for_file(ctx, (*v).value, ::core::ptr::null_mut::<file>())
    } else if (*v).append() != 0 {
        allocated_variable_append(ctx, v)
    } else {
        allocated_expand_string_for_file(ctx, (*v).value, ::core::ptr::null_mut::<file>())
    };
    (*v).set_expanding(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if set_reading != 0 {
        ctx.reading_file.0.set(::core::ptr::null::<Floc>());
    }
    if !file.is_null() {
        restore_file_context(ctx, savev, ::core::ptr::null::<Floc>());
    }
    ctx.expanding_var.set(saved_varp);
    value
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_variable_output(
    ctx: &crate::execctx::ExecContext,
    mut ptr: *mut ::core::ffi::c_char,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let v = lookup_variable(ctx, name, length);
    if v.is_null() {
        // SAFETY: `name` points to `length` valid bytes (caller contract);
        // read-only bridge to the safe `warn_undefined`.
        warn_undefined(
            ctx,
            ::core::slice::from_raw_parts(name as *const u8, length),
        );
    }
    if v.is_null() || *(*v).value.offset(0_i32 as isize) as i32 == 0 && (*v).append() == 0 {
        return ptr;
    }
    // A recursive variable's value is freshly expanded and owned here; an
    // `OwnedCStr` reclaims it on drop instead of the manual `free` the C code
    // did. A non-recursive variable's value is borrowed from the variable.
    let owned = ((*v).recursive() != 0).then(|| {
        OwnedCStr(recursively_expand_for_file(
            ctx,
            v,
            ::core::ptr::null_mut::<file>(),
        ))
    });
    let value = owned.as_ref().map_or((*v).value, OwnedCStr::as_ptr);
    ptr = variable_buffer_output(ctx, ptr, value, strlen(value) as size_t);
    ptr
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_variable_buf(
    ctx: &crate::execctx::ExecContext,
    mut buf: *mut ::core::ffi::c_char,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    if buf.is_null() {
        buf = initialize_variable_output(ctx);
    }
    let variable_buffer = ctx.variable_buffer.ptr();
    let variable_buffer_length = ctx.variable_buffer.length();
    assert!(buf >= variable_buffer, "output cursor before the buffer");
    assert!(
        buf < variable_buffer.add(variable_buffer_length),
        "output cursor past the buffer"
    );
    let offs = buf.offset_from(variable_buffer) as size_t;
    expand_variable_output(ctx, buf, name, length);
    ctx.variable_buffer.ptr().add(offs as usize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_expand_variable(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
    expand_variable_output(ctx, ctx.variable_buffer.ptr(), name, length);
    swap_variable_buffer(ctx, obuf, olen)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_string_buf(
    ctx: &crate::execctx::ExecContext,
    mut buf: *mut ::core::ffi::c_char,
    string: *const ::core::ffi::c_char,
    length: size_t,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char;
    let mut p1: *const ::core::ffi::c_char;
    let mut o: *mut ::core::ffi::c_char;
    if buf.is_null() {
        buf = initialize_variable_output(ctx);
    }
    o = buf;
    let line_offset = buf.offset_from(ctx.variable_buffer.ptr()) as usize;
    if length == 0 {
        return ctx.variable_buffer.ptr();
    }
    // Work on a stable copy: expansion may reuse the variable buffer the
    // input could be pointing into.
    let save = OwnedCStr(if length == SIZE_MAX {
        xstrdup(string)
    } else {
        xstrndup(string, length)
    });
    p = save.as_ptr();
    loop {
        // Copy everything up to the next `$` (or the rest of the string,
        // including its NUL) verbatim.
        p1 = strchr(p, '$' as i32);
        o = variable_buffer_output(
            ctx,
            o,
            p,
            if !p1.is_null() {
                p1.offset_from(p) as size_t
            } else {
                strlen(p) + 1
            },
        );
        if p1.is_null() {
            break;
        }
        p = p1.add(1);
        match *p as u8 {
            b'$' | 0 => {
                // `$$` (or a trailing lone `$`) expands to a literal `$`.
                o = variable_buffer_output(ctx, o, p1, 1);
            }
            b'(' | b'{' => {
                let openparen = *p as u8;
                let closeparen = if openparen == b'(' { b')' } else { b'}' };
                let mut beg: *const ::core::ffi::c_char = p.add(1);
                let mut abeg: Option<OwnedCStr> = None;
                let mut end: *const ::core::ffi::c_char;
                let mut colon: *const ::core::ffi::c_char;
                if handle_function(ctx, &raw mut o, &raw mut p) == 0 {
                    end = strchr(beg, closeparen as i32);
                    if end.is_null() {
                        fatal(
                            ctx,
                            ctx.expanding_var_floc(),
                            0,
                            c"unterminated variable reference".as_ptr(),
                            &[],
                        );
                    }
                    // Bridge the safe `lindex(&[u8], u8) -> Option<usize>` to
                    // the pointer-walking code below: view `[b, e)` as a byte
                    // slice, search it, and map the index back to a pointer
                    // (or null when not found, exactly as the old `lindex`).
                    // The `e <= b` guard ensures we never form a slice from an
                    // empty/invalid range.
                    let lindex_ptr = |b: *const ::core::ffi::c_char,
                                      e: *const ::core::ffi::c_char,
                                      c: u8|
                     -> *const ::core::ffi::c_char {
                        // SAFETY: when `b < e`, `[b, e)` is a single valid
                        // readable range (both point into the same buffer the
                        // surrounding code already walks); the cast to `*const
                        // u8` is a reinterpret of the same bytes.
                        let hay = unsafe {
                            if e <= b {
                                &[][..]
                            } else {
                                ::core::slice::from_raw_parts(
                                    b as *const u8,
                                    e.offset_from(b) as usize,
                                )
                            }
                        };
                        match lindex(hay, c) {
                            // SAFETY: `i` is within `[b, e)`.
                            Some(i) => unsafe { b.add(i) },
                            None => ::core::ptr::null(),
                        }
                    };
                    // A nested `$` means the variable name itself needs
                    // expanding first; find the matching close paren.
                    p1 = lindex_ptr(beg, end, b'$');
                    if !p1.is_null() {
                        let mut count = 1;
                        p = beg;
                        while *p != 0 {
                            if *p as u8 == openparen {
                                count += 1;
                            } else if *p as u8 == closeparen && {
                                count -= 1;
                                count == 0
                            } {
                                break;
                            }
                            p = p.add(1);
                        }
                        if count == 0 {
                            let owned = OwnedCStr(expand_argument(ctx, beg, p));
                            beg = owned.as_ptr();
                            abeg = Some(owned);
                            end = strchr(beg, 0);
                        }
                    } else {
                        p = end;
                    }
                    // `$(name:a=b)` substitution reference: rewrite it as
                    // a `%a` -> `%b` patsubst over the variable's value.
                    colon = lindex_ptr(beg, end, b':');
                    if !colon.is_null() {
                        let subst_beg: *const ::core::ffi::c_char = colon.add(1);
                        let subst_end: *const ::core::ffi::c_char =
                            lindex_ptr(subst_beg, end, b'=');
                        if subst_end.is_null() {
                            // A colon without `=` is just part of the name.
                            colon = ::core::ptr::null::<::core::ffi::c_char>();
                        } else {
                            let replace_beg: *const ::core::ffi::c_char = subst_end.add(1);
                            let replace_end: *const ::core::ffi::c_char = end;
                            let name_len = colon.offset_from(beg) as size_t;
                            let v = lookup_variable(ctx, beg, name_len).as_mut();
                            if v.is_none() {
                                // SAFETY: `beg` points to `name_len` valid
                                // bytes (`name_len = colon - beg`, both within
                                // the same buffer); read-only bridge to the
                                // safe `warn_undefined`.
                                warn_undefined(
                                    ctx,
                                    ::core::slice::from_raw_parts(beg as *const u8, name_len),
                                );
                            }
                            if let Some(v) = v.filter(|v| *v.value != 0) {
                                // Recursive values are freshly expanded and
                                // owned; `OwnedCStr` frees on drop in place of
                                // the manual `free` below.
                                let owned = (v.recursive() != 0).then(|| {
                                    OwnedCStr(recursively_expand_for_file(
                                        ctx,
                                        &raw mut *v,
                                        ::core::ptr::null_mut::<file>(),
                                    ))
                                });
                                let value: *mut ::core::ffi::c_char =
                                    owned.as_ref().map_or(v.value, OwnedCStr::as_ptr);

                                // Prefix both sides with `%` so an explicit
                                // percent in the makefile takes precedence.
                                // SAFETY: `subst_beg..subst_end` and
                                // `replace_beg..replace_end` are spans within the
                                // reference being parsed; each length is computed
                                // once via `offset_from` and the bytes are read as
                                // a single slice (no per-element strlen).
                                let mut pattern_buf = percent_prefixed(unsafe {
                                    ::core::slice::from_raw_parts(
                                        subst_beg as *const u8,
                                        subst_end.offset_from(subst_beg) as usize,
                                    )
                                });
                                let mut replace_buf = percent_prefixed(unsafe {
                                    ::core::slice::from_raw_parts(
                                        replace_beg as *const u8,
                                        replace_end.offset_from(replace_beg) as usize,
                                    )
                                });
                                let mut pattern =
                                    pattern_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
                                let mut replace =
                                    replace_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
                                let mut ppercent = find_percent(pattern.add(1));
                                let rpercent;
                                if !ppercent.is_null() {
                                    pattern = pattern.add(1);
                                    ppercent = ppercent.add(1);
                                    let r = find_percent(replace.add(1));
                                    replace = replace.add(1);
                                    rpercent = if r.is_null() { r } else { r.add(1) };
                                } else {
                                    // No explicit `%`: use the implicit one
                                    // we prefixed.
                                    ppercent = pattern.add(1);
                                    rpercent = replace.add(1);
                                }
                                o = patsubst_expand_pat(
                                    ctx, o, value, pattern, replace, ppercent, rpercent,
                                );
                            }
                        }
                    }
                    if colon.is_null() {
                        o = expand_variable_output(ctx, o, beg, end.offset_from(beg) as size_t);
                    }
                    // Free the expanded reference here, as the C code did.
                    drop(abeg);
                }
            }
            _ => {
                // `$X`: a single-character variable name. The guard mirrors
                // the C original, which tests `p[-1]` (the `$` itself).
                if !stop_set(*p1, MAP_BLANK | MAP_NEWLINE) {
                    o = expand_variable_output(ctx, o, p, 1);
                }
            }
        }
        if *p == 0 {
            break;
        }
        p = p.add(1);
    }
    ctx.variable_buffer.ptr().add(line_offset)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_argument(
    ctx: &crate::execctx::ExecContext,
    str: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if str == end {
        return xstrdup(c"".as_ptr());
    }
    if end.is_null() || *end.as_ref().unwrap() == 0 {
        return allocated_expand_string_for_file(ctx, str, ::core::ptr::null_mut::<file>());
    }
    // Copy the [str, end) slice into an owned, NUL-terminated buffer (the C
    // code chose alloca vs xmalloc by length; an owned Vec covers both).
    let len = end.offset_from(str) as usize;
    let mut tmp_buf = ::core::slice::from_raw_parts(str as *const u8, len).to_vec();
    tmp_buf.push(0);
    allocated_expand_string_for_file(
        ctx,
        tmp_buf.as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<file>(),
    )
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_string_for_file_c(
    ctx: &crate::execctx::ExecContext,
    string: *const ::core::ffi::c_char,
    file: *mut File,
) -> *mut ::core::ffi::c_char {
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut savef: *const Floc = ::core::ptr::null::<Floc>();
    if file.is_null() {
        return expand_string_buf(
            ctx,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            string,
            SIZE_MAX,
        );
    }
    install_file_context(ctx, file, &raw mut savev, &raw mut savef);
    let result = expand_string_buf(
        ctx,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        string,
        SIZE_MAX,
    );
    restore_file_context(ctx, savev, savef);
    result
}

/// FileId-based string expansion in a target's variable context.
///
/// `string` is the NUL-terminated source bytes to expand; the trailing NUL is
/// required (callers push one). Returns the expanded bytes, NUL-terminated.
/// Installs the target's transient variable-set chain (per-target/pattern
/// variables plus the parent/global scopes) for the duration of the expansion,
/// the idiomatic replacement for the former `*mut File` install/restore dance.
pub fn expand_string_for_file(
    ctx: &crate::execctx::ExecContext,
    string: &[u8],
    file: crate::file::FileId,
) -> Vec<u8> {
    // SAFETY: the inner expander remains the c2rust pointer machinery; we feed
    // it a NUL-terminated buffer and a freshly-built per-file scope, then read
    // the NUL-terminated result back into an owned Vec.
    unsafe {
        let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
        let mut savef: *const Floc = ::core::ptr::null::<Floc>();
        crate::variable::install_file_context_id(ctx, file, &raw mut savev, &raw mut savef);
        let cur = ctx.variable_globals.current_variable_set_list.get();
        let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut olen: size_t = 0;
        install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
        expand_string_buf(
            ctx,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            string.as_ptr() as *const ::core::ffi::c_char,
            SIZE_MAX,
        );
        let result = swap_variable_buffer(ctx, obuf, olen);
        crate::variable::restore_file_context_id(ctx, cur, savev, savef);
        if result.is_null() {
            vec![0]
        } else {
            let len = strlen(result) as usize;
            let mut v = ::core::slice::from_raw_parts(result as *const u8, len).to_vec();
            v.push(0);
            free(result as *mut ::core::ffi::c_void);
            v
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_expand_string_for_file(
    ctx: &crate::execctx::ExecContext,
    string: *const ::core::ffi::c_char,
    file: *mut File,
) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
    expand_string_for_file_c(ctx, string, file);
    swap_variable_buffer(ctx, obuf, olen)
}
/// Walk the variable-set chain outward, concatenating every `+=`-style
/// definition of `name` (oldest first) into the variable buffer.
unsafe fn variable_append(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
    set: *const variable_set_list,
    local: i32,
) -> *mut ::core::ffi::c_char {
    if set.is_null() {
        return initialize_variable_output(ctx);
    }
    let nextlocal = (local != 0 && (*set).next_is_parent == 0) as i32;
    let v: *const variable = lookup_variable_in_set(ctx, name, length, (*set).set);
    if v.is_null() || (local == 0 && (*v).private_var() != 0) {
        return variable_append(ctx, name, length, (*set).next, nextlocal);
    }

    // An appending definition stacks on whatever the outer sets produce.
    let mut buf = if (*v).append() != 0 {
        variable_append(ctx, name, length, (*set).next, nextlocal)
    } else {
        initialize_variable_output(ctx)
    };
    if buf > ctx.variable_buffer.ptr() {
        buf = variable_buffer_output(ctx, buf, c" ".as_ptr(), 1);
    }
    if (*v).recursive() == 0 {
        return variable_buffer_output(ctx, buf, (*v).value, strlen((*v).value));
    }
    buf = expand_string_buf(ctx, buf, (*v).value, strlen((*v).value));
    buf.add(strlen(buf) as usize)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_variable_append(
    ctx: &crate::execctx::ExecContext,
    v: *const variable,
) -> *mut ::core::ffi::c_char {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
    variable_append(
        ctx,
        (*v).name,
        strlen((*v).name) as size_t,
        ctx.variable_globals.current_variable_set_list.get(),
        1,
    );
    swap_variable_buffer(ctx, obuf, olen)
}

#[cfg(test)]
mod percent_prefixed_unsafe_oracle {
    /// Original c2rust pointer-based implementation, preserved verbatim as a
    /// differential-test oracle for the safe [`super::percent_prefixed`].
    unsafe fn percent_prefixed(
        beg: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> Vec<u8> {
        let len = end.offset_from(beg) as usize;
        let mut buf = Vec::with_capacity(len + 2);
        buf.push(b'%');
        buf.extend_from_slice(::core::slice::from_raw_parts(beg as *const u8, len));
        buf.push(0);
        buf
    }

    #[test]
    fn matches_oracle() {
        let cases: &[&[u8]] = &[
            b"",
            b"a",
            b"foo",
            b"foo.o",
            b"a=b",
            b"with space",
            b"%already",
            b"\x00trailing-nul-mid", // arbitrary embedded byte
            b"\xff\x80\x01",
        ];
        for &s in cases {
            let safe = super::percent_prefixed(s);
            // Build the same [beg, end) span the C bridge sees.
            let beg = s.as_ptr() as *const ::core::ffi::c_char;
            let end = unsafe { beg.add(s.len()) };
            let oracle = unsafe { percent_prefixed(beg, end) };
            assert_eq!(safe, oracle, "mismatch for input {s:?}");
        }
    }
}

#[cfg(test)]
mod no_recursive_expand_msg_tests {
    use super::no_recursive_expand_msg;

    /// Named and unnamed (built-in/env) variables format like the C
    /// printf did, including the glibc "(null)" for a missing file name.
    #[test]
    fn formats_with_and_without_file_name() {
        assert_eq!(
            no_recursive_expand_msg(Some(b"Makefile"), 12, b"FOO"),
            b"Makefile:12: not recursively expanding FOO to export to shell function\n".to_vec()
        );
        assert_eq!(
            no_recursive_expand_msg(None, 0, b"PATH"),
            b"(null):0: not recursively expanding PATH to export to shell function\n".to_vec()
        );
    }
}
