//! Data base of default implicit rules and default variables.
//!
//! Port of `default.c`. The tables hand out `*const c_char` pointers because
//! the consumers (`install_pattern_rule`, `define_variable_in_set`, ...) are
//! still C-shaped APIs shared across modules.

use ::core::{
    ffi::{c_char, CStr},
    ptr::null,
};

use crate::{
    dep::DepNode,
    ffi_types::size_t,
    file::{enter_file, enter_prereqs},
    floc::Floc,
    read::{parse_file_seq, MAP_NUL, PARSEFS_NONE},
    recipe::Recipe,
    rule::install_pattern_rule,
    variable::{define_variable_in_set, o_default, undefine_variable_in_set},
};

const RECIPEPREFIX_DEFAULT: c_char = b'\t' as c_char;

/// The default `.SUFFIXES` list, in the order in which the corresponding
/// suffix rules are tried.
///
/// A plain `const`, not a `static mut`: `parse_file_seq` parses (and may
/// de-escape) its input in place, so [`install_builtin_suffixes`] takes a
/// fresh stack copy on each call instead of sharing one mutable buffer across
/// calls/sessions.
const DEFAULT_SUFFIXES: [u8; 147] =
    *b".out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S .mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo .w .ch .web .sh .elc .el\0";

/// Default non-terminal pattern rules: (target, deps, recipe).
const DEFAULT_PATTERN_RULES: &[(&CStr, &CStr, &CStr)] = &[
    (c"(%)", c"%", c"$(AR) $(ARFLAGS) $@ $<"),
    (c"%.out", c"%", c"@rm -f $@ \n cp $< $@"),
    // Syntax is "ctangle foo.w foo.ch foo.c".
    (c"%.c", c"%.w %.ch", c"$(CTANGLE) $^ $@"),
    // Syntax is "cweave foo.w foo.ch foo.tex".
    (c"%.tex", c"%.w %.ch", c"$(CWEAVE) $^ $@"),
];

/// Default terminal pattern rules (RCS and SCCS checkouts).
const DEFAULT_TERMINAL_RULES: &[(&CStr, &CStr, &CStr)] = &[
    // RCS.
    (c"%", c"%,v", c"$(CHECKOUT,v)"),
    (c"%", c"RCS/%,v", c"$(CHECKOUT,v)"),
    (c"%", c"RCS/%", c"$(CHECKOUT,v)"),
    // SCCS.
    (c"%", c"s.%", c"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<"),
    (
        c"%",
        c"SCCS/s.%",
        c"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<",
    ),
];

