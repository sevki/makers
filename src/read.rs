pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __size_t, __syscall_slong_t, __time_t, __uid_t, size_t, uintmax_t,
};

/// Include-search-path construction and `~` expansion (split out of this file).
mod include_path;
pub use include_path::{construct_include_path, tilde_expand};

/// Raw makefile line reading from an `EBuffer` (split out of this file).
mod lines;
use crate::file::{dep, file, FileId, NameSeq};
use crate::file::{CommandState, Commands, Dep, File, UpdateStatus, VariableSet, VariableSetList};
use crate::misc::{
    collapse_continuations, find_next_token, next_token, xmalloc, xrealloc, xstrdup, xstrndup,
};
use crate::output::FmtArg;
use crate::strcache::{strcache_add, strcache_add_bytes};
use c2rust_bitfields;
use libc::{
    __errno_location, free, getenv, getlogin, strchr, strcpy, strerror, strpbrk,
};
pub use lines::{readline, readstring};
extern "C" {
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
    fn memset(__s: *mut ::core::ffi::c_void, __c: i32, __n: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn glob(
        __pattern: *const ::core::ffi::c_char,
        __flags: i32,
        __errfunc: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, i32) -> i32>,
        __pglob: *mut glob_t,
    ) -> i32;
    fn globfree(__pglob: *mut glob_t);
    fn getpwnam(__name: *const ::core::ffi::c_char) -> *mut passwd;
}
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
use crate::warning::{self, Action, Type};
pub type dirent = crate::dir::dirent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut ::core::ffi::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: i32,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut dirent>,
    pub gl_opendir:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> i32>,
    pub gl_stat: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> i32>,
}
pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;
pub type HashTable = crate::hash::HashTable;
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
pub type variable_scope = ::core::ffi::c_uint;
pub const s_pattern: variable_scope = 2;
pub const s_target: variable_scope = 1;
pub const s_global: variable_scope = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
use crate::ar::{ar_glob, ar_name_err, ParsedArName};
use crate::dir::{dir_setup_glob, file_exists_p};
use crate::expand::{
    allocated_expand_string_for_file, allocated_expand_variable, expand_string_buf,
    variable_buffer_output,
};
pub use crate::file::nameseq;
use crate::file::{enter_file, lookup_file};
use crate::function::{patsubst_expand_pat, pattern_matches, strip_whitespace};
use crate::load::load_file;
use crate::make_main::{
    db_level, one_shell, opt_snapped_deps, posix_pedantic, second_expansion, stopchar_map,
};
use crate::misc::{concat, cstr_bytes_or_empty};
use crate::output::{
    error, fatal_err, out_of_memory, perror_with_name, pfatal_with_name,
};
use crate::posixos::fd_noinherit;
use crate::rule::create_pattern_rule;
use crate::variable::{
    assign_variable_definition, create_pattern_var, define_variable_in_set,
    do_variable_definition, initialize_file_variables, lookup_variable,
    parse_variable_definition, try_variable_definition, undefine_variable_in_set,
};
use crate::vpath::construct_vpath_list;
use ::core::ffi::CStr;
pub type goaldep = crate::file::GoalDep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EBuffer {
    pub buffer: *mut ::core::ffi::c_char,
    pub bufnext: *mut ::core::ffi::c_char,
    pub bufstart: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub fp: *mut lines::MakefileReader,
    pub floc: Floc,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VModifiers {
    #[bitfield(name = "assign_v", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "define_v", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "undefine_v", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "override_v", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "private_v", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "export_v", ty = "variable_export", bits = "5..=6")]
    pub assign_v_define_v_undefine_v_override_v_private_v_export_v: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub use crate::variable::PatternVar;
pub const w_eol: make_word_type = 1;
pub type make_word_type = ::core::ffi::c_uint;
pub const w_ampdcolon: make_word_type = 8;
pub const w_ampcolon: make_word_type = 7;
pub const w_semicolon: make_word_type = 6;
pub const w_dcolon: make_word_type = 5;
pub const w_colon: make_word_type = 4;
pub const w_variable: make_word_type = 3;
pub const w_static: make_word_type = 2;
pub const w_bogus: make_word_type = 0;
pub const c_ifneq: C2RustUnnamed = 3;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const c_endif: C2RustUnnamed = 5;
pub const c_else: C2RustUnnamed = 4;
pub const c_ifeq: C2RustUnnamed = 2;
pub const c_ifndef: C2RustUnnamed = 1;
pub const c_ifdef: C2RustUnnamed = 0;
pub const __S_IFMT: i32 = 0o170000_i32;
pub const ENOENT: i32 = 2;
pub const EINTR: i32 = 4;
pub const ENOMEM: i32 = 12;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_NUL: i32 = 0x1_i32;
pub const MAP_BLANK: i32 = 0x2_i32;
pub const MAP_COMMENT: i32 = 0x8_i32;
pub const MAP_SEMI: i32 = 0x10_i32;
pub const MAP_VARIABLE: i32 = 0x4000_i32;
pub const MAP_VMSCOMMA: i32 = 0;
pub const GLOB_ALTDIRFUNC: i32 = (1) << 9;
pub const GLOB_NOSPACE: i32 = 1;
pub const GLOB_NOMATCH: i32 = 3;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const RM_NO_DEFAULT_GOAL: i32 = (1) << 0;
pub const RM_INCLUDED: i32 = (1) << 1;
pub const RM_DONTCARE: i32 = (1) << 2;
pub const RM_NO_TILDE: i32 = (1) << 3;
pub const PARSEFS_NONE: i32 = 0;
/// Decompose a (possibly null) `*const Floc` into the owned source-location
/// triple `GoalDepNode`/`Recipe` carry: `(defined_in, lineno, offset)`, copying
/// the `filenm` C string into owned bytes (`None` for a null `filenm`).
///
/// # Safety
/// `flocp` must be null or a valid `Floc` with a null-or-valid `filenm`.
unsafe fn floc_owned(flocp: *const Floc) -> (Option<Vec<u8>>, u64, u64) {
    match flocp.as_ref() {
        None => (None, 0, 0),
        Some(f) => {
            let defined_in = if f.filenm.is_null() {
                None
            } else {
                Some(::std::ffi::CStr::from_ptr(f.filenm).to_bytes().to_vec())
            };
            (defined_in, f.lineno as u64, f.offset as u64)
        }
    }
}
/// Count the nodes in a `NameSeq` chain, starting from its head node (or
/// `None` for an empty chain). Retained as a pure, tested helper; the makefile
/// reader no longer threads `NameSeq` chains (it uses owned `Vec<ParsedName>`).
#[cfg(test)]
fn name_seq_len(head: Option<&NameSeq>) -> usize {
    let mut len: usize = 0;
    let mut cur = head;
    while let Some(node) = cur {
        len += 1;
        // SAFETY: every `next` in a well-formed chain is null or points to a
        // live `NameSeq` that outlives this borrow; `as_ref` turns null into
        // `None`, ending the walk.
        cur = unsafe { node.next.as_ref() };
    }
    len
}
pub const NONEXISTENT_MTIME: i32 = 1;
// The former `static mut reading_file` now lives on `ExecContext` as
// `ctx.reading_file` (see `execctx::ReadingFile`); every use below reads
// through that owned field instead of a process-wide static.
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn read_all_makefiles(
    ctx: &crate::execctx::ExecContext,
    mut makefiles: *mut *const ::core::ffi::c_char,
) -> Result<Vec<crate::dep::GoalDepNode>, crate::build_result::BuildError> {
    let mut num_makefiles: ::core::ffi::c_uint = 0;
    crate::variable::define_named(
            ctx,
            b"MAKEFILE_LIST\0",
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_file,
            0,
        )?;
    if 0x1_i32 & db_level(ctx) != 0 {
        crate::output::trace_out(b"Reading makefiles...\n");
    }
    let value: *mut ::core::ffi::c_char;
    let mut name: *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;
    let mut length: size_t = 0;
    value = allocated_expand_variable(
        ctx,
        b"MAKEFILES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
    )?;
    p = value;
    loop {
        name = find_next_token(
            &raw mut p as *mut *const ::core::ffi::c_char,
            &raw mut length,
        );
        if name.is_null() {
            break;
        }
        if *p as i32 != 0 {
            let fresh10 = p;
            p = p.offset(1_i32 as isize);
            *fresh10 = 0;
        }
        eval_makefile(
            ctx,
            strcache_add(ctx, name),
            (RM_NO_DEFAULT_GOAL | RM_INCLUDED | RM_DONTCARE) as ::core::ffi::c_ushort,
        )?;
    }
    free(value as *mut ::core::ffi::c_void);
    if !makefiles.is_null() {
        while let Some(mref) = makefiles.as_mut().filter(|m| !m.is_null()) {
            let d: usize = eval_makefile(ctx, *mref, 0)?;
            if *__errno_location() != 0 {
                perror_with_name(ctx, b"\0" as *const u8 as *const ::core::ffi::c_char, *mref);
            }
            // The goal carries no `name`; report the resolved file's name (the
            // former `(*(*d).file)->name`). Re-intern its bytes as a cached C
            // string for the caller's `*mref` slot.
            // Snapshot the two fields we need out of the `RefCell` before any
            // further calls (`strcache_add_bytes`, file-arena lookups) so no
            // borrow is held across them.
            let (goal_name, goal_file) = {
                let rf = ctx.read_files.borrow();
                (rf[d].dep.name.clone(), rf[d].dep.file)
            };
            let name_ptr: *const ::core::ffi::c_char = if !goal_name.is_empty() {
                strcache_add_bytes(ctx, goal_name.as_bytes())
            } else if let Some(fid) = goal_file {
                let node = ctx
                    .filenodes
                    .get(fid)
                    .expect("read_all_makefiles: missing file");
                let nm = node.lock().expect("file node lock poisoned").name.clone();
                strcache_add_bytes(ctx, &nm)
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            };
            *mref = name_ptr;
            num_makefiles = num_makefiles.wrapping_add(1);
            makefiles = makefiles.offset(1_i32 as isize);
        }
    }
    if num_makefiles == 0 {
        // Read-only table (never reassigned): `const` avoids the `Sync` bound a
        // `static` would need for raw-pointer elements — the same treatment as
        // job.rs's `default_shell`/`sh_chars`/`sh_cmds`.
        const default_makefiles: [*const ::core::ffi::c_char; 4] = [
            b"GNUmakefile\0" as *const u8 as *const ::core::ffi::c_char,
            b"makefile\0" as *const u8 as *const ::core::ffi::c_char,
            b"Makefile\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        // `const` values aren't addressable in place; bind a local so `&raw
        // const` has somewhere to point.
        let default_makefiles_table = default_makefiles;
        let mut p_0: *const *const ::core::ffi::c_char =
            &raw const default_makefiles_table as *const *const ::core::ffi::c_char;
        while !(*p_0).is_null() && file_exists_p(ctx, *p_0)? == 0 {
            p_0 = p_0.offset(1_i32 as isize);
        }
        if !(*p_0).is_null() {
            eval_makefile(ctx, *p_0, 0)?;
            if *__errno_location() != 0 {
                perror_with_name(ctx, b"\0" as *const u8 as *const ::core::ffi::c_char, *p_0);
            }
        } else {
            p_0 = &raw const default_makefiles_table as *const *const ::core::ffi::c_char;
            while !(*p_0).is_null() {
                let mut d_0 = crate::dep::GoalDepNode::default();
                let fid = enter_file(ctx, CStr::from_ptr(*p_0).to_bytes());
                d_0.dep.file = Some(fid);
                d_0.dep.flags = crate::dep::DepFlags::DONTCARE;
                ctx.read_files.borrow_mut().push(d_0);
                p_0 = p_0.offset(1_i32 as isize);
            }
        }
    }
    // The c2rust list pushed each new goal onto the *front*; we appended, so
    // return the goals in reverse-push order to preserve the observable order.
    // `RefCell::take` mirrors the former `mem::take(&mut read_files)` exactly.
    let mut goals = ctx.read_files.take();
    goals.reverse();
    // Nothing on this path can yet produce an `Err`: the only `fatal()` calls
    // reachable from here are inside `eval_makefile`'s own I/O-error handling
    // and deep inside `eval()` (via `record_files`/`do_define`/`do_undefine`/
    // `record_target_var`, all bridged back to today's exact exit behavior
    // with `.unwrap_or_else(exit_on_err)` at their call sites — see #432
    // Phase B design notes), neither of which is converted this pass. The
    // `Result` signature is added now so `main_0`'s call site can use `?`, in
    // preparation for the follow-up pass that converts `eval_makefile`.
    Ok(goals)
}
/// Install a fresh, empty conditionals frame for a nested makefile-reading
/// scope (`include`, `eval_buffer`), returning the frame it replaced so the
/// caller can hand it back to [`restore_conditionals`] once the nested
/// scope's `if`/`endif` nesting is done. The former `install_conditionals`
/// pointer swap, now a plain `RefCell::replace` — no `unsafe` needed.
pub fn install_conditionals(
    ctx: &crate::execctx::ExecContext,
) -> crate::execctx::ConditionalsFrame {
    ctx.conditionals
        .replace(crate::execctx::ConditionalsFrame::default())
}
/// Restore a conditionals frame saved by [`install_conditionals`]. The
/// nested scope's frame this replaces is dropped, freeing its `Vec`s — the
/// former manual `free(ignoring)`/`free(seen_else)`.
pub fn restore_conditionals(
    ctx: &crate::execctx::ExecContext,
    saved: crate::execctx::ConditionalsFrame,
) {
    ctx.conditionals.replace(saved);
}
/// Read makefile `filename` and record a goal for it. Returns the index of the
/// goal it pushed onto `ctx.read_files` (the pointer-free replacement for
/// returning the `*mut goaldep` node). The goal is pushed *before* reading, so
/// any goals the nested `eval` records land after it in `ctx.read_files`.
unsafe fn eval_makefile(
    ctx: &crate::execctx::ExecContext,
    mut filename: *const ::core::ffi::c_char,
    flags: ::core::ffi::c_ushort,
) -> Result<usize, crate::build_result::BuildError> {
    let mut ebuf: EBuffer = EBuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufnext: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufstart: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        fp: ::core::ptr::null_mut::<lines::MakefileReader>(),
        floc: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
    };
    let mut expanded: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let deps_idx: usize = {
        let mut rf = ctx.read_files.borrow_mut();
        let idx = rf.len();
        rf.push(crate::dep::GoalDepNode::default());
        idx
    };
    ebuf.floc.filenm = filename;
    ebuf.floc.lineno = 1;
    ebuf.floc.offset = 0;
    if 0x2_i32 & db_level(ctx) != 0 {
        let mut msg = Vec::with_capacity(64);
        msg.extend_from_slice(b"Reading makefile '");
        msg.extend_from_slice(::core::ffi::CStr::from_ptr(filename).to_bytes());
        msg.extend_from_slice(b"'");
        if flags as i32 & RM_NO_DEFAULT_GOAL != 0 {
            msg.extend_from_slice(b" (no default goal)");
        }
        if flags as i32 & RM_INCLUDED != 0 {
            msg.extend_from_slice(b" (search path)");
        }
        if flags as i32 & RM_DONTCARE != 0 {
            msg.extend_from_slice(b" (don't care)");
        }
        if flags as i32 & RM_NO_TILDE != 0 {
            msg.extend_from_slice(b" (no ~ expansion)");
        }
        msg.extend_from_slice(b"...\n");
        crate::output::trace_out(&msg);
    }
    if flags as i32 & RM_NO_TILDE == 0 && *filename.offset(0_i32 as isize) as i32 == '~' as i32 {
        expanded = tilde_expand(ctx, filename)?;
        if !expanded.is_null() {
            filename = expanded;
        }
    }
    *__errno_location() = 0;
    {
        use std::os::unix::ffi::OsStrExt;
        let path = std::ffi::OsStr::from_bytes(CStr::from_ptr(filename).to_bytes());
        loop {
            match std::fs::File::open(path) {
                Ok(f) => {
                    ebuf.fp = lines::MakefileReader::into_raw(f);
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    *__errno_location() = e.raw_os_error().unwrap_or(0);
                    break;
                }
            }
        }
    }
    ctx.read_files.borrow_mut()[deps_idx].error = *__errno_location();
    let open_error = ctx.read_files.borrow()[deps_idx].error;
    match open_error {
        EMFILE | ENFILE | ENOMEM => {
            let err: *const ::core::ffi::c_char = strerror(open_error);
            return Err(fatal_err(
                ctx,
                ctx.reading_file.0.get(),
                strlen(err) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &[FmtArg::Str((err) as *const ::core::ffi::c_char)],
            ));
        }
        _ => {}
    }
    if ebuf.fp.is_null()
        && ctx.read_files.borrow()[deps_idx].error == ENOENT
        && flags as i32 & (1) << 1 != 0
        && !(*(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
            .offset(*filename.as_ref().expect("eval_makefile: null filename")
                as ::core::ffi::c_uchar as isize) as i32
            & 0x8000_i32
            != 0)
    {
        use std::os::unix::ffi::OsStrExt;
        // `filename` is an existing C string supplied by the caller; read its
        // bytes (no new C string constructed) to build candidate paths.
        let filename_bytes = CStr::from_ptr(filename).to_bytes().to_vec();
        let filename_os = std::ffi::OsStr::from_bytes(&filename_bytes);
        // The include search path is owned by `main_0`'s `Options` and reached
        // through the `with_options` borrow channel (no `static mut`). Snapshot
        // it into a local `Vec` so the `RefCell` borrow is released before the
        // file-open work below (which re-enters the eval engine on success).
        let search_dirs: Vec<std::path::PathBuf> =
            crate::make_main::with_options(ctx, |o| o.resolved_include_dirs.borrow().clone());
        for dir in &search_dirs {
            // Native path construction: PathBuf::join, not the C `concat` helper.
            let candidate = dir.join(filename_os);
            // Open via std::fs (std handles the syscall's NUL internally); retry
            // on EINTR to match the original fopen loop.
            let opened = loop {
                match std::fs::File::open(&candidate) {
                    Ok(f) => break Ok(f),
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(e) => break Err(e),
                }
            };
            match opened {
                Ok(f) => {
                    ebuf.fp = lines::MakefileReader::into_raw(f);
                    filename =
                        crate::strcache::strcache_add_bytes(ctx, candidate.as_os_str().as_bytes());
                    break;
                }
                Err(e) => {
                    let errno = e.raw_os_error().unwrap_or(ENOENT);
                    if errno != ENOENT {
                        filename = crate::strcache::strcache_add_bytes(
                            ctx,
                            candidate.as_os_str().as_bytes(),
                        );
                        ctx.read_files.borrow_mut()[deps_idx].error = errno;
                        break;
                    }
                }
            }
        }
    }
    filename = strcache_add(ctx, filename);
    let filename_bytes = CStr::from_ptr(filename).to_bytes();
    let file_id =
        lookup_file(ctx, filename_bytes).unwrap_or_else(|| enter_file(ctx, filename_bytes));
    {
        let mut rf = ctx.read_files.borrow_mut();
        rf[deps_idx].dep.file = Some(file_id);
        rf[deps_idx].dep.flags = crate::dep::DepFlags::from_bits_truncate(flags as u32);
    }
    // Resolved name (canonical hname) and `is_explicit` mark, under the node lock.
    let resolved_name: Vec<u8> = {
        let node = ctx
            .filenodes
            .get(file_id)
            .expect("eval_makefile: file just entered is absent");
        let mut n = node.lock().expect("file node lock poisoned");
        n.is_explicit = true;
        n.name.clone()
    };
    // Re-intern the resolved name as a cached C string for the C variable layer.
    filename = crate::strcache::strcache_add_bytes(ctx, &resolved_name);
    free(expanded as *mut ::core::ffi::c_void);
    if ebuf.fp.is_null() {
        *__errno_location() = ctx.read_files.borrow()[deps_idx].error;
        let node = ctx
            .filenodes
            .get(file_id)
            .expect("eval_makefile: missing file");
        node.lock().expect("file node lock poisoned").last_mtime = NONEXISTENT_MTIME as u64;
        return Ok(deps_idx);
    }
    ctx.read_files.borrow_mut()[deps_idx].error = 0;
    {
        let node = ctx
            .filenodes
            .get(file_id)
            .expect("eval_makefile: missing file");
        let mut n = node.lock().expect("file node lock poisoned");
        if n.last_mtime == NONEXISTENT_MTIME as u64 {
            n.last_mtime = 0;
        }
    }
    fd_noinherit(
        ebuf.fp
            .as_ref()
            .expect("eval_makefile: reader just opened")
            .as_raw_fd(),
    );
    do_variable_definition(
        ctx,
        &raw mut ebuf.floc,
        b"MAKEFILE_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        filename,
        o_file,
        f_append_value,
        0,
        s_global,
    )?;
    ebuf.size = 200;
    ebuf.bufstart = xmalloc(ebuf.size) as *mut ::core::ffi::c_char;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    let curfile = ctx.reading_file.0.get();
    ctx.reading_file.0.set(&raw mut ebuf.floc);
    // Hold the result: `reading_file`, the reader and the line buffer must be
    // torn down on the error path too, so this cannot be a bare `?`.
    let evaluated = eval(
        ctx,
        &raw mut ebuf,
        (flags as i32 & RM_NO_DEFAULT_GOAL == 0) as i32,
    );
    ctx.reading_file.0.set(curfile);
    drop(Box::from_raw(ebuf.fp));
    free(ebuf.bufstart as *mut ::core::ffi::c_void);
    evaluated?;
    *__errno_location() = 0;
    Ok(deps_idx)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn eval_buffer(
    ctx: &crate::execctx::ExecContext,
    buffer: *mut ::core::ffi::c_char,
    flocp: *const Floc,
) -> Result<(), crate::build_result::BuildError> {
    let mut ebuf: EBuffer = EBuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufnext: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufstart: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        fp: ::core::ptr::null_mut::<lines::MakefileReader>(),
        floc: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
    };
    ebuf.size = strlen(buffer) as size_t;
    ebuf.bufstart = buffer;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    ebuf.fp = ::core::ptr::null_mut::<lines::MakefileReader>();
    if let Some(fl) = flocp.as_ref() {
        ebuf.floc = *fl;
    } else if !ctx.reading_file.0.get().is_null() {
        ebuf.floc = *ctx.reading_file.0.get();
    } else {
        ebuf.floc.filenm = ::core::ptr::null::<::core::ffi::c_char>();
        ebuf.floc.lineno = 1;
        ebuf.floc.offset = 0;
    }
    let curfile = ctx.reading_file.0.get();
    ctx.reading_file.0.set(&raw mut ebuf.floc);
    let saved = install_conditionals(ctx);
    // Hold the result: the conditional stack and `reading_file` must be
    // restored on the error path too, so this cannot be a bare `?`.
    let evaluated = eval(ctx, &raw mut ebuf, 1);
    restore_conditionals(ctx, saved);
    ctx.reading_file.0.set(curfile);
    evaluated
}
unsafe fn parse_var_assignment(
    ctx: &crate::execctx::ExecContext,
    line: *const ::core::ffi::c_char,
    targvar: i32,
    flocp: *const Floc,
    vmod: *mut VModifiers,
) -> *mut ::core::ffi::c_char {
    memset(
        vmod as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<VModifiers>() as size_t,
    );
    // Scan the leading modifier keywords through the typed AST layer: a pure,
    // offset-based reproduction of make's modifier loop, replacing the
    // pointer-walking `parse_variable_definition`/`end_of_token`/`next_token`
    // dance. The side effects (flag writes, the TAB warning) stay here.
    let bytes = ::std::ffi::CStr::from_ptr(line).to_bytes();
    let scan = crate::parser::scan_var_modifiers(bytes, targvar != 0);
    match scan.mods.export {
        Some(crate::parser::ExportMode::Export) => {
            (*vmod).set_export_v(v_export as variable_export);
        }
        Some(crate::parser::ExportMode::NoExport) => {
            (*vmod).set_export_v(v_noexport as variable_export);
        }
        None => {}
    }
    // These are booleans into a freshly-zeroed bitfield, so set each directly
    // from the flag rather than branching (`bool` -> the setter's integer).
    (*vmod).set_override_v(scan.mods.over.into());
    (*vmod).set_private_v(scan.mods.private.into());
    (*vmod).set_define_v(scan.mods.define.into());
    (*vmod).set_undefine_v(scan.mods.undefine.into());
    if scan.had_modifier && !flocp.is_null() {
        error(
            ctx,
            flocp,
            0,
            b"warning: directive lines cannot start with TAB\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    if scan.assign {
        (*vmod).set_assign_v(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    line.add(scan.rest) as *mut ::core::ffi::c_char
}
/// Copy a `vpath` directive's pattern token into an owned, mutable,
/// NUL-terminated buffer, mirroring the `xstrndup(p, len)` the c2rust `eval`
/// used.
///
/// `strndup(p, len)` returns a fresh allocation holding at most `len` bytes
/// of `p`, truncated at the first embedded NUL and always NUL-terminated.
/// `construct_vpath_list` hands this buffer to `find_percent`, which rewrites
/// the pattern *in place* (shifting bytes when unescaping a backslashed `%`),
/// so the storage must be a genuinely **mutable**, owned buffer — not a
/// [`CString`](std::ffi::CString), whose `as_ptr` yields a shared/read-only
/// pointer and whose invariant forbids being written through. A
/// NUL-terminated `Vec<u8>` owns its storage (released by `Drop`, replacing
/// the paired libc `free`) while permitting the in-place rewrite. `token` is
/// the token slice `[p, p + len)`; the result holds its bytes up to the first
/// NUL, followed by a trailing NUL terminator.
///
/// Calls [`out_of_memory`] on allocation failure, matching the original
/// `xstrndup`.
fn vpath_pattern_token(token: &[u8]) -> Vec<u8> {
    let end = token.iter().position(|&b| b == 0).unwrap_or(token.len());
    // Reserve fallibly so OOM routes through make's `out_of_memory()`
    // ("virtual memory exhausted") diagnostic, matching the original
    // `xstrndup`, rather than aborting via Rust's allocation-error path.
    let mut pat = Vec::new();
    if pat.try_reserve_exact(end + 1).is_err() {
        out_of_memory();
    }
    pat.extend_from_slice(&token[..end]);
    pat.push(0);
    pat
}
/// Apply the pending `export`/`unexport` state to every whitespace-separated
/// name in `list`, defining any name that does not exist yet so the flag has
/// somewhere to live.
///
/// # Safety
///
/// `list` must be a NUL-terminated buffer that stays live for the call, and
/// `fstart` must point to the location definitions made here should record.
unsafe fn mark_exported_names(
    ctx: &crate::execctx::ExecContext,
    list: *const ::core::ffi::c_char,
    exporting: i32,
    fstart: *const Floc,
) -> Result<(), crate::build_result::BuildError> {
    let mut cp: *const ::core::ffi::c_char = list;
    let mut l: size_t = 0;
    let mut p = find_next_token(&raw mut cp, &raw mut l);
    let flag = (if exporting != 0 {
        v_export as i32
    } else {
        v_noexport as i32
    }) as variable_export;
    while !p.is_null() {
        let mut v: *mut variable = lookup_variable(ctx, p, l)?;
        if v.is_null() {
            v = define_variable_in_set(
                ctx,
                p,
                l,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                o_file,
                0,
                ::core::ptr::null_mut::<variable_set>(),
                fstart,
            )?;
        }
        v.as_mut().expect("export: null variable").set_export(flag);
        p = find_next_token(&raw mut cp, &raw mut l);
    }
    Ok(())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn eval(
    ctx: &crate::execctx::ExecContext,
    ebuf: *mut EBuffer,
    set_default: i32,
) -> Result<(), crate::build_result::BuildError> {
    let mut collapsed: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut collapsed_length: size_t = 0;
    let mut commands_len: size_t = 200;
    let mut commands: *mut ::core::ffi::c_char;
    let mut commands_idx: size_t = 0;
    let mut cmds_started: ::core::ffi::c_uint;
    let mut tgts_started: ::core::ffi::c_uint;
    let mut ignoring: i32 = 0;
    let mut in_ignored_define: i32 = 0;
    let mut no_targets: i32 = 0;
    let mut also_make_targets: i32 = 0;
    // Pending rule targets: `None` = no rule in progress (the former null
    // `filenames`), `Some(vec)` = parsed target names awaiting their recipe.
    let mut filenames: Option<Vec<ParsedName>> = None;
    let mut depstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nlines: ::core::ffi::c_long = 0;
    let mut two_colon: i32 = 0;
    let mut prefix: ::core::ffi::c_char = crate::make_main::opt_cmd_prefix(ctx);
    let mut pattern: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pattern_percent: *const ::core::ffi::c_char;
    let fstart: *mut Floc;
    let mut fi: Floc = Floc {
        filenm: ::core::ptr::null::<::core::ffi::c_char>(),
        lineno: 0,
        offset: 0,
    };
    pattern_percent = ::core::ptr::null::<::core::ffi::c_char>();
    tgts_started = 1;
    cmds_started = tgts_started;
    fstart = &raw mut (*ebuf).floc;
    fi.filenm = (*ebuf).floc.filenm;
    // Owned recipe accumulator, reused across rules and grown on demand (was
    // xmalloc/xrealloc/free). `commands_len` tracks the live length, `commands_idx`
    // the fill position; the raw pointer is re-fetched after each growth. The
    // Vec is kept fully initialized (len == capacity) so a growth preserves the
    // already-written bytes rather than only the [0, len) prefix.
    let mut cmd_buf: Vec<u8> = vec![0u8; commands_len as usize];
    commands = cmd_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
    loop {
        let linelen: size_t;
        let mut line: *mut ::core::ffi::c_char;
        let mut wlen: size_t;
        let mut p: *mut ::core::ffi::c_char;
        let mut p2: *mut ::core::ffi::c_char;
        let is_rule: ::core::ffi::c_uint;
        let mut vmod: VModifiers = VModifiers {
            assign_v_define_v_undefine_v_override_v_private_v_export_v: [0; 1],
            c2rust_padding: [0; 3],
        };
        (*ebuf).floc.lineno = (*ebuf)
            .floc
            .lineno
            .wrapping_add(nlines as ::core::ffi::c_ulong);
        nlines = readline(ctx, ebuf);
        if nlines < 0 {
            break;
        }
        line = (*ebuf).buffer;
        if (*ebuf).floc.lineno == 1 {
            let ul: *mut ::core::ffi::c_uchar = line as *mut ::core::ffi::c_uchar;
            if *ul.offset(0_i32 as isize) as i32 == 0xef_i32
                && *ul.offset(1_i32 as isize) as i32 == 0xbb_i32
                && *ul.offset(2_i32 as isize) as i32 == 0xbf_i32
            {
                line = line.offset(3_i32 as isize);
                if 0x1_i32 & db_level(ctx) != 0 {
                    if !(*ebuf).floc.filenm.is_null() {
                        crate::output::trace_parts(&[
                            b"Skipping UTF-8 BOM in makefile '",
                            ::core::ffi::CStr::from_ptr((*ebuf).floc.filenm).to_bytes(),
                            b"'\n",
                        ]);
                    } else {
                        crate::output::trace_out(b"Skipping UTF-8 BOM in makefile buffer\n");
                    }
                }
            }
        }
        // Classify the line by its first byte through the typed AST: empty
        // line, recipe line (begins with `cmd_prefix`), or a line to parse.
        let first_byte = *line.offset(0_i32 as isize) as ::core::ffi::c_uchar;
        let line_kind = crate::parser::LineKind::classify(
            first_byte,
            crate::make_main::opt_cmd_prefix(ctx) as ::core::ffi::c_uchar,
        );
        if line_kind == crate::parser::LineKind::Blank {
            continue;
        }
        let initial_tab = (first_byte as i32 == '\t' as i32) as i32 as ::core::ffi::c_uint;
        linelen = strlen(line) as size_t;
        if line_kind == crate::parser::LineKind::Recipe {
            if no_targets != 0 {
                continue;
            }
            if filenames.is_some() {
                if ignoring != 0 {
                    continue;
                }
                if commands_idx == 0 {
                    cmds_started = (*ebuf).floc.lineno as ::core::ffi::c_uint;
                }
                if linelen.wrapping_add(commands_idx) > commands_len {
                    commands_len = linelen.wrapping_add(commands_idx).wrapping_mul(2);
                    cmd_buf.resize(commands_len as usize, 0);
                    commands = cmd_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
                }
                memcpy(
                    commands.offset(commands_idx as isize) as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void,
                    line.offset(1_i32 as isize) as *const ::core::ffi::c_void,
                    (linelen as size_t).wrapping_sub(1),
                );
                commands_idx = commands_idx.wrapping_add(linelen.wrapping_sub(1));
                let fresh11 = commands_idx;
                commands_idx = commands_idx.wrapping_add(1);
                *commands.offset(fresh11 as isize) = '\n' as i32 as ::core::ffi::c_char;
                continue;
            }
        }
        if collapsed_length < linelen.wrapping_add(1) {
            collapsed_length = linelen.wrapping_add(1);
            free(collapsed as *mut ::core::ffi::c_void);
            collapsed = xmalloc(collapsed_length) as *mut ::core::ffi::c_char;
        }
        strcpy(collapsed, line);
        collapse_continuations(ctx, collapsed);
        remove_comments(collapsed);
        p = collapsed;
        while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
            .offset(*p.as_ref().expect("eval: null line pointer") as ::core::ffi::c_uchar as isize)
            as i32
            & (0x2_i32 | 0x4_i32)
            != 0
        {
            p = p.offset(1_i32 as isize);
        }
        p = parse_var_assignment(
            ctx,
            p,
            0,
            if initial_tab != 0 {
                &raw mut (*ebuf).floc
            } else {
                ::core::ptr::null_mut::<Floc>()
            },
            &raw mut vmod,
        );
        if vmod.assign_v() != 0 {
            let v: *mut variable;
            let origin: variable_origin = (if vmod.override_v() as i32 != 0 {
                o_override as i32
            } else {
                o_file as i32
            }) as variable_origin;
            if ignoring != 0 {
                if vmod.define_v() != 0 {
                    in_ignored_define = 1;
                }
            } else {
                if filenames.is_some() {
                    fi.lineno = tgts_started as ::core::ffi::c_ulong;
                    fi.offset = 0;
                    record_files(
                        ctx,
                        filenames.as_deref().unwrap_or(&[]),
                        also_make_targets,
                        pattern,
                        pattern_percent,
                        depstr,
                        cmds_started,
                        commands,
                        commands_idx,
                        two_colon,
                        prefix,
                        &raw mut fi,
                    )
                    ?;
                    filenames = None;
                }
                commands_idx = 0;
                no_targets = 0;
                pattern = ::core::ptr::null::<::core::ffi::c_char>();
                also_make_targets = 0;
                if vmod.undefine_v() != 0 {
                    do_undefine(ctx, p, origin, ebuf)
                        ?;
                } else {
                    if vmod.define_v() != 0 {
                        v = do_define(ctx, p, origin, ebuf)
                            ?;
                    } else {
                        v = try_variable_definition(ctx, fstart, p, origin, s_global)?;
                    }
                    let vref = v.as_mut().expect("assertion failed: v != NULL");
                    if vmod.export_v() as i32 != v_default as i32 {
                        vref.set_export(vmod.export_v() as variable_export);
                    }
                    if vmod.private_v() != 0 {
                        vref.set_private_var(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                }
            }
        } else {
            if *p as i32 == 0 {
                continue;
            }
            // Measure the first token, skip its trailing blanks, and decide
            // whether the line is a rule (`:`/`&:`/`|:`) through the typed AST
            // layer. `p2` is left at the text after the token, as before.
            let probe = crate::parser::rule_probe(::std::ffi::CStr::from_ptr(p).to_bytes());
            wlen = probe.word_len as size_t;
            p2 = p.add(probe.rest);
            is_rule = probe.is_rule as ::core::ffi::c_uint;
            if in_ignored_define != 0 {
                // A bare `endef` (only a comment or end-of-line after it) closes
                // the ignored define; classified through the typed AST layer.
                if crate::parser::closes_ignored_define(::std::ffi::CStr::from_ptr(p).to_bytes()) {
                    in_ignored_define = 0;
                }
            } else {
                let i: i32 = conditional_line(ctx, p, wlen, fstart, initial_tab)?;
                if i != -2_i32 {
                    if i == -1_i32 {
                        return Err(fatal_err(
                            ctx,
                            fstart,
                            0,
                            b"invalid syntax in conditional\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        ));
                    }
                    ignoring = i;
                } else {
                    if ignoring != 0 {
                        continue;
                    }
                    // The leading directive keyword classified once through the
                    // typed AST layer, replacing the per-keyword memcmp walls in
                    // this dispatch (export/unexport, vpath, include-family,
                    // load-family).
                    let dword = ::core::slice::from_raw_parts(p as *const u8, wlen);
                    // The whole-line classification through the typed AST layer,
                    // used to recognise the file/path directive arms below as a
                    // single interned line node. `export`/`unexport` keep their
                    // dedicated bare-word check (make recognises them before any
                    // assignment parsing, so `export = 1` exports rather than
                    // assigning — a distinction `classify_line` deliberately
                    // leaves to the modifier scan).
                    let line_class = crate::parser::classify_line(
                        &ctx.db,
                        ::std::ffi::CStr::from_ptr(p).to_bytes(),
                        false,
                    );
                    if matches!(
                        crate::parser::VarModifier::from_word(dword),
                        Some(
                            crate::parser::VarModifier::Export
                                | crate::parser::VarModifier::Unexport
                        )
                    ) {
                        let exporting: i32 = if *p as i32 == 'u' as i32 { 0 } else { 1 };
                        if initial_tab != 0 {
                            error(
                                ctx,
                                &raw mut (*ebuf).floc,
                                strlen(if exporting != 0 {
                                    b"export\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"unexport\0" as *const u8 as *const ::core::ffi::c_char
                                }) as size_t,
                                b"warning: %s lines cannot start with TAB\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str(
                                    (if exporting != 0 {
                                        b"export\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"unexport\0" as *const u8 as *const ::core::ffi::c_char
                                    })
                                        as *const ::core::ffi::c_char,
                                )],
                            );
                        }
                        if filenames.is_some() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames.as_deref().unwrap_or(&[]),
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &raw mut fi,
                            )
                            ?;
                            filenames = None;
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        if *p2 as i32 == 0 {
                            crate::make_main::with_options(ctx, |o| {
                                o.export_all_variables.set(exporting != 0)
                            });
                        } else {
                            let ap: *mut ::core::ffi::c_char;
                            ap = allocated_expand_string_for_file(
                                ctx,
                                p2,
                                ::core::ptr::null_mut::<File>(),
                            )?;
                            let marked = mark_exported_names(ctx, ap, exporting, fstart);
                            // The expansion buffer is released before the
                            // rejection escapes, so a bad reference inside the
                            // name list does not leak it (#561).
                            free(ap as *mut ::core::ffi::c_void);
                            marked?;
                        }
                    } else if matches!(
                        line_class,
                        crate::parser::LineClass::File(crate::parser::FileDirective::Vpath)
                    ) {
                        let mut cp_0: *const ::core::ffi::c_char;
                        let mut l_0: size_t = 0;
                        if initial_tab != 0 {
                            error(
                                ctx,
                                &raw mut (*ebuf).floc,
                                0,
                                b"warning: vpath directive lines cannot start with TAB\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
                        }
                        if filenames.is_some() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames.as_deref().unwrap_or(&[]),
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &raw mut fi,
                            )
                            ?;
                            filenames = None;
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        cp_0 = expand_string_buf(
                            ctx,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            p2,
                            SIZE_MAX as size_t,
                        )?;
                        p = find_next_token(&raw mut cp_0, &raw mut l_0);
                        // Own the pattern token in a mutable, NUL-terminated
                        // `Vec<u8>` whose `Drop` releases it, replacing the
                        // `xstrndup` + `free` pair. `construct_vpath_list`
                        // passes `pattern` to `find_percent`, which rewrites
                        // the buffer *in place* (shifting bytes when unescaping
                        // a backslashed `%`), so the storage must be genuinely
                        // mutable and owned — a `CString::as_ptr()` would be a
                        // shared/read-only pointer and writing through it would
                        // violate `CString`'s invariant and Rust aliasing. The
                        // callee never retains `pattern` (it interns a copy via
                        // `strcache_add` / compares it), so the buffer never
                        // escapes to C ownership.
                        let mut vpat = if !p.is_null() {
                            let token = ::core::slice::from_raw_parts(p as *const u8, l_0 as usize);
                            let owned = vpath_pattern_token(token);
                            p = find_next_token(&raw mut cp_0, &raw mut l_0);
                            Some(owned)
                        } else {
                            None
                        };
                        construct_vpath_list(
                            ctx,
                            vpat.as_mut().map_or(::core::ptr::null_mut(), |c| {
                                c.as_mut_ptr() as *mut ::core::ffi::c_char
                            }),
                            p,
                        )?;
                    } else if matches!(
                        line_class,
                        crate::parser::LineClass::File(
                            crate::parser::FileDirective::Include
                                | crate::parser::FileDirective::IncludeOpt
                        )
                    ) {
                        let save: crate::execctx::ConditionalsFrame;
                        let files: Vec<ParsedName>;
                        let noerror: i32 = (*p.offset(0_i32 as isize) as i32 != 'i' as i32) as i32;
                        if initial_tab != 0 {
                            error(
                                ctx,
                                &raw mut (*ebuf).floc,
                                strlen(if *p as i32 == 'i' as i32 {
                                    b"include\0" as *const u8 as *const ::core::ffi::c_char
                                } else if *p as i32 == '-' as i32 {
                                    b"-include\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"sinclude\0" as *const u8 as *const ::core::ffi::c_char
                                }) as size_t,
                                b"warning: %s lines cannot start with TAB\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str(if *p as i32 == 'i' as i32 {
                                    b"include\0" as *const u8 as *const ::core::ffi::c_char
                                } else if *p as i32 == '-' as i32 {
                                    b"-include\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"sinclude\0" as *const u8 as *const ::core::ffi::c_char
                                })],
                            );
                        }
                        if filenames.is_some() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames.as_deref().unwrap_or(&[]),
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &raw mut fi,
                            )
                            ?;
                            filenames = None;
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        p = allocated_expand_string_for_file(
                            ctx,
                            p2,
                            ::core::ptr::null_mut::<file>(),
                        )?;
                        if *p as i32 == 0 {
                            free(p as *mut ::core::ffi::c_void);
                        } else {
                            p2 = p;
                            // The expanded line is released before a rejected
                            // `~` expansion leaves the frame.
                            let parsed = parse_file_seq(
                                ctx,
                                &raw mut p2,
                                ::core::mem::size_of::<nameseq>() as size_t,
                                0x1_i32,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                0x2_i32,
                            );
                            free(p as *mut ::core::ffi::c_void);
                            files = parsed?;
                            save = install_conditionals(ctx);
                            if filenames.is_some() {
                                fi.lineno = tgts_started as ::core::ffi::c_ulong;
                                fi.offset = 0;
                                record_files(
                                    ctx,
                                    filenames.as_deref().unwrap_or(&[]),
                                    also_make_targets,
                                    pattern,
                                    pattern_percent,
                                    depstr,
                                    cmds_started,
                                    commands,
                                    commands_idx,
                                    two_colon,
                                    prefix,
                                    &raw mut fi,
                                )
                                ?;
                                filenames = None;
                            }
                            commands_idx = 0;
                            no_targets = 0;
                            pattern = ::core::ptr::null::<::core::ffi::c_char>();
                            also_make_targets = 0;
                            for fentry in &files {
                                let flags: ::core::ffi::c_ushort = (RM_INCLUDED
                                    | RM_NO_TILDE
                                    | (if noerror != 0 { RM_DONTCARE } else { 0 })
                                    | (if set_default != 0 {
                                        0
                                    } else {
                                        RM_NO_DEFAULT_GOAL
                                    }))
                                    as ::core::ffi::c_ushort;
                                let mut nb = fentry.name.clone();
                                nb.push(0);
                                let d: usize = eval_makefile(
                                    ctx,
                                    nb.as_ptr() as *const ::core::ffi::c_char,
                                    flags,
                                )?;
                                // Record the goal's source location (the former
                                // `(*d)->floc = *fstart`).
                                let (defined_in, lineno, offset) = floc_owned(fstart);
                                {
                                    let mut rf = ctx.read_files.borrow_mut();
                                    rf[d].defined_in = defined_in;
                                    rf[d].lineno = lineno;
                                    rf[d].offset = offset;
                                }
                            }
                            restore_conditionals(ctx, save);
                        }
                    } else if matches!(
                        line_class,
                        crate::parser::LineClass::File(
                            crate::parser::FileDirective::Load
                                | crate::parser::FileDirective::LoadOpt
                        )
                    ) && is_rule == 0
                    {
                        let files_0: Vec<ParsedName>;
                        let noerror_0: i32 =
                            (*p.offset(0_i32 as isize) as i32 == '-' as i32) as i32;
                        if initial_tab != 0 {
                            error(
                                ctx,
                                &raw mut (*ebuf).floc,
                                strlen(if noerror_0 != 0 {
                                    b"-load\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"load\0" as *const u8 as *const ::core::ffi::c_char
                                }) as size_t,
                                b"warning: %s lines cannot start with TAB\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str(
                                    (if noerror_0 != 0 {
                                        b"-load\0" as *const u8 as *const ::core::ffi::c_char
                                    } else {
                                        b"load\0" as *const u8 as *const ::core::ffi::c_char
                                    })
                                        as *const ::core::ffi::c_char,
                                )],
                            );
                        }
                        if filenames.is_some() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames.as_deref().unwrap_or(&[]),
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &raw mut fi,
                            )
                            ?;
                            filenames = None;
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        p = allocated_expand_string_for_file(
                            ctx,
                            p2,
                            ::core::ptr::null_mut::<file>(),
                        )?;
                        if *p as i32 == 0 {
                            free(p as *mut ::core::ffi::c_void);
                        } else {
                            p2 = p;
                            // As above: the expanded line is released first.
                            let parsed_0 = parse_file_seq(
                                ctx,
                                &raw mut p2,
                                ::core::mem::size_of::<nameseq>() as size_t,
                                0x1_i32,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                0x2_i32,
                            );
                            free(p as *mut ::core::ffi::c_void);
                            files_0 = parsed_0?;
                            for fentry in &files_0 {
                                let mut name_buf = fentry.name.clone();
                                name_buf.push(0);
                                let mut name: *const ::core::ffi::c_char =
                                    name_buf.as_ptr() as *const ::core::ffi::c_char;
                                let r: i32;
                                let mut file: file = {
                                    let mut init = File {
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
                                        variables: ::core::ptr::null_mut::<variable_set_list>(),
                                        pat_variables: ::core::ptr::null_mut::<variable_set_list>(),
                                        parent: ::core::ptr::null_mut::<File>(),
                                        double_colon: ::core::ptr::null_mut::<File>(),
                                        last_mtime: 0,
                                        mtime_before_update: 0,
                                        considered: 0,
                                        command_flags: 0,
                                        ..Default::default()
                                    };
                                    init.update_status = UpdateStatus::Success;
                                    init.command_state = CommandState::NotStarted;
                                    init.builtin = false;
                                    init.precious = false;
                                    init.loaded = false;
                                    init.unloaded = false;
                                    init.low_resolution_time = false;
                                    init.tried_implicit = false;
                                    init.updating = false;
                                    init.updated = false;
                                    init.is_target = false;
                                    init.cmd_target = false;
                                    init.phony = false;
                                    init.intermediate = false;
                                    init.is_explicit = false;
                                    init.secondary = false;
                                    init.notintermediate = false;
                                    init.dontcare = false;
                                    init.ignore_vpath = false;
                                    init.pat_searched = false;
                                    init.no_diag = false;
                                    init.was_shuffled = false;
                                    init.snapped = false;
                                    init.suffix = false;
                                    init
                                };
                                file.name = name;
                                r = load_file(ctx, &raw mut (*ebuf).floc, &raw mut file, noerror_0);
                                if r == 0 && noerror_0 == 0 {
                                    return Err(fatal_err(
                                        ctx,
                                        &raw mut (*ebuf).floc,
                                        strlen(name) as size_t,
                                        b"%s: failed to load\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[FmtArg::Str((name) as *const ::core::ffi::c_char)],
                                    ));
                                }
                                name = file.name;
                                let name_bytes = CStr::from_ptr(name).to_bytes().to_vec();
                                let f = lookup_file(ctx, &name_bytes)
                                    .unwrap_or_else(|| enter_file(ctx, &name_bytes));
                                {
                                    let node = ctx
                                        .filenodes
                                        .get(f)
                                        .expect("eval: loaded file missing from arena");
                                    let mut n = node.lock().expect("file node lock poisoned");
                                    n.loaded = true;
                                    n.unloaded = false;
                                }
                                if r == -1_i32 {
                                    continue;
                                }
                                let mut g = crate::dep::GoalDepNode::default();
                                let (defined_in, lineno, offset) =
                                    floc_owned(&raw const (*ebuf).floc);
                                g.defined_in = defined_in;
                                g.lineno = lineno;
                                g.offset = offset;
                                g.dep.file = Some(f);
                                ctx.read_files.borrow_mut().push(g);
                            }
                        }
                    } else {
                        if *line.offset(0_i32 as isize) as i32
                            == crate::make_main::opt_cmd_prefix(ctx) as i32
                        {
                            return Err(fatal_err(
                                ctx,
                                fstart,
                                0,
                                b"recipe commences before first target\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            ));
                        }
                        let mut wtype: make_word_type;
                        let mut cmdleft: *mut ::core::ffi::c_char;
                        let mut semip: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut lb_next: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut plen: size_t = 0;
                        let mut colonp: *mut ::core::ffi::c_char;
                        let mut end: *const ::core::ffi::c_char;
                        let mut beg: *const ::core::ffi::c_char;
                        if filenames.is_some() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames.as_deref().unwrap_or(&[]),
                                also_make_targets,
                                pattern,
                                pattern_percent,
                                depstr,
                                cmds_started,
                                commands,
                                commands_idx,
                                two_colon,
                                prefix,
                                &raw mut fi,
                            )
                            ?;
                            filenames = None;
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        tgts_started = (*fstart).lineno as ::core::ffi::c_uint;
                        cmdleft = find_map_unquote(line, MAP_SEMI | MAP_COMMENT | MAP_VARIABLE);
                        if !cmdleft.is_null() && cbyte(cmdleft) as i32 == '#' as i32 {
                            ::core::slice::from_raw_parts_mut(cmdleft as *mut u8, 1)[0] = 0;
                            cmdleft = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        } else if !cmdleft.is_null() {
                            let fresh12 = cmdleft;
                            cmdleft = cmdleft.offset(1_i32 as isize);
                            semip = fresh12;
                            ::core::slice::from_raw_parts_mut(semip as *mut u8, 1)[0] = 0;
                        }
                        collapse_continuations(ctx, line);
                        wtype = get_next_mword(line, &raw mut lb_next, &raw mut wlen);
                        match wtype as ::core::ffi::c_uint {
                            1 => {
                                if !cmdleft.is_null() {
                                    return Err(fatal_err(
                                        ctx,
                                        fstart,
                                        0,
                                        b"missing rule before recipe\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[],
                                    ));
                                }
                            }
                            4 | 5 | 7 | 8 => {
                                no_targets = 1;
                            }
                            _ => {
                                p2 = expand_string_buf(
                                    ctx,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    lb_next,
                                    wlen,
                                )?;
                                loop {
                                    lb_next = lb_next.offset(wlen as isize);
                                    if cmdleft.is_null() {
                                        cmdleft = find_char_unquote(p2, ';' as i32);
                                        if !cmdleft.is_null() {
                                            let p2_off: size_t = p2
                                                .offset_from(ctx.variable_buffer.ptr())
                                                as ::core::ffi::c_long
                                                as size_t;
                                            let cmd_off: size_t = cmdleft
                                                .offset_from(ctx.variable_buffer.ptr())
                                                as ::core::ffi::c_long
                                                as size_t;
                                            let pend: *mut ::core::ffi::c_char =
                                                p2.offset(strlen(p2) as isize);
                                            crate::expand::set_variable_buffer_byte(
                                                ctx, cmd_off, 0,
                                            );
                                            expand_string_buf(
                                                ctx,
                                                pend,
                                                lb_next,
                                                SIZE_MAX as size_t,
                                            )?;
                                            lb_next = lb_next.offset(strlen(lb_next) as isize);
                                            p2 = ctx.variable_buffer.ptr().add(p2_off);
                                            cmdleft = ctx.variable_buffer.ptr().add(cmd_off).offset(1);
                                        }
                                    }
                                    colonp = find_char_unquote(p2, ':' as i32);
                                    if !colonp.is_null() {
                                        let colon_off: size_t = colonp
                                            .offset_from(ctx.variable_buffer.ptr())
                                            as ::core::ffi::c_long
                                            as size_t;
                                        if colonp > p2
                                            && crate::expand::variable_buffer_byte(
                                                ctx,
                                                colon_off - 1,
                                            ) as i32
                                                == '&' as i32
                                        {
                                            colonp = colonp.offset(-1_i32 as isize);
                                        }
                                        break;
                                    } else {
                                        wtype = get_next_mword(
                                            lb_next,
                                            &raw mut lb_next,
                                            &raw mut wlen,
                                        );
                                        if wtype as ::core::ffi::c_uint
                                            == w_eol as i32 as ::core::ffi::c_uint
                                        {
                                            break;
                                        }
                                        p2 = p2.offset(strlen(p2) as isize);
                                        let fresh13 = p2;
                                        p2 = p2.offset(1_i32 as isize);
                                        *fresh13 = ' ' as i32 as ::core::ffi::c_char;
                                        p2 = expand_string_buf(ctx, p2, lb_next, wlen)?;
                                    }
                                }
                                p2 = next_token(ctx.variable_buffer.ptr());
                                if wtype as ::core::ffi::c_uint
                                    == w_eol as i32 as ::core::ffi::c_uint
                                {
                                    if *p2 as i32 == 0 {
                                        continue;
                                    }
                                    if crate::make_main::opt_cmd_prefix(ctx) as i32 == '\t' as i32
                                        && crate::parser::starts_with_eight_spaces(
                                            ::std::ffi::CStr::from_ptr(line).to_bytes(),
                                        )
                                    {
                                        return Err(fatal_err(
                                            ctx,
                                            fstart,
                                            0,
                                            b"missing separator (did you mean TAB instead of 8 spaces?)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
        &[],
    ));
                                    }
                                    p2 = next_token(line);
                                    // The more specific "ifeq/ifneq must be
                                    // followed by whitespace" diagnostic is a
                                    // pure byte classification on the token —
                                    // lift it into the typed parser layer
                                    // instead of the c2rust strncmp wall.
                                    if crate::parser::ifeq_ifneq_without_separator(
                                        ::std::ffi::CStr::from_ptr(p2).to_bytes(),
                                    ) {
                                        return Err(fatal_err(
                                            ctx,
                                            fstart,
                                            0,
                                            b"missing separator (ifeq/ifneq must be followed by whitespace)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
        &[],
    ));
                                    }
                                    return Err(fatal_err(
                                        ctx,
                                        fstart,
                                        0,
                                        b"missing separator\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[],
                                    ));
                                } else {
                                    let colon_off: size_t = colonp
                                        .offset_from(ctx.variable_buffer.ptr())
                                        as ::core::ffi::c_long
                                        as size_t;
                                    let save_0: ::core::ffi::c_char =
                                        crate::expand::variable_buffer_byte(ctx, colon_off);
                                    if save_0 as i32 == '&' as i32 {
                                        also_make_targets = 1;
                                    }
                                    crate::expand::set_variable_buffer_byte(ctx, colon_off, 0);
                                    let parsed_targets = parse_file_seq(
                                        ctx,
                                        &raw mut p2,
                                        ::core::mem::size_of::<NameSeq>() as size_t,
                                        MAP_NUL,
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        PARSEFS_NONE,
                                    )?;
                                    filenames = if parsed_targets.is_empty() {
                                        None
                                    } else {
                                        Some(parsed_targets)
                                    };
                                    crate::expand::set_variable_buffer_byte(ctx, colon_off, save_0);
                                    p2 = colonp
                                        .offset((save_0 as i32 == '&' as i32) as i32 as isize);
                                    if filenames.is_none() {
                                        no_targets = 1;
                                    } else {
                                        if *p2 as i32 != 0 {
                                        } else {
                                            panic!("assertion failed: *p2 != '\'");
                                        };
                                        p2 = p2.offset(1_i32 as isize);
                                        two_colon = (*p2 as i32 == ':' as i32) as i32;
                                        if two_colon != 0 {
                                            p2 = p2.offset(1_i32 as isize);
                                        }
                                        if *lb_next as i32 != 0 {
                                            let l_1: size_t = p2
                                                .offset_from(ctx.variable_buffer.ptr())
                                                as ::core::ffi::c_long
                                                as size_t;
                                            plen = strlen(p2) as size_t;
                                            variable_buffer_output(
                                                ctx,
                                                p2.offset(plen as isize),
                                                lb_next,
                                                (strlen(lb_next) as size_t).wrapping_add(1),
                                            );
                                            p2 = ctx.variable_buffer.ptr().add(l_1);
                                        }
                                        p2 = parse_var_assignment(
                                            ctx,
                                            p2,
                                            1,
                                            ::core::ptr::null::<Floc>(),
                                            &raw mut vmod,
                                        );
                                        if vmod.assign_v() != 0 {
                                            if !semip.is_null() {
                                                let l_2: size_t = p2
                                                    .offset_from(ctx.variable_buffer.ptr())
                                                    as ::core::ffi::c_long
                                                    as size_t;
                                                ::core::slice::from_raw_parts_mut(
                                                    semip as *mut u8,
                                                    1,
                                                )[0] = b';';
                                                collapse_continuations(ctx, semip);
                                                variable_buffer_output(
                                                    ctx,
                                                    p2.offset(strlen(p2) as isize),
                                                    semip,
                                                    (strlen(semip) as size_t).wrapping_add(1),
                                                );
                                                p2 = ctx.variable_buffer.ptr().add(l_2);
                                            }
                                            record_target_var(
                                                ctx,
                                                filenames.as_deref().unwrap_or(&[]),
                                                p2,
                                                (if vmod.override_v() as i32 != 0 {
                                                    o_override as i32
                                                } else {
                                                    o_file as i32
                                                })
                                                    as variable_origin,
                                                &raw mut vmod,
                                                fstart,
                                            )
                                            ?;
                                            filenames = None;
                                        } else {
                                            find_char_unquote(lb_next, '=' as i32);
                                            prefix = crate::make_main::opt_cmd_prefix(ctx);
                                            no_targets = 0;
                                            if *lb_next as i32 != 0 {
                                                let l_3: size_t = p2
                                                    .offset_from(ctx.variable_buffer.ptr())
                                                    as ::core::ffi::c_long
                                                    as size_t;
                                                expand_string_buf(
                                                    ctx,
                                                    p2.offset(plen as isize),
                                                    lb_next,
                                                    SIZE_MAX as size_t,
                                                )?;
                                                p2 = ctx.variable_buffer.ptr().add(l_3);
                                                if cmdleft.is_null() {
                                                    cmdleft = find_char_unquote(p2, ';' as i32);
                                                    if !cmdleft.is_null() {
                                                        let p2_start: usize = p2 as usize;
                                                        let p2_end: usize = p2_start
                                                            .saturating_add(strlen(p2) as usize);
                                                        let cmdleft_pos: usize = cmdleft as usize;
                                                        // NUL-terminate at the ';' before stepping
                                                        // past it, so the write goes through the
                                                        // just-null-checked pointer directly.
                                                        if cmdleft_pos >= p2_start
                                                            && cmdleft_pos < p2_end
                                                        {
                                                            let cmdleft_off: usize =
                                                                cmdleft_pos.wrapping_sub(p2_start);
                                                            let split =
                                                                p2.offset(cmdleft_off as isize);
                                                            *split = 0;
                                                            cmdleft = split.offset(1_i32 as isize);
                                                        } else {
                                                            cmdleft = ::core::ptr::null_mut::<
                                                                ::core::ffi::c_char,
                                                            >(
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                            p = strchr(p2, ':' as i32);
                                            while !p.is_null()
                                                && *p.offset(-1_i32 as isize) as i32 == '\\' as i32
                                            {
                                                let mut q: *mut ::core::ffi::c_char = p
                                                    .offset(-1_i32 as isize)
                                                    as *mut ::core::ffi::c_char;
                                                let mut backslash: i32 = 0;
                                                loop {
                                                    let fresh15 = q;
                                                    q = q.offset(-1_i32 as isize);
                                                    if !(*fresh15 as i32 == '\\' as i32) {
                                                        break;
                                                    }
                                                    backslash = (backslash == 0) as i32;
                                                }
                                                if !(backslash != 0) {
                                                    break;
                                                }
                                                p = strchr(p.offset(1_i32 as isize), ':' as i32);
                                            }
                                            if !p.is_null() {
                                                let target = parse_file_seq(
                                                    ctx,
                                                    &raw mut p2,
                                                    ::core::mem::size_of::<nameseq>() as size_t,
                                                    0x40_i32,
                                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                                    0x4_i32,
                                                )?;
                                                p2 = p2.offset(1_i32 as isize);
                                                if target.is_empty() {
                                                    return Err(fatal_err(
                                                        ctx,
                                                        fstart,
                                                        0,
                                                        b"missing target pattern\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        &[],
                                                    ));
                                                } else if target.len() > 1 {
                                                    return Err(fatal_err(
                                                        ctx,
                                                        fstart,
                                                        0,
                                                        b"multiple target patterns\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        &[],
                                                    ));
                                                }
                                                // Intern the single pattern target name so
                                                // `pattern`/`pattern_percent` remain stable
                                                // `*const c_char` for the rest of `eval`.
                                                pattern = strcache_add_bytes(ctx, &target[0].name);
                                                pattern_percent =
                                                    find_percent_cached(ctx, &raw mut pattern);
                                                if pattern_percent.is_null() {
                                                    return Err(fatal_err(
                                                        ctx,
                                                        fstart,
                                                        0,
                                                        b"target pattern contains no '%%'\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        &[],
                                                    ));
                                                }
                                            } else {
                                                pattern =
                                                    ::core::ptr::null::<::core::ffi::c_char>();
                                            }
                                            beg = p2;
                                            end = beg
                                                .offset(strlen(beg) as isize)
                                                .offset(-(1_i32 as isize));
                                            strip_whitespace(&raw mut beg, &raw mut end);
                                            if beg <= end && *beg as i32 != 0 {
                                                depstr = xstrndup(
                                                    beg,
                                                    (end.offset_from(beg) as ::core::ffi::c_long
                                                        + 1)
                                                        as size_t,
                                                );
                                            } else {
                                                depstr =
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                            }
                                            commands_idx = 0;
                                            if !cmdleft.is_null() {
                                                let l_4: size_t = strlen(cmdleft) as size_t;
                                                cmds_started =
                                                    (*fstart).lineno as ::core::ffi::c_uint;
                                                if l_4.wrapping_add(2) > commands_len {
                                                    commands_len =
                                                        l_4.wrapping_add(2).wrapping_mul(2);
                                                    cmd_buf.resize(commands_len as usize, 0);
                                                    commands = cmd_buf.as_mut_ptr()
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                memcpy(
                                                    commands as *mut ::core::ffi::c_void,
                                                    cmdleft as *const ::core::ffi::c_void,
                                                    l_4 as size_t,
                                                );
                                                commands_idx = commands_idx.wrapping_add(l_4);
                                                let fresh16 = commands_idx;
                                                commands_idx = commands_idx.wrapping_add(1);
                                                *commands.offset(fresh16 as isize) =
                                                    '\n' as i32 as ::core::ffi::c_char;
                                            }
                                            check_specials(
                                                ctx,
                                                filenames.as_deref().unwrap_or(&[]),
                                                set_default,
                                            )?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !ctx.conditionals.borrow().ignoring.is_empty() {
        return Err(fatal_err(
            ctx,
            fstart,
            0,
            b"missing 'endif'\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        ));
    }
    if filenames.is_some() {
        fi.lineno = tgts_started as ::core::ffi::c_ulong;
        fi.offset = 0;
        record_files(
            ctx,
            filenames.as_deref().unwrap_or(&[]),
            also_make_targets,
            pattern,
            pattern_percent,
            depstr,
            cmds_started,
            commands,
            commands_idx,
            two_colon,
            prefix,
            &raw mut fi,
        )
        ?;
    }
    free(collapsed as *mut ::core::ffi::c_void);
    drop(cmd_buf);
    Ok(())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn remove_comments(line: *mut ::core::ffi::c_char) {
    let comment: *mut ::core::ffi::c_char;
    comment = find_map_unquote(line, MAP_COMMENT | MAP_VARIABLE);
    if !comment.is_null() {
        *comment = 0;
    }
}
unsafe fn do_undefine(
    ctx: &crate::execctx::ExecContext,
    mut name: *mut ::core::ffi::c_char,
    origin: variable_origin,
    ebuf: *mut EBuffer,
) -> Result<(), crate::build_result::BuildError> {
    let var: *mut ::core::ffi::c_char =
        allocated_expand_string_for_file(ctx, name, ::core::ptr::null_mut::<file>())?;
    // Isolate the variable name (skip leading blanks, trim trailing blanks) via
    // the typed AST layer; an empty name is fatal.
    let span = match crate::parser::trimmed_token(::std::ffi::CStr::from_ptr(var).to_bytes()) {
        Some(s) => s,
        None => {
            return Err(crate::output::fatal_err(
                ctx,
                &raw mut (*ebuf).floc,
                0,
                b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            ))
        }
    };
    name = var.add(span.start);
    *var.add(span.end) = 0;
    undefine_variable_in_set(
        ctx,
        &raw mut (*ebuf).floc,
        name,
        (span.end - span.start) as size_t,
        origin,
        ::core::ptr::null_mut::<variable_set>(),
    )?;
    free(var as *mut ::core::ffi::c_void);
    Ok(())
}
unsafe fn do_define(
    ctx: &crate::execctx::ExecContext,
    mut name: *mut ::core::ffi::c_char,
    origin: variable_origin,
    ebuf: *mut EBuffer,
) -> Result<*mut variable, crate::build_result::BuildError> {
    let mut var: variable = variable {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        fileinfo: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut defstart: Floc;
    let mut nlevels: i32 = 1;
    let mut length: size_t = 100;
    // Owned accumulation buffer for the `define` body (was xmalloc/xrealloc/
    // free); `length` tracks the live length and `idx` the fill position as
    // before. The Vec is kept fully initialized (len == capacity) so a growth
    // preserves the already-written body rather than only the [0, len) prefix.
    let mut def_buf: Vec<u8> = vec![0u8; length as usize];
    let mut definition: *mut ::core::ffi::c_char = def_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
    let mut idx: size_t = 0;
    let mut p: *mut ::core::ffi::c_char;
    defstart = (*ebuf).floc;
    p = parse_variable_definition(ctx, name, &raw mut var);
    if p.is_null() {
        var.set_flavor(f_recursive as variable_flavor);
        var.set_conditional(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        if *var.value.offset(0_i32 as isize) as i32 != 0 {
            error(
                ctx,
                &raw mut defstart,
                0,
                b"extraneous text after 'define' directive\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        *var.name.offset(var.length as isize) = 0;
    }
    let n = allocated_expand_string_for_file(ctx, name, ::core::ptr::null_mut::<file>())?;
    // Isolate the variable name (skip leading blanks, trim trailing blanks) via
    // the typed AST layer; an empty name is fatal.
    let span = match crate::parser::trimmed_token(::std::ffi::CStr::from_ptr(n).to_bytes()) {
        Some(s) => s,
        None => {
            return Err(crate::output::fatal_err(
                ctx,
                &raw mut defstart,
                0,
                b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            ))
        }
    };
    name = n.add(span.start);
    *n.add(span.end) = 0;
    loop {
        let line: *mut ::core::ffi::c_char;
        let nlines: ::core::ffi::c_long = readline(ctx, ebuf);
        if nlines < 0 {
            return Err(crate::output::fatal_err(
                ctx,
                &raw mut defstart,
                0,
                b"missing 'endef', unterminated 'define'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            ));
        }
        (*ebuf).floc.lineno = (*ebuf)
            .floc
            .lineno
            .wrapping_add(nlines as ::core::ffi::c_ulong);
        line = (*ebuf).buffer;
        collapse_continuations(ctx, line);
        if *line.offset(0_i32 as isize) as i32 != crate::make_main::opt_cmd_prefix(ctx) as i32 {
            p = next_token(line);
            // Classify the leading `define`/`endef` keyword through the typed
            // AST layer (token delimited by a blank or NUL, matching make's
            // define-body scan), replacing the manual pointer walk.
            let (keyword, word_end) =
                crate::parser::define_keyword(::std::ffi::CStr::from_ptr(p).to_bytes());
            match keyword {
                Some(crate::parser::DefineKeyword::Define) => {
                    nlevels += 1;
                }
                Some(crate::parser::DefineKeyword::Endef) => {
                    p = p.add(word_end);
                    remove_comments(p);
                    if !crate::parser::rest_is_blank(::std::ffi::CStr::from_ptr(p).to_bytes()) {
                        error(
                            ctx,
                            &raw mut (*ebuf).floc,
                            0,
                            b"extraneous text after 'endef' directive\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    }
                    nlevels -= 1;
                    if nlevels == 0 {
                        break;
                    }
                }
                None => {}
            }
        }
        let len = strlen(line) as size_t;
        if idx.wrapping_add(len).wrapping_add(1) > length {
            length = idx.wrapping_add(len).wrapping_mul(2);
            def_buf.resize(length.wrapping_add(1) as usize, 0);
            definition = def_buf.as_mut_ptr() as *mut ::core::ffi::c_char;
        }
        memcpy(
            definition.offset(idx as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            line as *const ::core::ffi::c_void,
            len as size_t,
        );
        idx = idx.wrapping_add(len);
        let fresh30 = idx;
        idx = idx.wrapping_add(1);
        *definition.offset(fresh30 as isize) = '\n' as i32 as ::core::ffi::c_char;
    }
    if idx == 0 {
        *definition.offset(0_i32 as isize) = 0;
    } else {
        *definition.offset(idx.wrapping_sub(1) as isize) = 0;
    }
    // Held rather than `?`-ed: `n` is `malloc`ed and must be released on the
    // error path too, so the free below runs before the `BuildError` leaves
    // this frame (the cleanup-paths-report contract from #561).
    let defined = do_variable_definition(
        ctx,
        &raw mut defstart,
        name,
        definition,
        origin,
        var.flavor(),
        var.conditional() as i32,
        s_global,
    );
    free(n as *mut ::core::ffi::c_void);
    defined
}
/// Map a typed conditional [`crate::parser::Directive`] to make's internal
/// `cmdtype` code, the discriminant the rest of `conditional_line` switches on.
fn directive_cmdtype(d: crate::parser::Directive) -> C2RustUnnamed {
    use crate::parser::Directive;
    match d {
        Directive::Ifdef => c_ifdef,
        Directive::Ifndef => c_ifndef,
        Directive::Ifeq => c_ifeq,
        Directive::Ifneq => c_ifneq,
        Directive::Else => c_else,
        Directive::Endif => c_endif,
    }
}
unsafe fn conditional_line(
    ctx: &crate::execctx::ExecContext,
    mut line: *mut ::core::ffi::c_char,
    mut len: size_t,
    flocp: *const Floc,
    initial_tab: ::core::ffi::c_uint,
) -> Result<i32, crate::build_result::BuildError> {
    let cmdname: *const ::core::ffi::c_char;
    let cmdtype: C2RustUnnamed;
    // Classify the directive keyword (the line's first `len` bytes) via the
    // typed AST layer instead of a wall of `strncmp`/`size_of` comparisons.
    let directive =
        crate::parser::Directive::from_word(::core::slice::from_raw_parts(line as *const u8, len));
    match directive {
        Some(d) => {
            cmdtype = directive_cmdtype(d);
            cmdname = d.name().as_ptr();
        }
        None => return Ok(-2_i32),
    }
    if initial_tab != 0 {
        error(
            ctx,
            flocp,
            0,
            b"warning: conditional directive lines cannot start with TAB\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    line = line.offset(len as isize);
    while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
        .offset(*line as ::core::ffi::c_uchar as isize) as i32
        & (0x2_i32 | 0x4_i32)
        != 0
    {
        line = line.offset(1_i32 as isize);
    }
    if cmdtype as ::core::ffi::c_uint == c_endif as i32 as ::core::ffi::c_uint {
        if *line as i32 != 0 {
            error(
                ctx,
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous text after '%s' directive\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str((cmdname) as *const ::core::ffi::c_char)],
            );
        }
        if ctx.conditionals.borrow().ignoring.is_empty() {
            return Err(fatal_err(
                ctx,
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[FmtArg::Str((cmdname) as *const ::core::ffi::c_char)],
            ));
        }
        let mut cf = ctx.conditionals.borrow_mut();
        cf.ignoring.pop();
        cf.seen_else.pop();
    } else if cmdtype as ::core::ffi::c_uint == c_else as i32 as ::core::ffi::c_uint {
        let mut p: *const ::core::ffi::c_char;
        if ctx.conditionals.borrow().ignoring.is_empty() {
            return Err(fatal_err(
                ctx,
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[FmtArg::Str((cmdname) as *const ::core::ffi::c_char)],
            ));
        }
        let o: usize = ctx.conditionals.borrow().ignoring.len() - 1;
        if ctx.conditionals.borrow().seen_else[o] != 0 {
            return Err(fatal_err(
                ctx,
                flocp,
                0,
                b"only one 'else' per conditional\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            ));
        }
        {
            let mut cf = ctx.conditionals.borrow_mut();
            match cf.ignoring[o] {
                0 => cf.ignoring[o] = 2,
                1 => cf.ignoring[o] = 0,
                _ => {}
            }
        }
        if *line as i32 == 0 {
            ctx.conditionals.borrow_mut().seen_else[o] = 1;
        } else {
            p = line.offset(1_i32 as isize);
            while !(*(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize) as i32
                & (0x2_i32 | 0x1_i32)
                != 0)
            {
                p = p.offset(1_i32 as isize);
            }
            len = p.offset_from(line) as ::core::ffi::c_long as size_t;
            // `else <directive>` is only valid when the trailing directive is a
            // fresh conditional (`else ifeq …`); a bare `else`/`endif` after
            // `else`, or any non-directive, is extraneous text.
            let next = crate::parser::Directive::from_word(::core::slice::from_raw_parts(
                line as *const u8,
                len,
            ));
            // No `ctx.conditionals` borrow is held across this recursive call:
            // on the "open a new conditional" path it pushes its own frame
            // entry (at index `o + 1`), which the success arm below folds
            // back into `o` and pops — mirroring the former `if_cmds -= 1`.
            if matches!(
                next,
                Some(crate::parser::Directive::Else | crate::parser::Directive::Endif)
            ) || conditional_line(ctx, line, len, flocp, 0)? < 0
            {
                error(
                    ctx,
                    flocp,
                    strlen(cmdname) as size_t,
                    b"extraneous text after '%s' directive\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str((cmdname) as *const ::core::ffi::c_char)],
                );
            } else {
                let mut cf = ctx.conditionals.borrow_mut();
                if cf.ignoring[o] < 2 {
                    cf.ignoring[o] = cf.ignoring[o + 1];
                }
                cf.ignoring.pop();
                cf.seen_else.pop();
            }
        }
    } else {
        // Pushing a fresh entry replaces the manual `allocated`/`xmalloc`/
        // `xrealloc`-by-fives growth: `Vec::push` grows on demand, and
        // `o == ignoring.len() - 1 == seen_else.len() - 1` after the push
        // (the former `if_cmds` is simply this `Vec`'s length).
        let o: usize = {
            let mut cf = ctx.conditionals.borrow_mut();
            let o = cf.ignoring.len();
            cf.ignoring.push(0);
            cf.seen_else.push(0);
            o
        };
        if ctx.conditionals.borrow().ignoring[..o]
            .iter()
            .any(|&x| x != 0)
        {
            ctx.conditionals.borrow_mut().ignoring[o] = 1;
            return Ok(1);
        }
        if cmdtype as ::core::ffi::c_uint == c_ifdef as i32 as ::core::ffi::c_uint
            || cmdtype as ::core::ffi::c_uint == c_ifndef as i32 as ::core::ffi::c_uint
        {
            let v: *mut variable;
            let var: *mut ::core::ffi::c_char =
                allocated_expand_string_for_file(ctx, line, ::core::ptr::null_mut::<file>())?;
            // The condition is a single variable name: take the lone token (a
            // trailing second token is a syntax error) via the typed AST layer,
            // replacing the `end_of_token` + manual whitespace scan.
            let l: size_t =
                match crate::parser::lone_token(::std::ffi::CStr::from_ptr(var).to_bytes()) {
                    Some(l) => l as size_t,
                    None => return Ok(-1_i32),
                };
            *var.add(l) = 0;
            v = lookup_variable(ctx, var, l)?;
            ctx.conditionals.borrow_mut().ignoring[o] = ((!v.is_null() && *(*v).value as i32 != 0)
                as i32
                == (cmdtype as ::core::ffi::c_uint == c_ifndef as i32 as ::core::ffi::c_uint)
                    as i32) as u8;
            free(var as *mut ::core::ffi::c_void);
        } else {
            // The `ifeq`/`ifneq` argument forms — `(a,b)`, `"a" "b"`, `'a' 'b'`
            // — are parsed structurally (reference-aware) through the typed AST
            // layer, replacing the in-place pointer/NUL delimiter scan. Each
            // argument is NUL-terminated in place at its parsed end, expanded,
            // and the two expansions compared as owned byte strings.
            use crate::parser::ConditionalArgs;
            // Expand the argument occupying `range` (terminating it in place)
            // and return the expansion as an owned byte string.
            let expand_arg = |range: ::core::ops::Range<usize>|
             -> Result<Vec<u8>, crate::build_result::BuildError> {
                *line.add(range.end) = 0;
                let p = expand_string_buf(
                    ctx,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    line.add(range.start),
                    SIZE_MAX as size_t,
                )?;
                Ok(::std::ffi::CStr::from_ptr(p).to_bytes().to_vec())
            };
            match crate::parser::parse_conditional_args(::std::ffi::CStr::from_ptr(line).to_bytes())
            {
                ConditionalArgs::Error => return Ok(-1_i32),
                ConditionalArgs::FirstArgOnly { arg1 } => {
                    // make expands the first argument (for its side effects)
                    // before reporting the second-argument syntax error.
                    expand_arg(arg1)?;
                    return Ok(-1_i32);
                }
                ConditionalArgs::Both {
                    arg1,
                    arg2,
                    trailing_text,
                } => {
                    if trailing_text {
                        error(
                            ctx,
                            flocp,
                            strlen(cmdname) as size_t,
                            b"extraneous text after '%s' directive\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[FmtArg::Str((cmdname) as *const ::core::ffi::c_char)],
                        );
                    }
                    // Expand the first argument to an owned string before
                    // expanding the second (they share one scratch buffer).
                    let a1 = expand_arg(arg1)?;
                    let a2 = expand_arg(arg2)?;
                    ctx.conditionals.borrow_mut().ignoring[o] = ((a1 == a2)
                        == (cmdtype as ::core::ffi::c_uint
                            == c_ifneq as i32 as ::core::ffi::c_uint))
                        as u8;
                }
            }
        }
    }
    if ctx.conditionals.borrow().ignoring.iter().any(|&x| x != 0) {
        return Ok(1);
    }
    Ok(0)
}
// NOTE (slice-5 boundary): the body still drives the variable layer through
// raw `*mut File` / `*mut variable_set_list` (`initialize_file_variables`,
// `current_variable_set_list = f->variables`). Those calls do not type-check
// against the `FileId` arena yet — the per-target variable store moves in the
// final slice. The signature is converted to the owned `&[ParsedName]` so the
// reader (`eval`) compiles; the inner variable-layer references remain boundary
// errors until `variable.rs` is flipped.
unsafe fn record_target_var(
    ctx: &crate::execctx::ExecContext,
    filenames: &[ParsedName],
    defn: *mut ::core::ffi::c_char,
    origin: variable_origin,
    vmod: *mut VModifiers,
    flocp: *const Floc,
) -> Result<(), crate::build_result::BuildError> {
    let global: *mut variable_set_list = ctx.variable_globals.current_variable_set_list.get();
    for entry in filenames {
        let v: *mut variable;
        let mut name_buf = entry.name.clone();
        name_buf.push(0);
        let mut name: *const ::core::ffi::c_char = name_buf.as_ptr() as *const ::core::ffi::c_char;
        let p: *mut PatternVar;
        let percent: *const ::core::ffi::c_char = find_percent_cached(ctx, &raw mut name);
        if !percent.is_null() {
            // `create_pattern_var` stores the `target`/`suffix` pointers
            // verbatim (it does not copy), so they must point into persistent
            // storage. `name` may still point into the local `name_buf` (the
            // no-rewrite path of `find_percent_cached` does not intern), which
            // is freed at the end of this iteration — intern the name and
            // recompute the `%` offset into the cached copy so the stored
            // pattern survives until the build-phase lookup.
            let percent_off = percent.offset_from(name);
            let cached_name = strcache_add(ctx, name);
            let cached_percent = cached_name.offset(percent_off);
            p = create_pattern_var(ctx, cached_name, cached_percent);
            (*p).variable.fileinfo = *flocp;
            v = assign_variable_definition(ctx, &raw mut (*p).variable, defn)?;
            let vref = v.as_mut().expect("assertion failed: v != 0");
            vref.set_origin(origin as variable_origin);
            if vref.flavor() as i32 == f_simple as i32 {
                vref.value = allocated_expand_string_for_file(
                    ctx,
                    vref.value,
                    ::core::ptr::null_mut::<file>(),
                )?;
            } else {
                vref.value = xstrdup(vref.value);
            }
        } else {
            // Resolve (or enter) the target file. Per-target variable storage
            // now lives on the `FileNode` as a `Vec<TargetVariable>`; build a
            // transient set seeded from those, define into its head, then
            // snapshot the head back onto the node.
            let name_bytes = ::std::ffi::CStr::from_ptr(name).to_bytes();
            let fid = match lookup_file(ctx, name_bytes) {
                Some(existing) => existing,
                None => enter_file(ctx, name_bytes),
            };
            initialize_file_variables(ctx, fid, 1)?;
            // Plain `?`: the head is not installed until the next line, so a
            // rejection here leaves the globals still current.
            let head = crate::variable::build_file_setlist(ctx, fid)?;
            ctx.variable_globals.current_variable_set_list.set(head);
            v = try_variable_definition(ctx, flocp, defn, origin, s_target)?;
            if v.is_null() {
                return Err(crate::output::fatal_err(
                    ctx,
                    flocp,
                    0,
                    b"malformed target-specific variable definition\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                ));
            }
            let vref = v.as_mut().expect("record_target_var: null variable");
            vref.set_per_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            vref.set_private_var((*vmod).private_v() as ::core::ffi::c_uint);
            if (*vmod).export_v() as i32 != v_default as i32 {
                vref.set_export((*vmod).export_v() as variable_export);
            }
            if vref.origin() as i32 != o_override as i32 {
                let len: size_t = strlen(vref.name) as size_t;
                // The global lookup must search the underlying global set, not
                // the per-file head we just installed.
                ctx.variable_globals.current_variable_set_list.set(global);
                let looked = lookup_variable(ctx, vref.name, len);
                ctx.variable_globals.current_variable_set_list.set(head);
                // The set list is swapped back before the rejection escapes, so
                // a rejected reference check cannot leave the per-file head
                // installed over the globals (#561).
                let gv: *mut variable = looked?;
                if !gv.is_null()
                    && v != gv
                    && ((*gv).origin() as i32 == o_env_override as i32
                        || (*gv).origin() as i32 == o_command as i32)
                {
                    free(vref.value as *mut ::core::ffi::c_void);
                    vref.value = xstrdup((*gv).value);
                    vref.set_origin((*gv).origin() as variable_origin);
                    vref.set_recursive((*gv).recursive() as ::core::ffi::c_uint);
                    vref.set_append(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            // Snapshot the head set's variables back onto the file node.
            let snapshot = crate::variable::snapshot_set_to_targets((*head).set);
            if let Some(node) = ctx.filenodes.get(fid) {
                node.lock().expect("file node poisoned").variables = snapshot;
            }
            ctx.variable_globals.current_variable_set_list.set(global);
            crate::variable::free_file_setlist(ctx, head);
            continue;
        }
        // Pattern-variable branch: the variable was defined on a `PatternVar`,
        // not a file, so finalize its flags directly.
        let vref = v.as_mut().expect("record_target_var: null variable");
        vref.set_per_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        vref.set_private_var((*vmod).private_v() as ::core::ffi::c_uint);
        if (*vmod).export_v() as i32 != v_default as i32 {
            vref.set_export((*vmod).export_v() as variable_export);
        }
        if vref.origin() as i32 != o_override as i32 {
            let len: size_t = strlen(vref.name) as size_t;
            let gv: *mut variable = lookup_variable(ctx, vref.name, len)?;
            if !gv.is_null()
                && v != gv
                && ((*gv).origin() as i32 == o_env_override as i32
                    || (*gv).origin() as i32 == o_command as i32)
            {
                free(vref.value as *mut ::core::ffi::c_void);
                vref.value = xstrdup((*gv).value);
                vref.set_origin((*gv).origin() as variable_origin);
                vref.set_recursive((*gv).recursive() as ::core::ffi::c_uint);
                vref.set_append(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
    Ok(())
}
/// The name of a dependency as owned bytes — the idiomatic [`DepNode`] keeps its
/// `name: String` populated (the resolver no longer nulls it), so this is just
/// the name's bytes. Used by the suffix-rule check in [`check_specials`].
fn dep_name_bytes(dp: &crate::dep::DepNode) -> Vec<u8> {
    dp.name.clone().into_bytes()
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn check_specials(
    ctx: &crate::execctx::ExecContext,
    files: &[ParsedName],
    set_default: i32,
) -> Result<(), crate::build_result::BuildError> {
    for entry in files {
        // NUL-terminated name for the C variable-layer calls below.
        let mut nm_buf = entry.name.clone();
        nm_buf.push(0);
        let nm: *const ::core::ffi::c_char = nm_buf.as_ptr() as *const ::core::ffi::c_char;
        let special = crate::parser::SpecialTarget::from_name(&entry.name);
        if !posix_pedantic(ctx) && special == Some(crate::parser::SpecialTarget::Posix) {
            crate::make_main::set_posix_pedantic(ctx);
            crate::variable::define_named(
            ctx,
            b".SHELLFLAGS\0",
            b"-ec\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
        )?;
            crate::variable::define_named(
            ctx,
            b"CC\0",
            b"c99\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
        )?;
            crate::variable::define_named(
            ctx,
            b"CFLAGS\0",
            b"-O1\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
        )?;
            crate::variable::define_named(
            ctx,
            b"FC\0",
            b"fort77\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
        )?;
            crate::variable::define_named(
            ctx,
            b"FFLAGS\0",
            b"-O1\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
        )?;
            crate::variable::define_named(
            ctx,
            b"SCCSGETFLAGS\0",
            b"-s\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
        )?;
            crate::variable::define_named(
            ctx,
            b"ARFLAGS\0",
            b"-rv\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
        )?;
        } else if !second_expansion(ctx)
            && special == Some(crate::parser::SpecialTarget::SecondExpansion)
        {
            crate::make_main::set_second_expansion(ctx);
        } else if !one_shell(ctx) && special == Some(crate::parser::SpecialTarget::OneShell) {
            crate::make_main::set_one_shell(ctx);
        } else if set_default != 0 && *(*ctx.default_goal_var.0.get()).value.offset(0) as i32 == 0 {
            let mut reject = false;
            // Pattern targets (containing `%`) are never the default goal.
            let nm_bytes = entry.name.as_slice();
            if nm_bytes.contains(&b'%') {
                break;
            }
            if !(nm_bytes.first() == Some(&b'.') && !nm_bytes.contains(&b'/')) {
                // Snapshot `.SUFFIXES`'s prerequisite names from the arena (the
                // former `suffix_file->deps` chain; `suffix_file` is a rule.rs
                // `*mut File` global in the c2rust graph). No guard is held while
                // we test names.
                let suffix_deps: Vec<Vec<u8>> = match lookup_file(ctx, b".SUFFIXES") {
                    Some(sid) => match ctx.filenodes.get(sid) {
                        Some(node) => node
                            .lock()
                            .expect("file node lock poisoned")
                            .deps
                            .iter()
                            .map(dep_name_bytes)
                            .collect(),
                        None => Vec::new(),
                    },
                    None => Vec::new(),
                };
                'outer: for dname in &suffix_deps {
                    // A target is a suffix rule (and so must not become the
                    // default goal) when its name is itself a known suffix, or
                    // the concatenation of two known suffixes (e.g. `.c` + `.o`
                    // => `.c.o`).
                    if dname.first() != Some(&b'.') && nm_bytes == dname.as_slice() {
                        reject = true;
                        break;
                    }
                    for d2 in &suffix_deps {
                        if nm_bytes.strip_prefix(d2.as_slice()) == Some(dname.as_slice()) {
                            reject = true;
                            break 'outer;
                        }
                    }
                }
                if !reject {
                    define_variable_in_set(
                        ctx,
                        b".DEFAULT_GOAL\0" as *const u8 as *const ::core::ffi::c_char,
                        13,
                        nm,
                        o_file,
                        0,
                        ::core::ptr::null_mut::<variable_set>(),
                        ::core::ptr::null_mut::<Floc>(),
                    )?;
                }
            }
        }
    }
    Ok(())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn check_special_file(
    ctx: &crate::execctx::ExecContext,
    file: FileId,
    flocp: *const Floc,
) {
    let (is_wait, has_deps, has_recipe) = {
        let node = match ctx.filenodes.get(file) {
            Some(n) => n,
            None => return,
        };
        let n = node.lock().expect("file node lock poisoned");
        (
            crate::parser::is_wait_token(&n.name),
            !n.deps.is_empty(),
            n.recipe.is_some(),
        )
    };
    if is_wait {
        use std::sync::atomic::Ordering;
        if !ctx.wpre_warned.0.load(Ordering::Relaxed) && has_deps {
            error(
                ctx,
                flocp,
                0,
                b".WAIT should not have prerequisites\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
            ctx.wpre_warned.0.store(true, Ordering::Relaxed);
        }
        if !ctx.wcmd_warned.0.load(Ordering::Relaxed) && has_recipe {
            error(
                ctx,
                flocp,
                0,
                b".WAIT should not have commands\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
            ctx.wcmd_warned.0.store(true, Ordering::Relaxed);
        }
    }
}
/// Parse a prerequisite string into an owned `Vec<DepNode>` — the pointer-free
/// port of `split_prereqs` (`file.rs`). Normal prerequisites come before a `|`;
/// order-only prerequisites follow it and are marked `ignore_mtime`. Names are
/// produced by the new [`parse_file_seq`] (no intrusive chain).
///
/// # Safety
/// `p` must point at a live, NUL-terminated, writable buffer.
unsafe fn split_prereqs_vec(
    ctx: &crate::execctx::ExecContext,
    mut p: *mut ::core::ffi::c_char,
) -> Result<Vec<crate::dep::DepNode>, crate::build_result::BuildError> {
    // 0x100 = PARSEFS_NOSTRIP, 0x40 = PARSEFS_WAIT (recognise `.WAIT`).
    let names = parse_file_seq(
        ctx,
        &raw mut p,
        ::core::mem::size_of::<dep>() as size_t,
        0x100_i32,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x40_i32,
    )?;
    let mut deps: Vec<crate::dep::DepNode> = names
        .into_iter()
        .map(|n| dep_from_name(n.name, n.wait, false))
        .collect();
    if p.as_ref().is_some_and(|c| *c != 0) {
        p = p.offset(1_i32 as isize);
        let ood_names = parse_file_seq(
            ctx,
            &raw mut p,
            ::core::mem::size_of::<dep>() as size_t,
            0x1_i32,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0x40_i32,
        )?;
        for n in ood_names {
            let mut d = dep_from_name(n.name, n.wait, false);
            d.ignore_mtime = true;
            deps.push(d);
        }
    }
    Ok(deps)
}

/// Build a fresh [`DepNode`] from an owned prerequisite name plus its `.WAIT`
/// marker. `static_pattern` is the initial static-pattern flag.
fn dep_from_name(name: Vec<u8>, wait: bool, static_pattern: bool) -> crate::dep::DepNode {
    let mut d = crate::dep::DepNode::default();
    d.name = String::from_utf8_lossy(&name).into_owned();
    d.wait_here = wait;
    d.static_pattern = static_pattern;
    d
}

/// Resolve each non-second-expansion prerequisite to a [`FileId`] and apply
/// stem-based `%` substitution — the pointer-free port of `enter_prereqs`
/// (`file.rs`). Prereqs that expand to empty under a non-empty stem are dropped.
/// When `stem` is `None` (no static pattern), resolved targets are marked
/// `is_explicit`.
///
/// Locking discipline: target lookup (`enter_file`) is done with no `FileNode`
/// guard held — the dep `Vec` is owned locally and each name is resolved
/// separately.
unsafe fn enter_prereqs_vec(
    ctx: &crate::execctx::ExecContext,
    mut deps: Vec<crate::dep::DepNode>,
    stem: Option<&[u8]>,
) -> Vec<crate::dep::DepNode> {
    if deps.is_empty() {
        return deps;
    }
    if let Some(stem_bytes) = stem {
        let pattern: *const ::core::ffi::c_char = b"%\0" as *const u8 as *const ::core::ffi::c_char;
        let mut kept: Vec<crate::dep::DepNode> = Vec::with_capacity(deps.len());
        for mut d in deps.into_iter() {
            if d.needs_second_expansion {
                kept.push(d);
                continue;
            }
            // Mutable, NUL-terminated copy of the name for the in-place
            // `find_percent` rewrite.
            let mut nm: Vec<u8> = d.name.clone().into_bytes();
            nm.push(0);
            let nm_ptr = nm.as_mut_ptr() as *mut ::core::ffi::c_char;
            let percent = find_percent(nm_ptr);
            if !percent.is_null() {
                // NUL-terminated stem for the C patsubst/expand helpers.
                let mut stem_c: Vec<u8> = stem_bytes.to_vec();
                stem_c.push(0);
                let o: *mut ::core::ffi::c_char;
                if stem_bytes.is_empty() {
                    memmove(
                        percent as *mut ::core::ffi::c_void,
                        percent.offset(1_i32 as isize) as *const ::core::ffi::c_void,
                        strlen(percent),
                    );
                    o = variable_buffer_output(
                        ctx,
                        ctx.variable_buffer.ptr(),
                        nm_ptr,
                        (strlen(nm_ptr) as size_t).wrapping_add(1),
                    );
                } else {
                    o = patsubst_expand_pat(
                        ctx,
                        ctx.variable_buffer.ptr(),
                        stem_c.as_ptr() as *const ::core::ffi::c_char,
                        pattern,
                        nm_ptr,
                        pattern.offset(1_i32 as isize),
                        percent.offset(1_i32 as isize),
                    );
                }
                if *ctx.variable_buffer.ptr().offset(0_i32 as isize) as i32 == 0 {
                    // Expanded to nothing: drop this prerequisite.
                    continue;
                } else {
                    let result = ::core::slice::from_raw_parts(
                        ctx.variable_buffer.ptr() as *const u8,
                        o.offset_from(ctx.variable_buffer.ptr()) as usize,
                    );
                    d.name = String::from_utf8_lossy(result).into_owned();
                }
            }
            d.stem = Some(String::from_utf8_lossy(stem_bytes).into_owned());
            d.static_pattern = true;
            kept.push(d);
        }
        deps = kept;
    }
    // Resolve targets to FileIds for non-second-expansion deps.
    for d in deps.iter_mut() {
        if !d.needs_second_expansion {
            let name_bytes = d.name.clone().into_bytes();
            let fid = lookup_file(ctx, &name_bytes).unwrap_or_else(|| enter_file(ctx, &name_bytes));
            d.file = Some(fid);
            d.static_pattern = false;
            // The c2rust graph nulled `name` once `file` was resolved; we keep
            // `name` (cheap owned String) for diagnostics and dep_name parity.
            if stem.is_none() {
                if let Some(node) = ctx.filenodes.get(fid) {
                    node.lock().expect("file node lock poisoned").is_explicit = true;
                }
            }
        }
    }
    deps
}

#[allow(clippy::too_many_arguments)]
unsafe fn record_files(
    ctx: &crate::execctx::ExecContext,
    filenames: &[ParsedName],
    are_also_makes: i32,
    pattern: *const ::core::ffi::c_char,
    pattern_percent: *const ::core::ffi::c_char,
    mut depstr: *mut ::core::ffi::c_char,
    cmds_started: ::core::ffi::c_uint,
    commands: *mut ::core::ffi::c_char,
    commands_idx: size_t,
    two_colon: i32,
    prefix: ::core::ffi::c_char,
    flocp: *const Floc,
) -> Result<(), crate::build_result::BuildError> {
    if opt_snapped_deps(ctx) {
        return Err(crate::output::fatal_err(
            ctx,
            flocp,
            0,
            b"prerequisites cannot be defined in recipes\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        ));
    }
    debug_assert!(!filenames.is_empty(), "record_files: empty filenames");
    // The first target's name; `%` detection rewrites a cached copy in place.
    let mut name_buf: Vec<u8> = {
        let mut v = filenames[0].name.clone();
        v.push(0);
        v
    };
    let mut name: *const ::core::ffi::c_char = name_buf.as_ptr() as *const ::core::ffi::c_char;
    let implicit_percent: *const ::core::ffi::c_char = find_percent_cached(ctx, &raw mut name);

    // Build the recipe (the former `*mut Commands`) as an idiomatic `Recipe`.
    let recipe: Option<crate::recipe::Recipe> = if commands_idx > 0 {
        let text =
            ::core::slice::from_raw_parts(commands as *const u8, commands_idx as usize).to_vec();
        let (defined_in, _ln, _off) = floc_owned(flocp);
        Some(crate::recipe::Recipe {
            defined_in,
            defined_lineno: cmds_started as u64,
            text,
            lines: Vec::new(),
            recipe_prefix: prefix as u8,
            any_recurse: false,
        })
    } else if are_also_makes != 0 {
        return Err(crate::output::fatal_err(
            ctx,
            flocp,
            0,
            b"grouped targets must provide a recipe\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        ));
    } else {
        None
    };
    let have_cmds = recipe.is_some();

    // Build the prerequisite list as an owned `Vec<DepNode>`.
    let mut deps: Vec<crate::dep::DepNode> = Vec::new();
    if !depstr.is_null() {
        depstr = unescape_char(depstr, ':' as i32);
        if second_expansion(ctx)
            && crate::parser::prereq_needs_second_expansion(
                ::std::ffi::CStr::from_ptr(depstr).to_bytes(),
            )
        {
            let d = crate::dep::DepNode {
                name: String::from_utf8_lossy(::std::ffi::CStr::from_ptr(depstr).to_bytes())
                    .into_owned(),
                needs_second_expansion: true,
                static_pattern: !pattern.is_null(),
                ..Default::default()
            };
            deps.push(d);
            free(depstr as *mut ::core::ffi::c_void);
        } else {
            // The `depstr` buffer is released before a rejected `~` expansion
            // leaves the frame.
            let split = split_prereqs_vec(ctx, depstr);
            free(depstr as *mut ::core::ffi::c_void);
            deps = split?;
            if pattern.is_null() && implicit_percent.is_null() {
                deps = enter_prereqs_vec(ctx, deps, None);
            }
        }
    }

    if !implicit_percent.is_null() {
        // Implicit / pattern rule. Collect each target's owned pattern bytes and
        // the byte index of its `%`, then hand them to the pointer-free
        // `create_pattern_rule`.
        if !pattern.is_null() {
            return Err(crate::output::fatal_err(
                ctx,
                flocp,
                0,
                b"mixed implicit and static pattern rules\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            ));
        }
        let mut targets: Vec<Vec<u8>> = Vec::with_capacity(filenames.len());
        let mut percents: Vec<usize> = Vec::with_capacity(filenames.len());
        // The first target's bytes/percent come from `name`/`implicit_percent`.
        let first_name = ::std::ffi::CStr::from_ptr(name).to_bytes().to_vec();
        let first_pct = implicit_percent.offset_from(name) as usize;
        targets.push(first_name);
        percents.push(first_pct);
        for entry in &filenames[1..] {
            let mut nb = entry.name.clone();
            nb.push(0);
            let mut np: *const ::core::ffi::c_char =
                strcache_add(ctx, nb.as_ptr() as *const ::core::ffi::c_char);
            let ip = find_percent_cached(ctx, &raw mut np);
            if ip.is_null() {
                return Err(crate::output::fatal_err(
                    ctx,
                    flocp,
                    0,
                    b"mixed implicit and normal rules\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                ));
            }
            targets.push(::std::ffi::CStr::from_ptr(np).to_bytes().to_vec());
            percents.push(ip.offset_from(np) as usize);
        }
        let n = targets.len() as u16;
        create_pattern_rule(
            ctx,
            targets,
            percents,
            n,
            two_colon != 0,
            deps,
            recipe,
            true,
        );
        return Ok(());
    }

    // also_make group members, collected as (FileId) of each grouped target.
    let mut also_make_ids: Vec<FileId> = Vec::new();

    let mut idx = 0usize;
    loop {
        let is_last = idx + 1 >= filenames.len();
        // `this` is the per-target prerequisite list (cloned for all but the last).
        let mut this: Vec<crate::dep::DepNode> =
            if !pattern.is_null() && pattern_matches(pattern, pattern_percent, name) == 0 {
                error(
                    ctx,
                    flocp,
                    strlen(name) as size_t,
                    b"target '%s' doesn't match the target pattern\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str((name) as *const ::core::ffi::c_char)],
                );
                Vec::new()
            } else if !deps.is_empty() {
                if is_last {
                    ::core::mem::take(&mut deps)
                } else {
                    deps.clone()
                }
            } else {
                Vec::new()
            };

        let name_bytes = ::std::ffi::CStr::from_ptr(name).to_bytes().to_vec();
        let f: FileId;
        // For a `::` target, which inline entry this rule's deps/recipe/flags
        // belong to: `None` = the head (the first `::` rule), `Some(i)` = the
        // i-th appended `double_colon` entry (a subsequent rule). For a single
        // colon target this stays `None` (the head).
        let mut dc_index: Option<usize> = None;
        if two_colon == 0 {
            f = enter_file(ctx, &name_bytes);
            // Diagnostics + recipe/dep merge, under the node lock (no arena
            // re-entry while held).
            let node = ctx.filenodes.get(f).expect("record_files: missing target");
            {
                let mut n = node.lock().expect("file node lock poisoned");
                if n.is_double_colon {
                    let nm = n.name.clone();
                    // Formerly `fatal` (diverges/aborts) with the lock released by
                    // unwind; `fatal_err` returns instead, so the early `return`
                    // below drops `n` (and `node`) normally on the way out.
                    return Err(crate::output::fatal_err(
                        ctx,
                        flocp,
                        nm.len() as size_t,
                        b"target file '%s' has both : and :: entries\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(
                            strcache_add_bytes(ctx, &nm) as *const ::core::ffi::c_char
                        )],
                    ));
                }
                if have_cmds && n.recipe.is_some() && n.is_target {
                    let nm = n.name.clone();
                    let nptr = strcache_add_bytes(ctx, &nm) as *const ::core::ffi::c_char;
                    drop(n);
                    let l = nm.len() as size_t;
                    error(
                        ctx,
                        flocp,
                        l,
                        b"warning: overriding recipe for target '%s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(nptr)],
                    );
                    error(
                        ctx,
                        flocp,
                        l,
                        b"warning: ignoring old recipe for target '%s'\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(nptr)],
                    );
                    n = node.lock().expect("file node lock poisoned");
                }
                // `default_file`/`suffix_file` are rule.rs globals (still `*mut File`,
                // slice 5). The c2rust special-cased them by pointer identity; with
                // the arena keyed by name we compare names instead.
                if is_suffix_file(&n.name) && this.is_empty() {
                    n.deps.clear();
                }
                if let Some(r) = recipe.clone() {
                    n.recipe = Some(r);
                } else if is_default_file(&n.name) && this.is_empty() {
                    n.recipe = None;
                }
            }
        } else {
            // Double-colon.
            if let Some(existing) = lookup_file(ctx, &name_bytes) {
                let node = ctx
                    .filenodes
                    .get(existing)
                    .expect("record_files: missing dcolon");
                let n = node.lock().expect("file node lock poisoned");
                if n.is_target && !n.is_double_colon {
                    let nm = n.name.clone();
                    drop(n);
                    return Err(crate::output::fatal_err(
                        ctx,
                        flocp,
                        nm.len() as size_t,
                        b"target file '%s' has both : and :: entries\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(
                            strcache_add_bytes(ctx, &nm) as *const ::core::ffi::c_char
                        )],
                    ));
                }
            }
            // Was this target already a `::` head before this rule? If so,
            // `enter_file` will append a fresh inline entry for this rule; the
            // first `::` rule instead lives on the head itself.
            let was_double_colon = lookup_file(ctx, &name_bytes)
                .and_then(|existing| ctx.filenodes.get(existing))
                .map(|n| n.lock().expect("file node lock poisoned").is_double_colon)
                .unwrap_or(false);
            // `enter_file` appends a new inline `double_colon` entry when the
            // head is already a `::` target (see its doc); do NOT also call
            // `push_double_colon_entry` or the entry would be appended twice.
            f = enter_file(ctx, &name_bytes);
            {
                let node = ctx.filenodes.get(f).expect("record_files: missing dcolon");
                let mut n = node.lock().expect("file node lock poisoned");
                if was_double_colon {
                    // `enter_file` just appended this rule's entry.
                    dc_index = Some(n.double_colon.len() - 1);
                } else {
                    // First `::` rule: it lives on the head.
                    n.is_double_colon = true;
                    dc_index = None;
                }
            }
            if let Some(r) = recipe.clone() {
                let node = ctx.filenodes.get(f).expect("record_files: missing dcolon");
                let mut n = node.lock().expect("file node lock poisoned");
                match dc_index {
                    Some(i) => n.double_colon[i].recipe = Some(r),
                    None => n.recipe = Some(r),
                }
            }
        }

        // Mark target, set stem from the static pattern, resolve `this`.
        {
            let node = ctx.filenodes.get(f).expect("record_files: missing target");
            let mut n = node.lock().expect("file node lock poisoned");
            // The head always carries `is_target`/`is_explicit`; a subsequent
            // `::` entry mirrors them on itself too.
            n.is_explicit = true;
            n.is_target = true;
            if let Some(i) = dc_index {
                n.double_colon[i].is_explicit = true;
                n.double_colon[i].is_target = true;
            }
        }
        if are_also_makes != 0 {
            also_make_ids.push(f);
        }
        // Static pattern stem.
        let mut stem_bytes: Option<Vec<u8>> = None;
        if !pattern.is_null() {
            let percent: *const ::core::ffi::c_char =
                b"%\0" as *const u8 as *const ::core::ffi::c_char;
            let o: *mut ::core::ffi::c_char = patsubst_expand_pat(
                ctx,
                ctx.variable_buffer.ptr(),
                name,
                pattern,
                percent,
                pattern_percent.offset(1_i32 as isize),
                percent.offset(1_i32 as isize),
            );
            let stem = ::core::slice::from_raw_parts(
                ctx.variable_buffer.ptr() as *const u8,
                o.offset_from(ctx.variable_buffer.ptr()) as usize,
            )
            .to_vec();
            {
                let node = ctx.filenodes.get(f).expect("record_files: missing target");
                let mut n = node.lock().expect("file node lock poisoned");
                let s = Some(String::from_utf8_lossy(&stem).into_owned());
                match dc_index {
                    Some(i) => n.double_colon[i].stem = s,
                    None => n.stem = s,
                }
            }
            // Apply the stem to `this` (static-pattern prereqs).
            if !this.is_empty() {
                if this.iter().any(|d| d.needs_second_expansion) {
                    for d in this.iter_mut() {
                        d.stem = Some(String::from_utf8_lossy(&stem).into_owned());
                    }
                } else {
                    this = enter_prereqs_vec(ctx, this, Some(&stem));
                }
            }
            stem_bytes = Some(stem);
        }
        let _ = &stem_bytes;

        // Attach `this` to the target's deps (mirrors the c2rust ordering: with a
        // recipe present, the new prereqs go in front; otherwise appended).
        if !this.is_empty() {
            let node = ctx.filenodes.get(f).expect("record_files: missing target");
            let mut n = node.lock().expect("file node lock poisoned");
            let deps_slot: &mut Vec<crate::dep::DepNode> = match dc_index {
                Some(i) => &mut n.double_colon[i].deps,
                None => &mut n.deps,
            };
            if deps_slot.is_empty() {
                *deps_slot = this;
            } else if have_cmds {
                let mut combined = this;
                combined.append(deps_slot);
                *deps_slot = combined;
            } else {
                deps_slot.append(&mut this);
            }
        }
        check_special_file(ctx, f, flocp);

        if is_last {
            break;
        }
        idx += 1;
        name_buf = {
            let mut v = filenames[idx].name.clone();
            v.push(0);
            v
        };
        name = name_buf.as_ptr() as *const ::core::ffi::c_char;
        if !find_percent_cached(ctx, &raw mut name).is_null() {
            error(
                ctx,
                flocp,
                0,
                b"*** mixed implicit and normal rules: deprecated syntax\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
    }

    // also_make: every grouped target gets the others as `also_make` siblings.
    for &fid in &also_make_ids {
        // Sibling dep edges = all grouped targets except this one.
        let siblings: Vec<crate::dep::DepNode> = also_make_ids
            .iter()
            .filter(|&&o| o != fid)
            .map(|&o| {
                let mut d = crate::dep::DepNode::default();
                if let Some(node) = ctx.filenodes.get(o) {
                    d.name = String::from_utf8_lossy(
                        &node.lock().expect("file node lock poisoned").name,
                    )
                    .into_owned();
                }
                d.file = Some(o);
                d
            })
            .collect();
        let node = ctx
            .filenodes
            .get(fid)
            .expect("record_files: missing group target");
        let mut n = node.lock().expect("file node lock poisoned");
        if !n.also_make.is_empty() {
            let nm = n.name.clone();
            drop(n);
            error(
                ctx,
                flocp,
                nm.len() as size_t,
                b"warning: overriding group membership for target '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str(
                    strcache_add_bytes(ctx, &nm) as *const ::core::ffi::c_char
                )],
            );
            n = node.lock().expect("file node lock poisoned");
            n.also_make.clear();
        }
        n.also_make = siblings;
    }
    Ok(())
}

/// Whether `name` is the special `.SUFFIXES` suffix file (the c2rust
/// `suffix_file` pointer-identity test, by name). `suffix_file` itself lives in
/// rule.rs (slice 5).
fn is_suffix_file(name: &[u8]) -> bool {
    name == b".SUFFIXES"
}

/// Whether `name` is the special `.DEFAULT` file (the c2rust `default_file`
/// pointer-identity test, by name).
fn is_default_file(name: &[u8]) -> bool {
    name == b".DEFAULT"
}
/// Read the byte at a C-string cursor through a bounds-checked one-element
/// slice instead of a raw `*p` dereference. The cursors handled here come from
/// `find_map_unquote` (whose in-place rewrite makes the dataflow engine treat
/// the returned pointer as possibly-invalid); going through a slice keeps the
/// access checked and out of the raw-dereference sink.
///
/// # Safety
///
/// `p` must point at a readable byte (it always does here: a position within a
/// live, NUL-terminated buffer).
#[inline]
unsafe fn cbyte(p: *const ::core::ffi::c_char) -> u8 {
    ::core::slice::from_raw_parts(p as *const u8, 1)[0]
}
unsafe extern "C" fn find_map_unquote(
    string: *mut ::core::ffi::c_char,
    stopmap: i32,
) -> *mut ::core::ffi::c_char {
    // Bridge to the pure parser routine over a byte slice covering the C string
    // plus its NUL, instead of the c2rust stopchar_map/memmove pointer walk.
    let len = ::std::ffi::CStr::from_ptr(string).to_bytes().len();
    let buf = ::core::slice::from_raw_parts_mut(string as *mut u8, len + 1);
    match crate::parser::find_map_unquote_idx(buf, stopmap) {
        Some(i) => buf[i..].as_mut_ptr() as *mut ::core::ffi::c_char,
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}
unsafe extern "C" fn find_char_unquote(
    string: *mut ::core::ffi::c_char,
    stop: i32,
) -> *mut ::core::ffi::c_char {
    // Bridge to the pure parser routine over a byte slice covering the C string
    // plus its NUL terminator, instead of the c2rust strchr/strlen/memmove walk.
    // No raw pointer arithmetic: the length comes from `CStr` and the result
    // sub-pointer from `slice[i..].as_mut_ptr()` (per AGENTS.md).
    let len = ::std::ffi::CStr::from_ptr(string).to_bytes().len();
    let buf = ::core::slice::from_raw_parts_mut(string as *mut u8, len + 1);
    match crate::parser::find_char_unquote_idx(buf, stop as u8) {
        Some(i) => buf[i..].as_mut_ptr() as *mut ::core::ffi::c_char,
        None => ::core::ptr::null_mut::<::core::ffi::c_char>(),
    }
}
unsafe extern "C" fn unescape_char(
    string: *mut ::core::ffi::c_char,
    c: i32,
) -> *mut ::core::ffi::c_char {
    let len = ::std::ffi::CStr::from_ptr(string).to_bytes().len();
    // The pure routine reads the bytes and returns the (never longer)
    // unescaped result; write it back in place over the original buffer.
    let out = crate::parser::unescape_char(::std::ffi::CStr::from_ptr(string).to_bytes(), c as u8);
    let buf = ::core::slice::from_raw_parts_mut(string as *mut u8, len + 1);
    buf[..out.len()].copy_from_slice(&out);
    buf[out.len()] = 0;
    string
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn find_percent(pattern: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    find_char_unquote(pattern, '%' as i32)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn find_percent_cached(
    ctx: &crate::execctx::ExecContext,
    string: *mut *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let s = ::std::ffi::CStr::from_ptr(*string).to_bytes();
    match crate::parser::find_percent_cached(s) {
        // No rewrite: the `%` (if any) is returned as a pointer into the
        // unchanged interned string.
        crate::parser::FindPercentCached::AsIs(None) => ::core::ptr::null::<::core::ffi::c_char>(),
        crate::parser::FindPercentCached::AsIs(Some(i)) => {
            ::core::slice::from_raw_parts(*string as *const u8, s.len())[i..].as_ptr()
                as *const ::core::ffi::c_char
        }
        // Escaped `%`: intern the collapsed copy, update the caller's pointer,
        // and map the percent index back into the interned string.
        crate::parser::FindPercentCached::Collapsed { buf, idx } => {
            let cached = strcache_add(ctx, buf.as_ptr() as *const ::core::ffi::c_char);
            *string = cached;
            match idx {
                Some(i) => {
                    ::core::slice::from_raw_parts(cached as *const u8, strlen(cached) as size_t)
                        [i..]
                        .as_ptr() as *const ::core::ffi::c_char
                }
                None => ::core::ptr::null::<::core::ffi::c_char>(),
            }
        }
    }
}
unsafe extern "C" fn get_next_mword(
    buffer: *mut ::core::ffi::c_char,
    startp: *mut *mut ::core::ffi::c_char,
    length: *mut size_t,
) -> make_word_type {
    let bytes = ::std::ffi::CStr::from_ptr(buffer).to_bytes();
    let total = bytes.len() + 1;
    let (wtype, beg, len) = crate::parser::get_next_mword(bytes);
    if !startp.is_null() {
        // Sub-pointer into the buffer at the word start (no raw arithmetic).
        *startp = ::core::slice::from_raw_parts_mut(buffer as *mut u8, total)[beg..].as_mut_ptr()
            as *mut ::core::ffi::c_char;
    }
    if !length.is_null() {
        *length = len as size_t;
    }
    match wtype {
        crate::parser::MWordType::Eol => w_eol,
        crate::parser::MWordType::Static => w_static,
        crate::parser::MWordType::Variable => w_variable,
        crate::parser::MWordType::Colon => w_colon,
        crate::parser::MWordType::DColon => w_dcolon,
        crate::parser::MWordType::Semicolon => w_semicolon,
        crate::parser::MWordType::AmpColon => w_ampcolon,
        crate::parser::MWordType::AmpDColon => w_ampdcolon,
    }
}
/// One name produced by [`parse_file_seq`]: the resolved (glob-/archive-/tilde-
/// expanded, prefixed, optionally cache-interned) token bytes, plus whether a
/// preceding `.WAIT` marker applied to it. This is the pointer-free replacement
/// for the intrusive `*mut Dep`/`*mut GoalDep`/`*mut NameSeq` nodes the c2rust
/// parser threaded together: the caller turns each entry into the graph node it
/// wants (a [`DepNode`], a [`GoalDepNode`], or a plain name).
///
/// `name` carries no trailing NUL (it is the observable string bytes); a
/// `.WAIT` token never appears as an entry of its own — it sets `wait` on the
/// next real name, mirroring make's `dep->wait_here`.
pub struct ParsedName {
    pub name: Vec<u8>,
    pub wait: bool,
}

/// Parse a whitespace-separated file sequence into an owned `Vec<ParsedName>`,
/// the pointer-free keystone replacing the c2rust intrusive-chain
/// `parse_file_seq`. Globbing, archive-member (`lib(member)`) expansion, tilde
/// expansion, prefix application and strcache interning all still happen; the
/// result is collected into owned name byte-vectors instead of a `*mut T`
/// chain. `*stringp` is advanced past the consumed text exactly as before.
///
/// `cache` (the former `!PARSEFS_NOCACHE`) chose between `strcache_add` and
/// `xstrdup` for chain ownership; since each name is now copied into an owned
/// `Vec<u8>` regardless, that part of it no longer changes the result. It does
/// still select one behaviour: in the archive-member branch, `cachep` with no
/// `prefix` is the one case where read.c leaves `found->name` alone instead of
/// overwriting it with the archive name (see the loop below and #460).
///
/// # Safety
///
/// `*stringp` must point at a live, NUL-terminated, writable buffer (this parser
/// rewrites it in place while unquoting); `prefix` must be null or a valid C
/// string. All other pointer use is internal to the call.
pub unsafe fn parse_file_seq(
    ctx: &crate::execctx::ExecContext,
    stringp: *mut *mut ::core::ffi::c_char,
    _size: size_t,
    mut stopmap: i32,
    prefix: *const ::core::ffi::c_char,
    flags: i32,
) -> Result<Vec<ParsedName>, crate::build_result::BuildError> {
    let cachep: i32 = !(flags & 0x10_i32 != 0) as i32;
    // Collected results, owned, replacing the `*mut T` intrusive chain.
    let mut out: Vec<ParsedName> = Vec::new();
    let mut found_wait: i32 = 0;
    // Push one resolved name (a NUL-terminated C string `s`) as an owned entry,
    // applying any pending `.WAIT` marker. The bytes are copied out of `s`.
    macro_rules! push_name {
        ($s:expr) => {{
            let cs = ::std::ffi::CStr::from_ptr($s);
            out.push(ParsedName {
                name: cs.to_bytes().to_vec(),
                wait: found_wait != 0,
            });
            found_wait = 0;
        }};
    }
    let mut p: *mut ::core::ffi::c_char;
    let mut gl: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut tp: *mut ::core::ffi::c_char;
    let mut findmap: i32 = stopmap | MAP_VMSCOMMA | MAP_NUL;
    if !(flags & 0x20_i32 != 0) {
        findmap |= MAP_BLANK;
    }
    stopmap |= MAP_NUL;
    if !(flags & 0x4_i32 != 0) {
        // read.rs carries its own layout-identical glob_t; reconcile the
        // nominal types until the duplicate struct is unified.
        dir_setup_glob((&raw mut gl).cast());
    }
    let l: size_t = (strlen(*stringp.as_ref().expect("parse_file_seq: null stringp")) as size_t)
        .wrapping_add(1);
    // Reused unquoting scratch buffer (the former `static mut tmpbuf`/
    // `tmpbuf_len`), now an owned `Vec<u8>` on the per-run context so it
    // survives the `main_0` rebuild the same way `read_dirstream_buf` does.
    // The borrow is scoped to just the grow-and-take-pointer step; the raw
    // pointer it hands back is the sole FFI-typed value the rest of this
    // (already raw-pointer-heavy) function works with, matching the
    // `PidString`/`pid2str` treatment.
    let tmpbuf: *mut ::core::ffi::c_char = {
        let mut buf = ctx.file_seq_tmpbuf.borrow_mut();
        if buf.len() < l as usize {
            buf.resize(l as usize, 0);
        }
        buf.as_mut_ptr() as *mut ::core::ffi::c_char
    };
    tp = tmpbuf;
    p = *stringp.as_ref().expect("parse_file_seq: null stringp");
    loop {
        let mut name: *const ::core::ffi::c_char;
        let mut nlist: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut tildep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut globme: i32 = 1;
        let mut memname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        // Owns the split `archive`/`member` buffer for the iteration when
        // `ar_name` matches below (replacing the old `ar_parse_name`
        // xstrdup + `free`); stays `None` otherwise.
        let mut _parsed_ar: Option<ParsedArName> = None;
        let mut s: *mut ::core::ffi::c_char;
        let mut nlen: size_t;
        let mut tot: i32 = 0;
        let mut i: i32;
        while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort).offset(cbyte(p) as isize)
            as i32
            & (0x2_i32 | 0x4_i32)
            != 0
        {
            p = p.offset(1_i32 as isize);
        }
        if *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort).offset(cbyte(p) as isize) as i32
            & stopmap
            != 0
        {
            break;
        }
        s = p;
        p = find_map_unquote(p, findmap);
        if p.is_null() {
            p = s.offset(strlen(s) as isize);
        }
        if flags & 0x40_i32 != 0
            && crate::parser::is_wait_token(::core::slice::from_raw_parts(
                s as *const u8,
                p.offset_from(s) as usize,
            ))
        {
            found_wait = 1;
        } else {
            if !(flags & 0x1_i32 != 0) {
                // Strip redundant leading `./` prefixes via the pure parser
                // helper instead of walking the buffer with raw pointers.
                let token =
                    ::core::slice::from_raw_parts(s as *const u8, p.offset_from(s) as usize);
                s = s.add(crate::parser::strip_dot_slash_prefix(token));
            }
            if s == p {
                *tp.offset(0_i32 as isize) = '.' as i32 as ::core::ffi::c_char;
                *tp.offset(1_i32 as isize) = '/' as i32 as ::core::ffi::c_char;
                *tp.offset(2_i32 as isize) = 0;
                nlen = 2;
            } else {
                nlen = p.offset_from(s) as ::core::ffi::c_long as size_t;
                memcpy(
                    tp as *mut ::core::ffi::c_void,
                    s as *const ::core::ffi::c_void,
                    nlen as size_t,
                );
                *tp.offset(nlen as isize) = 0;
            }
            if !(flags & 0x2_i32 != 0)
                && tp == tmpbuf
                && *tp.offset(0_i32 as isize) as i32 != '(' as i32
                && *tp.offset(nlen.wrapping_sub(1) as isize) as i32 != ')' as i32
            {
                let n: *mut ::core::ffi::c_char = strchr(tp, '(' as i32);
                if !n.is_null() {
                    let mut e: *const ::core::ffi::c_char = p;
                    loop {
                        let o: *const ::core::ffi::c_char = e;
                        while *(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                            .offset(cbyte(e) as isize) as i32
                            & (0x2_i32 | 0x4_i32)
                            != 0
                        {
                            e = e.offset(1_i32 as isize);
                        }
                        while !(*(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
                            .offset(cbyte(e) as isize) as i32
                            & findmap
                            != 0)
                        {
                            e = e.offset(1_i32 as isize);
                        }
                        if e == o {
                            break;
                        }
                        if cbyte(e.offset(-1_i32 as isize)) as i32 == ')' as i32 {
                            nlen = nlen.wrapping_sub(n.offset(1_i32 as isize).offset_from(tp)
                                as ::core::ffi::c_long
                                as size_t);
                            tp = n.offset(1_i32 as isize);
                            break;
                        } else if !(cbyte(e) as i32 != 0) {
                            break;
                        }
                    }
                    if nlen == 0 {
                        continue;
                    }
                }
            }
            if tp > tmpbuf {
                if *tp.offset(nlen.wrapping_sub(1) as isize) as i32 == ')' as i32 {
                    tp = tmpbuf;
                    if nlen == 1 {
                        continue;
                    }
                } else {
                    let fresh6 = nlen;
                    nlen = nlen.wrapping_add(1);
                    *tp.offset(fresh6 as isize) = ')' as i32 as ::core::ffi::c_char;
                    *tp.offset(nlen as isize) = 0;
                }
            }
            if flags & 0x4_i32 != 0 {
                let __n_buf = concat(&[cstr_bytes_or_empty(prefix), cstr_bytes_or_empty(tmpbuf)]);
                push_name!(__n_buf.as_ptr() as *const ::core::ffi::c_char);
            } else {
                name = tmpbuf;
                if *tmpbuf.offset(0_i32 as isize) as i32 == '~' as i32 {
                    // Nothing is owned yet this iteration — `tildep` is still
                    // null and the glob has not run — and `*stringp` is left
                    // unadvanced, so the caller drops the whole sequence.
                    tildep = tilde_expand(ctx, tmpbuf)?;
                    if !tildep.is_null() {
                        name = tildep;
                    }
                }
                if !(flags & 0x2_i32 != 0) && ar_name_err(ctx, CStr::from_ptr(name))? {
                    let parsed = ParsedArName::parse(CStr::from_ptr(name));
                    memname = parsed.memname();
                    name = parsed.arname();
                    _parsed_ar = Some(parsed);
                }
                if !(flags & 0x8_i32 != 0)
                    && strpbrk(name, b"?*[\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                {
                    globme = 0;
                    tot = 1;
                    nlist = &raw mut name;
                } else {
                    // On any glob failure other than NOMATCH-without-NULL-glob,
                    // fall back to using the literal name.
                    let mut use_literal = false;
                    match glob(name, GLOB_ALTDIRFUNC, None, &raw mut gl) {
                        GLOB_NOSPACE => {
                            out_of_memory();
                        }
                        0 => {
                            tot = gl.gl_pathc as i32;
                            nlist = gl.gl_pathv as *mut *const ::core::ffi::c_char;
                        }
                        GLOB_NOMATCH => {
                            if flags & 0x8_i32 != 0 {
                                tot = 0;
                            } else {
                                use_literal = true;
                            }
                        }
                        _ => {
                            use_literal = true;
                        }
                    }
                    if use_literal {
                        tot = 1;
                        nlist = &raw mut name;
                    }
                }
                i = 0;
                while i < tot {
                    if !memname.is_null() {
                        // Archive member glob: `ar_glob` still builds an intrusive
                        // `NameSeq` chain (it lives in ar.rs, out of this slice);
                        // walk it to collect the resolved member names as owned
                        // entries, then free the chain. The `prefix` rewrite the
                        // c2rust loop did on each matched node is applied as we copy.
                        let found: *mut NameSeq =
                            ar_glob::<NameSeq>(ctx, *nlist.offset(i as isize), memname);
                        if found.is_null() {
                            let __n_0_buf = concat(&[
                                cstr_bytes_or_empty(prefix),
                                cstr_bytes_or_empty(*nlist.offset(i as isize)),
                                b"(",
                                cstr_bytes_or_empty(memname),
                                b")",
                            ]);
                            push_name!(__n_0_buf.as_ptr() as *const ::core::ffi::c_char);
                        } else {
                            // Which base name each element carries. `ar_glob`
                            // built `archive(member)`, but read.c then rewrites
                            // every element as `prefix + name` — and `name` is
                            // the *archive* name, so the member names are lost
                            // (upstream bug, #460: the intended operand is
                            // `found->name`). Only the `cachep && !prefix` case
                            // escapes the rewrite and keeps the member name.
                            // Reproduced bug-for-bug by default so the oracle
                            // diff stays byte-identical; opt in to the fix with
                            // `MAKERS_AR_GLOB_MEMBER_NAMES=1`.
                            let member_names = crate::ar::ar_glob_member_names();
                            let mut node = found;
                            while let Some(nref) = node.as_ref() {
                                let base: *const ::core::ffi::c_char =
                                    if member_names || (cachep != 0 && prefix.is_null()) {
                                        nref.name
                                    } else {
                                        name
                                    };
                                let nm_buf = if !prefix.is_null() {
                                    Some(concat(&[cstr_bytes_or_empty(prefix), cstr_bytes_or_empty(base)]))
                                } else {
                                    None
                                };
                                let nm: *const ::core::ffi::c_char = match &nm_buf {
                                    Some(buf) => buf.as_ptr() as *const ::core::ffi::c_char,
                                    None => base,
                                };
                                push_name!(nm);
                                node = nref.next;
                            }
                            crate::file::free_seq_chain(found);
                        }
                    } else {
                        let __n_1_buf =
                            concat(&[cstr_bytes_or_empty(prefix), cstr_bytes_or_empty(*nlist.offset(i as isize))]);
                        push_name!(__n_1_buf.as_ptr() as *const ::core::ffi::c_char);
                    }
                    i += 1;
                }
                if globme != 0 {
                    globfree(&raw mut gl);
                }
                free(tildep as *mut ::core::ffi::c_void);
            }
        }
    }
    *stringp = p;
    Ok(out)
}

#[cfg(test)]
mod name_seq_len_tests {
    use super::{name_seq_len, NameSeq};

    /// Original c2rust implementation, preserved verbatim as a differential
    /// oracle: counted with a `c_ushort` accumulator that wraps at 65536.
    unsafe fn name_seq_len_unsafe_oracle(mut n: *mut NameSeq) -> ::core::ffi::c_ushort {
        let mut len: ::core::ffi::c_ushort = 0;
        while let Some(node) = n.as_ref() {
            len = len.wrapping_add(1);
            n = node.next;
        }
        len
    }

    /// Build a `NameSeq` chain of `count` nodes (names are irrelevant to the
    /// length), leaking the nodes so the chain outlives the call.
    fn make_chain(count: usize) -> *mut NameSeq {
        let mut head: *mut NameSeq = ::core::ptr::null_mut();
        for _ in 0..count {
            let node = Box::new(NameSeq {
                next: head,
                name: ::core::ptr::null(),
            });
            head = Box::leak(node);
        }
        head
    }

    #[test]
    fn counts_chain_length() {
        for count in [0usize, 1, 2, 5, 64] {
            let chain = make_chain(count);
            assert_eq!(name_seq_len(unsafe { chain.as_ref() }), count);
        }
    }

    #[test]
    fn matches_unsafe_oracle_low_16_bits() {
        for count in [0usize, 1, 3, 100, 1000] {
            let chain = make_chain(count);
            let safe = name_seq_len(unsafe { chain.as_ref() });
            let oracle = unsafe { name_seq_len_unsafe_oracle(chain) };
            assert_eq!(safe as ::core::ffi::c_ushort, oracle);
            assert_eq!(safe, count);
        }
    }
}

#[cfg(test)]
mod vpath_pattern_token_unsafe_oracle {
    use super::vpath_pattern_token;

    /// Original c2rust ownership pattern preserved verbatim as a differential
    /// oracle for the safe [`super::vpath_pattern_token`]: `xstrndup(p, len)`
    /// produced a libc-`malloc`ed copy that the caller `free`d. We reproduce
    /// it with `libc::strndup` / `libc::free` and return the observable bytes.
    unsafe fn xstrndup_bytes(token: &[u8]) -> Vec<u8> {
        // `token` is borrowed by the C call only; the duplicate is owned here.
        let dup = libc::strndup(
            token.as_ptr() as *const ::core::ffi::c_char,
            token.len() as libc::size_t,
        );
        assert!(!dup.is_null(), "strndup allocation failed");
        let bytes = ::core::ffi::CStr::from_ptr(dup).to_bytes().to_vec();
        libc::free(dup as *mut ::core::ffi::c_void);
        bytes
    }

    #[test]
    fn matches_oracle() {
        let cases: &[&[u8]] = &[
            b"",
            b"a",
            b"%.o",
            b"%.c",
            b"src/%",
            b"foo.bar",
            b"with-high\xff\x80\x01-bytes",
            b"embedded\x00nul-truncates",
            b"\x00leading-nul",
            b"trailing-nul\x00",
        ];
        for &token in cases {
            // The safe version yields a mutable, NUL-terminated owned buffer;
            // it must end in a single terminator and its observable string
            // bytes (everything before that terminator) must match the bytes
            // the original `strndup`'d buffer exposed.
            let safe = vpath_pattern_token(token);
            assert_eq!(
                safe.last().copied(),
                Some(0u8),
                "vpath_pattern_token must be NUL-terminated for {token:?}"
            );
            let oracle = unsafe { xstrndup_bytes(token) };
            assert_eq!(
                &safe[..safe.len() - 1],
                oracle.as_slice(),
                "vpath_pattern_token mismatch for {token:?}"
            );
        }
    }
}

#[cfg(test)]
mod file_seq_rejection_tests {
    //! Since #442 `parse_file_seq` returns `Result`: a `~` prefix whose home
    //! lookup cannot be expanded comes back as a rejection instead of ending
    //! the process from inside makefile parsing. The whole cone above it —
    //! `split_prereqs{,_vec}`, `string_glob`, `parse_deps`, `parse_dep_names`
    //! and the `.SUFFIXES`/builtin-rule setup — propagates the same verdict.

    use super::parse_file_seq;
    use crate::build_result::BuildError;
    use crate::expand::VARIABLE_BUFFER_TEST_LOCK;
    use std::ffi::CString;

    /// Define `name` as a recursive global variable holding `value`.
    ///
    /// # Safety
    /// `ctx` must have its global variable set initialized.
    unsafe fn define_recursive(ctx: &crate::execctx::ExecContext, name: &str, value: &str) {
        let cname = CString::new(name).unwrap();
        let cvalue = CString::new(value).unwrap();
        crate::variable::define_variable_in_set(
            ctx,
            cname.as_ptr(),
            name.len() as crate::ffi_types::size_t,
            cvalue.as_ptr(),
            crate::variable::o_file,
            1,
            ctx.variable_globals.global_variable_set.as_ptr(),
            ::core::ptr::null::<crate::floc::Floc>(),
        )
        .expect("test fixture defines a well-formed name");
    }

    fn fresh_ctx() -> crate::execctx::ExecContext {
        crate::make_main::initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();
        // SAFETY: fresh context; each table is initialized once.
        unsafe {
            crate::function::hash_init_function_table(&ctx);
            crate::variable::init_hash_global_variable_set(&ctx);
            crate::expand::initialize_variable_output(&ctx);
        }
        ctx
    }

    /// Parse `seq` as a file sequence in a context where `HOME` expands to
    /// `$(word 1)` — a builtin called with the wrong number of arguments, so
    /// the expansion is refused.
    ///
    /// # Safety
    /// Single-threaded fresh context; `seq` is copied into a writable buffer.
    unsafe fn parse_with_rejected_home(
        ctx: &crate::execctx::ExecContext,
        seq: &str,
    ) -> Result<Vec<super::ParsedName>, BuildError> {
        define_recursive(ctx, "HOME", "$(word 1)");
        let mut buf = seq.as_bytes().to_vec();
        buf.push(0);
        let mut p: *mut ::core::ffi::c_char = buf.as_mut_ptr().cast();
        parse_file_seq(
            ctx,
            &raw mut p,
            0,
            0x1_i32,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0,
        )
    }

    /// `~/…` takes the `HOME` arm of `tilde_expand`, so the refused expansion
    /// travels out of the parse rather than exiting.
    #[test]
    fn rejected_home_expansion_propagates() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: see `parse_with_rejected_home`.
        unsafe {
            assert!(matches!(
                parse_with_rejected_home(&ctx, "~/somefile"),
                Err(BuildError::Failure)
            ));
        }
    }

    /// A bare `~` is the other half of the same arm (`name[1]` is NUL).
    #[test]
    fn rejected_home_expansion_propagates_for_bare_tilde() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: as above.
        unsafe {
            assert!(matches!(
                parse_with_rejected_home(&ctx, "~"),
                Err(BuildError::Failure)
            ));
        }
    }

    /// Names without a `~` never reach `tilde_expand`, so the same context
    /// still parses them — the flip did not turn ordinary parses into errors.
    #[test]
    fn plain_names_still_parse() {
        let _g = VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let ctx = fresh_ctx();
        // SAFETY: as above.
        unsafe {
            let parsed =
                parse_with_rejected_home(&ctx, "alpha beta").expect("no `~`, so no home lookup");
            let names: Vec<&[u8]> = parsed.iter().map(|p| p.name.as_slice()).collect();
            assert_eq!(names, vec![&b"alpha"[..], &b"beta"[..]]);
        }
    }
}
