//! Makefile syntax parsing, factored into a typed AST built through [`salsa`].
//!
//! GNU make has no syntax tree: its reader (`src/read.rs`) is a streaming,
//! pointer-walking state machine that mutates global state line by line. This
//! module introduces a small, idiomatic AST layer for the pieces of make syntax
//! that are *pure functions of the line text*, so the imperative reader can
//! consume typed nodes instead of re-deriving structure with raw-pointer scans.
//!
//! The first construct covered is **variable assignment** — the operator family
//! `=`, `:=`, `::=`, `:::=`, `?=`, `+=`, `!=` plus the `name`/`value` split that
//! make's `parse_variable_definition` computes. The parse itself is a safe,
//! slice-based reproduction of that state machine ([`parse_assignment`]); the
//! public entry point ([`assignment_ast`]) routes it through a salsa query so
//! identical lines are interned and parsed once, following the same database
//! pattern as [`crate::strcache`].
//!
//! Byte classification deliberately consults the process-global `stopchar_map`
//! (via [`crate::entry::stopchar_map`]) rather than re-deriving `isspace`,
//! so the AST agrees with the C reader byte-for-byte, locale and all.

use std::ops::Range;

use crate::{
    entry::{stopchar_map, MAP_BLANK, MAP_COMMENT, MAP_NEWLINE, MAP_NUL, MAP_VARSEP},
    variable::{f_append, f_expand, f_recursive, f_shell, f_simple, variable_flavor},
};

/// The assignment operator's flavor, mirroring make's `variable_flavor`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Flavor {
    /// `=` — recursively expanded.
    Recursive,
    /// `:=` / `::=` — simply (immediately) expanded.
    Simple,
    /// `:::=` — immediately expanded, value re-escaped.
    Expand,
    /// `+=` — append.
    Append,
    /// `!=` — shell assignment.
    Shell,
}

impl Flavor {
    /// The matching `variable_flavor` constant for write-back into a
    /// `struct variable`.
    pub fn to_variable_flavor(self) -> variable_flavor {
        match self {
            Flavor::Recursive => f_recursive,
            Flavor::Simple => f_simple,
            Flavor::Expand => f_expand,
            Flavor::Append => f_append,
            Flavor::Shell => f_shell,
        }
    }
}

/// A parsed variable assignment, expressed as byte offsets into the source line
/// so the caller can point make's `struct variable` straight at the original
/// buffer (matching `parse_variable_definition`, which never copies the name).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Assignment {
    /// Start offset of the variable name within the line.
    pub name_start: usize,
    /// Length of the variable name (it is not NUL-terminated in place).
    pub name_len: usize,
    /// Assignment flavor.
    pub flavor: Flavor,
    /// Whether the operator was conditional (`?=`).
    pub conditional: bool,
    /// Offset just past the operator — the value returned by
    /// `parse_variable_definition`.
    pub op_end: usize,
    /// Offset of the first value byte (the operator's trailing whitespace
    /// skipped, i.e. `next_token(op_end)`).
    pub value_start: usize,
}

impl Assignment {
    /// The variable name's byte range within the source line.
    pub fn name(&self) -> Range<usize> {
        self.name_start..self.name_start + self.name_len
    }
}

/// A conditional directive keyword (`ifdef`, `ifeq`, `else`, `endif`, …).
///
/// Classifying the keyword is a pure function of the line's first word, so it
/// belongs in the AST layer. Unlike [`Assignment`], a directive carries no
/// variable-length data and has only six possible values, so there is nothing
/// to dedup — it is classified directly by [`Directive::from_word`] rather than
/// interned through the salsa database (which would only add a lock to this hot
/// path).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Directive {
    /// `ifdef`
    Ifdef,
    /// `ifndef`
    Ifndef,
    /// `ifeq`
    Ifeq,
    /// `ifneq`
    Ifneq,
    /// `else`
    Else,
    /// `endif`
    Endif,
}

impl Directive {
    /// Classify `word` (a line's leading token) as a conditional directive, or
    /// `None` if it is not one. Matching is exact, mirroring make's
    /// `conditional_line`, which only accepts a word whose length equals the
    /// keyword's.
    pub fn from_word(word: &[u8]) -> Option<Directive> {
        Some(match word {
            b"ifdef" => Directive::Ifdef,
            b"ifndef" => Directive::Ifndef,
            b"ifeq" => Directive::Ifeq,
            b"ifneq" => Directive::Ifneq,
            b"else" => Directive::Else,
            b"endif" => Directive::Endif,
            _ => return None,
        })
    }

    /// The NUL-terminated keyword, matching the C `cmdname` used verbatim in
    /// make's diagnostics (`extraneous text after '%s' directive`, etc.).
    pub fn name(self) -> &'static core::ffi::CStr {
        match self {
            Directive::Ifdef => c"ifdef",
            Directive::Ifndef => c"ifndef",
            Directive::Ifeq => c"ifeq",
            Directive::Ifneq => c"ifneq",
            Directive::Else => c"else",
            Directive::Endif => c"endif",
        }
    }
}

/// A variable-definition modifier keyword that may prefix an assignment
/// (`export FOO = 1`, `override BAR := 2`, `define`, …).
///
/// Like [`Directive`], these are a small fixed set classified directly from the
/// line's leading word rather than interned through salsa.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VarModifier {
    /// `export`
    Export,
    /// `unexport`
    Unexport,
    /// `override`
    Override,
    /// `private`
    Private,
    /// `define`
    Define,
    /// `undefine`
    Undefine,
}

impl VarModifier {
    /// Classify `word` (a line's leading token) as a variable-definition
    /// modifier, or `None` if it is not one. Matching is exact, mirroring make's
    /// `eval`, which compares the whole first word against each keyword.
    pub fn from_word(word: &[u8]) -> Option<VarModifier> {
        Some(match word {
            b"export" => VarModifier::Export,
            b"unexport" => VarModifier::Unexport,
            b"override" => VarModifier::Override,
            b"private" => VarModifier::Private,
            b"define" => VarModifier::Define,
            b"undefine" => VarModifier::Undefine,
            _ => return None,
        })
    }
}

/// A file/path directive keyword recognized while parsing a target line
/// (`include`, `vpath`, `load`, …).
///
/// `-include` and `sinclude` are the error-tolerant form of `include`, and map
/// to the same [`FileDirective::IncludeOpt`]. Like the other keyword classifiers
/// this is a small fixed set, matched directly rather than interned.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FileDirective {
    /// `vpath`
    Vpath,
    /// `include` (missing file is an error)
    Include,
    /// `-include` / `sinclude` (missing file is silently ignored)
    IncludeOpt,
    /// `load` (missing object is an error)
    Load,
    /// `-load` (missing object is silently ignored)
    LoadOpt,
}

impl FileDirective {
    /// Classify `word` (a line's leading token) as a file/path directive, or
    /// `None` if it is not one. Matching is exact, mirroring make's `eval`.
    pub fn from_word(word: &[u8]) -> Option<FileDirective> {
        Some(match word {
            b"vpath" => FileDirective::Vpath,
            b"include" => FileDirective::Include,
            b"-include" | b"sinclude" => FileDirective::IncludeOpt,
            b"load" => FileDirective::Load,
            b"-load" => FileDirective::LoadOpt,
            _ => return None,
        })
    }
}

/// A `define`/`endef` block keyword.
///
/// `define` opens a multi-line variable definition and `endef` closes it; the
/// reader nests them while scanning a define body. Like the other keyword
/// classifiers this is a small fixed set, matched directly rather than interned.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DefineKeyword {
    /// `define` — opens a multi-line definition.
    Define,
    /// `endef` — closes a multi-line definition.
    Endef,
}

impl DefineKeyword {
    /// Classify `word` (a line's leading token) as a define-block keyword, or
    /// `None` if it is not one. Matching is exact, mirroring make's reader.
    pub fn from_word(word: &[u8]) -> Option<DefineKeyword> {
        Some(match word {
            b"define" => DefineKeyword::Define,
            b"endef" => DefineKeyword::Endef,
            _ => return None,
        })
    }
}

/// Classify the leading `define`/`endef` keyword of a line in a `define` body,
/// returning the keyword (if any) and the offset just past the leading token.
///
/// The token is delimited by a blank or NUL only — *not* a newline — matching
/// the scan make's `do_define` uses while reading a define body (where the
/// keyword recognition deliberately differs from the general `end_of_token`).
/// `bytes` begins at the line's first token (leading blanks already skipped).
pub fn define_keyword(bytes: &[u8]) -> (Option<DefineKeyword>, usize) {
    let mut i = 0;
    while !map_set(at(bytes, i), MAP_BLANK | MAP_NUL) {
        i += 1;
    }
    (DefineKeyword::from_word(&bytes[..i]), i)
}

/// The coarse kind of a logical line, decided from its first byte before any
/// further parsing — the very top of make's `eval` dispatch.
///
/// This is the first, purely-leading-byte slice of a whole-line classifier: a
/// recipe line begins with the recipe prefix (normally a tab, overridable via
/// `.RECIPEPREFIX`); an empty line has nothing on it; everything else is a line
/// the reader must parse further (assignment, directive, rule, …). It is a pure
/// function of the first byte, so it lives in the AST layer rather than as raw
/// byte comparisons in the reader.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LineKind {
    /// An empty line (the first byte is the NUL terminator).
    Blank,
    /// A recipe line (begins with the recipe prefix character).
    Recipe,
    /// Any other line — parsed further by the reader.
    Other,
}

impl LineKind {
    /// Classify a logical line from its `first_byte` and the active recipe
    /// `prefix` (`cmd_prefix`, normally `\t`). Mirrors `eval`'s opening checks:
    /// the empty-line test comes first, then the recipe-prefix test.
    pub fn classify(first_byte: u8, prefix: u8) -> LineKind {
        if first_byte == 0 {
            LineKind::Blank
        } else if first_byte == prefix {
            LineKind::Recipe
        } else {
            LineKind::Other
        }
    }
}

/// A built-in special target whose *declaration as a target* toggles a global
/// reader mode, recognised by name in `check_specials`. These are the
/// mode-setting specials make acts on the moment the target is seen (as opposed
/// to specials like `.PHONY`/`.SUFFIXES` whose effect is on their prerequisite
/// list, handled later in `snap_deps`).
///
/// Matching the name is a pure function of the bytes, so it belongs in the AST
/// layer rather than as inlined `strcmp` chains in the reader.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpecialTarget {
    /// `.POSIX` — enables POSIX-conformant (`pedantic`) mode.
    Posix,
    /// `.SECONDEXPANSION` — enables a second expansion pass over prerequisites.
    SecondExpansion,
    /// `.ONESHELL` — run each recipe in a single shell invocation.
    OneShell,
}

impl SpecialTarget {
    /// Classify a target `name` as a reader-mode special target, or `None` if it
    /// is not one of them. Matching is exact, mirroring make's reader.
    pub fn from_name(name: &[u8]) -> Option<SpecialTarget> {
        Some(match name {
            b".POSIX" => SpecialTarget::Posix,
            b".SECONDEXPANSION" => SpecialTarget::SecondExpansion,
            b".ONESHELL" => SpecialTarget::OneShell,
            _ => return None,
        })
    }
}

/// A built-in diagnostic ("logging") expansion function — the three names that
/// make routes to a single shared handler (`func_error`): `$(error …)`,
/// `$(warning …)`, and `$(info …)`. They differ only in how their single
/// argument is reported.
///
/// make's c2rust handler switched on the raw first byte of the function name
/// (`'e'`/`'w'`/`'i'`), an opaque magic-number wall. Recognising the function is
/// a pure function of the name bytes, so it belongs in the AST layer as a typed
/// classifier rather than as inlined byte comparisons.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LogFunction {
    /// `$(error …)` — print the argument as a fatal error and stop.
    Error,
    /// `$(warning …)` — print the argument as a non-fatal warning.
    Warning,
    /// `$(info …)` — print the argument to stdout, followed by a newline.
    Info,
}

