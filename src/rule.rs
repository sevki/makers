use libc::{abort, free, printf, putchar, puts, strchr, strcmp, strrchr};
use ::c2rust_bitfields;
use crate::stdio::{FILE};
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
extern "C" {
    static mut stdout: *mut FILE;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn error(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...);
    fn fatal(flocp: *const Floc, length: size_t, fmt: *const ::core::ffi::c_char, ...) -> !;
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xcalloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(_: *mut ::core::ffi::c_void, _: size_t) -> *mut ::core::ffi::c_void;
    fn xstrdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn find_percent_cached(_: *mut *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn dir_file_exists_p(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcache_add_len(str: *const ::core::ffi::c_char, len: size_t)
        -> *const ::core::ffi::c_char;
    static mut posix_pedantic: ::core::ffi::c_int;
    static mut second_expansion: ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn print_commands(cmds: *const commands);
    fn parse_file_seq(
        stringp: *mut *mut ::core::ffi::c_char,
        size: size_t,
        stopmap: ::core::ffi::c_int,
        prefix: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    fn lookup_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn expand_extra_prereqs(extra: *const variable) -> *mut dep;
    fn lookup_variable(name: *const ::core::ffi::c_char, length: size_t) -> *mut variable;
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspec {
    pub target: *const ::core::ffi::c_char,
    pub dep: *const ::core::ffi::c_char,
    pub commands: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_NUL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const INTSTR_LENGTH: usize = (53 as usize)
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22 as usize)
    .wrapping_add(3 as usize);
pub const RECIPEPREFIX_DEFAULT: ::core::ffi::c_int = '\t' as i32;
pub const PARSEFS_NONE: ::core::ffi::c_int = 0;
#[inline]

unsafe extern "C" fn alloc_dep() -> *mut dep {
    xcalloc(::core::mem::size_of::<dep>() as size_t) as *mut dep
}
#[inline]

unsafe extern "C" fn free_dep_chain(mut d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
#[no_mangle]
pub static mut pattern_rules: *mut rule = ::core::ptr::null::<rule>() as *mut rule;
#[no_mangle]
pub static mut last_pattern_rule: *mut rule = ::core::ptr::null::<rule>() as *mut rule;
#[no_mangle]
pub static mut num_pattern_rules: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut max_pattern_targets: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut max_pattern_deps: ::core::ffi::c_uint = 0;
#[no_mangle]
pub static mut max_pattern_dep_length: size_t = 0;
#[no_mangle]
pub static mut suffix_file: *mut file = ::core::ptr::null::<file>() as *mut file;
#[no_mangle]
pub unsafe extern "C" fn get_rule_defn(mut r: *mut rule) -> *const ::core::ffi::c_char {
    if (*r)._defn.is_null() {
        let mut len: size_t = 8;
        let mut k: ::core::ffi::c_uint = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut sep: *const ::core::ffi::c_char = b"\0" as *const u8 as *const ::core::ffi::c_char;
        let mut dep: *const dep = ::core::ptr::null::<dep>();
        let mut ood: *const dep = ::core::ptr::null::<dep>();
        k = 0;
        while k < (*r).num as ::core::ffi::c_uint {
            len = len.wrapping_add(
                (*(*r).lens.offset(k as isize)).wrapping_add(1) as size_t,
            );
            k = k.wrapping_add(1);
        }
        dep = (*r).deps;
        while !dep.is_null() {
            len = (len as ::core::ffi::c_ulong).wrapping_add(
                strlen(
                    if !(*dep).name.is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    },
                )
                .wrapping_add(
                    if (*dep).wait_here() as ::core::ffi::c_int != 0 {
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                            .wrapping_sub(1)
                    } else {
                        0
                    },
                )
                .wrapping_add(1) as ::core::ffi::c_ulong,
            ) as size_t as size_t;
            dep = (*dep).next;
        }
        (*r)._defn = xmalloc(len) as *mut ::core::ffi::c_char;
        p = (*r)._defn;
        k = 0;
        while k < (*r).num as ::core::ffi::c_uint {
            p = mempcpy(
                mempcpy(
                    p as *mut ::core::ffi::c_void,
                    sep as *const ::core::ffi::c_void,
                    strlen(sep),
                ),
                *(*r).targets.offset(k as isize) as *const ::core::ffi::c_void,
                *(*r).lens.offset(k as isize) as size_t,
            ) as *mut ::core::ffi::c_char;
            k = k.wrapping_add(1);
            sep = b" \0" as *const u8 as *const ::core::ffi::c_char;
        }
        let fresh4 = p;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        *fresh4 = ':' as i32 as ::core::ffi::c_char;
        if (*r).terminal != 0 {
            let fresh5 = p;
            p = p.offset(1 as ::core::ffi::c_int as isize);
            *fresh5 = ':' as i32 as ::core::ffi::c_char;
        }
        dep = (*r).deps;
        while !dep.is_null() {
            if (*dep).ignore_mtime() as ::core::ffi::c_int == 0 {
                if (*dep).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut ::core::ffi::c_void,
                        b" .WAIT\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                            .wrapping_sub(1),
                    ) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    mempcpy(
                        p as *mut ::core::ffi::c_void,
                        b" \0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        1,
                    ),
                    (if !(*dep).name.is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    }) as *const ::core::ffi::c_void,
                    strlen(if !(*dep).name.is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    }),
                ) as *mut ::core::ffi::c_char;
            } else if ood.is_null() {
                ood = dep;
            }
            dep = (*dep).next;
        }
        sep = b" | \0" as *const u8 as *const ::core::ffi::c_char;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    sep as *const ::core::ffi::c_void,
                    strlen(sep),
                ) as *mut ::core::ffi::c_char;
                if (*ood).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut ::core::ffi::c_void,
                        b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                            .wrapping_sub(1),
                    ) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    (if !(*ood).name.is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    }) as *const ::core::ffi::c_void,
                    strlen(if !(*ood).name.is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    }),
                ) as *mut ::core::ffi::c_char;
            }
            ood = (*ood).next;
            sep = b" \0" as *const u8 as *const ::core::ffi::c_char;
        }
        *p = 0;
    }
    (*r)._defn
}
#[no_mangle]
pub unsafe extern "C" fn snap_implicit_rules() {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut namelen: size_t = 0;
    let mut rule: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut dep: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut prereqs: *mut dep = expand_extra_prereqs(lookup_variable(
        b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
    ));
    let mut pre_deps: ::core::ffi::c_uint = 0;
    max_pattern_dep_length = 0;
    dep = prereqs;
    while !dep.is_null() {
        let mut d: *const ::core::ffi::c_char = if !(*dep).name.is_null() {
            (*dep).name
        } else {
            (*(*dep).file).name
        };
        let mut l: size_t = strlen(d) as size_t;
        if second_expansion != 0 {
            if (*dep).name.is_null() {
                (*dep).name = xstrdup((*(*dep).file).name);
            }
            (*dep).set_need_2nd_expansion(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (*dep).need_2nd_expansion() != 0 {
            loop {
                d = strchr(d, '%' as i32);
                if d.is_null() {
                    break;
                }
                l = l.wrapping_add(4);
                d = d.offset(1 as ::core::ffi::c_int as isize);
            }
        }
        if l > max_pattern_dep_length {
            max_pattern_dep_length = l;
        }
        pre_deps = pre_deps.wrapping_add(1);
        dep = (*dep).next;
    }
    max_pattern_deps = 0;
    max_pattern_targets = max_pattern_deps;
    num_pattern_rules = max_pattern_targets;
    rule = pattern_rules;
    while !rule.is_null() {
        let mut ndeps: ::core::ffi::c_uint = pre_deps;
        let mut lastdep: *mut dep = ::core::ptr::null_mut::<dep>();
        num_pattern_rules = num_pattern_rules.wrapping_add(1);
        if (*rule).num as ::core::ffi::c_uint > max_pattern_targets {
            max_pattern_targets = (*rule).num as ::core::ffi::c_uint;
        }
        dep = (*rule).deps as *mut dep;
        while !dep.is_null() {
            let mut dname: *const ::core::ffi::c_char = if !(*dep).name.is_null() {
                (*dep).name
            } else {
                (*(*dep).file).name
            };
            let mut len: size_t = strlen(dname) as size_t;
            let mut p: *const ::core::ffi::c_char = strrchr(dname, '/' as i32);
            let mut p2: *const ::core::ffi::c_char = if !p.is_null() {
                strchr(p, '%' as i32)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
            ndeps = ndeps.wrapping_add(1);
            if len > max_pattern_dep_length {
                max_pattern_dep_length = len;
            }
            if (*dep).next.is_null() {
                lastdep = dep;
            }
            if !p2.is_null() {
                if p == dname {
                    p = p.offset(1 as ::core::ffi::c_int as isize);
                }
                if p.offset_from(dname) as ::core::ffi::c_long as size_t > namelen {
                    namelen = p.offset_from(dname) as ::core::ffi::c_long as size_t;
                    name = xrealloc(
                        name as *mut ::core::ffi::c_void,
                        namelen.wrapping_add(1),
                    ) as *mut ::core::ffi::c_char;
                }
                memcpy(
                    name as *mut ::core::ffi::c_void,
                    dname as *const ::core::ffi::c_void,
                    p.offset_from(dname) as ::core::ffi::c_long as size_t,
                );
                *name.offset(p.offset_from(dname) as ::core::ffi::c_long as isize) =
                    0;
                (*dep).set_changed(
                    (dir_file_exists_p(name, b"\0" as *const u8 as *const ::core::ffi::c_char) == 0)
                        as ::core::ffi::c_int as ::core::ffi::c_uint
                        as ::core::ffi::c_uint,
                );
            } else {
                (*dep).set_changed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            dep = (*dep).next;
        }
        if !prereqs.is_null() {
            if !lastdep.is_null() {
                (*lastdep).next = copy_dep_chain(prereqs);
            } else {
                (*rule).deps = copy_dep_chain(prereqs) as *mut dep;
            }
        }
        if ndeps > max_pattern_deps {
            max_pattern_deps = ndeps;
        }
        rule = (*rule).next;
    }
    free(name as *mut ::core::ffi::c_void);
    free_dep_chain(prereqs);
}
unsafe extern "C" fn convert_suffix_rule(
    mut target: *const ::core::ffi::c_char,
    mut source: *const ::core::ffi::c_char,
    mut cmds: *mut commands,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut names: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut percents: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut deps: *mut dep = ::core::ptr::null_mut::<dep>();
    names = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    percents = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    if target.is_null() {
        *names = strcache_add_len(
            b"(%.o)\0" as *const u8 as *const ::core::ffi::c_char,
            5,
        );
        *percents = (*names).offset(1 as ::core::ffi::c_int as isize);
    } else {
        let mut len: size_t = strlen(target) as size_t;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (1 as size_t).wrapping_add(len).wrapping_add(1) as usize,
        ));
        let mut p: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        *p.offset(0 as ::core::ffi::c_int as isize) = '%' as i32 as ::core::ffi::c_char;
        memcpy(
            p.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            target as *const ::core::ffi::c_void,
            (len as size_t).wrapping_add(1),
        );
        *names = strcache_add_len(p, len.wrapping_add(1));
        *percents = *names;
    }
    if source.is_null() {
        deps = ::core::ptr::null_mut::<dep>();
    } else {
        let mut len_0: size_t = strlen(source) as size_t;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (1 as size_t).wrapping_add(len_0).wrapping_add(1) as usize,
        ));
        let mut p_0: *mut ::core::ffi::c_char =
            alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
        *p_0.offset(0 as ::core::ffi::c_int as isize) = '%' as i32 as ::core::ffi::c_char;
        memcpy(
            p_0.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            source as *const ::core::ffi::c_void,
            (len_0 as size_t).wrapping_add(1),
        );
        deps = alloc_dep();
        (*deps).name = strcache_add_len(p_0, len_0.wrapping_add(1));
    }
    create_pattern_rule(
        names,
        percents,
        1,
        0,
        deps,
        cmds,
        0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn convert_to_pattern() {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut d2: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut rulename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut maxsuffix: size_t = 0;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut l: size_t = strlen(if !(*d).name.is_null() {
            (*d).name
        } else {
            (*(*d).file).name
        }) as size_t;
        if l > maxsuffix {
            maxsuffix = l;
        }
        d = (*d).next;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        maxsuffix
            .wrapping_mul(2)
            .wrapping_add(1) as usize,
    ));
    rulename = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut f: *mut file = ::core::ptr::null_mut::<file>();
        let mut slen: size_t = 0;
        convert_suffix_rule(
            if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            },
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<commands>(),
        );
        if !(*(*d).file).cmds.is_null() {
            convert_suffix_rule(
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*(*d).file).name
                },
                (*(*d).file).cmds,
            );
        }
        slen = strlen(if !(*d).name.is_null() {
            (*d).name
        } else {
            (*(*d).file).name
        }) as size_t;
        memcpy(
            rulename as *mut ::core::ffi::c_void,
            (if !(*d).name.is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            }) as *const ::core::ffi::c_void,
            (slen as size_t).wrapping_add(1),
        );
        f = lookup_file(rulename);
        if !f.is_null() && !(*f).cmds.is_null() {
            if (*f).deps.is_null() {
                (*f).set_suffix(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else if posix_pedantic == 0 {
                error(
                    &raw mut (*(*f).cmds).fileinfo,
                    0,
                    b"warning: ignoring prerequisites on suffix rule definition\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*f).set_suffix(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        let mut current_block_29: u64;
        d2 = (*suffix_file).deps;
        while !d2.is_null() {
            let mut s2len: size_t = 0;
            s2len = strlen(if !(*d2).name.is_null() {
                (*d2).name
            } else {
                (*(*d2).file).name
            }) as size_t;
            if !(slen == s2len
                && (*(if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*(*d).file).name
                }) as ::core::ffi::c_int
                    == *(if !(*d2).name.is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    }) as ::core::ffi::c_int
                    && (*(if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as ::core::ffi::c_int
                        == 0
                        || strcmp(
                            (if !(*d).name.is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }) . offset ( 1 ) ,
                            (if !(*d2).name.is_null() {
                                (*d2).name
                            } else {
                                (*(*d2).file).name
                            }) . offset ( 1 ) ,
                        ) == 0)))
            {
                memcpy(
                    rulename.offset(slen as isize) as *mut ::core::ffi::c_void,
                    (if !(*d2).name.is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    }) as *const ::core::ffi::c_void,
                    (s2len as size_t).wrapping_add(1),
                );
                f = lookup_file(rulename);
                if !(f.is_null() || (*f).cmds.is_null()) {
                    if !(*f).deps.is_null() {
                        if posix_pedantic != 0 {
                            current_block_29 = 11584701595673473500;
                        } else {
                            error(
                                &raw mut (*(*f).cmds).fileinfo,
                                0,
                                b"warning: ignoring prerequisites on suffix rule definition\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block_29 = 14359455889292382949;
                        }
                    } else {
                        current_block_29 = 14359455889292382949;
                    }
                    match current_block_29 {
                        11584701595673473500 => {}
                        _ => {
                            (*f).set_suffix(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            if s2len == 2
                                && *rulename.offset(slen as isize) as ::core::ffi::c_int
                                    == '.' as i32
                                && *rulename.offset(slen.wrapping_add(1) as isize)
                                    as ::core::ffi::c_int
                                    == 'a' as i32
                            {
                                convert_suffix_rule(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    if !(*d).name.is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    },
                                    (*f).cmds,
                                );
                            }
                            convert_suffix_rule(
                                if !(*d2).name.is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                },
                                if !(*d).name.is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                },
                                (*f).cmds,
                            );
                        }
                    }
                }
            }
            d2 = (*d2).next;
        }
        d = (*d).next;
    }
}
unsafe extern "C" fn new_pattern_rule(
    mut rule: *mut rule,
    mut override_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut lastrule: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    (*rule).in_use = 0;
    (*rule).terminal = 0;
    (*rule).next = ::core::ptr::null_mut::<rule>();
    lastrule = ::core::ptr::null_mut::<rule>();
    r = pattern_rules;
    's_18: while !r.is_null() {
        i = 0;
        while i < (*rule).num as ::core::ffi::c_uint {
            j = 0;
            while j < (*r).num as ::core::ffi::c_uint {
                if !(**(*rule).targets.offset(i as isize) as ::core::ffi::c_int
                    == **(*r).targets.offset(j as isize) as ::core::ffi::c_int
                    && (**(*rule).targets.offset(i as isize) as ::core::ffi::c_int == 0
                        || strcmp(
                            (*(*rule).targets.offset(i as isize)).offset(1 as ::core::ffi::c_int as isize),
                            (*(*r).targets.offset(j as isize)).offset(1 as ::core::ffi::c_int as isize),
                        ) == 0))
                {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if j == (*r).num as ::core::ffi::c_uint {
                let mut d: *mut dep = ::core::ptr::null_mut::<dep>();
                let mut d2: *mut dep = ::core::ptr::null_mut::<dep>();
                d = (*rule).deps as *mut dep;
                d2 = (*r).deps as *mut dep;
                while !d.is_null() && !d2.is_null() {
                    if !(*(if !(*d).name.is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as ::core::ffi::c_int
                        == *(if !(*d2).name.is_null() {
                            (*d2).name
                        } else {
                            (*(*d2).file).name
                        }) as ::core::ffi::c_int
                        && (*(if !(*d).name.is_null() {
                            (*d).name
                        } else {
                            (*(*d).file).name
                        }) as ::core::ffi::c_int
                            == 0
                            || strcmp(
                                (if !(*d).name.is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                }) . offset ( 1 ) ,
                                (if !(*d2).name.is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                }) . offset ( 1 ) ,
                            ) == 0))
                    {
                        break;
                    }
                    d = (*d).next;
                    d2 = (*d2).next;
                }
                if d.is_null() && d2.is_null() {
                    if override_0 != 0 {
                        freerule(r, lastrule);
                        if pattern_rules.is_null() {
                            pattern_rules = rule;
                        } else {
                            (*last_pattern_rule).next = rule;
                        }
                        last_pattern_rule = rule;
                        break 's_18;
                    } else {
                        freerule(rule, ::core::ptr::null_mut::<rule>());
                        return 0;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        lastrule = r;
        r = (*r).next;
    }
    if r.is_null() {
        if pattern_rules.is_null() {
            pattern_rules = rule;
        } else {
            (*last_pattern_rule).next = rule;
        }
        last_pattern_rule = rule;
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn install_pattern_rule(
    mut p: *const pspec,
    mut terminal: ::core::ffi::c_int,
) {
    let mut r: *mut rule = ::core::ptr::null_mut::<rule>();
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    r = xmalloc(::core::mem::size_of::<rule>() as size_t) as *mut rule;
    (*r).num = 1;
    (*r).targets = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    (*r).suffixes = xmalloc(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t)
        as *mut *const ::core::ffi::c_char;
    (*r).lens = xmalloc(::core::mem::size_of::<::core::ffi::c_uint>() as size_t)
        as *mut ::core::ffi::c_uint;
    (*r)._defn = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *(*r).lens.offset(0 as ::core::ffi::c_int as isize) = strlen((*p).target) as ::core::ffi::c_uint;
    let ref mut fresh1 = *(*r).targets.offset(0 as ::core::ffi::c_int as isize);
    *fresh1 = (*p).target;
    let ref mut fresh2 = *(*r).suffixes.offset(0 as ::core::ffi::c_int as isize);
    *fresh2 =
        find_percent_cached((*r).targets.offset(0 as ::core::ffi::c_int as isize) as *mut *const ::core::ffi::c_char);
    '_c2rust_label: {
        if !(*(*r).suffixes.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        } else {
            __assert_fail(
                b"r->suffixes[0] != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/rule.c\0" as *const u8 as *const ::core::ffi::c_char,
                492,
                b"void install_pattern_rule(const struct pspec *, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let ref mut fresh3 = *(*r).suffixes.offset(0 as ::core::ffi::c_int as isize);
    *fresh3 = (*fresh3).offset(1 as ::core::ffi::c_int as isize);
    ptr = (*p).dep;
    (*r).deps = parse_file_seq(
        &raw mut ptr as *mut *mut ::core::ffi::c_char,
        ::core::mem::size_of::<dep>() as size_t,
        MAP_NUL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        PARSEFS_NONE,
    ) as *mut dep as *mut dep;
    if new_pattern_rule(r, 0) != 0 {
        (*r).terminal = (if terminal != 0 {
            1
        } else {
            0
        }) as ::core::ffi::c_char;
        (*r).cmds = xmalloc(::core::mem::size_of::<commands>() as size_t) as *mut commands;
        (*(*r).cmds).fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
        (*(*r).cmds).fileinfo.lineno = 0;
        (*(*r).cmds).fileinfo.offset = 0;
        (*(*r).cmds).commands = xstrdup((*p).commands);
        (*(*r).cmds).command_lines = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*(*r).cmds).recipe_prefix = RECIPEPREFIX_DEFAULT as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn freerule(mut rule: *mut rule, mut lastrule: *mut rule) {
    let mut next: *mut rule = (*rule).next;
    free_dep_chain((*rule).deps as *mut dep);
    free((*rule).targets as *mut ::core::ffi::c_void);
    free((*rule).suffixes as *mut ::core::ffi::c_void);
    free((*rule).lens as *mut ::core::ffi::c_void);
    free((*rule)._defn as *mut ::core::ffi::c_void);
    free(rule as *mut ::core::ffi::c_void);
    if pattern_rules == rule {
        if !lastrule.is_null() {
            abort();
        } else {
            pattern_rules = next;
        }
    } else if !lastrule.is_null() {
        (*lastrule).next = next;
    }
    if last_pattern_rule == rule {
        last_pattern_rule = lastrule;
    }
}
#[no_mangle]
pub unsafe extern "C" fn create_pattern_rule(
    mut targets: *mut *const ::core::ffi::c_char,
    mut target_percents: *mut *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_ushort,
    mut terminal: ::core::ffi::c_int,
    mut deps: *mut dep,
    mut commands: *mut commands,
    mut override_0: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut r: *mut rule = xmalloc(::core::mem::size_of::<rule>() as size_t) as *mut rule;
    (*r).num = n;
    (*r).cmds = commands as *mut commands;
    (*r).deps = deps as *mut dep;
    (*r).targets = targets;
    (*r).suffixes = target_percents;
    (*r).lens = xmalloc(
        (n as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
    ) as *mut ::core::ffi::c_uint;
    (*r)._defn = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 0;
    while i < n as ::core::ffi::c_uint {
        *(*r).lens.offset(i as isize) = strlen(*targets.offset(i as isize)) as ::core::ffi::c_uint;
        '_c2rust_label: {
            if !(*(*r).suffixes.offset(i as isize)).is_null() {
            } else {
                __assert_fail(
                    b"r->suffixes[i] != NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"src/rule.c\0" as *const u8 as *const ::core::ffi::c_char,
                    584,
                    b"void create_pattern_rule(const char **, const char **, unsigned short, int, struct dep *, struct commands *, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        let ref mut fresh0 = *(*r).suffixes.offset(i as isize);
        *fresh0 = (*fresh0).offset(1 as ::core::ffi::c_int as isize);
        i = i.wrapping_add(1);
    }
    if new_pattern_rule(r, override_0) != 0 {
        (*r).terminal = (if terminal != 0 {
            1
        } else {
            0
        }) as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_rule(mut r: *mut rule) {
    fputs(get_rule_defn(r), stdout);
    putchar('\n' as i32);
    if !(*r).cmds.is_null() {
        print_commands((*r).cmds);
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_rule_data_base() {
    let mut rules: ::core::ffi::c_uint = 0;
    let mut terminal: ::core::ffi::c_uint = 0;
    let mut r: *mut rule = ::core::ptr::null_mut::<rule>();
    puts(b"\n# Implicit Rules\0" as *const u8 as *const ::core::ffi::c_char);
    terminal = 0;
    rules = terminal;
    r = pattern_rules;
    while !r.is_null() {
        rules = rules.wrapping_add(1);
        putchar('\n' as i32);
        print_rule(r);
        if (*r).terminal != 0 {
            terminal = terminal.wrapping_add(1);
        }
        r = (*r).next;
    }
    if rules == 0 {
        puts(b"\n# No implicit rules.\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        printf(
            b"\n# %u implicit rules, %u (%.1f%%) terminal.\0" as *const u8
                as *const ::core::ffi::c_char,
            rules,
            terminal,
            terminal as ::core::ffi::c_double / rules as ::core::ffi::c_double * 100.0f64,
        );
    }
    if num_pattern_rules != rules {
        if num_pattern_rules != 0 {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                INTSTR_LENGTH.wrapping_mul(2),
                b"INTERNAL: num_pattern_rules is wrong!  %u != %u\0" as *const u8
                    as *const ::core::ffi::c_char,
                num_pattern_rules,
                rules,
            );
        }
    }
}
