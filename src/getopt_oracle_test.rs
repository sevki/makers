//! Test-only oracle: the original `getopt_long`-based `decode_switches`
//! implementation, preserved verbatim (per AGENTS.md rule 3) so the clap-based
//! replacement in `make_main` can be differentially fuzzed against it before
//! being trusted. Compiled only for `cargo test` — never part of the shipping
//! `make`/`libmake` binary, and the only place `optarg`/`optind`/`opterr`/
//! `getopt_long` are referenced anywhere in this crate.
#![cfg(test)]

use super::{
    error, expand_command_line_file, flag, handle_non_switch_argument, make_toui, opt_origin_cell,
    opt_set_flag, opt_set_str, option, strcache_add, string, strlist, CommandSwitch, FmtArg,
    Options, TEMP_STDIN_OPT, WARN_OPT,
};
use crate::execctx::ExecContext;
use crate::ffi_types::size_t;
use libc::{atof, strlen, EOF};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stringlist_oracle {
    pub list: *mut *const ::core::ffi::c_char,
    pub idx: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
}

extern "C" {
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: i32;
    static mut opterr: i32;
    fn getopt_long(
        argc: i32,
        argv: *const *mut ::core::ffi::c_char,
        shortopts: *const ::core::ffi::c_char,
        longopts: *const option,
        longind: *mut i32,
    ) -> i32;
}

const CHAR_MAX: i32 = super::CHAR_MAX;
const no_argument: i32 = super::no_argument;
const required_argument: i32 = super::required_argument;
const optional_argument: i32 = super::optional_argument;