impl LogFunction {
    /// Classify a built-in function `name` as one of the diagnostic functions,
    /// or `None` if it is not one of them. Matching is exact on the full name,
    /// mirroring the closed set the function table routes to `func_error`.
    pub fn from_funcname(name: &[u8]) -> Option<LogFunction> {
        Some(match name {
            b"error" => LogFunction::Error,
            b"warning" => LogFunction::Warning,
            b"info" => LogFunction::Info,
            _ => return None,
        })
    }
}

/// The two list-token-trimming functions that make routes to a single shared
/// handler (`func_notdir_suffix`): `$(notdir …)` and `$(suffix …)`. Both walk a
/// whitespace-separated list and emit a slice of each token, differing only in
/// which part they keep — the tail after the last directory separator
/// (`$(notdir)`), or the file suffix from the last `.` (`$(suffix)`).
///
/// make's c2rust handler distinguished the two by reading the raw first byte of
/// the function name (`*funcname == 's'`), an opaque magic-number wall. Which
/// function is selected is a pure function of the name bytes, so it belongs in
/// the AST layer as a typed classifier rather than as an inline byte comparison.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NotdirSuffix {
    /// `$(notdir …)` — keep each token's tail after the last directory separator.
    Notdir,
    /// `$(suffix …)` — keep each token's suffix from its last `.` (else nothing).
    Suffix,
}

impl NotdirSuffix {
    /// Classify a built-in function `name` as one of the two list-trimming
    /// functions, or `None` if it is not one of them. Matching is exact on the
    /// full name, mirroring the closed set the function table routes to
    /// `func_notdir_suffix`.
    pub fn from_funcname(name: &[u8]) -> Option<NotdirSuffix> {
        Some(match name {
            b"notdir" => NotdirSuffix::Notdir,
            b"suffix" => NotdirSuffix::Suffix,
            _ => return None,
        })
    }
}

/// The two path-component functions that make routes to a single shared handler
/// (`func_basename_dir`): `$(basename …)` and `$(dir …)`. Both walk a
/// whitespace-separated list and emit a slice of each token, differing only in
/// which part they keep — the directory part up to and including the last
/// directory separator (`$(dir)`, defaulting to `./`), or the token with its
/// extension (from the last `.`) stripped (`$(basename)`).
///
/// make's c2rust handler distinguished the two by reading the raw first byte of
/// the function name (`*funcname == 'b'`), an opaque magic-number wall. Which
/// function is selected is a pure function of the name bytes, so it belongs in
/// the AST layer as a typed classifier rather than as an inline byte comparison.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BasenameDir {
    /// `$(basename …)` — keep each token with its extension (from the last `.`)
    /// removed; the scan also stops at directory separators so a `.` in a
    /// parent directory is not mistaken for an extension.
    Basename,
    /// `$(dir …)` — keep each token's directory part up to and including the
    /// last directory separator, defaulting to `./` when there is none.
    Dir,
}

impl BasenameDir {
    /// Classify a built-in function `name` as one of the two path-component
    /// functions, or `None` if it is not one of them. Matching is exact on the
    /// full name, mirroring the closed set the function table routes to
    /// `func_basename_dir`.
    pub fn from_funcname(name: &[u8]) -> Option<BasenameDir> {
        Some(match name {
            b"basename" => BasenameDir::Basename,
            b"dir" => BasenameDir::Dir,
            _ => return None,
        })
    }
}

/// `$(addprefix prefix,list)` and `$(addsuffix suffix,list)` both attach a fixed
/// string to each whitespace-separated word of `list`; they differ only in which
/// side the fixed string lands on.
///
/// make's c2rust handler distinguished the two by reading the raw fourth byte of
/// the function name (`funcname[3] == 'p'`), an opaque magic-number wall. Which
/// function is selected is a pure function of the name bytes, so it belongs in
/// the AST layer as a typed classifier rather than as an inline byte comparison.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AddprefixAddsuffix {
    /// `$(addprefix …)` — prepend the fixed string to each word.
    Addprefix,
    /// `$(addsuffix …)` — append the fixed string to each word.
    Addsuffix,
}

impl AddprefixAddsuffix {
    /// Classify a built-in function `name` as one of the two affix functions, or
    /// `None` if it is not one of them. Matching is exact on the full name,
    /// mirroring the closed set the function table routes to
    /// `func_addsuffix_addprefix`.
    pub fn from_funcname(name: &[u8]) -> Option<AddprefixAddsuffix> {
        Some(match name {
            b"addprefix" => AddprefixAddsuffix::Addprefix,
            b"addsuffix" => AddprefixAddsuffix::Addsuffix,
            _ => return None,
        })
    }
}

/// The typed classification of a whole logical (non-recipe) line: which of
/// make's four `eval` dispatch arms the line takes. It is composed from the
/// leading-word classifiers in make's `eval` order — conditional directives are
/// recognised first, then variable definitions (with their stackable leading
/// modifiers), then file/path directives, and finally everything else (a rule or
/// other plain line).
///
/// This unifies the per-keyword classifiers ([`Directive`], [`VarModifier`],
/// [`FileDirective`]) and the modifier/assignment scan into a single pure entry
/// point, so the reader can classify a line through one typed AST call rather
/// than re-deriving the dispatch inline. The variable-definition variant is
/// interned through salsa (see [`classify_line`]); the others are small `Copy`
/// enums matched directly, as interning them would only add lock traffic on a
/// hot path.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LineClass {
    /// A conditional directive line (`ifdef`/`ifeq`/`else`/`endif`/…).
    Conditional(Directive),
    /// A variable definition, carrying its leading modifiers and the offset of
    /// the definition tail.
    VarDef(VarLine),
    /// A file/path directive line (`include`/`vpath`/`load`/…).
    File(FileDirective),
    /// Anything else — a rule or other plain line.
    Plain,
}

/// The variable-definition payload of [`LineClass::VarDef`]: the leading
/// modifiers consumed, whether any modifier was consumed, whether the remainder
/// is an assignment, and the offset (into the original line) where the remainder
/// begins. This is exactly the pure-data result of [`scan_var_modifiers`], lifted
/// into the whole-line classification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VarLine {
    /// The modifier flags accumulated from the leading keywords.
    pub mods: VarModifiers,
    /// At least one modifier keyword was consumed.
    pub had_modifier: bool,
    /// The text after the modifiers is a variable definition (`assign_v`).
    pub assign: bool,
    /// Offset into the original line where the definition tail begins.
    pub rest: usize,
}

/// Which export state a leading `export`/`unexport` modifier requests.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ExportMode {
    /// `export`
    Export,
    /// `unexport`
    NoExport,
}

/// The variable-definition modifiers (`export`/`override`/`private`/`define`/…)
/// that may stack in front of an assignment or `define` line — the owned,
/// pure-data result of scanning a line's leading modifier keywords. Mirrors the
/// `VModifiers` bitfield make's `parse_var_assignment` fills, but as plain data
/// with no pointers or side effects.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct VarModifiers {
    /// `export` / `unexport`, if present.
    pub export: Option<ExportMode>,
    /// `override` seen.
    pub over: bool,
    /// `private` seen.
    pub private: bool,
    /// `define` seen (opens a multi-line definition).
    pub define: bool,
    /// `undefine` seen.
    pub undefine: bool,
}

/// The result of [`scan_var_modifiers`]: the modifiers consumed at the front of
/// a line, whether the remainder is a variable definition, and the byte offset
/// where the remainder begins (relative to the original line).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VarModScan {
    /// The modifier flags accumulated from the leading keywords.
    pub mods: VarModifiers,
    /// At least one modifier keyword was consumed. (Make warns once,
    /// "directive lines cannot start with TAB", when this holds and the line
    /// began with a tab.)
    pub had_modifier: bool,
    /// The text after the modifiers is a variable definition (`assign_v`).
    pub assign: bool,
    /// Offset into the original line of the returned position: the start of the
    /// definition when `assign`, otherwise the first non-blank byte of the line.
    pub rest: usize,
}

/// Advance past a token (to the first blank/newline/NUL), mirroring make's
/// `end_of_token`.
fn end_of_token_off(bytes: &[u8], mut i: usize) -> usize {
    while !map_set(at(bytes, i), MAP_BLANK | MAP_NEWLINE | MAP_NUL) {
        i += 1;
    }
    i
}

/// Skip leading blanks/newlines, mirroring make's `next_token`.
fn next_token_off(bytes: &[u8], mut i: usize) -> usize {
    while map_set(at(bytes, i), MAP_BLANK | MAP_NEWLINE) {
        i += 1;
    }
    i
}

/// Whether `bytes` (a logical line) closes an *ignored* `define` body: its
/// first token is exactly `endef` and only a comment or end-of-line follows.
///
/// Mirrors the check make's `eval` applies while skipping the body of a `define`
/// it is ignoring (inside a false conditional): the body is consumed verbatim
/// until a line that is a bare `endef`.
pub fn closes_ignored_define(bytes: &[u8]) -> bool {
    let word_end = end_of_token_off(bytes, 0);
    if DefineKeyword::from_word(&bytes[..word_end]) != Some(DefineKeyword::Endef) {
        return false;
    }
    map_set(
        at(bytes, next_token_off(bytes, word_end)),
        MAP_COMMENT | MAP_NUL,
    )
}

/// The result of [`rule_probe`]: where a line's first token ends, where the
/// text after it (its trailing blanks skipped) begins, and whether the line is
/// a rule.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RuleProbe {
    /// Length of the leading token (`end_of_token`).
    pub word_len: usize,
    /// Offset of the first non-blank byte after the leading token.
    pub rest: usize,
    /// The line is a rule: a `:` (or `&:` / `|:`) follows the first token.
    pub is_rule: bool,
}

/// Probe a target/rule line, mirroring make's `eval`: measure the first token,
/// skip the blanks after it, and decide whether the line is a rule — its next
/// significant character is `:`, or `&:` / `|:` (grouped/`.NOTPARALLEL`-style
/// separators followed by the rule colon). `bytes` begins at the line's first
/// non-blank byte.
pub fn rule_probe(bytes: &[u8]) -> RuleProbe {
    let word_len = end_of_token_off(bytes, 0);
    let rest = next_token_off(bytes, word_len);
    let c = at(bytes, rest);
    let is_rule = c == b':' || ((c == b'&' || c == b'|') && at(bytes, rest + 1) == b':');
    RuleProbe {
        word_len,
        rest,
        is_rule,
    }
}

/// The byte range of the name in `bytes`: skip leading whitespace, then trim
/// trailing blanks, returning `None` when the input is empty or all whitespace.
///
/// This mirrors the `next_token` + trailing-blank-trim that make's `do_define`
/// and `do_undefine` apply to isolate an (already-expanded) variable name; an
/// empty result is make's "empty variable name" fatal. Only blanks are trimmed
/// from the tail (not newlines), and at least one byte is always kept, matching
/// make's `while (p > name && ...)` loop.
pub fn trimmed_token(bytes: &[u8]) -> Option<Range<usize>> {
    let start = next_token_off(bytes, 0);
    if at(bytes, start) == 0 {
        return None;
    }
    let mut end = bytes.len();
    while end - 1 > start && map_set(at(bytes, end - 1), MAP_BLANK) {
        end -= 1;
    }
    Some(start..end)
}

/// Whether `bytes` is empty or only whitespace — the first significant byte
/// (after skipping leading blanks/newlines) is the NUL terminator. Mirrors
/// make's `*next_token(p) == '\0'` "nothing left on this line" check.
pub fn rest_is_blank(bytes: &[u8]) -> bool {
    at(bytes, next_token_off(bytes, 0)) == 0
}

