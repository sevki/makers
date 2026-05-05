use libc::{free, printf, strchr, strcmp, strcpy};
use ::c2rust_bitfields;
use crate::stdio::{_IO_codecvt, _IO_marker, _IO_wide_data, FILE};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
extern "C" {
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memrchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn skip_reference(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn lindex(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn print_spaces(_: ::core::ffi::c_uint);
    fn ar_name(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn file_exists_p(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn file_impossible_p(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn file_impossible(_: *const ::core::ffi::c_char);
    fn vpath_search(
        file: *const ::core::ffi::c_char,
        mtime_ptr: *mut uintmax_t,
        vpath_index: *mut ::core::ffi::c_uint,
        path_index: *mut ::core::ffi::c_uint,
    ) -> *const ::core::ffi::c_char;
    fn strcache_add(str: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn strcache_add_len(str: *const ::core::ffi::c_char, len: size_t)
        -> *const ::core::ffi::c_char;
    static mut stopchar_map: [::core::ffi::c_ushort; 0];
    static mut no_intermediates: ::core::ffi::c_uint;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn set_file_variables(file: *mut file, stem: *const ::core::ffi::c_char);
    static mut db_level: ::core::ffi::c_int;
    fn parse_file_seq(
        stringp: *mut *mut ::core::ffi::c_char,
        size: size_t,
        stopmap: ::core::ffi::c_int,
        prefix: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn free_ns_chain(n: *mut nameseq);
    fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn enter_file(name: *const ::core::ffi::c_char) -> *mut file;
    static mut pattern_rules: *mut rule;
    static mut num_pattern_rules: ::core::ffi::c_uint;
    static mut max_pattern_deps: ::core::ffi::c_uint;
    static mut max_pattern_targets: ::core::ffi::c_uint;
    static mut max_pattern_dep_length: size_t;
    fn get_rule_defn(rule: *mut rule) -> *const ::core::ffi::c_char;
    fn shuffle_deps_recursive(g: *mut dep);
    fn expand_string_for_file(
        string: *const ::core::ffi::c_char,
        file: *mut file,
    ) -> *mut ::core::ffi::c_char;
    fn free_variable_set(_: *mut variable_set_list);
    fn initialize_file_variables(file: *mut file, reading: ::core::ffi::c_int);
    fn merge_variable_set_lists(
        to_list: *mut *mut variable_set_list,
        from_list: *mut variable_set_list,
    );
    fn define_variable_in_set(
        name: *const ::core::ffi::c_char,
        length: size_t,
        value: *const ::core::ffi::c_char,
        origin: variable_origin,
        recursive: ::core::ffi::c_int,
        set: *mut variable_set,
        flocp: *const Floc,
    ) -> *mut variable;
}
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uintmax_t = ::libc::uintmax_t;
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
pub type dep = Dep;
pub type commands = Commands;
use crate::floc::Floc;

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
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub next: *mut rule,
    pub targets: *mut *const ::core::ffi::c_char,
    pub lens: *mut ::core::ffi::c_uint,
    pub suffixes: *mut *const ::core::ffi::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub _defn: *mut ::core::ffi::c_char,
    pub num: ::core::ffi::c_ushort,
    pub terminal: ::core::ffi::c_char,
    pub in_use: ::core::ffi::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct patdeps {
    pub name: *const ::core::ffi::c_char,
    pub pattern: *const ::core::ffi::c_char,
    pub file: *mut file,
    #[bitfield(name = "ignore_mtime", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(
        name = "ignore_automatic_vars",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(name = "is_explicit", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "wait_here", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub ignore_mtime_ignore_automatic_vars_is_explicit_wait_here: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tryrule {
    pub rule: *mut rule,
    pub stemlen: size_t,
    pub matches: ::core::ffi::c_uint,
    pub order: ::core::ffi::c_uint,
    pub checked_lastslash: ::core::ffi::c_char,
}
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const GET_PATH_MAX: ::core::ffi::c_int = PATH_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 72] = unsafe {
    ::core::mem::transmute::<[u8; 72], [::core::ffi::c_char; 72]>(
        *b"int pattern_search(struct file *, int, unsigned int, unsigned int, int)\0",
    )
};
#[inline]
#[no_mangle]
pub unsafe extern "C" fn alloc_dep() -> *mut dep {
    xcalloc(::core::mem::size_of::<dep>() as size_t) as *mut dep
}
#[inline]

unsafe extern "C" fn free_dep_chain(mut d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
#[no_mangle]
pub unsafe extern "C" fn try_implicit_rule(
    mut file: *mut file,
    mut depth: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if 0x8 as ::core::ffi::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            b"Looking for an implicit rule for '%s'.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*file).name,
        );
        fflush(stdout);
    }
    if pattern_search(
        file,
        0,
        depth,
        0,
        0,
    ) != 0
    {
        return 1;
    }
    if ar_name((*file).name) != 0 {
        if 0x8 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Looking for archive-member implicit rule for '%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
        if pattern_search(
            file,
            1,
            depth,
            0,
            0,
        ) != 0
        {
            return 1;
        }
        if 0x8 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"No archive-member implicit rule found for '%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*file).name,
            );
            fflush(stdout);
        }
    }
    0
}
unsafe extern "C" fn get_next_word(
    mut buffer: *const ::core::ffi::c_char,
    mut length: *mut size_t,
) -> *const ::core::ffi::c_char {
    let mut current_block: u64;
    let mut p: *const ::core::ffi::c_char = buffer;
    let mut beg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_char = 0;
    while *(&raw mut stopchar_map as *mut ::core::ffi::c_ushort)
        .offset(*p as ::core::ffi::c_uchar as isize) as ::core::ffi::c_int
        & (0x2 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int)
        != 0
    {
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    beg = p;
    let fresh9 = p;
    p = p.offset(1 as ::core::ffi::c_int as isize);
    c = *fresh9;
    if c as ::core::ffi::c_int == 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    loop {
        match c as ::core::ffi::c_int {
            0 | 32 | 9 => {
                current_block = 1785022817647621871;
                break;
            }
            36 => {
                p = skip_reference(p);
            }
            124 => {
                current_block = 12230172844548880253;
                break;
            }
            _ => {}
        }
        let fresh10 = p;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        c = *fresh10;
    }
    match current_block {
        1785022817647621871 => {
            p = p.offset(-(1 as ::core::ffi::c_int) as isize);
        }
        _ => {}
    }
    if !length.is_null() {
        *length = p.offset_from(beg) as ::core::ffi::c_long as size_t;
    }
    beg
}
#[no_mangle]
pub unsafe extern "C" fn stemlen_compare(
    mut v1: *const ::core::ffi::c_void,
    mut v2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut r1: *const tryrule = v1 as *const tryrule;
    let mut r2: *const tryrule = v2 as *const tryrule;
    let mut r: ::core::ffi::c_int = (*r1).stemlen.wrapping_sub((*r2).stemlen) as ::core::ffi::c_int;
    if r != 0 {
        r
    } else {
        (*r1).order.wrapping_sub((*r2).order) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn pattern_search(
    mut file: *mut file,
    mut archive: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_uint,
    mut recursions: ::core::ffi::c_uint,
    mut allow_compat_rules: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut filename: *const ::core::ffi::c_char = if archive != 0 {
        strchr((*file).name, '(' as i32) as *const ::core::ffi::c_char
    } else {
        (*file).name
    };
    let mut namelen: size_t = strlen(filename) as size_t;
    let mut lastslash: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut int_file: *mut file = ::core::ptr::null_mut::<file>();
    let mut max_deps: ::core::ffi::c_uint = max_pattern_deps;
    let mut deplist: *mut patdeps =
        xmalloc((max_deps as size_t).wrapping_mul(::core::mem::size_of::<patdeps>() as size_t))
            as *mut patdeps;
    let mut pat: *mut patdeps = deplist;
    let mut deplen: size_t = namelen
        .wrapping_add(max_pattern_dep_length)
        .wrapping_add(4);
    alloca_allocations.push(::std::vec::from_elem(0, deplen as usize));
    let mut depname: *mut ::core::ffi::c_char =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    let mut dend: *mut ::core::ffi::c_char = depname.offset(deplen as isize);
    let mut stem: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut stemlen: size_t = 0;
    let mut fullstemlen: size_t = 0;
    let mut tryrules: *mut tryrule = xmalloc(
        (num_pattern_rules.wrapping_mul(max_pattern_targets) as size_t)
            .wrapping_mul(::core::mem::size_of::<tryrule>() as size_t),
    ) as *mut tryrule;
    let mut nrules: ::core::ffi::c_uint = 0;
    let mut foundrule: ::core::ffi::c_uint = 0;
    let mut intermed_ok: ::core::ffi::c_int = 0;
    let mut file_vars_initialized: ::core::ffi::c_int = 0;
    let mut specific_rule_matched: ::core::ffi::c_int = 0;
    let mut ri: ::core::ffi::c_uint = 0;
    let mut found_compat_rule: ::core::ffi::c_int = 0;
    let mut rule: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut pathdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pathlen: size_t = 0;
    let mut stem_str: [::core::ffi::c_char; 4097] = [0; 4097];
    depth = depth.wrapping_add(1);
    if archive != 0 || ar_name(filename) != 0 {
        lastslash = ::core::ptr::null::<::core::ffi::c_char>();
    } else {
        lastslash = memrchr(
            filename as *const ::core::ffi::c_void,
            '/' as i32,
            (namelen as size_t).wrapping_sub(1),
        ) as *const ::core::ffi::c_char;
    }
    pathlen = (if !lastslash.is_null() {
        lastslash.offset_from(filename) as ::core::ffi::c_long + 1
    } else {
        0
    }) as size_t;
    nrules = 0;
    rule = pattern_rules;
    while !rule.is_null() {
        let mut ti: ::core::ffi::c_uint = 0;
        if !(!(*rule).deps.is_null() && (*rule).cmds.is_null()) {
            if (*rule).in_use != 0 {
                if 0x8 as ::core::ffi::c_int & db_level != 0 {
                    print_spaces(depth);
                    printf(
                        b"Avoiding implicit rule recursion for rule '%s'.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        get_rule_defn(rule),
                    );
                    fflush(stdout);
                }
            } else {
                let mut current_block_31: u64;
                ti = 0;
                while ti < (*rule).num as ::core::ffi::c_uint {
                    let mut target: *const ::core::ffi::c_char =
                        *(*rule).targets.offset(ti as isize);
                    let mut suffix: *const ::core::ffi::c_char =
                        *(*rule).suffixes.offset(ti as isize);
                    let mut check_lastslash: ::core::ffi::c_char = 0;
                    if !(recursions > 0
                        && *target.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0
                        && (*rule).terminal == 0)
                    {
                        if !(*(*rule).lens.offset(ti as isize) as size_t > namelen) {
                            stem = filename.offset(
                                (suffix.offset_from(target) as ::core::ffi::c_long
                                    - 1)
                                    as isize,
                            );
                            stemlen = namelen
                                .wrapping_sub(*(*rule).lens.offset(ti as isize) as size_t)
                                .wrapping_add(1);
                            check_lastslash = 0;
                            if !lastslash.is_null() {
                                check_lastslash = (strchr(target, '/' as i32)
                                    == ::core::ptr::null_mut::<::core::ffi::c_char>())
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_char;
                            }
                            if check_lastslash != 0 {
                                if pathlen > stemlen {
                                    current_block_31 = 18386322304582297246;
                                } else {
                                    stemlen = stemlen.wrapping_sub(pathlen);
                                    stem = stem.offset(pathlen as isize);
                                    current_block_31 = 14832935472441733737;
                                }
                            } else {
                                current_block_31 = 14832935472441733737;
                            }
                            match current_block_31 {
                                18386322304582297246 => {}
                                _ => {
                                    if check_lastslash != 0 {
                                        if stem > lastslash.offset(1 as ::core::ffi::c_int as isize)
                                            && !(strncmp(
                                                target,
                                                lastslash.offset(1 as ::core::ffi::c_int as isize),
                                                (stem.offset_from(lastslash) as ::core::ffi::c_long
                                                    - 1)
                                                    as size_t,
                                            ) == 0)
                                        {
                                            current_block_31 = 18386322304582297246;
                                        } else {
                                            current_block_31 = 17784502470059252271;
                                        }
                                    } else if stem > filename
                                        && !(strncmp(
                                            target,
                                            filename,
                                            stem.offset_from(filename) as ::core::ffi::c_long
                                                as size_t,
                                        ) == 0)
                                    {
                                        current_block_31 = 18386322304582297246;
                                    } else {
                                        current_block_31 = 17784502470059252271;
                                    }
                                    match current_block_31 {
                                        18386322304582297246 => {}
                                        _ => {
                                            if !(*suffix as ::core::ffi::c_int
                                                != *stem.offset(stemlen as isize)
                                                    as ::core::ffi::c_int
                                                || *suffix as ::core::ffi::c_int != 0
                                                    && !(*suffix . offset ( 1 ) as ::core::ffi::c_int
                                                        == *stem.offset(
                                                            stemlen.wrapping_add(1)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                        && (*suffix.offset(
                                                            1 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == 0
                                                            || strcmp(
                                                                (suffix.offset(
                                                                    1 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                    as *const ::core::ffi::c_char)
                                                                    .offset(
                                                                        1 as ::core::ffi::c_int
                                                                            as isize,
                                                                    ),
                                                                (stem.offset(
                                                                    stemlen
                                                                        .wrapping_add(1)
                                                                        as isize,
                                                                )
                                                                    as *const ::core::ffi::c_char)
                                                                    .offset(
                                                                        1 as ::core::ffi::c_int
                                                                            as isize,
                                                                    ),
                                                            ) == 0)))
                                            {
                                                if *target . offset ( 1 ) as ::core::ffi::c_int
                                                    != 0
                                                {
                                                    specific_rule_matched = 1;
                                                }
                                                if !((*rule).deps.is_null()
                                                    && (*rule).cmds.is_null())
                                                {
                                                    let ref mut fresh0 =
                                                        (*tryrules.offset(nrules as isize)).rule;
                                                    *fresh0 = rule;
                                                    (*tryrules.offset(nrules as isize)).matches =
                                                        ti;
                                                    (*tryrules.offset(nrules as isize)).stemlen =
                                                        stemlen.wrapping_add(
                                                            if check_lastslash
                                                                as ::core::ffi::c_int
                                                                != 0
                                                            {
                                                                pathlen
                                                            } else {
                                                                0
                                                            },
                                                        );
                                                    (*tryrules.offset(nrules as isize)).order =
                                                        nrules;
                                                    (*tryrules.offset(nrules as isize))
                                                        .checked_lastslash = check_lastslash;
                                                    nrules = nrules.wrapping_add(1);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ti = ti.wrapping_add(1);
                }
            }
        }
        rule = (*rule).next;
    }
    if !(nrules == 0) {
        if nrules > 1 {
            qsort(
                tryrules as *mut ::core::ffi::c_void,
                nrules as size_t,
                ::core::mem::size_of::<tryrule>() as size_t,
                Some(
                    stemlen_compare
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
            );
        }
        if specific_rule_matched != 0 {
            ri = 0;
            while ri < nrules {
                if (*(*tryrules.offset(ri as isize)).rule).terminal == 0 {
                    let mut j: ::core::ffi::c_uint = 0;
                    j = 0;
                    while j < (*(*tryrules.offset(ri as isize)).rule).num as ::core::ffi::c_uint {
                        if *(*(*(*tryrules.offset(ri as isize)).rule)
                            .targets
                            .offset(j as isize)) . offset ( 1 ) as ::core::ffi::c_int
                            == 0
                        {
                            let ref mut fresh1 = (*tryrules.offset(ri as isize)).rule;
                            *fresh1 = ::core::ptr::null_mut::<rule>();
                            break;
                        } else {
                            j = j.wrapping_add(1);
                        }
                    }
                }
                ri = ri.wrapping_add(1);
            }
        }
        intermed_ok = 0;
        while intermed_ok < 2 {
            pat = deplist;
            if intermed_ok != 0 {
                if 0x8 as ::core::ffi::c_int & db_level != 0 {
                    print_spaces(depth);
                    printf(b"Trying harder.\n\0" as *const u8 as *const ::core::ffi::c_char);
                    fflush(stdout);
                }
            }
            ri = 0;
            while ri < nrules {
                let mut dep: *mut dep = ::core::ptr::null_mut::<dep>();
                let mut check_lastslash_0: ::core::ffi::c_char = 0;
                let mut failed: ::core::ffi::c_uint = 0;
                let mut file_variables_set: ::core::ffi::c_int = 0;
                let mut deps_found: ::core::ffi::c_uint = 0;
                let mut nptr: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut order_only: ::core::ffi::c_int = 0;
                let mut matches: ::core::ffi::c_uint = 0;
                rule = (*tryrules.offset(ri as isize)).rule;
                if !rule.is_null() {
                    if !(intermed_ok != 0 && (*rule).terminal as ::core::ffi::c_int != 0) {
                        matches = (*tryrules.offset(ri as isize)).matches;
                        stem = filename
                            .offset(
                                (*(*rule).suffixes.offset(matches as isize))
                                    .offset_from(*(*rule).targets.offset(matches as isize))
                                    as ::core::ffi::c_long as isize,
                            )
                            .offset(-(1 as ::core::ffi::c_int as isize));
                        stemlen = namelen
                            .wrapping_sub(*(*rule).lens.offset(matches as isize) as size_t)
                            .wrapping_add(1);
                        check_lastslash_0 = (*tryrules.offset(ri as isize)).checked_lastslash;
                        if check_lastslash_0 != 0 {
                            stem = stem.offset(pathlen as isize);
                            stemlen = stemlen.wrapping_sub(pathlen);
                            if pathdir.is_null() {
                                alloca_allocations.push(::std::vec::from_elem(
                                    0,
                                    pathlen.wrapping_add(1) as usize,
                                ));
                                pathdir = alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                    as *mut ::core::ffi::c_char;
                                memcpy(
                                    pathdir as *mut ::core::ffi::c_void,
                                    filename as *const ::core::ffi::c_void,
                                    pathlen as size_t,
                                );
                                *pathdir.offset(pathlen as isize) =
                                    0;
                            }
                        }
                        if 0x8 as ::core::ffi::c_int & db_level != 0 {
                            print_spaces(depth);
                            printf(
                                b"Trying pattern rule '%s' with stem '%.*s'.\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                get_rule_defn(rule),
                                stemlen as ::core::ffi::c_int,
                                stem,
                            );
                            fflush(stdout);
                        }
                        if stemlen.wrapping_add(
                            if check_lastslash_0 as ::core::ffi::c_int != 0 {
                                pathlen
                            } else {
                                0
                            },
                        ) > GET_PATH_MAX as size_t
                        {
                            if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                print_spaces(depth);
                                printf(
                                    b"Stem too long: '%s%.*s'.\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    if check_lastslash_0 as ::core::ffi::c_int != 0 {
                                        pathdir as *const ::core::ffi::c_char
                                    } else {
                                        b"\0" as *const u8 as *const ::core::ffi::c_char
                                    },
                                    stemlen as ::core::ffi::c_int,
                                    stem,
                                );
                                fflush(stdout);
                            }
                        } else {
                            if check_lastslash_0 == 0 {
                                memcpy(
                                    &raw mut stem_str as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                    stem as *const ::core::ffi::c_void,
                                    stemlen as size_t,
                                );
                                stem_str[stemlen as usize] = 0;
                            } else {
                                memcpy(
                                    &raw mut stem_str as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                    filename as *const ::core::ffi::c_void,
                                    pathlen as size_t,
                                );
                                memcpy(
                                    (&raw mut stem_str as *mut ::core::ffi::c_char)
                                        .offset(pathlen as isize)
                                        as *mut ::core::ffi::c_void,
                                    stem as *const ::core::ffi::c_void,
                                    stemlen as size_t,
                                );
                                stem_str[pathlen.wrapping_add(stemlen) as usize] =
                                    0;
                            }
                            if (*rule).deps.is_null() {
                                break;
                            }
                            (*rule).in_use = 1;
                            pat = deplist;
                            dep = (*rule).deps;
                            nptr = if !(*dep).name.is_null() {
                                (*dep).name
                            } else {
                                (*(*dep).file).name
                            };
                            loop {
                                let mut dl: *mut dep = ::core::ptr::null_mut::<dep>();
                                let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
                                if nptr.is_null() {
                                    dep = (*dep).next;
                                    if dep.is_null() {
                                        break;
                                    }
                                    nptr = if !(*dep).name.is_null() {
                                        (*dep).name
                                    } else {
                                        (*(*dep).file).name
                                    };
                                }
                                if (*dep).need_2nd_expansion() == 0 {
                                    let mut p: *mut ::core::ffi::c_char =
                                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                                    let mut is_explicit: ::core::ffi::c_int =
                                        1;
                                    let mut cp: *const ::core::ffi::c_char =
                                        strchr(nptr, '%' as i32);
                                    if cp.is_null() {
                                        strcpy(depname, nptr);
                                    } else {
                                        let mut o: *mut ::core::ffi::c_char = depname;
                                        if check_lastslash_0 != 0 {
                                            o = mempcpy(
                                                o as *mut ::core::ffi::c_void,
                                                filename as *const ::core::ffi::c_void,
                                                pathlen as size_t,
                                            )
                                                as *mut ::core::ffi::c_char;
                                        }
                                        o = mempcpy(
                                            o as *mut ::core::ffi::c_void,
                                            nptr as *const ::core::ffi::c_void,
                                            cp.offset_from(nptr) as ::core::ffi::c_long as size_t,
                                        )
                                            as *mut ::core::ffi::c_char;
                                        o = mempcpy(
                                            o as *mut ::core::ffi::c_void,
                                            stem as *const ::core::ffi::c_void,
                                            stemlen as size_t,
                                        )
                                            as *mut ::core::ffi::c_char;
                                        strcpy(o, cp.offset(1 as ::core::ffi::c_int as isize));
                                        is_explicit = 0;
                                    }
                                    p = depname;
                                    dl = parse_file_seq(
                                        &raw mut p,
                                        ::core::mem::size_of::<dep>() as size_t,
                                        0x1 as ::core::ffi::c_int,
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        0x20 as ::core::ffi::c_int | 0x40 as ::core::ffi::c_int,
                                    ) as *mut dep;
                                    d = dl;
                                    while !d.is_null() {
                                        deps_found = deps_found.wrapping_add(1);
                                        (*d).set_ignore_mtime(
                                            (*dep).ignore_mtime() as ::core::ffi::c_uint
                                        );
                                        (*d).set_ignore_automatic_vars(
                                            (*dep).ignore_automatic_vars() as ::core::ffi::c_uint,
                                        );
                                        (*d).set_wait_here(
                                            (*d).wait_here()
                                                | (*dep).wait_here() as ::core::ffi::c_int
                                                    as ::core::ffi::c_uint,
                                        );
                                        (*d).set_is_explicit(
                                            is_explicit as ::core::ffi::c_uint
                                                as ::core::ffi::c_uint,
                                        );
                                        d = (*d).next;
                                    }
                                    nptr = ::core::ptr::null::<::core::ffi::c_char>();
                                } else {
                                    let mut add_dir: ::core::ffi::c_int = 0;
                                    let mut len: size_t = 0;
                                    let mut end: *const ::core::ffi::c_char =
                                        ::core::ptr::null::<::core::ffi::c_char>();
                                    let mut dptr: *mut *mut dep =
                                        ::core::ptr::null_mut::<*mut dep>();
                                    let mut is_explicit_0: ::core::ffi::c_int = 0;
                                    let mut cp_0: *const ::core::ffi::c_char =
                                        ::core::ptr::null::<::core::ffi::c_char>();
                                    let mut p_0: *mut ::core::ffi::c_char =
                                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                                    nptr = get_next_word(nptr, &raw mut len);
                                    if nptr.is_null() {
                                        continue;
                                    }
                                    end = nptr.offset(len as isize);
                                    if order_only == 0
                                        && len == 1
                                        && *nptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '|' as i32
                                    {
                                        order_only = 1;
                                        nptr = end;
                                        continue;
                                    } else {
                                        cp_0 = lindex(nptr, end, '%' as i32);
                                        if cp_0.is_null() {
                                            memcpy(
                                                depname as *mut ::core::ffi::c_void,
                                                nptr as *const ::core::ffi::c_void,
                                                len as size_t,
                                            );
                                            *depname.offset(len as isize) =
                                                0;
                                            is_explicit_0 = 1;
                                        } else {
                                            let mut o_0: *mut ::core::ffi::c_char = depname;
                                            is_explicit_0 = 0;
                                            loop {
                                                let mut i: size_t = cp_0.offset_from(nptr)
                                                    as ::core::ffi::c_long
                                                    as size_t;
                                                '_c2rust_label: {
                                                    if o_0.offset(i as isize) < dend {
                                                    } else {
                                                        __assert_fail(
                                                            b"o + i < dend\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"src/implicit.c\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            632,
                                                            __ASSERT_FUNCTION.as_ptr(),
                                                        );
                                                    }
                                                };
                                                o_0 = mempcpy(
                                                    o_0 as *mut ::core::ffi::c_void,
                                                    nptr as *const ::core::ffi::c_void,
                                                    i as size_t,
                                                )
                                                    as *mut ::core::ffi::c_char;
                                                if check_lastslash_0 != 0 {
                                                    add_dir = 1;
                                                    '_c2rust_label_0: {
                                                        if o_0.offset(
                                                            5 as ::core::ffi::c_int as isize,
                                                        ) < dend
                                                        {
                                                        } else {
                                                            __assert_fail(
                                                                b"o + 5 < dend\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                b"src/implicit.c\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                637,
                                                                __ASSERT_FUNCTION.as_ptr(),
                                                            );
                                                        }
                                                    };
                                                    o_0 = mempcpy(
                                                        o_0 as *mut ::core::ffi::c_void,
                                                        b"$(*F)\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                            as *const ::core::ffi::c_void,
                                                        5,
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                } else {
                                                    '_c2rust_label_1: {
                                                        if o_0.offset(
                                                            2 as ::core::ffi::c_int as isize,
                                                        ) < dend
                                                        {
                                                        } else {
                                                            __assert_fail(
                                                                b"o + 2 < dend\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                b"src/implicit.c\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                642,
                                                                __ASSERT_FUNCTION.as_ptr(),
                                                            );
                                                        }
                                                    };
                                                    o_0 = mempcpy(
                                                        o_0 as *mut ::core::ffi::c_void,
                                                        b"$*\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                            as *const ::core::ffi::c_void,
                                                        2,
                                                    )
                                                        as *mut ::core::ffi::c_char;
                                                }
                                                '_c2rust_label_2: {
                                                    if o_0 < dend {
                                                    } else {
                                                        __assert_fail(
                                                            b"o < dend\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"src/implicit.c\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            645,
                                                            __ASSERT_FUNCTION.as_ptr(),
                                                        );
                                                    }
                                                };
                                                cp_0 = cp_0.offset(1 as ::core::ffi::c_int as isize);
                                                '_c2rust_label_3: {
                                                    if cp_0 <= end {
                                                    } else {
                                                        __assert_fail(
                                                            b"cp <= end\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"src/implicit.c\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            647,
                                                            __ASSERT_FUNCTION.as_ptr(),
                                                        );
                                                    }
                                                };
                                                nptr = cp_0;
                                                if nptr == end {
                                                    break;
                                                }
                                                while cp_0 < end
                                                    && !(*(&raw mut stopchar_map
                                                        as *mut ::core::ffi::c_ushort)
                                                        .offset(
                                                            *cp_0 as ::core::ffi::c_uchar as isize,
                                                        )
                                                        as ::core::ffi::c_int
                                                        & (0x2 as ::core::ffi::c_int
                                                            | 0x4 as ::core::ffi::c_int
                                                            | 0x1 as ::core::ffi::c_int)
                                                        != 0)
                                                {
                                                    cp_0 = cp_0.offset(1 as ::core::ffi::c_int as isize);
                                                }
                                                cp_0 = lindex(cp_0, end, '%' as i32);
                                                if cp_0.is_null() {
                                                    break;
                                                }
                                            }
                                            len = end.offset_from(nptr) as ::core::ffi::c_long
                                                as size_t;
                                            memcpy(
                                                o_0 as *mut ::core::ffi::c_void,
                                                nptr as *const ::core::ffi::c_void,
                                                len as size_t,
                                            );
                                            *o_0.offset(len as isize) =
                                                0;
                                        }
                                        nptr = end;
                                        if file_vars_initialized == 0 {
                                            initialize_file_variables(
                                                file,
                                                0,
                                            );
                                            set_file_variables(
                                                file,
                                                &raw mut stem_str as *mut ::core::ffi::c_char,
                                            );
                                            file_vars_initialized = 1;
                                        } else if file_variables_set == 0 {
                                            define_variable_in_set(
                                                b"*\0" as *const u8 as *const ::core::ffi::c_char,
                                                1,
                                                &raw mut stem_str as *mut ::core::ffi::c_char,
                                                o_automatic,
                                                0,
                                                (*(*file).variables).set,
                                                NILF,
                                            );
                                            file_variables_set = 1;
                                        }
                                        p_0 = expand_string_for_file(depname, file);
                                        dptr = &raw mut dl;
                                        loop {
                                            let mut dp: *mut dep = parse_file_seq(
                                                &raw mut p_0,
                                                ::core::mem::size_of::<dep>() as size_t,
                                                if order_only != 0 {
                                                    0x1 as ::core::ffi::c_int
                                                } else {
                                                    0x100 as ::core::ffi::c_int
                                                },
                                                if add_dir != 0 {
                                                    pathdir
                                                } else {
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                                                },
                                                0x40 as ::core::ffi::c_int,
                                            )
                                                as *mut dep;
                                            *dptr = dp;
                                            d = dp;
                                            while !d.is_null() {
                                                deps_found = deps_found.wrapping_add(1);
                                                if order_only != 0 {
                                                    (*d).set_ignore_mtime(
                                                        1 as ::core::ffi::c_uint
                                                            as ::core::ffi::c_uint,
                                                    );
                                                }
                                                (*d).set_is_explicit(
                                                    is_explicit_0 as ::core::ffi::c_uint
                                                        as ::core::ffi::c_uint,
                                                );
                                                dptr = &raw mut (*d).next;
                                                d = (*d).next;
                                            }
                                            if *p_0 as ::core::ffi::c_int == '|' as i32 {
                                                order_only = 1;
                                                p_0 = p_0.offset(1 as ::core::ffi::c_int as isize);
                                            }
                                            if !(*p_0 as ::core::ffi::c_int != 0) {
                                                break;
                                            }
                                        }
                                    }
                                }
                                if deps_found > max_deps {
                                    let mut l: size_t =
                                        pat.offset_from(deplist) as ::core::ffi::c_long as size_t;
                                    max_pattern_deps = if max_pattern_deps > deps_found {
                                        max_pattern_deps
                                    } else {
                                        deps_found
                                    };
                                    max_deps = max_pattern_deps;
                                    deplist =
                                        xrealloc(
                                            deplist as *mut ::core::ffi::c_void,
                                            (max_deps as size_t).wrapping_mul(
                                                ::core::mem::size_of::<patdeps>() as size_t,
                                            ),
                                        ) as *mut patdeps;
                                    pat = deplist.offset(l as isize);
                                }
                                let mut current_block_294: u64;
                                d = dl;
                                while !d.is_null() {
                                    let mut df: *mut file = ::core::ptr::null_mut::<file>();
                                    let mut is_rule: ::core::ffi::c_int = ((*d).name
                                        == (if !(*dep).name.is_null() {
                                            (*dep).name
                                        } else {
                                            (*(*dep).file).name
                                        }))
                                        as ::core::ffi::c_int;
                                    let mut explicit: ::core::ffi::c_int = 0;
                                    let mut dp_0: *mut dep = ::core::ptr::null_mut::<dep>();
                                    if file_impossible_p((*d).name) != 0 {
                                        if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                            print_spaces(depth);
                                            printf(
                                                if is_rule != 0 {
                                                    b"Rejecting rule '%s' due to impossible rule prerequisite '%s'.\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                } else {
                                                    b"Rejecting rule '%s' due to impossible implicit prerequisite '%s'.\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char
                                                },
                                                get_rule_defn(rule),
                                                (*d).name,
                                            );
                                            fflush(stdout);
                                        }
                                        let ref mut fresh2 = (*tryrules.offset(ri as isize)).rule;
                                        *fresh2 = ::core::ptr::null_mut::<rule>();
                                        failed = 1;
                                        break;
                                    } else {
                                        memset(
                                            pat as *mut ::core::ffi::c_void,
                                            0,
                                            ::core::mem::size_of::<patdeps>() as size_t,
                                        );
                                        (*pat).set_ignore_mtime(
                                            (*d).ignore_mtime() as ::core::ffi::c_uint
                                        );
                                        (*pat).set_ignore_automatic_vars(
                                            (*d).ignore_automatic_vars() as ::core::ffi::c_uint,
                                        );
                                        (*pat)
                                            .set_wait_here((*d).wait_here() as ::core::ffi::c_uint);
                                        (*pat).set_is_explicit(
                                            (*d).is_explicit() as ::core::ffi::c_uint
                                        );
                                        if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                            print_spaces(depth);
                                            printf(
                                                if is_rule != 0 {
                                                    b"Trying rule prerequisite '%s'.\n\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char
                                                } else {
                                                    b"Trying implicit prerequisite '%s'.\n\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char
                                                },
                                                (*d).name,
                                            );
                                            fflush(stdout);
                                        }
                                        df = lookup_file((*d).name);
                                        if !df.is_null()
                                            && (*df).is_explicit() as ::core::ffi::c_int != 0
                                        {
                                            (*pat).set_is_explicit(
                                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                            );
                                        }
                                        if !df.is_null()
                                            && (*df).is_explicit() == 0
                                            && (*d).is_explicit() == 0
                                        {
                                            (*df).set_intermediate(
                                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                            );
                                        }
                                        if !df.is_null()
                                            && (*df).is_target() as ::core::ffi::c_int != 0
                                        {
                                            explicit = 1;
                                        } else {
                                            dp_0 = (*file).deps;
                                            while !dp_0.is_null() {
                                                if *(*d).name as ::core::ffi::c_int
                                                    == *(if !(*dp_0).name.is_null() {
                                                        (*dp_0).name
                                                    } else {
                                                        (*(*dp_0).file).name
                                                    })
                                                        as ::core::ffi::c_int
                                                    && (*(*d).name as ::core::ffi::c_int
                                                        == 0
                                                        || strcmp(
                                                            (*d).name.offset(
                                                                1 as ::core::ffi::c_int as isize,
                                                            ),
                                                            (if !(*dp_0).name.is_null() {
                                                                (*dp_0).name
                                                            } else {
                                                                (*(*dp_0).file).name
                                                            })
                                                            .offset(
                                                                1 as ::core::ffi::c_int as isize,
                                                            ),
                                                        ) == 0)
                                                {
                                                    break;
                                                }
                                                dp_0 = (*dp_0).next;
                                            }
                                        }
                                        if explicit != 0 || !dp_0.is_null() {
                                            let fresh3 = pat;
                                            pat = pat.offset(1 as ::core::ffi::c_int as isize);
                                            (*fresh3).name = (*d).name;
                                            if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    b"'%s' ought to exist.\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else if file_exists_p((*d).name) != 0 {
                                            let fresh4 = pat;
                                            pat = pat.offset(1 as ::core::ffi::c_int as isize);
                                            (*fresh4).name = (*d).name;
                                            if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    b"Found '%s'.\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else if !df.is_null() && allow_compat_rules != 0 {
                                            let fresh5 = pat;
                                            pat = pat.offset(1 as ::core::ffi::c_int as isize);
                                            (*fresh5).name = (*d).name;
                                            if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    b"Using compatibility rule '%s' due to '%s'.\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char,
                                                    get_rule_defn(rule),
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else {
                                            if !df.is_null() {
                                                if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                                    print_spaces(depth);
                                                    printf(
                                                        b"Prerequisite '%s' of rule '%s' does not qualify as ought to exist.\n\0"
                                                            as *const u8 as *const ::core::ffi::c_char,
                                                        (*d).name,
                                                        get_rule_defn(rule),
                                                    );
                                                    fflush(stdout);
                                                }
                                                found_compat_rule = 1;
                                            }
                                            let mut vname: *const ::core::ffi::c_char =
                                                vpath_search(
                                                    (*d).name,
                                                    ::core::ptr::null_mut::<uintmax_t>(),
                                                    ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                                    ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                                );
                                            if !vname.is_null() {
                                                if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                                    print_spaces(depth);
                                                    printf(
                                                        b"Found prerequisite '%s' as VPATH '%s'.\n\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        (*d).name,
                                                        vname,
                                                    );
                                                    fflush(stdout);
                                                }
                                                let fresh6 = pat;
                                                pat = pat.offset(1 as ::core::ffi::c_int as isize);
                                                (*fresh6).name = (*d).name;
                                            } else {
                                                if intermed_ok != 0 {
                                                    if 0x8 as ::core::ffi::c_int & db_level != 0 {
                                                        print_spaces(depth);
                                                        printf(
                                                            if (*d).is_explicit()
                                                                as ::core::ffi::c_int
                                                                != 0
                                                                || !df.is_null()
                                                                    && (*df).is_explicit()
                                                                        as ::core::ffi::c_int
                                                                        != 0
                                                            {
                                                                b"Looking for a rule with explicit file '%s'.\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char
                                                            } else {
                                                                b"Looking for a rule with intermediate file '%s'.\n\0"
                                                                    as *const u8 as *const ::core::ffi::c_char
                                                            },
                                                            (*d).name,
                                                        );
                                                        fflush(stdout);
                                                    }
                                                    if int_file.is_null() {
                                                        alloca_allocations.push(
                                                            ::std::vec::from_elem(
                                                                0,
                                                                ::core::mem::size_of::<file>()
                                                                    as usize,
                                                            ),
                                                        );
                                                        int_file = alloca_allocations
                                                            .last_mut()
                                                            .unwrap()
                                                            .as_mut_ptr()
                                                            as *mut file;
                                                    }
                                                    memset(
                                                        int_file as *mut ::core::ffi::c_void,
                                                        0,
                                                        ::core::mem::size_of::<file>() as size_t,
                                                    );
                                                    (*int_file).name = (*d).name;
                                                    if pattern_search(
                                                        int_file,
                                                        0,
                                                        depth,
                                                        recursions
                                                            .wrapping_add(1),
                                                        allow_compat_rules,
                                                    ) != 0
                                                    {
                                                        (*pat).pattern = (*int_file).name;
                                                        (*int_file).name = (*d).name;
                                                        (*pat).file = int_file;
                                                        int_file = ::core::ptr::null_mut::<file>();
                                                        let fresh7 = pat;
                                                        pat = pat.offset(1 as ::core::ffi::c_int as isize);
                                                        (*fresh7).name = (*d).name;
                                                        current_block_294 = 3620302738604709257;
                                                    } else {
                                                        if !(*int_file).variables.is_null() {
                                                            free_variable_set(
                                                                (*int_file).variables,
                                                            );
                                                        }
                                                        if !(*int_file).pat_variables.is_null() {
                                                            free_variable_set(
                                                                (*int_file).pat_variables,
                                                            );
                                                        }
                                                        if df.is_null() {
                                                            file_impossible((*d).name);
                                                        }
                                                        current_block_294 = 8298116646536739282;
                                                    }
                                                } else {
                                                    current_block_294 = 8298116646536739282;
                                                }
                                                match current_block_294 {
                                                    3620302738604709257 => {}
                                                    _ => {
                                                        if intermed_ok != 0 {
                                                            if 0x8 as ::core::ffi::c_int & db_level
                                                                != 0
                                                            {
                                                                print_spaces(depth);
                                                                printf(
                                                                    b"Rejecting rule '%s' due to impossible prerequisite '%s'.\n\0"
                                                                        as *const u8 as *const ::core::ffi::c_char,
                                                                    get_rule_defn(rule),
                                                                    (*d).name,
                                                                );
                                                                fflush(stdout);
                                                            }
                                                        } else if 0x8 as ::core::ffi::c_int
                                                            & db_level
                                                            != 0
                                                        {
                                                            print_spaces(depth);
                                                            printf(
                                                                b"Not found '%s'.\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                (*d).name,
                                                            );
                                                            fflush(stdout);
                                                        }
                                                        failed = 1;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        d = (*d).next;
                                    }
                                }
                                free_dep_chain(dl);
                                if failed != 0 {
                                    break;
                                }
                            }
                            (*rule).in_use = 0;
                            if failed == 0 {
                                break;
                            }
                        }
                    }
                }
                ri = ri.wrapping_add(1);
            }
            if ri < nrules {
                break;
            }
            rule = ::core::ptr::null_mut::<rule>();
            intermed_ok += 1;
        }
        if !rule.is_null() {
            foundrule = ri;
            if recursions > 0 {
                (*file).name = *(*rule)
                    .targets
                    .offset((*tryrules.offset(foundrule as isize)).matches as isize);
            }
            loop {
                let fresh8 = pat;
                pat = pat.offset(-(1 as ::core::ffi::c_int) as isize);
                if !(fresh8 > deplist) {
                    break;
                }
                let mut dep_0: *mut dep = ::core::ptr::null_mut::<dep>();
                let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
                if !(*pat).file.is_null() {
                    let mut imf: *mut file = (*pat).file;
                    let mut f: *mut file = lookup_file((*imf).name);
                    if f.is_null() {
                        f = enter_file((*imf).name);
                    }
                    (*f).deps = (*imf).deps;
                    (*f).cmds = (*imf).cmds;
                    (*f).stem = (*imf).stem;
                    merge_variable_set_lists(&raw mut (*f).variables, (*imf).variables);
                    (*f).pat_variables = (*imf).pat_variables;
                    (*f).set_pat_searched((*imf).pat_searched() as ::core::ffi::c_uint);
                    (*f).also_make = (*imf).also_make;
                    (*f).set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*f).set_is_explicit(
                        (*f).is_explicit()
                            | ((*imf).is_explicit() as ::core::ffi::c_int != 0
                                || (*pat).is_explicit() as ::core::ffi::c_int != 0)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint,
                    );
                    (*f).set_notintermediate(
                        (*f).notintermediate()
                            | ((*imf).notintermediate() as ::core::ffi::c_int != 0
                                || no_intermediates != 0)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint,
                    );
                    (*f).set_intermediate(
                        (*f).intermediate()
                            | ((*f).is_explicit() == 0 && (*f).notintermediate() == 0)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint,
                    );
                    (*f).set_tried_implicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    imf = lookup_file((*pat).pattern);
                    if !imf.is_null() && (*imf).precious() as ::core::ffi::c_int != 0 {
                        (*f).set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    dep_0 = (*f).deps;
                    while !dep_0.is_null() {
                        (*dep_0).file = enter_file((*dep_0).name);
                        (*dep_0).name = ::core::ptr::null::<::core::ffi::c_char>();
                        (*(*dep_0).file).set_tried_implicit(
                            (*(*dep_0).file).tried_implicit()
                                | (*dep_0).changed() as ::core::ffi::c_int as ::core::ffi::c_uint,
                        );
                        dep_0 = (*dep_0).next;
                    }
                }
                dep_0 = alloc_dep();
                (*dep_0).set_ignore_mtime((*pat).ignore_mtime() as ::core::ffi::c_uint);
                (*dep_0).set_is_explicit((*pat).is_explicit() as ::core::ffi::c_uint);
                (*dep_0).set_ignore_automatic_vars(
                    (*pat).ignore_automatic_vars() as ::core::ffi::c_uint
                );
                (*dep_0).set_wait_here((*pat).wait_here() as ::core::ffi::c_uint);
                s = strcache_add((*pat).name);
                if recursions != 0 {
                    (*dep_0).name = s;
                } else {
                    (*dep_0).file = lookup_file(s);
                    if (*dep_0).file.is_null() {
                        (*dep_0).file = enter_file(s);
                    }
                }
                if (*pat).file.is_null()
                    && (*(*tryrules.offset(foundrule as isize)).rule).terminal as ::core::ffi::c_int
                        != 0
                {
                    if (*dep_0).file.is_null() {
                        (*dep_0).set_changed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    } else {
                        (*(*dep_0).file)
                            .set_tried_implicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                }
                (*dep_0).next = (*file).deps;
                (*file).deps = dep_0;
                (*file).set_was_shuffled(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            if (*file).was_shuffled() == 0 {
                shuffle_deps_recursive((*file).deps);
            }
            if (*tryrules.offset(foundrule as isize)).checked_lastslash == 0 {
                (*file).stem = strcache_add_len(stem, stemlen);
                fullstemlen = stemlen;
            } else {
                fullstemlen = pathlen.wrapping_add(stemlen);
                memcpy(
                    &raw mut stem_str as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    filename as *const ::core::ffi::c_void,
                    pathlen as size_t,
                );
                memcpy(
                    (&raw mut stem_str as *mut ::core::ffi::c_char).offset(pathlen as isize)
                        as *mut ::core::ffi::c_void,
                    stem as *const ::core::ffi::c_void,
                    stemlen as size_t,
                );
                stem_str[fullstemlen as usize] = 0;
                (*file).stem = strcache_add(&raw mut stem_str as *mut ::core::ffi::c_char);
            }
            (*file).cmds = (*rule).cmds;
            (*file).set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            let mut f_0: *mut file = lookup_file(
                *(*rule)
                    .targets
                    .offset((*tryrules.offset(foundrule as isize)).matches as isize),
            );
            if !f_0.is_null() {
                if (*f_0).precious() != 0 {
                    (*file).set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                if (*f_0).notintermediate() as ::core::ffi::c_int != 0 || no_intermediates != 0 {
                    (*file).set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            if (*rule).num as ::core::ffi::c_int > 1 {
                ri = 0;
                while ri < (*rule).num as ::core::ffi::c_uint {
                    if ri != (*tryrules.offset(foundrule as isize)).matches {
                        alloca_allocations.push(::std::vec::from_elem(
                            0,
                            (*(*rule).lens.offset(ri as isize) as size_t)
                                .wrapping_add(fullstemlen)
                                .wrapping_add(1) as usize,
                        ));
                        let mut nm: *mut ::core::ffi::c_char =
                            alloca_allocations.last_mut().unwrap().as_mut_ptr()
                                as *mut ::core::ffi::c_char;
                        let mut p_1: *mut ::core::ffi::c_char = nm;
                        let mut f_1: *mut file = ::core::ptr::null_mut::<file>();
                        let mut new: *mut dep = alloc_dep();
                        p_1 = mempcpy(
                            p_1 as *mut ::core::ffi::c_void,
                            *(*rule).targets.offset(ri as isize) as *const ::core::ffi::c_void,
                            ((*(*rule).suffixes.offset(ri as isize))
                                .offset_from(*(*rule).targets.offset(ri as isize))
                                as ::core::ffi::c_long
                                - 1) as size_t,
                        ) as *mut ::core::ffi::c_char;
                        p_1 = mempcpy(
                            p_1 as *mut ::core::ffi::c_void,
                            (*file).stem as *const ::core::ffi::c_void,
                            fullstemlen as size_t,
                        ) as *mut ::core::ffi::c_char;
                        memcpy(
                            p_1 as *mut ::core::ffi::c_void,
                            *(*rule).suffixes.offset(ri as isize) as *const ::core::ffi::c_void,
                            (*(*rule).lens.offset(ri as isize) as ::core::ffi::c_long
                                - (*(*rule).suffixes.offset(ri as isize))
                                    .offset_from(*(*rule).targets.offset(ri as isize))
                                    as ::core::ffi::c_long
                                + 1) as size_t,
                        );
                        (*new).name = strcache_add(nm);
                        (*new).file = enter_file((*new).name);
                        (*new).next = (*file).also_make;
                        f_1 = lookup_file(*(*rule).targets.offset(ri as isize));
                        if !f_1.is_null() {
                            if (*f_1).precious() != 0 {
                                (*(*new).file)
                                    .set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            }
                            if (*f_1).notintermediate() as ::core::ffi::c_int != 0
                                || no_intermediates != 0
                            {
                                (*(*new).file).set_notintermediate(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                            }
                        }
                        (*(*new).file)
                            .set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*file).also_make = new;
                    }
                    ri = ri.wrapping_add(1);
                }
            }
        }
    }
    free(tryrules as *mut ::core::ffi::c_void);
    free(deplist as *mut ::core::ffi::c_void);
    depth = depth.wrapping_sub(1);
    if !rule.is_null() {
        if 0x8 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Found implicit rule '%s' for '%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                get_rule_defn(rule),
                filename,
            );
            fflush(stdout);
        }
        return 1;
    }
    if found_compat_rule != 0 {
        if 0x8 as ::core::ffi::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                b"Searching for a compatibility rule for '%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                filename,
            );
            fflush(stdout);
        }
        '_c2rust_label_4: {
            if allow_compat_rules == 0 {
            } else {
                __assert_fail(
                    b"allow_compat_rules == 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/implicit.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1134 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        return pattern_search(file, archive, depth, recursions, 1);
    }
    if 0x8 as ::core::ffi::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            b"No implicit rule found for '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        fflush(stdout);
    }
    0
}
