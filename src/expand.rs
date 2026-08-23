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
use crate::output::fatal_err;
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
/// a `$(shell …)` function. A NULL file name renders as `(null)`, exactly
/// as glibc printf did for built-in and environment variables.
/// # Safety
/// `filenm` must be NULL or a valid NUL-terminated string; `name` likewise
/// valid for the call.
unsafe fn no_recursive_expand_msg(
    filenm: *const ::core::ffi::c_char,
    lineno: u64,
    name: &[u8],
) -> Vec<u8> {
    let mut msg = Vec::with_capacity(64);
    msg.extend_from_slice(if filenm.is_null() {
        b"(null)"
    } else {
        ::core::ffi::CStr::from_ptr(filenm).to_bytes()
    });
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
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
    let mut set_reading: i32 = 0;
    let nl: size_t = strlen((*v).name) as size_t;
    if (*v).expanding() != 0 && env_recursion(ctx) != 0 {
        // A self-referencing variable being exported to a $(shell ...)
        // function: hand back the unexpanded environment value instead.
        if DB_VERBOSE & db_level(ctx) != 0 {
            crate::output::trace_out(&no_recursive_expand_msg(
                (*v).fileinfo.filenm,
                (*v).fileinfo.lineno,
                ::core::ffi::CStr::from_ptr((*v).name).to_bytes(),
            ));
        }
        let mut ep = environ;
        while !(*ep).is_null() {
            if strncmp(*ep, (*v).name, nl) == 0
                && *(*ep).add(nl as usize) == b'=' as ::core::ffi::c_char
            {
                return Ok(xstrdup((*ep).add(nl as usize + 1)));
            }
            ep = ep.add(1);
        }
        return Ok(xstrdup(c"".as_ptr()));
    }
    let saved_varp = ctx.expanding_var.get();
    if !(*v).fileinfo.filenm.is_null() {
        ctx.expanding_var
            .set(Some(&raw mut (*v).fileinfo as *const Floc));
    }
    if ctx.reading_file.0.get().is_null() {
        set_reading = 1;
        ctx.reading_file.0.set(&raw mut (*v).fileinfo);
    }
    if (*v).expanding() != 0 {
        if (*v).exp_count() == 0 {
            // Built before the unwind below, because the diagnostic reports the
            // location this frame installed into `ctx.expanding_var`.
            let err = fatal_err(
                ctx,
                ctx.expanding_var_floc(),
                strlen((*v).name),
                c"recursive variable '%s' references itself (eventually)".as_ptr(),
                &[FmtArg::Str(((*v).name) as *const ::core::ffi::c_char)],
            );
            // The old `exit_on_err` made unwinding moot — the process ended
            // here. Now the error travels back to a caller that keeps running,
            // so the context this frame installed has to come back off first,
            // exactly as the success path does at the tail (#442).
            if set_reading != 0 {
                ctx.reading_file.0.set(::core::ptr::null::<Floc>());
            }
            ctx.expanding_var.set(saved_varp);
            return Err(err);
        }
        (*v).set_exp_count((*v).exp_count() - 1);
    }
    if !file.is_null() {
        install_file_context(
            ctx,
            file,
            &raw mut savev,
            ::core::ptr::null_mut::<*const Floc>(),
        );
    }
    (*v).set_expanding(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    // Held rather than `?`-ed on the spot: the four restorations below have to
    // run on the error path too, or a rejected expansion would leave `v`
    // flagged as expanding and the file context installed (the cleanup-paths
    // contract from #561). The parent search joins the same hold because it
    // now runs after that context is installed, and its own reference check
    // can be rejected.
    let value = append_override_parent(ctx, v, nl).and_then(|parent| {
        if let Some(pref) = parent.as_ref() {
            if (*v).origin() == o_override {
                allocated_variable_append(ctx, v)
            } else {
                Ok(xstrdup(pref.value))
            }
        } else if (*v).origin() == o_command || (*v).origin() == o_env_override {
            allocated_expand_string_for_file(ctx, (*v).value, ::core::ptr::null_mut::<file>())
        } else if (*v).append() != 0 {
            allocated_variable_append(ctx, v)
        } else {
            allocated_expand_string_for_file(ctx, (*v).value, ::core::ptr::null_mut::<file>())
        }
    });
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
/// Find the `override` definition of `v` that a pending `+=` should append to,
/// walking the current variable set list outward. Returns null when `v` is not
/// an append or no override is in scope.
///
/// # Safety
///
/// `v` must point to a live `variable` whose name is `nl` bytes long.
unsafe fn append_override_parent(
    ctx: &crate::execctx::ExecContext,
    v: *mut variable,
    nl: size_t,
) -> Result<*mut variable, crate::build_result::BuildError> {
    let mut parent = ::core::ptr::null_mut::<variable>();
    if (*v).append() == 0 {
        return Ok(parent);
    }
    let mut sl = ctx.variable_globals.current_variable_set_list.get();
    while !sl.is_null() && parent.is_null() {
        let vp: *mut variable = lookup_variable_in_set(ctx, (*v).name, nl, (*sl).set)?;
        if !vp.is_null() && vp != v && (*vp).origin() as i32 == o_override as i32 {
            parent = vp;
        }
        sl = (*sl).next;
    }
    Ok(parent)
}
/// Resolve `name` the way variable output needs it: the variable if one is
/// defined, null otherwise, having first announced an undefined reference
/// under the `undefined-var` warning. Since #442 that announcement can be a
/// rejection, which travels out instead of ending the process.
///
/// # Safety
///
/// `name` must point to `length` readable bytes that stay live for the call.
unsafe fn lookup_for_output(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> Result<*mut variable, crate::build_result::BuildError> {
    let v = lookup_variable(ctx, name, length)?;
    if v.is_null() {
        // SAFETY: `name` points to `length` valid bytes (caller contract);
        // read-only bridge to the safe `warn_undefined`.
        warn_undefined(
            ctx,
            ::core::slice::from_raw_parts(name as *const u8, length),
        )?;
    }
    Ok(v)
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
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    let v = lookup_for_output(ctx, name, length)?;
    if v.is_null() || *(*v).value.offset(0_i32 as isize) as i32 == 0 && (*v).append() == 0 {
        return Ok(ptr);
    }
    // A recursive variable's value is freshly expanded and owned here; an
    // `OwnedCStr` reclaims it on drop instead of the manual `free` the C code
    // did. A non-recursive variable's value is borrowed from the variable.
    //
    // Since #442 a rejected recursion travels out of here rather than ending
    // the process; `OwnedCStr` still reclaims the buffer on the success path.
    let owned = ((*v).recursive() != 0)
        .then(|| {
            recursively_expand_for_file(ctx, v, ::core::ptr::null_mut::<file>()).map(OwnedCStr)
        })
        .transpose()?;
    let value = owned.as_ref().map_or((*v).value, OwnedCStr::as_ptr);
    ptr = variable_buffer_output(ctx, ptr, value, strlen(value) as size_t);
    Ok(ptr)
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
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
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
    // `map` rather than `?`: the cursor is recomputed from `offs` either way,
    // and the combinator keeps this frame branch-free apart from the null check.
    expand_variable_output(ctx, buf, name, length)
        .map(|_| ctx.variable_buffer.ptr().add(offs as usize))
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_expand_variable(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
    // Held rather than `?`-ed on the spot so the swap runs on the error path
    // too; `claim_expansion` releases the orphaned partial expansion (#561).
    let expanded = expand_variable_output(ctx, ctx.variable_buffer.ptr(), name, length);
    claim_expansion(expanded.map(|_| ()), swap_variable_buffer(ctx, obuf, olen))
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
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    let mut p: *const ::core::ffi::c_char;
    let mut p1: *const ::core::ffi::c_char;
    let mut o: *mut ::core::ffi::c_char;
    if buf.is_null() {
        buf = initialize_variable_output(ctx);
    }
    o = buf;
    let line_offset = buf.offset_from(ctx.variable_buffer.ptr()) as usize;
    if length == 0 {
        return Ok(ctx.variable_buffer.ptr());
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
                // #570 made the builtin dispatch chain (`handle_function` →
                // `expand_builtin_function` → the raw handlers) hand its
                // diagnostics back as `BuildError` values, and left this
                // bridge naming `expand_string_buf` as the thing it waited on.
                // That is now this function, so the error propagates.
                let handled = handle_function(ctx, &raw mut o, &raw mut p)?;
                if handled == 0 {
                    end = strchr(beg, closeparen as i32);
                    if end.is_null() {
                        return Err(fatal_err(
                            ctx,
                            ctx.expanding_var_floc(),
                            0,
                            c"unterminated variable reference".as_ptr(),
                            &[],
                        ));
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
                            let owned = OwnedCStr(expand_argument(ctx, beg, p)?);
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
                            // Bound before the reference is taken, so the
                            // pointer `as_mut` reads is only ever one the
                            // `Result` has already yielded.
                            let looked = lookup_variable(ctx, beg, name_len)?;
                            let v = looked.as_mut();
                            if v.is_none() {
                                // SAFETY: `beg` points to `name_len` valid
                                // bytes (`name_len = colon - beg`, both within
                                // the same buffer); read-only bridge to the
                                // safe `warn_undefined`.
                                warn_undefined(
                                    ctx,
                                    ::core::slice::from_raw_parts(beg as *const u8, name_len),
                                )?;
                            }
                            if let Some(v) = v.filter(|v| *v.value != 0) {
                                // Recursive values are freshly expanded and
                                // owned; `OwnedCStr` frees on drop in place of
                                // the manual `free` below.
                                let owned = (v.recursive() != 0)
                                    .then(|| {
                                        recursively_expand_for_file(
                                            ctx,
                                            &raw mut *v,
                                            ::core::ptr::null_mut::<file>(),
                                        )
                                        .map(OwnedCStr)
                                    })
                                    .transpose()?;
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
                        o = expand_variable_output(ctx, o, beg, end.offset_from(beg) as size_t)?;
                    }
                    // Free the expanded reference here, as the C code did.
                    drop(abeg);
                }
            }
            _ => {
                // `$X`: a single-character variable name. The guard mirrors
                // the C original, which tests `p[-1]` (the `$` itself).
                if !stop_set(*p1, MAP_BLANK | MAP_NEWLINE) {
                    o = expand_variable_output(ctx, o, p, 1)?;
                }
            }
        }
        if *p == 0 {
            break;
        }
        p = p.add(1);
    }
    Ok(ctx.variable_buffer.ptr().add(line_offset))
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_argument(
    ctx: &crate::execctx::ExecContext,
    str: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    if str == end {
        return Ok(xstrdup(c"".as_ptr()));
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
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
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
    // Held rather than `?`-ed: the file context must be restored before the
    // error leaves this frame, per the cleanup-paths-report contract (#561).
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
) -> Result<Vec<u8>, crate::build_result::BuildError> {
    // SAFETY: the inner expander remains the c2rust pointer machinery; we feed
    // it a NUL-terminated buffer and a freshly-built per-file scope, then read
    // the NUL-terminated result back into an owned Vec.
    unsafe {
        let mut savev: *mut variable_set_list = ::core::ptr::null_mut::<variable_set_list>();
        let mut savef: *const Floc = ::core::ptr::null::<Floc>();
        crate::variable::install_file_context_id(ctx, file, &raw mut savev, &raw mut savef)?;
        let cur = ctx.variable_globals.current_variable_set_list.get();
        let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut olen: size_t = 0;
        install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
        // Held rather than `?`-ed so the variable buffer is swapped back and
        // the file context restored before the error leaves this frame (the
        // cleanup-paths contract from #561).
        let expanded = expand_string_buf(
            ctx,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            string.as_ptr() as *const ::core::ffi::c_char,
            SIZE_MAX,
        );
        let result = swap_variable_buffer(ctx, obuf, olen);
        crate::variable::restore_file_context_id(ctx, cur, savev, savef);
        claim_expansion(expanded.map(|_| ()), result).map(|p| owned_nul_bytes(p))
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
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
    // Held rather than `?`-ed on the spot: the swap has to run on the error
    // path too, or a failed expansion would leave the caller's variable buffer
    // swapped out (the cleanup-paths contract from #561).
    let expanded = expand_string_for_file_c(ctx, string, file);
    claim_expansion(expanded.map(|_| ()), swap_variable_buffer(ctx, obuf, olen))
}

/// Hand the buffer a completed variable-buffer swap produced to the caller, or
/// release it when the expansion that filled it was rejected.
///
/// The swap transfers ownership of the buffer out unconditionally, so on the
/// error path nobody would claim it — the free belongs here. Shared by the two
/// entry points that swap a buffer in, so neither of them grows a branch for
/// the seam (#432 Phase B, #442).
///
/// # Safety
///
/// `produced` must be the (possibly null) buffer just returned by
/// `swap_variable_buffer`; ownership transfers into this function.
pub(crate) unsafe fn claim_expansion(
    outcome: Result<(), crate::build_result::BuildError>,
    produced: *mut ::core::ffi::c_char,
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    match outcome {
        Ok(()) => Ok(produced),
        Err(e) => {
            free(produced as *mut ::core::ffi::c_void);
            Err(e)
        }
    }
}

/// Copy a NUL-terminated expander result into owned bytes (keeping the
/// terminator) and release the original. A null buffer — the expander produced
/// nothing — reads as the empty string.
///
/// # Safety
///
/// `produced` must be null or a live NUL-terminated `malloc`ed buffer;
/// ownership transfers into this function.
unsafe fn owned_nul_bytes(produced: *mut ::core::ffi::c_char) -> Vec<u8> {
    if produced.is_null() {
        return vec![0];
    }
    let len = strlen(produced) as usize;
    let mut v = ::core::slice::from_raw_parts(produced as *const u8, len).to_vec();
    v.push(0);
    free(produced as *mut ::core::ffi::c_void);
    v
}
/// Walk the variable-set chain outward, concatenating every `+=`-style
/// definition of `name` (oldest first) into the variable buffer.
unsafe fn variable_append(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
    length: size_t,
    set: *const variable_set_list,
    local: i32,
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    if set.is_null() {
        return Ok(initialize_variable_output(ctx));
    }
    let nextlocal = (local != 0 && (*set).next_is_parent == 0) as i32;
    let v: *const variable = lookup_variable_in_set(ctx, name, length, (*set).set)?;
    if v.is_null() || (local == 0 && (*v).private_var() != 0) {
        return variable_append(ctx, name, length, (*set).next, nextlocal);
    }

    // An appending definition stacks on whatever the outer sets produce.
    let mut buf = if (*v).append() != 0 {
        variable_append(ctx, name, length, (*set).next, nextlocal)?
    } else {
        initialize_variable_output(ctx)
    };
    if buf > ctx.variable_buffer.ptr() {
        buf = variable_buffer_output(ctx, buf, c" ".as_ptr(), 1);
    }
    if (*v).recursive() == 0 {
        return Ok(variable_buffer_output(
            ctx,
            buf,
            (*v).value,
            strlen((*v).value),
        ));
    }
    // A malformed reference inside an appended definition — say
    // `FOO += $(word 1)` — now travels out through the recursion and through
    // `allocated_variable_append` instead of ending the process (#442).
    buf = expand_string_buf(ctx, buf, (*v).value, strlen((*v).value))?;
    Ok(buf.add(strlen(buf) as usize))
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn allocated_variable_append(
    ctx: &crate::execctx::ExecContext,
    v: *const variable,
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut olen: size_t = 0;
    install_variable_buffer(ctx, &raw mut obuf, &raw mut olen);
    // Held rather than `?`-ed on the spot: the swap has to run on the error
    // path too, or a failed append would leave the caller's variable buffer
    // swapped out (the cleanup-paths contract from #561).
    let appended = variable_append(
        ctx,
        (*v).name,
        strlen((*v).name) as size_t,
        ctx.variable_globals.current_variable_set_list.get(),
        1,
    );
    let produced = swap_variable_buffer(ctx, obuf, olen);
    if let Err(e) = appended {
        // Ownership of the partial expansion transferred out with the swap and
        // no caller will claim it, so it is released here rather than leaked.
        free(produced as *mut ::core::ffi::c_void);
        return Err(e);
    }
    Ok(produced)
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
    /// printf did, including the glibc "(null)" for a NULL file name.
    #[test]
    fn formats_with_and_without_file_name() {
        // SAFETY: valid NUL-terminated pointer / NULL, as the contract asks.
        unsafe {
            assert_eq!(
                no_recursive_expand_msg(c"Makefile".as_ptr(), 12, b"FOO"),
                b"Makefile:12: not recursively expanding FOO to export to shell function\n"
                    .to_vec()
            );
            assert_eq!(
                no_recursive_expand_msg(::core::ptr::null(), 0, b"PATH"),
                b"(null):0: not recursively expanding PATH to export to shell function\n".to_vec()
            );
        }
    }
}

#[cfg(test)]
mod expander_rejection_tests {
    //! Since #442 `expand_string_buf` returns `Result`, so a malformed variable
    //! reference hands a `BuildError` back instead of ending the process. That
    //! makes the `unterminated variable reference` arm reachable from a unit
    //! test for the first time — reaching it used to abort the test binary,
    //! which is why it sat at 0% coverage.
    //!
    //! Each rejection is asserted next to a well-formed input, so the success
    //! path stays pinned alongside it.

    use super::{
        expand_string_buf, initialize_variable_output, SIZE_MAX, VARIABLE_BUFFER_TEST_LOCK,
    };
    use crate::build_result::BuildError;
    use crate::make_main::initialize_stopchar_map;
    use std::ffi::CString;

    /// Expand `input` in a fresh context and return the expanded bytes, or the
    /// `BuildError` the expander rejected it with.
    unsafe fn expand(input: &str) -> Result<Vec<u8>, BuildError> {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        initialize_stopchar_map();
        // `$(word …)` and friends are looked up in the builtin table, and
        // `$(FOO)` in the variable sets; both are hash tables that must be
        // constructed before the expander walks a `$(` reference.
        let ctx = crate::execctx::ExecContext::default();
        crate::function::hash_init_function_table(&ctx);
        crate::variable::init_hash_global_variable_set(&ctx);
        let src = CString::new(input).unwrap();
        initialize_variable_output(&ctx);
        let end = expand_string_buf(
            &ctx,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            src.as_ptr(),
            SIZE_MAX,
        )?;
        assert!(!end.is_null(), "expansion returned a null cursor");
        Ok(::std::ffi::CStr::from_ptr(end).to_bytes().to_vec())
    }

    /// A reference opened with `$(` or `${` and never closed is a fatal error.
    /// Before #442 this line exited the process from inside the expander.
    #[test]
    fn rejects_unterminated_variable_reference() {
        // SAFETY: single-threaded under the shared variable-buffer lock; the
        // inputs are NUL-terminated `CString`s.
        unsafe {
            for bad in ["$(FOO", "${FOO", "text $(unclosed"] {
                assert!(
                    matches!(expand(bad), Err(BuildError::Failure)),
                    "expected {bad:?} to be rejected, not expanded"
                );
            }
        }
    }

    /// The matching well-formed references still expand. An undefined variable
    /// is empty, not an error — only the *syntax* above is fatal.
    #[test]
    fn expands_terminated_variable_reference() {
        // SAFETY: as above.
        unsafe {
            assert_eq!(expand("$(FOO)").expect("well-formed"), b"".to_vec());
            assert_eq!(expand("${FOO}").expect("well-formed"), b"".to_vec());
            assert_eq!(
                expand("text $(FOO) tail").expect("well-formed"),
                b"text  tail".to_vec()
            );
        }
    }

    /// A builtin's own rejection now travels out through the dispatch ABI
    /// (#570) and then through `expand_string_buf` (this change), rather than
    /// stopping at the bridge that used to sit between them.
    #[test]
    fn propagates_builtin_rejection_through_the_expander() {
        // SAFETY: as above.
        unsafe {
            assert!(
                matches!(expand("$(word 0,a b c)"), Err(BuildError::Failure)),
                "a bad $(word ...) index must reach the caller as a value"
            );
            assert_eq!(
                expand("$(word 2,a b c)").expect("valid index"),
                b"b".to_vec()
            );
        }
    }

    /// `$(let …)` binds its names to the words of the list, then expands its
    /// body in that scope. It is exercised here because it is one of the
    /// builtins whose body now carries the expander's `Result` out through
    /// `?` — the branch count went up, so the success path is pinned.
    #[test]
    fn expands_let_bindings() {
        // SAFETY: as above.
        unsafe {
            // Each name takes one word; the last name absorbs the remainder.
            assert_eq!(
                expand("$(let a b,1 2 3,$(a)-$(b))").expect("well-formed let"),
                b"1-2 3".to_vec()
            );
            // Fewer words than names leaves the surplus names empty.
            assert_eq!(
                expand("$(let a b,1,[$(a)][$(b)])").expect("well-formed let"),
                b"[1][]".to_vec()
            );
            // The bindings are scoped to the body and do not leak out.
            assert_eq!(
                expand("$(let x,inner,$(x))[$(x)]").expect("well-formed let"),
                b"inner[]".to_vec()
            );
        }
    }

    /// `$(foreach …)` is the other list-binding builtin reached through the
    /// same converted path.
    #[test]
    fn expands_foreach_bindings() {
        // SAFETY: as above.
        unsafe {
            assert_eq!(
                expand("$(foreach v,a b c,<$(v)>)").expect("well-formed foreach"),
                b"<a> <b> <c>".to_vec()
            );
            assert_eq!(
                expand("$(foreach v,,<$(v)>)").expect("empty list"),
                b"".to_vec()
            );
        }
    }

    /// Literal text and `$$` escapes are untouched by the conversion.
    #[test]
    fn expands_literal_text_unchanged() {
        // SAFETY: as above.
        unsafe {
            assert_eq!(
                expand("plain text").expect("literal"),
                b"plain text".to_vec()
            );
            assert_eq!(expand("a$$b").expect("escape"), b"a$b".to_vec());
        }
    }
}

#[cfg(test)]
mod recursive_expansion_tests {
    //! Since #442 `recursively_expand_for_file` returns `Result`, so the
    //! `recursive variable '%s' references itself (eventually)` diagnostic is a
    //! value rather than a `process::exit`. That makes the arm reachable from a
    //! unit test for the first time — reaching it used to abort the test
    //! binary, which is why it sat at 0% coverage.
    //!
    //! Both branches of `expand_string_buf` are covered: the substitution
    //! reference (`$(NAME:a=b)`), which has propagated since #576, and the
    //! plain `$(NAME)` reference, which goes through `expand_variable_output`
    //! and only started propagating when that cone converted.

    use super::{
        expand_string_buf, initialize_variable_output, SIZE_MAX, VARIABLE_BUFFER_TEST_LOCK,
    };
    use crate::build_result::BuildError;
    use crate::make_main::initialize_stopchar_map;
    use crate::variable::{define_variable_in_set, o_file};
    use std::ffi::CString;

    /// Define `name` as a recursive variable holding `value`, then expand
    /// `input` in the same context. `appending` marks the definition `+=`,
    /// which routes the lookup through `variable_append` instead of straight
    /// into `allocated_expand_string_for_file`.
    unsafe fn expand_with(
        name: &str,
        value: &str,
        input: &str,
        appending: bool,
    ) -> Result<Vec<u8>, BuildError> {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        crate::function::hash_init_function_table(&ctx);
        crate::variable::init_hash_global_variable_set(&ctx);
        let cname = CString::new(name).unwrap();
        let cvalue = CString::new(value).unwrap();
        let v = define_variable_in_set(
            &ctx,
            cname.as_ptr(),
            name.len() as libc::size_t,
            cvalue.as_ptr(),
            o_file,
            // Recursive: the value is expanded on reference, which is what
            // routes the lookup through `recursively_expand_for_file`.
            1,
            ctx.variable_globals.global_variable_set.as_ptr(),
            ::core::ptr::null::<crate::floc::Floc>(),
        )
        .expect("test fixture defines a well-formed name");
        if appending {
            (*v).set_append(1);
        }
        let src = CString::new(input).unwrap();
        initialize_variable_output(&ctx);
        let end = expand_string_buf(
            &ctx,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            src.as_ptr(),
            SIZE_MAX,
        )?;
        assert!(!end.is_null(), "expansion returned a null cursor");
        Ok(::std::ffi::CStr::from_ptr(end).to_bytes().to_vec())
    }

    /// A recursive variable whose value references itself never terminates, so
    /// the expander refuses it. Before #442 this line ended the process from
    /// inside `recursively_expand_for_file`.
    ///
    /// Both definition flavours are checked. The appending one (`+=`) reaches
    /// the rejection through `variable_append`, which has called
    /// `expand_string_buf` directly since #576. The plain one re-enters through
    /// `allocated_expand_string_for_file`, which #578 flipped.
    #[test]
    fn rejects_self_referencing_recursive_variable() {
        // SAFETY: single-threaded under the shared variable-buffer lock; every
        // string handed across is a NUL-terminated `CString`.
        unsafe {
            for appending in [true, false] {
                assert!(
                    matches!(
                        expand_with("SELF", "$(SELF:x=y)", "$(SELF:x=y)", appending),
                        Err(BuildError::Failure)
                    ),
                    "a self-referencing recursive variable must come back as a \
                     value (appending = {appending})"
                );
            }
        }
    }

    /// The same rejection reached through a plain `$(NAME)` reference rather
    /// than a substitution reference. That route runs through
    /// `expand_variable_output`, which bridged until this slice — so #576 and
    /// #578 could only test the substitution form.
    #[test]
    fn rejects_self_reference_through_a_plain_reference() {
        // SAFETY: as above.
        unsafe {
            for appending in [true, false] {
                assert!(
                    matches!(
                        expand_with("PSELF", "$(PSELF)", "$(PSELF)", appending),
                        Err(BuildError::Failure)
                    ),
                    "a plain self-reference must come back as a value \
                     (appending = {appending})"
                );
            }
            // A single-character name takes the `$X` arm of `expand_string_buf`,
            // which reaches `expand_variable_output` by its own path.
            assert!(matches!(
                expand_with("S", "$S", "$S", false),
                Err(BuildError::Failure)
            ));
        }
    }

    /// The same route with a value that terminates still expands, so the
    /// rejection above is about the recursion and not about substitution
    /// references in general. This pins the success path of the arms that now
    /// carry a `Result` back out of `recursively_expand_for_file` — the
    /// appending case through `allocated_variable_append`, and the plain one
    /// through the expander.
    #[test]
    fn expands_substitution_reference_on_a_recursive_variable() {
        // SAFETY: as above.
        unsafe {
            assert_eq!(
                expand_with("SRCS", "a.c b.c", "$(SRCS:.c=.o)", false).expect("well-formed"),
                b"a.o b.o".to_vec()
            );
            assert_eq!(
                expand_with("SRCS", "a.c b.c", "$(SRCS:.c=.o)", true).expect("well-formed"),
                b"a.o b.o".to_vec()
            );
        }
    }
}

#[cfg(test)]
mod expander_cleanup_path_tests {
    //! Since #442 the three expander entry points — `expand_argument`,
    //! `expand_string_for_file{,_c}` and `allocated_expand_string_for_file` —
    //! return `Result`, so a malformed reference comes back as a value instead
    //! of ending the process. Each of them swaps a buffer or installs a context
    //! that has to be undone before the error leaves the frame; these tests
    //! drive that error path and then check the frame was left clean.

    use super::{
        allocated_expand_string_for_file, expand_argument, expand_string_for_file,
        expand_string_for_file_c, install_variable_buffer, VARIABLE_BUFFER_TEST_LOCK,
    };
    use crate::build_result::BuildError;
    use crate::ffi_types::size_t;
    use crate::file::File;
    use std::ffi::CString;

    /// `$(word 1)` is a well-formed reference to a builtin called with the
    /// wrong number of arguments, so expanding it is refused.
    const BAD: &str = "$(word 1)";

    fn fresh_ctx() -> crate::execctx::ExecContext {
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        // SAFETY: fresh context; both tables are initialized once per test.
        unsafe {
            crate::function::hash_init_function_table(&ctx);
            crate::variable::init_hash_global_variable_set(&ctx);
        }
        ctx
    }

    /// The allocating entry point installs a fresh variable buffer and swaps it
    /// back out. On the error path the swap still has to run — otherwise the
    /// caller's buffer stays swapped out — and the partial expansion it hands
    /// back is owned by nobody, so it is freed rather than leaked.
    #[test]
    fn allocated_expansion_rejects_and_restores_the_buffer() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: NUL-terminated source; null file means the global context.
        unsafe {
            let mut obuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
            let mut olen: size_t = 0;
            install_variable_buffer(&ctx, &raw mut obuf, &raw mut olen);
            let outer = ctx.variable_buffer.ptr();

            let bad = CString::new(BAD).unwrap();
            let outcome = allocated_expand_string_for_file(
                &ctx,
                bad.as_ptr(),
                ::core::ptr::null_mut::<File>(),
            );

            assert!(matches!(outcome, Err(BuildError::Failure)));
            assert_eq!(
                ctx.variable_buffer.ptr(),
                outer,
                "the caller's variable buffer must be swapped back in"
            );

            // The same call on a well-formed source still yields the expansion.
            let good = CString::new("plain").unwrap();
            let p = allocated_expand_string_for_file(
                &ctx,
                good.as_ptr(),
                ::core::ptr::null_mut::<File>(),
            )
            .expect("well-formed");
            assert_eq!(::std::ffi::CStr::from_ptr(p).to_bytes(), b"plain");
            libc::free(p as *mut ::core::ffi::c_void);
        }
    }

    /// `expand_string_for_file_c` with a null file takes the no-context arm,
    /// which now returns the expander's verdict directly.
    #[test]
    fn raw_expansion_without_a_file_context_rejects() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: NUL-terminated source; null file selects the global context.
        unsafe {
            super::initialize_variable_output(&ctx);
            let bad = CString::new(BAD).unwrap();
            assert!(matches!(
                expand_string_for_file_c(&ctx, bad.as_ptr(), ::core::ptr::null_mut::<File>()),
                Err(BuildError::Failure)
            ));
        }
    }

    /// `expand_argument` has two routes: a null/empty `end` delegates straight
    /// to the allocating entry point, a non-null `end` copies the `[str, end)`
    /// slice first. Both must surface the rejection, and the degenerate
    /// `str == end` case still yields an empty string.
    #[test]
    fn expand_argument_rejects_on_both_routes() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: `bad` is NUL-terminated and the end pointer indexes it.
        unsafe {
            let bad = CString::new(BAD).unwrap();
            let beg = bad.as_ptr();

            assert!(matches!(
                expand_argument(&ctx, beg, ::core::ptr::null()),
                Err(BuildError::Failure)
            ));
            assert!(matches!(
                expand_argument(&ctx, beg, beg.add(BAD.len())),
                Err(BuildError::Failure)
            ));

            let empty = expand_argument(&ctx, beg, beg).expect("str == end is the empty string");
            assert_eq!(::std::ffi::CStr::from_ptr(empty).to_bytes(), b"");
            libc::free(empty as *mut ::core::ffi::c_void);
        }
    }

    /// Expand `src` in a fresh global context and return the bytes.
    unsafe fn expand(ctx: &crate::execctx::ExecContext, src: &str) -> Vec<u8> {
        let c = CString::new(src).unwrap();
        let p = allocated_expand_string_for_file(ctx, c.as_ptr(), ::core::ptr::null_mut::<File>())
            .unwrap_or_else(|_| panic!("`{src}` should expand"));
        let out = ::std::ffi::CStr::from_ptr(p).to_bytes().to_vec();
        libc::free(p as *mut ::core::ffi::c_void);
        out
    }

    /// The builtins that expand their own arguments — `$(foreach)`, `$(let)`,
    /// `$(intcmp)`, `$(if)`, `$(or)`, `$(and)` — each grew a `?` on that
    /// expansion in this slice. Pin their success paths so the rejection
    /// plumbing below is a change in the failure behaviour only.
    #[test]
    fn argument_expanding_builtins_still_expand() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: every source is a NUL-terminated `CString` in a fresh context.
        unsafe {
            assert_eq!(expand(&ctx, "$(foreach v,a b,[$(v)])"), b"[a] [b]".to_vec());
            assert_eq!(expand(&ctx, "$(let x,7,<$(x)>)"), b"<7>".to_vec());
            assert_eq!(expand(&ctx, "$(intcmp 1,2,lt,eq,gt)"), b"lt".to_vec());
            assert_eq!(expand(&ctx, "$(intcmp 2,2,lt,eq,gt)"), b"eq".to_vec());
            assert_eq!(expand(&ctx, "$(if ,then,else)"), b"else".to_vec());
            assert_eq!(expand(&ctx, "$(if x,then,else)"), b"then".to_vec());
            assert_eq!(expand(&ctx, "$(or ,,last)"), b"last".to_vec());
            assert_eq!(expand(&ctx, "$(and x,y)"), b"y".to_vec());
            assert_eq!(expand(&ctx, "$(and x,,y)"), b"".to_vec());
        }
    }

    /// ...and each of them now surfaces a rejection from inside an argument
    /// instead of ending the process there.
    #[test]
    fn argument_expanding_builtins_propagate_rejections() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: as above.
        unsafe {
            for src in [
                "$(foreach $(word 1),a,x)",
                "$(let $(word 1),a,x)",
                "$(intcmp $(word 1),2,lt,eq,gt)",
                "$(if $(word 1),then)",
                "$(or $(word 1))",
                "$(and $(word 1))",
            ] {
                let c = CString::new(src).unwrap();
                assert!(
                    matches!(
                        allocated_expand_string_for_file(
                            &ctx,
                            c.as_ptr(),
                            ::core::ptr::null_mut::<File>()
                        ),
                        Err(BuildError::Failure)
                    ),
                    "`{src}` must come back as a value"
                );
            }
        }
    }

    /// The `FileId` form installs the target's scope and a fresh buffer. Both
    /// have to be undone on the error path, so a second expansion in the same
    /// context still behaves.
    #[test]
    fn file_scoped_expansion_rejects_and_stays_usable() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        let f = crate::file::enter_file(&ctx, b"expander_reject_probe");
        let before = ctx.variable_globals.current_variable_set_list.get();

        let mut bad = BAD.as_bytes().to_vec();
        bad.push(0);
        assert!(matches!(
            expand_string_for_file(&ctx, &bad, f),
            Err(BuildError::Failure)
        ));
        assert_eq!(
            ctx.variable_globals.current_variable_set_list.get(),
            before,
            "the file's scope must be restored on the error path too"
        );

        assert_eq!(
            expand_string_for_file(&ctx, b"plain\0", f).expect("well-formed"),
            b"plain\0".to_vec()
        );
    }
}
