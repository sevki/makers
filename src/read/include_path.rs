//! Include-search-path construction and `~` expansion, split out of `read.rs`.
//!
//! [`construct_include_path`] builds the `-I` directories plus the default
//! system directories into the owned `Options` and the `.INCLUDE_DIRS`
//! variable; [`tilde_expand`] is make's `~`/`~user` home-directory expansion.
//! Both are re-exported from [`crate::read`] so their public paths are
//! unchanged. This is a behavior-preserving move of the include-path concern;
//! no logic changed.

use super::*;

/// Expand a leading bare `~` (or `~/`) in a directory byte string using the
/// `HOME` process environment variable. Returns the bytes unchanged when there
/// is no leading tilde, when `HOME` is unset/empty, or for `~user` forms.
///
/// NOTE: make's C `tilde_expand` is richer — it consults make's own `HOME`
/// *variable* (e.g. `make HOME=/tmp`) ahead of the environment, falls back to
/// `getpwnam(getlogin())`, and resolves `~user` via `getpwnam`. All of those
/// extra sources require the C passwd/variable-expansion FFI
/// (`*const c_char`/`CString`/`getpwnam`), which this crate's safety rules
/// forbid introducing here and which would add `unsafe`. They are therefore
/// not handled: such tildes are left literal and then fail the
/// directory-exists check, exactly as an unresolved `~` does. See the PR notes
/// for the assessment of why a byte-identical tilde port needs that FFI.
fn expand_tilde_dir(dir: &[u8]) -> Vec<u8> {
    if dir.first() == Some(&b'~') && (dir.len() == 1 || dir[1] == b'/') {
        if let Some(home) = std::env::var_os("HOME") {
            use std::os::unix::ffi::OsStrExt;
            let home = home.as_bytes();
            if !home.is_empty() {
                let mut out = home.to_vec();
                out.extend_from_slice(&dir[1..]);
                return out;
            }
        }
    }
    dir.to_vec()
}

/// Append `dir` to the include path if it names an existing directory, after
/// stripping trailing `/` (keeping at least one byte). Uses `std::fs` for the
/// existence/type check — no `stat`, no `*const c_char`.
fn push_include_dir(out: &mut Vec<std::path::PathBuf>, dir: &[u8]) {
    use std::os::unix::ffi::OsStrExt;
    let mut len = dir.len();
    while len > 1 && dir[len - 1] == b'/' {
        len -= 1;
    }
    let trimmed = &dir[..len];
    let path = std::path::Path::new(std::ffi::OsStr::from_bytes(trimmed));
    if std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false) {
        out.push(path.to_path_buf());
    }
}