/// Whether a "missing separator" line begins with `ifeq`/`ifneq` that is *not*
/// followed by whitespace — e.g. the user wrote `ifeq(...)` or `ifneq(...)`
/// with no space, so `eval` should emit the more specific "ifeq/ifneq must be
/// followed by whitespace" diagnostic instead of the bare "missing separator".
///
/// Mirrors make's c2rust `strncmp` wall: the line must start with `if`, then
/// either `neq` followed by a non-blank byte, or `eq` followed by a non-blank
/// byte. "Non-blank" tests only `MAP_BLANK` (matching the C `& 0x2`), so a NUL
/// terminator past the token end counts as non-blank and still triggers.
pub fn ifeq_ifneq_without_separator(bytes: &[u8]) -> bool {
    if bytes.get(..2) != Some(b"if".as_slice()) {
        return false;
    }
    let non_blank = |i: usize| !map_set(at(bytes, i), MAP_BLANK);
    (bytes.get(2..5) == Some(b"neq".as_slice()) && non_blank(5))
        || (bytes.get(2..4) == Some(b"eq".as_slice()) && non_blank(4))
}

/// Whether a parsed file-sequence token is exactly `.WAIT` — the special
/// prerequisite ordering marker make recognizes when `PARSEFS_WAIT` is set.
/// Mirrors `parse_file_seq`'s `(p - s) == 5 && memcmp(s, ".WAIT", 5) == 0`.
pub fn is_wait_token(token: &[u8]) -> bool {
    token == b".WAIT"
}

/// Whether a prerequisite string needs second expansion: under
/// `.SECONDEXPANSION` it is re-expanded only if it actually contains a `$`.
/// Mirrors `record_files`' `strchr(depstr, '$') != NULL` test.
pub fn prereq_needs_second_expansion(depstr: &[u8]) -> bool {
    depstr.contains(&b'$')
}

/// In-place port of make's `find_char_unquote`: scan `buf` (a NUL-terminated C
/// string slice, including the terminator) for the first *unquoted* `stop`
/// byte, collapsing any run of backslashes that precedes a `stop` and shifting
/// the tail left. Returns the index of the unescaped `stop`, or `None` if none
/// remains.
///
/// A `stop` is escaped when an odd number of backslashes immediately precedes
/// it; each pair of backslashes collapses to one. This mirrors the c2rust
/// `strchr`/`strlen`/`memmove` pointer routine exactly, but over byte indices
/// and `copy_within` instead of raw pointers.
pub fn find_char_unquote_idx(buf: &mut [u8], stop: u8) -> Option<usize> {
    let mut string_len: Option<usize> = None;
    let mut p = 0usize;
    loop {
        // strchr(p, stop): advance to the next `stop` byte or the NUL.
        while buf[p] != 0 && buf[p] != stop {
            p += 1;
        }
        if buf[p] == 0 {
            return None;
        }
        if p > 0 && buf[p - 1] == b'\\' {
            // Number of consecutive backslashes immediately before `p` (the C
            // routine's `-i`). It is at most `p`, so all index math below stays
            // in `usize` without any negative-to-unsigned casts.
            let n = buf[..p].iter().rev().take_while(|&&b| b == b'\\').count();
            let slen = *string_len
                .get_or_insert_with(|| buf.iter().position(|&b| b == 0).unwrap_or(buf.len()));
            // Collapse each pair of backslashes: shift the tail (from the first
            // kept backslash, p - n/2) left over the escaping ones (to p - n).
            let half = n / 2;
            let dest = p - n;
            let src = p - half;
            // `src + len == slen + 1`, i.e. the whole tail including the NUL.
            let len = slen - p + half + 1;
            buf.copy_within(src..src + len, dest);
            p = src;
            if n % 2 == 0 {
                // Even run: the `stop` is unescaped.
                return Some(p);
            }
            // Odd run: the `stop` was escaped and consumed; keep scanning.
        } else {
            return Some(p);
        }
    }
}

/// In-place port of make's `find_map_unquote`: scan `buf` (a NUL-terminated C
/// string slice, including the terminator) for the first *unquoted* byte whose
/// stopchar-map flags intersect `stopmap`.
///
/// Like [`find_char_unquote_idx`] it collapses a backslash run before a stop
/// byte (an odd run escapes it), shifting the tail left; additionally it skips
/// over `$(...)`/`${...}` references (so a stop byte inside one does not count),
/// reusing [`skip_reference`]. `MAP_NUL` is always added to `stopmap`, so the
/// scan halts at the terminator. Returns the index of the unescaped stop byte,
/// or `None` if none remains. Mirrors the c2rust pointer routine exactly.
pub fn find_map_unquote_idx(buf: &mut [u8], stopmap: i32) -> Option<usize> {
    let stopmap = stopmap | MAP_NUL;
    let mut string_len: Option<usize> = None;
    let mut p = 0usize;
    loop {
        // Advance to the next stop-map byte (or the NUL, which is in `stopmap`).
        while !map_set(buf[p], stopmap) {
            p += 1;
        }
        if buf[p] == 0 {
            return None;
        }
        if buf[p] == b'$' {
            // A reference is opaque to the scan; skip past it and continue.
            p = skip_reference(buf, p + 1);
        } else if p > 0 && buf[p - 1] == b'\\' {
            // Same backslash collapse as `find_char_unquote_idx`.
            let n = buf[..p].iter().rev().take_while(|&&b| b == b'\\').count();
            let slen = *string_len
                .get_or_insert_with(|| buf.iter().position(|&b| b == 0).unwrap_or(buf.len()));
            let half = n / 2;
            let dest = p - n;
            let src = p - half;
            let len = slen - p + half + 1;
            buf.copy_within(src..src + len, dest);
            p = src;
            if n % 2 == 0 {
                return Some(p);
            }
        } else {
            return Some(p);
        }
    }
}

/// Pure port of make's `unescape_char`: remove one level of `\` escaping in
/// front of the byte `c`, returning the rewritten bytes (without a trailing
/// NUL).
///
/// A run of `n` backslashes immediately preceding `c` escapes it only when `n`
/// is odd; that run collapses to `n / 2` backslashes and the `c` is kept
/// unescaped. Runs that are even, or that precede any other byte (or the end of
/// string), are copied through verbatim — `c` itself is otherwise untouched.
/// This mirrors the c2rust `memmove` pointer routine exactly, over byte indices.
pub fn unescape_char(s: &[u8], c: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    let mut i = 0;
    while i < s.len() {
        if s[i] == b'\\' {
            // Span the run of backslashes; `i` lands on the following byte.
            let start = i;
            while i < s.len() && s[i] == b'\\' {
                i += 1;
            }
            let run = i - start;
            let after = s.get(i).copied();
            if after != Some(c) || run % 2 == 0 {
                // Not an escape of `c`: copy the whole run verbatim. If it ran
                // to the end of the string, we are done.
                out.extend_from_slice(&s[start..i]);
                if after.is_none() {
                    return out;
                }
            } else if run > 1 {
                // Odd run before `c`: collapse each pair, dropping the escaping
                // backslash. (`run == 1` copies nothing — the lone `\` is gone.)
                out.extend_from_slice(&s[start..start + run / 2]);
            }
            // Fall through to copy the byte after the run (the `c` or other).
        }
        if i < s.len() {
            out.push(s[i]);
            i += 1;
        }
    }
    out
}

/// Result of [`find_percent_cached`]: either the input needs no rewrite (the
/// `%`, if any, is already unquoted) or its backslashes were collapsed into a
/// fresh buffer that the caller must intern.
#[derive(Debug, PartialEq, Eq)]
pub enum FindPercentCached {
    /// No rewrite needed; the `%` sits at this index in the original string, or
    /// `None` if there is no `%` at all.
    AsIs(Option<usize>),
    /// The string contained an escaped `%`: `buf` is the NUL-terminated copy
    /// with each preceding backslash run collapsed, and `idx` is the index of
    /// the first now-unquoted `%` within it (`None` if none remains).
    Collapsed { buf: Vec<u8>, idx: Option<usize> },
}

/// Pure port of make's `find_percent_cached`: locate the first *unquoted* `%` in
/// a pattern, collapsing backslash escapes only when an escaped `%` is actually
/// present.
///
/// `s` is the pattern bytes without the trailing NUL. If the first `%` is absent,
/// at the start, or not preceded by `\`, no copy is made and the `%` index (or
/// `None`) is returned as [`FindPercentCached::AsIs`] — the caller leaves the
/// interned string untouched. Otherwise a NUL-terminated copy is collapsed via
/// [`find_char_unquote_idx`] (the same routine `find_percent` already uses) and
/// returned as [`FindPercentCached::Collapsed`] for the caller to intern. This
/// mirrors the c2rust `strchr`/`memmove`/`strcache_add` pointer routine.
pub fn find_percent_cached(s: &[u8]) -> FindPercentCached {
    match s.iter().position(|&b| b == b'%') {
        None => FindPercentCached::AsIs(None),
        Some(0) => FindPercentCached::AsIs(Some(0)),
        Some(p) if s[p - 1] != b'\\' => FindPercentCached::AsIs(Some(p)),
        Some(_) => {
            let mut buf = Vec::with_capacity(s.len() + 1);
            buf.extend_from_slice(s);
            buf.push(0);
            let idx = find_char_unquote_idx(&mut buf, b'%');
            FindPercentCached::Collapsed { buf, idx }
        }
    }
}

/// Whether a line begins with eight space characters — the heuristic make uses
/// (when the command prefix is a TAB) to suggest "did you mean TAB instead of 8
/// spaces?" on a missing-separator error. Mirrors `strncmp(line, "        ", 8)`.
pub fn starts_with_eight_spaces(line: &[u8]) -> bool {
    line.get(..8) == Some(b"        ".as_slice())
}

/// How many leading bytes of a file-sequence token make's `parse_file_seq`
/// strips as redundant `./` prefixes (when `PARSEFS_NOSTRIP` is not set):
/// repeatedly drop a `./` pair followed by any run of `/`, as long as more than
/// two bytes of the original token remain. Returns the offset to advance the
/// token start by. Mirrors the c2rust pointer loop without mutating the buffer.
pub fn strip_dot_slash_prefix(token: &[u8]) -> usize {
    let mut i = 0;
    while token.len() - i > 2 && token.get(i) == Some(&b'.') && token.get(i + 1) == Some(&b'/') {
        i += 2;
        while token.get(i) == Some(&b'/') {
            i += 1;
        }
    }
    i
}

/// If `bytes` is a single leading token optionally followed by only trailing
/// blanks, return the token's length; otherwise `None`.
///
/// This is the pure check make's `conditional_line` performs on the (expanded)
/// argument of `ifdef`/`ifndef`: it takes the first token (`end_of_token`),
/// skips any trailing whitespace, and requires the string to end there — a
/// second token is a syntax error (make's `-1`, "invalid syntax in
/// conditional"). The token starts at offset 0 (the caller has already expanded
/// the argument, which has no leading blanks).
pub fn lone_token(bytes: &[u8]) -> Option<usize> {
    let end = end_of_token_off(bytes, 0);
    if at(bytes, next_token_off(bytes, end)) == 0 {
        Some(end)
    } else {
        None
    }
}

