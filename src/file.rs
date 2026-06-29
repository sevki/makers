use crate::content_hash::ContentHash;

pub use crate::ffi_types::{
    __clockid_t, __off64_t, __off_t, __suseconds_t, __syscall_slong_t, __time_t, clockid_t,
    intmax_t, size_t, time_t, uintmax_t,
};
use crate::misc::free_ns_chain;
use crate::misc::{copy_dep_chain, end_of_token, xcalloc, xmalloc, xrealloc, xstrdup};
use crate::stdio::FILE;
use crate::strcache::{strcache_add_len, strcache_iscached};
use c2rust_bitfields;
use libc::{__errno_location, abort, free, printf, putchar, puts, strchr, strcmp, strcpy, unlink};
use std::ffi::{CStr, CString};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> i32;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub use crate::sys_stat::timespec;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
/// A node in make's dependency graph: one target (or prerequisite) file.
#[derive(Copy, Clone)]
pub struct File {
    pub name: *const ::core::ffi::c_char,
    pub hname: *const ::core::ffi::c_char,
    pub vpath: *const ::core::ffi::c_char,
    pub deps: *mut Dep,
    pub cmds: *mut Commands,
    pub stem: *const ::core::ffi::c_char,
    pub also_make: *mut Dep,
    pub prev: *mut File,
    pub last: *mut File,
    pub renamed: *mut File,
    pub variables: *mut VariableSetList,
    pub pat_variables: *mut VariableSetList,
    pub parent: *mut File,
    pub double_colon: *mut File,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: ::core::ffi::c_uint,
    pub command_flags: i32,
    pub update_status: UpdateStatus,
    pub command_state: CommandState,
    pub builtin: bool,
    pub precious: bool,
    pub loaded: bool,
    pub unloaded: bool,
    pub low_resolution_time: bool,
    pub tried_implicit: bool,
    pub updating: bool,
    pub updated: bool,
    pub is_target: bool,
    pub cmd_target: bool,
    pub phony: bool,
    pub intermediate: bool,
    pub is_explicit: bool,
    pub secondary: bool,
    pub notintermediate: bool,
    pub dontcare: bool,
    pub ignore_vpath: bool,
    pub pat_searched: bool,
    pub no_diag: bool,
    pub was_shuffled: bool,
    pub snapped: bool,
    pub suffix: bool,
}
impl Default for File {
    fn default() -> Self {
        Self {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            hname: ::core::ptr::null::<::core::ffi::c_char>(),
            vpath: ::core::ptr::null::<::core::ffi::c_char>(),
            deps: ::core::ptr::null_mut::<Dep>(),
            cmds: ::core::ptr::null_mut::<Commands>(),
            stem: ::core::ptr::null::<::core::ffi::c_char>(),
            also_make: ::core::ptr::null_mut::<Dep>(),
            prev: ::core::ptr::null_mut::<File>(),
            last: ::core::ptr::null_mut::<File>(),
            renamed: ::core::ptr::null_mut::<File>(),
            variables: ::core::ptr::null_mut::<VariableSetList>(),
            pat_variables: ::core::ptr::null_mut::<VariableSetList>(),
            parent: ::core::ptr::null_mut::<File>(),
            double_colon: ::core::ptr::null_mut::<File>(),
            last_mtime: 0,
            mtime_before_update: 0,
            considered: 0,
            command_flags: 0,
            update_status: UpdateStatus::Success,
            command_state: CommandState::NotStarted,
            builtin: false,
            precious: false,
            loaded: false,
            unloaded: false,
            low_resolution_time: false,
            tried_implicit: false,
            updating: false,
            updated: false,
            is_target: false,
            cmd_target: false,
            phony: false,
            intermediate: false,
            is_explicit: false,
            secondary: false,
            notintermediate: false,
            dontcare: false,
            ignore_vpath: false,
            pat_searched: false,
            no_diag: false,
            was_shuffled: false,
            snapped: false,
            suffix: false,
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSetList {
    pub next: *mut VariableSetList,
    pub set: *mut VariableSet,
    pub next_is_parent: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSet {
    pub table: hash_table,
}
pub type hash_table = crate::hash::hash_table;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dep {
    pub next: *mut Dep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut File,
    pub shuf: *mut Dep,
    pub stem: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_uint,
    pub changed: bool,
    pub ignore_mtime: bool,
    pub staticpattern: bool,
    pub need_2nd_expansion: bool,
    pub ignore_automatic_vars: bool,
    pub is_explicit: bool,
    pub wait_here: bool,
}
impl Default for Dep {
    fn default() -> Self {
        Dep {
            next: ::core::ptr::null_mut(),
            name: ::core::ptr::null(),
            file: ::core::ptr::null_mut(),
            shuf: ::core::ptr::null_mut(),
            stem: ::core::ptr::null(),
            flags: 0,
            changed: false,
            ignore_mtime: false,
            staticpattern: false,
            need_2nd_expansion: false,
            ignore_automatic_vars: false,
            is_explicit: false,
            wait_here: false,
        }
    }
}

const HASH_SIZE: usize = 32;

/// Idiomatic Rust dep edge for the new dependency graph layer.
/// Replaces `Dep` once all FFI bodies have been migrated.
#[derive(Debug, Clone, ContentHash)]
pub struct DepNode {
    pub name: String,
    pub file: Option<FileId>,
    pub shuf: Option<DepId>,
    pub stem: Option<String>,
    pub flags: DepFlags,
    pub changed: bool,
    pub ignore_mtime: bool,
    pub static_pattern: bool,
    pub needs_second_expansion: bool,
    pub ignore_automatic_vars: bool,
    pub is_explicit: bool,
    pub wait_here: bool,
}

// Stable identity for a dep edge: content-hash of the full (immutable) `DepNode`.
crate::id_wireformat!(DepId[HASH_SIZE] <- DepNode);

// Stable identity for a file: derived from its canonical name only.
// Mutable runtime state (timestamps, flags, command state) does not
// contribute to the key, so a file's identity survives updates.
crate::id_wireformat!(FileId[HASH_SIZE] |f: String| f.as_str());

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DepFlags: u32 {
        const NONE = 0;
        // fill these from GNU make’s DEP_* flags
        // const SOME_FLAG = 1 << 0;
    }
}

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

/// How a variable's value is expanded — the idiomatic form of the c2rust
/// `variable_flavor`. Discriminants match the `f_*` constants so the two
/// representations round-trip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Idiomatic Rust file node for the new dependency graph layer — the file-side
/// counterpart of [`DepNode`]. Replaces the c2rust [`File`] once all FFI bodies
/// have been migrated: it lives in the [`FileId`]-keyed arena
/// (`ExecContext::filenodes`) as an `Arc<Mutex<FileNode>>` instead of a raw
/// `*mut file`, so the build graph shares nodes by handle ([`FileId`], `Copy`)
/// rather than by raw pointer.
///
/// Inter-file links (`renamed`/`parent`/`double_colon`) are `Option<FileId>`
/// into the same arena; prerequisites are owned [`DepNode`]s, the recipe is an
/// owned [`Recipe`], and per-target/pattern variables are owned
/// [`TargetVariable`]s. The struct is free of raw pointers and `c_char`.
#[derive(Debug, Clone)]
pub struct FileNode {
    /// Name as written in the makefile. Raw bytes, not `String`: file names are
    /// arbitrary OS bytes and need not be valid UTF-8 (use
    /// `String::from_utf8_lossy` only for display).
    pub name: Vec<u8>,
    /// Hash name: the canonical key this file is interned under. Raw bytes so
    /// that `id()` (`FileId::from_bytes(&hname)`) is byte-exact and always
    /// equals the key the node was interned under.
    pub hname: Vec<u8>,
    /// Resolved VPATH location, once found.
    pub vpath: Option<String>,
    /// Stem from an implicit-rule match (`%`).
    pub stem: Option<String>,
    /// Prerequisites of this target.
    pub deps: Vec<DepNode>,
    /// Sibling targets built by the same recipe (`also_make`).
    pub also_make: Vec<DepNode>,
    /// The target's recipe, if any — the idiomatic replacement for the c2rust
    /// `*mut Commands`. `None` is the former null `cmds`.
    pub recipe: Option<Recipe>,
    /// This target's own per-target variable definitions (the head set of the
    /// former `*mut VariableSetList variables`), keyed-by-name lookup left to
    /// the variable layer. Empty when the target defines none.
    pub variables: Vec<TargetVariable>,
    /// Pattern-specific variables applying to this target (the former
    /// `*mut VariableSetList pat_variables`).
    pub pat_variables: Vec<TargetVariable>,
    /// Whether this is a double-colon (`::`) target. The c2rust graph marked
    /// this by a non-null `double_colon` self-link; here it is an explicit flag
    /// so that a single arena node (keyed by name) represents the whole target.
    pub is_double_colon: bool,
    /// The additional double-colon (`::`) entries beyond this head, in order.
    /// The c2rust graph threaded these as a separate `prev`/`last`/`double_colon`
    /// linked list of `*mut file`; since every entry shares this file's name (so
    /// they cannot each be a distinct name-derived [`FileId`]), they live inline
    /// on the head instead. Each entry carries its own deps/recipe/state; entries
    /// never nest (their own `double_colon` stays empty).
    pub double_colon: Vec<FileNode>,
    /// The file this one was renamed to (`rehash_file`).
    pub renamed: Option<FileId>,
    /// Parent for an intermediate file produced by a chain of implicit rules.
    pub parent: Option<FileId>,
    /// Last-known modification time (packed make timestamp).
    pub last_mtime: u64,
    /// Modification time captured before an update began.
    pub mtime_before_update: u64,
    /// Recursion guard / "considered" generation counter.
    pub considered: u32,
    /// Command-line flags affecting this target's recipe.
    pub command_flags: i32,
    /// Result of the last update attempt.
    pub update_status: UpdateStatus,
    /// Where this target is in the build state machine.
    pub command_state: CommandState,
    pub builtin: bool,
    pub precious: bool,
    pub loaded: bool,
    pub unloaded: bool,
    pub low_resolution_time: bool,
    pub tried_implicit: bool,
    pub updating: bool,
    pub updated: bool,
    pub is_target: bool,
    pub cmd_target: bool,
    pub phony: bool,
    pub intermediate: bool,
    pub is_explicit: bool,
    pub secondary: bool,
    pub notintermediate: bool,
    pub dontcare: bool,
    pub ignore_vpath: bool,
    pub pat_searched: bool,
    pub no_diag: bool,
    pub was_shuffled: bool,
    pub snapped: bool,
    pub suffix: bool,
}

impl FileNode {
    /// Create a fresh node interned under `name` (its hash name starts equal to
    /// its name; `rehash_file` may later rekey it). All build state starts in
    /// the same "not yet looked at" position as `File::default`.
    pub fn new(name: Vec<u8>) -> Self {
        FileNode {
            hname: name.clone(),
            name,
            vpath: None,
            stem: None,
            deps: Vec::new(),
            also_make: Vec::new(),
            recipe: None,
            variables: Vec::new(),
            pat_variables: Vec::new(),
            is_double_colon: false,
            double_colon: Vec::new(),
            renamed: None,
            parent: None,
            last_mtime: 0,
            mtime_before_update: 0,
            considered: 0,
            command_flags: 0,
            update_status: UpdateStatus::Success,
            command_state: CommandState::NotStarted,
            builtin: false,
            precious: false,
            loaded: false,
            unloaded: false,
            low_resolution_time: false,
            tried_implicit: false,
            updating: false,
            updated: false,
            is_target: false,
            cmd_target: false,
            phony: false,
            intermediate: false,
            is_explicit: false,
            secondary: false,
            notintermediate: false,
            dontcare: false,
            ignore_vpath: false,
            pat_searched: false,
            no_diag: false,
            was_shuffled: false,
            snapped: false,
            suffix: false,
        }
    }

    /// This file's stable arena identity, derived from its current hash name
    /// (the key it is interned under — equal to `name` until `rehash` rekeys
    /// it). Byte-exact, so non-UTF-8 names stay distinct.
    pub fn id(&self) -> FileId {
        FileId::from_bytes(&self.hname)
    }
}
impl Default for GoalDep {
    fn default() -> Self {
        GoalDep {
            next: ::core::ptr::null_mut(),
            name: ::core::ptr::null(),
            file: ::core::ptr::null_mut(),
            shuf: ::core::ptr::null_mut(),
            stem: ::core::ptr::null(),
            flags: 0,
            changed: false,
            ignore_mtime: false,
            staticpattern: false,
            need_2nd_expansion: false,
            ignore_automatic_vars: false,
            is_explicit: false,
            wait_here: false,
            error: 0,
            floc: Floc {
                filenm: ::core::ptr::null(),
                lineno: 0,
                offset: 0,
            },
        }
    }
}

/// A goal: a top-level target make was asked to build, with error/location
/// tracking. Mirrors `Dep` (a goal is an edge from "the command line" to a
/// target) plus bookkeeping.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GoalDep {
    pub next: *mut GoalDep,
    pub name: *const ::core::ffi::c_char,
    pub file: *mut File,
    pub shuf: *mut GoalDep,
    pub stem: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_uint,
    pub changed: bool,
    pub ignore_mtime: bool,
    pub staticpattern: bool,
    pub need_2nd_expansion: bool,
    pub ignore_automatic_vars: bool,
    pub is_explicit: bool,
    pub wait_here: bool,
    pub error: ::core::ffi::c_int,
    pub floc: Floc,
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Commands {
    pub fileinfo: Floc,
    pub commands: *mut ::core::ffi::c_char,
    pub command_lines: *mut *mut ::core::ffi::c_char,
    pub lines_flags: *mut ::core::ffi::c_uchar,
    pub ncommand_lines: ::core::ffi::c_ushort,
    pub recipe_prefix: ::core::ffi::c_char,
    #[bitfield(name = "any_recurse", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
use crate::commands::{print_commands, set_file_variables};
use crate::expand::{
    expand_string_buf, expand_string_for_file, variable_buffer, variable_buffer_output,
};
use crate::floc::Floc;
use crate::function::patsubst_expand_pat;
use crate::make_main::{
    db_level, second_expansion, stopchar_map, with_options, MAP_DIRSEP,
};
use crate::output::{error, fatal, perror_with_name, FmtArg};
use crate::read::{find_percent, parse_file_seq};
use crate::variable::{
    initialize_file_variables, lookup_variable, lookup_variable_in_set, merge_variable_set_lists,
    print_file_variables, print_target_variables,
};

pub type variable_set_list = VariableSetList;
pub type variable_set = VariableSet;

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
/// Intrusive singly-linked name-chain node. `NameSeq`, `Dep`, and `GoalDep`
/// all share this shape; the trait lets `parse_file_seq` and the chain
/// helpers operate on any of them without layout punning.
pub trait SeqNode: NextLinked {
    /// Allocate a zeroed node on the C heap (chains are freed with `free`).
    unsafe fn alloc() -> *mut Self;
    unsafe fn name(this: *const Self) -> *const ::core::ffi::c_char;
    unsafe fn set_name(this: *mut Self, name: *const ::core::ffi::c_char);
    unsafe fn set_next(this: *mut Self, next: *mut Self);
    unsafe fn next_slot(this: *mut Self) -> *mut *mut Self;
    /// Record a `.WAIT` marker on this node (only meaningful for `Dep`).
    unsafe fn mark_wait(_this: *mut Self) {}
}

/// Anything that forms an intrusive singly-linked chain via a `next` field.
pub trait NextLinked: Sized {
    unsafe fn next(this: *const Self) -> *mut Self;
}

macro_rules! impl_seq_node {
    ($t:ty $(, $extra:tt)?) => {
        impl NextLinked for $t {
            unsafe fn next(this: *const Self) -> *mut Self {
                if this.is_null() {
                    return ::core::ptr::null_mut::<Self>();
                }
                (*this).next
            }
        }
        impl SeqNode for $t {
            unsafe fn alloc() -> *mut Self {
                xcalloc(::core::mem::size_of::<Self>() as size_t) as *mut Self
            }
            unsafe fn name(this: *const Self) -> *const ::core::ffi::c_char {
                if this.is_null() {
                    return ::core::ptr::null::<::core::ffi::c_char>();
                }
                (*this).name
            }
            unsafe fn set_name(this: *mut Self, name: *const ::core::ffi::c_char) {
                if this.is_null() {
                    return;
                }
                (*this).name = name;
            }
            unsafe fn set_next(this: *mut Self, next: *mut Self) {
                if this.is_null() {
                    return;
                }
                (*this).next = next;
            }
            unsafe fn next_slot(this: *mut Self) -> *mut *mut Self {
                if this.is_null() {
                    return ::core::ptr::null_mut::<*mut Self>();
                }
                &raw mut (*this).next
            }
            $(impl_seq_node!(@wait $extra);)?
        }
    };
    (@wait wait) => {
        unsafe fn mark_wait(this: *mut Self) {
            if this.is_null() {
                return;
            }
            (*this).wait_here = true;
        }
    };
}
impl_seq_node!(NameSeq);
impl_seq_node!(Dep, wait);
impl_seq_node!(GoalDep, wait);

/// Iterator over an intrusive `next`-linked chain (`Dep`, `GoalDep`,
/// `NameSeq`), yielding raw node pointers. The next pointer is read before
/// the item is yielded, so the current node may be freed or relinked by the
/// loop body.
pub struct SeqIter<T: NextLinked> {
    cur: *mut T,
}

impl<T: NextLinked> Iterator for SeqIter<T> {
    type Item = *mut T;
    fn next(&mut self) -> Option<*mut T> {
        if self.cur.is_null() {
            return None;
        }
        let item = self.cur;
        self.cur = unsafe { T::next(item) };
        Some(item)
    }
}

/// Iterate a chain starting at `head`.
///
/// # Safety
/// `head` must be null or point to a valid chain; nodes must stay valid
/// until yielded.
pub unsafe fn seq_iter<T: NextLinked>(head: *mut T) -> SeqIter<T> {
    SeqIter { cur: head }
}

impl File {
    /// Iterate this file's prerequisite chain.
    ///
    /// # Safety
    /// The `deps` chain must be a valid chain; nodes must stay valid until
    /// yielded.
    pub unsafe fn deps_iter(&self) -> SeqIter<Dep> {
        seq_iter(self.deps)
    }
}

/// Free a whole chain of nodes, following `next` links.
pub unsafe fn free_seq_chain<T: NextLinked>(mut n: *mut T) {
    while !n.is_null() {
        let next = T::next(n);
        free(n as *mut ::core::ffi::c_void);
        n = next;
    }
}

/// A simple chain of names, as produced by parse_file_seq.
#[derive(Copy, Clone)]
pub struct NameSeq {
    pub next: *mut NameSeq,
    pub name: *const ::core::ffi::c_char,
}

#[allow(non_camel_case_types)]
pub type file = File;
#[allow(non_camel_case_types)]
pub type dep = Dep;
#[allow(non_camel_case_types)]
pub type nameseq = NameSeq;
#[allow(non_camel_case_types)]
pub type commands = Commands;

/// A file's recipe state — the former `cs_*` integer constants. Discriminants
/// match the original `cmd_state` values (0..=3) so the 2-bit `command_state`
/// bitfield round-trips bit-for-bit.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CommandState {
    NotStarted = 0,
    DepsRunning = 1,
    Running = 2,
    Finished = 3,
}

impl CommandState {
    /// Decode from the raw 2-bit field value (any out-of-range value, which the
    /// 2-bit field can never hold, maps to `Finished`).
    pub fn from_bits(bits: ::core::ffi::c_uint) -> Self {
        match bits {
            0 => CommandState::NotStarted,
            1 => CommandState::DepsRunning,
            2 => CommandState::Running,
            _ => CommandState::Finished,
        }
    }
}

/// A file's update result — the former `us_*` integer constants. Discriminants
/// match the original `update_status` values (0..=3) so the 2-bit
/// `update_status` bitfield round-trips bit-for-bit.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum UpdateStatus {
    Success = 0,
    None = 1,
    Question = 2,
    Failed = 3,
}

impl UpdateStatus {
    /// Decode from the raw 2-bit field value.
    pub fn from_bits(bits: ::core::ffi::c_uint) -> Self {
        match bits {
            0 => UpdateStatus::Success,
            1 => UpdateStatus::None,
            2 => UpdateStatus::Question,
            _ => UpdateStatus::Failed,
        }
    }
}

#[allow(non_camel_case_types)]
pub type cmd_state = CommandState;
#[allow(non_camel_case_types)]
pub type update_status = UpdateStatus;

pub const cs_not_started: CommandState = CommandState::NotStarted;
pub const cs_deps_running: CommandState = CommandState::DepsRunning;
pub const cs_running: CommandState = CommandState::Running;
pub const cs_finished: CommandState = CommandState::Finished;

pub const us_success: UpdateStatus = UpdateStatus::Success;
pub const us_none: UpdateStatus = UpdateStatus::None;
pub const us_question: UpdateStatus = UpdateStatus::Question;
pub const us_failed: UpdateStatus = UpdateStatus::Failed;

impl File {
    pub fn command_state(&self) -> CommandState {
        self.command_state
    }

    pub fn set_command_state(&mut self, state: CommandState) {
        self.command_state = state;
    }

    pub fn update_status(&self) -> UpdateStatus {
        self.update_status
    }

    pub fn set_update_status(&mut self, status: UpdateStatus) {
        self.update_status = status;
    }

    pub fn builtin(&self) -> ::core::ffi::c_uint {
        self.builtin as ::core::ffi::c_uint
    }
    pub fn set_builtin(&mut self, value: ::core::ffi::c_uint) {
        self.builtin = value != 0;
    }
    pub fn precious(&self) -> ::core::ffi::c_uint {
        self.precious as ::core::ffi::c_uint
    }
    pub fn set_precious(&mut self, value: ::core::ffi::c_uint) {
        self.precious = value != 0;
    }
    pub fn loaded(&self) -> ::core::ffi::c_uint {
        self.loaded as ::core::ffi::c_uint
    }
    pub fn set_loaded(&mut self, value: ::core::ffi::c_uint) {
        self.loaded = value != 0;
    }
    pub fn unloaded(&self) -> ::core::ffi::c_uint {
        self.unloaded as ::core::ffi::c_uint
    }
    pub fn set_unloaded(&mut self, value: ::core::ffi::c_uint) {
        self.unloaded = value != 0;
    }
    pub fn low_resolution_time(&self) -> ::core::ffi::c_uint {
        self.low_resolution_time as ::core::ffi::c_uint
    }
    pub fn set_low_resolution_time(&mut self, value: ::core::ffi::c_uint) {
        self.low_resolution_time = value != 0;
    }
    pub fn tried_implicit(&self) -> ::core::ffi::c_uint {
        self.tried_implicit as ::core::ffi::c_uint
    }
    pub fn set_tried_implicit(&mut self, value: ::core::ffi::c_uint) {
        self.tried_implicit = value != 0;
    }
    pub fn updating(&self) -> ::core::ffi::c_uint {
        self.updating as ::core::ffi::c_uint
    }
    pub fn set_updating(&mut self, value: ::core::ffi::c_uint) {
        self.updating = value != 0;
    }
    pub fn updated(&self) -> ::core::ffi::c_uint {
        self.updated as ::core::ffi::c_uint
    }
    pub fn set_updated(&mut self, value: ::core::ffi::c_uint) {
        self.updated = value != 0;
    }
    pub fn is_target(&self) -> ::core::ffi::c_uint {
        self.is_target as ::core::ffi::c_uint
    }
    pub fn set_is_target(&mut self, value: ::core::ffi::c_uint) {
        self.is_target = value != 0;
    }
    pub fn cmd_target(&self) -> ::core::ffi::c_uint {
        self.cmd_target as ::core::ffi::c_uint
    }
    pub fn set_cmd_target(&mut self, value: ::core::ffi::c_uint) {
        self.cmd_target = value != 0;
    }
    pub fn phony(&self) -> ::core::ffi::c_uint {
        self.phony as ::core::ffi::c_uint
    }
    pub fn set_phony(&mut self, value: ::core::ffi::c_uint) {
        self.phony = value != 0;
    }
    pub fn intermediate(&self) -> ::core::ffi::c_uint {
        self.intermediate as ::core::ffi::c_uint
    }
    pub fn set_intermediate(&mut self, value: ::core::ffi::c_uint) {
        self.intermediate = value != 0;
    }
    pub fn is_explicit(&self) -> ::core::ffi::c_uint {
        self.is_explicit as ::core::ffi::c_uint
    }
    pub fn set_is_explicit(&mut self, value: ::core::ffi::c_uint) {
        self.is_explicit = value != 0;
    }
    pub fn secondary(&self) -> ::core::ffi::c_uint {
        self.secondary as ::core::ffi::c_uint
    }
    pub fn set_secondary(&mut self, value: ::core::ffi::c_uint) {
        self.secondary = value != 0;
    }
    pub fn notintermediate(&self) -> ::core::ffi::c_uint {
        self.notintermediate as ::core::ffi::c_uint
    }
    pub fn set_notintermediate(&mut self, value: ::core::ffi::c_uint) {
        self.notintermediate = value != 0;
    }
    pub fn dontcare(&self) -> ::core::ffi::c_uint {
        self.dontcare as ::core::ffi::c_uint
    }
    pub fn set_dontcare(&mut self, value: ::core::ffi::c_uint) {
        self.dontcare = value != 0;
    }
    pub fn ignore_vpath(&self) -> ::core::ffi::c_uint {
        self.ignore_vpath as ::core::ffi::c_uint
    }
    pub fn set_ignore_vpath(&mut self, value: ::core::ffi::c_uint) {
        self.ignore_vpath = value != 0;
    }
    pub fn pat_searched(&self) -> ::core::ffi::c_uint {
        self.pat_searched as ::core::ffi::c_uint
    }
    pub fn set_pat_searched(&mut self, value: ::core::ffi::c_uint) {
        self.pat_searched = value != 0;
    }
    pub fn no_diag(&self) -> ::core::ffi::c_uint {
        self.no_diag as ::core::ffi::c_uint
    }
    pub fn set_no_diag(&mut self, value: ::core::ffi::c_uint) {
        self.no_diag = value != 0;
    }
    pub fn was_shuffled(&self) -> ::core::ffi::c_uint {
        self.was_shuffled as ::core::ffi::c_uint
    }
    pub fn set_was_shuffled(&mut self, value: ::core::ffi::c_uint) {
        self.was_shuffled = value != 0;
    }
    pub fn snapped(&self) -> ::core::ffi::c_uint {
        self.snapped as ::core::ffi::c_uint
    }
    pub fn set_snapped(&mut self, value: ::core::ffi::c_uint) {
        self.snapped = value != 0;
    }
    pub fn suffix(&self) -> ::core::ffi::c_uint {
        self.suffix as ::core::ffi::c_uint
    }
    pub fn set_suffix(&mut self, value: ::core::ffi::c_uint) {
        self.suffix = value != 0;
    }
}

impl Dep {
    pub fn changed(&self) -> ::core::ffi::c_uint {
        self.changed as ::core::ffi::c_uint
    }

    pub fn set_changed(&mut self, value: ::core::ffi::c_uint) {
        self.changed = value != 0;
    }

    pub fn ignore_mtime(&self) -> ::core::ffi::c_uint {
        self.ignore_mtime as ::core::ffi::c_uint
    }

    pub fn set_ignore_mtime(&mut self, value: ::core::ffi::c_uint) {
        self.ignore_mtime = value != 0;
    }

    pub fn need_2nd_expansion(&self) -> ::core::ffi::c_uint {
        self.need_2nd_expansion as ::core::ffi::c_uint
    }

    pub fn set_need_2nd_expansion(&mut self, value: ::core::ffi::c_uint) {
        self.need_2nd_expansion = value != 0;
    }

    pub fn ignore_automatic_vars(&self) -> ::core::ffi::c_uint {
        self.ignore_automatic_vars as ::core::ffi::c_uint
    }

    pub fn set_ignore_automatic_vars(&mut self, value: ::core::ffi::c_uint) {
        self.ignore_automatic_vars = value != 0;
    }

    pub fn wait_here(&self) -> ::core::ffi::c_uint {
        self.wait_here as ::core::ffi::c_uint
    }

    pub fn is_explicit(&self) -> ::core::ffi::c_uint {
        self.is_explicit as ::core::ffi::c_uint
    }
    pub fn set_is_explicit(&mut self, value: ::core::ffi::c_uint) {
        self.is_explicit = value != 0;
    }
    pub fn staticpattern(&self) -> ::core::ffi::c_uint {
        self.staticpattern as ::core::ffi::c_uint
    }
    pub fn set_staticpattern(&mut self, value: ::core::ffi::c_uint) {
        self.staticpattern = value != 0;
    }
    pub fn flags(&self) -> ::core::ffi::c_uint {
        self.flags
    }
    pub fn set_flags(&mut self, value: ::core::ffi::c_uint) {
        self.flags = value;
    }
    pub fn set_wait_here(&mut self, value: ::core::ffi::c_uint) {
        self.wait_here = value != 0;
    }
}

impl GoalDep {
    pub fn flags(&self) -> ::core::ffi::c_uint {
        self.flags
    }

    pub fn set_flags(&mut self, value: ::core::ffi::c_uint) {
        self.flags = value;
    }

    pub fn changed(&self) -> ::core::ffi::c_uint {
        self.changed as ::core::ffi::c_uint
    }

    pub fn set_changed(&mut self, value: ::core::ffi::c_uint) {
        self.changed = value != 0;
    }

    pub fn wait_here(&self) -> ::core::ffi::c_uint {
        self.wait_here as ::core::ffi::c_uint
    }

    pub fn set_wait_here(&mut self, value: ::core::ffi::c_uint) {
        self.wait_here = value != 0;
    }
    pub fn ignore_mtime(&self) -> ::core::ffi::c_uint {
        self.ignore_mtime as ::core::ffi::c_uint
    }
    pub fn set_ignore_mtime(&mut self, value: ::core::ffi::c_uint) {
        self.ignore_mtime = value != 0;
    }
    pub fn staticpattern(&self) -> ::core::ffi::c_uint {
        self.staticpattern as ::core::ffi::c_uint
    }
    pub fn set_staticpattern(&mut self, value: ::core::ffi::c_uint) {
        self.staticpattern = value != 0;
    }
    pub fn need_2nd_expansion(&self) -> ::core::ffi::c_uint {
        self.need_2nd_expansion as ::core::ffi::c_uint
    }
    pub fn set_need_2nd_expansion(&mut self, value: ::core::ffi::c_uint) {
        self.need_2nd_expansion = value != 0;
    }
    pub fn ignore_automatic_vars(&self) -> ::core::ffi::c_uint {
        self.ignore_automatic_vars as ::core::ffi::c_uint
    }
    pub fn set_ignore_automatic_vars(&mut self, value: ::core::ffi::c_uint) {
        self.ignore_automatic_vars = value != 0;
    }
    pub fn is_explicit(&self) -> ::core::ffi::c_uint {
        self.is_explicit as ::core::ffi::c_uint
    }
    pub fn set_is_explicit(&mut self, value: ::core::ffi::c_uint) {
        self.is_explicit = value != 0;
    }
}

impl crate::content_hash::ContentHash for DepFlags {
    fn hash(&self, state: &mut impl crate::content_hash::DigestUpdate) {
        state.update(&self.bits().to_le_bytes());
    }
}

pub type hash_map_func_t = crate::hash::hash_map_func_t;
pub type qsort_cmp_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> i32>;
pub const ENOENT: i32 = 2;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const INTSTR_LENGTH: usize = 53_usize
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22_usize)
    .wrapping_add(3_usize);
pub const RECIPEPREFIX_DEFAULT: i32 = '\t' as i32;
pub const COMMANDS_SILENT: i32 = 2;
pub const COMMANDS_NOERROR: i32 = 4;

impl File {
    fn new_named(name: *const ::core::ffi::c_char) -> Self {
        let mut file = Self {
            name,
            hname: name,
            ..Self::default()
        };
        file.set_update_status(us_none);
        file
    }
}

fn boxed_file(name: *const ::core::ffi::c_char) -> *mut file {
    Box::into_raw(Box::new(File::new_named(name)))
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_ns(n: *mut nameseq) {
    free(n as *mut ::core::ffi::c_void);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_dep(d: *mut dep) {
    free_ns(d as *mut nameseq);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn free_dep_chain(d: *mut dep) {
    free_ns_chain(d as *mut nameseq);
}
pub const UNKNOWN_MTIME: i32 = 0;
pub const NONEXISTENT_MTIME: i32 = 1;
pub const OLD_MTIME: i32 = 2;
pub const ORDINARY_MTIME_MIN: i32 = OLD_MTIME + 1;
// The file table lives on `ExecContext` (`ctx.files`, an idiomatic
// `FxHashMap` keyed by hash-name bytes); the former `static mut files`
// gnulib `hash_table` and its `file_hash_1`/`file_hash_2`/`file_hash_cmp`
// callbacks are gone.

#[derive(Copy, Clone)]
struct RehashedFile {
    _ptr: *mut file,
}

// These file records are process-global C objects. The mutex protects the
// side-list ownership; the records themselves are still managed by `files`.
unsafe impl Send for RehashedFile {}

static REHASHED_FILES: Mutex<Vec<RehashedFile>> = Mutex::new(Vec::new());
fn stop_set_byte(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
}

unsafe fn normalize_lookup_name(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    assert!(!name.is_null(), "assertion failed: name != NULL");
    let bytes = CStr::from_ptr(name).to_bytes_with_nul();
    assert!(bytes[0] != 0, "assertion failed: *name != '\\0'");

    let mut pos = 0usize;
    while bytes[pos] == b'.' && stop_set_byte(bytes[pos + 1], MAP_DIRSEP) && bytes[pos + 2] != 0 {
        pos += 2;
        while stop_set_byte(bytes[pos], MAP_DIRSEP) {
            pos += 1;
        }
    }

    if bytes[pos] == 0 {
        c"./".as_ptr()
    } else {
        bytes[pos..].as_ptr().cast()
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn lookup_file(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
) -> *mut file {
    let name = normalize_lookup_name(name);
    // The file table is keyed by the hash-name bytes; an absent key is the
    // former "no real item in the slot" (null) result.
    let key = CStr::from_ptr(name).to_bytes();
    ctx.files
        .0
        .borrow()
        .get(key)
        .copied()
        .unwrap_or(::core::ptr::null_mut())
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn enter_file(
    ctx: &crate::execctx::ExecContext,
    name: *const ::core::ffi::c_char,
) -> *mut file {
    if name.as_ref().is_some_and(|c| *c as i32 != 0) {
    } else {
        panic!("assertion failed: *name != '\'");
    };
    if !with_options(|o| o.verify.get()) || strcache_iscached(name) != 0 {
    } else {
        panic!("assertion failed: ! verify_flag || strcache_iscached (name)");
    };
    // The table stores each name's chain *head* (the first-entered `file`),
    // keyed by the hash-name bytes; double-colon entries link off that head via
    // `double_colon`/`last`/`prev` and never get their own table slot.
    let key = CStr::from_ptr(name).to_bytes();
    let head = ctx.files.0.borrow().get(key).copied();
    if let Some(f) = head {
        if (*f).double_colon.is_null() {
            // Existing single-colon file: reuse it.
            (*f).builtin = false;
            return f;
        }
        // Existing double-colon head: append a new entry to its chain.
        let new = boxed_file(name);
        (*new).double_colon = f;
        (*f).last
            .as_mut()
            .expect("a double-colon chain head has a last entry")
            .prev = new;
        (*f).last = new;
        return new;
    }
    // Brand-new name: this file becomes the chain head in the table.
    let new = boxed_file(name);
    (*new).last = new;
    ctx.files.0.borrow_mut().insert(Box::from(key), new);
    new
}

/// Byte-exact, allocation-light port of [`normalize_lookup_name`]: collapse any
/// leading `./` (or `.//`, `././`, …) segments. An all-`./` name canonicalizes
/// to `"./"`. Operates on the raw name bytes (no NUL, no `c_char`) and returns
/// the canonical key bytes, so it is usable from safe code.
fn normalize_lookup_name_bytes(name: &[u8]) -> &[u8] {
    let n = name.len();
    let mut pos = 0usize;
    // Mirror the c2rust loop, which reads `name[pos+2]` against the NUL
    // terminator: here that "there is a third byte" test is `pos + 2 < n`.
    while pos + 2 < n && name[pos] == b'.' && stop_set_byte(name[pos + 1], MAP_DIRSEP) {
        pos += 2;
        while pos < n && stop_set_byte(name[pos], MAP_DIRSEP) {
            pos += 1;
        }
    }
    if pos >= n {
        b"./"
    } else {
        &name[pos..]
    }
}

/// Idiomatic, fully-safe counterpart of [`lookup_file`] on the [`FileId`] arena
/// (`ctx.filenodes`): normalize `name` and report the head's `FileId` if it is
/// interned. No `unsafe`, no raw pointers, no `c_char`.
pub fn lookup_filenode(ctx: &crate::execctx::ExecContext, name: &[u8]) -> Option<FileId> {
    let key = normalize_lookup_name_bytes(name);
    let id = FileId::from_bytes(key);
    ctx.filenodes.get(id).map(|_| id)
}

/// Idiomatic, fully-safe counterpart of [`enter_file`] on the [`FileId`] arena:
/// return the head `FileId` for `name`, interning a fresh [`FileNode`] if it is
/// new. Like `enter_file`'s single-colon path, an existing head is reused and
/// its `builtin` mark cleared. A brand-new double-colon (`::`) *entry* is added
/// by [`push_double_colon_entry`] (the rule-recording consumer decides when a
/// target is double-colon); this mirrors how the c2rust `enter_file` only
/// appended once the head was already marked double-colon.
pub fn enter_filenode(ctx: &crate::execctx::ExecContext, name: &[u8]) -> FileId {
    let key = normalize_lookup_name_bytes(name);
    let id = FileId::from_bytes(key);
    // Store the raw key bytes verbatim (no lossy `String`), so `node.id()`
    // equals `id` even for names that are not valid UTF-8.
    let node = ctx
        .filenodes
        .get_or_insert_with(id, || FileNode::new(key.to_vec()));
    node.lock().expect("file node lock poisoned").builtin = false;
    id
}

/// Record a double-colon (`::`) rule for the target `head`, mirroring the
/// c2rust `enter_file` path. The **first** `::` rule lives on the head itself
/// (legacy set `f->double_colon = f` and attached that rule's deps/recipe to
/// `f`); only subsequent `::` rules append a fresh entry. So this marks the
/// head on the first call and appends an inline entry on later calls. No-op if
/// `head` is not interned.
pub fn push_double_colon_entry(ctx: &crate::execctx::ExecContext, head: FileId) {
    if let Some(node) = ctx.filenodes.get(head) {
        let mut n = node.lock().expect("file node lock poisoned");
        if !n.is_double_colon {
            // First `::` definition: the head is the first entry.
            n.is_double_colon = true;
        } else {
            let entry = FileNode::new(n.name.clone());
            n.double_colon.push(entry);
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn rehash_file(
    ctx: &crate::execctx::ExecContext,
    mut from_file: *mut file,
    to_hname: *const ::core::ffi::c_char,
) {
    // Callers always pass a live file here; bind a checked reference so the
    // initial field accesses are null-safe without adding a branch.
    let from_ref = from_file
        .as_mut()
        .expect("rehash_file called with null from_file");
    from_ref.set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    // Already keyed under `to_hname`? Nothing to rehash.
    if CStr::from_ptr(from_ref.hname).to_bytes() == CStr::from_ptr(to_hname).to_bytes() {
        return;
    }
    let from_hname = from_ref.hname;
    // `from_file` is non-null here and each followed `renamed` link is itself
    // non-null, so it stays non-null; read the link through a checked
    // reference, keeping the walk a single branch.
    while !from_file
        .as_ref()
        .expect("rehash_file: null in renamed walk")
        .renamed
        .is_null()
    {
        from_file = from_file
            .as_ref()
            .expect("rehash_file: null in renamed walk")
            .renamed;
    }
    // Re-read the (possibly renamed) file's hash-name through a checked
    // reference so the deref stays null-safe.
    let walked_hname = from_file
        .as_ref()
        .expect("rehash_file: from_file became null")
        .hname;
    if CStr::from_ptr(walked_hname).to_bytes() != CStr::from_ptr(from_hname).to_bytes() {
        abort();
    }
    // Remove `from_file` from the table by its current hash-name; it must be the
    // head stored under that key.
    let removed = ctx
        .files
        .0
        .borrow_mut()
        .remove(CStr::from_ptr(walked_hname).to_bytes());
    if removed != Some(from_file) {
        abort();
    }
    let to_key = CStr::from_ptr(to_hname).to_bytes();
    let to_file = ctx
        .files
        .0
        .borrow()
        .get(to_key)
        .copied()
        .unwrap_or(::core::ptr::null_mut());
    // `from_file` walked only non-null `renamed` links above, so it is still
    // live here; bind a checked reference without adding a branch.
    let fr2 = from_file
        .as_mut()
        .expect("rehash_file: from_file became null");
    fr2.hname = to_hname;
    let mut f = fr2.double_colon;
    while let Some(fr) = f.as_mut() {
        fr.hname = to_hname;
        f = fr.prev;
    }
    if to_file.is_null() {
        // Destination name was free: `from_file` takes that key.
        ctx.files.0.borrow_mut().insert(Box::from(to_key), from_file);
        return;
    }
    if !fr2.cmds.is_null() {
        if (*to_file).cmds.is_null() {
            (*to_file).cmds = fr2.cmds;
        } else if fr2.cmds != (*to_file).cmds {
            let l: size_t = strlen(fr2.name) as size_t;
            let from_cmds = fr2
                .cmds
                .as_mut()
                .expect("from_file recipe is non-null in this branch");
            let to_cmds = (*to_file)
                .cmds
                .as_ref()
                .expect("to_file recipe is non-null in this branch");
            let from_floc = &raw mut from_cmds.fileinfo;
            if !to_cmds.fileinfo.filenm.is_null() {
                error(
        ctx,
        from_floc,
        l.wrapping_add(strlen(to_cmds.fileinfo.filenm) as size_t)
                        .wrapping_add(INTSTR_LENGTH),
        b"recipe was specified for file '%s' at %s:%lu,\0" as *const u8
                        as *const ::core::ffi::c_char,
        &[FmtArg::Str((fr2.name) as *const ::core::ffi::c_char),
            FmtArg::Str((from_cmds.fileinfo.filenm) as *const ::core::ffi::c_char),
            FmtArg::Uint((from_cmds.fileinfo.lineno) as u64)],
    );
            } else {
                error(
        ctx,
        from_floc,
        l,
        b"recipe for file '%s' was found by implicit rule search,\0" as *const u8
                        as *const ::core::ffi::c_char,
        &[FmtArg::Str((fr2.name) as *const ::core::ffi::c_char)],
    );
            }
            error(
        ctx,
        from_floc,
        l,
        b"but '%s' is now considered the same file as '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
        &[FmtArg::Str((fr2.name) as *const ::core::ffi::c_char),
            FmtArg::Str((to_hname) as *const ::core::ffi::c_char)],
    );
            error(
        ctx,
        from_floc,
        l,
        b"recipe for '%s' will be ignored in favor of the one for '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
        &[FmtArg::Str((fr2.name) as *const ::core::ffi::c_char),
            FmtArg::Str((to_hname) as *const ::core::ffi::c_char)],
    );
        }
    }
    if (*to_file).deps.is_null() {
        (*to_file).deps = fr2.deps;
    } else {
        let mut deps: *mut Dep = (*to_file).deps;
        while !(*deps).next.is_null() {
            deps = (*deps).next;
        }
        (*deps).next = fr2.deps;
    }
    merge_variable_set_lists(&raw mut (*to_file).variables, fr2.variables);
    if !(*to_file).double_colon.is_null()
        && fr2.is_target() as i32 != 0
        && fr2.double_colon.is_null()
    {
        fatal(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        (strlen(fr2.name) as size_t).wrapping_add(strlen(to_hname) as size_t),
        b"can't rename single-colon '%s' to double-colon '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
        &[FmtArg::Str((fr2.name) as *const ::core::ffi::c_char),
            FmtArg::Str((to_hname) as *const ::core::ffi::c_char)],
    );
    }
    if (*to_file).double_colon.is_null() && !fr2.double_colon.is_null() {
        if (*to_file).is_target() != 0 {
            fatal(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        (strlen(fr2.name) as size_t).wrapping_add(strlen(to_hname) as size_t),
        b"can't rename double-colon '%s' to single-colon '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
        &[FmtArg::Str((fr2.name) as *const ::core::ffi::c_char),
            FmtArg::Str((to_hname) as *const ::core::ffi::c_char)],
    );
        } else {
            (*to_file).double_colon = fr2.double_colon;
        }
    }
    if fr2.last_mtime > (*to_file).last_mtime {
        (*to_file).last_mtime = fr2.last_mtime;
    }
    (*to_file).mtime_before_update = fr2.mtime_before_update;
    (*to_file).set_precious((*to_file).precious() | fr2.precious() as i32 as ::core::ffi::c_uint);
    (*to_file).set_loaded((*to_file).loaded() | fr2.loaded() as i32 as ::core::ffi::c_uint);
    (*to_file).set_tried_implicit(
        (*to_file).tried_implicit() | fr2.tried_implicit() as i32 as ::core::ffi::c_uint,
    );
    (*to_file).set_updating((*to_file).updating() | fr2.updating() as i32 as ::core::ffi::c_uint);
    (*to_file).set_updated((*to_file).updated() | fr2.updated() as i32 as ::core::ffi::c_uint);
    (*to_file)
        .set_is_target((*to_file).is_target() | fr2.is_target() as i32 as ::core::ffi::c_uint);
    (*to_file)
        .set_cmd_target((*to_file).cmd_target() | fr2.cmd_target() as i32 as ::core::ffi::c_uint);
    (*to_file).set_phony((*to_file).phony() | fr2.phony() as i32 as ::core::ffi::c_uint);
    (*to_file).set_is_explicit(
        (*to_file).is_explicit() | fr2.is_explicit() as i32 as ::core::ffi::c_uint,
    );
    (*to_file)
        .set_secondary((*to_file).secondary() | fr2.secondary() as i32 as ::core::ffi::c_uint);
    (*to_file).set_notintermediate(
        (*to_file).notintermediate() | fr2.notintermediate() as i32 as ::core::ffi::c_uint,
    );
    (*to_file).set_ignore_vpath(
        (*to_file).ignore_vpath() | fr2.ignore_vpath() as i32 as ::core::ffi::c_uint,
    );
    (*to_file).set_snapped((*to_file).snapped() | fr2.snapped() as i32 as ::core::ffi::c_uint);
    (*to_file).set_suffix((*to_file).suffix() | fr2.suffix() as i32 as ::core::ffi::c_uint);
    (*to_file).set_builtin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    fr2.renamed = to_file;
    REHASHED_FILES
        .lock()
        .expect("rehashed file list lock poisoned")
        .push(RehashedFile { _ptr: from_file });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn rename_file(
    ctx: &crate::execctx::ExecContext,
    mut from_file: *mut file,
    to_hname: *const ::core::ffi::c_char,
) {
    rehash_file(ctx, from_file, to_hname);
    while let Some(ff) = from_file.as_mut() {
        ff.name = ff.hname;
        from_file = ff.prev;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn remove_intermediates(ctx: &crate::execctx::ExecContext, sig: i32) {
    let mut doneany: i32 = 0;
    if crate::make_main::opt_question()
        || crate::make_main::opt_touch()
        || ctx.all_secondary.get()
        || ctx.no_intermediates.get()
    {
        return;
    }
    if sig != 0 && crate::make_main::opt_just_print() {
        return;
    }
    // `try_borrow`: this runs from the async `fatal_error_signal` path, which may
    // have interrupted a `borrow_mut` of the table. Best-effort — skip cleanup
    // rather than panic if the table is momentarily borrowed (the former global
    // raw table just raced here).
    let Ok(table) = ctx.files.0.try_borrow() else {
        return;
    };
    let intermediates: Vec<*mut file> = table.values().copied().collect();
    drop(table);
    for f in intermediates {
        {
            if (*f).intermediate() as i32 != 0
                && ((*f).dontcare() as i32 != 0 || (*f).precious() == 0)
                && (*f).secondary() == 0
                && (*f).notintermediate() == 0
                && (*f).cmd_target() == 0
            {
                let status: i32;
                if (*f).update_status() as i32 != us_none as i32 {
                    // ENOENT from unlink means the file was already gone: skip the
                    // diagnostic/bookkeeping below (the C code `continue`d here).
                    let skip: bool;
                    if crate::make_main::opt_just_print() {
                        status = 0;
                        skip = false;
                    } else {
                        status = unlink((*f).name);
                        skip = status < 0 && *__errno_location() == ENOENT;
                    }
                    if !skip && !(*f).dontcare {
                        if sig != 0 {
                            error(
                                ctx,
                                ::core::ptr::null_mut::<Floc>(),
                                strlen((*f).name) as size_t,
                                b"*** deleting intermediate file '%s'\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str(((*f).name) as *const ::core::ffi::c_char)],
                            );
                        } else {
                            if doneany == 0 && 0x1_i32 & db_level != 0 {
                                printf(
                                    b"Removing intermediate files...\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                fflush(stdout);
                            }
                            if !crate::make_main::opt_run_silent() {
                                if doneany == 0 {
                                    fputs(
                                        b"rm \0" as *const u8 as *const ::core::ffi::c_char,
                                        stdout,
                                    );
                                    doneany = 1;
                                } else {
                                    putchar(' ' as i32);
                                }
                                fputs((*f).name, stdout);
                                fflush(stdout);
                            }
                        }
                        if status < 0 {
                            if doneany != 0 {
                                fputs(b"\n\0" as *const u8 as *const ::core::ffi::c_char, stdout);
                            }
                            fflush(stdout);
                            perror_with_name(
                                ctx,
                                b"unlink: \0" as *const u8 as *const ::core::ffi::c_char,
                                (*f).name,
                            );
                            doneany = 0;
                        }
                    }
                }
            }
        }
    }
    if doneany != 0 && sig == 0 {
        putchar('\n' as i32);
        fflush(stdout);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn split_prereqs(
    ctx: &crate::execctx::ExecContext,
    mut p: *mut ::core::ffi::c_char,
) -> *mut dep {
    let mut new: *mut dep = parse_file_seq::<dep>(
        ctx,
        &raw mut p,
        ::core::mem::size_of::<dep>() as size_t,
        0x100_i32,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x40_i32,
    ) as *mut dep;
    if p.as_ref().is_some_and(|c| *c != 0) {
        let mut ood: *mut dep;
        p = p.offset(1_i32 as isize);
        ood = parse_file_seq::<dep>(
            ctx,
            &raw mut p,
            ::core::mem::size_of::<dep>() as size_t,
            0x1_i32,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0x40_i32,
        ) as *mut dep;
        if new.is_null() {
            new = ood;
        } else {
            let mut dp: *mut Dep;
            dp = new;
            while !(*dp).next.is_null() {
                dp = (*dp).next;
            }
            (*dp).next = ood;
        }
        while !ood.is_null() {
            (*ood).ignore_mtime = true;
            ood = (*ood).next;
        }
    }
    new
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn enter_prereqs(
    ctx: &crate::execctx::ExecContext,
    mut deps: *mut dep,
    stem: *const ::core::ffi::c_char,
) -> *mut dep {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut d1: *mut Dep;
    if deps.is_null() {
        return ::core::ptr::null_mut::<Dep>();
    }
    if !stem.is_null() {
        let pattern: *const ::core::ffi::c_char = b"%\0" as *const u8 as *const ::core::ffi::c_char;
        let mut dp: *mut dep = deps;
        let mut dl: *mut dep = ::core::ptr::null_mut::<dep>();
        while let Some(dpr) = dp.as_mut() {
            let percent: *mut ::core::ffi::c_char;
            let nl: size_t = (strlen(dpr.name) as size_t).wrapping_add(1);
            alloca_allocations.push(::std::vec::from_elem(0, nl as usize));
            let nm: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                nm as *mut ::core::ffi::c_void,
                dpr.name as *const ::core::ffi::c_void,
                nl as size_t,
            );
            percent = find_percent(nm);
            if !percent.is_null() {
                let o: *mut ::core::ffi::c_char;
                if *stem.offset(0_i32 as isize) as i32 == 0 {
                    memmove(
                        percent as *mut ::core::ffi::c_void,
                        percent.offset(1_i32 as isize) as *const ::core::ffi::c_void,
                        strlen(percent),
                    );
                    o = variable_buffer_output(
                        variable_buffer,
                        nm,
                        (strlen(nm) as size_t).wrapping_add(1),
                    );
                } else {
                    o = patsubst_expand_pat(
                        variable_buffer,
                        stem,
                        pattern,
                        nm,
                        pattern.offset(1_i32 as isize),
                        percent.offset(1_i32 as isize),
                    );
                }
                if *variable_buffer.offset(0_i32 as isize) as i32 == 0 {
                    let df: *mut dep = dp;
                    if dp == deps {
                        // `dpr` is the null-checked reference to `dp`, and here
                        // `dp == deps`, so it is also the reference to `deps`.
                        deps = dpr.next;
                        dp = deps;
                    } else {
                        // `dl` is the previous list node; it was assigned from a
                        // non-null `dp` on a prior iteration, so it is non-null.
                        // Bind a checked reference (CodeQL-safe, no extra branch).
                        let dlr = dl.as_mut().expect("enter_prereqs: null prev dep");
                        dlr.next = dpr.next;
                        dp = dlr.next;
                    }
                    free_dep(df);
                    continue;
                } else {
                    dpr.name = strcache_add_len(
                        variable_buffer,
                        o.offset_from(variable_buffer) as ::core::ffi::c_long as size_t,
                    );
                }
            }
            dpr.stem = stem;
            dpr.set_staticpattern(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            dl = dp;
            dp = dpr.next;
        }
    }
    d1 = deps;
    while let Some(d1r) = d1.as_mut() {
        if !(d1r.need_2nd_expansion() != 0) {
            d1r.file = lookup_file(ctx, d1r.name);
            if d1r.file.is_null() {
                d1r.file = enter_file(ctx, d1r.name);
            }
            d1r.set_staticpattern(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            d1r.name = ::core::ptr::null::<::core::ffi::c_char>();
            if stem.is_null() {
                d1r.file
                    .as_mut()
                    .expect("dep file was just entered above")
                    .set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        d1 = d1r.next;
    }
    deps
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_deps(ctx: &crate::execctx::ExecContext, f: *mut file) {
    let mut d: *mut dep;
    let mut dp: *mut *mut dep;
    let mut fstem: *const ::core::ffi::c_char;
    let mut initialized: i32 = 0;
    let mut changed_dep: i32 = 0;
    if (*f).snapped() != 0 {
        return;
    }
    (*f).snapped = true;
    dp = &raw mut (*f).deps;
    d = (*f).deps;
    while !d.is_null() {
        let p: *mut ::core::ffi::c_char;
        let mut new: *mut Dep;
        let next: *mut Dep;
        if (*d).name.is_null() || !(*d).need_2nd_expansion {
            dp = &raw mut (*d).next;
            d = (*d).next;
        } else {
            if (*d).staticpattern {
                let mut cs: *const ::core::ffi::c_char = (*d).name;
                let mut nperc: size_t = 0;
                loop {
                    cs = strchr(cs, '%' as i32);
                    if cs.is_null() {
                        break;
                    }
                    nperc = nperc.wrapping_add(1);
                    cs = cs.offset(1_i32 as isize);
                }
                if nperc != 0 {
                    let name_len = strlen((*d).name) as size_t;
                    let slen: size_t = name_len.wrapping_add(nperc).wrapping_add(1);
                    // End of the source name, computed once so the per-`%`
                    // token scan below stays bounded against it instead of
                    // re-`strlen`'ing the whole remaining suffix each iteration
                    // (which would be O(n^2) over a name with many `%`).
                    let name_end: *const ::core::ffi::c_char = (*d).name.add(name_len as usize);
                    let mut pcs: *const ::core::ffi::c_char = (*d).name;
                    let name: *mut ::core::ffi::c_char = xmalloc(slen) as *mut ::core::ffi::c_char;
                    let mut s: *mut ::core::ffi::c_char = name;
                    cs = strchr(pcs, '%' as i32);
                    while !cs.is_null() {
                        s = mempcpy(
                            s as *mut ::core::ffi::c_void,
                            pcs as *const ::core::ffi::c_void,
                            cs.offset_from(pcs) as ::core::ffi::c_long as size_t,
                        ) as *mut ::core::ffi::c_char;
                        let fresh0 = s;
                        s = s.offset(1_i32 as isize);
                        *fresh0 = '$' as i32 as ::core::ffi::c_char;
                        let fresh1 = s;
                        s = s.offset(1_i32 as isize);
                        *fresh1 = '*' as i32 as ::core::ffi::c_char;
                        cs = cs.offset(1_i32 as isize);
                        pcs = cs;
                        // Bridge to the safe `end_of_token`: it returns the
                        // offset of the first whitespace/NUL within `[cs, NUL)`,
                        // which we add back to `cs` to recover the C pointer.
                        // `cs` points within `(*d).name`, so bound the slice with
                        // the precomputed `name_end` rather than re-`strlen`'ing.
                        let cs_avail = name_end.offset_from(cs) as usize;
                        let eot = cs.add(end_of_token(::core::slice::from_raw_parts(
                            cs as *const u8,
                            cs_avail,
                        )));
                        cs = strchr(eot, '%' as i32);
                    }
                    strcpy(s, pcs);
                    free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
                    (*d).name = name;
                }
            }
            if initialized == 0 {
                initialize_file_variables(ctx, f, 0);
                initialized = 1;
            }
            set_file_variables(
                ctx,
                f,
                if !(*d).stem.is_null() {
                    (*d).stem
                } else {
                    (*f).stem
                },
            );
            p = expand_string_for_file(ctx, (*d).name, f);
            free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            new = split_prereqs(ctx, p);
            if new.is_null() {
                *dp = (*d).next;
                changed_dep = 1;
                free_dep(d);
                d = *dp;
            } else {
                fstem = (*d).stem;
                next = (*d).next;
                changed_dep = 1;
                free_dep(d);
                *dp = new;
                dp = &raw mut new;
                for d in seq_iter(new) {
                    (*d).file = lookup_file(ctx, (*d).name);
                    if (*d).file.is_null() {
                        (*d).file = enter_file(ctx, (*d).name);
                    }
                    (*d).name = ::core::ptr::null::<::core::ffi::c_char>();
                    (*d).stem = fstem;
                    if fstem.is_null() {
                        (*d).file
                            .as_mut()
                            .expect("dep file was just entered above")
                            .set_is_explicit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    dp = &raw mut (*d).next;
                }
                *dp = next;
                d = *dp;
            }
        }
    }
    if changed_dep != 0 {
        crate::shuffle::shuffle_deps_recursive((*f).deps);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_extra_prereqs(
    ctx: &crate::execctx::ExecContext,
    extra: *const variable,
) -> *mut dep {
    let mut d: *mut dep;
    let prereqs: *mut dep = if !extra.is_null() {
        split_prereqs(
            ctx,
            expand_string_buf(
                ctx,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                (*extra).value,
                SIZE_MAX as size_t,
            ),
        )
    } else {
        ::core::ptr::null_mut::<Dep>()
    };
    d = prereqs;
    while let Some(dr) = d.as_mut() {
        dr.file = lookup_file(ctx, dr.name);
        if dr.file.is_null() {
            dr.file = enter_file(ctx, dr.name);
        }
        dr.name = ::core::ptr::null::<::core::ffi::c_char>();
        dr.set_ignore_automatic_vars(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        d = dr.next;
    }
    prereqs
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn snap_file(ctx: &crate::execctx::ExecContext, f: *mut file, deps: *const dep) {
    let mut prereqs: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut d: *mut dep;
    // `snap_file` is only ever called with a non-null file (its sole caller
    // `expand_deps` filters out null slots). Bind a checked reference so the
    // derefs below are null-safe without adding a control-flow branch.
    let fr = f.as_mut().expect("snap_file called with null file");
    if !second_expansion() {
        fr.set_updating(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if ctx.all_secondary.get() && fr.notintermediate() == 0 {
        fr.set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if ctx.no_intermediates.get() && fr.intermediate() == 0 && fr.secondary() == 0 {
        fr.set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if let Some(file_vars) = fr.variables.as_ref() {
        prereqs = expand_extra_prereqs(
            ctx,
            lookup_variable_in_set(
                ctx,
                b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
                file_vars.set,
            ),
        );
        if second_expansion() {
            d = prereqs;
            while let Some(dr) = d.as_mut() {
                if dr.name.is_null() {
                    dr.name = xstrdup(dr.file.as_ref().expect("a nameless prereq has a file").name);
                }
                dr.set_need_2nd_expansion(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                d = dr.next;
            }
        }
    } else if fr.is_target() != 0 {
        prereqs = copy_dep_chain(deps);
    }
    if !prereqs.is_null() {
        d = prereqs;
        while let Some(dr) = d.as_ref() {
            let dname: *const ::core::ffi::c_char = if !dr.name.is_null() {
                dr.name
            } else {
                dr.file.as_ref().expect("expand_deps: null dep file").name
            };
            let fname = fr.name;
            let same = match (dname.as_ref(), fname.as_ref()) {
                (Some(&db), Some(&fb)) => {
                    fb as i32 == db as i32
                        && (fb as i32 == 0 || strcmp(fname.offset(1), dname.offset(1)) == 0)
                }
                _ => false,
            };
            if same {
                break;
            }
            d = dr.next;
        }
        if !d.is_null() {
            free_dep_chain(prereqs);
        } else if fr.deps.is_null() {
            fr.deps = prereqs;
        } else {
            // `fr.deps` is non-null in this arm and each `.next` we follow is
            // non-null until the last node; walk to the tail through checked
            // references (single branch) and append.
            d = fr.deps;
            while !d
                .as_ref()
                .expect("snap_file: null in deps walk")
                .next
                .is_null()
            {
                d = d.as_ref().expect("snap_file: null in deps walk").next;
            }
            d.as_mut().expect("snap_file: null deps tail").next = prereqs;
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn snap_deps(ctx: &crate::execctx::ExecContext) {
    let mut f: *mut file;
    let mut f2: *mut file;
    let mut d: *mut dep;
    crate::make_main::mark_snapped_deps();
    f = lookup_file(ctx, b".PRECIOUS\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                f2r.set_precious(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(ctx, b".LOW_RESOLUTION_TIME\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                f2r.set_low_resolution_time(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(ctx, b".PHONY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                f2r.set_phony(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2r.set_is_target(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                f2r.last_mtime = NONEXISTENT_MTIME as uintmax_t;
                f2r.mtime_before_update = NONEXISTENT_MTIME as uintmax_t;
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(ctx, b".NOTINTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        if !fr.deps.is_null() {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    f2r.set_notintermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        } else {
            ctx.no_intermediates.set(true);
        }
        f = fr.prev;
    }
    f = lookup_file(ctx, b".INTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        d = fr.deps;
        while let Some(dr) = d.as_ref() {
            f2 = dr.file;
            while let Some(f2r) = f2.as_mut() {
                if f2r.notintermediate() != 0 {
                    fatal(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        strlen(f2r.name) as size_t,
        b"%s cannot be both .NOTINTERMEDIATE and .INTERMEDIATE\0" as *const u8
                            as *const ::core::ffi::c_char,
        &[FmtArg::Str((f2r.name) as *const ::core::ffi::c_char)],
    );
                } else {
                    f2r.set_intermediate(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                f2 = f2r.prev;
            }
            d = dr.next;
        }
        f = fr.prev;
    }
    f = lookup_file(ctx, b".SECONDARY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        let Some(fr) = f.as_ref() else { break };
        if !fr.deps.is_null() {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    if f2r.notintermediate() != 0 {
                        fatal(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        strlen(f2r.name) as size_t,
        b"%s cannot be both .NOTINTERMEDIATE and .SECONDARY\0" as *const u8
                                as *const ::core::ffi::c_char,
        &[FmtArg::Str((f2r.name) as *const ::core::ffi::c_char)],
    );
                    } else {
                        let rhs = {
                            f2r.set_secondary(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            f2r.secondary()
                        } as ::core::ffi::c_uint;
                        f2r.set_intermediate(rhs);
                    }
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        } else {
            ctx.all_secondary.set(true);
        }
        f = fr.prev;
    }
    if ctx.no_intermediates.get() && ctx.all_secondary.get() {
        fatal(
            ctx,
            ::core::ptr::null_mut::<Floc>(),
            0,
            b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    f = lookup_file(ctx, b".EXPORT_ALL_VARIABLES\0" as *const u8 as *const ::core::ffi::c_char);
    if f.as_ref().is_some_and(|fr| fr.is_target() as i32 != 0) {
        with_options(|o| o.export_all_variables.set(true));
    }
    f = lookup_file(ctx, b".IGNORE\0" as *const u8 as *const ::core::ffi::c_char);
    if let Some(fr) = f.as_ref().filter(|fr| fr.is_target() as i32 != 0) {
        if fr.deps.is_null() {
            crate::make_main::set_ignore_errors_mirror(true);
        } else {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    f2r.command_flags |= COMMANDS_NOERROR;
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        }
    }
    f = lookup_file(ctx, b".SILENT\0" as *const u8 as *const ::core::ffi::c_char);
    if let Some(fr) = f.as_ref().filter(|fr| fr.is_target() as i32 != 0) {
        if fr.deps.is_null() {
            with_options(|o| o.run_silent.set(true));
        } else {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    f2r.command_flags |= COMMANDS_SILENT;
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        }
    }
    f = lookup_file(ctx, b".NOTPARALLEL\0" as *const u8 as *const ::core::ffi::c_char);
    if let Some(fr) = f.as_ref().filter(|fr| fr.is_target() as i32 != 0) {
        let mut d2: *mut dep;
        if fr.deps.is_null() {
            crate::make_main::set_not_parallel();
        } else {
            d = fr.deps;
            while let Some(dr) = d.as_ref() {
                f2 = dr.file;
                while let Some(f2r) = f2.as_mut() {
                    if !f2r.deps.is_null() {
                        d2 = f2r.deps.as_ref().expect("checked non-null above").next;
                        while let Some(d2r) = d2.as_mut() {
                            d2r.set_wait_here(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            d2 = d2r.next;
                        }
                    }
                    f2 = f2r.prev;
                }
                d = dr.next;
            }
        }
    }
    let prereqs: *mut dep = expand_extra_prereqs(
        ctx,
        lookup_variable(
            ctx,
            b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
        ),
    );
    // Snapshot the table's files, then snap each. Matching the C `hash_dump`,
    // any files entered while snapping are not themselves re-processed here.
    let filedump: Vec<*mut file> = ctx.files.0.borrow().values().copied().collect();
    for fp in filedump {
        snap_file(ctx, fp, prereqs);
    }
    free_dep_chain(prereqs);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn set_command_state(file: *mut file, state: cmd_state) {
    let mut d: *mut dep;
    (*file).set_command_state(state);
    d = (*file).also_make;
    while !d.is_null() {
        let dfile = (*d)
            .file
            .as_mut()
            .expect("an also_make dep always has a file");
        if state as ::core::ffi::c_uint > dfile.command_state() as ::core::ffi::c_uint {
            dfile.set_command_state(state);
        }
        d = (*d).next;
    }
}
/// Build a [`SystemTime`] from a Unix `(seconds, nanoseconds)` pair as reported
/// by `stat(2)`. `secs` may be negative (pre-1970); `nsec` is the in-second
/// remainder in `[0, 1_000_000_000)`. This is the `std::time` boundary for the
/// raw `time_t`/`c_long` values the filesystem hands us.
pub fn system_time_from_unix(secs: i64, nsec: u32) -> SystemTime {
    if secs >= 0 {
        UNIX_EPOCH + Duration::new(secs as u64, nsec)
    } else {
        // `unsigned_abs` keeps `i64::MIN` well-defined; `nsec` shifts forward.
        UNIX_EPOCH - Duration::from_secs(secs.unsigned_abs()) + Duration::from_nanos(nsec as u64)
    }
}
/// Pack a Unix `(seconds, nanoseconds)` offset into GNU make's `FILE_TIMESTAMP`
/// encoding (whole seconds in the high bits, sub-second resolution units in the
/// low `FILE_TIMESTAMP_HI_RES ? 30 : 0` bits, biased by `ORDINARY_MTIME_MIN`).
/// Pure arithmetic — no I/O, no pointers.
///
/// Returns `Ok(ts)` for a stamp inside the encodable range, or `Err(clamp)`
/// when it falls outside: `ORDINARY_MTIME_MIN` when the stamp is at or below
/// `OLD_MTIME` (underflow), otherwise the maximum ordinary timestamp
/// (overflow). The caller decides what to do with the out-of-range report.
///
/// Operates on raw `i64`/`i64` so the full out-of-range domain (which
/// `SystemTime` cannot represent) stays reachable; all arithmetic is
/// `wrapping_*` to reproduce the C macro's modular `uintmax_t` behavior
/// byte-for-byte, differential-tested against the verbatim c2rust expression.
fn pack_unix_timestamp(stamp: i64, ns: i64) -> Result<uintmax_t, uintmax_t> {
    let hi = if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 };
    let res = (if FILE_TIMESTAMP_HI_RES != 0 {
        1_000_000_000
    } else {
        1
    }) as uintmax_t;
    let min = ORDINARY_MTIME_MIN as uintmax_t;

    let offset =
        (ORDINARY_MTIME_MIN as i64 + if FILE_TIMESTAMP_HI_RES != 0 { ns } else { 0 }) as i32;
    let s = stamp as uintmax_t;
    let product = s << hi;
    let ts = product.wrapping_add(offset as uintmax_t);

    // `base` is the largest multiple of the resolution that still fits a
    // `uintmax_t` once biased off `min`. `ordinary_max` is the largest
    // encodable packed timestamp; `s_max` is the largest whole-seconds value it
    // admits (the same bound shifted back down by `hi`).
    let base = uintmax_t::MAX.wrapping_sub(min) >> hi << hi;
    let ordinary_max = base.wrapping_add(min).wrapping_add(res).wrapping_sub(1);
    let s_max = (base.wrapping_add(res).wrapping_sub(1)) >> hi;

    if s <= s_max && product <= ts && ts <= ordinary_max {
        Ok(ts)
    } else if s <= OLD_MTIME as uintmax_t {
        Err(min)
    } else {
        Err(ordinary_max)
    }
}
/// Decompose a [`SystemTime`] to its Unix `(seconds, nanoseconds)` offset and
/// pack it via [`pack_unix_timestamp`]. Pre-epoch times truncate to whole
/// seconds, matching the original `file_timestamp_now` clock path.
fn pack_file_timestamp(t: SystemTime) -> Result<uintmax_t, uintmax_t> {
    let (secs, ns) = match t.duration_since(UNIX_EPOCH) {
        Ok(d) => (d.as_secs() as i64, d.subsec_nanos() as i64),
        Err(e) => (-(e.duration().as_secs() as i64), 0),
    };
    pack_unix_timestamp(secs, ns)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; `fname`, when non-null, must be a valid NUL-terminated C
/// string. (`fname` and the variadic C `error` sink are deliberately retained
/// so the out-of-range diagnostic stays byte-identical to GNU make's; the
/// timestamp `t` is now a `std::time::SystemTime` and the packing math itself
/// is the safe [`pack_file_timestamp`].)
pub unsafe fn file_timestamp_cons(
    ctx: &crate::execctx::ExecContext,
    fname: *const ::core::ffi::c_char,
    t: SystemTime,
) -> uintmax_t {
    match pack_file_timestamp(t) {
        Ok(ts) => ts,
        Err(substitute) => {
            let f: *const ::core::ffi::c_char = if !fname.is_null() {
                fname
            } else {
                b"Current time\0" as *const u8 as *const ::core::ffi::c_char
            };
            let stamp = CString::new(file_timestamp_string(substitute))
                .expect("formatted timestamp never contains an interior NUL");
            let buf = stamp.as_ptr();
            error(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                (strlen(f) as size_t).wrapping_add(strlen(buf) as size_t),
                b"%s: timestamp out of range: substituting %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[FmtArg::Str(f), FmtArg::Str(buf)],
            );
            substitute
        }
    }
}
/// Sample the wall clock and pack it into a `FILE_TIMESTAMP`, returning the
/// packed value together with the clock resolution (always `1` ns on the
/// `std::time` path). Safe: the only `unsafe` is the inner `file_timestamp_cons`
/// call, which is sound with a null filename.
pub fn file_timestamp_now(ctx: &crate::execctx::ExecContext) -> (uintmax_t, i32) {
    // The original c2rust translation tried clock_gettime(CLOCK_REALTIME),
    // then gettimeofday, then time(). On supported platforms the clock_gettime
    // path (nanosecond resolution, r = 1) always succeeds, so
    // std::time::SystemTime::now (backed by CLOCK_REALTIME on Linux) preserves
    // the observed behavior; its pre-epoch handling lives in
    // `pack_file_timestamp`.
    let resolution = 1;
    // SAFETY: a null filename is the documented "no name" sentinel that
    // `file_timestamp_cons` handles.
    let stamp = unsafe {
        file_timestamp_cons(
            ctx,
            ::core::ptr::null::<::core::ffi::c_char>(),
            SystemTime::now(),
        )
    };
    (stamp, resolution)
}
/// Render a packed `FILE_TIMESTAMP` exactly as GNU make's
/// `file_timestamp_sprintf`: `YYYY-MM-DD HH:MM:SS` in **local** time, followed
/// by the sub-second fraction with trailing zeros trimmed (and the `.` dropped
/// entirely when the fraction is zero). When the broken-down local time can't
/// be computed (the year overflows the calendar range) it falls back to the
/// raw seconds count — signed if negative, unsigned otherwise — matching the
/// C `%ld`/`%lu` branches.
///
/// Local broken-down time comes from `chrono::Local` — a pure-Rust,
/// timezone-aware instant->local-calendar conversion (no libc `localtime`
/// global, no `tm` FFI struct). All formatting, fraction trimming, and the
/// fallback are safe Rust. Output is byte-for-byte identical to the C oracle.
pub fn file_timestamp_string(ts: uintmax_t) -> String {
    use chrono::{Datelike, Local, TimeZone, Timelike};

    let shift = if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 };
    let units = ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t);
    let t = (units >> shift) as time_t;
    // Sub-second units; the `& (2^shift - 1)` mask yields 0 when HI_RES is off.
    let frac = (units & (((1u64 << shift) - 1) as uintmax_t)) as i32;

    // Broken-down LOCAL time via `chrono::Local`: `timestamp_opt` maps the UTC
    // instant `t` to the single local calendar time, or `None` when the year
    // overflows the representable range — the same condition under which C's
    // `localtime` returned NULL and make fell back to the raw seconds count.
    let mut out = match Local.timestamp_opt(t as i64, 0).single() {
        // `%04ld` of `tm_year + 1900`; for every realistic file timestamp this
        // is a >= 4-digit non-negative year, so `{:04}` matches `%04ld`.
        Some(dt) => format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
        ),
        None if t < 0 => format!("{}", t as i64), // C `%ld`
        None => format!("{}", t as u64),          // C `%lu`
    };

    // C prints `.%09d` then walks back over trailing '0's, dropping the '.'
    // too when nothing but zeros remain.
    let mut frac_str = format!(".{frac:09}");
    while frac_str.ends_with('0') {
        frac_str.pop();
    }
    if frac_str.ends_with('.') {
        frac_str.pop();
    }
    out.push_str(&frac_str);
    out
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_prereqs(mut deps: *const dep) {
    let mut ood: *const dep = ::core::ptr::null::<dep>();
    while !deps.is_null() {
        if !(*deps).ignore_mtime {
            printf(
                b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                if (*deps).wait_here() as i32 != 0 {
                    b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !(*deps).name.is_null() {
                    (*deps).name
                } else {
                    (*deps)
                        .file
                        .as_ref()
                        .expect("a nameless dep has a file")
                        .name
                },
            );
        } else if ood.is_null() {
            ood = deps;
        }
        deps = (*deps).next;
    }
    if let Some(oodr) = ood.as_ref() {
        printf(
            b" | %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            if oodr.wait_here() as i32 != 0 {
                b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !oodr.name.is_null() {
                oodr.name
            } else {
                oodr.file.as_ref().expect("a nameless dep has a file").name
            },
        );
        ood = oodr.next;
        while let Some(oodn) = ood.as_ref() {
            if oodn.ignore_mtime() != 0 {
                printf(
                    b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    if oodn.wait_here() as i32 != 0 {
                        b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    if !oodn.name.is_null() {
                        oodn.name
                    } else {
                        oodn.file.as_ref().expect("a nameless dep has a file").name
                    },
                );
            }
            ood = oodn.next;
        }
    }
    putchar('\n' as i32);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_file(item: *const ::core::ffi::c_void) {
    let f: *const file = item as *const file;
    if crate::make_main::opt_no_builtin_rules() && (*f).builtin() as i32 != 0 {
        return;
    }
    putchar('\n' as i32);
    if (*f)
        .cmds
        .as_ref()
        .is_some_and(|c| c.recipe_prefix as i32 != crate::make_main::opt_cmd_prefix() as i32)
    {
        fputs(
            b".RECIPEPREFIX = \0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        let new_prefix = (*f)
            .cmds
            .as_ref()
            .expect("cmds is non-null when its recipe_prefix differs")
            .recipe_prefix;
        with_options(|o| o.cmd_prefix.set(new_prefix));
        if new_prefix as i32 != RECIPEPREFIX_DEFAULT {
            putchar(new_prefix as i32);
        }
        putchar('\n' as i32);
    }
    if !(*f).variables.is_null() {
        print_target_variables(f);
    }
    if !(*f).is_target {
        puts(b"# Not a target:\0" as *const u8 as *const ::core::ffi::c_char);
    }
    printf(
        b"%s:%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*f).name,
        if !(*f).double_colon.is_null() {
            b":\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    print_prereqs((*f).deps);
    if (*f).precious {
        puts(
            b"#  Precious file (prerequisite of .PRECIOUS).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).phony {
        puts(
            b"#  Phony target (prerequisite of .PHONY).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).cmd_target {
        puts(b"#  Command line target.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*f).dontcare {
        puts(
            b"#  A default, MAKEFILES, or -include/sinclude makefile.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).builtin {
        puts(b"#  Builtin rule\0" as *const u8 as *const ::core::ffi::c_char);
    }
    puts(if (*f).tried_implicit() as i32 != 0 {
        b"#  Implicit rule search has been done.\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"#  Implicit rule search has not been done.\0" as *const u8 as *const ::core::ffi::c_char
    });
    if !(*f).stem.is_null() {
        printf(
            b"#  Implicit/static pattern stem: '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*f).stem,
        );
    }
    if (*f).intermediate {
        puts(
            b"#  File is an intermediate prerequisite.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).notintermediate {
        puts(
            b"#  File is a prerequisite of .NOTINTERMEDIATE.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).secondary {
        puts(
            b"#  File is secondary (prerequisite of .SECONDARY).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*f).is_explicit {
        puts(b"#  File is explicitly mentioned.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !(*f).also_make.is_null() {
        let _d: *const Dep;
        fputs(
            b"#  Also makes:\0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        for d in seq_iter((*f).also_make) {
            printf(
                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                if !(*d).name.is_null() {
                    (*d).name
                } else {
                    (*d).file.as_ref().expect("a nameless dep has a file").name
                },
            );
        }
        putchar('\n' as i32);
    }
    if (*f).last_mtime == UNKNOWN_MTIME as uintmax_t {
        puts(b"#  Modification time never checked.\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*f).last_mtime == NONEXISTENT_MTIME as uintmax_t {
        puts(b"#  File does not exist.\0" as *const u8 as *const ::core::ffi::c_char);
    } else if (*f).last_mtime == OLD_MTIME as uintmax_t {
        puts(b"#  File is very old.\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        let stamp = CString::new(file_timestamp_string((*f).last_mtime))
            .expect("formatted timestamp never contains an interior NUL");
        printf(
            b"#  Last modified %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            stamp.as_ptr(),
        );
    }
    puts(if (*f).updated() as i32 != 0 {
        b"#  File has been updated.\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"#  File has not been updated.\0" as *const u8 as *const ::core::ffi::c_char
    });
    match (*f).command_state() as i32 {
        2 => {
            puts(
                b"#  Recipe currently running (THIS IS A BUG).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        1 => {
            puts(
                b"#  Dependencies recipe running (THIS IS A BUG).\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        0 | 3 => match (*f).update_status() as i32 {
            0 => {
                puts(b"#  Successfully updated.\0" as *const u8 as *const ::core::ffi::c_char);
            }
            2 => {
                if crate::make_main::opt_question() {
                } else {
                    panic!("assertion failed: question_flag");
                };
                puts(
                    b"#  Needs to be updated (-q is set).\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        },
        _ => {
            puts(
                b"#  Invalid value in 'command_state' member!\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fflush(stdout);
            fflush(stderr);
            abort();
        }
    }
    if !(*f).variables.is_null() {
        print_file_variables(f);
    }
    if !(*f).cmds.is_null() {
        print_commands((*f).cmds);
    }
    if !(*f).prev.is_null() {
        print_file((*f).prev as *const ::core::ffi::c_void);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_file_data_base(ctx: &crate::execctx::ExecContext) {
    puts(b"\n# Files\0" as *const u8 as *const ::core::ffi::c_char);
    // `print_file` walks each name's prev/double-colon chain; `for_each`
    // snapshots so the table borrow is not held across it.
    ctx.files
        .for_each(|f| unsafe { print_file(f as *const ::core::ffi::c_void) });
    printf(
        b"\n# %lu files in the file table.\n\0" as *const u8 as *const ::core::ffi::c_char,
        ctx.files.0.borrow().len() as ::core::ffi::c_ulong,
    );
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_target(item: *const ::core::ffi::c_void) {
    let f: *const file = item as *const file;
    if (*f).is_target() == 0 || (*f).suffix() as i32 != 0 {
        return;
    }
    // Skip built-in special targets, whose names are a dot followed by one
    // or more all-uppercase letters (e.g. `.SUFFIXES`, `.PHONY`).
    let name = ::core::ffi::CStr::from_ptr((*f).name).to_bytes();
    if name.len() >= 2 && name[0] == b'.' && name[1..].iter().all(u8::is_ascii_uppercase) {
        return;
    }
    puts((*f).name);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_targets(ctx: &crate::execctx::ExecContext) {
    ctx.files
        .for_each(|f| unsafe { print_target(f as *const ::core::ffi::c_void) });
}
/// Report (via `error`) when a single file/dep field is set but not interned
/// in the strcache. A null/empty field, or one already cached, is silent.
unsafe fn verify_field_cached(
    ctx: &crate::execctx::ExecContext,
    owner: *const ::core::ffi::c_char,
    field: &::core::ffi::CStr,
    value: *const ::core::ffi::c_char,
) {
    // A field is well-formed when it is null/empty, or interned in the strcache.
    if value.is_null() || *value as i32 == 0 || strcache_iscached(value) != 0 {
        return;
    }
    error(
        ctx,
        ::core::ptr::null::<Floc>(),
        (strlen(owner) as size_t)
            .wrapping_add(field.count_bytes() as size_t)
            .wrapping_add(strlen(value) as size_t),
        b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
        &[FmtArg::Str((owner) as *const ::core::ffi::c_char),
            FmtArg::Str((field.as_ptr()) as *const ::core::ffi::c_char),
            FmtArg::Str((value) as *const ::core::ffi::c_char)],
    );
}

/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file(item: *const ::core::ffi::c_void, arg: *mut ::core::ffi::c_void) {
    // Invoked via `hash_map_arg`; `arg` carries the borrowed `ExecContext` so the
    // diagnostics below can be prefixed correctly without any global.
    let ctx = &*(arg as *const crate::execctx::ExecContext);
    let f: *const file = item as *const file;

    verify_field_cached(ctx, (*f).name, c"name", (*f).name);
    verify_field_cached(ctx, (*f).name, c"hname", (*f).hname);
    verify_field_cached(ctx, (*f).name, c"vpath", (*f).vpath);
    verify_field_cached(ctx, (*f).name, c"stem", (*f).stem);

    let mut d: *const dep = (*f).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 {
            verify_field_cached(ctx, (*d).name, c"name", (*d).name);
        }
        verify_field_cached(ctx, (*d).name, c"stem", (*d).stem);
        d = (*d).next;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file_data_base(ctx: &crate::execctx::ExecContext) {
    let ctx_arg = ctx as *const crate::execctx::ExecContext as *mut ::core::ffi::c_void;
    ctx.files
        .for_each(|f| unsafe { verify_file(f as *const ::core::ffi::c_void, ctx_arg) });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn build_target_list(
    ctx: &crate::execctx::ExecContext,
    mut value: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut last_targ_count: ::core::ffi::c_ulong = 0;
    let fill = ctx.files.0.borrow().len() as ::core::ffi::c_ulong;
    if fill != last_targ_count {
        let mut max: size_t = (strlen(value) as size_t)
            .wrapping_div(500)
            .wrapping_add(1)
            .wrapping_mul(500);
        let mut len: size_t;
        let mut p: *mut ::core::ffi::c_char;
        value = xrealloc(value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
        p = value;
        len = 0;
        let targets: Vec<*mut file> = ctx.files.0.borrow().values().copied().collect();
        for f in targets {
            {
                if (*f).is_target() == 0 {
                    continue;
                }
                let l: size_t = strlen((*f).name) as size_t;
                len = len.wrapping_add(l.wrapping_add(1));
                if len > max {
                    let off: size_t = p.offset_from(value) as ::core::ffi::c_long as size_t;
                    max = max.wrapping_add(
                        l.wrapping_add(1)
                            .wrapping_div(500)
                            .wrapping_add(1)
                            .wrapping_mul(500),
                    );
                    value = xrealloc(value as *mut ::core::ffi::c_void, max)
                        as *mut ::core::ffi::c_char;
                    p = value.offset(off as isize) as *mut ::core::ffi::c_char;
                }
                p = mempcpy(
                    p as *mut ::core::ffi::c_void,
                    (*f).name as *const ::core::ffi::c_void,
                    l as size_t,
                ) as *mut ::core::ffi::c_char;
                let fresh4 = p;
                p = p.offset(1_i32 as isize);
                *fresh4 = ' ' as i32 as ::core::ffi::c_char;
            }
        }
        *p.offset(-(1_i32 as isize)) = 0;
        last_targ_count = fill;
    }
    value
}
pub const FILE_TIMESTAMP_HI_RES: i32 = 1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::implicit::alloc_dep;
    use crate::make_main::initialize_stopchar_map;
    use crate::strcache::strcache_add;

    // FFI declarations and types the pre-std clock cascade depended on. They
    // were removed from production code when `file_timestamp_now` moved to
    // `std::time::SystemTime`, so we re-declare them here (test-only) purely to
    // keep the verbatim oracle compilable per AGENTS.md "preserve the original
    // as a test oracle". Production stays free of this FFI/unsafe.
    const CLOCK_REALTIME: i32 = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    extern "C" {
        fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> i32;
        fn time(__timer: *mut time_t) -> time_t;
        fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> i32;
    }

    /// Verbatim copy of the pre-std `file_timestamp_now` clock cascade:
    /// `clock_gettime(CLOCK_REALTIME)` -> `gettimeofday` -> `time()` fallback.
    /// Kept test-only as the differential oracle for the new safe
    /// `std::time::SystemTime` implementation (AGENTS.md verbatim-oracle rule).
    unsafe fn file_timestamp_now_oracle(
        ctx: &crate::execctx::ExecContext,
        resolution: *mut i32,
    ) -> uintmax_t {
        let r: i32;
        let s: time_t;
        let ns: i32;
        let mut timespec: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if clock_gettime(CLOCK_REALTIME, &raw mut timespec) == 0 {
            r = 1;
            s = timespec.tv_sec as time_t;
            ns = timespec.tv_nsec as i32;
        } else {
            let mut timeval: timeval = timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            if gettimeofday(
                &raw mut timeval,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ) == 0
            {
                r = 1000_i32;
                s = timeval.tv_sec as time_t;
                ns = (timeval.tv_usec * 1000 as __suseconds_t) as i32;
            } else {
                r = 1000000000_i32;
                s = time(::core::ptr::null_mut::<time_t>());
                ns = 0;
            }
        }
        *resolution = r;
        file_timestamp_cons(
            ctx,
            ::core::ptr::null::<::core::ffi::c_char>(),
            system_time_from_unix(s as i64, ns as u32),
        )
    }

    /// Unpack the whole-seconds field the same way `file_timestamp_sprintf`
    /// does, so we can compare two packed timestamps in seconds-since-epoch.
    fn decode_secs(ts: uintmax_t) -> i64 {
        (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) as i64
    }

    /// Differential test: the new `std::time` `file_timestamp_now` must agree
    /// with the preserved unsafe clock cascade. Resolution must match exactly
    /// (both use the nanosecond CLOCK_REALTIME path => 1). The packed
    /// timestamps decode to within a small wall-clock tolerance because the two
    /// `now()` reads happen microseconds apart; we deliberately do not assert
    /// exact equality or subsec_nanos equality, which would be flaky.
    #[test]
    fn file_timestamp_now_matches_unsafe_oracle() {
        let ctx = crate::execctx::ExecContext::default();
        let (ts_new, res_new) = file_timestamp_now(&ctx);

        let mut res_oracle: i32 = -1;
        let ts_oracle = unsafe { file_timestamp_now_oracle(&ctx, &raw mut res_oracle) };

        assert_eq!(res_new, 1, "std path sets resolution to 1");
        assert_eq!(
            res_new, res_oracle,
            "resolution must match oracle (CLOCK_REALTIME ns path)"
        );

        assert_ne!(ts_new, 0, "packed timestamp is non-zero");
        assert_ne!(ts_oracle, 0, "oracle packed timestamp is non-zero");

        let decoded_new = decode_secs(ts_new);
        let decoded_oracle = decode_secs(ts_oracle);

        // Two separate now() reads occur microseconds apart; 2s is safe.
        assert!(
            (decoded_new - decoded_oracle).abs() <= 2,
            "new={decoded_new} oracle={decoded_oracle} differ by more than 2s"
        );
    }

    // FFI the verbatim `file_timestamp_sprintf` oracle depends on. Production
    // dropped the `localtime` global and the `sprintf` formatting when the
    // function moved to safe Rust over `localtime_r`, so we re-declare `sprintf`
    // here (test-only) to keep the original compilable as the differential
    // oracle (AGENTS.md "preserve the original as a test oracle"). `localtime`
    // and its broken-down-time struct come from `libc` rather than the old
    // c2rust-generated `struct tm` redefinition (issue #197).
    use crate::ffi_types::intmax_t;
    extern "C" {
        fn sprintf(__s: *mut ::core::ffi::c_char, __format: *const ::core::ffi::c_char, ...)
            -> i32;
    }

    /// Verbatim copy of the pre-std `file_timestamp_sprintf`: `localtime` +
    /// `sprintf` into the caller buffer, then the manual trailing-zero / dot
    /// trim. Kept test-only as the byte-for-byte oracle for the safe
    /// `file_timestamp_string`.
    unsafe fn file_timestamp_sprintf_unsafe_oracle(mut p: *mut ::core::ffi::c_char, ts: uintmax_t) {
        let mut t: time_t = (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
            as time_t;
        let tm: *mut libc::tm = libc::localtime(&raw mut t);
        if !tm.is_null() {
            let year: intmax_t = (*tm).tm_year as intmax_t;
            p = p.offset(sprintf(
                p,
                b"%04ld-%02d-%02d %02d:%02d:%02d\0" as *const u8 as *const ::core::ffi::c_char,
                year + 1900 as intmax_t,
                (*tm).tm_mon + 1,
                (*tm).tm_mday,
                (*tm).tm_hour,
                (*tm).tm_min,
                (*tm).tm_sec,
            ) as isize);
        } else if t < 0 as time_t {
            p = p.offset(sprintf(
                p,
                b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                t as intmax_t,
            ) as isize);
        } else {
            p = p.offset(sprintf(
                p,
                b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
                t as uintmax_t,
            ) as isize);
        }
        p = p.offset(
            (sprintf(
                p,
                b".%09d\0" as *const u8 as *const ::core::ffi::c_char,
                (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    & (((1) << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) - 1) as uintmax_t)
                    as i32,
            ) - 1) as isize,
        );
        while *p as i32 == '0' as i32 {
            p = p.offset(-1_i32 as isize);
        }
        p = p.offset((*p as i32 != '.' as i32) as i32 as isize);
        *p = 0;
    }

    /// Drive the verbatim oracle through a 43-byte buffer (the
    /// `FILE_TIMESTAMP_PRINT_LEN_BOUND` callers use) and read the result back.
    fn oracle_string(ts: uintmax_t) -> String {
        let mut buf = [0_i8; 43];
        unsafe { file_timestamp_sprintf_unsafe_oracle(buf.as_mut_ptr(), ts) };
        unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_str()
            .unwrap()
            .to_owned()
    }

    /// Differential test: the safe `file_timestamp_string` must produce
    /// byte-for-byte the same output as the preserved `localtime`/`sprintf`
    /// oracle across a representative spread of seconds and sub-second
    /// fractions (epoch, recent, far-future-in-range, and every fraction shape
    /// that exercises the trailing-zero / bare-dot trimming).
    #[test]
    fn file_timestamp_string_matches_unsafe_oracle() {
        let shift = if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 };
        let mask: uintmax_t = ((1u64 << shift) - 1) as uintmax_t;
        // Pack the same way the decoders read it back: high bits = seconds,
        // low `shift` bits = fraction, biased by ORDINARY_MTIME_MIN. Both the
        // safe fn and the oracle decode any `ts` identically, so equality of
        // their *outputs* is what we assert.
        let pack = |secs: u64, frac: u64| -> uintmax_t {
            (((secs << shift) | (frac & mask as u64)) as uintmax_t)
                .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
        };
        let secs_cases: [u64; 6] = [
            0,
            1,
            1_700_000_000,
            2_000_000_000,
            9_999_999_999,
            17_179_869_183,
        ];
        let frac_cases: [u64; 6] = [
            0,
            1,
            123_000_000,
            500_000_000,
            (1u64 << 30) - 1,
            999_999_999,
        ];
        for &s in &secs_cases {
            for &fr in &frac_cases {
                let ts = pack(s, fr);
                assert_eq!(
                    file_timestamp_string(ts),
                    oracle_string(ts),
                    "mismatch at secs={s} frac={fr} (ts={ts})"
                );
            }
        }
    }

    #[test]
    fn normalize_lookup_name_collapses_leading_dot_dirs() {
        unsafe {
            initialize_stopchar_map();

            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c"plain".as_ptr())).to_bytes(),
                b"plain"
            );
            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c"./".as_ptr())).to_bytes(),
                b"./"
            );
            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c".//".as_ptr())).to_bytes(),
                b"./"
            );
            assert_eq!(
                CStr::from_ptr(normalize_lookup_name(c"././src/file".as_ptr())).to_bytes(),
                b"src/file"
            );
        }
    }

    // Serialize the tests that touch the process-wide `files` hash table and
    // the file-graph globals so they never race each other.
    static FILE_GRAPH_LOCK: Mutex<()> = Mutex::new(());

    /// `snap_file` on a plain (non-target, variable-less) file just clears the
    /// `updating` flag and returns, since `all_secondary`/`no_intermediates`
    /// are unset by default. Drives that branch on a stack file.
    #[test]
    fn snap_file_plain_target_clears_updating() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            let ctx = crate::execctx::ExecContext::default();
            let mut f = File::default();
            f.set_updating(1);
            snap_file(&ctx, &raw mut f, ::core::ptr::null());
            assert_eq!(f.updating(), 0, "updating cleared when not 2nd-expanding");
        }
    }

    /// For a target file with no per-target variables, `snap_file` copies the
    /// `.EXTRA_PREREQS` dep chain (here a single prereq whose name matches the
    /// target, so the self-match break path runs and the copy is freed).
    #[test]
    fn snap_file_target_copies_extra_prereqs() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            let name = strcache_add(c"snapself".as_ptr());
            let mut f = File::default();
            f.name = name;
            f.hname = name;
            f.set_is_target(1);

            // A one-element prereq chain whose dep name equals the target name.
            let d = alloc_dep();
            (*d).name = name;
            (*d).next = ::core::ptr::null_mut();
            snap_file(&ctx, &raw mut f, d as *const Dep);
            // The self-referential prereq is dropped, so deps stays empty.
            assert!(f.deps.is_null(), "self-prereq is not appended");
            free_dep(d);
        }
    }

    /// `enter_prereqs(deps, NULL)` resolves each prerequisite to a file via
    /// `enter_file`, nulls the dep name, and (with a null stem) marks the
    /// entered file explicit. Drives the common no-pattern path.
    #[test]
    fn enter_prereqs_resolves_files_for_plain_deps() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();

            let nm = strcache_add(c"enter_prereqs_probe_target".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            let head = enter_prereqs(&ctx, d, ::core::ptr::null());
            assert_eq!(head, d, "the chain head is returned unchanged");
            // Name is consumed (replaced by the resolved file) and a file exists.
            assert!((*head).name.is_null(), "resolved dep name is cleared");
            assert!(!(*head).file.is_null(), "prereq resolved to a file");
            assert!(
                !lookup_file(&ctx, nm).is_null(),
                "the prerequisite file is now in the table"
            );
        }
    }

    /// `enter_prereqs(NULL, _)` is a no-op returning null.
    #[test]
    fn enter_prereqs_null_is_noop() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let ctx = crate::execctx::ExecContext::default();
            assert!(enter_prereqs(&ctx, ::core::ptr::null_mut(), ::core::ptr::null()).is_null());
        }
    }

    /// The file table is owned per-`ExecContext`, not a process global: a file
    /// entered in one context is found there but is invisible to an independent
    /// context. Guards against the table regressing to a `static mut`.
    #[test]
    fn file_table_is_per_context_not_global() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            initialize_stopchar_map();
            let a = crate::execctx::ExecContext::default();
            let b = crate::execctx::ExecContext::default();
            let nm = strcache_add(c"per_ctx_probe_target".as_ptr());

            let f = enter_file(&a, nm);
            assert!(!f.is_null(), "file is entered into context a");
            assert_eq!(lookup_file(&a, nm), f, "and found again in context a");
            assert!(
                lookup_file(&b, nm).is_null(),
                "an independent context shares no global file table"
            );
        }
    }

    /// With a non-null stem, `enter_prereqs` walks the static-pattern block. A
    /// prerequisite name with no `%` finds no percent, so it keeps its name but
    /// is tagged with the stem and `staticpattern`, then resolved to a file.
    /// This exercises the stem branch without touching the variable buffer.
    #[test]
    fn enter_prereqs_static_pattern_without_percent() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();

            let nm = strcache_add(c"enter_prereqs_static_probe".as_ptr());
            let stem = strcache_add(c"thestem".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            let head = enter_prereqs(&ctx, d, stem);
            assert_eq!(head, d);
            // The dep was tagged with the stem (staticpattern path ran) and then
            // resolved: name cleared, file entered, staticpattern reset to 0.
            assert_eq!((*head).stem, stem, "stem recorded on the static pattern");
            assert!((*head).name.is_null(), "resolved dep name is cleared");
            assert!(!(*head).file.is_null(), "prereq resolved to a file");
            assert_eq!(
                (*head).staticpattern(),
                0,
                "staticpattern is reset after resolution"
            );
        }
    }

    /// The pattern-substitution arm of the stem branch: a prerequisite name
    /// containing `%` is expanded against the stem via the variable buffer
    /// (e.g. `%.o` with stem `epp_stem` -> `epp_stem.o`). Holds both the
    /// file-graph and variable-buffer locks (only this test needs both).
    #[test]
    fn enter_prereqs_static_pattern_substitutes_percent() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _b = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            crate::expand::initialize_variable_output();

            let nm = strcache_add(c"%.o".as_ptr());
            let stem = strcache_add(c"epp_stem".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            let head = enter_prereqs(&ctx, d, stem);
            assert_eq!(head, d);
            // `%` expanded to the stem and the dep resolved to a file named
            // "epp_stem.o"; the dep name itself is cleared after resolution.
            assert!((*head).name.is_null(), "resolved dep name is cleared");
            assert!(
                !lookup_file(&ctx, strcache_add(c"epp_stem.o".as_ptr())).is_null(),
                "the expanded prerequisite file was entered"
            );
        }
    }

    /// When a `%` prerequisite expands to the empty string (a bare `%` with an
    /// empty stem: the percent is dropped and nothing remains), `enter_prereqs`
    /// removes that prerequisite from the chain and frees it. With a single
    /// such dep the chain collapses to empty.
    #[test]
    fn enter_prereqs_drops_prereq_that_expands_empty() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _b = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            crate::expand::initialize_variable_output();

            let nm = strcache_add(c"%".as_ptr());
            let stem = strcache_add(c"".as_ptr());
            let d = alloc_dep();
            (*d).name = nm;
            (*d).next = ::core::ptr::null_mut();

            // The bare `%` with an empty stem expands to "", so the dep is
            // dropped and freed; the returned chain is empty.
            let head = enter_prereqs(&ctx, d, stem);
            assert!(head.is_null(), "the empty-expanding prereq is removed");
        }
    }

    /// `file_timestamp_cons` packs an in-range `(seconds, nanoseconds)` pair
    /// into a `FILE_TIMESTAMP`. Two ordinary stamps round-trip without the
    /// out-of-range substitution, and a later second yields a strictly larger
    /// encoded timestamp than an earlier one (ordering, not absolute value).
    #[test]
    fn file_timestamp_cons_in_range_is_monotonic() {
        unsafe {
            let ctx = crate::execctx::ExecContext::default();
            let earlier = file_timestamp_cons(
                &ctx,
                c"probe_a".as_ptr(),
                system_time_from_unix(1_000_000, 0),
            );
            let later = file_timestamp_cons(
                &ctx,
                c"probe_b".as_ptr(),
                system_time_from_unix(1_000_001, 0),
            );
            assert!(
                later > earlier,
                "a later second encodes to a larger timestamp ({later} > {earlier})"
            );
            // Both land in the ordinary range, above the reserved sentinels.
            assert!(earlier > ORDINARY_MTIME_MIN as uintmax_t);
            // The nanosecond component widens the value within the same second.
            let with_ns = file_timestamp_cons(
                &ctx,
                c"probe_a".as_ptr(),
                system_time_from_unix(1_000_000, 500_000_000),
            );
            assert!(
                with_ns > earlier,
                "added nanoseconds raise the encoded timestamp within a second"
            );
        }
    }

    /// Serializes the tests that drive the real `error()` output path, which
    /// reads the process-global `program`/`makelevel`.
    static TIMESTAMP_ERR_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// A stamp below the encodable range (`s <= OLD_MTIME`) drives the
    /// out-of-range substitution branch: it formats the clamped timestamp and
    /// calls `error()` ("timestamp out of range: substituting"), then returns
    /// the substituted value `ORDINARY_MTIME_MIN`. Driving this requires a
    /// valid `program` name so `error()` does not dereference a null pointer.
    #[test]
    fn file_timestamp_cons_low_out_of_range_substitutes() {
        let _g = TIMESTAMP_ERR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            crate::make_main::install_program_name_for_test();
            let ctx = crate::execctx::ExecContext::default();
            // s = 0 <= OLD_MTIME (2): below the encodable range.
            let ts = file_timestamp_cons(&ctx, c"too_old".as_ptr(), system_time_from_unix(0, 0));
            assert_eq!(
                ts, ORDINARY_MTIME_MIN as uintmax_t,
                "an underflowing stamp is substituted with ORDINARY_MTIME_MIN"
            );
        }
    }

    /// A stamp above the encodable range drives the same out-of-range
    /// substitution `error()` branch but takes the upper clamp (the `else` arm
    /// of the `s <= OLD_MTIME` selection). A null `fname` exercises the
    /// "Current time" default label inside that branch.
    #[test]
    fn file_timestamp_cons_high_out_of_range_substitutes() {
        let _g = TIMESTAMP_ERR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            crate::make_main::install_default_options_for_test();
            crate::make_main::install_program_name_for_test();
            // A stamp near time_t::MAX overflows the 30-bit left shift, so it
            // is above the encodable range and clamps to the upper bound.
            let ctx = crate::execctx::ExecContext::default();
            let ts = file_timestamp_cons(
                &ctx,
                ::core::ptr::null::<::core::ffi::c_char>(),
                system_time_from_unix(i64::MAX, 0),
            );
            assert!(
                ts > ORDINARY_MTIME_MIN as uintmax_t,
                "an overflowing stamp clamps to the upper ordinary bound"
            );
        }
    }

    /// Verbatim copy of the pre-extraction `file_timestamp_cons` packing and
    /// range-check arithmetic (value only — the `error()` side effect is
    /// dropped). Kept test-only as the differential oracle for the safe
    /// `pack_file_timestamp` (AGENTS.md "preserve the original as a test
    /// oracle"). Returns `Ok(ts)` in range, `Err(substitute)` out of range, so
    /// it can be compared field-for-field with the new `Result`.
    fn pack_file_timestamp_oracle(
        stamp: time_t,
        ns: ::core::ffi::c_long,
    ) -> Result<uintmax_t, uintmax_t> {
        let offset: i32 = (ORDINARY_MTIME_MIN as ::core::ffi::c_long
            + (if FILE_TIMESTAMP_HI_RES != 0 { ns } else { 0 })) as i32;
        let s: uintmax_t = stamp as uintmax_t;
        let product: uintmax_t = s << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 });
        let ts: uintmax_t = product.wrapping_add(offset as uintmax_t);
        if !(s
            <= ((!(0_i32 as uintmax_t))
                .wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                    0_i32 as uintmax_t
                } else {
                    !(0_i32 as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(8_usize)
                            .wrapping_sub(1_usize)
                })
                .wrapping_sub((2 + 1) as uintmax_t)
                >> (if 1 != 0 { 30 } else { 0 })
                << (if 1 != 0 { 30 } else { 0 }))
            .wrapping_add((2 + 1) as uintmax_t)
            .wrapping_add((if 1 != 0 { 1000000000_i32 } else { 1 }) as uintmax_t)
            .wrapping_sub(1 as uintmax_t)
            .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
            && product <= ts
            && ts
                <= ((!(0_i32 as uintmax_t))
                    .wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                        0_i32 as uintmax_t
                    } else {
                        !(0_i32 as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8_usize)
                                .wrapping_sub(1_usize)
                    })
                    .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
                    << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
                .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
                .wrapping_add(
                    (if FILE_TIMESTAMP_HI_RES != 0 {
                        1000000000_i32
                    } else {
                        1
                    }) as uintmax_t,
                )
                .wrapping_sub(1 as uintmax_t))
        {
            let substitute = if s <= OLD_MTIME as uintmax_t {
                ORDINARY_MTIME_MIN as uintmax_t
            } else {
                ((!(0_i32 as uintmax_t))
                    .wrapping_sub(if !(-1_i32 as uintmax_t <= 0 as uintmax_t) {
                        0_i32 as uintmax_t
                    } else {
                        !(0_i32 as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8_usize)
                                .wrapping_sub(1_usize)
                    })
                    .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                    >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
                    << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
                .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
                .wrapping_add(
                    (if FILE_TIMESTAMP_HI_RES != 0 {
                        1000000000_i32
                    } else {
                        1
                    }) as uintmax_t,
                )
                .wrapping_sub(1 as uintmax_t)
            };
            Err(substitute)
        } else {
            Ok(ts)
        }
    }

    /// Differential test: the clean `pack_file_timestamp` must agree exactly
    /// with the verbatim c2rust arithmetic across the whole-seconds boundaries
    /// (underflow, ordinary range, the 30-bit shift overflow, and `time_t`
    /// extremes) crossed with representative sub-second values.
    #[test]
    fn pack_file_timestamp_matches_verbatim_oracle() {
        let stamps: [time_t; 12] = [
            time_t::MIN,
            -1_000_000,
            -1,
            0,
            1,
            2,
            3,
            1_700_000_000,
            (1 << 33) - 1,
            1 << 33,
            ::core::ffi::c_long::MAX as time_t,
            time_t::MAX,
        ];
        let nss: [::core::ffi::c_long; 5] = [0, 1, 500_000_000, 999_999_999, 1_000_000_000];
        for &stamp in &stamps {
            for &ns in &nss {
                assert_eq!(
                    pack_unix_timestamp(stamp as i64, ns as i64),
                    pack_file_timestamp_oracle(stamp, ns),
                    "diverged at stamp={stamp}, ns={ns}"
                );
            }
        }
    }

    /// The `SystemTime` front door (`pack_file_timestamp`) must agree with the
    /// integer core for every value `SystemTime` can faithfully represent: a
    /// `(secs, nsec)` pair built via `system_time_from_unix` and decomposed back
    /// round-trips, so the packed result matches `pack_unix_timestamp` directly.
    #[test]
    fn pack_file_timestamp_systemtime_matches_int_core() {
        let cases: [(i64, u32); 6] = [
            (0, 0),
            (1, 1),
            (1_700_000_000, 500_000_000),
            (1_700_000_000, 999_999_999),
            (4_000_000_000, 0),
            (i64::MAX, 0),
        ];
        for &(secs, nsec) in &cases {
            let t = system_time_from_unix(secs, nsec);
            assert_eq!(
                pack_file_timestamp(t),
                pack_unix_timestamp(secs, nsec as i64),
                "SystemTime path diverged at secs={secs}, nsec={nsec}"
            );
        }
    }

    /// The byte-level name normalizer matches `normalize_lookup_name`'s leading
    /// `./` collapse on the cases that function's own test exercises.
    #[test]
    fn normalize_lookup_name_bytes_collapses_leading_dot_dirs() {
        initialize_stopchar_map();
        assert_eq!(normalize_lookup_name_bytes(b"foo.o"), b"foo.o");
        assert_eq!(normalize_lookup_name_bytes(b"./foo.o"), b"foo.o");
        assert_eq!(normalize_lookup_name_bytes(b".///foo.o"), b"foo.o");
        assert_eq!(normalize_lookup_name_bytes(b"././foo.o"), b"foo.o");
        // An all-"./" name canonicalizes to "./", and "." / "./" are preserved.
        assert_eq!(normalize_lookup_name_bytes(b"././"), b"./");
        assert_eq!(normalize_lookup_name_bytes(b"./"), b"./");
        assert_eq!(normalize_lookup_name_bytes(b"."), b".");
        // A bare directory separator run is not a leading "./" and is left alone.
        assert_eq!(normalize_lookup_name_bytes(b"sub/foo.o"), b"sub/foo.o");
    }

    /// `enter_filenode` interns a fresh node and is idempotent on the name;
    /// `lookup_filenode` finds it (after normalization) and misses otherwise.
    #[test]
    fn enter_and_lookup_filenode_round_trip_on_the_arena() {
        initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();

        assert!(lookup_filenode(&ctx, b"foo.o").is_none());

        let id = enter_filenode(&ctx, b"foo.o");
        assert_eq!(ctx.filenodes.len(), 1);
        // Re-entering the same (and the "./"-prefixed) name reuses the head.
        assert_eq!(enter_filenode(&ctx, b"foo.o"), id);
        assert_eq!(enter_filenode(&ctx, b"./foo.o"), id);
        assert_eq!(ctx.filenodes.len(), 1);

        assert_eq!(lookup_filenode(&ctx, b"foo.o"), Some(id));
        assert_eq!(lookup_filenode(&ctx, b"./foo.o"), Some(id));
        assert!(lookup_filenode(&ctx, b"bar.o").is_none());

        let node = ctx.filenodes.get(id).expect("interned");
        assert_eq!(node.lock().unwrap().name, b"foo.o");
    }

    /// Double-colon entries live inline on the head (they share its name, so
    /// they cannot be distinct name-derived `FileId`s) and the head is marked.
    #[test]
    fn double_colon_entries_are_inline_on_the_head() {
        initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();

        let id = enter_filenode(&ctx, b"all");
        // First `::` rule marks the head (it is the first entry); the next two
        // append inline entries.
        push_double_colon_entry(&ctx, id);
        push_double_colon_entry(&ctx, id);
        push_double_colon_entry(&ctx, id);

        // Still one arena entry (the head); the chain lives inside it.
        assert_eq!(ctx.filenodes.len(), 1);
        let node = ctx.filenodes.get(id).expect("interned");
        let n = node.lock().unwrap();
        assert!(n.is_double_colon);
        assert_eq!(n.double_colon.len(), 2);
        assert!(n.double_colon.iter().all(|e| e.name == b"all"));
    }

    /// The idiomatic `Recipe` defaults to a TAB introducer and an empty recipe,
    /// its line flags match the c2rust `COMMANDS_*` byte values, and a fresh
    /// `FileNode` carries no recipe.
    #[test]
    fn recipe_defaults_and_line_flags_match_c_constants() {
        let r = Recipe::default();
        assert_eq!(r.recipe_prefix, b'\t');
        assert!(r.text.is_empty() && r.lines.is_empty() && !r.any_recurse);
        assert_eq!(r.defined_in, None);

        // Flag values round-trip with the c2rust lines_flags byte.
        assert_eq!(
            RecipeLineFlags::RECURSE.bits() as i32,
            crate::commands::COMMANDS_RECURSE
        );
        assert_eq!(RecipeLineFlags::SILENT.bits() as i32, COMMANDS_SILENT);
        assert_eq!(RecipeLineFlags::NOERROR.bits() as i32, COMMANDS_NOERROR);

        let line = RecipeLine {
            text: b"echo hi".to_vec(),
            flags: RecipeLineFlags::SILENT | RecipeLineFlags::NOERROR,
        };
        assert!(line.flags.contains(RecipeLineFlags::SILENT));
        assert!(!line.flags.contains(RecipeLineFlags::RECURSE));

        // A fresh file has no recipe (the former null `cmds`).
        assert_eq!(FileNode::new(b"x".to_vec()).recipe, None);
    }

    /// The idiomatic variable enums' discriminants match the c2rust
    /// flavor/origin/export constants, and a fresh `FileNode` has no per-target
    /// or pattern variables.
    #[test]
    fn target_variable_enums_match_c_constants() {
        use crate::variable::{
            f_append, f_append_value, f_bogus, f_expand, f_recursive, f_shell, f_simple,
            o_automatic, o_command, o_default, o_env, o_env_override, o_file, o_invalid,
            o_override, v_default, v_export, v_ifset, v_noexport,
        };

        assert_eq!(VarFlavor::Bogus as u32, f_bogus);
        assert_eq!(VarFlavor::Simple as u32, f_simple);
        assert_eq!(VarFlavor::Recursive as u32, f_recursive);
        assert_eq!(VarFlavor::Expand as u32, f_expand);
        assert_eq!(VarFlavor::Append as u32, f_append);
        assert_eq!(VarFlavor::Shell as u32, f_shell);
        assert_eq!(VarFlavor::AppendValue as u32, f_append_value);

        assert_eq!(VarOrigin::Default as u32, o_default);
        assert_eq!(VarOrigin::Environment as u32, o_env);
        assert_eq!(VarOrigin::File as u32, o_file);
        assert_eq!(VarOrigin::EnvOverride as u32, o_env_override);
        assert_eq!(VarOrigin::Command as u32, o_command);
        assert_eq!(VarOrigin::Override as u32, o_override);
        assert_eq!(VarOrigin::Automatic as u32, o_automatic);
        assert_eq!(VarOrigin::Invalid as u32, o_invalid);

        assert_eq!(VarExport::Default as u32, v_default);
        assert_eq!(VarExport::Export as u32, v_export);
        assert_eq!(VarExport::NoExport as u32, v_noexport);
        assert_eq!(VarExport::IfSet as u32, v_ifset);

        // Defaults and a fresh file's (empty) variable sets.
        assert_eq!(VarFlavor::default(), VarFlavor::Bogus);
        assert_eq!(VarOrigin::default(), VarOrigin::Default);
        let f = FileNode::new(b"t".to_vec());
        assert!(f.variables.is_empty() && f.pat_variables.is_empty());
    }

    /// `FileId` is byte-exact: names that differ only outside valid UTF-8 stay
    /// distinct (the raw-pointer table keyed by bytes; the arena must too).
    #[test]
    fn file_id_from_bytes_is_byte_exact() {
        assert_ne!(
            FileId::from_bytes(&[0xff, 0x01]),
            FileId::from_bytes(&[0xff, 0x02])
        );
        assert_eq!(
            FileId::from_bytes(b"foo.o"),
            FileId::from_bytes(b"foo.o")
        );
    }
}