/// The former `static mut getopt_shorts`/`long_options` pair, preserved
/// verbatim as the oracle's own copy.
unsafe fn build_getopt_tables_oracle(
    options: &Options,
) -> (Vec<::core::ffi::c_char>, Vec<option>) {
    let mut getopt_shorts: Vec<::core::ffi::c_char>;
    let mut long_options: Vec<option>;
    let empty_option = option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0,
        flag: ::core::ptr::null_mut::<i32>(),
        val: 0,
    };
    let switches = options.switches.borrow();
    let mut p: *mut ::core::ffi::c_char;
    let mut c: ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint;
    let switch_count = switches.iter().take_while(|cs| cs.c != 0).count();
    let long_opts_len = switch_count + super::LONG_OPTION_ALIASES.len() + 1;
    let short_opts_len = 1 + switch_count * 3 + 1;
    getopt_shorts = vec![0; short_opts_len];
    long_options = vec![empty_option; long_opts_len];
    p = getopt_shorts.as_mut_ptr();
    let fresh24 = p;
    p = p.offset(1_i32 as isize);
    *fresh24 = '-' as i32 as ::core::ffi::c_char;
    i = 0;
    while switches[i as usize].c != 0 {
        long_options[i as usize].name = (if switches[i as usize].long_name.is_null() {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            switches[i as usize].long_name
        }) as *mut ::core::ffi::c_char;
        long_options[i as usize].flag = ::core::ptr::null_mut::<i32>();
        long_options[i as usize].val = switches[i as usize].c;
        if switches[i as usize].c <= CHAR_MAX {
            let fresh25 = p;
            p = p.offset(1_i32 as isize);
            *fresh25 = switches[i as usize].c as ::core::ffi::c_char;
        }
        match switches[i as usize].type_0 as ::core::ffi::c_uint {
            0 | 1 | 7 => {
                long_options[i as usize].has_arg = no_argument;
            }
            2 | 3 | 4 | 5 | 6 => {
                if switches[i as usize].c <= CHAR_MAX {
                    let fresh26 = p;
                    p = p.offset(1_i32 as isize);
                    *fresh26 = ':' as i32 as ::core::ffi::c_char;
                }
                if !switches[i as usize].noarg_value.is_null() {
                    if switches[i as usize].c <= CHAR_MAX {
                        let fresh27 = p;
                        p = p.offset(1_i32 as isize);
                        *fresh27 = ':' as i32 as ::core::ffi::c_char;
                    }
                    long_options[i as usize].has_arg = optional_argument;
                } else {
                    long_options[i as usize].has_arg = required_argument;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    *p = 0;
    c = 0;
    while (c as usize) < super::LONG_OPTION_ALIASES.len() {
        let fresh28 = i;
        i = i.wrapping_add(1);
        long_options[fresh28 as usize] = super::LONG_OPTION_ALIASES[c as usize];
        c = c.wrapping_add(1);
    }
    long_options[i as usize].name = ::core::ptr::null::<::core::ffi::c_char>();
    (getopt_shorts, long_options)
}

/// The former `getopt_long`-based `decode_switches`, preserved verbatim as
/// the differential-test oracle for the clap-based replacement.
pub unsafe fn decode_switches_oracle(
    ctx: &ExecContext,
    options: &Options,
    argc: i32,
    argv: *mut *const ::core::ffi::c_char,
    origin: super::variable_origin,
) -> Result<(), crate::build_result::BuildError> {
    use super::o_command;
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut bad: i32 = 0;
    let mut cs: *mut CommandSwitch;
    let mut targets: stringlist_oracle = stringlist_oracle {
        list: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        idx: 0,
        max: 0,
    };
    let mut c: i32;
    let mut found_wait: ::core::ffi::c_uint = 0;
    let mut a: *mut *const ::core::ffi::c_char;
    targets.max = (argc + 1) as ::core::ffi::c_uint;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (targets.max as usize)
            .wrapping_mul(::core::mem::size_of::<*mut *const ::core::ffi::c_char>() as usize)
            as usize,
    ));
    targets.list =
        alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *const ::core::ffi::c_char;
    targets.idx = 0;
    let (mut getopt_shorts, mut long_options) = build_getopt_tables_oracle(options);
    opterr = (origin as ::core::ffi::c_uint == o_command as i32 as ::core::ffi::c_uint) as i32;
    optind = 0;
    while optind < argc {
        let mut coptarg: *const ::core::ffi::c_char;
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            getopt_shorts.as_mut_ptr(),
            long_options.as_mut_ptr(),
            ::core::ptr::null_mut::<i32>(),
        );
        coptarg = optarg;
        if c == EOF {
            break;
        }
        if c == '?' as i32 {
            bad = 1;
        } else if c == 1 {
            let fresh8 = targets.idx;
            targets.idx = targets.idx.wrapping_add(1);
            let fresh9 = &mut (*targets.list.offset(fresh8 as isize));
            *fresh9 = coptarg;
        } else {
            let mut switches = options.switches.borrow_mut();
            cs = switches.as_mut_ptr();
            while (*cs).c != 0 {
                if (*cs).c == c {
                    let cs_origin = opt_origin_cell(options, (*cs).c);
                    let doit: i32 = (origin as ::core::ffi::c_uint
                        == o_command as i32 as ::core::ffi::c_uint
                        || (*cs).env() as i32 != 0
                            && (cs_origin.is_none()
                                || origin as ::core::ffi::c_uint
                                    >= cs_origin.unwrap().get() as ::core::ffi::c_uint))
                        as i32;
                    if doit != 0 {
                        (*cs).set_specified(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    match (*cs).type_0 as ::core::ffi::c_uint {
                        7 => {}
                        0 | 1 => {
                            if doit != 0 {
                                let on = (*cs).type_0 as ::core::ffi::c_uint
                                    == flag as i32 as ::core::ffi::c_uint;
                                opt_set_flag(options, (*cs).c, on);
                                if let Some(oc) = cs_origin {
                                    oc.set(origin);
                                }
                            }
                        }
                        2 | 3 | 4 => {
                            if !(doit == 0) {
                                let arg_ok =
                                    if coptarg.is_null() {
                                        coptarg = (*cs).noarg_value as *const ::core::ffi::c_char;
                                        true
                                    } else if *coptarg as i32 == 0 {
                                        let mut opt: [::core::ffi::c_char; 2] =
                                            ::core::mem::transmute::<
                                                [u8; 2],
                                                [::core::ffi::c_char; 2],
                                            >(*b"c\0");
                                        let mut op: *const ::core::ffi::c_char =
                                            &raw mut opt as *mut ::core::ffi::c_char;
                                        if (*cs).c <= CHAR_MAX {
                                            opt[0_i32 as usize] = (*cs).c as ::core::ffi::c_char;
                                        } else {
                                            op = (*cs).long_name;
                                        }
                                        error(ctx,
                                        super::NILF,
                                        strlen(op) as size_t,
                                        b"the '%s%s' option requires a non-empty string argument\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        &[
                                            FmtArg::Str(if (*cs).c <= CHAR_MAX {
                                                b"-\0" as *const u8 as *const ::core::ffi::c_char
                                            } else {
                                                b"--\0" as *const u8 as *const ::core::ffi::c_char
                                            }),
                                            FmtArg::Str(op),
                                        ],
                                    );
                                        bad = 1;
                                        false
                                    } else {
                                        true
                                    };
                                if arg_ok {
                                    if (*cs).type_0 as ::core::ffi::c_uint
                                        == string as i32 as ::core::ffi::c_uint
                                    {
                                        let s = ::core::ffi::CStr::from_ptr(coptarg)
                                            .to_string_lossy()
                                            .into_owned();
                                        opt_set_str(options, (*cs).c, s);
                                        if let Some(oc) = cs_origin {
                                            oc.set(origin);
                                        }
                                    } else if (*cs).c == CHAR_MAX + 1 {
                                        let mut db_flags = options.db_flags.borrow_mut();
                                        let want = ::core::ffi::CStr::from_ptr(coptarg);
                                        let duplicate =
                                            db_flags.iter().any(|e| e.as_c_str() == want);
                                        if !duplicate {
                                            db_flags.push(want.to_owned());
                                            if let Some(oc) = cs_origin {
                                                oc.set(origin);
                                            }
                                        }
                                    } else {
                                        let mut list = match (*cs).c {
                                            c if c == 'C' as i32 => {
                                                options.directories.borrow_mut()
                                            }
                                            c if c == 'f' as i32 || c == TEMP_STDIN_OPT => {
                                                options.makefiles.borrow_mut()
                                            }
                                            c if c == 'I' as i32 => {
                                                options.include_dirs.borrow_mut()
                                            }
                                            c if c == 'o' as i32 => options.old_files.borrow_mut(),
                                            c if c == 'W' as i32 => options.new_files.borrow_mut(),
                                            c if c == 'E' as i32 => {
                                                options.eval_strings.borrow_mut()
                                            }
                                            c if c == WARN_OPT => options.warn_flags.borrow_mut(),
                                            _ => {
                                                unreachable!("non-list option in list arm")
                                            }
                                        };
                                        let duplicate =
                                            if (*cs).c != 'f' as i32 && (*cs).c != WARN_OPT {
                                                let want = ::core::ffi::CStr::from_ptr(coptarg);
                                                list.iter().any(|e| e.as_c_str() == want)
                                            } else {
                                                false
                                            };
                                        if !duplicate {
                                            let stored: ::std::ffi::CString = if (*cs).type_0
                                                as ::core::ffi::c_uint
                                                == strlist as i32 as ::core::ffi::c_uint
                                            {
                                                ::core::ffi::CStr::from_ptr(coptarg).to_owned()
                                            } else if (*cs).c == TEMP_STDIN_OPT {
                                                if options.stdin_offset.get() > 0 {
                                                    // The oracle keeps the
                                                    // pre-#537 diverging
                                                    // `fatal`; the production
                                                    // path now bridges through
                                                    // `fatal_err`/`exit_on_err`.
                                                    crate::output::fatal(ctx, super::NILF, 0, b"INTERNAL: multiple --temp-stdin options provided!\0"
                                                                    as *const u8 as *const ::core::ffi::c_char, &[]);
                                                }
                                                options.stdin_offset.set(list.len() as i32);
                                                let cached = strcache_add(ctx, coptarg);
                                                ctx.temp_stdin_name.0.set(cached);
                                                ::core::ffi::CStr::from_ptr(cached).to_owned()
                                            } else {
                                                ::core::ffi::CStr::from_ptr(
                                                    expand_command_line_file(ctx, coptarg)?,
                                                )
                                                .to_owned()
                                            };
                                            list.push(stored);
                                            if let Some(oc) = cs_origin {
                                                oc.set(origin);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        5 => {
                            if coptarg.is_null() && argc > optind {
                                let mut cp: *const ::core::ffi::c_char;
                                cp = *argv.offset(optind as isize);
                                while (*cp.offset(0_i32 as isize) as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                                    <= 9
                                {
                                    cp = cp.offset(1_i32 as isize);
                                }
                                if *cp.offset(0_i32 as isize) as i32 == 0 {
                                    let fresh18 = optind;
                                    optind += 1;
                                    coptarg = *argv.offset(fresh18 as isize);
                                }
                            }
                            if !(doit == 0) {
                                if !coptarg.is_null() {
                                    let i = make_toui(::core::ffi::CStr::from_ptr(coptarg))
                                        .unwrap_or(0);
                                    if i == 0 {
                                        error(ctx,
                                            super::NILF,
                                            0,
                                            b"the '-%c' option requires a positive integer argument\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                        &[FmtArg::Int(((*cs).c) as i64)],
                    );
                                        bad = 1;
                                    } else {
                                        options.arg_job_slots.set(Some(i));
                                        if let Some(oc) = cs_origin {
                                            oc.set(origin);
                                        }
                                    }
                                } else {
                                    let n = *((*cs).noarg_value as *const ::core::ffi::c_uint);
                                    options.arg_job_slots.set(Some(n));
                                    if let Some(oc) = cs_origin {
                                        oc.set(origin);
                                    }
                                }
                            }
                        }
                        6 => {
                            if coptarg.is_null()
                                && optind < argc
                                && ((*(*argv.offset(optind as isize)).offset(0_i32 as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_sub('0' as i32 as ::core::ffi::c_uint)
                                    <= 9
                                    || *(*argv.offset(optind as isize)).offset(0_i32 as isize)
                                        as i32
                                        == '.' as i32)
                            {
                                let fresh19 = optind;
                                optind += 1;
                                coptarg = *argv.offset(fresh19 as isize);
                            }
                            if doit != 0 {
                                let v = if !coptarg.is_null() {
                                    atof(coptarg)
                                } else {
                                    *((*cs).noarg_value as *const ::core::ffi::c_double)
                                };
                                options.max_load_average.set(v);
                                if let Some(oc) = cs_origin {
                                    oc.set(origin);
                                }
                            }
                        }
                        _ => {
                            libc::abort();
                        }
                    }
                    break;
                } else {
                    cs = cs.offset(1_i32 as isize);
                }
            }
        }
    }
    while optind < argc {
        let fresh20 = optind;
        optind += 1;
        let fresh21 = targets.idx;
        targets.idx = targets.idx.wrapping_add(1);
        let fresh22 = &mut (*targets.list.offset(fresh21 as isize));
        *fresh22 = *argv.offset(fresh20 as isize);
    }
    let fresh23 = &mut (*targets.list.offset(targets.idx as isize));
    *fresh23 = ::core::ptr::null::<::core::ffi::c_char>();
    a = targets.list;
    while !(*a).is_null() {
        let prior_found_wait: i32 = found_wait as i32;
        found_wait = handle_non_switch_argument(ctx, options, *a, origin)?;
        if prior_found_wait != 0 {
            if let Some(last) = options.goals.borrow_mut().last_mut() {
                last.dep.wait_here = true;
            }
        }
        a = a.offset(1_i32 as isize);
    }
    if bad != 0 && origin as ::core::ffi::c_uint == o_command as i32 as ::core::ffi::c_uint {
        // The oracle never needs to actually exit the test process; the
        // differential harness checks `bad`/`Options` state directly instead
        // of calling the diverging `print_usage(..) -> !`.
    }
    super::decode_debug_flags(ctx, options)?;
    super::decode_output_sync_flags(ctx, options)?;
    if options.warn_undefined_variables.get() {
        crate::warning::decode_actions(ctx, "undefined-var", None);
        options.warn_undefined_variables.set(false);
    }
    {
        let warn_flags = options.warn_flags.borrow();
        for wf in warn_flags.iter() {
            let arg = wf.to_str().unwrap_or("");
            crate::warning::decode_actions(ctx, arg, None);
        }
    }
    options.run_silent.set(options.silent.get());
    super::reset_env_override(ctx);
    Ok(())
}