/// Scan the leading variable-definition modifiers of a line, a pure,
/// offset-based reproduction of make's `parse_var_assignment` (`read.c`).
///
/// `targvar` is true in a target-specific variable context, where `define` /
/// `undefine` are *not* treated as modifiers (they are plain names). The scan
/// tries the assignment detector ([`parse_assignment`]) at each step *before*
/// classifying a modifier keyword, so `export = 1` is an assignment to the
/// variable `export`, while `export FOO = 1` consumes `export` as a modifier.
pub fn scan_var_modifiers(bytes: &[u8], targvar: bool) -> VarModScan {
    let mut i = next_token_off(bytes, 0);
    let line_start = i;
    let mut mods = VarModifiers::default();
    let mut had_modifier = false;
    if at(bytes, i) == 0 {
        return VarModScan {
            mods,
            had_modifier,
            assign: false,
            rest: line_start,
        };
    }
    loop {
        // An assignment at the current position ends the scan (the remainder is
        // a definition, not another modifier).
        if parse_assignment(&bytes[i..]).is_some() {
            return VarModScan {
                mods,
                had_modifier,
                assign: true,
                rest: i,
            };
        }
        let w_end = end_of_token_off(bytes, i);
        let word = &bytes[i..w_end];
        match VarModifier::from_word(word) {
            Some(VarModifier::Export) => mods.export = Some(ExportMode::Export),
            Some(VarModifier::Unexport) => mods.export = Some(ExportMode::NoExport),
            Some(VarModifier::Override) => mods.over = true,
            Some(VarModifier::Private) => mods.private = true,
            Some(VarModifier::Define) if !targvar => {
                mods.define = true;
                return VarModScan {
                    mods,
                    had_modifier: true,
                    assign: true,
                    rest: next_token_off(bytes, w_end),
                };
            }
            Some(VarModifier::Undefine) if !targvar => {
                mods.undefine = true;
                return VarModScan {
                    mods,
                    had_modifier: true,
                    assign: true,
                    rest: next_token_off(bytes, w_end),
                };
            }
            // Any other word (including `define`/`undefine` in a target-var
            // context) means this is not a modifier-led definition.
            _ => {
                return VarModScan {
                    mods,
                    had_modifier,
                    assign: false,
                    rest: line_start,
                };
            }
        }
        had_modifier = true;
        i = next_token_off(bytes, w_end);
        if at(bytes, i) == 0 {
            return VarModScan {
                mods,
                had_modifier,
                assign: false,
                rest: line_start,
            };
        }
    }
}

/// `stopchar_map` class bits for byte `b`.
fn flags(b: u8) -> i32 {
    stopchar_map()[b as usize] as i32
}

/// Is any bit of `mask` set for byte `b` in the stopchar map?
fn map_set(b: u8, mask: i32) -> bool {
    flags(b) & mask != 0
}

/// Byte at `i`, or a NUL terminator past the end — the slice excludes the
/// trailing NUL of the original C string, and reading it stops the scan.
fn at(bytes: &[u8], i: usize) -> u8 {
    bytes.get(i).copied().unwrap_or(0)
}

/// Skip a `$(...)`/`${...}` reference embedded in a variable name, honoring
/// nested parens/braces. `p` points just past the `$`. Mirrors
/// [`crate::misc::skip_reference`].
fn skip_reference(bytes: &[u8], mut p: usize) -> usize {
    let openparen = at(bytes, p);
    if openparen == 0 {
        return p;
    }
    let closeparen = match openparen {
        b'(' => b')',
        b'{' => b'}',
        // Single-character reference like `$X`.
        _ => return p + 1,
    };
    let mut count: i32 = 1;
    loop {
        p += 1;
        let c = at(bytes, p);
        if !map_set(c, MAP_NUL | MAP_VARSEP) {
            continue;
        }
        if c == 0 {
            break;
        }
        if c == openparen {
            count += 1;
        } else if c == closeparen {
            count -= 1;
            if count == 0 {
                p += 1;
                break;
            }
        }
    }
    p
}

/// The classification make's `get_next_mword` assigns to the next word of a
/// rule line: the special separators set their type directly; everything else
/// is `Static` (or `Variable` if a `$ref` was crossed while scanning).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MWordType {
    Eol,
    Static,
    Variable,
    Colon,
    DColon,
    Semicolon,
    AmpColon,
    AmpDColon,
}

/// Pure port of make's `get_next_mword`: scan the next "make word" out of a rule
/// line, returning its type and its `[start, start + len)` span within `buf`.
///
/// Leading blanks/newlines are skipped; the first byte selects a separator
/// (`;`, `:`/`::`, `&:`/`&::`, or end-of-line), otherwise a static word is
/// scanned up to the next separator/blank, honoring `$(...)` references (which
/// promote the word to [`MWordType::Variable`]) and `\`-escapes. Mirrors the
/// c2rust pointer routine exactly, over byte indices, reusing [`skip_reference`].
pub fn get_next_mword(buf: &[u8]) -> (MWordType, usize, usize) {
    let mut p = 0usize;
    while map_set(at(buf, p), MAP_BLANK | MAP_NEWLINE) {
        p += 1;
    }
    let beg = p;
    let mut c = at(buf, p);
    p += 1;
    let mut scan_static = false;
    let mut wtype = MWordType::Eol;
    match c {
        0 => wtype = MWordType::Eol,
        b';' => wtype = MWordType::Semicolon,
        b':' => {
            wtype = MWordType::Colon;
            if at(buf, p) == b':' {
                p += 1;
                wtype = MWordType::DColon;
            }
        }
        b'&' => {
            if at(buf, p) == b':' {
                p += 1;
                if at(buf, p) != b':' {
                    wtype = MWordType::AmpColon;
                } else {
                    p += 1;
                    wtype = MWordType::AmpDColon;
                }
            } else {
                scan_static = true;
            }
        }
        _ => scan_static = true,
    }
    if scan_static {
        wtype = MWordType::Static;
        while !map_set(c, MAP_BLANK | MAP_NEWLINE | MAP_NUL) {
            match c {
                b':' => break,
                b'$' => {
                    c = at(buf, p);
                    p += 1;
                    if c != b'$' {
                        if c == 0 {
                            break;
                        }
                        wtype = MWordType::Variable;
                        p = skip_reference(buf, p - 1);
                    }
                }
                b'\\' => {
                    if matches!(at(buf, p), b':' | b';' | b'=' | b'\\') {
                        p += 1;
                    }
                }
                b'&' if at(buf, p) == b':' => break,
                _ => {}
            }
            c = at(buf, p);
            p += 1;
        }
        p -= 1;
    }
    (wtype, beg, p - beg)
}

/// Parse `bytes` (one logical line, without its trailing NUL) as a variable
/// assignment, returning the typed [`Assignment`] or `None` when the line is not
/// a definition.
///
/// This is a safe, slice-based reproduction of make's `parse_variable_definition`
/// state machine; every branch is preserved so the result is identical to the C
/// reader for the same input.
pub fn parse_assignment(bytes: &[u8]) -> Option<Assignment> {
    let space = MAP_BLANK | MAP_NEWLINE;

    // NEXT_TOKEN: skip leading whitespace to the start of the name.
    let mut p = 0usize;
    while map_set(at(bytes, p), space) {
        p += 1;
    }
    let name_start = p;
    let mut end: Option<usize> = None;
    let mut conditional = false;
    let flavor;

    loop {
        let c0 = at(bytes, p);
        p += 1;

        // A comment or end-of-string before an operator: not a definition.
        if map_set(c0, MAP_COMMENT | MAP_NUL) {
            return None;
        }

        // Whitespace ends the name; a second run of whitespace (i.e. a name
        // with embedded blanks) means this isn't a definition.
        if map_set(c0, MAP_BLANK) {
            if end.is_some() {
                return None;
            }
            end = Some(p - 1);
            while map_set(at(bytes, p), space) {
                p += 1;
            }
            continue;
        }

        let start = p - 1;
        let mut c = c0;
        if c == b'?' {
            conditional = true;
            c = at(bytes, p);
            p += 1;
        }

        if c == b'=' {
            if end.is_none() {
                end = Some(start);
            }
            flavor = Flavor::Recursive;
            break;
        } else if c == b':' {
            if end.is_none() {
                end = Some(start);
            }
            c = at(bytes, p);
            p += 1;
            if c == b'=' {
                flavor = Flavor::Simple;
                break;
            } else {
                if c == b':' {
                    c = at(bytes, p);
                    p += 1;
                    if c == b'=' {
                        flavor = Flavor::Simple;
                        break;
                    } else if c == b':' && {
                        let t = at(bytes, p);
                        p += 1;
                        t == b'='
                    } {
                        flavor = Flavor::Expand;
                        break;
                    }
                }
                return None;
            }
        } else {
            if at(bytes, p) == b'=' {
                match c {
                    b'+' => {
                        flavor = Flavor::Append;
                        if end.is_none() {
                            end = Some(start);
                        }
                        p += 1;
                        break;
                    }
                    b'!' => {
                        flavor = Flavor::Shell;
                        if end.is_none() {
                            end = Some(start);
                        }
                        p += 1;
                        break;
                    }
                    _ => {}
                }
            }
            if end.is_some() {
                return None;
            }
            if c == b'$' {
                p = skip_reference(bytes, p);
            }
            conditional = false;
        }
    }

    let end = end.expect("end is set on every break path");
    let op_end = p;
    let mut value = p;
    while map_set(at(bytes, value), space) {
        value += 1;
    }

    Some(Assignment {
        name_start,
        name_len: end - name_start,
        flavor,
        conditional,
        op_end,
        value_start: value,
    })
}

/// The result of [`parse_conditional_args`]: the byte-ranges of an `ifeq`/`ifneq`
/// conditional's two arguments, parsed from the (unexpanded) text after the
/// directive keyword.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ConditionalArgs {
    /// Both arguments parsed successfully.
    Both {
        /// Byte range of the first argument within the line.
        arg1: Range<usize>,
        /// Byte range of the second argument.
        arg2: Range<usize>,
        /// Non-blank text follows the closing delimiter. make warns `extraneous
        /// text after '%s' directive` but still evaluates the conditional.
        trailing_text: bool,
    },
    /// The first argument parsed, but the second is malformed. make expands the
    /// first argument (firing any `$(info)`/`$(warning)`/`$(error)`/`$(file)`
    /// side effects) *before* it validates the second delimiter, so the caller
    /// must still expand `arg1` before reporting the syntax error.
    FirstArgOnly {
        /// Byte range of the (well-formed) first argument.
        arg1: Range<usize>,
    },
    /// The line is malformed before a complete first argument; make reports the
    /// syntax error without expanding anything.
    Error,
}

/// Parse the argument forms of an `ifeq`/`ifneq` line — `(a,b)`, `"a" "b"`,
/// `'a' 'b'`.
///
/// `bytes` begins at the first argument character (the opening `(` or quote).
/// This is a safe, slice-based reproduction of the reference-aware delimiter
/// scan in make's `conditional_line`: a `$` begins a `$(...)`/`${...}` reference
/// skipped via [`skip_reference`], the comma form trims trailing blanks of the
/// first argument and balances parentheses while scanning the second, and the
/// quoted forms take everything up to the matching quote verbatim.
///
/// The result distinguishes the two syntax-error positions make has, because
/// they differ observably: a failure *after* the first argument is complete
/// still expands the first argument (see [`ConditionalArgs::FirstArgOnly`]),
/// while a failure before it expands nothing ([`ConditionalArgs::Error`]).
pub fn parse_conditional_args(bytes: &[u8]) -> ConditionalArgs {
    let mut p = 0usize;
    // Opening delimiter: `(` selects the comma-separated form; a quote selects
    // the quoted form (the same quote closes the argument).
    let first = at(bytes, p);
    let comma_form = first == b'(';
    let mut termin = if comma_form { b',' } else { first };
    if termin != b',' && termin != b'"' && termin != b'\'' {
        return ConditionalArgs::Error;
    }
    p += 1;
    let arg1_start = p;
    while at(bytes, p) != 0 && at(bytes, p) != termin {
        if at(bytes, p) == b'$' {
            p = skip_reference(bytes, p + 1);
        } else {
            p += 1;
        }
    }
    if at(bytes, p) == 0 {
        return ConditionalArgs::Error;
    }
    let arg1_end = if comma_form {
        // Trim trailing blanks of the first argument before the comma.
        let mut e = p;
        while map_set(at(bytes, e - 1), MAP_BLANK) {
            e -= 1;
        }
        p += 1; // past the comma
        e
    } else {
        let e = p;
        p += 1; // past the closing quote
        e
    };
    let arg1 = arg1_start..arg1_end;

    // From here the first argument is complete, so any failure is reported as
    // `FirstArgOnly` (make has already expanded it by this point).
    // Second delimiter: the quoted form skips the blanks between the two
    // strings; the comma form expects a balanced `)`.
    if !comma_form {
        while map_set(at(bytes, p), MAP_BLANK | MAP_NEWLINE) {
            p += 1;
        }
    }
    termin = if comma_form { b')' } else { at(bytes, p) };
    if termin != b')' && termin != b'"' && termin != b'\'' {
        return ConditionalArgs::FirstArgOnly { arg1 };
    }
    let arg2_start;
    if termin == b')' {
        // Skip leading blanks, then scan to the matching close paren.
        while map_set(at(bytes, p), MAP_BLANK | MAP_NEWLINE) {
            p += 1;
        }
        arg2_start = p;
        let mut count: i32 = 0;
        while at(bytes, p) != 0 {
            let c = at(bytes, p);
            if c == b'(' {
                count += 1;
            } else if c == b')' {
                if count <= 0 {
                    break;
                }
                count -= 1;
            }
            p += 1;
        }
    } else {
        p += 1; // skip the opening quote
        arg2_start = p;
        while at(bytes, p) != 0 && at(bytes, p) != termin {
            p += 1;
        }
    }
    if at(bytes, p) == 0 {
        return ConditionalArgs::FirstArgOnly { arg1 };
    }
    let arg2_end = p;
    p += 1; // past the closing delimiter
    while map_set(at(bytes, p), MAP_BLANK | MAP_NEWLINE) {
        p += 1;
    }
    ConditionalArgs::Both {
        arg1,
        arg2: arg2_start..arg2_end,
        trailing_text: at(bytes, p) != 0,
    }
}

