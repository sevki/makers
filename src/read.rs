pub use crate::expand::allocated_expand_string_for_file;
pub use crate::file::enter_file;
pub use crate::file::enter_prereqs;
pub use crate::file::lookup_file;
pub use crate::file::split_prereqs;
pub use crate::load::load_file;
pub use crate::make_main::default_file;
pub use crate::rule::create_pattern_rule;
pub use crate::rule::suffix_file;
pub use crate::variable::initialize_file_variables;
use crate::ar::ar_glob;
use crate::file::SeqNode;
pub use crate::file::{CommandState, UpdateStatus};
pub use crate::ffi_types::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off64_t, __off_t,
    __size_t, __syscall_slong_t, __time_t, __uid_t, size_t, uintmax_t,
};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
use crate::misc::{
    collapse_continuations, copy_dep, copy_dep_chain, end_of_token, find_next_token, next_token,
    skip_reference, xcalloc, xmalloc, xrealloc, xstrdup, xstrndup,
};
use crate::stdio::FILE;
use crate::strcache::{strcache_add, strcache_add_len};
use c2rust_bitfields;
use libc::{
    __errno_location, free, getenv, getlogin, printf, puts, strchr, strcmp, strcpy, strerror,
    strpbrk,
};
extern "C" {
    pub type dirent;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn glob(
        __pattern: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
        __errfunc: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> ::core::ffi::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn getpwnam(__name: *const ::core::ffi::c_char) -> *mut passwd;
}
pub use crate::sys_stat::stat;
pub use crate::sys_stat::timespec;
use crate::warning::{self, Action, Type};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut ::core::ffi::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: ::core::ffi::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut dirent>,
    pub gl_opendir:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>,
    pub gl_lstat:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
    pub gl_stat:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
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
use crate::file::{enter_file, enter_prereqs, lookup_file, snapped_deps, split_prereqs};
use crate::function::{patsubst_expand_pat, pattern_matches, strip_whitespace};
use crate::load::load_file;
use crate::make_main::{
    cmd_prefix, db_level, default_file, default_goal_var, export_all_variables, one_shell,
    posix_pedantic, second_expansion, stopchar_map,
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
    pub error: ::core::ffi::c_int,
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
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4;
pub const ENOMEM: ::core::ffi::c_int = 12;
pub const ENFILE: ::core::ffi::c_int = 23;
pub const EMFILE: ::core::ffi::c_int = 24;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_NUL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_BLANK: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_COMMENT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MAP_SEMI: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MAP_VARIABLE: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const MAP_VMSCOMMA: ::core::ffi::c_int = 0;
pub const GLOB_ALTDIRFUNC: ::core::ffi::c_int = (1) << 9;
pub const GLOB_NOSPACE: ::core::ffi::c_int = 1;
pub const GLOB_NOMATCH: ::core::ffi::c_int = 3;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const RM_NO_DEFAULT_GOAL: ::core::ffi::c_int = (1) << 0;
pub const RM_INCLUDED: ::core::ffi::c_int = (1) << 1;
pub const RM_DONTCARE: ::core::ffi::c_int = (1) << 2;
pub const RM_NO_TILDE: ::core::ffi::c_int = (1) << 3;
pub const PARSEFS_NONE: ::core::ffi::c_int = 0;
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
#[inline]
unsafe extern "C" fn free_dep_chain(d: *mut Dep) {
    crate::file::free_seq_chain(d);
}
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
static mut toplevel_conditionals: conditionals = conditionals {
    if_cmds: 0,
    allocated: 0,
    ignoring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    seen_else: ::core::ptr::null_mut::<::core::ffi::c_char>(),
};
static mut conditionals: *mut conditionals = &raw const toplevel_conditionals as *mut conditionals;
static mut default_include_directories: [*const ::core::ffi::c_char; 4] = [
    b"/usr/gnu/include\0" as *const u8 as *const ::core::ffi::c_char,
    b"/usr/local/include\0" as *const u8 as *const ::core::ffi::c_char,
    b"/usr/include\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut include_directories: *mut *const ::core::ffi::c_char =
    ::core::ptr::null::<*const ::core::ffi::c_char>() as *mut *const ::core::ffi::c_char;
static mut max_incl_len: size_t = 0;
pub static mut reading_file: *const Floc = ::core::ptr::null::<Floc>();
static mut read_files: *mut goaldep = ::core::ptr::null::<goaldep>() as *mut goaldep;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn read_all_makefiles(mut makefiles: *mut *const ::core::ffi::c_char) -> *mut goaldep {
    let mut num_makefiles: ::core::ffi::c_uint = 0;
    define_variable_in_set(
        b"MAKEFILE_LIST\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t).wrapping_sub(1),
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_file,
        0,
        (*current_variable_set_list).set,
        NILF,
    );
    if 0x1 as ::core::ffi::c_int & db_level != 0 {
        printf(b"Reading makefiles...\n\0" as *const u8 as *const ::core::ffi::c_char);
        fflush(stdout);
    }
    let value: *mut ::core::ffi::c_char;
    let mut name: *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;
    let mut length: size_t = 0;
    value = allocated_expand_variable(
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
        if *p as ::core::ffi::c_int != 0 {
            let fresh10 = p;
            p = p.offset(1 as ::core::ffi::c_int as isize);
            *fresh10 = 0;
        }
        eval_makefile(
            strcache_add(name),
            (RM_NO_DEFAULT_GOAL | RM_INCLUDED | RM_DONTCARE) as ::core::ffi::c_ushort,
        );
    }
    free(value as *mut ::core::ffi::c_void);
    if !makefiles.is_null() {
        while !(*makefiles).is_null() {
            let d: *mut GoalDep = eval_makefile(*makefiles, 0);
            if *__errno_location() != 0 {
                perror_with_name(b"\0" as *const u8 as *const ::core::ffi::c_char, *makefiles);
            }
            *makefiles = if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            num_makefiles = num_makefiles.wrapping_add(1);
            makefiles = makefiles.offset(1 as ::core::ffi::c_int as isize);
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
        while !(*p_0).is_null() && file_exists_p(*p_0) == 0 {
            p_0 = p_0.offset(1 as ::core::ffi::c_int as isize);
        }
        if !(*p_0).is_null() {
            eval_makefile(*p_0, 0);
            if *__errno_location() != 0 {
                perror_with_name(b"\0" as *const u8 as *const ::core::ffi::c_char, *p_0);
            }
        } else {
            p_0 = &raw const default_makefiles as *const *const ::core::ffi::c_char;
            while !(*p_0).is_null() {
                let d_0: *mut GoalDep = alloc_goaldep();
                (*d_0).file = enter_file(strcache_add(*p_0));
                (*d_0).flags = RM_DONTCARE as ::core::ffi::c_uint as ::core::ffi::c_uint;
                (*d_0).next = read_files;
                read_files = d_0;
                p_0 = p_0.offset(1 as ::core::ffi::c_int as isize);
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
unsafe extern "C" fn eval_makefile(
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
    if 0x2 as ::core::ffi::c_int & db_level != 0 {
        printf(
            b"Reading makefile '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        if flags as ::core::ffi::c_int & RM_NO_DEFAULT_GOAL != 0 {
            printf(b" (no default goal)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if flags as ::core::ffi::c_int & RM_INCLUDED != 0 {
            printf(b" (search path)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if flags as ::core::ffi::c_int & RM_DONTCARE != 0 {
            printf(b" (don't care)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if flags as ::core::ffi::c_int & RM_NO_TILDE != 0 {
            printf(b" (no ~ expansion)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        puts(b"...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if flags as ::core::ffi::c_int & RM_NO_TILDE == 0
        && *filename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '~' as i32
    {
        expanded = tilde_expand(filename);
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
        && !include_directories.is_null()
        && flags as ::core::ffi::c_int & (1) << 1 != 0
        && !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*filename as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x8000 as ::core::ffi::c_int
            != 0)
    {
        let mut dir: *mut *const ::core::ffi::c_char;
        dir = include_directories;
        while !(*dir).is_null() {
            let included: *const ::core::ffi::c_char = concat(
                3,
                *dir,
                b"/\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
            loop {
                *__errno_location() = 0;
                ebuf.fp =
                    fopen(included, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
                if !(ebuf.fp.is_null() && *__errno_location() == EINTR) {
                    break;
                }
            }
            if !ebuf.fp.is_null() {
                filename = included;
                break;
            } else if *__errno_location() != ENOENT {
                filename = included;
                (*deps).error = *__errno_location();
                break;
            } else {
                dir = dir.offset(1 as ::core::ffi::c_int as isize);
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
        &raw mut ebuf,
        (flags as ::core::ffi::c_int & RM_NO_DEFAULT_GOAL == 0) as ::core::ffi::c_int,
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
pub unsafe fn eval_buffer(buffer: *mut ::core::ffi::c_char, flocp: *const Floc) {
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
    eval(&raw mut ebuf, 1);
    restore_conditionals(saved);
    reading_file = curfile;
}
unsafe extern "C" fn parse_var_assignment(
    mut line: *const ::core::ffi::c_char,
    targvar: ::core::ffi::c_int,
    mut flocp: *const Floc,
    vmod: *mut vmodifiers,
) -> *mut ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char;
    memset(
        vmod as *mut ::core::ffi::c_void,
        0,
        ::core::mem::size_of::<vmodifiers>() as size_t,
    );
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*line as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        line = line.offset(1 as ::core::ffi::c_int as isize);
    }
    if *line as ::core::ffi::c_int == 0 {
        return line as *mut ::core::ffi::c_char;
    }
    p = line;
    loop {
        let wlen: size_t;
        let mut p2: *const ::core::ffi::c_char;
        let mut v: variable = variable {
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
        p2 = parse_variable_definition(p, &raw mut v);
        if !p2.is_null() {
            break;
        }
        p2 = end_of_token(p);
        wlen = p2.offset_from(p) as ::core::ffi::c_long as size_t;
        if wlen
            == (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                .wrapping_sub(1 as usize)
            && memcmp(
                b"export\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
            ) == 0
        {
            (*vmod).set_export_v(v_export as variable_export);
        } else if wlen
            == (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                .wrapping_sub(1 as usize)
            && memcmp(
                b"unexport\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1),
            ) == 0
        {
            (*vmod).set_export_v(v_noexport as variable_export);
        } else if wlen
            == (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                .wrapping_sub(1 as usize)
            && memcmp(
                b"override\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1),
            ) == 0
        {
            (*vmod).set_override_v(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else if wlen
            == (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize)
                .wrapping_sub(1 as usize)
            && memcmp(
                b"private\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                p as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1),
            ) == 0
        {
            (*vmod).set_private_v(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else if targvar == 0
            && (wlen
                == (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                    .wrapping_sub(1 as usize)
                && memcmp(
                    b"define\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
                ) == 0)
        {
            if !flocp.is_null() {
                error(
                    flocp,
                    0,
                    b"warning: directive lines cannot start with TAB\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*vmod).set_define_v(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            p = next_token(p2);
            break;
        } else if targvar == 0
            && (wlen
                == (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                    .wrapping_sub(1 as usize)
                && memcmp(
                    b"undefine\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t).wrapping_sub(1),
                ) == 0)
        {
            if !flocp.is_null() {
                error(
                    flocp,
                    0,
                    b"warning: directive lines cannot start with TAB\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*vmod).set_undefine_v(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            p = next_token(p2);
            break;
        } else {
            return line as *mut ::core::ffi::c_char;
        }
        if !flocp.is_null() {
            error(
                flocp,
                0,
                b"warning: directive lines cannot start with TAB\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            flocp = ::core::ptr::null::<Floc>();
        }
        p = next_token(p2);
        if *p as ::core::ffi::c_int == 0 {
            return line as *mut ::core::ffi::c_char;
        }
    }
    (*vmod).set_assign_v(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    p as *mut ::core::ffi::c_char
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn eval(ebuf: *mut ebuffer, set_default: ::core::ffi::c_int) {
    let mut collapsed: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut collapsed_length: size_t = 0;
    let mut commands_len: size_t = 200;
    let mut commands: *mut ::core::ffi::c_char;
    let mut commands_idx: size_t = 0;
    let mut cmds_started: ::core::ffi::c_uint;
    let mut tgts_started: ::core::ffi::c_uint;
    let mut ignoring: ::core::ffi::c_int = 0;
    let mut in_ignored_define: ::core::ffi::c_int = 0;
    let mut no_targets: ::core::ffi::c_int = 0;
    let mut also_make_targets: ::core::ffi::c_int = 0;
    let mut filenames: *mut NameSeq = ::core::ptr::null_mut::<NameSeq>();
    let mut depstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nlines: ::core::ffi::c_long = 0;
    let mut two_colon: ::core::ffi::c_int = 0;
    let mut prefix: ::core::ffi::c_char = cmd_prefix;
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
        let initial_tab: ::core::ffi::c_uint;
        let mut vmod: vmodifiers = vmodifiers {
            assign_v_define_v_undefine_v_override_v_private_v_export_v: [0; 1],
            c2rust_padding: [0; 3],
        };
        (*ebuf).floc.lineno = (*ebuf)
            .floc
            .lineno
            .wrapping_add(nlines as ::core::ffi::c_ulong);
        nlines = readline(ebuf);
        if nlines < 0 {
            break;
        }
        line = (*ebuf).buffer;
        if (*ebuf).floc.lineno == 1 {
            let ul: *mut ::core::ffi::c_uchar = line as *mut ::core::ffi::c_uchar;
            if *ul.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xef as ::core::ffi::c_int
                && *ul.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0xbb as ::core::ffi::c_int
                && *ul.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0xbf as ::core::ffi::c_int
            {
                line = line.offset(3 as ::core::ffi::c_int as isize);
                if 0x1 as ::core::ffi::c_int & db_level != 0 {
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
        if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
            continue;
        }
        initial_tab = (*line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\t' as i32) as ::core::ffi::c_int as ::core::ffi::c_uint;
        linelen = strlen(line) as size_t;
        if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == cmd_prefix as ::core::ffi::c_int
        {
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
                    line.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
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
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
        {
            p = p.offset(1 as ::core::ffi::c_int as isize);
        }
        p = parse_var_assignment(
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
            let origin: variable_origin = (if vmod.override_v() as ::core::ffi::c_int != 0 {
                o_override as ::core::ffi::c_int
            } else {
                o_file as ::core::ffi::c_int
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
                    do_undefine(p, origin, ebuf);
                } else {
                    if vmod.define_v() != 0 {
                        v = do_define(p, origin, ebuf);
                    } else {
                        v = try_variable_definition(fstart, p, origin, s_global);
                    }
                    if !v.is_null() {
                    } else {
                        panic!("assertion failed: v != NULL");
                    };
                    if vmod.export_v() as ::core::ffi::c_int != v_default as ::core::ffi::c_int {
                        (*v).set_export(vmod.export_v() as variable_export);
                    }
                    if vmod.private_v() != 0 {
                        (*v).set_private_var(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                }
            }
        } else {
            if *p as ::core::ffi::c_int == 0 {
                continue;
            }
            p2 = end_of_token(p);
            wlen = p2.offset_from(p) as ::core::ffi::c_long as size_t;
            while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p2 as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int
                & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                != 0
            {
                p2 = p2.offset(1 as ::core::ffi::c_int as isize);
            }
            is_rule = (*p2 as ::core::ffi::c_int == ':' as i32
                || (*p2 as ::core::ffi::c_int == '&' as i32
                    || *p2 as ::core::ffi::c_int == '|' as i32)
                    && *p2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ':' as i32) as ::core::ffi::c_int
                as ::core::ffi::c_uint;
            if in_ignored_define != 0 {
                if wlen
                    == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
                        .wrapping_sub(1 as usize)
                    && memcmp(
                        b"endef\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        p as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                            .wrapping_sub(1),
                    ) == 0
                    && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                        .offset(*p2 as ::core::ffi::c_uchar as isize)
                        as ::core::ffi::c_int
                        & (0x8 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                        != 0
                {
                    in_ignored_define = 0;
                }
            } else {
                let i: ::core::ffi::c_int = conditional_line(p, wlen, fstart, initial_tab);
                if i != -(2 as ::core::ffi::c_int) {
                    if i == -(1 as ::core::ffi::c_int) {
                        fatal(
                            fstart,
                            0,
                            b"invalid syntax in conditional\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    ignoring = i;
                } else {
                    if ignoring != 0 {
                        continue;
                    }
                    if wlen
                        == (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                            .wrapping_sub(1 as usize)
                        && memcmp(
                            b"export\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            p as *const ::core::ffi::c_void,
                            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                                .wrapping_sub(1),
                        ) == 0
                        || wlen
                            == (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                                .wrapping_sub(1 as usize)
                            && memcmp(
                                b"unexport\0" as *const u8 as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                p as *const ::core::ffi::c_void,
                                (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                                    .wrapping_sub(1),
                            ) == 0
                    {
                        let exporting: ::core::ffi::c_int =
                            if *p as ::core::ffi::c_int == 'u' as i32 {
                                0
                            } else {
                                1
                            };
                        if initial_tab != 0 {
                            error(
                                &raw mut (*ebuf).floc,
                                strlen(if exporting != 0 {
                                    b"export\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"unexport\0" as *const u8 as *const ::core::ffi::c_char
                                }) as size_t,
                                b"warning: %s lines cannot start with TAB\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                if exporting != 0 {
                                    b"export\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"unexport\0" as *const u8 as *const ::core::ffi::c_char
                                },
                            );
                        }
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
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
                        if *p2 as ::core::ffi::c_int == 0 {
                            export_all_variables = exporting;
                        } else {
                            let mut l: size_t = 0;
                            let mut cp: *const ::core::ffi::c_char;
                            let ap: *mut ::core::ffi::c_char;
                            ap = allocated_expand_string_for_file(
                                p2,
                                ::core::ptr::null_mut::<File>(),
                            );
                            cp = ap;
                            p = find_next_token(&raw mut cp, &raw mut l);
                            while !p.is_null() {
                                let mut v_0: *mut variable = lookup_variable(p, l);
                                if v_0.is_null() {
                                    v_0 = define_variable_in_set(
                                        p,
                                        l,
                                        b"\0" as *const u8 as *const ::core::ffi::c_char,
                                        o_file,
                                        0,
                                        ::core::ptr::null_mut::<variable_set>(),
                                        fstart,
                                    );
                                }
                                (*v_0).set_export(
                                    (if exporting != 0 {
                                        v_export as ::core::ffi::c_int
                                    } else {
                                        v_noexport as ::core::ffi::c_int
                                    }) as variable_export
                                        as variable_export,
                                );
                                p = find_next_token(&raw mut cp, &raw mut l);
                            }
                            free(ap as *mut ::core::ffi::c_void);
                        }
                    } else if wlen
                        == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
                            .wrapping_sub(1 as usize)
                        && memcmp(
                            b"vpath\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            p as *const ::core::ffi::c_void,
                            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                                .wrapping_sub(1),
                        ) == 0
                    {
                        let mut cp_0: *const ::core::ffi::c_char;
                        let vpat: *mut ::core::ffi::c_char;
                        let mut l_0: size_t = 0;
                        if initial_tab != 0 {
                            error(
                                &raw mut (*ebuf).floc,
                                0,
                                b"warning: vpath directive lines cannot start with TAB\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
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
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            p2,
                            SIZE_MAX as size_t,
                        );
                        p = find_next_token(&raw mut cp_0, &raw mut l_0);
                        if !p.is_null() {
                            vpat = xstrndup(p, l_0);
                            p = find_next_token(&raw mut cp_0, &raw mut l_0);
                        } else {
                            vpat = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        }
                        construct_vpath_list(vpat, p);
                        free(vpat as *mut ::core::ffi::c_void);
                    } else if wlen
                        == (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as usize)
                            .wrapping_sub(1 as usize)
                        && memcmp(
                            b"include\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            p as *const ::core::ffi::c_void,
                            (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t)
                                .wrapping_sub(1),
                        ) == 0
                        || wlen
                            == (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                                .wrapping_sub(1 as usize)
                            && memcmp(
                                b"-include\0" as *const u8 as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                p as *const ::core::ffi::c_void,
                                (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                                    .wrapping_sub(1),
                            ) == 0
                        || wlen
                            == (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as usize)
                                .wrapping_sub(1 as usize)
                            && memcmp(
                                b"sinclude\0" as *const u8 as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                p as *const ::core::ffi::c_void,
                                (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                                    .wrapping_sub(1),
                            ) == 0
                    {
                        let save: *mut conditionals;
                        let mut new_conditionals: conditionals = conditionals {
                            if_cmds: 0,
                            allocated: 0,
                            ignoring: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            seen_else: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        };
                        let mut files: *mut NameSeq;
                        let noerror: ::core::ffi::c_int =
                            (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                != 'i' as i32) as ::core::ffi::c_int;
                        if initial_tab != 0 {
                            error(
                                &raw mut (*ebuf).floc,
                                strlen(if *p as ::core::ffi::c_int == 'i' as i32 {
                                    b"include\0" as *const u8 as *const ::core::ffi::c_char
                                } else if *p as ::core::ffi::c_int == '-' as i32 {
                                    b"-include\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"sinclude\0" as *const u8 as *const ::core::ffi::c_char
                                }) as size_t,
                                b"warning: %s lines cannot start with TAB\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                if *p as ::core::ffi::c_int == 'i' as i32 {
                                    b"include\0" as *const u8 as *const ::core::ffi::c_char
                                } else if *p as ::core::ffi::c_int == '-' as i32 {
                                    b"-include\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"sinclude\0" as *const u8 as *const ::core::ffi::c_char
                                },
                            );
                        }
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
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
                        p = allocated_expand_string_for_file(p2, ::core::ptr::null_mut::<File>());
                        if *p as ::core::ffi::c_int == 0 {
                            free(p as *mut ::core::ffi::c_void);
                        } else {
                            p2 = p;
                            files = parse_file_seq::<NameSeq>(
        &raw mut p2,
        0x1 as ::core::ffi::c_int,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                0x2 as ::core::ffi::c_int,
    );
                            free(p as *mut ::core::ffi::c_void);
                            save = install_conditionals(&raw mut new_conditionals);
                            if !filenames.is_null() {
                                fi.lineno = tgts_started as ::core::ffi::c_ulong;
                                fi.offset = 0;
                                record_files(
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
                                let d: *mut GoalDep = eval_makefile((*files).name, flags);
                                (*d).floc = *fstart;
                                free_ns(files);
                                files = next;
                            }
                            restore_conditionals(save);
                        }
                    } else if (wlen
                        == (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
                            .wrapping_sub(1 as usize)
                        && memcmp(
                            b"load\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            p as *const ::core::ffi::c_void,
                            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t)
                                .wrapping_sub(1),
                        ) == 0
                        || wlen
                            == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
                                .wrapping_sub(1 as usize)
                            && memcmp(
                                b"-load\0" as *const u8 as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                p as *const ::core::ffi::c_void,
                                (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                                    .wrapping_sub(1),
                            ) == 0)
                        && is_rule == 0
                    {
                        let mut files_0: *mut NameSeq;
                        let noerror_0: ::core::ffi::c_int =
                            (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == '-' as i32) as ::core::ffi::c_int;
                        if initial_tab != 0 {
                            error(
                                &raw mut (*ebuf).floc,
                                strlen(if noerror_0 != 0 {
                                    b"-load\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"load\0" as *const u8 as *const ::core::ffi::c_char
                                }) as size_t,
                                b"warning: %s lines cannot start with TAB\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                if noerror_0 != 0 {
                                    b"-load\0" as *const u8 as *const ::core::ffi::c_char
                                } else {
                                    b"load\0" as *const u8 as *const ::core::ffi::c_char
                                },
                            );
                        }
                        if !filenames.is_null() {
                            fi.lineno = tgts_started as ::core::ffi::c_ulong;
                            fi.offset = 0;
                            record_files(
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
                        p = allocated_expand_string_for_file(p2, ::core::ptr::null_mut::<File>());
                        if *p as ::core::ffi::c_int == 0 {
                            free(p as *mut ::core::ffi::c_void);
                        } else {
                            p2 = p;
                            files_0 = parse_file_seq::<NameSeq>(
        &raw mut p2,
        0x1 as ::core::ffi::c_int,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                0x2 as ::core::ffi::c_int,
    );
                            free(p as *mut ::core::ffi::c_void);
                            while !files_0.is_null() {
                                let next_0: *mut NameSeq = (*files_0).next;
                                let mut name: *const ::core::ffi::c_char = (*files_0).name;
                                let deps: *mut GoalDep;
                                let mut f: *mut File;
                                let r: ::core::ffi::c_int;
                                let mut file = File::default();
                                file.name = name;
                                r = load_file(&raw mut (*ebuf).floc, &raw mut file, noerror_0);
                                if r == 0 && noerror_0 == 0 {
                                    fatal(
                                        &raw mut (*ebuf).floc,
                                        strlen(name) as size_t,
                                        b"%s: failed to load\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        name,
                                    );
                                }
                                name = file.name;
                                f = lookup_file(name);
                                if f.is_null() {
                                    f = enter_file(name);
                                }
                                (*f).loaded = true;
                                (*f).unloaded = false;
                                free_ns(files_0);
                                files_0 = next_0;
                                if r == -(1 as ::core::ffi::c_int) {
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
                        if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == cmd_prefix as ::core::ffi::c_int
                        {
                            fatal(
                                fstart,
                                0,
                                b"recipe commences before first target\0" as *const u8
                                    as *const ::core::ffi::c_char,
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
                        if !cmdleft.is_null() && *cmdleft as ::core::ffi::c_int == '#' as i32 {
                            *cmdleft = 0;
                            cmdleft = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        } else if !cmdleft.is_null() {
                            let fresh12 = cmdleft;
                            cmdleft = cmdleft.offset(1 as ::core::ffi::c_int as isize);
                            semip = fresh12;
                            *semip = 0;
                        }
                        collapse_continuations(line);
                        wtype = get_next_mword(line, &raw mut lb_next, &raw mut wlen);
                        match wtype as ::core::ffi::c_uint {
                            1 => {
                                if !cmdleft.is_null() {
                                    fatal(
                                        fstart,
                                        0,
                                        b"missing rule before recipe\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            }
                            4 | 5 | 7 | 8 => {
                                no_targets = 1;
                            }
                            _ => {
                                p2 = expand_string_buf(
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
                                            *cmdleft = 0;
                                            expand_string_buf(pend, lb_next, SIZE_MAX as size_t);
                                            lb_next = lb_next.offset(strlen(lb_next) as isize);
                                            p2 = variable_buffer.offset(p2_off as isize);
                                            cmdleft =
                                                variable_buffer.offset(cmd_off as isize).offset(1);
                                        }
                                    }
                                    colonp = find_char_unquote(p2, ':' as i32);
                                    if !colonp.is_null() {
                                        if colonp > p2
                                            && *colonp.offset(-(1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int
                                                == '&' as i32
                                        {
                                            colonp =
                                                colonp.offset(-(1 as ::core::ffi::c_int) as isize);
                                        }
                                        break;
                                    } else {
                                        wtype = get_next_mword(
                                            lb_next,
                                            &raw mut lb_next,
                                            &raw mut wlen,
                                        );
                                        if wtype as ::core::ffi::c_uint
                                            == w_eol as ::core::ffi::c_int as ::core::ffi::c_uint
                                        {
                                            break;
                                        }
                                        p2 = p2.offset(strlen(p2) as isize);
                                        let fresh13 = p2;
                                        p2 = p2.offset(1 as ::core::ffi::c_int as isize);
                                        *fresh13 = ' ' as i32 as ::core::ffi::c_char;
                                        p2 = expand_string_buf(p2, lb_next, wlen);
                                    }
                                }
                                p2 = next_token(variable_buffer);
                                if wtype as ::core::ffi::c_uint
                                    == w_eol as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    if *p2 as ::core::ffi::c_int == 0 {
                                        continue;
                                    }
                                    if cmd_prefix as ::core::ffi::c_int == '\t' as i32
                                        && strncmp(
                                            line,
                                            b"        \0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            8,
                                        ) == 0
                                    {
                                        fatal(
                                            fstart,
                                            0,
                                            b"missing separator (did you mean TAB instead of 8 spaces?)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    }
                                    p2 = next_token(line);
                                    if strncmp(
                                        p2,
                                        b"if\0" as *const u8 as *const ::core::ffi::c_char,
                                        2,
                                    ) == 0
                                        && (strncmp(
                                            p2.offset(2 as ::core::ffi::c_int as isize)
                                                as *mut ::core::ffi::c_char,
                                            b"neq\0" as *const u8 as *const ::core::ffi::c_char,
                                            3,
                                        ) == 0
                                            && !(*(&raw mut stopchar_map
                                                as *mut ::core::ffi::c_ushort)
                                                .offset(
                                                    *p2.offset(5 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_uchar
                                                        as isize,
                                                )
                                                as ::core::ffi::c_int
                                                & 0x2 as ::core::ffi::c_int
                                                != 0)
                                            || strncmp(
                                                p2.offset(2 as ::core::ffi::c_int as isize)
                                                    as *mut ::core::ffi::c_char,
                                                b"eq\0" as *const u8 as *const ::core::ffi::c_char,
                                                2,
                                            ) == 0
                                                && !(*(&raw mut stopchar_map
                                                    as *mut ::core::ffi::c_ushort)
                                                    .offset(
                                                        *p2.offset(4 as ::core::ffi::c_int as isize)
                                                            as ::core::ffi::c_uchar
                                                            as isize,
                                                    )
                                                    as ::core::ffi::c_int
                                                    & 0x2 as ::core::ffi::c_int
                                                    != 0))
                                    {
                                        fatal(
                                            fstart,
                                            0,
                                            b"missing separator (ifeq/ifneq must be followed by whitespace)\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                        );
                                    }
                                    fatal(
                                        fstart,
                                        0,
                                        b"missing separator\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    let save_0: ::core::ffi::c_char = *colonp;
                                    if save_0 as ::core::ffi::c_int == '&' as i32 {
                                        also_make_targets = 1;
                                    }
                                    *colonp = 0;
                                    filenames = parse_file_seq::<NameSeq>(
        &raw mut p2,
        MAP_NUL,
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        PARSEFS_NONE,
    );
                                    *colonp = save_0;
                                    p2 = colonp.offset(
                                        (save_0 as ::core::ffi::c_int == '&' as i32)
                                            as ::core::ffi::c_int
                                            as isize,
                                    );
                                    if filenames.is_null() {
                                        no_targets = 1;
                                    } else {
                                        if *p2 as ::core::ffi::c_int != 0 {
                                        } else {
                                            panic!("assertion failed: *p2 != '\'");
                                        };
                                        p2 = p2.offset(1 as ::core::ffi::c_int as isize);
                                        two_colon = (*p2 as ::core::ffi::c_int == ':' as i32)
                                            as ::core::ffi::c_int;
                                        if two_colon != 0 {
                                            p2 = p2.offset(1 as ::core::ffi::c_int as isize);
                                        }
                                        if *lb_next as ::core::ffi::c_int != 0 {
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
                                                *semip = ';' as i32 as ::core::ffi::c_char;
                                                collapse_continuations(semip);
                                                variable_buffer_output(
                                                    p2.offset(strlen(p2) as isize),
                                                    semip,
                                                    (strlen(semip) as size_t).wrapping_add(1),
                                                );
                                                p2 = variable_buffer.offset(l_2 as isize);
                                            }
                                            record_target_var(
                                                filenames,
                                                p2,
                                                (if vmod.override_v() as ::core::ffi::c_int != 0 {
                                                    o_override as ::core::ffi::c_int
                                                } else {
                                                    o_file as ::core::ffi::c_int
                                                })
                                                    as variable_origin,
                                                &raw mut vmod,
                                                fstart,
                                            );
                                            filenames = ::core::ptr::null_mut::<NameSeq>();
                                        } else {
                                            find_char_unquote(lb_next, '=' as i32);
                                            prefix = cmd_prefix;
                                            no_targets = 0;
                                            if *lb_next as ::core::ffi::c_int != 0 {
                                                let l_3: size_t = p2.offset_from(variable_buffer)
                                                    as ::core::ffi::c_long
                                                    as size_t;
                                                expand_string_buf(
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
                                                            cmdleft = split.offset(
                                                                1 as ::core::ffi::c_int as isize,
                                                            );
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
                                                && *p.offset(-(1 as ::core::ffi::c_int) as isize)
                                                    as ::core::ffi::c_int
                                                    == '\\' as i32
                                            {
                                                let mut q: *mut ::core::ffi::c_char = p
                                                    .offset(-(1 as ::core::ffi::c_int) as isize)
                                                    as *mut ::core::ffi::c_char;
                                                let mut backslash: ::core::ffi::c_int = 0;
                                                loop {
                                                    let fresh15 = q;
                                                    q = q.offset(
                                                        -(1 as ::core::ffi::c_int) as isize,
                                                    );
                                                    if !(*fresh15 as ::core::ffi::c_int
                                                        == '\\' as i32)
                                                    {
                                                        break;
                                                    }
                                                    backslash =
                                                        (backslash == 0) as ::core::ffi::c_int;
                                                }
                                                if !(backslash != 0) {
                                                    break;
                                                }
                                                p = strchr(
                                                    p.offset(1 as ::core::ffi::c_int as isize),
                                                    ':' as i32,
                                                );
                                            }
                                            if !p.is_null() {
                                                let target: *mut NameSeq;
                                                target = parse_file_seq::<NameSeq>(
        &raw mut p2,
        0x40 as ::core::ffi::c_int,
                                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                                    0x4 as ::core::ffi::c_int,
    );
                                                p2 = p2.offset(1 as ::core::ffi::c_int as isize);
                                                if target.is_null() {
                                                    fatal(
                                                        fstart,
                                                        0,
                                                        b"missing target pattern\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                } else if !(*target).next.is_null() {
                                                    fatal(
                                                        fstart,
                                                        0,
                                                        b"multiple target patterns\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                                pattern_percent =
                                                    find_percent_cached(&raw mut (*target).name);
                                                pattern = (*target).name;
                                                if pattern_percent.is_null() {
                                                    fatal(
                                                        fstart,
                                                        0,
                                                        b"target pattern contains no '%%'\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
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
                                                .offset(-(1 as ::core::ffi::c_int as isize));
                                            strip_whitespace(&raw mut beg, &raw mut end);
                                            if beg <= end && *beg as ::core::ffi::c_int != 0 {
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
                                            check_specials(filenames, set_default);
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
            fstart,
            0,
            b"missing 'endif'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !filenames.is_null() {
        fi.lineno = tgts_started as ::core::ffi::c_ulong;
        fi.offset = 0;
        record_files(
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
unsafe extern "C" fn do_undefine(
    mut name: *mut ::core::ffi::c_char,
    origin: variable_origin,
    ebuf: *mut ebuffer,
) {
    let mut p: *mut ::core::ffi::c_char;
    let var: *mut ::core::ffi::c_char;
    var = allocated_expand_string_for_file(name, ::core::ptr::null_mut::<File>());
    name = next_token(var);
    if *name as ::core::ffi::c_int == 0 {
        fatal(
            &raw mut (*ebuf).floc,
            0,
            b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    p = name
        .offset(strlen(name) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while p > name
        && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x2 as ::core::ffi::c_int
            != 0
    {
        p = p.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    *p.offset(1 as ::core::ffi::c_int as isize) = 0;
    undefine_variable_in_set(
        &raw mut (*ebuf).floc,
        name,
        (p.offset_from(name) as ::core::ffi::c_long + 1) as size_t,
        origin,
        ::core::ptr::null_mut::<variable_set>(),
    );
    free(var as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn do_define(
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
    let mut nlevels: ::core::ffi::c_int = 1;
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
        if *var.value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
            error(
                &raw mut defstart,
                0,
                b"extraneous text after 'define' directive\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        *var.name.offset(var.length as isize) = 0;
    }
    n = allocated_expand_string_for_file(name, ::core::ptr::null_mut::<File>());
    name = next_token(n);
    if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
        fatal(
            &raw mut defstart,
            0,
            b"empty variable name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    p = name
        .offset(strlen(name) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while p > name
        && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & 0x2 as ::core::ffi::c_int
            != 0
    {
        p = p.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    *p.offset(1 as ::core::ffi::c_int as isize) = 0;
    loop {
        let mut len: size_t;
        let line: *mut ::core::ffi::c_char;
        let nlines: ::core::ffi::c_long = readline(ebuf);
        if nlines < 0 {
            fatal(
                &raw mut defstart,
                0,
                b"missing 'endef', unterminated 'define'\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*ebuf).floc.lineno = (*ebuf)
            .floc
            .lineno
            .wrapping_add(nlines as ::core::ffi::c_ulong);
        line = (*ebuf).buffer;
        collapse_continuations(line);
        if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != cmd_prefix as ::core::ffi::c_int
        {
            p = next_token(line);
            len = strlen(p) as size_t;
            if (len == 6
                || len > 6
                    && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
                        *p.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as isize,
                    ) as ::core::ffi::c_int
                        & 0x2 as ::core::ffi::c_int
                        != 0)
                && strncmp(p, b"define\0" as *const u8 as *const ::core::ffi::c_char, 6) == 0
            {
                nlevels += 1;
            } else if (len == 5
                || len > 5
                    && *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
                        *p.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                            as isize,
                    ) as ::core::ffi::c_int
                        & 0x2 as ::core::ffi::c_int
                        != 0)
                && strncmp(p, b"endef\0" as *const u8 as *const ::core::ffi::c_char, 5) == 0
            {
                p = p.offset(5 as ::core::ffi::c_int as isize);
                remove_comments(p);
                if *next_token(p) as ::core::ffi::c_int != 0 {
                    error(
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
        }
        len = strlen(line) as size_t;
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
        *definition.offset(0 as ::core::ffi::c_int as isize) = 0;
    } else {
        *definition.offset(idx.wrapping_sub(1) as isize) = 0;
    }
    v = do_variable_definition(
        &raw mut defstart,
        name,
        definition,
        origin,
        var.flavor(),
        var.conditional() as ::core::ffi::c_int,
        s_global,
    );
    free(n as *mut ::core::ffi::c_void);
    v
}
unsafe extern "C" fn conditional_line(
    mut line: *mut ::core::ffi::c_char,
    mut len: size_t,
    flocp: *const Floc,
    initial_tab: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let cmdname: *const ::core::ffi::c_char;
    let cmdtype: C2RustUnnamed;
    let mut i: ::core::ffi::c_uint;
    let o: ::core::ffi::c_uint;
    if len == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize).wrapping_sub(1 as usize)
        && strncmp(
            b"ifdef\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        ) == 0
    {
        cmdtype = c_ifdef;
        cmdname = b"ifdef\0" as *const u8 as *const ::core::ffi::c_char;
    } else if len
        == (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize).wrapping_sub(1 as usize)
        && strncmp(
            b"ifndef\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
        ) == 0
    {
        cmdtype = c_ifndef;
        cmdname = b"ifndef\0" as *const u8 as *const ::core::ffi::c_char;
    } else if len
        == (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize)
        && strncmp(
            b"ifeq\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
        ) == 0
    {
        cmdtype = c_ifeq;
        cmdname = b"ifeq\0" as *const u8 as *const ::core::ffi::c_char;
    } else if len
        == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize).wrapping_sub(1 as usize)
        && strncmp(
            b"ifneq\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        ) == 0
    {
        cmdtype = c_ifneq;
        cmdname = b"ifneq\0" as *const u8 as *const ::core::ffi::c_char;
    } else if len
        == (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize)
        && strncmp(
            b"else\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
        ) == 0
    {
        cmdtype = c_else;
        cmdname = b"else\0" as *const u8 as *const ::core::ffi::c_char;
    } else if len
        == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize).wrapping_sub(1 as usize)
        && strncmp(
            b"endif\0" as *const u8 as *const ::core::ffi::c_char,
            line,
            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
        ) == 0
    {
        cmdtype = c_endif;
        cmdname = b"endif\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        return -(2 as ::core::ffi::c_int);
    }
    if initial_tab != 0 {
        error(
            flocp,
            0,
            b"warning: conditional directive lines cannot start with TAB\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    line = line.offset(len as isize);
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*line as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        line = line.offset(1 as ::core::ffi::c_int as isize);
    }
    if cmdtype as ::core::ffi::c_uint == c_endif as ::core::ffi::c_int as ::core::ffi::c_uint {
        if *line as ::core::ffi::c_int != 0 {
            error(
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous text after '%s' directive\0" as *const u8
                    as *const ::core::ffi::c_char,
                cmdname,
            );
        }
        if (*conditionals).if_cmds == 0 {
            fatal(
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                cmdname,
            );
        }
        (*conditionals).if_cmds = (*conditionals).if_cmds.wrapping_sub(1);
    } else if cmdtype as ::core::ffi::c_uint == c_else as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut p: *const ::core::ffi::c_char;
        if (*conditionals).if_cmds == 0 {
            fatal(
                flocp,
                strlen(cmdname) as size_t,
                b"extraneous '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                cmdname,
            );
        }
        o = (*conditionals).if_cmds.wrapping_sub(1);
        if *(*conditionals).seen_else.offset(o as isize) != 0 {
            fatal(
                flocp,
                0,
                b"only one 'else' per conditional\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        match *(*conditionals).ignoring.offset(o as isize) as ::core::ffi::c_int {
            0 => {
                *(*conditionals).ignoring.offset(o as isize) = 2;
            }
            1 => {
                *(*conditionals).ignoring.offset(o as isize) = 0;
            }
            _ => {}
        }
        if *line as ::core::ffi::c_int == 0 {
            *(*conditionals).seen_else.offset(o as isize) = 1;
        } else {
            p = line.offset(1 as ::core::ffi::c_int as isize);
            while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int
                & (0x2 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
                != 0)
            {
                p = p.offset(1 as ::core::ffi::c_int as isize);
            }
            len = p.offset_from(line) as ::core::ffi::c_long as size_t;
            if len
                == (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize)
                    .wrapping_sub(1 as usize)
                && strncmp(
                    b"else\0" as *const u8 as *const ::core::ffi::c_char,
                    line,
                    (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
                ) == 0
                || len
                    == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
                        .wrapping_sub(1 as usize)
                    && strncmp(
                        b"endif\0" as *const u8 as *const ::core::ffi::c_char,
                        line,
                        (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t)
                            .wrapping_sub(1),
                    ) == 0
                || conditional_line(line, len, flocp, 0) < 0
            {
                error(
                    flocp,
                    strlen(cmdname) as size_t,
                    b"extraneous text after '%s' directive\0" as *const u8
                        as *const ::core::ffi::c_char,
                    cmdname,
                );
            } else {
                if (*(*conditionals).ignoring.offset(o as isize) as ::core::ffi::c_int) < 2 {
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
        if cmdtype as ::core::ffi::c_uint == c_ifdef as ::core::ffi::c_int as ::core::ffi::c_uint
            || cmdtype as ::core::ffi::c_uint
                == c_ifndef as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let l: size_t;
            let var: *mut ::core::ffi::c_char;
            let v: *mut variable;
            let mut p_0: *mut ::core::ffi::c_char;
            var = allocated_expand_string_for_file(line, ::core::ptr::null_mut::<File>());
            p_0 = end_of_token(var);
            l = p_0.offset_from(var) as ::core::ffi::c_long as size_t;
            while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*p_0 as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int
                & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                != 0
            {
                p_0 = p_0.offset(1 as ::core::ffi::c_int as isize);
            }
            if *p_0 as ::core::ffi::c_int != 0 {
                return -(1 as ::core::ffi::c_int);
            }
            *var.offset(l as isize) = 0;
            v = lookup_variable(var, l);
            *(*conditionals).ignoring.offset(o as isize) =
                ((!v.is_null() && *(*v).value as ::core::ffi::c_int != 0) as ::core::ffi::c_int
                    == (cmdtype as ::core::ffi::c_uint
                        == c_ifndef as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_char;
            free(var as *mut ::core::ffi::c_void);
        } else {
            let mut s1: *mut ::core::ffi::c_char;
            let mut s2: *mut ::core::ffi::c_char;
            let l_0: size_t;
            let mut termin: ::core::ffi::c_char = (if *line as ::core::ffi::c_int == '(' as i32 {
                ',' as i32
            } else {
                *line as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            if termin as ::core::ffi::c_int != ',' as i32
                && termin as ::core::ffi::c_int != '"' as i32
                && termin as ::core::ffi::c_int != '\'' as i32
            {
                return -(1 as ::core::ffi::c_int);
            }
            line = line.offset(1 as ::core::ffi::c_int as isize);
            s1 = line;
            while *line as ::core::ffi::c_int != 0
                && *line as ::core::ffi::c_int != termin as ::core::ffi::c_int
            {
                if *line as ::core::ffi::c_int == '$' as i32 {
                    line = skip_reference(line.offset(1 as ::core::ffi::c_int as isize));
                } else {
                    line = line.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            if *line as ::core::ffi::c_int == 0 {
                return -(1 as ::core::ffi::c_int);
            }
            if termin as ::core::ffi::c_int == ',' as i32 {
                let fresh27 = line;
                line = line.offset(1 as ::core::ffi::c_int as isize);
                let mut p_1: *mut ::core::ffi::c_char = fresh27;
                while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort).offset(
                    *p_1.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uchar
                        as isize,
                ) as ::core::ffi::c_int
                    & 0x2 as ::core::ffi::c_int
                    != 0
                {
                    p_1 = p_1.offset(-(1 as ::core::ffi::c_int) as isize);
                }
                *p_1 = 0;
            } else {
                let fresh28 = line;
                line = line.offset(1 as ::core::ffi::c_int as isize);
                *fresh28 = 0;
            }
            s2 = expand_string_buf(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                s1,
                SIZE_MAX as size_t,
            );
            l_0 = strlen(s2) as size_t;
            alloca_allocations.push(::std::vec::from_elem(0, l_0.wrapping_add(1) as usize));
            s1 = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                s1 as *mut ::core::ffi::c_void,
                s2 as *const ::core::ffi::c_void,
                (l_0 as size_t).wrapping_add(1),
            );
            if termin as ::core::ffi::c_int != ',' as i32 {
                while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                    .offset(*line as ::core::ffi::c_uchar as isize)
                    as ::core::ffi::c_int
                    & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                    != 0
                {
                    line = line.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            termin = (if termin as ::core::ffi::c_int == ',' as i32 {
                ')' as i32
            } else {
                *line as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            if termin as ::core::ffi::c_int != ')' as i32
                && termin as ::core::ffi::c_int != '"' as i32
                && termin as ::core::ffi::c_int != '\'' as i32
            {
                return -(1 as ::core::ffi::c_int);
            }
            if termin as ::core::ffi::c_int == ')' as i32 {
                let mut count: ::core::ffi::c_int = 0;
                s2 = next_token(line);
                line = s2;
                while *line as ::core::ffi::c_int != 0 {
                    if *line as ::core::ffi::c_int == '(' as i32 {
                        count += 1;
                    } else if *line as ::core::ffi::c_int == ')' as i32 {
                        if count <= 0 {
                            break;
                        }
                        count -= 1;
                    }
                    line = line.offset(1 as ::core::ffi::c_int as isize);
                }
            } else {
                line = line.offset(1 as ::core::ffi::c_int as isize);
                s2 = line;
                while *line as ::core::ffi::c_int != 0
                    && *line as ::core::ffi::c_int != termin as ::core::ffi::c_int
                {
                    line = line.offset(1 as ::core::ffi::c_int as isize);
                }
            }
            if *line as ::core::ffi::c_int == 0 {
                return -(1 as ::core::ffi::c_int);
            }
            let fresh29 = line;
            line = line.offset(1 as ::core::ffi::c_int as isize);
            *fresh29 = 0;
            while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                .offset(*line as ::core::ffi::c_uchar as isize)
                as ::core::ffi::c_int
                & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                != 0
            {
                line = line.offset(1 as ::core::ffi::c_int as isize);
            }
            if *line as ::core::ffi::c_int != 0 {
                error(
                    flocp,
                    strlen(cmdname) as size_t,
                    b"extraneous text after '%s' directive\0" as *const u8
                        as *const ::core::ffi::c_char,
                    cmdname,
                );
            }
            s2 = expand_string_buf(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                s2,
                SIZE_MAX as size_t,
            );
            *(*conditionals).ignoring.offset(o as isize) =
                ((*s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int
                    && (*s1 as ::core::ffi::c_int == 0
                        || strcmp(
                            s1.offset(1 as ::core::ffi::c_int as isize),
                            s2.offset(1 as ::core::ffi::c_int as isize),
                        ) == 0)) as ::core::ffi::c_int
                    == (cmdtype as ::core::ffi::c_uint
                        == c_ifneq as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_char;
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
unsafe extern "C" fn record_target_var(
    mut filenames: *mut NameSeq,
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
            v = assign_variable_definition(&raw mut (*p).variable, defn);
            if !v.is_null() {
            } else {
                panic!("assertion failed: v != 0");
            };
            (*v).set_origin(origin as variable_origin);
            if (*v).flavor() as ::core::ffi::c_int == f_simple as ::core::ffi::c_int {
                (*v).value =
                    allocated_expand_string_for_file((*v).value, ::core::ptr::null_mut::<File>());
            } else {
                (*v).value = xstrdup((*v).value);
            }
        } else {
            let mut f: *mut File;
            f = lookup_file(name);
            if f.is_null() {
                f = enter_file(strcache_add(name));
            } else if !(*f).double_colon.is_null() {
                f = (*f).double_colon;
            }
            initialize_file_variables(f, 1);
            current_variable_set_list = (*f).variables;
            v = try_variable_definition(flocp, defn, origin, s_target);
            if v.is_null() {
                fatal(
                    flocp,
                    0,
                    b"malformed target-specific variable definition\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            current_variable_set_list = global;
        }
        (*v).set_per_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*v).set_private_var((*vmod).private_v() as ::core::ffi::c_uint);
        if (*vmod).export_v() as ::core::ffi::c_int != v_default as ::core::ffi::c_int {
            (*v).set_export((*vmod).export_v() as variable_export);
        }
        if (*v).origin() as ::core::ffi::c_int != o_override as ::core::ffi::c_int {
            let gv: *mut variable;
            let len: size_t = strlen((*v).name) as size_t;
            gv = lookup_variable((*v).name, len);
            if !gv.is_null()
                && v != gv
                && ((*gv).origin() as ::core::ffi::c_int == o_env_override as ::core::ffi::c_int
                    || (*gv).origin() as ::core::ffi::c_int == o_command as ::core::ffi::c_int)
            {
                free((*v).value as *mut ::core::ffi::c_void);
                (*v).value = xstrdup((*gv).value);
                (*v).set_origin((*gv).origin() as variable_origin);
                (*v).set_recursive((*gv).recursive() as ::core::ffi::c_uint);
                (*v).set_append(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        filenames = nextf;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn check_specials(files: *mut nameseq, set_default: ::core::ffi::c_int) {
    let mut t: *mut nameseq;
    t = files;
    while !t.is_null() {
        let nm: *const ::core::ffi::c_char = (*t).name;
        if posix_pedantic == 0
            && (*nm as ::core::ffi::c_int
                == *(b".POSIX\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
                && (*nm as ::core::ffi::c_int == 0
                    || strcmp(
                        nm.offset(1 as ::core::ffi::c_int as isize),
                        (b".POSIX\0" as *const u8 as *const ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
                    ) == 0))
        {
            posix_pedantic = 1;
            define_variable_in_set(
                b".SHELLFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t).wrapping_sub(1),
                b"-ec\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                b"CC\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
                b"c99\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                b"CFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
                b"-O1\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                b"FC\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t).wrapping_sub(1),
                b"fort77\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                b"FFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t).wrapping_sub(1),
                b"-O1\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                b"SCCSGETFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 13]>() as size_t).wrapping_sub(1),
                b"-s\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
            define_variable_in_set(
                b"ARFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 8]>() as size_t).wrapping_sub(1),
                b"-rv\0" as *const u8 as *const ::core::ffi::c_char,
                o_default,
                0,
                (*current_variable_set_list).set,
                NILF,
            );
        } else if second_expansion == 0
            && (*nm as ::core::ffi::c_int
                == *(b".SECONDEXPANSION\0" as *const u8 as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int
                && (*nm as ::core::ffi::c_int == 0
                    || strcmp(
                        nm.offset(1 as ::core::ffi::c_int as isize),
                        (b".SECONDEXPANSION\0" as *const u8 as *const ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
                    ) == 0))
        {
            second_expansion = 1;
        } else if one_shell == 0
            && (*nm as ::core::ffi::c_int
                == *(b".ONESHELL\0" as *const u8 as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int
                && (*nm as ::core::ffi::c_int == 0
                    || strcmp(
                        nm.offset(1 as ::core::ffi::c_int as isize),
                        (b".ONESHELL\0" as *const u8 as *const ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
                    ) == 0))
        {
            one_shell = 1;
        } else if set_default != 0
            && *(*default_goal_var).value.offset(0) as ::core::ffi::c_int == 0
        {
            let mut d: *mut Dep;
            let mut reject: ::core::ffi::c_int = 0;
            if !strchr(nm, '%' as i32).is_null() {
                break;
            }
            if !(*nm as ::core::ffi::c_int == '.' as i32 && strchr(nm, '/' as i32).is_null()) {
                d = (*suffix_file).deps;
                while !d.is_null() {
                    let mut d2: *mut Dep;
                    if *(if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as ::core::ffi::c_int
                        != '.' as i32
                        && (*nm as ::core::ffi::c_int
                            == *(if !(*d).name.is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }) as ::core::ffi::c_int
                            && (*nm as ::core::ffi::c_int == 0
                                || strcmp(
                                    nm.offset(1 as ::core::ffi::c_int as isize),
                                    (if !(*d).name.is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    })
                                    .offset(1),
                                ) == 0))
                    {
                        reject = 1;
                        break;
                    } else {
                        d2 = (*suffix_file).deps;
                        while !d2.is_null() {
                            let l: size_t = strlen(if !(*d2).name.is_null() {
                                (*d2).name
                            } else {
                                (*(*d2).file).name
                            }) as size_t;
                            if strncmp(
                                nm,
                                if !(*d2).name.is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                },
                                l as size_t,
                            ) == 0
                                && *nm.offset(l as isize) as ::core::ffi::c_int
                                    == *(if !(*d).name.is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    }) as ::core::ffi::c_int
                                && (*nm.offset(l as isize) as ::core::ffi::c_int == 0
                                    || strcmp(
                                        nm.offset(l as isize)
                                            .offset(1 as ::core::ffi::c_int as isize),
                                        (if !(*d).name.is_null() {
                                            (*d).name
                                        } else {
                                            (*(*d).file).name
                                        })
                                        .offset(1),
                                    ) == 0)
                            {
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
                }
                if reject == 0 {
                    define_variable_in_set(
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
pub unsafe fn check_special_file(file: *mut file, flocp: *const Floc) {
    if *(*file).name as ::core::ffi::c_int
        == *(b".WAIT\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
        && (*(*file).name as ::core::ffi::c_int == 0
            || strcmp(
                (*file).name.offset(1 as ::core::ffi::c_int as isize),
                (b".WAIT\0" as *const u8 as *const ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
            ) == 0)
    {
        static mut wpre: ::core::ffi::c_uint = 0;
        static mut wcmd: ::core::ffi::c_uint = 0;
        if wpre == 0 && !(*file).deps.is_null() {
            error(
                flocp,
                0,
                b".WAIT should not have prerequisites\0" as *const u8 as *const ::core::ffi::c_char,
            );
            wpre = 1;
        }
        if wcmd == 0 && !(*file).cmds.is_null() {
            error(
                flocp,
                0,
                b".WAIT should not have commands\0" as *const u8 as *const ::core::ffi::c_char,
            );
            wcmd = 1;
        }
    }
}
unsafe extern "C" fn record_files(
    mut filenames: *mut NameSeq,
    are_also_makes: ::core::ffi::c_int,
    pattern: *const ::core::ffi::c_char,
    pattern_percent: *const ::core::ffi::c_char,
    mut depstr: *mut ::core::ffi::c_char,
    cmds_started: ::core::ffi::c_uint,
    commands: *mut ::core::ffi::c_char,
    commands_idx: size_t,
    two_colon: ::core::ffi::c_int,
    prefix: ::core::ffi::c_char,
    flocp: *const Floc,
) {
    let cmds: *mut Commands;
    let mut deps: *mut Dep;
    let mut also_make: *mut Dep = ::core::ptr::null_mut::<Dep>();
    let mut implicit_percent: *const ::core::ffi::c_char;
    let mut name: *const ::core::ffi::c_char;
    if snapped_deps != 0 {
        fatal(
            flocp,
            0,
            b"prerequisites cannot be defined in recipes\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    name = (*filenames).name;
    implicit_percent = find_percent_cached(&raw mut name);
    if commands_idx > 0 {
        cmds = xmalloc(::core::mem::size_of::<Commands>() as size_t) as *mut Commands;
        (*cmds).fileinfo.filenm = (*flocp).filenm;
        (*cmds).fileinfo.lineno = cmds_started as ::core::ffi::c_ulong;
        (*cmds).fileinfo.offset = 0;
        (*cmds).commands = xstrndup(commands, commands_idx);
        (*cmds).command_lines = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*cmds).recipe_prefix = prefix;
    } else if are_also_makes != 0 {
        fatal(
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
        if second_expansion != 0 && !strchr(depstr, '$' as i32).is_null() {
            deps = alloc_dep();
            (*deps).name = depstr;
            (*deps).need_2nd_expansion = true;
            (*deps).staticpattern = !pattern.is_null();
        } else {
            deps = split_prereqs(depstr);
            free(depstr as *mut ::core::ffi::c_void);
            if pattern.is_null() && implicit_percent.is_null() {
                deps = enter_prereqs(deps, ::core::ptr::null::<::core::ffi::c_char>());
            }
        }
    }
    if !implicit_percent.is_null() {
        let mut nextf: *mut NameSeq;
        let targets: *mut *const ::core::ffi::c_char;
        let target_pats: *mut *const ::core::ffi::c_char;
        let mut c: ::core::ffi::c_ushort;
        if !pattern.is_null() {
            fatal(
                flocp,
                0,
                b"mixed implicit and static pattern rules\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        nextf = (*filenames).next;
        free_ns(filenames);
        filenames = nextf;
        c = 1;
        while !nextf.is_null() {
            c = c.wrapping_add(1);
            nextf = (*nextf).next;
        }
        targets = xmalloc(
            (c as size_t)
                .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
        ) as *mut *const ::core::ffi::c_char;
        target_pats = xmalloc(
            (c as size_t)
                .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
        ) as *mut *const ::core::ffi::c_char;
        let fresh17 = &mut (*targets.offset(0 as ::core::ffi::c_int as isize));
        *fresh17 = name;
        let fresh18 = &mut (*target_pats.offset(0 as ::core::ffi::c_int as isize));
        *fresh18 = implicit_percent;
        c = 1;
        while !filenames.is_null() {
            name = (*filenames).name;
            implicit_percent = find_percent_cached(&raw mut name);
            if implicit_percent.is_null() {
                fatal(
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
            nextf = (*filenames).next;
            free_ns(filenames);
            filenames = nextf;
        }
        create_pattern_rule(targets, target_pats, c, two_colon, deps, cmds, 1);
        return;
    }
    loop {
        let nextf_0: *mut NameSeq = (*filenames).next;
        let mut f: *mut File;
        let mut this: *mut Dep = ::core::ptr::null_mut::<Dep>();
        free_ns(filenames);
        if !pattern.is_null() && pattern_matches(pattern, pattern_percent, name) == 0 {
            error(
                flocp,
                strlen(name) as size_t,
                b"target '%s' doesn't match the target pattern\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
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
                    flocp,
                    strlen((*f).name) as size_t,
                    b"target file '%s' has both : and :: entries\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*f).name,
                );
            }
            if !cmds.is_null() && cmds == (*f).cmds {
                error(
                    flocp,
                    strlen((*f).name) as size_t,
                    b"target '%s' given more than once in the same rule\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*f).name,
                );
            } else if !cmds.is_null()
                && !(*f).cmds.is_null()
                && (*f).is_target
            {
                let l: size_t = strlen((*f).name) as size_t;
                error(
                    &raw mut (*cmds).fileinfo,
                    l,
                    b"warning: overriding recipe for target '%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*f).name,
                );
                error(
                    &raw mut (*(*f).cmds).fileinfo,
                    l,
                    b"warning: ignoring old recipe for target '%s'\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*f).name,
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
            if !f.is_null()
                && (*f).is_target
                && (*f).double_colon.is_null()
            {
                fatal(
                    flocp,
                    strlen((*f).name) as size_t,
                    b"target file '%s' has both : and :: entries\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*f).name,
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
                pattern_percent.offset(1 as ::core::ffi::c_int as isize),
                percent.offset(1 as ::core::ffi::c_int as isize),
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
        check_special_file(f, flocp);
        if nextf_0.is_null() {
            break;
        }
        filenames = nextf_0;
        name = (*filenames).name;
        if !find_percent_cached(&raw mut name).is_null() {
            error(
                flocp,
                0,
                b"*** mixed implicit and normal rules: deprecated syntax\0" as *const u8
                    as *const ::core::ffi::c_char,
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
                &raw mut (*cmds).fileinfo,
                strlen(f0.name) as size_t,
                b"warning: overriding group membership for target '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                f0.name,
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
unsafe extern "C" fn find_map_unquote(
    string: *mut ::core::ffi::c_char,
    mut stopmap: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut string_len: size_t = 0;
    let mut p: *mut ::core::ffi::c_char = string;
    stopmap |= MAP_NUL;
    loop {
        while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & stopmap
            != 0)
        {
            p = p.offset(1 as ::core::ffi::c_int as isize);
        }
        if *p as ::core::ffi::c_int == 0 {
            break;
        }
        if *p as ::core::ffi::c_int == '$' as i32 {
            p = skip_reference(p.offset(1 as ::core::ffi::c_int as isize));
        } else if p > string
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '\\' as i32
        {
            let mut i: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
            while p.offset(i as isize) as *mut ::core::ffi::c_char >= string
                && *p.offset(i as isize) as ::core::ffi::c_int == '\\' as i32
            {
                i -= 1;
            }
            i += 1;
            if string_len == 0 {
                string_len = strlen(string) as size_t;
            }
            let hi: ::core::ffi::c_int = -(i / 2);
            memmove(
                p.offset(i as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                p.offset((i / 2) as isize) as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                (string_len as size_t)
                    .wrapping_sub(p.offset_from(string) as ::core::ffi::c_long as size_t)
                    .wrapping_add(hi as size_t)
                    .wrapping_add(1),
            );
            p = p.offset((i / 2) as isize);
            if i % 2 == 0 {
                return p;
            }
        } else {
            return p;
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
unsafe extern "C" fn find_char_unquote(
    string: *mut ::core::ffi::c_char,
    stop: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut string_len: size_t = 0;
    let mut p: *mut ::core::ffi::c_char = string;
    loop {
        p = strchr(p, stop);
        if p.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if p > string
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '\\' as i32
        {
            let mut i: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
            while p.offset(i as isize) as *mut ::core::ffi::c_char >= string
                && *p.offset(i as isize) as ::core::ffi::c_int == '\\' as i32
            {
                i -= 1;
            }
            i += 1;
            if string_len == 0 {
                string_len = strlen(string) as size_t;
            }
            let hi: ::core::ffi::c_int = -(i / 2);
            memmove(
                p.offset(i as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                p.offset((i / 2) as isize) as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                (string_len as size_t)
                    .wrapping_sub(p.offset_from(string) as ::core::ffi::c_long as size_t)
                    .wrapping_add(hi as size_t)
                    .wrapping_add(1),
            );
            p = p.offset((i / 2) as isize);
            if i % 2 == 0 {
                return p;
            }
        } else {
            return p;
        }
    }
}
unsafe extern "C" fn unescape_char(
    string: *mut ::core::ffi::c_char,
    c: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = string;
    let mut s: *mut ::core::ffi::c_char = string;
    while *s as ::core::ffi::c_int != 0 {
        if *s as ::core::ffi::c_int == '\\' as i32 {
            let mut e: *mut ::core::ffi::c_char = s;
            let mut l: size_t;
            while *e as ::core::ffi::c_int == '\\' as i32 {
                e = e.offset(1 as ::core::ffi::c_int as isize);
            }
            l = e.offset_from(s) as ::core::ffi::c_long as size_t;
            if *e as ::core::ffi::c_int != c || l.wrapping_rem(2) == 0 {
                memmove(
                    p as *mut ::core::ffi::c_void,
                    s as *const ::core::ffi::c_void,
                    l as size_t,
                );
                p = p.offset(l as isize);
                if *e as ::core::ffi::c_int == 0 {
                    break;
                }
            } else if l > 1 {
                l = l.wrapping_div(2);
                memmove(
                    p as *mut ::core::ffi::c_void,
                    s as *const ::core::ffi::c_void,
                    l as size_t,
                );
                p = p.offset(l as isize);
            }
            s = e;
        }
        let fresh21 = s;
        s = s.offset(1 as ::core::ffi::c_int as isize);
        let fresh22 = p;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        *fresh22 = *fresh21;
    }
    *p = 0;
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
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let p: *const ::core::ffi::c_char = strchr(*string, '%' as i32);
    let new: *mut ::core::ffi::c_char;
    let mut np: *mut ::core::ffi::c_char;
    let mut slen: size_t;
    if p.is_null()
        || p == *string
        || *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != '\\' as i32
    {
        return p;
    }
    slen = strlen(*string) as size_t;
    alloca_allocations.push(::std::vec::from_elem(0, slen.wrapping_add(1) as usize));
    new = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    memcpy(
        new as *mut ::core::ffi::c_void,
        *string as *const ::core::ffi::c_void,
        (slen as size_t).wrapping_add(1),
    );
    np = new.offset(p.offset_from(*string) as ::core::ffi::c_long as isize);
    loop {
        let pp: *mut ::core::ffi::c_char = np;
        let mut i: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
        while np.offset(i as isize) as *mut ::core::ffi::c_char >= new
            && *np.offset(i as isize) as ::core::ffi::c_int == '\\' as i32
        {
            i -= 1;
        }
        i += 1;
        let hi: ::core::ffi::c_int = -(i / 2);
        memmove(
            pp.offset(i as isize) as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            pp.offset((i / 2) as isize) as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            (slen as size_t)
                .wrapping_sub(pp.offset_from(new) as ::core::ffi::c_long as size_t)
                .wrapping_add(hi as size_t)
                .wrapping_add(1),
        );
        slen = slen.wrapping_add((i / 2 + i % 2) as size_t);
        np = np.offset((i / 2) as isize);
        if i % 2 == 0 {
            break;
        }
        np = strchr(np, '%' as i32);
        if !(!np.is_null()
            && *np.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == '\\' as i32)
        {
            break;
        }
    }
    *string = strcache_add(new);
    if !np.is_null() {
        (*string).offset(np.offset_from(new) as ::core::ffi::c_long as isize)
    } else {
        ::core::ptr::null::<::core::ffi::c_char>()
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn readstring(ebuf: *mut ebuffer) -> ::core::ffi::c_long {
    let mut eol: *mut ::core::ffi::c_char;
    if (*ebuf).bufnext >= (*ebuf).bufstart.offset((*ebuf).size as isize) {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    (*ebuf).buffer = (*ebuf).bufnext;
    eol = (*ebuf).buffer;
    loop {
        let mut backslash: ::core::ffi::c_int = 0;
        let bol: *const ::core::ffi::c_char = eol;
        let mut p: *const ::core::ffi::c_char;
        eol = strchr(eol, '\n' as i32);
        p = eol;
        if eol.is_null() {
            (*ebuf).bufnext = (*ebuf).bufstart.offset((*ebuf).size as isize).offset(1);
            return 0;
        }
        while p > bol && {
            p = p.offset(-(1 as ::core::ffi::c_int) as isize);
            *p as ::core::ffi::c_int == '\\' as i32
        } {
            backslash = (backslash == 0) as ::core::ffi::c_int;
        }
        if backslash == 0 {
            break;
        }
        eol = eol.offset(1 as ::core::ffi::c_int as isize);
    }
    *eol = 0;
    (*ebuf).bufnext = eol.offset(1 as ::core::ffi::c_int as isize);
    0
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn readline(ebuf: *mut ebuffer) -> ::core::ffi::c_long {
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
        end.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int,
        (*ebuf).fp,
    )
    .is_null()
    {
        let mut p2: *mut ::core::ffi::c_char;
        let mut len: size_t;
        let mut backslash: ::core::ffi::c_int;
        len = strlen(p) as size_t;
        if len == 0 {
            error(
                &raw mut (*ebuf).floc,
                0,
                b"warning: NUL character seen; rest of line ignored\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            *p.offset(0 as ::core::ffi::c_int as isize) = '\n' as i32 as ::core::ffi::c_char;
            len = 1;
        }
        p = p.offset(len as isize);
        if !(*p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != '\n' as i32) {
            nlines += 1;
            if p.offset_from(start) as ::core::ffi::c_long > 1
                && *p.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == '\r' as i32
            {
                p = p.offset(-(1 as ::core::ffi::c_int) as isize);
                memmove(
                    p.offset(-(1 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    strlen(p).wrapping_add(1),
                );
            }
            backslash = 0;
            p2 = p.offset(-(2 as ::core::ffi::c_int as isize));
            while p2 >= start {
                if *p2 as ::core::ffi::c_int != '\\' as i32 {
                    break;
                }
                backslash = (backslash == 0) as ::core::ffi::c_int;
                p2 = p2.offset(-(1 as ::core::ffi::c_int) as isize);
            }
            if backslash == 0 {
                *p.offset(-(1 as ::core::ffi::c_int) as isize) = 0;
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
        pfatal_with_name((*ebuf).floc.filenm);
    }
    if nlines != 0 {
        nlines
    } else {
        (if p == (*ebuf).bufstart {
            -(1 as ::core::ffi::c_int)
        } else {
            1
        }) as ::core::ffi::c_long
    }
}
unsafe extern "C" fn get_next_mword(
    buffer: *mut ::core::ffi::c_char,
    startp: *mut *mut ::core::ffi::c_char,
    length: *mut size_t,
) -> make_word_type {
    // True for words that need the static-text scan below; the special
    // separators (eol, ';', ':', '&:') set their type directly and skip it.
    let mut scan_static = false;
    let mut wtype: make_word_type = w_bogus;
    let mut p: *mut ::core::ffi::c_char = buffer;
    let beg: *mut ::core::ffi::c_char;
    let mut c: ::core::ffi::c_char;
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    beg = p;
    let fresh23 = p;
    p = p.offset(1 as ::core::ffi::c_int as isize);
    c = *fresh23;
    match c as ::core::ffi::c_int {
        0 => {
            wtype = w_eol;
        }
        59 => {
            wtype = w_semicolon;
        }
        58 => {
            wtype = w_colon;
            if *p as ::core::ffi::c_int == ':' as i32 {
                p = p.offset(1 as ::core::ffi::c_int as isize);
                wtype = w_dcolon;
            }
        }
        38 => {
            if *p as ::core::ffi::c_int == ':' as i32 {
                p = p.offset(1 as ::core::ffi::c_int as isize);
                if *p as ::core::ffi::c_int != ':' as i32 {
                    wtype = w_ampcolon;
                } else {
                    p = p.offset(1 as ::core::ffi::c_int as isize);
                    wtype = w_ampdcolon;
                }
            } else {
                scan_static = true;
            }
        }
        _ => {
            scan_static = true;
        }
    }
    if scan_static {
        wtype = w_static;
        while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(c as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int)
            != 0)
        {
            match c as ::core::ffi::c_int {
                58 => {
                    break;
                }
                36 => {
                    let fresh24 = p;
                    p = p.offset(1 as ::core::ffi::c_int as isize);
                    c = *fresh24;
                    if !(c as ::core::ffi::c_int == '$' as i32) {
                        if c as ::core::ffi::c_int == 0 {
                            break;
                        }
                        wtype = w_variable;
                        p = skip_reference(p.offset(-(1 as ::core::ffi::c_int as isize)));
                    }
                }
                92 => match *p as ::core::ffi::c_int {
                    58 | 59 | 61 | 92 => {
                        p = p.offset(1 as ::core::ffi::c_int as isize);
                    }
                    _ => {}
                },
                38 => {
                    if *p as ::core::ffi::c_int == ':' as i32 {
                        break;
                    }
                }
                _ => {}
            }
            let fresh25 = p;
            p = p.offset(1 as ::core::ffi::c_int as isize);
            c = *fresh25;
        }
        p = p.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    if !startp.is_null() {
        *startp = beg;
    }
    if !length.is_null() {
        *length = p.offset_from(beg) as ::core::ffi::c_long as size_t;
    }
    wtype
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn construct_include_path(mut arg_dirs: *mut *const ::core::ffi::c_char) {
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let dirs: *mut *const ::core::ffi::c_char;
    let mut cpp: *mut *const ::core::ffi::c_char;
    let mut idx: size_t;
    let mut disable: ::core::ffi::c_int = 0;
    idx = (::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>() as usize)
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        as size_t;
    if !arg_dirs.is_null() {
        cpp = arg_dirs;
        while !(*cpp).is_null() {
            idx = idx.wrapping_add(1);
            cpp = cpp.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    dirs = xmalloc(idx.wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t))
        as *mut *const ::core::ffi::c_char;
    idx = 0;
    max_incl_len = 0;
    if !arg_dirs.is_null() {
        while !(*arg_dirs).is_null() {
            let fresh0 = arg_dirs;
            arg_dirs = arg_dirs.offset(1 as ::core::ffi::c_int as isize);
            let mut dir: *const ::core::ffi::c_char = *fresh0;
            let mut expanded: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut e: ::core::ffi::c_int;
            if *dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
                && *dir.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
            {
                disable = 1;
                idx = 0;
                max_incl_len = 0;
            } else {
                if *dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '~' as i32
                {
                    expanded = tilde_expand(dir);
                    if !expanded.is_null() {
                        dir = expanded;
                    }
                }
                loop {
                    e = stat(dir, &raw mut stbuf);
                    if !(e == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                        break;
                    }
                }
                if e == 0 && stbuf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
                    let mut len: size_t = strlen(dir) as size_t;
                    while len > 1
                        && *dir.offset(len.wrapping_sub(1) as isize) as ::core::ffi::c_int
                            == '/' as i32
                    {
                        len = len.wrapping_sub(1);
                    }
                    if len > max_incl_len {
                        max_incl_len = len;
                    }
                    let fresh1 = idx;
                    idx = idx.wrapping_add(1);
                    let fresh2 = &mut (*dirs.offset(fresh1 as isize));
                    *fresh2 = strcache_add_len(dir, len);
                }
                free(expanded as *mut ::core::ffi::c_void);
            }
        }
    }
    if disable == 0 {
        let mut ccpp: *const *const ::core::ffi::c_char;
        ccpp = &raw const default_include_directories as *const *const ::core::ffi::c_char;
        while !(*ccpp).is_null() {
            let mut e_0: ::core::ffi::c_int;
            loop {
                e_0 = stat(*ccpp, &raw mut stbuf);
                if !(e_0 == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if e_0 == 0 && stbuf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
                let mut len_0: size_t = strlen(*ccpp) as size_t;
                while len_0 > 1
                    && *(*ccpp).offset(len_0.wrapping_sub(1) as isize) as ::core::ffi::c_int
                        == '/' as i32
                {
                    len_0 = len_0.wrapping_sub(1);
                }
                if len_0 > max_incl_len {
                    max_incl_len = len_0;
                }
                let fresh3 = idx;
                idx = idx.wrapping_add(1);
                let fresh4 = &mut (*dirs.offset(fresh3 as isize));
                *fresh4 = strcache_add_len(*ccpp, len_0);
            }
            ccpp = ccpp.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    let fresh5 = &mut (*dirs.offset(idx as isize));
    *fresh5 = ::core::ptr::null::<::core::ffi::c_char>();
    do_variable_definition(
        NILF,
        b".INCLUDE_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        o_default,
        f_simple,
        0,
        s_global,
    );
    cpp = dirs;
    while !(*cpp).is_null() {
        do_variable_definition(
            NILF,
            b".INCLUDE_DIRS\0" as *const u8 as *const ::core::ffi::c_char,
            *cpp,
            o_default,
            f_append,
            0,
            s_global,
        );
        cpp = cpp.offset(1 as ::core::ffi::c_int as isize);
    }
    free(include_directories as *mut ::core::ffi::c_void);
    include_directories = dirs;
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn tilde_expand(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    if *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
        || *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
    {
        let mut home_dir: *mut ::core::ffi::c_char;
        let is_variable: ::core::ffi::c_int;
        let save: Action = warning::action(Type::UndefinedVar);
        warning::set_action(Type::UndefinedVar, Action::Ignore);
        home_dir = allocated_expand_variable(
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as size_t).wrapping_sub(1),
        );
        warning::set_action(Type::UndefinedVar, save);
        is_variable = (*home_dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int;
        if is_variable == 0 {
            free(home_dir as *mut ::core::ffi::c_void);
            home_dir = getenv(b"HOME\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if home_dir.is_null()
            || *home_dir.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
        {
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
            let new: *mut ::core::ffi::c_char = xstrdup(concat(
                2,
                home_dir,
                name.offset(1 as ::core::ffi::c_int as isize),
            ));
            if is_variable != 0 {
                free(home_dir as *mut ::core::ffi::c_void);
            }
            return new;
        }
    } else {
        let pwent: *mut passwd;
        let userend: *mut ::core::ffi::c_char =
            strchr(name.offset(1 as ::core::ffi::c_int as isize), '/' as i32);
        if !userend.is_null() {
            *userend = 0;
        }
        pwent = getpwnam(name.offset(1 as ::core::ffi::c_int as isize));
        if !pwent.is_null() {
            if userend.is_null() {
                return xstrdup((*pwent).pw_dir);
            }
            *userend = '/' as i32 as ::core::ffi::c_char;
            return xstrdup(concat(
                3,
                (*pwent).pw_dir,
                b"/\0" as *const u8 as *const ::core::ffi::c_char,
                userend.offset(1 as ::core::ffi::c_int as isize),
            ));
        } else if !userend.is_null() {
            *userend = '/' as i32 as ::core::ffi::c_char;
        }
    }
    ::core::ptr::null_mut::<::core::ffi::c_char>()
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn parse_file_seq(
    stringp: *mut *mut ::core::ffi::c_char,
    mut stopmap: ::core::ffi::c_int,
    prefix: *const ::core::ffi::c_char,
    flags: ::core::ffi::c_int,
) -> *mut T {
    static mut tmpbuf: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    let cachep: ::core::ffi::c_int =
        !(flags & 0x10 as ::core::ffi::c_int != 0) as ::core::ffi::c_int;
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
    let mut findmap: ::core::ffi::c_int = stopmap | MAP_VMSCOMMA | MAP_NUL;
    let mut found_wait: ::core::ffi::c_int = 0;
    if !(flags & 0x20 as ::core::ffi::c_int != 0) {
        findmap |= MAP_BLANK;
    }
    stopmap |= MAP_NUL;
    if !(flags & 0x4 as ::core::ffi::c_int != 0) {
        // read.rs carries its own layout-identical glob_t; reconcile the
        // nominal types until the duplicate struct is unified.
        dir_setup_glob((&raw mut gl).cast());
    }
    static mut tmpbuf_len: size_t = 0;
    let l: size_t = (strlen(*stringp) as size_t).wrapping_add(1);
    if l > tmpbuf_len {
        tmpbuf = xrealloc(tmpbuf as *mut ::core::ffi::c_void, l) as *mut ::core::ffi::c_char;
        tmpbuf_len = l;
    }
    tp = tmpbuf;
    p = *stringp;
    loop {
        let mut name: *const ::core::ffi::c_char;
        let mut nlist: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut tildep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut globme: ::core::ffi::c_int = 1;
        let mut arname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut memname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut s: *mut ::core::ffi::c_char;
        let mut nlen: size_t;
        let mut tot: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int;
        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
            != 0
        {
            p = p.offset(1 as ::core::ffi::c_int as isize);
        }
        if *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
            .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
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
        if flags & 0x40 as ::core::ffi::c_int != 0
            && p.offset_from(s) as ::core::ffi::c_long as usize
                == (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize)
                    .wrapping_sub(1 as usize)
            && memcmp(
                s as *const ::core::ffi::c_void,
                b".WAIT\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t).wrapping_sub(1),
            ) == 0
        {
            found_wait = 1;
        } else {
            if !(flags & 0x1 as ::core::ffi::c_int != 0) {
                while p.offset_from(s) as ::core::ffi::c_long > 2
                    && *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '.' as i32
                    && *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '/' as i32
                {
                    s = s.offset(2 as ::core::ffi::c_int as isize);
                    while *s as ::core::ffi::c_int == '/' as i32 {
                        s = s.offset(1 as ::core::ffi::c_int as isize);
                    }
                }
            }
            if s == p {
                *tp.offset(0 as ::core::ffi::c_int as isize) = '.' as i32 as ::core::ffi::c_char;
                *tp.offset(1 as ::core::ffi::c_int as isize) = '/' as i32 as ::core::ffi::c_char;
                *tp.offset(2 as ::core::ffi::c_int as isize) = 0;
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
            if !(flags & 0x2 as ::core::ffi::c_int != 0)
                && tp == tmpbuf
                && *tp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '(' as i32
                && *tp.offset(nlen.wrapping_sub(1) as isize) as ::core::ffi::c_int != ')' as i32
            {
                let n: *mut ::core::ffi::c_char = strchr(tp, '(' as i32);
                if !n.is_null() {
                    let mut e: *const ::core::ffi::c_char = p;
                    loop {
                        let o: *const ::core::ffi::c_char = e;
                        while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                            .offset(*e as ::core::ffi::c_uchar as isize)
                            as ::core::ffi::c_int
                            & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
                            != 0
                        {
                            e = e.offset(1 as ::core::ffi::c_int as isize);
                        }
                        while !(*(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
                            .offset(*e as ::core::ffi::c_uchar as isize)
                            as ::core::ffi::c_int
                            & findmap
                            != 0)
                        {
                            e = e.offset(1 as ::core::ffi::c_int as isize);
                        }
                        if e == o {
                            break;
                        }
                        if *e.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                            == ')' as i32
                        {
                            nlen = nlen.wrapping_sub(
                                n.offset(1 as ::core::ffi::c_int as isize).offset_from(tp)
                                    as ::core::ffi::c_long
                                    as size_t,
                            );
                            tp = n.offset(1 as ::core::ffi::c_int as isize);
                            break;
                        } else if !(*e as ::core::ffi::c_int != 0) {
                            break;
                        }
                    }
                    if nlen == 0 {
                        continue;
                    }
                }
            }
            if tp > tmpbuf {
                if *tp.offset(nlen.wrapping_sub(1) as isize) as ::core::ffi::c_int == ')' as i32 {
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
            if flags & 0x4 as ::core::ffi::c_int != 0 {
                let mut _ns: *mut NameSeq = xcalloc(size) as *mut NameSeq;
                let mut __n: *const ::core::ffi::c_char = concat(2, prefix, tmpbuf);
                let ns = _ns
                    .as_mut()
                    .expect("parse_file_seq: xcalloc returned null nameseq");
                ns.name = if cachep != 0 {
                    strcache_add(__n)
                } else {
                    xstrdup(__n) as *const ::core::ffi::c_char
                });
                if found_wait != 0 {
                    let dep_ref = (ns as *mut nameseq as *mut dep)
                        .as_mut()
                        .expect("parse_file_seq: null wait marker dep");
                    dep_ref.set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    found_wait = 0;
                }
                *newp = _ns;
                newp = &raw mut ns.next;
            } else {
                name = tmpbuf;
                if *tmpbuf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '~' as i32
                {
                    tildep = tilde_expand(tmpbuf);
                    if !tildep.is_null() {
                        name = tildep;
                    }
                }
                if !(flags & 0x2 as ::core::ffi::c_int != 0) && ar_name(name) != 0 {
                    ar_parse_name(name, &raw mut arname, &raw mut memname);
                    name = arname;
                }
                if !(flags & 0x8 as ::core::ffi::c_int != 0)
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
                            tot = gl.gl_pathc as ::core::ffi::c_int;
                            nlist = gl.gl_pathv as *mut *const ::core::ffi::c_char;
                        }
                        GLOB_NOMATCH => {
                            if flags & 0x8 as ::core::ffi::c_int != 0 {
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
                            ar_glob::<T>(*nlist.offset(i as isize), memname);
                        if found.is_null() {
                            let _ns_0: *mut T = T::alloc();
                            let mut __n_0: *const ::core::ffi::c_char = concat(
                                5,
                                prefix,
                                *nlist.offset(i as isize),
                                b"(\0" as *const u8 as *const ::core::ffi::c_char,
                                memname,
                                b")\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            let ns = _ns_0
                                .as_mut()
                                .expect("parse_file_seq: xcalloc returned null archive nameseq");
                            ns.name = if cachep != 0 {
                                strcache_add(__n_0)
                            } else {
                                xstrdup(__n_0) as *const ::core::ffi::c_char
                            });
                            if found_wait != 0 {
                                let dep_ref = (ns as *mut nameseq as *mut dep)
                                    .as_mut()
                                    .expect("parse_file_seq: null archive wait marker dep");
                                dep_ref
                                    .set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                                found_wait = 0;
                            }
                            *newp = _ns_0;
                            newp = &raw mut ns.next;
                        } else {
                            if !(*newp).is_null() {
                                T::set_next(*newp, found);
                            } else {
                                *newp = found;
                            }
                            loop {
                                let found_ref = found
                                    .as_mut()
                                    .expect("parse_file_seq: null archive glob result");
                                if cachep == 0 {
                                    found_ref.name = xstrdup(concat(2, prefix, name));
                                } else if !prefix.is_null() {
                                    found_ref.name = strcache_add(concat(2, prefix, name));
                                }
                                if found_ref.next.is_null() {
                                    newp = &raw mut found_ref.next;
                                    break;
                                }
                                found = found_ref.next;
                            }
                        }
                    } else {
                        let _ns_1: *mut T = T::alloc();
                        let __n_1: *const ::core::ffi::c_char =
                            concat(2, prefix, *nlist.offset(i as isize));
                        let ns = _ns_1
                            .as_mut()
                            .expect("parse_file_seq: xcalloc returned null nameseq");
                        ns.name = if cachep != 0 {
                            strcache_add(__n_1)
                        } else {
                            xstrdup(__n_1) as *const ::core::ffi::c_char
                        });
                        if found_wait != 0 {
                            let dep_ref = (ns as *mut nameseq as *mut dep)
                                .as_mut()
                                .expect("parse_file_seq: null wait marker dep");
                            dep_ref.set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            found_wait = 0;
                        }
                        *newp = _ns_1;
                        newp = &raw mut ns.next;
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