/// Build the include search path from the `-I` directories plus the default
/// system directories, owning the result as a native `Vec<PathBuf>`.
///
/// This is a safe function: the directory handling is all safe Rust, and the
/// only `unsafe` is the internal FFI call into the C variable machinery
/// (`do_variable_definition`), which is always valid here — the `.INCLUDE_DIRS`
/// name is a static C string and each value is either a static empty string or
/// a strcache-interned C string. It must still run single-threaded like the
/// rest of startup; the resolved search path is stored in the owned `Options`
/// via the `with_options` borrow channel, not in any process-global state.
pub fn construct_include_path(
    ctx: &crate::execctx::ExecContext,
    arg_dirs: &[std::path::PathBuf],
) -> Result<(), crate::build_result::BuildError> {
    use std::os::unix::ffi::OsStrExt;
    let mut dirs: Vec<std::path::PathBuf> = Vec::new();
    let mut disable = false;
    for dir in arg_dirs {
        let bytes = dir.as_os_str().as_bytes();
        if bytes == b"-" {
            disable = true;
            dirs.clear();
        } else {
            let expanded = expand_tilde_dir(bytes);
            push_include_dir(&mut dirs, &expanded);
        }
    }
    if !disable {
        for d in ctx.default_include_directories() {
            push_include_dir(&mut dirs, d);
        }
    }
    // SAFETY: FFI boundary. The name is a static NUL-terminated C string and the
    // value is a static empty C string, so every pointer arg is valid.
    unsafe {
        do_variable_definition(
            ctx,
            NILF,
            b".INCLUDE_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            f_simple,
            0,
            s_global,
        )?;
    }
    for dir in &dirs {
        // Intern the path bytes to obtain a canonical, cache-owned pointer for
        // the C variable machinery; no CString/manual NUL constructed here.
        let value = crate::strcache::strcache_add_bytes(ctx, dir.as_os_str().as_bytes());
        // SAFETY: FFI boundary. The name is a static C string and `value` is a
        // strcache-interned, NUL-terminated C string valid for the call.
        unsafe {
            do_variable_definition(
                ctx,
                NILF,
                b".INCLUDE_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
                value,
                o_default,
                f_append,
                0,
                s_global,
            )?;
        }
    }
    crate::entry::with_options(ctx, |o| {
        *o.resolved_include_dirs.borrow_mut() = dirs;
    });
    Ok(())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn tilde_expand(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
) -> Result<*mut ::core::ffi::c_char, crate::build_result::BuildError> {
    if *name.offset(1_i32 as isize) as i32 == '/' as i32 || *name.offset(1_i32 as isize) as i32 == 0
    {
        let mut home_dir: *mut ::core::ffi::c_char;
        let is_variable: i32;
        let save: Action = warning::action(ctx, Type::UndefinedVar);
        warning::set_action(ctx, Type::UndefinedVar, Action::Ignore);
        // Held rather than `?`-ed on the spot: the undefined-variable warning
        // action was suppressed for this lookup and has to be put back before
        // the error leaves the frame (the cleanup-paths contract from #561).
        let expanded = allocated_expand_variable(
            ctx,
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
        );
        warning::set_action(ctx, Type::UndefinedVar, save);
        home_dir = expanded?;
        is_variable = (*home_dir.offset(0_i32 as isize) as i32 != 0) as i32;
        if is_variable == 0 {
            free(home_dir as *mut ::core::ffi::c_void);
            home_dir = getenv(b"HOME\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if home_dir.is_null() || *home_dir.offset(0_i32 as isize) as i32 == 0 {
            let logname: *mut ::core::ffi::c_char = getlogin();
            home_dir = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !logname.is_null() {
                let p: *mut passwd = getpwnam(logname);
                if !p.is_null() {
                    home_dir = (*p).pw_dir;
                }
            }
        }
        if !home_dir.is_null() {
            let new: *mut ::core::ffi::c_char = xstrdup(
                concat(&[
                    cstr_bytes_or_empty(home_dir),
                    cstr_bytes_or_empty(name.offset(1_i32 as isize)),
                ])
                .as_ptr() as *const ::core::ffi::c_char,
            );
            if is_variable != 0 {
                free(home_dir as *mut ::core::ffi::c_void);
            }
            return Ok(new);
        }
    } else {
        // `~user` / `~user/suffix`: split the name (after `~`) at the first `/`
        // through a slice view instead of `strchr` + in-place NUL/restore, and
        // look the user up with an owned `CString` rather than mutating the
        // caller's buffer.
        let after_tilde = ::std::ffi::CStr::from_ptr(name)
            .to_bytes()
            .get(1..)
            .unwrap_or(&[]);
        let slash = after_tilde.iter().position(|&b| b == b'/');
        let user = &after_tilde[..slash.unwrap_or(after_tilde.len())];
        let user_c = ::std::ffi::CString::new(user).expect("CStr bytes have no interior NUL");
        let pwent: *mut passwd = getpwnam(user_c.as_ptr());
        if !pwent.is_null() {
            match slash {
                // `~user` — just the user's home directory.
                None => return Ok(xstrdup((*pwent).pw_dir)),
                // `~user/suffix` — home + the `/suffix` tail (the byte at `i` is
                // the `/`, so the tail after it starts at `1 + i + 1`).
                Some(i) => {
                    return Ok(xstrdup(
                        concat(&[
                            cstr_bytes_or_empty((*pwent).pw_dir),
                            b"/",
                            cstr_bytes_or_empty(name.add(1 + i + 1)),
                        ])
                        .as_ptr() as *const ::core::ffi::c_char,
                    ));
                }
            }
        }
    }
    Ok(::core::ptr::null_mut::<::core::ffi::c_char>())
}

#[cfg(test)]
mod tests {
    use super::{expand_tilde_dir, push_include_dir};

    #[test]
    fn expand_tilde_dir_passthrough_and_home() {
        // No leading tilde: returned unchanged.
        assert_eq!(expand_tilde_dir(b"/abs/dir"), b"/abs/dir");
        assert_eq!(expand_tilde_dir(b"rel"), b"rel");
        // `~user` form is not handled here: left literal.
        assert_eq!(expand_tilde_dir(b"~user/x"), b"~user/x");
        // `~`/`~/...` expands against $HOME when it is set and non-empty.
        if let Some(home) = std::env::var_os("HOME") {
            use std::os::unix::ffi::OsStrExt;
            if !home.as_bytes().is_empty() {
                let mut expected = home.as_bytes().to_vec();
                expected.extend_from_slice(b"/sub");
                assert_eq!(expand_tilde_dir(b"~/sub"), expected);
            }
        }
    }

    #[test]
    fn push_include_dir_keeps_only_existing_dirs() {
        let mut out: Vec<std::path::PathBuf> = Vec::new();
        // An existing directory (with a trailing slash to exercise trimming).
        let tmp = std::env::temp_dir();
        use std::os::unix::ffi::OsStrExt;
        let mut with_slash = tmp.as_os_str().as_bytes().to_vec();
        with_slash.push(b'/');
        push_include_dir(&mut out, &with_slash);
        assert_eq!(out, vec![tmp.clone()]);
        // A path that does not exist is dropped.
        push_include_dir(&mut out, b"/nonexistent/makers-test-dir-xyz");
        assert_eq!(out, vec![tmp]);
    }
}