// --- salsa front-end -------------------------------------------------------

/// An assignment AST node interned in the parser database. Interning is keyed by
/// the node's offsets/flavor — not the source bytes — so the database holds at
/// most one entry per distinct assignment shape and never retains a per-line
/// byte copy.
#[salsa::interned]
struct AssignmentNode<'db> {
    name_start: usize,
    name_len: usize,
    flavor: Flavor,
    conditional: bool,
    op_end: usize,
    value_start: usize,
}

/// A variable-definition line node interned in the parser database. Like
/// [`AssignmentNode`], interning is keyed by the node's pure-data fields (the
/// modifier flags and the offset of the definition tail) rather than the source
/// bytes, so the database holds at most one entry per distinct definition shape
/// and never retains a per-line byte copy. Only lines that are genuine variable
/// definitions (a leading modifier was consumed, or the tail parses as an
/// assignment) are interned — the same "intern only real matches" discipline
/// [`assignment_ast`] applies, keeping conditionals, rules and plain lines out of
/// the database.
#[salsa::interned]
struct VarDefNode<'db> {
    export: Option<ExportMode>,
    over: bool,
    private: bool,
    define: bool,
    undefine: bool,
    had_modifier: bool,
    assign: bool,
    rest: usize,
}

/// Parse `bytes` as a variable assignment, returning the typed [`Assignment`] or
/// `None`.
///
/// The reader probes this for *every* candidate line, and most lines (rules,
/// directives, comments) are not assignments. Those are parsed by the safe
/// [`parse_assignment`] and returned immediately, so they never enter the salsa
/// database — interning them would copy each mostly-unique line into static
/// storage retained until process exit. Only lines that actually parse as
/// assignments are interned, which both bounds memory and routes genuine AST
/// nodes through salsa.
pub fn assignment_ast(db: &crate::makedb::MakeDb, bytes: &[u8]) -> Option<Assignment> {
    let parsed = parse_assignment(bytes)?;
    let node = AssignmentNode::new(
        db,
        parsed.name_start,
        parsed.name_len,
        parsed.flavor,
        parsed.conditional,
        parsed.op_end,
        parsed.value_start,
    );
    Some(Assignment {
        name_start: *node.name_start(db),
        name_len: *node.name_len(db),
        flavor: *node.flavor(db),
        conditional: *node.conditional(db),
        op_end: *node.op_end(db),
        value_start: *node.value_start(db),
    })
}

/// Classify a whole logical (non-recipe) line into its [`LineClass`], mirroring
/// the dispatch order at the top of make's `eval`. make runs the modifier/
/// assignment scan (`parse_var_assignment`) *first*, so a line whose variable
/// name happens to be a keyword — `include = x`, `vpath = x`, `ifdef = x` — is a
/// genuine assignment, not a directive. Only when the line is **not** an
/// assignment do the directive arms run, in `eval`'s order: a conditional
/// directive, then a file/path directive (both keyed off the leading word). A
/// multi-word keyword line such as `include FOO = 1` is *not* an assignment (the
/// scan rejects the embedded blank), so it still falls through to
/// [`LineClass::File`]. A modifier-led line that is not itself a definition
/// (bare `export`, `override foo: bar`) is reported as a [`LineClass::VarDef`]
/// carrying the consumed modifiers. Everything else is [`LineClass::Plain`].
///
/// Only the variable-definition variant carries an offset payload, so it is the
/// only one interned (through [`VarDefNode`]); the keyword variants are small
/// `Copy` enums returned directly, the same "intern only the node with real
/// variable-length state" discipline as [`assignment_ast`].
///
/// `targvar` is true in a target-specific variable context, where `define` /
/// `undefine` are plain names rather than modifiers (see [`scan_var_modifiers`]).
pub fn classify_line(db: &crate::makedb::MakeDb, bytes: &[u8], targvar: bool) -> LineClass {
    // make's `eval` probes for a variable definition before any directive
    // dispatch, so a genuine assignment wins even when its name is a keyword.
    let scan = scan_var_modifiers(bytes, targvar);
    if scan.assign {
        return var_def(db, scan);
    }
    // Not an assignment: the directive arms run, keyed off the leading word.
    let i = next_token_off(bytes, 0);
    let w_end = end_of_token_off(bytes, i);
    let word = &bytes[i..w_end];
    if let Some(d) = Directive::from_word(word) {
        return LineClass::Conditional(d);
    }
    if let Some(f) = FileDirective::from_word(word) {
        return LineClass::File(f);
    }
    // A modifier keyword was consumed but the remainder is not a definition
    // (bare `export`, `override foo: bar`): still a variable-definition line.
    if scan.had_modifier {
        return var_def(db, scan);
    }
    LineClass::Plain
}

/// Intern a variable-definition [`VarModScan`] as a [`VarDefNode`] and return the
/// owned [`LineClass::VarDef`] for it.
fn var_def(db: &crate::makedb::MakeDb, scan: VarModScan) -> LineClass {
    let node = VarDefNode::new(
        db,
        scan.mods.export,
        scan.mods.over,
        scan.mods.private,
        scan.mods.define,
        scan.mods.undefine,
        scan.had_modifier,
        scan.assign,
        scan.rest,
    );
    LineClass::VarDef(VarLine {
        mods: VarModifiers {
            export: *node.export(db),
            over: *node.over(db),
            private: *node.private(db),
            define: *node.define(db),
            undefine: *node.undefine(db),
        },
        had_modifier: *node.had_modifier(db),
        assign: *node.assign(db),
        rest: *node.rest(db),
    })
}

#[cfg(test)]
mod tests {
    use {super::*, crate::entry::initialize_stopchar_map, std::sync::Once};

    /// The byte classifier reads the process-global stopchar map, which `main`
    /// builds at startup; tests must initialize it once before parsing.
    fn ensure_map() {
        static INIT: Once = Once::new();
        INIT.call_once(initialize_stopchar_map);
    }

    fn parse(s: &str) -> Option<Assignment> {
        ensure_map();
        parse_assignment(s.as_bytes())
    }

    /// Helper: parse and return (name, flavor, conditional, value) as strings.
    fn parts(s: &str) -> Option<(String, Flavor, bool, String)> {
        let a = parse(s)?;
        let b = s.as_bytes();
        Some((
            String::from_utf8(b[a.name()].to_vec()).unwrap(),
            a.flavor,
            a.conditional,
            String::from_utf8(b[a.value_start..].to_vec()).unwrap(),
        ))
    }

    #[test]
    fn recursive_assignment() {
        assert_eq!(
            parts("FOO = bar"),
            Some(("FOO".into(), Flavor::Recursive, false, "bar".into()))
        );
    }

    #[test]
    fn simple_assignment_colon_eq() {
        assert_eq!(
            parts("FOO := bar"),
            Some(("FOO".into(), Flavor::Simple, false, "bar".into()))
        );
    }

    #[test]
    fn simple_assignment_double_colon_eq() {
        assert_eq!(
            parts("FOO ::= bar"),
            Some(("FOO".into(), Flavor::Simple, false, "bar".into()))
        );
    }

    #[test]
    fn expand_assignment_triple_colon_eq() {
        assert_eq!(
            parts("FOO :::= bar"),
            Some(("FOO".into(), Flavor::Expand, false, "bar".into()))
        );
    }

    #[test]
    fn conditional_assignment() {
        assert_eq!(
            parts("FOO ?= bar"),
            Some(("FOO".into(), Flavor::Recursive, true, "bar".into()))
        );
    }

    #[test]
    fn append_assignment() {
        assert_eq!(
            parts("FOO += bar"),
            Some(("FOO".into(), Flavor::Append, false, "bar".into()))
        );
    }

    #[test]
    fn shell_assignment() {
        assert_eq!(
            parts("FOO != echo bar"),
            Some(("FOO".into(), Flavor::Shell, false, "echo bar".into()))
        );
    }

    #[test]
    fn no_whitespace_around_operator() {
        assert_eq!(
            parts("FOO:=bar"),
            Some(("FOO".into(), Flavor::Simple, false, "bar".into()))
        );
    }

    #[test]
    fn name_with_variable_reference() {
        // The `$(X)` in the name is skipped as a single unit, not treated as a
        // word break, so this remains one definition.
        let a = parse("pre$(X)post = v").unwrap();
        assert_eq!(a.flavor, Flavor::Recursive);
        assert_eq!(&"pre$(X)post = v".as_bytes()[a.name()], b"pre$(X)post");
    }

    #[test]
    fn comment_is_not_a_definition() {
        assert_eq!(parse("# just a comment"), None);
    }

