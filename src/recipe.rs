//! A target's recipe — the idiomatic replacement for the c2rust `Commands`
//! record (`*mut Commands` on `File`).
//!
//! Split out of `file.rs` to keep that module focused on the file node. The
//! recipe text and its chopped per-line view ([`RecipeLine`]/[`RecipeLineFlags`])
//! carry no raw pointers and no `c_char`. `file.rs` re-exports these names.

bitflags::bitflags! {
    /// Per-line recipe modifiers — the idiomatic form of the c2rust
    /// `lines_flags` byte. Values match `COMMANDS_RECURSE`/`COMMANDS_SILENT`/
    /// `COMMANDS_NOERROR` so the two representations round-trip bit-for-bit.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct RecipeLineFlags: u8 {
        /// Line recurses into a sub-make (`+`, or it mentions `$(MAKE)`).
        const RECURSE = 1;
        /// Line is silent (`@`): not echoed before running.
        const SILENT = 2;
        /// Errors on this line are ignored (`-`).
        const NOERROR = 4;
    }
}

/// One logical recipe line: its (still-unexpanded) command text with the
/// leading `@`/`-`/`+` modifiers parsed off into [`RecipeLineFlags`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeLine {
    pub text: Vec<u8>,
    pub flags: RecipeLineFlags,
}

/// A target's recipe — the idiomatic replacement for the c2rust `Commands`
/// (`*mut Commands` on `File`). Holds the recipe text as written plus, once
/// `chop_commands` has run, the per-line view that unifies `command_lines`,
/// `lines_flags`, and `ncommand_lines`. No raw pointers, no `c_char`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    /// Source file the recipe was defined in (raw bytes; `None` if synthetic,
    /// the former null `fileinfo.filenm`).
    pub defined_in: Option<Vec<u8>>,
    /// 1-based line number of the recipe's definition (`fileinfo.lineno`).
    pub defined_lineno: u64,
    /// Recipe text as written — logical lines joined by `\n`, before variable
    /// expansion (the former `commands` C string).
    pub text: Vec<u8>,
    /// The chopped per-line view; empty until `chop_commands` populates it.
    pub lines: Vec<RecipeLine>,
    /// The recipe-line introducer in effect (`.RECIPEPREFIX`, default TAB).
    pub recipe_prefix: u8,
    /// Whether any line recurses into a sub-make — the `any_recurse` bit.
    pub any_recurse: bool,
}

impl Default for Recipe {
    fn default() -> Self {
        Recipe {
            defined_in: None,
            defined_lineno: 0,
            text: Vec::new(),
            lines: Vec::new(),
            // The default introducer is a literal TAB, as in GNU make.
            recipe_prefix: b'\t',
            any_recurse: false,
        }
    }
}
