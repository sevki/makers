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
//! (via [`crate::make_main::stopchar_map`]) rather than re-deriving `isspace`,
//! so the AST agrees with the C reader byte-for-byte, locale and all.

use std::ops::Range;
use std::sync::{Mutex, OnceLock};

use crate::make_main::{stopchar_map, MAP_BLANK, MAP_COMMENT, MAP_NEWLINE, MAP_NUL, MAP_VARSEP};
use crate::variable::{f_append, f_expand, f_recursive, f_shell, f_simple, variable_flavor};

/// The assignment operator's flavor, mirroring make's `variable_flavor`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, salsa::Update)]
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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, salsa::Update)]
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

// --- salsa front-end -------------------------------------------------------

#[salsa::db]
#[derive(Clone, Default)]
struct ParserDb {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for ParserDb {}

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

static DB: OnceLock<Mutex<ParserDb>> = OnceLock::new();

fn db() -> &'static Mutex<ParserDb> {
    DB.get_or_init(|| Mutex::new(ParserDb::default()))
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
pub fn assignment_ast(bytes: &[u8]) -> Option<Assignment> {
    let parsed = parse_assignment(bytes)?;
    let db = db().lock().unwrap_or_else(|e| e.into_inner());
    let node = AssignmentNode::new(
        &*db,
        parsed.name_start,
        parsed.name_len,
        parsed.flavor,
        parsed.conditional,
        parsed.op_end,
        parsed.value_start,
    );
    Some(Assignment {
        name_start: node.name_start(&*db),
        name_len: node.name_len(&*db),
        flavor: node.flavor(&*db),
        conditional: node.conditional(&*db),
        op_end: node.op_end(&*db),
        value_start: node.value_start(&*db),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::make_main::initialize_stopchar_map;
    use std::sync::Once;

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
        for line in ["A = 1", "B := 2", "C ?= 3", "not a def", "# x"] {
            assert_eq!(
                assignment_ast(line.as_bytes()),
                parse_assignment(line.as_bytes()),
                "salsa query and pure parser disagree on {line:?}"
            );
        }
    }
}