    #[test]
    fn empty_line_is_not_a_definition() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("   "), None);
    }

    #[test]
    fn bare_word_is_not_a_definition() {
        assert_eq!(parse("target"), None);
    }

    #[test]
    fn two_words_before_operator_is_not_a_definition() {
        // A space-separated second word before any operator is a rule/other
        // construct, not an assignment.
        assert_eq!(parse("FOO BAR = baz"), None);
    }

    #[test]
    fn empty_value_after_operator() {
        assert_eq!(
            parts("FOO ="),
            Some(("FOO".into(), Flavor::Recursive, false, "".into()))
        );
    }

    #[test]
    fn salsa_query_matches_pure_parser() {
        ensure_map();
        let db = crate::makedb::MakeDb::default();
        for line in ["A = 1", "B := 2", "C ?= 3", "not a def", "# x"] {
            assert_eq!(
                assignment_ast(&db, line.as_bytes()),
                parse_assignment(line.as_bytes()),
                "salsa query and pure parser disagree on {line:?}"
            );
        }
    }

    #[test]
    fn conditional_directives_classify() {
        assert_eq!(Directive::from_word(b"ifdef"), Some(Directive::Ifdef));
        assert_eq!(Directive::from_word(b"ifndef"), Some(Directive::Ifndef));
        assert_eq!(Directive::from_word(b"ifeq"), Some(Directive::Ifeq));
        assert_eq!(Directive::from_word(b"ifneq"), Some(Directive::Ifneq));
        assert_eq!(Directive::from_word(b"else"), Some(Directive::Else));
        assert_eq!(Directive::from_word(b"endif"), Some(Directive::Endif));
    }

    #[test]
    fn non_directives_are_rejected() {
        // Matching is exact: prefixes, suffixes, and unrelated words are not
        // directives.
        for w in [
            &b""[..],
            b"if",
            b"ifdefx",
            b"xifdef",
            b"IFDEF",
            b"endi",
            b"endiff",
            b"target",
        ] {
            assert_eq!(Directive::from_word(w), None, "{:?} must not classify", w);
        }
    }

    #[test]
    fn directive_names_are_the_keywords() {
        assert_eq!(Directive::Ifdef.name().to_bytes(), b"ifdef");
        assert_eq!(Directive::Ifndef.name().to_bytes(), b"ifndef");
        assert_eq!(Directive::Ifeq.name().to_bytes(), b"ifeq");
        assert_eq!(Directive::Ifneq.name().to_bytes(), b"ifneq");
        assert_eq!(Directive::Else.name().to_bytes(), b"else");
        assert_eq!(Directive::Endif.name().to_bytes(), b"endif");
    }

    #[test]
    fn var_modifiers_classify() {
        assert_eq!(VarModifier::from_word(b"export"), Some(VarModifier::Export));
        assert_eq!(
            VarModifier::from_word(b"unexport"),
            Some(VarModifier::Unexport)
        );
        assert_eq!(
            VarModifier::from_word(b"override"),
            Some(VarModifier::Override)
        );
        assert_eq!(
            VarModifier::from_word(b"private"),
            Some(VarModifier::Private)
        );
        assert_eq!(VarModifier::from_word(b"define"), Some(VarModifier::Define));
        assert_eq!(
            VarModifier::from_word(b"undefine"),
            Some(VarModifier::Undefine)
        );
    }

    #[test]
    fn non_modifiers_are_rejected() {
        for w in [
            &b""[..],
            b"exp",
            b"exports",
            b"EXPORT",
            b"definex",
            b"ifdef",
            b"FOO",
        ] {
            assert_eq!(VarModifier::from_word(w), None, "{:?} must not classify", w);
        }
    }

    #[test]
    fn file_directives_classify() {
        assert_eq!(
            FileDirective::from_word(b"vpath"),
            Some(FileDirective::Vpath)
        );
        assert_eq!(
            FileDirective::from_word(b"include"),
            Some(FileDirective::Include)
        );
        // `-include` and `sinclude` are the same error-tolerant include.
        assert_eq!(
            FileDirective::from_word(b"-include"),
            Some(FileDirective::IncludeOpt)
        );
        assert_eq!(
            FileDirective::from_word(b"sinclude"),
            Some(FileDirective::IncludeOpt)
        );
        assert_eq!(FileDirective::from_word(b"load"), Some(FileDirective::Load));
        assert_eq!(
            FileDirective::from_word(b"-load"),
            Some(FileDirective::LoadOpt)
        );
    }

    #[test]
    fn non_file_directives_are_rejected() {
        for w in [
            &b""[..],
            b"vpaths",
            b"includ",
            b"includes",
            b"VPATH",
            b"loaded",
            b"export",
            b"FOO",
        ] {
            assert_eq!(
                FileDirective::from_word(w),
                None,
                "{:?} must not classify",
                w
            );
        }
    }

    #[test]
    fn define_keywords_classify() {
        assert_eq!(
            DefineKeyword::from_word(b"define"),
            Some(DefineKeyword::Define)
        );
        assert_eq!(
            DefineKeyword::from_word(b"endef"),
            Some(DefineKeyword::Endef)
        );
    }

    #[test]
    fn non_define_keywords_are_rejected() {
        // Prefixes, suffixes, case variants, and the trailing-NUL form (the
        // slice excludes the C string's NUL) must all fail to classify.
        for w in [
            &b""[..],
            b"defin",
            b"defines",
            b"Define",
            b"DEFINE",
            b"endefs",
            b"ende",
            b"Endef",
            b"define\0",
            b"undefine",
            b"export",
        ] {
            assert_eq!(
                DefineKeyword::from_word(w),
                None,
                "{:?} must not classify",
                w
            );
        }
    }

    #[test]
    fn line_kind_classifies_leading_byte() {
        // With the default tab prefix: NUL is blank, a leading tab is a recipe,
        // anything else parses further.
        assert_eq!(LineKind::classify(0, b'\t'), LineKind::Blank);
        assert_eq!(LineKind::classify(b'\t', b'\t'), LineKind::Recipe);
        assert_eq!(LineKind::classify(b'a', b'\t'), LineKind::Other);
        assert_eq!(LineKind::classify(b' ', b'\t'), LineKind::Other);
        assert_eq!(LineKind::classify(b'#', b'\t'), LineKind::Other);
    }

    #[test]
    fn line_kind_honors_recipe_prefix_override() {
        // `.RECIPEPREFIX` can change the recipe character: with `>` as the
        // prefix, a leading `>` is a recipe and a leading tab is just `Other`.
        assert_eq!(LineKind::classify(b'>', b'>'), LineKind::Recipe);
        assert_eq!(LineKind::classify(b'\t', b'>'), LineKind::Other);
        // The empty-line test still wins regardless of the prefix.
        assert_eq!(LineKind::classify(0, b'>'), LineKind::Blank);
    }

    #[test]
    fn special_targets_classify() {
        assert_eq!(
            SpecialTarget::from_name(b".POSIX"),
            Some(SpecialTarget::Posix)
        );
        assert_eq!(
            SpecialTarget::from_name(b".SECONDEXPANSION"),
            Some(SpecialTarget::SecondExpansion)
        );
        assert_eq!(
            SpecialTarget::from_name(b".ONESHELL"),
            Some(SpecialTarget::OneShell)
        );
    }

    #[test]
    fn log_functions_classify() {
        assert_eq!(
            LogFunction::from_funcname(b"error"),
            Some(LogFunction::Error)
        );
        assert_eq!(
            LogFunction::from_funcname(b"warning"),
            Some(LogFunction::Warning)
        );
        assert_eq!(LogFunction::from_funcname(b"info"), Some(LogFunction::Info));
    }

    #[test]
    fn log_functions_reject_non_matches() {
        // Other built-in functions sharing a first byte must not classify
        // (the old wall switched on the first byte alone).
        for w in [
            b"eval".as_slice(),
            b"if".as_slice(),
            b"wildcard".as_slice(),
            b"intcmp".as_slice(),
            b"Error".as_slice(),
            b"errors".as_slice(),
            b"".as_slice(),
        ] {
            assert_eq!(
                LogFunction::from_funcname(w),
                None,
                "{:?} must not classify",
                w
            );
        }
    }

    #[test]
    fn notdir_suffix_classify() {
        assert_eq!(
            NotdirSuffix::from_funcname(b"notdir"),
            Some(NotdirSuffix::Notdir)
        );
        assert_eq!(
            NotdirSuffix::from_funcname(b"suffix"),
            Some(NotdirSuffix::Suffix)
        );
    }

    #[test]
    fn notdir_suffix_reject_non_matches() {
        // Other built-in functions sharing a first byte ('s' for suffix,
        // 'n' for notdir) must not classify — the old wall switched on the
        // first byte alone (`*funcname == 's'`).
        for w in [
            b"subst".as_slice(),
            b"sort".as_slice(),
            b"strip".as_slice(),
            b"shell".as_slice(),
            b"basename".as_slice(),
            b"dir".as_slice(),
            b"Suffix".as_slice(),
            b"notdirs".as_slice(),
            b"".as_slice(),
        ] {
            assert_eq!(
                NotdirSuffix::from_funcname(w),
                None,
                "{:?} must not classify",
                w
            );
        }
    }

    #[test]
    fn basename_dir_classify() {
        assert_eq!(
            BasenameDir::from_funcname(b"basename"),
            Some(BasenameDir::Basename)
        );
        assert_eq!(BasenameDir::from_funcname(b"dir"), Some(BasenameDir::Dir));
    }

    #[test]
    fn basename_dir_reject_non_matches() {
        // Other built-in functions sharing a first byte ('b' for basename,
        // 'd' for dir) must not classify — the old wall switched on the first
        // byte alone (`*funcname == 'b'`).
        for w in [
            b"basenam".as_slice(),
            b"basenames".as_slice(),
            b"Basename".as_slice(),
            b"d".as_slice(),
            b"directory".as_slice(),
            b"notdir".as_slice(),
            b"suffix".as_slice(),
            b"".as_slice(),
        ] {
            assert_eq!(
                BasenameDir::from_funcname(w),
                None,
                "{:?} must not classify",
                w
            );
        }
    }

    #[test]
    fn addprefix_addsuffix_classify() {
        assert_eq!(
            AddprefixAddsuffix::from_funcname(b"addprefix"),
            Some(AddprefixAddsuffix::Addprefix)
        );
        assert_eq!(
            AddprefixAddsuffix::from_funcname(b"addsuffix"),
            Some(AddprefixAddsuffix::Addsuffix)
        );
    }

    #[test]
    fn addprefix_addsuffix_reject_non_matches() {
        // The old wall switched on the fourth byte alone (`funcname[3] == 'p'`),
        // so any name with 'p'/'s' there would have been miscategorised; the
        // typed classifier matches the full name instead.
        for w in [
            b"addprefi".as_slice(),
            b"addprefixes".as_slice(),
            b"addp".as_slice(),
            b"addsuffi".as_slice(),
            b"Addprefix".as_slice(),
            b"subst".as_slice(),
            b"".as_slice(),
        ] {
            assert_eq!(
                AddprefixAddsuffix::from_funcname(w),
                None,
                "{:?} must not classify",
                w
            );
        }
    }

    fn scan(s: &str) -> VarModScan {
        ensure_map();
        scan_var_modifiers(s.as_bytes(), false)
    }

    #[test]
    fn var_modifiers_plain_assignment() {
        // No modifiers; the whole line is a definition.
        let r = scan("FOO = 1");
        assert_eq!(r.mods, VarModifiers::default());
        assert!(!r.had_modifier);
        assert!(r.assign);
        assert_eq!(r.rest, 0);
    }

    #[test]
    fn var_modifiers_export_is_a_name_when_assigned() {
        // `export = 1` assigns the variable named `export`; it is not a modifier.
        let r = scan("export = 1");
        assert_eq!(r.mods.export, None);
        assert!(!r.had_modifier);
        assert!(r.assign);
        assert_eq!(r.rest, 0);
    }

    #[test]
    fn var_modifiers_single_and_stacked() {
        let r = scan("export FOO = 1");
        assert_eq!(r.mods.export, Some(ExportMode::Export));
        assert!(r.had_modifier);
        assert!(r.assign);
        assert_eq!(r.rest, "export ".len());

        let r = scan("override private BAR := 2");
        assert!(r.mods.over && r.mods.private);
        assert!(r.assign);
        assert_eq!(r.rest, "override private ".len());
    }

    #[test]
    fn var_modifiers_define_and_undefine() {
        let r = scan("define X");
        assert!(r.mods.define && r.assign && r.had_modifier);
        assert_eq!(r.rest, "define ".len());

        let r = scan("undefine Y");
        assert!(r.mods.undefine && r.assign);
        assert_eq!(r.rest, "undefine ".len());
    }

    #[test]
    fn var_modifiers_define_is_plain_name_in_targetvar() {
        // In a target-specific context, `define`/`undefine` are plain names.
        let r = scan_var_modifiers(b"define X", true);
        assert!(!r.mods.define);
        assert!(!r.assign);
        assert_eq!(r.rest, 0);
    }

    #[test]
    fn var_modifiers_rule_line_is_not_assignment() {
        // A modifier followed by a non-definition (a rule) keeps the flag but is
        // not an assignment, and rewinds to the line start.
        let r = scan("override foo: bar");
        assert!(r.mods.over);
        assert!(!r.assign);
        assert_eq!(r.rest, 0);
    }

    fn classify(s: &str) -> LineClass {
        ensure_map();
        classify_line(&crate::makedb::MakeDb::default(), s.as_bytes(), false)
    }

    #[test]
    fn classify_conditional_directive() {
        assert_eq!(
            classify("ifdef FOO"),
            LineClass::Conditional(Directive::Ifdef)
        );
        assert_eq!(
            classify("ifeq (a,b)"),
            LineClass::Conditional(Directive::Ifeq)
        );
        assert_eq!(classify("endif"), LineClass::Conditional(Directive::Endif));
        assert_eq!(
            classify("  else  "),
            LineClass::Conditional(Directive::Else)
        );
    }

    #[test]
    fn classify_file_directives() {
        assert_eq!(
            classify("vpath %.c src"),
            LineClass::File(FileDirective::Vpath)
        );
        assert_eq!(
            classify("include foo.mk"),
            LineClass::File(FileDirective::Include)
        );
        assert_eq!(
            classify("-include foo.mk"),
            LineClass::File(FileDirective::IncludeOpt)
        );
        assert_eq!(
            classify("sinclude foo.mk"),
            LineClass::File(FileDirective::IncludeOpt)
        );
        assert_eq!(
            classify("load plugin.so"),
            LineClass::File(FileDirective::Load)
        );
        assert_eq!(
            classify("-load plugin.so"),
            LineClass::File(FileDirective::LoadOpt)
        );
    }

    #[test]
    fn classify_multiword_keyword_is_file_directive() {
        // `include FOO = 1` is an include of the file `FOO = 1`: the embedded
        // blank stops it from parsing as an assignment, so it falls through to
        // the file-directive arm — exactly make's behavior.
        assert_eq!(
            classify("include FOO = 1"),
            LineClass::File(FileDirective::Include)
        );
    }

    #[test]
    fn classify_assignment_beats_keyword() {
        // make's `eval` probes for an assignment before the directive dispatch,
        // so a keyword used as a bare variable name is an assignment, not a
        // directive (`include = x`, `vpath = x`, `ifdef = x`).
        for line in ["include = x", "vpath = x", "ifdef = x"] {
            match classify(line) {
                LineClass::VarDef(vl) => {
                    assert!(
                        vl.assign && !vl.had_modifier,
                        "{line:?} should be a plain assignment"
                    )
                }
                other => panic!("{line:?} expected VarDef, got {other:?}"),
            }
        }
    }

    #[test]
    fn classify_var_definitions() {
        match classify("FOO = 1") {
            LineClass::VarDef(vl) => {
                assert_eq!(vl.mods, VarModifiers::default());
                assert!(vl.assign && !vl.had_modifier);
                assert_eq!(vl.rest, 0);
            }
            other => panic!("expected VarDef, got {other:?}"),
        }
        match classify("export FOO = 1") {
            LineClass::VarDef(vl) => {
                assert_eq!(vl.mods.export, Some(ExportMode::Export));
                assert!(vl.assign && vl.had_modifier);
                assert_eq!(vl.rest, "export ".len());
            }
            other => panic!("expected VarDef, got {other:?}"),
        }
        // `export = 1` assigns the variable named `export`; still a VarDef, but
        // with no export modifier (the bare-word export arm in `eval` is what
        // treats `export` as a directive — `classify_line` leaves it to the scan).
        match classify("export = 1") {
            LineClass::VarDef(vl) => {
                assert_eq!(vl.mods.export, None);
                assert!(vl.assign && !vl.had_modifier);
            }
            other => panic!("expected VarDef, got {other:?}"),
        }
    }

    /// Parse a conditional argument line and return the two argument substrings
    /// plus the trailing-text flag, or `None` for a syntax error.
    /// Parse `s` and, for a [`ConditionalArgs::Both`] result, return the two
    /// argument substrings and the trailing-text flag; `None` for the other
    /// (error) variants.
    fn cargs(s: &str) -> Option<(String, String, bool)> {
        ensure_map();
        let b = s.as_bytes();
        match parse_conditional_args(b) {
            ConditionalArgs::Both {
                arg1,
                arg2,
                trailing_text,
            } => {
                Some((
                    String::from_utf8(b[arg1].to_vec()).unwrap(),
                    String::from_utf8(b[arg2].to_vec()).unwrap(),
                    trailing_text,
                ))
            }
            _ => None,
        }
    }

    #[test]
    fn conditional_paren_form() {
        assert_eq!(cargs("(a,b)"), Some(("a".into(), "b".into(), false)));
        // Only the first argument's *trailing* blanks (before the comma) are
        // trimmed — its leading blank is kept; the second argument drops leading
        // blanks but keeps trailing ones (mirroring make's exact scan).
        assert_eq!(cargs("( a , b )"), Some((" a".into(), "b ".into(), false)));
        assert_eq!(cargs("(,)"), Some(("".into(), "".into(), false)));
    }

    #[test]
    fn conditional_quoted_forms() {
        assert_eq!(cargs("\"a\" \"b\""), Some(("a".into(), "b".into(), false)));
        assert_eq!(cargs("'a' 'b'"), Some(("a".into(), "b".into(), false)));
        // A quoted argument keeps its inner blanks verbatim.
        assert_eq!(
            cargs("\"a b\" \"c\""),
            Some(("a b".into(), "c".into(), false))
        );
    }

    #[test]
    fn conditional_references_and_balanced_parens() {
        // A `$(...)` reference inside an argument is skipped whole, so a comma
        // or paren inside it does not terminate the argument.
        assert_eq!(
            cargs("($(x,y),b)"),
            Some(("$(x,y)".into(), "b".into(), false))
        );
        // The second argument balances parentheses up to the matching close.
        assert_eq!(cargs("(a,b(c))"), Some(("a".into(), "b(c)".into(), false)));
    }

    #[test]
    fn conditional_trailing_text() {
        assert_eq!(cargs("(a,b) x"), Some(("a".into(), "b".into(), true)));
        assert_eq!(
            cargs("\"a\" \"b\"  junk"),
            Some(("a".into(), "b".into(), true))
        );
    }

    #[test]
    fn rest_is_blank_cases() {
        ensure_map();
        assert!(rest_is_blank(b""));
        assert!(rest_is_blank(b"   "));
        assert!(rest_is_blank(b" \t "));
        assert!(!rest_is_blank(b"x"));
        assert!(!rest_is_blank(b"   x"));
    }

    #[test]
    fn define_keyword_cases() {
        ensure_map();
        assert_eq!(
            define_keyword(b"define X"),
            (Some(DefineKeyword::Define), 6)
        );
        assert_eq!(define_keyword(b"endef"), (Some(DefineKeyword::Endef), 5));
        assert_eq!(
            define_keyword(b"endef # c"),
            (Some(DefineKeyword::Endef), 5)
        );
        // Not a define keyword (word still measured up to the blank/NUL).
        assert_eq!(define_keyword(b"FOO = 1"), (None, 3));
        assert_eq!(define_keyword(b"definex"), (None, 7));
    }

    #[test]
    fn closes_ignored_define_cases() {
        ensure_map();
        let c = |s: &str| closes_ignored_define(s.as_bytes());
        assert!(c("endef"));
        assert!(c("endef   "));
        assert!(c("endef # comment"));
        // Not a bare endef.
        assert!(!c("endef x"));
        assert!(!c("define X"));
        assert!(!c("endefx"));
        assert!(!c(""));
    }

    #[test]
    fn rule_probe_cases() {
        ensure_map();
        let rp = |s: &str| rule_probe(s.as_bytes());
        // The separator must follow the first token's trailing blanks: a `:`
        // (or `&:` / `|:`) after at least one blank. (make's `end_of_token`
        // stops only at whitespace/NUL, so a colon attached to the token —
        // `all:` — is part of the word and is *not* detected by this probe; the
        // real rule parse handles that case later.)
        assert_eq!(rp("all : dep").is_rule, true);
        assert_eq!(rp("all &: x").is_rule, true);
        assert_eq!(rp("all |: x").is_rule, true);
        // Not detected here.
        assert_eq!(rp("all: dep").is_rule, false);
        assert_eq!(rp("FOO = 1").is_rule, false);
        assert_eq!(rp("all").is_rule, false);
        assert_eq!(rp("a b &: x").is_rule, false);
        // Word length and rest offset (blanks after the token skipped).
        let r = rp("all : x");
        assert_eq!(r.word_len, 3);
        assert_eq!(r.rest, 4); // the ':'
        assert!(r.is_rule);
    }

    #[test]
    fn trimmed_token_cases() {
        ensure_map();
        fn tt(s: &str) -> Option<String> {
            let r = trimmed_token(s.as_bytes())?;
            Some(String::from_utf8(s.as_bytes()[r].to_vec()).unwrap())
        }
        assert_eq!(tt("FOO").as_deref(), Some("FOO"));
        assert_eq!(tt("  FOO  ").as_deref(), Some("FOO"));
        // A second word is kept (the name is everything up to the trailing
        // blanks); only leading and trailing blanks are stripped.
        assert_eq!(tt("FOO BAR").as_deref(), Some("FOO BAR"));
        // Empty or all-whitespace is "empty variable name" (None → fatal).
        assert_eq!(tt(""), None);
        assert_eq!(tt("   "), None);
    }

    #[test]
    fn lone_token_cases() {
        ensure_map();
        let lone = |s: &str| lone_token(s.as_bytes());
        // A single token, with or without trailing blanks, yields its length.
        assert_eq!(lone("FOO"), Some(3));
        assert_eq!(lone("FOO   "), Some(3));
        assert_eq!(lone(""), Some(0));
        // A second token is a syntax error.
        assert_eq!(lone("FOO BAR"), None);
        assert_eq!(lone("FOO  BAR  "), None);
    }

    #[test]
    fn conditional_error_before_first_arg() {
        // A bad opener or an unterminated/incomplete first argument expands
        // nothing — make reports the syntax error immediately.
        ensure_map();
        for s in ["x a b", "(a)", "(a", ""] {
            assert_eq!(
                parse_conditional_args(s.as_bytes()),
                ConditionalArgs::Error,
                "{s:?} should be Error"
            );
        }
    }

    #[test]
    fn conditional_first_arg_only_on_second_arg_error() {
        // The first argument is well-formed but the second is malformed: make
        // still expands the first (for its side effects), so the parser reports
        // the first argument's range rather than a bare error.
        ensure_map();
        for (s, want_arg1) in [("(a,b", "a"), ("\"a\"", "a"), ("\"a\" b", "a")] {
            match parse_conditional_args(s.as_bytes()) {
                ConditionalArgs::FirstArgOnly { arg1 } => {
                    assert_eq!(&s.as_bytes()[arg1], want_arg1.as_bytes(), "{s:?}")
                }
                other => panic!("{s:?} expected FirstArgOnly, got {other:?}"),
            }
        }
    }

    #[test]
    fn classify_plain_lines() {
        assert_eq!(classify("foo: bar"), LineClass::Plain);
        assert_eq!(classify("\tnot reached here"), LineClass::Plain);
        assert_eq!(classify(""), LineClass::Plain);
    }

    #[test]
    fn classify_modifier_led_rule_is_vardef() {
        // A modifier followed by a rule keeps the modifier flag (so the reader
        // still emits make's TAB warning) but is not an assignment.
        match classify("override foo: bar") {
            LineClass::VarDef(vl) => {
                assert!(vl.mods.over && vl.had_modifier && !vl.assign);
            }
            other => panic!("expected VarDef, got {other:?}"),
        }
    }

    #[test]
    fn non_special_targets_are_rejected() {
        // Other targets (including other dotted specials handled elsewhere),
        // case variants, and the trailing-NUL form must all fail to classify.
        for w in [
            &b""[..],
            b".POSIXX",
            b".posix",
            b".POSIX\0",
            b".ONESHELLX",
            b".PHONY",
            b".SUFFIXES",
            b"all",
            b".",
        ] {
            assert_eq!(
                SpecialTarget::from_name(w),
                None,
                "{:?} must not classify",
                w
            );
        }
    }

    #[test]
    fn ifeq_ifneq_separator_classifier() {
        ensure_map();
        // ifeq/ifneq with no separating whitespace → specific diagnostic.
        assert!(ifeq_ifneq_without_separator(b"ifeq(a,b)"));
        assert!(ifeq_ifneq_without_separator(b"ifneq(a,b)"));
        // No trailing byte at all (token ends right after the keyword): the
        // missing byte is treated as a NUL, which is non-blank → triggers.
        assert!(ifeq_ifneq_without_separator(b"ifeq"));
        assert!(ifeq_ifneq_without_separator(b"ifneq"));
        // Followed by a blank → properly separated, so the generic diagnostic
        // applies instead.
        assert!(!ifeq_ifneq_without_separator(b"ifeq (a,b)"));
        assert!(!ifeq_ifneq_without_separator(b"ifneq (a,b)"));
        assert!(!ifeq_ifneq_without_separator(b"ifeq\t(a,b)"));
        // Other tokens never match.
        assert!(!ifeq_ifneq_without_separator(b"ifdef X"));
        assert!(!ifeq_ifneq_without_separator(b"include x"));
        assert!(!ifeq_ifneq_without_separator(b""));
        assert!(!ifeq_ifneq_without_separator(b"if"));
        // `ifeqfoo` / `ifneqbar` (longer words) still count as unseparated.
        assert!(ifeq_ifneq_without_separator(b"ifeqfoo"));
        assert!(ifeq_ifneq_without_separator(b"ifneqbar"));
    }

    #[test]
    fn eight_space_indent_classifier() {
        assert!(starts_with_eight_spaces(b"        @echo hi")); // 8 spaces
        assert!(starts_with_eight_spaces(b"        ")); // exactly 8
        assert!(!starts_with_eight_spaces(b"       x")); // 7 spaces
        assert!(!starts_with_eight_spaces(b"\t@echo hi")); // a tab
        assert!(!starts_with_eight_spaces(b"")); // empty
        assert!(!starts_with_eight_spaces(b"    ")); // 4 spaces, too short
    }

    #[test]
    fn find_char_unquote_collapses_backslashes() {
        // Helper: run the unquote over an owned NUL-terminated buffer and return
        // (result index, resulting C string up to the new NUL).
        fn run(s: &[u8], stop: u8) -> (Option<usize>, Vec<u8>) {
            let mut buf = s.to_vec();
            buf.push(0);
            let r = find_char_unquote_idx(&mut buf, stop);
            let end = buf.iter().position(|&b| b == 0).unwrap();
            (r, buf[..end].to_vec())
        }

        // No backslash: the stop is found as-is, buffer unchanged.
        assert_eq!(run(b"abc%def", b'%'), (Some(3), b"abc%def".to_vec()));
        // No stop at all.
        assert_eq!(run(b"abcdef", b'%'), (None, b"abcdef".to_vec()));
        // Single backslash escapes the `%`: it is consumed (one backslash
        // removed) and no *unescaped* `%` remains.
        assert_eq!(run(b"a\\%b", b'%'), (None, b"a%b".to_vec()));
        // Two backslashes -> one literal backslash, `%` is unescaped at index 2.
        assert_eq!(run(b"a\\\\%b", b'%'), (Some(2), b"a\\%b".to_vec()));
        // Three backslashes -> one literal backslash then an escaped `%`: the
        // `%` is consumed, none remains; buffer keeps a single backslash + `%`.
        assert_eq!(run(b"a\\\\\\%b", b'%'), (None, b"a\\%b".to_vec()));
        // Backslash that does not precede the stop is left untouched.
        assert_eq!(run(b"a\\b%c", b'%'), (Some(3), b"a\\b%c".to_vec()));
    }

    #[test]
    fn find_percent_cached_collapses_only_when_escaped() {
        use super::FindPercentCached::*;

        // Reduce a result to (index, collapsed C string up to the NUL); the
        // `Collapsed` buffer may retain stale bytes past the NUL after the
        // in-place collapse, so compare only the live C string.
        fn norm(r: FindPercentCached) -> (Option<usize>, Option<Vec<u8>>) {
            match r {
                AsIs(idx) => (idx, None),
                Collapsed { buf, idx } => {
                    let end = buf.iter().position(|&b| b == 0).unwrap();
                    (idx, Some(buf[..end].to_vec()))
                }
            }
        }

        // No-copy cases: the `%` (if any) is returned as-is at its index.
        assert_eq!(norm(find_percent_cached(b"a%b")), (Some(1), None));
        assert_eq!(norm(find_percent_cached(b"%abc")), (Some(0), None));
        assert_eq!(norm(find_percent_cached(b"abc")), (None, None));
        assert_eq!(norm(find_percent_cached(b"ab%c")), (Some(2), None));

        // Escaped `%` (single backslash): collapsed, backslash removed, no
        // unquoted `%` remains.
        assert_eq!(
            norm(find_percent_cached(b"a\\%b")),
            (None, Some(b"a%b".to_vec()))
        );
        // Double backslash: one literal backslash kept, `%` unescaped at index 2.
        assert_eq!(
            norm(find_percent_cached(b"a\\\\%b")),
            (Some(2), Some(b"a\\%b".to_vec()))
        );
        // A later `%` becomes the unquoted match after the escaped one collapses.
        assert_eq!(
            norm(find_percent_cached(b"a\\%b%c")),
            (Some(3), Some(b"a%b%c".to_vec()))
        );
    }

    #[test]
    fn get_next_mword_classifies_and_spans() {
        use super::MWordType::*;
        // (type, start, len) for the first word.
        // End-of-line spans the NUL terminator (length 1), matching the C scan.
        assert_eq!(get_next_mword(b""), (Eol, 0, 1));
        assert_eq!(get_next_mword(b"   "), (Eol, 3, 1));
        // Leading blanks skipped; a plain word spans to the next separator.
        assert_eq!(get_next_mword(b"  foo bar"), (Static, 2, 3));
        assert_eq!(get_next_mword(b";rest"), (Semicolon, 0, 1));
        assert_eq!(get_next_mword(b":x"), (Colon, 0, 1));
        assert_eq!(get_next_mword(b"::x"), (DColon, 0, 2));
        assert_eq!(get_next_mword(b"&:x"), (AmpColon, 0, 2));
        assert_eq!(get_next_mword(b"&::x"), (AmpDColon, 0, 3));
        // A bare `&` (not `&:`) is just a static word.
        assert_eq!(get_next_mword(b"&x y"), (Static, 0, 2));
        // A `$(...)` reference promotes the word to Variable and is spanned whole.
        assert_eq!(get_next_mword(b"a$(V)b c"), (Variable, 0, 6));
        // `\:` is escaped inside a static word, so the colon does not end it.
        assert_eq!(get_next_mword(b"a\\:b:c"), (Static, 0, 4));
        // A word ending exactly at end-of-string.
        assert_eq!(get_next_mword(b"word"), (Static, 0, 4));
    }

    #[test]
    fn find_map_unquote_skips_refs_and_collapses() {
        use crate::entry::{MAP_SEMI, MAP_VARIABLE};
        ensure_map();

        // (result index, resulting C string up to the new NUL).
        fn run(s: &[u8], stopmap: i32) -> (Option<usize>, Vec<u8>) {
            let mut buf = s.to_vec();
            buf.push(0);
            let r = find_map_unquote_idx(&mut buf, stopmap);
            let end = buf.iter().position(|&b| b == 0).unwrap();
            (r, buf[..end].to_vec())
        }

        // Plain stop byte (`;`) found as-is.
        assert_eq!(run(b"abc;def", MAP_SEMI), (Some(3), b"abc;def".to_vec()));
        // No stop byte present.
        assert_eq!(run(b"abcdef", MAP_SEMI), (None, b"abcdef".to_vec()));
        // A `;` inside a `$(...)` reference is skipped; the later `;` matches.
        assert_eq!(
            run(b"a$(x;y)b;c", MAP_SEMI | MAP_VARIABLE),
            (Some(8), b"a$(x;y)b;c".to_vec())
        );
        // `\;` escapes the stop byte: collapsed, no unescaped `;` remains.
        assert_eq!(run(b"a\\;b", MAP_SEMI), (None, b"a;b".to_vec()));
        // `\\;` is an even run: `;` is unescaped at index 2 after collapsing.
        assert_eq!(run(b"a\\\\;b", MAP_SEMI), (Some(2), b"a\\;b".to_vec()));
    }

    #[test]
    fn unescape_char_removes_one_escape_level() {
        let u = |s: &[u8]| unescape_char(s, b':');

        // No backslashes: unchanged.
        assert_eq!(u(b"a:b"), b"a:b".to_vec());
        // Single backslash before `:` escapes it; the `\` is dropped.
        assert_eq!(u(b"a\\:b"), b"a:b".to_vec());
        // Two backslashes = even run: `:` is not escaped, both `\` kept.
        assert_eq!(u(b"a\\\\:b"), b"a\\\\:b".to_vec());
        // Three backslashes (odd) before `:`: collapse to one, keep `:`.
        assert_eq!(u(b"a\\\\\\:b"), b"a\\:b".to_vec());
        // Backslashes before a non-`:` byte are copied verbatim.
        assert_eq!(u(b"a\\\\x"), b"a\\\\x".to_vec());
        assert_eq!(u(b"a\\xb"), b"a\\xb".to_vec());
        // Trailing backslash run not followed by `:` is preserved.
        assert_eq!(u(b"abc\\"), b"abc\\".to_vec());
        assert_eq!(u(b"abc\\\\"), b"abc\\\\".to_vec());
        // Only `:` escapes are touched; other chars' backslashes stay.
        assert_eq!(u(b"\\:\\;"), b":\\;".to_vec());
    }

    #[test]
    fn second_expansion_prereq_classifier() {
        assert!(prereq_needs_second_expansion(b"$$(VAR)"));
        assert!(prereq_needs_second_expansion(b"a $(x) b"));
        assert!(prereq_needs_second_expansion(b"$"));
        assert!(!prereq_needs_second_expansion(b"plain dep1 dep2"));
        assert!(!prereq_needs_second_expansion(b""));
    }

    #[test]
    fn wait_token_classifier() {
        assert!(is_wait_token(b".WAIT"));
        assert!(!is_wait_token(b".wait"));
        assert!(!is_wait_token(b".WAITX"));
        assert!(!is_wait_token(b".WAI"));
        assert!(!is_wait_token(b""));
    }

    #[test]
    fn dot_slash_prefix_stripping() {
        // No prefix to strip.
        assert_eq!(strip_dot_slash_prefix(b"foo"), 0);
        assert_eq!(strip_dot_slash_prefix(b"./"), 0); // <= 2 bytes: untouched
        assert_eq!(strip_dot_slash_prefix(b".x"), 0);
        // A single `./` before a longer name.
        assert_eq!(strip_dot_slash_prefix(b"./foo"), 2);
        // Repeated `./` pairs.
        assert_eq!(strip_dot_slash_prefix(b"././foo"), 4);
        // `./` followed by a run of extra slashes.
        assert_eq!(strip_dot_slash_prefix(b".//foo"), 3);
        assert_eq!(strip_dot_slash_prefix(b".///foo"), 4);
        // `./a` is 3 bytes, so the first `> 2` check passes and the pair is
        // stripped; the next iteration sees only 1 byte left and stops.
        assert_eq!(strip_dot_slash_prefix(b"./a"), 2);
        // `./` alone (2 bytes) fails the `> 2` guard immediately.
        assert_eq!(strip_dot_slash_prefix(b"./"), 0);
        // A leading slash that is not part of `./` is left alone.
        assert_eq!(strip_dot_slash_prefix(b"/foo"), 0);
    }
}
