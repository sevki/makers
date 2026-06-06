use ::c2rust_bitfields;
pub use crate::ffi_types::{size_t, uintmax_t};
use crate::strcache::strcache_add;
use crate::misc::{xmalloc, xstrdup};
extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut no_builtin_rules_flag: ::core::ffi::c_int;
    static mut no_builtin_variables_flag: ::core::ffi::c_int;
    fn parse_file_seq(
        stringp: *mut *mut ::core::ffi::c_char,
        size: size_t,
        stopmap: ::core::ffi::c_int,
        prefix: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn enter_file(name: *const ::core::ffi::c_char) -> *mut file;
    fn enter_prereqs(prereqs: *mut dep, stem: *const ::core::ffi::c_char) -> *mut dep;
    static mut suffix_file: *mut file;
    fn install_pattern_rule(p: *const pspec, terminal: ::core::ffi::c_int);
    static mut current_variable_set_list: *mut variable_set_list;
    fn define_variable_in_set(
        name: *const ::core::ffi::c_char,
        length: size_t,
        value: *const ::core::ffi::c_char,
        origin: variable_origin,
        recursive: ::core::ffi::c_int,
        set: *mut variable_set,
        flocp: *const Floc,
    ) -> *mut variable;
    fn undefine_variable_in_set(
        flocp: *const Floc,
        name: *const ::core::ffi::c_char,
        length: size_t,
        origin: variable_origin,
        set: *mut variable_set,
    );
}
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
use crate::file::{Commands, Dep, File, VariableSet, VariableSetList};
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
pub struct pspec {
    pub target: *const ::core::ffi::c_char,
    pub dep: *const ::core::ffi::c_char,
    pub commands: *const ::core::ffi::c_char,
}
pub const MAKE_CXX: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"g++\0") };
pub const SCCS_GET: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"get\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAP_NUL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const NILF: *mut Floc = ::core::ptr::null_mut::<Floc>();
pub const GNUMAKEFLAGS_NAME: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"GNUMAKEFLAGS\0") };
pub const RECIPEPREFIX_DEFAULT: ::core::ffi::c_int = '\t' as i32;
pub const PARSEFS_NONE: ::core::ffi::c_int = 0;
static mut default_suffixes: [::core::ffi::c_char; 147] = unsafe {
    ::core::mem::transmute::<
        [u8; 147],
        [::core::ffi::c_char; 147],
    >(
        *b".out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S .mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo .w .ch .web .sh .elc .el\0",
    )
};
static mut default_pattern_rules: [pspec; 5] = [
    pspec {
        target: b"(%)\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"%\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(AR) $(ARFLAGS) $@ $<\0" as *const u8 as *const ::core::ffi::c_char,
    },
    pspec {
        target: b"%.out\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"%\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"@rm -f $@ \n cp $< $@\0" as *const u8 as *const ::core::ffi::c_char,
    },
    pspec {
        target: b"%.c\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"%.w %.ch\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(CTANGLE) $^ $@\0" as *const u8 as *const ::core::ffi::c_char,
    },
    pspec {
        target: b"%.tex\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"%.w %.ch\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(CWEAVE) $^ $@\0" as *const u8 as *const ::core::ffi::c_char,
    },
    pspec {
        target: ::core::ptr::null::<::core::ffi::c_char>(),
        dep: ::core::ptr::null::<::core::ffi::c_char>(),
        commands: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut default_terminal_rules: [pspec; 6] = [
    pspec {
        target: b"%\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"%,v\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(CHECKOUT,v)\0" as *const u8 as *const ::core::ffi::c_char,
    },
    pspec {
        target: b"%\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"RCS/%,v\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(CHECKOUT,v)\0" as *const u8 as *const ::core::ffi::c_char,
    },
    pspec {
        target: b"%\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"RCS/%\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(CHECKOUT,v)\0" as *const u8 as *const ::core::ffi::c_char,
    },
    pspec {
        target: b"%\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"s.%\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    pspec {
        target: b"%\0" as *const u8 as *const ::core::ffi::c_char,
        dep: b"SCCS/s.%\0" as *const u8 as *const ::core::ffi::c_char,
        commands: b"$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<\0" as *const u8
            as *const ::core::ffi::c_char,
    },
    pspec {
        target: ::core::ptr::null::<::core::ffi::c_char>(),
        dep: ::core::ptr::null::<::core::ffi::c_char>(),
        commands: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut default_suffix_rules: [*const ::core::ffi::c_char; 100] = [
    b".o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".s\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".S\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".c\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".cc\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".C\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".cpp\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".f\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".m\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".p\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".F\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".r\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".mod\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.mod) -o $@ -e $@ $^\0" as *const u8 as *const ::core::ffi::c_char,
    b".def.sym\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.def) -o $@ $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".sh\0" as *const u8 as *const ::core::ffi::c_char,
    b"cat $< >$@ \n chmod a+x $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".s.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.s) -o $@ $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".S.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.S) -o $@ $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".c.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.c) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".cc.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.cc) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".C.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.C) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".cpp.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.cpp) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".f.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.f) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".m.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.m) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".p.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.p) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".F.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.F) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".r.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.r) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".mod.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.mod) -o $@ $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".c.ln\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINT.c) -C$* $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".y.ln\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(YACC.y) $< \n $(LINT.c) -C$* y.tab.c \n $(RM) y.tab.c\0" as *const u8
        as *const ::core::ffi::c_char,
    b".l.ln\0" as *const u8 as *const ::core::ffi::c_char,
    b"@$(RM) $*.c\n $(LEX.l) $< > $*.c\n$(LINT.c) -i $*.c -o $@\n $(RM) $*.c\0" as *const u8
        as *const ::core::ffi::c_char,
    b".y.c\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(YACC.y) $< \n mv -f y.tab.c $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".l.c\0" as *const u8 as *const ::core::ffi::c_char,
    b"@$(RM) $@ \n $(LEX.l) $< > $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".ym.m\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(YACC.m) $< \n mv -f y.tab.c $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".lm.m\0" as *const u8 as *const ::core::ffi::c_char,
    b"@$(RM) $@ \n $(LEX.m) $< > $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".F.f\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(PREPROCESS.F) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".r.f\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(PREPROCESS.r) $(OUTPUT_OPTION) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".l.r\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LEX.l) $< > $@ \n mv -f lex.yy.r $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".S.s\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(PREPROCESS.S) $< > $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".texinfo.info\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".texi.info\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".txinfo.info\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(MAKEINFO) $(MAKEINFO_FLAGS) $< -o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".tex.dvi\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(TEX) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".texinfo.dvi\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".texi.dvi\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".txinfo.dvi\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(TEXI2DVI) $(TEXI2DVI_FLAGS) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".w.c\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CTANGLE) $< - $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".web.p\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(TANGLE) $<\0" as *const u8 as *const ::core::ffi::c_char,
    b".w.tex\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CWEAVE) $< - $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".web.tex\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(WEAVE) $<\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut default_variables: [*const ::core::ffi::c_char; 130] = [
    b"AR\0" as *const u8 as *const ::core::ffi::c_char,
    b"ar\0" as *const u8 as *const ::core::ffi::c_char,
    b"ARFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
    b"-rv\0" as *const u8 as *const ::core::ffi::c_char,
    b"AS\0" as *const u8 as *const ::core::ffi::c_char,
    b"as\0" as *const u8 as *const ::core::ffi::c_char,
    b"CC\0" as *const u8 as *const ::core::ffi::c_char,
    b"cc\0" as *const u8 as *const ::core::ffi::c_char,
    b"OBJC\0" as *const u8 as *const ::core::ffi::c_char,
    b"cc\0" as *const u8 as *const ::core::ffi::c_char,
    b"CXX\0" as *const u8 as *const ::core::ffi::c_char,
    MAKE_CXX.as_ptr(),
    b"CHECKOUT,v\0" as *const u8 as *const ::core::ffi::c_char,
    b"+$(if $(wildcard $@),,$(CO) $(COFLAGS) $< $@)\0" as *const u8 as *const ::core::ffi::c_char,
    b"CO\0" as *const u8 as *const ::core::ffi::c_char,
    b"co\0" as *const u8 as *const ::core::ffi::c_char,
    b"COFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"CPP\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CC) -E\0" as *const u8 as *const ::core::ffi::c_char,
    b"FC\0" as *const u8 as *const ::core::ffi::c_char,
    b"f77\0" as *const u8 as *const ::core::ffi::c_char,
    b"F77\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC)\0" as *const u8 as *const ::core::ffi::c_char,
    b"F77FLAGS\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FFLAGS)\0" as *const u8 as *const ::core::ffi::c_char,
    b"GET\0" as *const u8 as *const ::core::ffi::c_char,
    SCCS_GET.as_ptr(),
    b"LD\0" as *const u8 as *const ::core::ffi::c_char,
    b"ld\0" as *const u8 as *const ::core::ffi::c_char,
    b"LEX\0" as *const u8 as *const ::core::ffi::c_char,
    b"lex\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINT\0" as *const u8 as *const ::core::ffi::c_char,
    b"lint\0" as *const u8 as *const ::core::ffi::c_char,
    b"M2C\0" as *const u8 as *const ::core::ffi::c_char,
    b"m2c\0" as *const u8 as *const ::core::ffi::c_char,
    b"PC\0" as *const u8 as *const ::core::ffi::c_char,
    b"pc\0" as *const u8 as *const ::core::ffi::c_char,
    b"YACC\0" as *const u8 as *const ::core::ffi::c_char,
    b"yacc\0" as *const u8 as *const ::core::ffi::c_char,
    b"MAKEINFO\0" as *const u8 as *const ::core::ffi::c_char,
    b"makeinfo\0" as *const u8 as *const ::core::ffi::c_char,
    b"TEX\0" as *const u8 as *const ::core::ffi::c_char,
    b"tex\0" as *const u8 as *const ::core::ffi::c_char,
    b"TEXI2DVI\0" as *const u8 as *const ::core::ffi::c_char,
    b"texi2dvi\0" as *const u8 as *const ::core::ffi::c_char,
    b"WEAVE\0" as *const u8 as *const ::core::ffi::c_char,
    b"weave\0" as *const u8 as *const ::core::ffi::c_char,
    b"CWEAVE\0" as *const u8 as *const ::core::ffi::c_char,
    b"cweave\0" as *const u8 as *const ::core::ffi::c_char,
    b"TANGLE\0" as *const u8 as *const ::core::ffi::c_char,
    b"tangle\0" as *const u8 as *const ::core::ffi::c_char,
    b"CTANGLE\0" as *const u8 as *const ::core::ffi::c_char,
    b"ctangle\0" as *const u8 as *const ::core::ffi::c_char,
    b"RM\0" as *const u8 as *const ::core::ffi::c_char,
    b"rm -f\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.o\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CC) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILE.c\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CC) $(CFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.c\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"COMPILE.m\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8
        as *const ::core::ffi::c_char,
    b"LINK.m\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(OBJC) $(OBJCFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"COMPILE.cc\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8
        as *const ::core::ffi::c_char,
    b"COMPILE.C\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.cc)\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILE.cpp\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(COMPILE.cc)\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.cc\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"LINK.C\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.cc)\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.cpp\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINK.cc)\0" as *const u8 as *const ::core::ffi::c_char,
    b"YACC.y\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(YACC) $(YFLAGS)\0" as *const u8 as *const ::core::ffi::c_char,
    b"LEX.l\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LEX) $(LFLAGS) -t\0" as *const u8 as *const ::core::ffi::c_char,
    b"YACC.m\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(YACC) $(YFLAGS)\0" as *const u8 as *const ::core::ffi::c_char,
    b"LEX.m\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LEX) $(LFLAGS) -t\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILE.f\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.f\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILE.F\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.F\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"COMPILE.r\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.r\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"COMPILE.def\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(M2C) $(M2FLAGS) $(DEFFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILE.mod\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(M2C) $(M2FLAGS) $(MODFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILE.p\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(PC) $(PFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -c\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.p\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(PC) $(PFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_ARCH)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"LINK.s\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CC) $(ASFLAGS) $(LDFLAGS) $(TARGET_MACH)\0" as *const u8 as *const ::core::ffi::c_char,
    b"COMPILE.s\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(AS) $(ASFLAGS) $(TARGET_MACH)\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINK.S\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CC) $(ASFLAGS) $(CPPFLAGS) $(LDFLAGS) $(TARGET_MACH)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"COMPILE.S\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CC) $(ASFLAGS) $(CPPFLAGS) $(TARGET_MACH) -c\0" as *const u8 as *const ::core::ffi::c_char,
    b"PREPROCESS.S\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(CPP) $(CPPFLAGS)\0" as *const u8 as *const ::core::ffi::c_char,
    b"PREPROCESS.F\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(CPPFLAGS) $(TARGET_ARCH) -F\0" as *const u8 as *const ::core::ffi::c_char,
    b"PREPROCESS.r\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(FC) $(FFLAGS) $(RFLAGS) $(TARGET_ARCH) -F\0" as *const u8 as *const ::core::ffi::c_char,
    b"LINT.c\0" as *const u8 as *const ::core::ffi::c_char,
    b"$(LINT) $(LINTFLAGS) $(CPPFLAGS) $(TARGET_ARCH)\0" as *const u8 as *const ::core::ffi::c_char,
    b"OUTPUT_OPTION\0" as *const u8 as *const ::core::ffi::c_char,
    b"-o $@\0" as *const u8 as *const ::core::ffi::c_char,
    b".LIBPATTERNS\0" as *const u8 as *const ::core::ffi::c_char,
    b"lib%.so lib%.a\0" as *const u8 as *const ::core::ffi::c_char,
    GNUMAKEFLAGS_NAME.as_ptr(),
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe fn set_default_suffixes() {
    suffix_file = enter_file(strcache_add(
        b".SUFFIXES\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    (*suffix_file).set_builtin(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if no_builtin_rules_flag != 0 {
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                .wrapping_sub(1),
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            o_default,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
    } else {
        let mut d: *mut dep;
        let mut p: *const ::core::ffi::c_char =
            &raw const default_suffixes as *const ::core::ffi::c_char;
        (*suffix_file).deps = enter_prereqs(
            parse_file_seq(
                &raw mut p as *mut *mut ::core::ffi::c_char,
                ::core::mem::size_of::<dep>() as size_t,
                MAP_NUL,
                ::core::ptr::null::<::core::ffi::c_char>(),
                PARSEFS_NONE,
            ) as *mut dep,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        d = (*suffix_file).deps;
        while !d.is_null() {
            (*(*d).file).set_builtin(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            d = (*d).next;
        }
        define_variable_in_set(
            b"SUFFIXES\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 9]>() as size_t)
                .wrapping_sub(1),
            &raw const default_suffixes as *const ::core::ffi::c_char,
            o_default,
            0,
            (*current_variable_set_list).set,
            NILF,
        );
    };
}
#[no_mangle]
pub unsafe fn install_default_suffix_rules() {
    let mut s: *const *const ::core::ffi::c_char;
    if no_builtin_rules_flag != 0 {
        return;
    }
    s = &raw const default_suffix_rules as *const *const ::core::ffi::c_char;
    while !(*s).is_null() {
        let f: *mut file = enter_file(strcache_add(*s.offset(0 as ::core::ffi::c_int as isize)));
        if (*f).cmds.is_null() {
            (*f).cmds = xmalloc(::core::mem::size_of::<commands>() as size_t) as *mut commands;
            (*(*f).cmds).fileinfo.filenm = ::core::ptr::null::<::core::ffi::c_char>();
            (*(*f).cmds).commands = xstrdup(*s.offset(1 as ::core::ffi::c_int as isize));
            (*(*f).cmds).command_lines = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            (*(*f).cmds).recipe_prefix = RECIPEPREFIX_DEFAULT as ::core::ffi::c_char;
            (*f).set_builtin(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        s = s.offset(2 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe fn install_default_implicit_rules() {
    let mut p: *const pspec;
    if no_builtin_rules_flag != 0 {
        return;
    }
    p = &raw const default_pattern_rules as *const pspec;
    while !(*p).target.is_null() {
        install_pattern_rule(p, 0);
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
    p = &raw const default_terminal_rules as *const pspec;
    while !(*p).target.is_null() {
        install_pattern_rule(p, 1);
        p = p.offset(1 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe fn define_default_variables() {
    let mut s: *const *const ::core::ffi::c_char;
    if no_builtin_variables_flag != 0 {
        return;
    }
    s = &raw const default_variables as *const *const ::core::ffi::c_char;
    while !(*s).is_null() {
        define_variable_in_set(
            *s.offset(0 as ::core::ffi::c_int as isize),
            strlen(*s.offset(0 as ::core::ffi::c_int as isize)) as size_t,
            *s.offset(1 as ::core::ffi::c_int as isize),
            o_default,
            1,
            (*current_variable_set_list).set,
            NILF,
        );
        s = s.offset(2 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]
pub unsafe fn undefine_default_variables() {
    let mut s: *const *const ::core::ffi::c_char;
    s = &raw const default_variables as *const *const ::core::ffi::c_char;
    while !(*s).is_null() {
        undefine_variable_in_set(
            ::core::ptr::null_mut::<Floc>(),
            *s.offset(0 as ::core::ffi::c_int as isize),
            strlen(*s.offset(0 as ::core::ffi::c_int as isize)) as size_t,
            o_default,
            ::core::ptr::null_mut::<variable_set>(),
        );
        s = s.offset(2 as ::core::ffi::c_int as isize);
    }
}