/// Default old-style suffix rules: (suffix target, recipe).
const DEFAULT_SUFFIX_RULES: &[(&CStr, &CStr)] = &[
    (c".o", c"$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".s", c"$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".S", c"$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".c", c"$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".cc", c"$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".C", c"$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".cpp", c"$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".f", c"$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".m", c"$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".p", c"$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".F", c"$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".r", c"$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (c".mod", c"$(COMPILE.mod) -o $@ -e $@ $^"),
    (c".def.sym", c"$(COMPILE.def) -o $@ $<"),
    (c".sh", c"cat $< >$@ \n chmod a+x $@"),
    (c".s.o", c"$(COMPILE.s) -o $@ $<"),
    (c".S.o", c"$(COMPILE.S) -o $@ $<"),
    (c".c.o", c"$(COMPILE.c) $(OUTPUT_OPTION) $<"),
    (c".cc.o", c"$(COMPILE.cc) $(OUTPUT_OPTION) $<"),
    (c".C.o", c"$(COMPILE.C) $(OUTPUT_OPTION) $<"),
    (c".cpp.o", c"$(COMPILE.cpp) $(OUTPUT_OPTION) $<"),
    (c".f.o", c"$(COMPILE.f) $(OUTPUT_OPTION) $<"),
    (c".m.o", c"$(COMPILE.m) $(OUTPUT_OPTION) $<"),
    (c".p.o", c"$(COMPILE.p) $(OUTPUT_OPTION) $<"),
    (c".F.o", c"$(COMPILE.F) $(OUTPUT_OPTION) $<"),
    (c".r.o", c"$(COMPILE.r) $(OUTPUT_OPTION) $<"),
    (c".mod.o", c"$(COMPILE.mod) -o $@ $<"),
    (c".c.ln", c"$(LINT.c) -C$* $<"),
    (
        c".y.ln",
        c"$(YACC.y) $< \n $(LINT.c) -C$* y.tab.c \n $(RM) y.tab.c",
    ),
    (
        c".l.ln",
        c"@$(RM) $*.c\n $(LEX.l) $< > $*.c\n$(LINT.c) -i $*.c -o $@\n $(RM) $*.c",
    ),
    (c".y.c", c"$(YACC.y) $< \n mv -f y.tab.c $@"),
    (c".l.c", c"@$(RM) $@ \n $(LEX.l) $< > $@"),
    (c".ym.m", c"$(YACC.m) $< \n mv -f y.tab.c $@"),
    (c".lm.m", c"@$(RM) $@ \n $(LEX.m) $< > $@"),
    (c".F.f", c"$(PREPROCESS.F) $(OUTPUT_OPTION) $<"),
    (c".r.f", c"$(PREPROCESS.r) $(OUTPUT_OPTION) $<"),
    // This might actually make lex.yy.c if there's no %R% directive in $*.l,
    // but in that case why were you trying to make $*.r anyway?
    (c".l.r", c"$(LEX.l) $< > $@ \n mv -f lex.yy.r $@"),
    (c".S.s", c"$(PREPROCESS.S) $< > $@"),
    (c".texinfo.info", c"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@"),
    (c".texi.info", c"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@"),
    (c".txinfo.info", c"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@"),
    (c".tex.dvi", c"$(TEX) $<"),
    (c".texinfo.dvi", c"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<"),
    (c".texi.dvi", c"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<"),
    (c".txinfo.dvi", c"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<"),
    (c".w.c", c"$(CTANGLE) $< - $@"),
    (c".web.p", c"$(TANGLE) $<"),
    (c".w.tex", c"$(CWEAVE) $< - $@"),
    (c".web.tex", c"$(WEAVE) $<"),
];

/// Default variables: (name, value). Defined as recursively-expanding with
/// `o_default` origin so makefiles and the environment can override them.
const DEFAULT_VARIABLES: &[(&CStr, &CStr)] = &[
    (c"AR", c"ar"),
    (c"ARFLAGS", c"-rv"),
    (c"AS", c"as"),
    (c"CC", c"cc"),
    (c"OBJC", c"cc"),
    (c"CXX", c"g++"),
    (
        c"CHECKOUT,v",
        c"+$(if $(wildcard $@),,$(CO) $(COFLAGS) $< $@)",
    ),
    (c"CO", c"co"),
    (c"COFLAGS", c""),
    (c"CPP", c"$(CC) -E"),
    (c"FC", c"f77"),
    // System V uses these, so explicit rules using them should work.
    // However, there is no way to make implicit rules use them and FC.
    (c"F77", c"$(FC)"),
    (c"F77FLAGS", c"$(FFLAGS)"),
    (c"GET", c"get"),
    (c"LD", c"ld"),
    (c"LEX", c"lex"),
    (c"LINT", c"lint"),
    (c"M2C", c"m2c"),
    (c"PC", c"pc"),
    (c"YACC", c"yacc"),
    (c"MAKEINFO", c"makeinfo"),
    (c"TEX", c"tex"),
    (c"TEXI2DVI", c"texi2dvi"),
    (c"WEAVE", c"weave"),
    (c"CWEAVE", c"cweave"),
    (c"TANGLE", c"tangle"),
    (c"CTANGLE", c"ctangle"),
    (c"RM", c"rm -f"),
    (c"LINK.o", c"$(CC) $(LDFLAGS) $(TARGET_ARCH)"),
    (
        c"COMPILE.c",
        c"$(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c",
    ),
    (
        c"LINK.c",
        c"$(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)",
    ),
    (
        c"COMPILE.m",
        c"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c",
    ),
    (
        c"LINK.m",
        c"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)",
    ),
    (
        c"COMPILE.cc",
        c"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c",
    ),
    (c"COMPILE.C", c"$(COMPILE.cc)"),
    (c"COMPILE.cpp", c"$(COMPILE.cc)"),
    (
        c"LINK.cc",
        c"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)",
    ),
    (c"LINK.C", c"$(LINK.cc)"),
    (c"LINK.cpp", c"$(LINK.cc)"),
    (c"YACC.y", c"$(YACC) $(YFLAGS)"),
    (c"LEX.l", c"$(LEX) $(LFLAGS) -t"),
    (c"YACC.m", c"$(YACC) $(YFLAGS)"),
    (c"LEX.m", c"$(LEX) $(LFLAGS) -t"),
    (c"COMPILE.f", c"$(FC) $(FFLAGS) $(TARGET_ARCH) -c"),
    (c"LINK.f", c"$(FC) $(FFLAGS) $(LDFLAGS) $(TARGET_ARCH)"),
    (
        c"COMPILE.F",
        c"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c",
    ),
    (
        c"LINK.F",
        c"$(FC) $(FFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)",
    ),
    (c"COMPILE.r", c"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -c"),
    (
        c"LINK.r",
        c"$(FC) $(FFLAGS) $(RFLAGS) $(LDFLAGS) $(TARGET_ARCH)",
    ),
    (
        c"COMPILE.def",
        c"$(M2C) $(M2FLAGS) $(DEFFLAGS) $(TARGET_ARCH)",
    ),
    (
        c"COMPILE.mod",
        c"$(M2C) $(M2FLAGS) $(MODFLAGS) $(TARGET_ARCH)",
    ),
    (
        c"COMPILE.p",
        c"$(PC) $(PFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c",
    ),
    (
        c"LINK.p",
        c"$(PC) $(PFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)",
    ),
    (c"LINK.s", c"$(CC) $(ASFLAGS) $(LDFLAGS) $(TARGET_MACH)"),
    (c"COMPILE.s", c"$(AS) $(ASFLAGS) $(TARGET_MACH)"),
    (
        c"LINK.S",
        c"$(CC) $(ASFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_MACH)",
    ),
    (
        c"COMPILE.S",
        c"$(CC) $(ASFLAGS) $(CPPFLAGS) $(TARGET_MACH) -c",
    ),
    (c"PREPROCESS.S", c"$(CPP) $(CPPFLAGS)"),
    (
        c"PREPROCESS.F",
        c"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -F",
    ),
    (
        c"PREPROCESS.r",
        c"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -F",
    ),
    (
        c"LINT.c",
        c"$(LINT) $(LINTFLAGS) $(CPPFLAGS) $(TARGET_ARCH)",
    ),
    (c"OUTPUT_OPTION", c"-o $@"),
    (c".LIBPATTERNS", c"lib%.so lib%.a"),
    (c"GNUMAKEFLAGS", c""),
];

/// Set up the `.SUFFIXES` special target and the `SUFFIXES` variable. With
/// `--no-builtin-rules` both are left empty.
///
/// # Safety
/// Must run single-threaded: it mutates the global file table, the global
/// variable set, and the `suffix_file` global.
pub unsafe fn set_default_suffixes(
    ctx: &crate::execctx::ExecContext,
    options: &crate::entry::Options,
) -> Result<(), crate::build_result::BuildError> {
    let suffix_file = enter_file(ctx, b".SUFFIXES");
    if let Some(node) = ctx.filenodes.get(suffix_file) {
        node.lock().expect("file node poisoned").builtin = true;
    }
    populate_suffixes(ctx, options, suffix_file)
}

/// Define the `SUFFIXES` variable for [`set_default_suffixes`]: empty under
/// `--no-builtin-rules`, else the built-in suffix list (which also enters and
/// marks each suffix file). Split out so the branch lives on its own.
///
/// # Safety
/// Must run single-threaded: mutates global file/variable state.
unsafe fn populate_suffixes(
    ctx: &crate::execctx::ExecContext,
    options: &crate::entry::Options,
    suffix_file: crate::file::FileId,
) -> Result<(), crate::build_result::BuildError> {
    if options.no_builtin_rules.get() {
        define_variable_in_set(
            ctx,
            c"SUFFIXES".as_ptr(),
            8,
            c"".as_ptr(),
            o_default,
            0,
            (*ctx.variable_globals.current_variable_set_list.get()).set,
            null::<Floc>(),
        )?;
    } else {
        install_builtin_suffixes(ctx, suffix_file)?;
    }
    Ok(())
}

/// Parse the built-in `.SUFFIXES` list, resolve+enter each prerequisite, mark
/// the entered files built-in, attach them as `.SUFFIXES`' deps, and define the
/// `SUFFIXES` variable. Split out of [`set_default_suffixes`] so its prereq walk
/// lives in its own function.
///
/// # Safety
/// Must run single-threaded: mutates global file/variable state.
unsafe fn install_builtin_suffixes(
    ctx: &crate::execctx::ExecContext,
    suffix_file: crate::file::FileId,
) -> Result<(), crate::build_result::BuildError> {
    let mut default_suffixes = DEFAULT_SUFFIXES;
    let mut p = default_suffixes.as_mut_ptr() as *mut c_char;
    let parsed = parse_file_seq(
        ctx,
        &mut p,
        MAP_NUL as size_t,
        MAP_NUL,
        null(),
        PARSEFS_NONE,
    )?;
    let deps: Vec<DepNode> = parsed
        .into_iter()
        .map(|pn| {
            let mut d = dep_with_name(pn.name);
            d.wait_here = pn.wait;
            d
        })
        .collect();
    let deps = enter_prereqs(ctx, deps, None);
    deps.iter()
        .filter_map(|d| d.file)
        .filter_map(|fid| ctx.filenodes.get(fid))
        .for_each(|fnode| fnode.lock().expect("file node poisoned").builtin = true);
    if let Some(node) = ctx.filenodes.get(suffix_file) {
        node.lock().expect("file node poisoned").deps = deps;
    }

    define_variable_in_set(
        ctx,
        c"SUFFIXES".as_ptr(),
        8,
        default_suffixes.as_ptr() as *const c_char,
        o_default,
        0,
        (*ctx.variable_globals.current_variable_set_list.get()).set,
        null::<Floc>(),
    )?;
    Ok(())
}

/// Build a fresh [`DepNode`] carrying just a name (no resolved file yet) — the
/// pointer-free analogue of allocating a `Dep` and setting its `name`.
fn dep_with_name(name: Vec<u8>) -> DepNode {
    DepNode {
        name: String::from_utf8_lossy(&name).into_owned(),
        file: None,
        shuf: None,
        stem: None,
        flags: crate::dep::DepFlags::empty(),
        changed: false,
        ignore_mtime: false,
        static_pattern: false,
        needs_second_expansion: false,
        ignore_automatic_vars: false,
        is_explicit: false,
        wait_here: false,
    }
}

/// Enter the default suffix rules into the file table as targets with
/// recipes, unless `--no-builtin-rules` was given.
///
/// # Safety
/// Must run single-threaded: it mutates the global file table.
pub unsafe fn install_default_suffix_rules(
    ctx: &crate::execctx::ExecContext,
    options: &crate::entry::Options,
) {
    if options.no_builtin_rules.get() {
        return;
    }
    DEFAULT_SUFFIX_RULES
        .iter()
        .for_each(|&(target, recipe)| install_one_suffix_rule(ctx, target, recipe));
}

/// Install one built-in suffix rule's recipe onto its target file, unless the
/// makefile already gave the target a recipe. Split out of
/// [`install_default_suffix_rules`] so the per-rule branching lives on its own.
///
/// # Safety
/// Must run single-threaded: mutates the global file store.
unsafe fn install_one_suffix_rule(
    ctx: &crate::execctx::ExecContext,
    target: &::core::ffi::CStr,
    recipe: &::core::ffi::CStr,
) {
    let f = enter_file(ctx, target.to_bytes());
    if let Some(node) = ctx.filenodes.get(f) {
        let mut guard = node.lock().expect("file node poisoned");
        // Don't clobber a recipe given in a makefile if there was one.
        if guard.recipe.is_none() {
            guard.recipe = Some(Recipe {
                defined_in: None,
                defined_lineno: 0,
                text: recipe.to_bytes().to_vec(),
                lines: Vec::new(),
                recipe_prefix: RECIPEPREFIX_DEFAULT as u8,
                any_recurse: false,
            });
            guard.builtin = true;
        }
    }
}

/// Install the default pattern rules, unless `--no-builtin-rules` was given.
///
/// # Safety
/// Must run single-threaded: it mutates the global pattern-rule lists.
pub unsafe fn install_default_implicit_rules(
    ctx: &crate::execctx::ExecContext,
    options: &crate::entry::Options,
) -> Result<(), crate::build_result::BuildError> {
    if options.no_builtin_rules.get() {
        return Ok(());
    }
    install_rule_table(ctx, DEFAULT_PATTERN_RULES, false)?;
    install_rule_table(ctx, DEFAULT_TERMINAL_RULES, true)
}

/// Install every `(target, dep, commands)` triple in `table` as a pattern
/// rule. Split out of [`install_default_implicit_rules`] so the two tables
/// share one walk.
fn install_rule_table(
    ctx: &crate::execctx::ExecContext,
    table: &[(&::core::ffi::CStr, &::core::ffi::CStr, &::core::ffi::CStr)],
    terminal: bool,
) -> Result<(), crate::build_result::BuildError> {
    for &(target, dep, commands) in table {
        install_pattern_rule(
            ctx,
            target.to_bytes(),
            dep.to_bytes(),
            commands.to_bytes(),
            terminal,
        )?;
    }
    Ok(())
}

/// Define the default variables, unless `--no-builtin-variables` was given.
///
/// # Safety
/// Must run single-threaded: it mutates the global variable set.
pub unsafe fn define_default_variables(
    ctx: &crate::execctx::ExecContext,
    options: &crate::entry::Options,
) -> Result<(), crate::build_result::BuildError> {
    if options.no_builtin_variables.get() {
        return Ok(());
    }
    // `try_for_each` rather than a `for` + `?`: the table is pure data and the
    // rejection is the whole result, so the iteration costs this frame no
    // decision point of its own.
    DEFAULT_VARIABLES.iter().try_for_each(|&(name, value)| {
        define_variable_in_set(
            ctx,
            name.as_ptr(),
            name.to_bytes().len() as size_t,
            value.as_ptr(),
            o_default,
            1,
            (*ctx.variable_globals.current_variable_set_list.get()).set,
            null::<Floc>(),
        )
        .map(|_| ())
    })
}

/// Undefine all the default variables (used by `-R`/`--no-builtin-variables`
/// after the environment has been processed).
///
/// # Safety
/// Must run single-threaded: it mutates the global variable set.
pub unsafe fn undefine_default_variables(
    ctx: &crate::execctx::ExecContext,
) -> Result<(), crate::build_result::BuildError> {
    // Same shape as `define_default_variables`: the table drives the whole
    // frame, so the rejection threads through without a branch here.
    DEFAULT_VARIABLES.iter().try_for_each(|&(name, _)| {
        undefine_variable_in_set(
            ctx,
            null(),
            name.as_ptr(),
            name.to_bytes().len() as size_t,
            o_default,
            ::core::ptr::null_mut(),
        )
    })
}
