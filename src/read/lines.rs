//! Raw makefile line reading from an `ebuffer`, split out of `read.rs`.
//!
//! [`readline`] reads one logical line (joining backslash-continuations) from a
//! file-backed buffer, growing it as needed; [`readstring`] does the same for an
//! in-memory string buffer. Behavior-preserving move of the line-reading
//! concern; re-exported from [`crate::read`] so the public paths are unchanged.

use super::*;

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn readstring(ebuf: *mut ebuffer) -> ::core::ffi::c_long {
    let mut eol: *mut ::core::ffi::c_char;
    if (*ebuf).bufnext >= (*ebuf).bufstart.offset((*ebuf).size as isize) {
        return -1_i32 as ::core::ffi::c_long;
    }
    (*ebuf).buffer = (*ebuf).bufnext;
    eol = (*ebuf).buffer;
    loop {
        let mut backslash: i32 = 0;
        let bol: *const ::core::ffi::c_char = eol;
        let mut p: *const ::core::ffi::c_char;
        eol = strchr(eol, '\n' as i32);
        p = eol;
        if eol.is_null() {
            (*ebuf).bufnext = (*ebuf).bufstart.offset((*ebuf).size as isize).offset(1);
            return 0;
        }
        while p > bol && {
            p = p.offset(-1_i32 as isize);
            *p as i32 == '\\' as i32
        } {
            backslash = (backslash == 0) as i32;
        }
        if backslash == 0 {
            break;
        }
        eol = eol.offset(1_i32 as isize);
    }
    *eol = 0;
    (*ebuf).bufnext = eol.offset(1_i32 as isize);
    0
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn readline(
    ctx: &crate::execctx::ExecContext,
    ebuf: *mut ebuffer,
) -> ::core::ffi::c_long {
    let mut p: *mut ::core::ffi::c_char;
    let mut end: *mut ::core::ffi::c_char;
    let mut start: *mut ::core::ffi::c_char;
    let mut nlines: ::core::ffi::c_long = 0;
    if (*ebuf).fp.is_null() {
        return readstring(ebuf);
    }
    start = (*ebuf).bufstart;
    p = start;
    end = p.offset((*ebuf).size as isize);
    *p = 0;
    while !fgets(
        p,
        end.offset_from(p) as ::core::ffi::c_long as i32,
        (*ebuf).fp,
    )
    .is_null()
    {
        let mut p2: *mut ::core::ffi::c_char;
        let mut len: size_t;
        let mut backslash: i32;
        len = strlen(p) as size_t;
        if len == 0 {
            error(ctx, &raw mut (*ebuf).floc, 0, b"warning: NUL character seen; rest of line ignored\0" as *const u8
                    as *const ::core::ffi::c_char, &[]);
            *p.offset(0_i32 as isize) = '\n' as i32 as ::core::ffi::c_char;
            len = 1;
        }
        p = p.offset(len as isize);
        if !(*p.offset(-1_i32 as isize) as i32 != '\n' as i32) {
            nlines += 1;
            if p.offset_from(start) as ::core::ffi::c_long > 1
                && *p.offset(-2_i32 as isize) as i32 == '\r' as i32
            {
                p = p.offset(-1_i32 as isize);
                memmove(
                    p.offset(-(1_i32 as isize)) as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    strlen(p).wrapping_add(1),
                );
            }
            backslash = 0;
            p2 = p.offset(-(2_i32 as isize));
            while p2 >= start {
                if *p2 as i32 != '\\' as i32 {
                    break;
                }
                backslash = (backslash == 0) as i32;
                p2 = p2.offset(-1_i32 as isize);
            }
            if backslash == 0 {
                *p.offset(-1_i32 as isize) = 0;
                break;
            } else if end.offset_from(p) as ::core::ffi::c_long >= 80 {
                continue;
            }
        }
        let off: size_t = p.offset_from(start) as ::core::ffi::c_long as size_t;
        (*ebuf).size = (*ebuf).size.wrapping_mul(2);
        (*ebuf).bufstart =
            xrealloc(start as *mut ::core::ffi::c_void, (*ebuf).size) as *mut ::core::ffi::c_char;
        (*ebuf).buffer = (*ebuf).bufstart;
        start = (*ebuf).buffer;
        p = start.offset(off as isize);
        end = start.offset((*ebuf).size as isize);
        *p = 0;
    }
    if ferror((*ebuf).fp) != 0 {
        pfatal_with_name(ctx, (*ebuf).floc.filenm);
    }
    if nlines != 0 {
        nlines
    } else {
        (if p == (*ebuf).bufstart { -1_i32 } else { 1 }) as ::core::ffi::c_long
    }
}
