//! Per-target (and pattern) variable types — the idiomatic replacement for the
//! c2rust `variable` record held in a target's `VariableSetList`.
//!
//! Split out of `file.rs` to keep that module focused on the file node. The
//! flavor/origin/export enums mirror the c2rust `f_*`/`o_*`/`v_*` discriminants
//! so the two representations round-trip. `file.rs` re-exports these names.

use crate::content_hash::ContentHash;

/// How a variable's value is expanded — the idiomatic form of the c2rust
/// `variable_flavor`. Discriminants match the `f_*` constants so the two
/// representations round-trip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ContentHash)]
pub enum VarFlavor {
    /// `f_bogus` — undefined/placeholder.
    #[default]
    Bogus = 0,
    /// `f_simple` — `:=` / `::=` (expanded once at definition).
    Simple = 1,
    /// `f_recursive` — `=` (expanded on each use).
    Recursive = 2,
    /// `f_expand` — `:::=` (expand-then-escape).
    Expand = 3,
    /// `f_append` — `+=`.
    Append = 4,
    /// `f_shell` — `!=`.
    Shell = 5,
    /// `f_append_value`.
    AppendValue = 6,
}

/// Where a variable came from — the idiomatic form of `variable_origin`.
/// Discriminants match the `o_*` constants and order by precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ContentHash)]
pub enum VarOrigin {
    /// `o_default` — make's built-in default.
    #[default]
    Default = 0,
    /// `o_env` — the environment.
    Environment = 1,
    /// `o_file` — a makefile.
    File = 2,
    /// `o_env_override` — environment, with `-e`.
    EnvOverride = 3,
    /// `o_command` — the command line.
    Command = 4,
    /// `o_override` — an `override` directive.
    Override = 5,
    /// `o_automatic` — an automatic variable (`$@`, `$<`, …).
    Automatic = 6,
    /// `o_invalid`.
    Invalid = 7,
}

/// A variable's export disposition — the idiomatic form of `variable_export`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ContentHash)]
pub enum VarExport {
    /// `v_default` — follow the global export rules.
    #[default]
    Default = 0,
    /// `v_export` — always export.
    Export = 1,
    /// `v_noexport` — never export.
    NoExport = 2,
    /// `v_ifset` — export only if set.
    IfSet = 3,
}

/// A per-target (or pattern) variable definition — the idiomatic replacement
/// for the c2rust `variable` record held in a target's `VariableSetList`. Name
/// and value are raw bytes (no `c_char`); the c2rust bitfield is split into
/// plain enums/bools.
#[derive(Debug, Clone, PartialEq, Eq, ContentHash)]
pub struct TargetVariable {
    pub name: Vec<u8>,
    pub value: Vec<u8>,
    /// Where the variable was defined (raw bytes; `None` if synthetic).
    pub defined_in: Option<Vec<u8>>,
    pub defined_lineno: u64,
    pub flavor: VarFlavor,
    pub origin: VarOrigin,
    pub export: VarExport,
    pub recursive: bool,
    pub append: bool,
    pub conditional: bool,
    pub per_target: bool,
    pub special: bool,
    pub exportable: bool,
    pub private_var: bool,
}
