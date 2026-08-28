//! Raw makefile line reading from an `EBuffer`, split out of `read.rs`.
//!
//! [`readline`] reads one logical line (joining backslash-continuations) from a
//! file-backed buffer, growing it as needed; [`readstring`] does the same for an
//! in-memory string buffer. Behavior-preserving move of the line-reading
//! concern; re-exported from [`crate::read`] so the public paths are unchanged.

use super::*;

/// Owned, buffered replacement for the `FILE*` a makefile `EBuffer` used to
/// carry. `EBuffer` stays `Copy`, so it holds a raw pointer to one of these
/// (`into_raw`/`from_raw`-managed by `eval_makefile`) rather than the reader
/// itself.
pub struct MakefileReader {
    inner: std::io::BufReader<std::fs::File>,
    err: Option<i32>,
}

impl MakefileReader {
    /// Box a freshly opened file and leak it to a raw pointer for `EBuffer.fp`.
    pub fn into_raw(f: std::fs::File) -> *mut MakefileReader {
        Box::into_raw(Box::new(MakefileReader {
            inner: std::io::BufReader::new(f),
            err: None,
        }))
    }

    pub fn as_raw_fd(&self) -> i32 {
        // `std::os::fd` (unlike `std::os::unix::io`) is available on both
        // unix and wasi, and provides the identical `AsRawFd` trait.
        use std::os::fd::AsRawFd;
        self.inner.get_ref().as_raw_fd()
    }

    /// fgets(3) over the buffered reader: read at most `n-1` bytes into `p`,
    /// stopping after a newline, and NUL-terminate. Returns false at EOF with
    /// nothing read or on a read error (recorded for [`Self::error`], matching
    /// fgets returning NULL with ferror set).
    ///
    /// # Safety
    /// `p` must be valid for writes of `n` bytes.
    pub unsafe fn fgets(&mut self, p: *mut ::core::ffi::c_char, n: i32) -> bool {
        use std::io::BufRead;
        if n <= 1 {
            return false;
        }
        let max = (n - 1) as usize;
        let mut i = 0usize;
        while i < max {
            let avail = loop {
                match self.inner.fill_buf() {
                    Ok(b) => break b,
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        self.err = Some(e.raw_os_error().unwrap_or(0));
                        return false;
                    }
                }
            };
            if avail.is_empty() {
                break;
            }
            let take = avail.len().min(max - i);
            let nl = avail[..take].iter().position(|&b| b == b'\n');
            let cnt = nl.map_or(take, |x| x + 1);
            ::core::ptr::copy_nonoverlapping(
                avail.as_ptr() as *const ::core::ffi::c_char,
                p.add(i),
                cnt,
            );
            self.inner.consume(cnt);
            i += cnt;
            if nl.is_some() {
                break;
            }
        }
        if i == 0 {
            return false;
        }
        *p.add(i) = 0;
        true
    }

    /// The errno of the first read error, if any — the ferror(3) check.
    pub fn error(&self) -> Option<i32> {
        self.err
    }
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn readstring(ebuf: *mut EBuffer) -> ::core::ffi::c_long {
    let mut eol: *mut ::core::ffi::c_char;
    if (*ebuf).bufnext >= (*ebuf).bufstart.add((*ebuf).size) {
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
            (*ebuf).bufnext = (*ebuf).bufstart.add((*ebuf).size).add(1);
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
    ebuf: *mut EBuffer,
) -> ::core::ffi::c_long {
    let mut p: *mut ::core::ffi::c_char;
    let mut end: *mut ::core::ffi::c_char;
    let mut start: *mut ::core::ffi::c_char;
    let mut nlines: ::core::ffi::c_long = 0;
    let reader = match (*ebuf).fp.as_mut() {
        Some(r) => r,
        None => return readstring(ebuf),
    };
    start = (*ebuf).bufstart;
    p = start;
    end = p.add((*ebuf).size);
    *p = 0;
    while reader.fgets(p, end.offset_from(p) as ::core::ffi::c_long as i32) {
        let mut p2: *mut ::core::ffi::c_char;
        let mut len: size_t;
        let mut backslash: i32;
        len = strlen(p) as size_t;
        if len == 0 {
            error(
                ctx,
                &raw mut (*ebuf).floc,
                0,
                b"warning: NUL character seen; rest of line ignored\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
            *p.offset(0_i32 as isize) = '\n' as i32 as ::core::ffi::c_char;
            len = 1;
        }
        p = p.add(len);
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
        p = start.add(off);
        end = start.add((*ebuf).size);
        *p = 0;
    }
    if let Some(e) = reader.error() {
        *__errno_location() = e;
        pfatal_with_name(ctx, (*ebuf).floc.filenm);
    }
    if nlines != 0 {
        nlines
    } else {
        (if p == (*ebuf).bufstart { -1_i32 } else { 1 }) as ::core::ffi::c_long
    }
}
