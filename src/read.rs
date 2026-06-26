pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __size_t, __syscall_slong_t, __time_t, __uid_t, size_t, uintmax_t,
};
use crate::file::{dep, file, NameSeq, SeqNode};
use crate::file::{
    commands, CommandState, Commands, Dep, File, GoalDep, UpdateStatus, VariableSet,
    VariableSetList,
};
use crate::misc::{
    collapse_continuations, copy_dep, copy_dep_chain, find_next_token, next_token, xcalloc,
    xmalloc, xrealloc, xstrdup, xstrndup,
};
use crate::output::FmtArg;
use crate::stdio::FILE;
use crate::strcache::{strcache_add, strcache_add_len};
use c2rust_bitfields;
use libc::{
    __errno_location, free, getenv, getlogin, printf, puts, strchr, strcpy, strerror, strpbrk,
};
extern "C" {
    pub type dirent;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fdopen(__fd: i32, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: i32,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
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
use crate::ar::{ar_glob, ar_name, ar_parse_name};
use crate::dir::{dir_setup_glob, file_exists_p};
use crate::expand::{
    allocated_expand_string_for_file, allocated_expand_variable, expand_string_buf,
    variable_buffer, variable_buffer_output,
};
pub use crate::file::nameseq;
use crate::file::{enter_file, enter_prereqs, lookup_file, split_prereqs};
use crate::function::{patsubst_expand_pat, pattern_matches, strip_whitespace};
use crate::load::load_file;
use crate::make_main::{
    db_level, default_file, default_goal_var, one_shell, opt_snapped_deps, posix_pedantic,
    second_expansion, stopchar_map,
};
use crate::misc::concat;
use crate::output::{error, fatal, out_of_memory, perror_with_name, pfatal_with_name};
use crate::posixos::fd_noinherit;
use crate::rule::{create_pattern_rule, suffix_file};
use crate::variable::{
    assign_variable_definition, create_pattern_var, current_variable_set_list,
    define_variable_in_set, do_variable_definition, initialize_file_variables, lookup_variable,
    parse_variable_definition, try_variable_definition, undefine_variable_in_set,
};
use crate::vpath::construct_vpath_list;
use ::core::ffi::CStr;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct goaldep {
    pub next: *mut goaldep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut file,
    pub shuf: *mut goaldep,
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
    pub c2rust_padding: [u8; 2],
    pub error: i32,
    pub floc: Floc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ebuffer {
    pub buffer: *mut ::core::ffi::c_char,
    pub bufnext: *mut ::core::ffi::c_char,
    pub bufstart: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub fp: *mut FILE,
    pub floc: Floc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conditionals {
    pub if_cmds: ::core::ffi::c_uint,
    pub allocated: ::core::ffi::c_uint,
    pub ignoring: *mut ::core::ffi::c_char,
    pub seen_else: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct vmodifiers {
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
pub use crate::variable::pattern_var;
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
#[inline]
unsafe extern "C" fn alloc_dep() -> *mut Dep {
    xcalloc(::core::mem::size_of::<Dep>() as size_t) as *mut Dep
}
#[inline]
unsafe extern "C" fn alloc_goaldep() -> *mut GoalDep {
    xcalloc(::core::mem::size_of::<GoalDep>() as size_t) as *mut GoalDep
}
#[inline]
unsafe extern "C" fn free_ns(n: *mut NameSeq) {
    free(n as *mut ::core::ffi::c_void);
}
struct NameSeqNode {
    name: *const ::core::ffi::c_char,
    next: *mut NameSeq,
}
unsafe fn name_seq_len(mut n: *mut NameSeq) -> ::core::ffi::c_ushort {
    let mut len: ::core::ffi::c_ushort = 0;
    while let Some(node) = n.as_ref() {
        len = len.wrapping_add(1);
        n = node.next;
    }
    len
}
unsafe fn pop_name_seq(n: *mut NameSeq, context: &str) -> NameSeqNode {
    let node = n.as_ref().expect(context);
    let popped = NameSeqNode {
        name: node.name,
        next: node.next,
    };
    free_ns(n);
    popped
}
#[inline]
unsafe extern "C" fn free_dep_chain(d: *mut Dep) {
    crate::file::free_seq_chain(d);
}
pub const NONEXISTENT_MTIME: i32 = 1;
static mut toplevel_conditionals: conditionals = conditionals {
    if_cmds: 0,
    allocated: 0,
    ignoring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    seen_else: ::core::ptr::null_mut::<::core::ffi::c_char>(),
};
static mut conditionals: *mut conditionals = &raw const toplevel_conditionals as *mut conditionals;
/// Default system include directories searched when `-I` does not disable them.
/// Genuine Rust byte slices (no NUL terminators, no `*const c_char`).
static DEFAULT_INCLUDE_DIRECTORIES: [&[u8]; 3] =
    [b"/usr/gnu/include", b"/usr/local/include", b"/usr/include"];
pub static mut reading_file: *const Floc = ::core::ptr::null::<Floc>();
static mut read_files: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn read_all_makefiles(
    ctx: &crate::execctx::ExecContext,
    mut makefiles: *mut *const ::core::ffi::c_char,
) -> *mut goaldep {
    let mut num_makefiles: ::core::ffi::c_uint = 0;
    define_variable_in_set(
        ctx,
        b"MAKEFILE_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_file,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    if 0x1_i32 & db_level != 0 {
        printf(b"Reading makefiles...\n\0" as *const u8 as *const ::core::ffi::c_char);
        fflush(stdout);
    }
    let value: *mut ::core::ffi::c_char;
    let mut name: *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;
    let mut length: size_t = 0;
    value = allocated_expand_variable(
        ctx,
        b"MAKEFILES\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1),
    );
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
            strcache_add(name),
            (RM_NO_DEFAULT_GOAL | RM_INCLUDED | RM_DONTCARE) as ::core::ffi::c_ushort,
        );
    }
    free(value as *mut ::core::ffi::c_void);
    if !makefiles.is_null() {
        while let Some(mref) = makefiles.as_mut().filter(|m| !m.is_null()) {
            let d: *mut goaldep = eval_makefile(ctx, *mref, 0);
            if *__errno_location() != 0 {
                perror_with_name(ctx, b"\0" as *const u8 as *const ::core::ffi::c_char, *mref);
            }
            *mref = if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            num_makefiles = num_makefiles.wrapping_add(1);
            makefiles = makefiles.offset(1_i32 as isize);
        }
    }
    if num_makefiles == 0 {
        static mut default_makefiles: [*const ::core::ffi::c_char; 4] = [
            b"GNUmakefile\0" as *const u8 as *const ::core::ffi::c_char,
            b"makefile\0" as *const u8 as *const ::core::ffi::c_char,
            b"Makefile\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut p_0: *const *const ::core::ffi::c_char =
            &raw const default_makefiles as *const *const ::core::ffi::c_char;
        while !(*p_0).is_null() && file_exists_p(ctx, *p_0) == 0 {
            p_0 = p_0.offset(1_i32 as isize);
        }
        if !(*p_0).is_null() {
            eval_makefile(ctx, *p_0, 0);
            if *__errno_location() != 0 {
                perror_with_name(ctx, b"\0" as *const u8 as *const ::core::ffi::c_char, *p_0);
            }
        } else {
            p_0 = &raw const default_makefiles as *const *const ::core::ffi::c_char;
            while !(*p_0).is_null() {
                let d_0: *mut GoalDep = alloc_goaldep();
                (*d_0).file = enter_file(strcache_add(*p_0));
                (*d_0).flags = RM_DONTCARE as ::core::ffi::c_uint as ::core::ffi::c_uint;
                (*d_0).next = read_files;
                read_files = d_0;
                p_0 = p_0.offset(1_i32 as isize);
            }
        }
    }
    read_files
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn install_conditionals(new: *mut conditionals) -> *mut conditionals {
    let save: *mut conditionals = conditionals;
    memset(
        new as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<conditionals>() as size_t,
    );
    conditionals = new;
    save
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn restore_conditionals(saved: *mut conditionals) {
    free((*conditionals).ignoring as *mut ::core::ffi::c_void);
    free((*conditionals).seen_else as *mut ::core::ffi::c_void);
    conditionals = saved;
}
unsafe fn eval_makefile(
    ctx: &crate::execctx::ExecContext,
    mut filename: *const ::core::ffi::c_char,
    flags: ::core::ffi::c_ushort,
) -> *mut GoalDep {
    let deps: *mut GoalDep;
    let mut ebuf: ebuffer = ebuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufnext: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufstart: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        fp: ::core::ptr::null_mut::<FILE>(),
        floc: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
    };
    let curfile: *const Floc;
    let mut expanded: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    deps = alloc_goaldep();
    (*deps).next = read_files;
    read_files = deps;
    ebuf.floc.filenm = filename;
    ebuf.floc.lineno = 1;
    ebuf.floc.offset = 0;
    if 0x2_i32 & db_level != 0 {
        printf(
            b"Reading makefile '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        if flags as i32 & RM_NO_DEFAULT_GOAL != 0 {
            printf(b" (no default goal)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if flags as i32 & RM_INCLUDED != 0 {
            printf(b" (search path)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if flags as i32 & RM_DONTCARE != 0 {
            printf(b" (don't care)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if flags as i32 & RM_NO_TILDE != 0 {
            printf(b" (no ~ expansion)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        puts(b"...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if flags as i32 & RM_NO_TILDE == 0 && *filename.offset(0_i32 as isize) as i32 == '~' as i32 {
        expanded = tilde_expand(ctx, filename);
        if !expanded.is_null() {
            filename = expanded;
        }
    }
    *__errno_location() = 0;
    loop {
        *__errno_location() = 0;
        ebuf.fp = fopen(filename, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
        if !(ebuf.fp.is_null() && *__errno_location() == EINTR) {
            break;
        }
    }
    (*deps).error = *__errno_location();
    match (*deps).error {
        EMFILE | ENFILE | ENOMEM => {
            let err: *const ::core::ffi::c_char = strerror((*deps).error);
            fatal(
                ctx,
                reading_file,
                strlen(err) as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                err,
            );
        }
        _ => {}
    }
    if ebuf.fp.is_null()
        && (*deps).error == ENOENT
        && flags as i32 & (1) << 1 != 0
        && !(*(stopchar_map().as_ptr() as *mut ::core::ffi::c_ushort)
            .offset(*filename.as_ref().expect("eval_makefile: null filename")
                as ::core::ffi::c_uchar as isize) as i32
            & 0x8000_i32
            != 0)
    {
        use std::os::unix::ffi::OsStrExt;
        use std::os::unix::io::IntoRawFd;
        // `filename` is an existing C string supplied by the caller; read its
        // bytes (no new C string constructed) to build candidate paths.
        let filename_bytes = CStr::from_ptr(filename).to_bytes().to_vec();
        let filename_os = std::ffi::OsStr::from_bytes(&filename_bytes);
        // The include search path is owned by `main_0`'s `Options` and reached
        // through the `with_options` borrow channel (no `static mut`). Snapshot
        // it into a local `Vec` so the `RefCell` borrow is released before the
        // file-open work below (which re-enters the eval engine on success).
        let search_dirs: Vec<std::path::PathBuf> =
            crate::make_main::with_options(|o| o.resolved_include_dirs.borrow().clone());
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
                    // Hand the descriptor to the C `FILE*` eval machinery; the
                    // path never becomes a `*const c_char` in our code.
                    ebuf.fp = fdopen(
                        f.into_raw_fd(),
                        b"r\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as *mut FILE;
                    filename =
                        crate::strcache::strcache_add_bytes(candidate.as_os_str().as_bytes());
                    break;
                }
                Err(e) => {
                    let errno = e.raw_os_error().unwrap_or(ENOENT);
                    if errno != ENOENT {
                        filename =
                            crate::strcache::strcache_add_bytes(candidate.as_os_str().as_bytes());
                        (*deps).error = errno;
                        break;
                    }
                }
            }
        }
    }
    filename = strcache_add(filename);
    (*deps).file = lookup_file(filename);
    if (*deps).file.is_null() {
        (*deps).file = enter_file(filename);
    }
    filename = (*(*deps).file).name;
    (*deps).flags = flags as ::core::ffi::c_uint as ::core::ffi::c_uint;
    (*(*deps).file).is_explicit = true;
    free(expanded as *mut ::core::ffi::c_void);
    if ebuf.fp.is_null() {
        *__errno_location() = (*deps).error;
        (*(*deps).file).last_mtime = NONEXISTENT_MTIME as uintmax_t;
        return deps;
    }
    (*deps).error = 0;
    if (*(*deps).file).last_mtime == NONEXISTENT_MTIME as uintmax_t {
        (*(*deps).file).last_mtime = 0 as uintmax_t;
    }
    fd_noinherit(fileno(ebuf.fp));
    do_variable_definition(
        ctx,
        &raw mut ebuf.floc,
        b"MAKEFILE_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        filename,
        o_file,
        f_append_value,
        0,
        s_global,
    );
    ebuf.size = 200;
    ebuf.bufstart = xmalloc(ebuf.size) as *mut ::core::ffi::c_char;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    curfile = reading_file;
    reading_file = &raw mut ebuf.floc;
    eval(
        ctx,
        &raw mut ebuf,
        (flags as i32 & RM_NO_DEFAULT_GOAL == 0) as i32,
    );
    reading_file = curfile;
    fclose(ebuf.fp);
    free(ebuf.bufstart as *mut ::core::ffi::c_void);
    *__errno_location() = 0;
    deps
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn eval_buffer(
    ctx: &crate::execctx::ExecContext,
    buffer: *mut ::core::ffi::c_char,
    flocp: *const Floc,
) {
    let mut ebuf: ebuffer = ebuffer {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufnext: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bufstart: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        size: 0,
        fp: ::core::ptr::null_mut::<FILE>(),
        floc: Floc {
            filenm: ::core::ptr::null::<::core::ffi::c_char>(),
            lineno: 0,
            offset: 0,
        },
    };
    let saved: *mut conditionals;
    let mut new: conditionals = conditionals {
        if_cmds: 0,
        allocated: 0,
        ignoring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        seen_else: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let curfile: *const Floc;
    ebuf.size = strlen(buffer) as size_t;
    ebuf.bufstart = buffer;
    ebuf.bufnext = ebuf.bufstart;
    ebuf.buffer = ebuf.bufnext;
    ebuf.fp = ::core::ptr::null_mut::<FILE>();
    if !flocp.is_null() {
        ebuf.floc = *flocp;
    } else if !reading_file.is_null() {
        ebuf.floc = *reading_file;
    } else {
        ebuf.floc.filenm = ::core::ptr::null::<::core::ffi::c_char>();
        ebuf.floc.lineno = 1;
        ebuf.floc.offset = 0;
    }
    curfile = reading_file;
    reading_file = &raw mut ebuf.floc;
    saved = install_conditionals(&raw mut new);
    eval(ctx, &raw mut ebuf, 1);
    restore_conditionals(saved);
    reading_file = curfile;
}
unsafe fn parse_var_assignment(
    ctx: &crate::execctx::ExecContext,
    line: *const ::core::ffi::c_char,
    targvar: i32,
    flocp: *const Floc,
    vmod: *mut vmodifiers,
) -> *mut ::core::ffi::c_char {
    memset(
        vmod as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<vmodifiers>() as size_t,
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
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn eval(ctx: &crate::execctx::ExecContext, ebuf: *mut ebuffer, set_default: i32) {
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
    let mut filenames: *mut nameseq = ::core::ptr::null_mut::<nameseq>();
    let mut depstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nlines: ::core::ffi::c_long = 0;
    let mut two_colon: i32 = 0;
    let mut prefix: ::core::ffi::c_char = crate::make_main::opt_cmd_prefix();
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
        let mut vmod: vmodifiers = vmodifiers {
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
                if 0x1_i32 & db_level != 0 {
                    if !(*ebuf).floc.filenm.is_null() {
                        printf(
                            b"Skipping UTF-8 BOM in makefile '%s'\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*ebuf).floc.filenm,
                        );
                    } else {
                        printf(
                            b"Skipping UTF-8 BOM in makefile buffer\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
            }
        }
        // Classify the line by its first byte through the typed AST: empty
        // line, recipe line (begins with `cmd_prefix`), or a line to parse.
        let first_byte = *line.offset(0_i32 as isize) as ::core::ffi::c_uchar;
        let line_kind = crate::parser::LineKind::classify(
            first_byte,
            crate::make_main::opt_cmd_prefix() as ::core::ffi::c_uchar,
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
            if !filenames.is_null() {
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
        collapse_continuations(collapsed);
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
                if !filenames.is_null() {
                    fi.lineno = tgts_started as ::core::ffi::c_ulong;
                    fi.offset = 0;
                    record_files(
                        ctx,
                        filenames,
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
                    );
                    filenames = ::core::ptr::null_mut::<NameSeq>();
                }
                commands_idx = 0;
                no_targets = 0;
                pattern = ::core::ptr::null::<::core::ffi::c_char>();
                also_make_targets = 0;
                if vmod.undefine_v() != 0 {
                    do_undefine(ctx, p, origin, ebuf);
                } else {
                    if vmod.define_v() != 0 {
                        v = do_define(ctx, p, origin, ebuf);
                    } else {
                        v = try_variable_definition(ctx, fstart, p, origin, s_global);
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
                let i: i32 = conditional_line(ctx, p, wlen, fstart, initial_tab);
                if i != -2_i32 {
                    if i == -1_i32 {
                        fatal(
                            ctx,
                            fstart,
                            0,
                            b"invalid syntax in conditional\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
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
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames,
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
                            );
                            filenames = ::core::ptr::null_mut::<NameSeq>();
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        if *p2 as i32 == 0 {
                            crate::make_main::with_options(|o| {
                                o.export_all_variables.set(exporting != 0)
                            });
                        } else {
                            let mut l: size_t = 0;
                            let mut cp: *const ::core::ffi::c_char;
                            let ap: *mut ::core::ffi::c_char;
                            ap = allocated_expand_string_for_file(
                                ctx,
                                p2,
                                ::core::ptr::null_mut::<File>(),
                            );
                            cp = ap;
                            p = find_next_token(&raw mut cp, &raw mut l);
                            while !p.is_null() {
                                let mut v_0: *mut variable = lookup_variable(ctx, p, l);
                                if v_0.is_null() {
                                    v_0 = define_variable_in_set(
                                        ctx,
                                        p,
                                        l,
                                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                                        o_file,
                                        0,
                                        ::core::ptr::null_mut::<variable_set>(),
                                        fstart,
                                    );
                                }
                                v_0.as_mut().expect("export: null variable").set_export(
                                    (if exporting != 0 {
                                        v_export as i32
                                    } else {
                                        v_noexport as i32
                                    }) as variable_export
                                        as variable_export,
                                );
                                p = find_next_token(&raw mut cp, &raw mut l);
                            }
                            free(ap as *mut ::core::ffi::c_void);
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
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames,
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
                            );
                            filenames = ::core::ptr::null_mut::<NameSeq>();
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
                        );
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
                        );
                    } else if matches!(
                        line_class,
                        crate::parser::LineClass::File(
                            crate::parser::FileDirective::Include
                                | crate::parser::FileDirective::IncludeOpt
                        )
                    ) {
                        let save: *mut conditionals;
                        let mut new_conditionals: conditionals = conditionals {
                            if_cmds: 0,
                            allocated: 0,
                            ignoring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            seen_else: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        };
                        let mut files: *mut nameseq;
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
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames,
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
                            );
                            filenames = ::core::ptr::null_mut::<NameSeq>();
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        p = allocated_expand_string_for_file(
                            ctx,
                            p2,
                            ::core::ptr::null_mut::<file>(),
                        );
                        if *p as i32 == 0 {
                            free(p as *mut ::core::ffi::c_void);
                        } else {
                            p2 = p;
                            files = parse_file_seq::<nameseq>(
                                ctx,
                                &raw mut p2,
                                ::core::mem::size_of::<nameseq>() as size_t,
                                0x1_i32,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                0x2_i32,
                            ) as *mut nameseq;
                            free(p as *mut ::core::ffi::c_void);
                            save = install_conditionals(&raw mut new_conditionals);
                            if !filenames.is_null() {
                                fi.lineno = tgts_started as ::core::ffi::c_ulong;
                                fi.offset = 0;
                                record_files(
                                    ctx,
                                    filenames,
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
                                );
                                filenames = ::core::ptr::null_mut::<NameSeq>();
                            }
                            commands_idx = 0;
                            no_targets = 0;
                            pattern = ::core::ptr::null::<::core::ffi::c_char>();
                            also_make_targets = 0;
                            while !files.is_null() {
                                let next: *mut NameSeq = (*files).next;
                                let flags: ::core::ffi::c_ushort = (RM_INCLUDED
                                    | RM_NO_TILDE
                                    | (if noerror != 0 { RM_DONTCARE } else { 0 })
                                    | (if set_default != 0 {
                                        0
                                    } else {
                                        RM_NO_DEFAULT_GOAL
                                    }))
                                    as ::core::ffi::c_ushort;
                                let d: *mut goaldep = eval_makefile(ctx, (*files).name, flags);
                                (*d).floc = *fstart;
                                free_ns(files);
                                files = next;
                            }
                            restore_conditionals(save);
                        }
                    } else if matches!(
                        line_class,
                        crate::parser::LineClass::File(
                            crate::parser::FileDirective::Load
                                | crate::parser::FileDirective::LoadOpt
                        )
                    ) && is_rule == 0
                    {
                        let mut files_0: *mut nameseq;
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
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames,
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
                            );
                            filenames = ::core::ptr::null_mut::<NameSeq>();
                        }
                        commands_idx = 0;
                        no_targets = 0;
                        pattern = ::core::ptr::null::<::core::ffi::c_char>();
                        also_make_targets = 0;
                        p = allocated_expand_string_for_file(
                            ctx,
                            p2,
                            ::core::ptr::null_mut::<file>(),
                        );
                        if *p as i32 == 0 {
                            free(p as *mut ::core::ffi::c_void);
                        } else {
                            p2 = p;
                            files_0 = parse_file_seq::<nameseq>(
                                ctx,
                                &raw mut p2,
                                ::core::mem::size_of::<nameseq>() as size_t,
                                0x1_i32,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                0x2_i32,
                            ) as *mut nameseq;
                            free(p as *mut ::core::ffi::c_void);
                            while let Some(fref) = files_0.as_mut() {
                                let next_0: *mut nameseq = fref.next;
                                let mut name: *const ::core::ffi::c_char = fref.name;
                                let deps: *mut goaldep;
                                let mut f: *mut file;
                                let r: i32;
                                let mut file: file = {
                                    let mut init = File {
                                        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped_suffix: [0; 4],
                                        c2rust_padding: [0; 4],
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
                                    };
                                    init.UpdateStatus = UpdateStatus :: Success;
                                    init.command_state = CommandState :: NotStarted;
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
                                    fatal(
                                        ctx,
                                        &raw mut (*ebuf).floc,
                                        strlen(name) as size_t,
                                        b"%s: failed to load\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[FmtArg::Str((name) as *const ::core::ffi::c_char)],
                                    );
                                }
                                name = file.name;
                                f = lookup_file(name);
                                if f.is_null() {
                                    f = enter_file(name);
                                }
                                f.as_mut()
                                    .expect("eval: null loaded file")
                                    .set_loaded(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                                f.as_mut()
                                    .expect("eval: null loaded file")
                                    .set_unloaded(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                                free_ns(files_0);
                                files_0 = next_0;
                                if r == -1_i32 {
                                    continue;
                                }
                                deps = alloc_goaldep();
                                (*deps).next = read_files;
                                (*deps).floc = (*ebuf).floc;
                                read_files = deps;
                                (*deps).file = f;
                            }
                        }
                    } else {
                        if *line.offset(0_i32 as isize) as i32
                            == crate::make_main::opt_cmd_prefix() as i32
                        {
                            fatal(
                                ctx,
                                fstart,
                                0,
                                b"recipe commences before first target\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[],
                            );
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
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
                                ctx,
                                filenames,
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
                            );
                            filenames = ::core::ptr::null_mut::<NameSeq>();
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
                        collapse_continuations(line);
                        wtype = get_next_mword(line, &raw mut lb_next, &raw mut wlen);
                        match wtype as ::core::ffi::c_uint {
                            1 => {
                                if !cmdleft.is_null() {
                                    fatal(
                                        ctx,
                                        fstart,
                                        0,
                                        b"missing rule before recipe\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[],
                                    );
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
                                );
                                loop {
                                    lb_next = lb_next.offset(wlen as isize);
                                    if cmdleft.is_null() {
                                        cmdleft = find_char_unquote(p2, ';' as i32);
                                        if !cmdleft.is_null() {
                                            let p2_off: size_t = p2.offset_from(variable_buffer)
                                                as ::core::ffi::c_long
                                                as size_t;
                                            let cmd_off: size_t = cmdleft
                                                .offset_from(variable_buffer)
                                                as ::core::ffi::c_long
                                                as size_t;
                                            let pend: *mut ::core::ffi::c_char =
                                                p2.offset(strlen(p2) as isize);
                                            crate::expand::set_variable_buffer_byte(cmd_off, 0);
                                            expand_string_buf(
                                                ctx,
                                                pend,
                                                lb_next,
                                                SIZE_MAX as size_t,
                                            );
                                            lb_next = lb_next.offset(strlen(lb_next) as isize);
                                            p2 = variable_buffer.offset(p2_off as isize);
                                            cmdleft =
                                                variable_buffer.offset(cmd_off as isize).offset(1);
                                        }
                                    }
                                    colonp = find_char_unquote(p2, ':' as i32);
                                    if !colonp.is_null() {
                                        let colon_off: size_t = colonp.offset_from(variable_buffer)
                                            as ::core::ffi::c_long
                                            as size_t;
                                        if colonp > p2
                                            && crate::expand::variable_buffer_byte(colon_off - 1)
                                                as i32
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
                                        p2 = expand_string_buf(ctx, p2, lb_next, wlen);
                                    }
                                }
                                p2 = next_token(variable_buffer);
                                if wtype as ::core::ffi::c_uint
                                    == w_eol as i32 as ::core::ffi::c_uint
                                {
                                    if *p2 as i32 == 0 {
                                        continue;
                                    }
                                    if crate::make_main::opt_cmd_prefix() as i32 == '\t' as i32
                                        && crate::parser::starts_with_eight_spaces(
                                            ::std::ffi::CStr::from_ptr(line).to_bytes(),
                                        )
                                    {
                                        fatal(
                                            ctx,
                                            fstart,
                                            0,
                                            b"missing separator (did you mean TAB instead of 8 spaces?)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
        &[],
    );
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
                                        fatal(
                                            ctx,
                                            fstart,
                                            0,
                                            b"missing separator (ifeq/ifneq must be followed by whitespace)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
        &[],
    );
                                    }
                                    fatal(
                                        ctx,
                                        fstart,
                                        0,
                                        b"missing separator\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[],
                                    );
                                } else {
                                    let colon_off: size_t = colonp.offset_from(variable_buffer)
                                        as ::core::ffi::c_long
                                        as size_t;
                                    let save_0: ::core::ffi::c_char =
                                        crate::expand::variable_buffer_byte(colon_off);
                                    if save_0 as i32 == '&' as i32 {
                                        also_make_targets = 1;
                                    }
                                    crate::expand::set_variable_buffer_byte(colon_off, 0);
                                    filenames = parse_file_seq::<nameseq>(
                                        ctx,
                                        &raw mut p2,
                                        ::core::mem::size_of::<NameSeq>() as size_t,
                                        MAP_NUL,
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        PARSEFS_NONE,
                                    )
                                        as *mut nameseq;
                                    crate::expand::set_variable_buffer_byte(colon_off, save_0);
                                    p2 = colonp
                                        .offset((save_0 as i32 == '&' as i32) as i32 as isize);
                                    if filenames.is_null() {
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
                                            let l_1: size_t = p2.offset_from(variable_buffer)
                                                as ::core::ffi::c_long
                                                as size_t;
                                            plen = strlen(p2) as size_t;
                                            variable_buffer_output(
                                                p2.offset(plen as isize),
                                                lb_next,
                                                (strlen(lb_next) as size_t).wrapping_add(1),
                                            );
                                            p2 = variable_buffer.offset(l_1 as isize);
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
                                                let l_2: size_t = p2.offset_from(variable_buffer)
                                                    as ::core::ffi::c_long
                                                    as size_t;
                                                ::core::slice::from_raw_parts_mut(
                                                    semip as *mut u8,
                                                    1,
                                                )[0] = b';';
                                                collapse_continuations(semip);
                                                variable_buffer_output(
                                                    p2.offset(strlen(p2) as isize),
                                                    semip,
                                                    (strlen(semip) as size_t).wrapping_add(1),
                                                );
                                                p2 = variable_buffer.offset(l_2 as isize);
                                            }
                                            record_target_var(
                                                ctx,
                                                filenames,
                                                p2,
                                                (if vmod.override_v() as i32 != 0 {
                                                    o_override as i32
                                                } else {
                                                    o_file as i32
                                                })
                                                    as variable_origin,
                                                &raw mut vmod,
                                                fstart,
                                            );
                                            filenames = ::core::ptr::null_mut::<NameSeq>();
                                        } else {
                                            find_char_unquote(lb_next, '=' as i32);
                                            prefix = crate::make_main::opt_cmd_prefix();
                                            no_targets = 0;
                                            if *lb_next as i32 != 0 {
                                                let l_3: size_t = p2.offset_from(variable_buffer)
                                                    as ::core::ffi::c_long
                                                    as size_t;
                                                expand_string_buf(
                                                    ctx,
                                                    p2.offset(plen as isize),
                                                    lb_next,
                                                    SIZE_MAX as size_t,
                                                );
                                                p2 = variable_buffer.offset(l_3 as isize);
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
                                                let target: *mut NameSeq;
                                                target = parse_file_seq::<nameseq>(
                                                    ctx,
                                                    &raw mut p2,
                                                    ::core::mem::size_of::<nameseq>() as size_t,
                                                    0x40_i32,
                                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                                    0x4_i32,
                                                )
                                                    as *mut nameseq;
                                                p2 = p2.offset(1_i32 as isize);
                                                if target.is_null() {
                                                    fatal(
                                                        ctx,
                                                        fstart,
                                                        0,
                                                        b"missing target pattern\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        &[],
                                                    );
                                                } else if !(*target).next.is_null() {
                                                    fatal(
                                                        ctx,
                                                        fstart,
                                                        0,
                                                        b"multiple target patterns\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        &[],
                                                    );
                                                }
                                                pattern_percent =
                                                    find_percent_cached(&raw mut (*target).name);
                                                pattern = (*target).name;
                                                if pattern_percent.is_null() {
                                                    fatal(
                                                        ctx,
                                                        fstart,
                                                        0,
                                                        b"target pattern contains no '%%'\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        &[],
                                                    );
                                                }
                                                free_ns(target);
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
                                            check_specials(ctx, filenames, set_default);
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
    if (*conditionals).if_cmds != 0 {
        fatal(
            ctx,
            fstart,
            0,
            b"missing 'endif'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !filenames.is_null() {
        fi.lineno = tgts_started as ::core::ffi::c_ulong;
        fi.offset = 0;
        record_files(
            ctx,
            filenames,
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
        );
    }
    free(collapsed as *mut ::core::ffi::c_void);
    drop(cmd_buf);
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
    ebuf: *mut ebuffer,
) {
    let var: *mut ::core::ffi::c_char =
        allocated_expand_string_for_file(ctx, name, ::core::ptr::null_mut::<file>());
    // Isolate the variable name (skip leading blanks, trim trailing blanks) via
    // the typed AST layer; an empty name is fatal.
    let span = match crate::parser::trimmed_token(::std::ffi::CStr::from_ptr(var).to_bytes()) {
        Some(s) => s,
        None => fatal(
            ctx,
            &raw mut (*ebuf).floc,
            0,
            b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char,
        ),
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
    );
    free(var as *mut ::core::ffi::c_void);
}
unsafe fn do_define(
    ctx: &crate::execctx::ExecContext,
    mut name: *mut ::core::ffi::c_char,
    origin: variable_origin,
    ebuf: *mut ebuffer,
) -> *mut variable {
    let v: *mut variable;
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
    let n: *mut ::core::ffi::c_char;
    defstart = (*ebuf).floc;
    p = parse_variable_definition(name, &raw mut var);
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
    n = allocated_expand_string_for_file(ctx, name, ::core::ptr::null_mut::<file>());
    // Isolate the variable name (skip leading blanks, trim trailing blanks) via
    // the typed AST layer; an empty name is fatal.
    let span = match crate::parser::trimmed_token(::std::ffi::CStr::from_ptr(n).to_bytes()) {
        Some(s) => s,
        None => fatal(
            ctx,
            &raw mut defstart,
            0,
            b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    };
    name = n.add(span.start);
    *n.add(span.end) = 0;
    loop {
        let line: *mut ::core::ffi::c_char;
        let nlines: ::core::ffi::c_long = readline(ctx, ebuf);
        if nlines < 0 {
            fatal(
                ctx,
                &raw mut defstart,
                0,
                b"missing 'endef', unterminated 'define'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        (*ebuf).floc.lineno = (*ebuf)
            .floc
            .lineno
            .wrapping_add(nlines as ::core::ffi::c_ulong);
        line = (*ebuf).buffer;
        collapse_continuations(line);
        if *line.offset(0_i32 as isize) as i32 != crate::make_main::opt_cmd_prefix() as i32 {
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
    v = do_variable_definition(
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
    v
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
) -> i32 {
    let cmdname: *const ::core::ffi::c_char;
    let cmdtype: C2RustUnnamed;
    let mut i: ::core::ffi::c_uint;
    let o: ::core::ffi::c_uint;
    // Classify the directive keyword (the line's first `len` bytes) via the
    // typed AST layer instead of a wall of `strncmp`/`size_of` comparisons.
    let directive =
        crate::parser::Directive::from_word(::core::slice::from_raw_parts(line as *const u8, len));
    match directive {
        Some(d) => {
            cmdtype = directive_cmdtype(d);
            cmdname = d.name().as_ptr();
        }
        None => return -2_i32,
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
        if (*conditionals).if_cmds == 0 {
            fatal(
                ctx,
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                cmdname,
            );
        }
        (*conditionals).if_cmds = (*conditionals).if_cmds.wrapping_sub(1);
    } else if cmdtype as ::core::ffi::c_uint == c_else as i32 as ::core::ffi::c_uint {
        let mut p: *const ::core::ffi::c_char;
        if (*conditionals).if_cmds == 0 {
            fatal(
                ctx,
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                cmdname,
            );
        }
        o = (*conditionals).if_cmds.wrapping_sub(1);
        if *(*conditionals).seen_else.offset(o as isize) != 0 {
            fatal(
                ctx,
                flocp,
                0,
                b"only one 'else' per conditional\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        match *(*conditionals).ignoring.offset(o as isize) as i32 {
            0 => {
                *(*conditionals).ignoring.offset(o as isize) = 2;
            }
            1 => {
                *(*conditionals).ignoring.offset(o as isize) = 0;
            }
            _ => {}
        }
        if *line as i32 == 0 {
            *(*conditionals).seen_else.offset(o as isize) = 1;
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
            if matches!(
                next,
                Some(crate::parser::Directive::Else | crate::parser::Directive::Endif)
            ) || conditional_line(ctx, line, len, flocp, 0) < 0
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
                if (*(*conditionals).ignoring.offset(o as isize) as i32) < 2 {
                    *(*conditionals).ignoring.offset(o as isize) =
                        *(*conditionals).ignoring.offset(o.wrapping_add(1) as isize);
                }
                (*conditionals).if_cmds = (*conditionals).if_cmds.wrapping_sub(1);
            }
        }
    } else {
        if (*conditionals).allocated == 0 {
            (*conditionals).allocated = 5;
            (*conditionals).ignoring =
                xmalloc((*conditionals).allocated as size_t) as *mut ::core::ffi::c_char;
            (*conditionals).seen_else =
                xmalloc((*conditionals).allocated as size_t) as *mut ::core::ffi::c_char;
        }
        let fresh26 = (*conditionals).if_cmds;
        (*conditionals).if_cmds = (*conditionals).if_cmds.wrapping_add(1);
        o = fresh26;
        if (*conditionals).if_cmds > (*conditionals).allocated {
            (*conditionals).allocated = (*conditionals).allocated.wrapping_add(5);
            (*conditionals).ignoring = xrealloc(
                (*conditionals).ignoring as *mut ::core::ffi::c_void,
                (*conditionals).allocated as size_t,
            ) as *mut ::core::ffi::c_char;
            (*conditionals).seen_else = xrealloc(
                (*conditionals).seen_else as *mut ::core::ffi::c_void,
                (*conditionals).allocated as size_t,
            ) as *mut ::core::ffi::c_char;
        }
        *(*conditionals).seen_else.offset(o as isize) = 0;
        i = 0;
        while i < o {
            if *(*conditionals).ignoring.offset(i as isize) != 0 {
                *(*conditionals).ignoring.offset(o as isize) = 1;
                return 1;
            }
            i = i.wrapping_add(1);
        }
        if cmdtype as ::core::ffi::c_uint == c_ifdef as i32 as ::core::ffi::c_uint
            || cmdtype as ::core::ffi::c_uint == c_ifndef as i32 as ::core::ffi::c_uint
        {
            let v: *mut variable;
            let var: *mut ::core::ffi::c_char =
                allocated_expand_string_for_file(ctx, line, ::core::ptr::null_mut::<file>());
            // The condition is a single variable name: take the lone token (a
            // trailing second token is a syntax error) via the typed AST layer,
            // replacing the `end_of_token` + manual whitespace scan.
            let l: size_t =
                match crate::parser::lone_token(::std::ffi::CStr::from_ptr(var).to_bytes()) {
                    Some(l) => l as size_t,
                    None => return -1_i32,
                };
            *var.add(l) = 0;
            v = lookup_variable(ctx, var, l);
            *(*conditionals).ignoring.offset(o as isize) =
                ((!v.is_null() && *(*v).value as i32 != 0) as i32
                    == (cmdtype as ::core::ffi::c_uint == c_ifndef as i32 as ::core::ffi::c_uint)
                        as i32) as i32 as ::core::ffi::c_char;
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
            let expand_arg = |range: ::core::ops::Range<usize>| -> Vec<u8> {
                *line.add(range.end) = 0;
                let p = expand_string_buf(
                    ctx,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    line.add(range.start),
                    SIZE_MAX as size_t,
                );
                ::std::ffi::CStr::from_ptr(p).to_bytes().to_vec()
            };
            match crate::parser::parse_conditional_args(::std::ffi::CStr::from_ptr(line).to_bytes())
            {
                ConditionalArgs::Error => return -1_i32,
                ConditionalArgs::FirstArgOnly { arg1 } => {
                    // make expands the first argument (for its side effects)
                    // before reporting the second-argument syntax error.
                    expand_arg(arg1);
                    return -1_i32;
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
                            cmdname,
                        );
                    }
                    // Expand the first argument to an owned string before
                    // expanding the second (they share one scratch buffer).
                    let a1 = expand_arg(arg1);
                    let a2 = expand_arg(arg2);
                    *(*conditionals).ignoring.offset(o as isize) = ((a1 == a2)
                        == (cmdtype as ::core::ffi::c_uint
                            == c_ifneq as i32 as ::core::ffi::c_uint))
                        as i32
                        as ::core::ffi::c_char;
                }
            }
        }
    }
    i = 0;
    while i < (*conditionals).if_cmds {
        if *(*conditionals).ignoring.offset(i as isize) != 0 {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}
unsafe fn record_target_var(
    ctx: &crate::execctx::ExecContext,
    mut filenames: *mut nameseq,
    defn: *mut ::core::ffi::c_char,
    origin: variable_origin,
    vmod: *mut vmodifiers,
    flocp: *const Floc,
) {
    let mut nextf: *mut NameSeq;
    let global: *mut variable_set_list;
    global = current_variable_set_list;
    while !filenames.is_null() {
        let v: *mut variable;
        let mut name: *const ::core::ffi::c_char = (*filenames).name;
        let percent: *const ::core::ffi::c_char;
        let p: *mut pattern_var;
        nextf = (*filenames).next;
        free_ns(filenames);
        percent = find_percent_cached(&raw mut name);
        if !percent.is_null() {
            p = create_pattern_var(name, percent);
            (*p).variable.fileinfo = *flocp;
            v = assign_variable_definition(ctx, &raw mut (*p).variable, defn);
            let vref = v.as_mut().expect("assertion failed: v != 0");
            vref.set_origin(origin as variable_origin);
            if vref.flavor() as i32 == f_simple as i32 {
                vref.value = allocated_expand_string_for_file(
                    ctx,
                    vref.value,
                    ::core::ptr::null_mut::<file>(),
                );
            } else {
                vref.value = xstrdup(vref.value);
            }
        } else {
            let mut f: *mut File;
            f = lookup_file(name);
            if f.is_null() {
                f = enter_file(strcache_add(name));
            } else if let Some(fref) = f.as_ref().filter(|x| !x.double_colon.is_null()) {
                f = fref.double_colon;
            }
            initialize_file_variables(
                ctx,
                ::core::ptr::NonNull::new(f)
                    .expect("record_target_var: null file")
                    .as_ptr(),
                1,
            );
            current_variable_set_list = f.as_ref().expect("record_target_var: null file").variables;
            v = try_variable_definition(ctx, flocp, defn, origin, s_target);
            if v.is_null() {
                fatal(
                    ctx,
                    flocp,
                    0,
                    b"malformed target-specific variable definition\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
            current_variable_set_list = global;
        }
        let vref = v.as_mut().expect("record_target_var: null variable");
        vref.set_per_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        vref.set_private_var((*vmod).private_v() as ::core::ffi::c_uint);
        if (*vmod).export_v() as i32 != v_default as i32 {
            vref.set_export((*vmod).export_v() as variable_export);
        }
        if vref.origin() as i32 != o_override as i32 {
            let len: size_t = strlen(vref.name) as size_t;
            let gv: *mut variable = lookup_variable(ctx, vref.name, len);
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
        filenames = nextf;
    }
}
/// The name of a dependency as a byte slice: the `name` field if set, else the
/// linked `file`'s name. Used by the suffix-rule check in [`check_specials`] to
/// compare names as slices rather than via raw `c_char` pointers.
///
/// # Safety
/// `dp` must be a valid `dep` whose `name`/`file` pointers are valid NUL-
/// terminated C strings; the returned slice borrows that C string.
unsafe fn dep_name_bytes<'a>(dp: *mut dep) -> &'a [u8] {
    ::std::ffi::CStr::from_ptr(if !(*dp).name.is_null() {
        (*dp).name
    } else {
        (*(*dp).file).name
    })
    .to_bytes()
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn check_specials(
    ctx: &crate::execctx::ExecContext,
    files: *mut nameseq,
    set_default: i32,
) {
    let mut t: *mut nameseq;
    t = files;
    while !t.is_null() {
        let nm: *const ::core::ffi::c_char = (*t).name;
        let special =
            crate::parser::SpecialTarget::from_name(::std::ffi::CStr::from_ptr(nm).to_bytes());
        if !posix_pedantic() && special == Some(crate::parser::SpecialTarget::Posix) {
            crate::make_main::set_posix_pedantic();
            define_variable_in_set(
                ctx,
                b".SHELLFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t).wrapping_sub(1),
                b"-ec\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                ctx,
                b"CC\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
                b"c99\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                ctx,
                b"CFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
                b"-O1\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                ctx,
                b"FC\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
                b"fort77\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                ctx,
                b"FFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
                b"-O1\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                ctx,
                b"SCCSGETFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
                b"-s\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                ctx,
                b"ARFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1),
                b"-rv\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
        } else if !second_expansion()
            && special == Some(crate::parser::SpecialTarget::SecondExpansion)
        {
            crate::make_main::set_second_expansion();
        } else if !one_shell() && special == Some(crate::parser::SpecialTarget::OneShell) {
            crate::make_main::set_one_shell();
        } else if set_default != 0 && *(*default_goal_var).value.offset(0) as i32 == 0 {
            let mut d: *mut dep;
            let mut reject: i32 = 0;
            // Pattern targets (containing `%`) are never the default goal; test
            // the name as a byte slice via CStr rather than strchr.
            let nm_bytes = ::std::ffi::CStr::from_ptr(nm).to_bytes();
            if nm_bytes.contains(&b'%') {
                break;
            }
            if !(nm_bytes.first() == Some(&b'.') && !nm_bytes.contains(&b'/')) {
                d = (*suffix_file).deps;
                while !d.is_null() {
                    // A target is a suffix rule (and so must not become the
                    // default goal) when its name is itself a known suffix, or
                    // the concatenation of two known suffixes (e.g. `.c` + `.o`
                    // => `.c.o`). Compare names as byte slices via CStr rather
                    // than the c2rust first-char + strcmp / strncmp idioms.
                    let dname = dep_name_bytes(d);
                    if dname.first() != Some(&b'.') && nm_bytes == dname {
                        reject = 1;
                        break;
                    }
                    let mut d2: *mut dep = (*suffix_file).deps;
                    while !d2.is_null() {
                        if nm_bytes.strip_prefix(dep_name_bytes(d2)) == Some(dname) {
                            reject = 1;
                            break;
                        }
                        d2 = (*d2).next;
                    }
                    if reject != 0 {
                        break;
                    }
                    d = (*d).next;
                }
                if reject == 0 {
                    define_variable_in_set(
                        ctx,
                        b".DEFAULT_GOAL\0" as *const u8 as *const ::core::ffi::c_char,
                        13,
                        (*t).name,
                        o_file,
                        0,
                        ::core::ptr::null_mut::<variable_set>(),
                        ::core::ptr::null_mut::<Floc>(),
                    );
                }
            }
        }
        t = (*t).next;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn check_special_file(
    ctx: &crate::execctx::ExecContext,
    file: *mut file,
    flocp: *const Floc,
) {
    if crate::parser::is_wait_token(::std::ffi::CStr::from_ptr((*file).name).to_bytes()) {
        use std::sync::atomic::{AtomicBool, Ordering};
        static WPRE: AtomicBool = AtomicBool::new(false);
        static WCMD: AtomicBool = AtomicBool::new(false);
        if !WPRE.load(Ordering::Relaxed) && !(*file).deps.is_null() {
            error(
                ctx,
                flocp,
                0,
                b".WAIT should not have prerequisites\0" as *const u8 as *const ::core::ffi::c_char,
            );
            WPRE.store(true, Ordering::Relaxed);
        }
        if !WCMD.load(Ordering::Relaxed) && !(*file).cmds.is_null() {
            error(
                ctx,
                flocp,
                0,
                b".WAIT should not have commands\0" as *const u8 as *const ::core::ffi::c_char,
            );
            WCMD.store(true, Ordering::Relaxed);
        }
    }
}
#[allow(clippy::too_many_arguments)]
unsafe fn record_files(
    ctx: &crate::execctx::ExecContext,
    filenames: *mut nameseq,
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
) {
    let cmds: *mut Commands;
    let mut deps: *mut Dep;
    let mut also_make: *mut Dep = ::core::ptr::null_mut::<Dep>();
    let mut implicit_percent: *const ::core::ffi::c_char;
    let mut name: *const ::core::ffi::c_char;
    if opt_snapped_deps() {
        fatal(
            ctx,
            flocp,
            0,
            b"prerequisites cannot be defined in recipes\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    let mut filenames = ::core::ptr::NonNull::new(filenames)
        .expect("record_files: null filenames")
        .as_ptr();
    name = (*filenames).name;
    implicit_percent = find_percent_cached(&raw mut name);
    if commands_idx > 0 {
        cmds = xmalloc(::core::mem::size_of::<commands>() as size_t) as *mut commands;
        let cmdsref = cmds.as_mut().expect("record_files: null cmds");
        cmdsref.fileinfo.filenm = (*flocp).filenm;
        cmdsref.fileinfo.lineno = cmds_started as ::core::ffi::c_ulong;
        cmdsref.fileinfo.offset = 0;
        cmdsref.commands = xstrndup(commands, commands_idx);
        cmdsref.command_lines = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        cmdsref.recipe_prefix = prefix;
    } else if are_also_makes != 0 {
        fatal(
            ctx,
            flocp,
            0,
            b"grouped targets must provide a recipe\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        cmds = ::core::ptr::null_mut::<Commands>();
    }
    if depstr.is_null() {
        deps = ::core::ptr::null_mut::<Dep>();
    } else {
        depstr = unescape_char(depstr, ':' as i32);
        if second_expansion()
            && crate::parser::prereq_needs_second_expansion(
                ::std::ffi::CStr::from_ptr(depstr).to_bytes(),
            )
        {
            deps = alloc_dep();
            (*deps).name = depstr;
            (*deps).need_2nd_expansion = true;
            (*deps).set_staticpattern(
                (pattern != ::core::ptr::null::<::core::ffi::c_char>()) as i32
                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        } else {
            deps = split_prereqs(ctx, depstr);
            free(depstr as *mut ::core::ffi::c_void);
            if pattern.is_null() && implicit_percent.is_null() {
                deps = enter_prereqs(deps, ::core::ptr::null::<::core::ffi::c_char>());
            }
        }
    }
    if !implicit_percent.is_null() {
        let targets: *mut *const ::core::ffi::c_char;
        let target_pats: *mut *const ::core::ffi::c_char;
        let mut c: ::core::ffi::c_ushort;
        if !pattern.is_null() {
            fatal(
                ctx,
                flocp,
                0,
                b"mixed implicit and static pattern rules\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        let first_target = pop_name_seq(filenames, "record_files target list is null");
        filenames = first_target.next;
        c = name_seq_len(filenames).wrapping_add(1);
        targets = xmalloc(
            (c as size_t)
                .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
        ) as *mut *const ::core::ffi::c_char;
        target_pats = xmalloc(
            (c as size_t)
                .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
        ) as *mut *const ::core::ffi::c_char;
        let fresh17 = &mut (*targets.offset(0_i32 as isize));
        *fresh17 = name;
        let fresh18 = &mut (*target_pats.offset(0_i32 as isize));
        *fresh18 = implicit_percent;
        c = 1;
        while !filenames.is_null() {
            let target = pop_name_seq(filenames, "record_files target list is null");
            name = target.name;
            implicit_percent = find_percent_cached(&raw mut name);
            if implicit_percent.is_null() {
                fatal(
                    ctx,
                    flocp,
                    0,
                    b"mixed implicit and normal rules\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            let fresh19 = &mut (*targets.offset(c as isize));
            *fresh19 = name;
            let fresh20 = &mut (*target_pats.offset(c as isize));
            *fresh20 = implicit_percent;
            c = c.wrapping_add(1);
            filenames = target.next;
        }
        create_pattern_rule(targets, target_pats, c, two_colon, deps, cmds, 1);
        return;
    }
    while !filenames.is_null() {
        let target = pop_name_seq(filenames, "record_files target list is null");
        let nextf_0: *mut NameSeq = target.next;
        let mut f: *mut File;
        let mut this: *mut Dep = ::core::ptr::null_mut::<Dep>();
        if !pattern.is_null() && pattern_matches(pattern, pattern_percent, name) == 0 {
            error(
                ctx,
                flocp,
                strlen(name) as size_t,
                b"target '%s' doesn't match the target pattern\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str((name) as *const ::core::ffi::c_char)],
            );
        } else if !deps.is_null() {
            this = if !nextf_0.is_null() {
                copy_dep_chain(deps)
            } else {
                deps
            };
        }
        if two_colon == 0 {
            f = enter_file(strcache_add(name));
            if !(*f).double_colon.is_null() {
                fatal(
                    ctx,
                    flocp,
                    strlen((*f).name) as size_t,
                    b"target file '%s' has both : and :: entries\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(((*f).name) as *const ::core::ffi::c_char)],
                );
            }
            if !cmds.is_null() && cmds == (*f).cmds {
                error(
                    ctx,
                    flocp,
                    strlen((*f).name) as size_t,
                    b"target '%s' given more than once in the same rule\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*f).name,
                );
            } else if !cmds.is_null() && !(*f).cmds.is_null() && (*f).is_target() as i32 != 0 {
                let l: size_t = strlen((*f).name) as size_t;
                error(
                    ctx,
                    &raw mut (*cmds).fileinfo,
                    l,
                    b"warning: overriding recipe for target '%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(((*f).name) as *const ::core::ffi::c_char)],
                );
                error(
                    ctx,
                    &raw mut (*(*f).cmds).fileinfo,
                    l,
                    b"warning: ignoring old recipe for target '%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(((*f).name) as *const ::core::ffi::c_char)],
                );
            }
            if f == default_file && this.is_null() && cmds.is_null() {
                (*f).cmds = ::core::ptr::null_mut::<Commands>();
            }
            if !cmds.is_null() {
                (*f).cmds = cmds;
            }
            if f == suffix_file && this.is_null() {
                free_dep_chain((*f).deps);
                (*f).deps = ::core::ptr::null_mut::<Dep>();
            }
        } else {
            f = lookup_file(name);
            if let Some(fref) = f
                .as_ref()
                .filter(|x| x.is_target() as i32 != 0 && x.double_colon.is_null())
            {
                fatal(
                    ctx,
                    flocp,
                    strlen(fref.name) as size_t,
                    b"target file '%s' has both : and :: entries\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fref.name,
                );
            }
            f = enter_file(strcache_add(name));
            if (*f).double_colon.is_null() {
                (*f).double_colon = f;
            }
            (*f).cmds = cmds;
        }
        (*f).is_explicit = true;
        if are_also_makes != 0 {
            let also: *mut Dep = alloc_dep();
            (*also).name = (*f).name;
            (*also).file = f;
            (*also).next = also_make;
            also_make = also;
        }
        // Checked view of the target file for the updates below.
        let fr = f.as_mut().expect("record_files: null target file");
        fr.set_is_target(1);
        if !pattern.is_null() {
            static mut percent: *const ::core::ffi::c_char =
                b"%\0" as *const u8 as *const ::core::ffi::c_char;
            let o: *mut ::core::ffi::c_char = patsubst_expand_pat(
                variable_buffer,
                name,
                pattern,
                percent,
                pattern_percent.offset(1_i32 as isize),
                percent.offset(1_i32 as isize),
            );
            fr.stem = strcache_add_len(
                variable_buffer,
                o.offset_from(variable_buffer) as ::core::ffi::c_long as size_t,
            );
            if let Some(thisr) = this.as_mut() {
                if thisr.need_2nd_expansion() == 0 {
                    this = enter_prereqs(this, fr.stem);
                } else {
                    thisr.stem = fr.stem;
                }
            }
        }
        let f_ref = f
            .as_mut()
            .expect("record_files target lookup returned a null file");
        if !this.is_null() {
            if fr.deps.is_null() {
                fr.deps = this;
            } else if !cmds.is_null() {
                let mut d: *mut dep = this;
                while let Some(dr) = d.as_mut() {
                    if dr.next.is_null() {
                        dr.next = fr.deps;
                        break;
                    }
                    d = dr.next;
                }
                fr.deps = this;
            } else {
                let mut d_0: *mut dep = fr.deps;
                while let Some(d0r) = d_0.as_mut() {
                    if d0r.next.is_null() {
                        d0r.next = this;
                        break;
                    }
                    d_0 = d0r.next;
                }
            }
        }
        check_special_file(ctx, f, flocp);
        if nextf_0.is_null() {
            break;
        }
        filenames = nextf_0;
        name = filenames
            .as_ref()
            .expect("record_files target list is null")
            .name;
        if !find_percent_cached(&raw mut name).is_null() {
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
    let mut i: *mut Dep = also_make;
    while let Some(node) = i.as_ref() {
        let f_0: *mut file = node.file;
        let f0 = f_0
            .as_mut()
            .expect("record_files: null also-make target file");
        if !f0.also_make.is_null() {
            error(
                ctx,
                &raw mut cmds
                    .as_mut()
                    .expect("record_files: null cmds in group warning")
                    .fileinfo,
                strlen(f0.name) as size_t,
                b"warning: overriding group membership for target '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str(f0.name)],
            );
            free_dep_chain(f0.also_make);
            f0.also_make = ::core::ptr::null_mut::<dep>();
        }
        let mut dp: *mut dep = also_make;
        while let Some(dep_ref) = dp.as_ref() {
            if dep_ref.file != f_0 {
                let cpy: *mut dep = copy_dep(dp);
                if let Some(c) = cpy.as_mut() {
                    c.next = f0.also_make;
                    f0.also_make = cpy;
                }
            }
            dp = dep_ref.next;
        }
        i = node.next;
    }
    free_dep_chain(also_make);
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
            let cached = strcache_add(buf.as_ptr() as *const ::core::ffi::c_char);
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
            error(
                ctx,
                &raw mut (*ebuf).floc,
                0,
                b"warning: NUL character seen; rest of line ignored\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
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
/// # Safety
///
/// Calls into the C variable machinery (`do_variable_definition`); must run
/// single-threaded like the rest of startup. The resolved search path is then
/// stored in `main_0`'s owned `Options` via the `with_options` borrow channel,
/// not in any process-global mutable state.
pub unsafe fn construct_include_path(
    ctx: &crate::execctx::ExecContext,
    arg_dirs: &[std::path::PathBuf],
) {
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
        for d in DEFAULT_INCLUDE_DIRECTORIES {
            push_include_dir(&mut dirs, d);
        }
    }
    do_variable_definition(
        ctx,
        NILF,
        b".INCLUDE_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        f_simple,
        0,
        s_global,
    );
    for dir in &dirs {
        // Intern the path bytes to obtain a canonical, cache-owned pointer for
        // the C variable machinery; no CString/manual NUL constructed here.
        let value = crate::strcache::strcache_add_bytes(dir.as_os_str().as_bytes());
        do_variable_definition(
            ctx,
            NILF,
            b".INCLUDE_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
            value,
            o_default,
            f_append,
            0,
            s_global,
        );
    }
    crate::make_main::with_options(|o| {
        *o.resolved_include_dirs.borrow_mut() = dirs;
    });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn tilde_expand(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *name.offset(1_i32 as isize) as i32 == '/' as i32 || *name.offset(1_i32 as isize) as i32 == 0
    {
        let mut home_dir: *mut ::core::ffi::c_char;
        let is_variable: i32;
        let save: Action = warning::action(Type::UndefinedVar);
        warning::set_action(Type::UndefinedVar, Action::Ignore);
        home_dir = allocated_expand_variable(
            ctx,
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
        );
        warning::set_action(Type::UndefinedVar, save);
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
            let new: *mut ::core::ffi::c_char =
                xstrdup(concat(2, home_dir, name.offset(1_i32 as isize)));
            if is_variable != 0 {
                free(home_dir as *mut ::core::ffi::c_void);
            }
            return new;
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
                None => return xstrdup((*pwent).pw_dir),
                // `~user/suffix` — home + the `/suffix` tail (the byte at `i` is
                // the `/`, so the tail after it starts at `1 + i + 1`).
                Some(i) => {
                    return xstrdup(concat(
                        3,
                        (*pwent).pw_dir,
                        b"/\0" as *const u8 as *const ::core::ffi::c_char,
                        name.add(1 + i + 1),
                    ));
                }
            }
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn parse_file_seq<T: SeqNode>(
    ctx: &crate::execctx::ExecContext,
    stringp: *mut *mut ::core::ffi::c_char,
    mut size: size_t,
    mut stopmap: i32,
    prefix: *const ::core::ffi::c_char,
    flags: i32,
) -> *mut T {
    static mut tmpbuf: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    let cachep: i32 = !(flags & 0x10_i32 != 0) as i32;
    let mut new: *mut T = ::core::ptr::null_mut::<T>();
    let mut newp: *mut *mut T = &raw mut new;
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
    let mut found_wait: i32 = 0;
    if !(flags & 0x20_i32 != 0) {
        findmap |= MAP_BLANK;
    }
    stopmap |= MAP_NUL;
    if size < ::core::mem::size_of::<NameSeq>() as usize {
        size = ::core::mem::size_of::<NameSeq>() as usize as size_t;
    }
    if !(flags & 0x4_i32 != 0) {
        // read.rs carries its own layout-identical glob_t; reconcile the
        // nominal types until the duplicate struct is unified.
        dir_setup_glob((&raw mut gl).cast());
    }
    static mut tmpbuf_len: size_t = 0;
    let l: size_t = (strlen(*stringp.as_ref().expect("parse_file_seq: null stringp")) as size_t)
        .wrapping_add(1);
    if l > tmpbuf_len {
        tmpbuf = xrealloc(tmpbuf as *mut ::core::ffi::c_void, l) as *mut ::core::ffi::c_char;
        tmpbuf_len = l;
    }
    tp = tmpbuf;
    p = *stringp.as_ref().expect("parse_file_seq: null stringp");
    loop {
        let mut name: *const ::core::ffi::c_char;
        let mut nlist: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut tildep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut globme: i32 = 1;
        let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
                let mut _ns: *mut T = T::alloc();
                let mut __n: *const ::core::ffi::c_char = concat(2, prefix, tmpbuf);
                let ns = _ns
                    .as_mut()
                    .expect("parse_file_seq: xcalloc returned null nameseq");
                ns.name = if cachep != 0 {
                    strcache_add(__n)
                } else {
                    xstrdup(__n) as *const ::core::ffi::c_char
                };
                if found_wait != 0 {
                    T::mark_wait(_ns);
                    found_wait = 0;
                }
                *newp = _ns;
                newp = T::next_slot(_ns);
            } else {
                name = tmpbuf;
                if *tmpbuf.offset(0_i32 as isize) as i32 == '~' as i32 {
                    tildep = tilde_expand(ctx, tmpbuf);
                    if !tildep.is_null() {
                        name = tildep;
                    }
                }
                if !(flags & 0x2_i32 != 0) && ar_name(ctx, CStr::from_ptr(name)) {
                    ar_parse_name(ctx, name, &raw mut arname, &raw mut memname);
                    name = arname;
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
                        let mut found: *mut T =
                            ar_glob::<T>(ctx, *nlist.offset(i as isize), memname);
                        if found.is_null() {
                            let _ns_0: *mut T = T::alloc();
                            let __n_0: *const ::core::ffi::c_char = concat(&[
                                prefix,
                                *nlist.offset(i as isize),
                                b"(\0" as *const u8 as *const ::core::ffi::c_char,
                                memname,
                                b")\0" as *const u8 as *const ::core::ffi::c_char,
                            ]);
                            T::set_name(
                                _ns_0,
                                if cachep != 0 {
                                    strcache_add(__n_0)
                                } else {
                                    xstrdup(__n_0) as *const ::core::ffi::c_char
                                },
                            );
                            if found_wait != 0 {
                                T::mark_wait(_ns_0);
                                found_wait = 0;
                            }
                            *newp = _ns_0;
                            newp = T::next_slot(_ns_0);
                        } else {
                            if let Some(node) = newp.as_mut().and_then(|s| s.as_mut()) {
                                node.next = found;
                            } else {
                                *newp = found;
                            }
                            loop {
                                if cachep == 0 {
                                    T::set_name(found, xstrdup(concat(&[prefix, name])));
                                } else if !prefix.is_null() {
                                    T::set_name(found, strcache_add(concat(&[prefix, name])));
                                }
                                if T::next(found).is_null() {
                                    newp = T::next_slot(found);
                                    break;
                                }
                                found = T::next(found);
                            }
                        }
                    } else {
                        let _ns_1: *mut T = T::alloc();
                        let __n_1: *const ::core::ffi::c_char =
                            concat(&[prefix, *nlist.offset(i as isize)]);
                        T::set_name(
                            _ns_1,
                            if cachep != 0 {
                                strcache_add(__n_1)
                            } else {
                                xstrdup(__n_1) as *const ::core::ffi::c_char
                            },
                        );
                        if found_wait != 0 {
                            T::mark_wait(_ns_1);
                            found_wait = 0;
                        }
                        *newp = _ns_1;
                        newp = T::next_slot(_ns_1);
                    }
                    i += 1;
                }
                if globme != 0 {
                    globfree(&raw mut gl);
                }
                free(arname as *mut ::core::ffi::c_void);
                free(tildep as *mut ::core::ffi::c_void);
            }
        }
    }
    *stringp = p;
    new
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
