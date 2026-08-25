// The idiomatic dependency-graph types live in `crate::dep` (pointer-free);
// re-export so `crate::file::DepNode` paths keep resolving. The legacy C-ABI
// `Dep`/`GoalDep` records stay below with the other c2rust FFI structs until
// the `*mut`-to-handle swap deletes them.
use crate::content_hash::ContentHash;
pub use crate::dep::{DepFlags, DepId, DepNode, GoalDepId, GoalDepNode};
// The recipe types (idiomatic replacement for the c2rust `Commands`) live in
// `crate::recipe`; re-export so `crate::file::Recipe` paths keep resolving.
pub use crate::recipe::{Recipe, RecipeLine, RecipeLineFlags};
// Per-target variable types live in `crate::target_var`; re-export so
// `crate::file::TargetVariable` / `VarFlavor` paths keep resolving.
pub use crate::target_var::{TargetVariable, VarExport, VarFlavor, VarOrigin};

pub use crate::ffi_types::{
    __clockid_t,
    __off64_t,
    __off_t,
    __suseconds_t,
    __syscall_slong_t,
    __time_t,
    clockid_t,
    intmax_t,
    size_t,
    time_t,
    uintmax_t,
};
#[cfg(test)]
use std::ffi::CStr;
use {
    crate::misc::{xcalloc, xrealloc},
    libc::{__errno_location, free},
    std::{
        ffi::CString,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
};
extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
// The legacy c2rust variable-scope containers `VariableSet` / `VariableSetList`
// now live in their domain module (`crate::variable`). Re-exported here so
// existing `crate::file::VariableSet` / `VariableSetList` paths — and the
// `variable_set` / `variable_set_list` aliases — keep resolving.
pub use crate::variable::{VariableSet, VariableSetList};
pub type HashTable = crate::hash::HashTable;
pub type hash_cmp_func_t = crate::hash::hash_cmp_func_t;
pub type hash_func_t = crate::hash::hash_func_t;

pub(crate) const HASH_SIZE: usize = 32;

// `FileId[HASH_SIZE] <- FileNode` also derives `impl From<&FileNode> for
// FileId`, a content-hash of the *whole* node (blake3), mirroring
// `DepId <- DepNode` (dep.rs). That conversion is scaffolding, not the
// graph's live identity path: the arena keys nodes by `FileNode::id()`
// (`FileId::from_bytes(&self.hname)`), derived from the canonical name only,
// so mutable runtime state (timestamps, flags, command state) does not
// contribute to a file's identity and it survives updates.
crate::id_wireformat!(FileId[HASH_SIZE] <- FileNode);

// The legacy c2rust dependency-edge records `Dep`/`GoalDep` and their base
// name-chain sibling `NameSeq` now live in their domain module (`crate::dep`),
// next to the idiomatic `DepNode` they migrate to. Re-exported here so existing
// `crate::file::Dep` / `GoalDep` / `NameSeq` paths (and the `nameseq` alias)
// keep resolving during the `*mut`-to-handle swap.
pub use crate::dep::{Dep, GoalDep, NameSeq};

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
#[derive(Debug, Clone, ContentHash)]
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
    /// Provenance: the pattern rule whose match supplied this target's
    /// deps/recipe (set by `pattern_search` when it commits a match; `None`
    /// for explicit targets). Keyed by the rule's semantic content hash, so
    /// it stays valid across rule-database reordering. `depgraph` turns this
    /// into a `DerivedBy` edge.
    pub matched_rule: Option<crate::rule::RuleId>,
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
            matched_rule: None,
            last_mtime: 0,
            mtime_before_update: 0,
            considered: 0,
            command_flags: 0,
            // A freshly-interned node has not been remade yet: start in
            // `us_none` (UpdateStatus::None), matching the legacy
            // `File::new_named`. `remove_intermediates` keys unlinking off
            // `update_status != us_none`, so defaulting to `Success` here would
            // let cleanup unlink an intermediate that was only entered/discovered
            // and never actually updated.
            update_status: UpdateStatus::None,
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
// The legacy c2rust `Commands` record now lives in its domain module
// (`crate::recipe`), next to the idiomatic `Recipe` that replaces it.
// Re-exported here so existing `crate::file::Commands` paths — and the
// `pub type commands = Commands` alias below — keep resolving.
pub use crate::recipe::Commands;
use crate::{
    commands::{print_commands, set_file_variables},
    entry::{db_level, second_expansion, stopchar_map, with_options, MAP_DIRSEP},
    expand::{expand_string_buf, expand_string_for_file, variable_buffer_output},
    floc::Floc,
    function::patsubst_expand_pat,
    output::{error, fatal_err, perror_with_name, FmtArg},
    read::{find_percent, parse_file_seq},
    variable::{
        initialize_file_variables,
        lookup_variable,
        print_file_variables,
        print_target_variables,
    },
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
#[derive(Copy, Clone, PartialEq, Eq, Debug, ContentHash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Debug, ContentHash)]
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
    #[cfg(test)]
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

pub const UNKNOWN_MTIME: i32 = 0;
pub const NONEXISTENT_MTIME: i32 = 1;
pub const OLD_MTIME: i32 = 2;
pub const ORDINARY_MTIME_MIN: i32 = OLD_MTIME + 1;
// The file store lives on `ExecContext` (`ctx.filenodes`, the `FileId`-keyed
// arena of `Arc<Mutex<FileNode>>`); the former `static mut files` gnulib
// `HashTable` and its `file_hash_1`/`file_hash_2`/`file_hash_cmp` callbacks —
// and the interim raw-pointer `FileTable` — are gone.

fn stop_set_byte(c: u8, mask: i32) -> bool {
    stopchar_map()[c as usize] as i32 & mask != 0
}

#[cfg(test)]
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
/// Look up a file by `name`, returning its head [`FileId`] if it is interned.
///
/// The pointer-free port of the c2rust `lookup_file`: it normalizes the leading
/// `./` segments off `name` (via [`normalize_lookup_name_bytes`]) and reports
/// the head's `FileId` if the arena (`ctx.filenodes`) holds it. An absent key is
/// the former "no real item in the slot" (null) result, here `None`. No raw
/// pointers, no `c_char`, no `unsafe`.
pub fn lookup_file(ctx: &crate::execctx::ExecContext, name: &[u8]) -> Option<FileId> {
    let key = normalize_lookup_name_bytes(name);
    let id = FileId::from_bytes(key);
    ctx.filenodes.get(id).map(|_| id)
}

/// Enter `name` into the file store, returning its head [`FileId`] and interning
/// a fresh [`FileNode`] if it is new.
///
/// The pointer-free port of the c2rust `enter_file`. The arena stores each
/// name's chain *head* keyed by its name-derived `FileId`; double-colon (`::`)
/// entries live inline on the head's `double_colon` vec (not as separate
/// table slots), so a single arena node represents the whole target. Like the
/// c2rust single-colon path, an existing non-double-colon head is reused and its
/// `builtin` mark cleared; an existing double-colon head appends a fresh inline
/// entry. No raw pointers, no `c_char`, no `unsafe`.
pub fn enter_file(ctx: &crate::execctx::ExecContext, name: &[u8]) -> FileId {
    assert!(!name.is_empty(), "assertion failed: *name != '\\0'");
    let key = normalize_lookup_name_bytes(name);
    let id = FileId::from_bytes(key);
    // Store the raw key bytes verbatim (no lossy `String`), so `node.id()`
    // equals `id` even for names that are not valid UTF-8.
    let node = ctx
        .filenodes
        .get_or_insert_with(id, || FileNode::new(key.to_vec()));
    let mut n = node.lock().expect("file node lock poisoned");
    if n.is_double_colon {
        // Existing double-colon head: append a new inline entry to its chain.
        let entry = FileNode::new(n.name.clone());
        n.double_colon.push(entry);
    } else {
        // Brand-new or existing single-colon head: reuse it.
        n.builtin = false;
    }
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

/// Byte-exact, allocation-light port of the name normalizer: collapse any
/// leading `./` (or `.//`, `././`, …) segments. An all-`./` name canonicalizes
/// to `"./"`. Operates on the raw name bytes (no NUL, no `c_char`) and returns
/// the canonical key bytes, so it is usable from safe code.
pub(crate) fn normalize_lookup_name_bytes(name: &[u8]) -> &[u8] {
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

/// Re-key the file `from_id` under the new hash-name `to_hname` in the file
/// store, merging into any existing destination node — the pointer-free port of
/// the c2rust `rehash_file`.
///
/// In the c2rust graph this rewrote the `hname` field, walked the `renamed`
/// chain, removed/re-inserted the raw-pointer table slot, and merged
/// deps/recipe/variables/flags into the destination `file`. Here the arena is
/// keyed by the name-derived [`FileId`], so re-keying means: walk the `renamed`
/// links to the live node, remove it under its old `FileId`, set its `hname`
/// (and that of every inline double-colon entry) to `to_hname`, then either
/// re-insert it under the new `FileId` (destination free) or merge its contents
/// into the existing destination node and mark it `renamed`.
///
/// Locking discipline: a `FileNode` guard is never held across a call that
/// re-enters the arena. The `renamed` walk collects `FileId`s into a `Vec`
/// first; each node is locked, the needed values copied out, and the guard
/// dropped before the next arena access.
pub fn rehash_file(ctx: &crate::execctx::ExecContext, from_id: FileId, to_hname: &[u8]) {
    let to_id = FileId::from_bytes(to_hname);

    // Clear the `builtin` mark and read the starting hash-name.
    let from_hname = {
        let node = match ctx.filenodes.get(from_id) {
            Some(n) => n,
            None => return,
        };
        let mut n = node.lock().expect("file node lock poisoned");
        n.builtin = false;
        n.hname.clone()
    };

    // Already keyed under `to_hname`? Nothing to rehash.
    if from_hname == to_hname {
        return;
    }

    // Walk the `renamed` links to the live node, collecting the chain into a
    // `Vec<FileId>` first so no guard is held across an arena lookup.
    let mut walked_id = from_id;
    loop {
        let next = {
            let node = match ctx.filenodes.get(walked_id) {
                Some(n) => n,
                None => return,
            };
            let n = node.lock().expect("file node lock poisoned");
            n.renamed
        };
        match next {
            Some(next_id) => walked_id = next_id,
            None => break,
        }
    }

    // The walked node must still carry the original hash-name (the invariant
    // the c2rust translation asserted with `abort()`).
    let walked = match ctx.filenodes.get(walked_id) {
        Some(n) => n,
        None => return,
    };
    {
        let n = walked.lock().expect("file node lock poisoned");
        assert!(
            n.hname == from_hname,
            "rehash_file: walked hash-name diverged from the original"
        );
    }

    // Remove the walked node from the store under its current `FileId` and set
    // its (and its inline double-colon entries') hash-name to `to_hname`.
    let from_node = match ctx
        .filenodes
        .0
        .lock()
        .expect("file arena poisoned")
        .remove(&walked_id)
    {
        Some(n) => n,
        None => return,
    };
    {
        let mut n = from_node.lock().expect("file node lock poisoned");
        n.hname = to_hname.to_vec();
        for entry in &mut n.double_colon {
            entry.hname = to_hname.to_vec();
        }
    }

    // Destination already present?
    let to_node = ctx.filenodes.get(to_id);
    let Some(to_node) = to_node else {
        // Destination name was free: the rehashed node takes the new key.
        ctx.filenodes
            .0
            .lock()
            .expect("file arena poisoned")
            .insert(to_id, from_node);
        return;
    };

    // Merge `from_node`'s contents into the existing destination node. The two
    // guards are taken in a fixed order (destination then source) and both are
    // dropped together at the end of this block; neither is held across an
    // arena lookup.
    let mut to = to_node.lock().expect("file node lock poisoned");
    let mut from = from_node.lock().expect("file node lock poisoned");
    merge_rehashed_node(&mut to, &mut from, to_id);
    drop(from);
    drop(to);

    // The destination node (`to_node`) was never removed from the arena, so it
    // is already keyed under `to_id` with the merged contents. Re-insert the
    // (now-emptied, renamed) source node under the key it was removed from
    // (`walked_id`) so the `renamed` chain stays reachable: a later
    // `lookup_file`/`rehash_file` that lands on this waypoint follows its
    // `renamed` link to the destination, exactly as the c2rust graph followed
    // the `renamed` pointer.
    ctx.filenodes
        .0
        .lock()
        .expect("file arena poisoned")
        .insert(walked_id, from_node);
}

/// Merge the rehashed source node `from` into the destination node `to` (both
/// already locked by [`rehash_file`]), then mark `from` renamed to `to_id`.
/// Extracted from `rehash_file` so the merge's branchy flag/recipe/dep folding
/// lives in its own function.
fn merge_rehashed_node(to: &mut FileNode, from: &mut FileNode, to_id: FileId) {
    if from.recipe.is_some() {
        if to.recipe.is_none() {
            to.recipe = from.recipe.take();
        } else if to.recipe != from.recipe {
            // c2rust emitted a chain of "recipe was specified … will be ignored"
            // diagnostics here; the diagnostic layer is slice 2+. The destination
            // recipe wins, matching the c2rust behaviour.
        }
    }

    // Append the source deps onto the destination's.
    let from_deps = ::core::mem::take(&mut from.deps);
    to.deps.extend(from_deps);

    // Per-target variables: append the source set onto the destination's
    // (the idiomatic stand-in for `merge_variable_set_lists`).
    let from_vars = ::core::mem::take(&mut from.variables);
    to.variables.extend(from_vars);

    if to.is_double_colon && from.is_target && !from.is_double_colon {
        panic!("can't rename single-colon to double-colon");
    }
    if !to.is_double_colon && from.is_double_colon {
        if to.is_target {
            panic!("can't rename double-colon to single-colon");
        } else {
            to.is_double_colon = true;
            let from_dc = ::core::mem::take(&mut from.double_colon);
            to.double_colon = from_dc;
        }
    }

    if from.last_mtime > to.last_mtime {
        to.last_mtime = from.last_mtime;
    }
    to.mtime_before_update = from.mtime_before_update;
    to.precious |= from.precious;
    to.loaded |= from.loaded;
    to.tried_implicit |= from.tried_implicit;
    to.updating |= from.updating;
    to.updated |= from.updated;
    to.is_target |= from.is_target;
    to.cmd_target |= from.cmd_target;
    to.phony |= from.phony;
    to.is_explicit |= from.is_explicit;
    to.secondary |= from.secondary;
    to.notintermediate |= from.notintermediate;
    to.ignore_vpath |= from.ignore_vpath;
    to.snapped |= from.snapped;
    to.suffix |= from.suffix;
    to.builtin = false;

    // Mark the source as renamed to the destination.
    from.renamed = Some(to_id);
}

/// Rename the file `from_id` to `to_hname` — the pointer-free port of the
/// c2rust `rename_file`.
///
/// This rehashes the file (see [`rehash_file`]) and then sets each entry's
/// `name` equal to its (now-updated) `hname`. In the c2rust graph the second
/// step walked the `prev` double-colon chain; here every double-colon entry is
/// inline on the node, so a single locked pass over the node and its
/// `double_colon` vec suffices. The rehash drops every guard before this runs,
/// so no two `FileNode` guards are held at once.
pub fn rename_file(ctx: &crate::execctx::ExecContext, from_id: FileId, to_hname: &[u8]) {
    rehash_file(ctx, from_id, to_hname);
    // After rehashing, the node lives under the destination `FileId`; sync each
    // entry's `name` to its hash-name there.
    let to_id = FileId::from_bytes(to_hname);
    if let Some(node) = ctx.filenodes.get(to_id) {
        let mut n = node.lock().expect("file node lock poisoned");
        n.name = n.hname.clone();
        n.double_colon
            .iter_mut()
            .for_each(|entry| entry.name = entry.hname.clone());
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn remove_intermediates(ctx: &crate::execctx::ExecContext, sig: i32) {
    let mut doneany: i32 = 0;
    if crate::entry::opt_question(ctx)
        || crate::entry::opt_touch(ctx)
        || ctx.all_secondary.get()
        || ctx.no_intermediates.get()
    {
        return;
    }
    if sig != 0 && crate::entry::opt_just_print(ctx) {
        return;
    }
    // Snapshot the intermediate candidates from the arena: lock the map, grab the
    // `Arc` handles, release the map lock, then read each node's flags/name under
    // its own lock and drop the guard before acting. The former global raw table
    // is gone, so the async-signal race the c2rust `try_borrow` guarded against no
    // longer applies; the arena's own `Mutex` serialises access.
    let nodes: Vec<::std::sync::Arc<::std::sync::Mutex<FileNode>>> = {
        let Ok(map) = ctx.filenodes.0.lock() else {
            return;
        };
        map.values().map(::std::sync::Arc::clone).collect()
    };
    for node in nodes {
        // Copy out the flags and name under the node lock, then drop the guard so
        // the FFI calls below never run while holding it.
        let (
            intermediate,
            dontcare,
            precious,
            secondary,
            notintermediate,
            cmd_target,
            status_none,
            name,
        ) = {
            let n = node.lock().expect("file node lock poisoned");
            (
                n.intermediate,
                n.dontcare,
                n.precious,
                n.secondary,
                n.notintermediate,
                n.cmd_target,
                n.update_status != us_none,
                n.name.clone(),
            )
        };
        if intermediate && (dontcare || !precious) && !secondary && !notintermediate && !cmd_target
        {
            let status: i32;
            if status_none {
                // NUL-terminate the name for the C FFI calls below.
                let mut cname = name.clone();
                cname.push(0);
                let cname_ptr = cname.as_ptr() as *const ::core::ffi::c_char;
                // ENOENT from unlink means the file was already gone: skip the
                // diagnostic/bookkeeping below (the C code `continue`d here).
                let skip: bool;
                if crate::entry::opt_just_print(ctx) {
                    status = 0;
                    skip = false;
                } else {
                    status = crate::misc::unlink_c(cname_ptr);
                    skip = status < 0 && *__errno_location() == ENOENT;
                }
                if !skip && !dontcare {
                    if sig != 0 {
                        error(
                            ctx,
                            ::core::ptr::null_mut::<Floc>(),
                            name.len() as size_t,
                            b"*** deleting intermediate file '%s'\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[FmtArg::Str(cname_ptr)],
                        );
                    } else {
                        if doneany == 0 && 0x1_i32 & db_level(ctx) != 0 {
                            crate::output::trace_out(b"Removing intermediate files...\n");
                        }
                        if !crate::entry::opt_run_silent(ctx) {
                            if doneany == 0 {
                                crate::output::trace_out(b"rm ");
                                doneany = 1;
                            } else {
                                crate::output::trace_out(b" ");
                            }
                            crate::output::trace_out(&name);
                        }
                    }
                    if status < 0 {
                        if doneany != 0 {
                            crate::output::trace_out(b"\n");
                        }
                        perror_with_name(
                            ctx,
                            b"unlink: \0" as *const u8 as *const ::core::ffi::c_char,
                            cname_ptr,
                        );
                        doneany = 0;
                    }
                }
            }
        }
    }
    if doneany != 0 && sig == 0 {
        crate::output::trace_out(b"\n");
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn split_prereqs(
    ctx: &crate::execctx::ExecContext,
    mut p: *mut ::core::ffi::c_char,
) -> Result<Vec<DepNode>, crate::build_result::BuildError> {
    // 0x100 = PARSEFS_NOSTRIP, 0x40 = PARSEFS_WAIT (recognise `.WAIT`).
    let names = parse_file_seq(
        ctx,
        &raw mut p,
        ::core::mem::size_of::<dep>() as size_t,
        0x100_i32,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x40_i32,
    )?;
    let mut deps: Vec<DepNode> = names
        .into_iter()
        .map(|n| dep_node_from_name(n.name, n.wait, false))
        .collect();
    push_order_only_prereqs(ctx, p, &mut deps).map(|()| deps)
}

/// Append the order-only prerequisites that follow a `|` (tagged
/// `ignore_mtime`) to `deps`. Split out of [`split_prereqs`] so the second
/// `~`-expanding parse does not add a decision point to the first one's frame.
///
/// # Safety
/// As [`split_prereqs`]: `p` must point just past the parsed head.
unsafe fn push_order_only_prereqs(
    ctx: &crate::execctx::ExecContext,
    mut p: *mut ::core::ffi::c_char,
    deps: &mut Vec<DepNode>,
) -> Result<(), crate::build_result::BuildError> {
    if p.as_ref().is_none_or(|c| *c == 0) {
        return Ok(());
    }
    p = p.offset(1_i32 as isize);
    let ood_names = parse_file_seq(
        ctx,
        &raw mut p,
        ::core::mem::size_of::<dep>() as size_t,
        0x1_i32,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x40_i32,
    )?;
    for n in ood_names {
        let mut d = dep_node_from_name(n.name, n.wait, false);
        d.ignore_mtime = true;
        deps.push(d);
    }
    Ok(())
}

/// Build a fresh [`DepNode`] from an owned prerequisite name plus its `.WAIT`
/// marker. `static_pattern` is the initial static-pattern flag. The pointer-free
/// companion to the former `alloc_dep`/name-set dance.
fn dep_node_from_name(name: Vec<u8>, wait: bool, static_pattern: bool) -> DepNode {
    DepNode {
        name: String::from_utf8_lossy(&name).into_owned(),
        file: None,
        shuf: None,
        stem: None,
        flags: DepFlags::empty(),
        changed: false,
        ignore_mtime: false,
        static_pattern,
        needs_second_expansion: false,
        ignore_automatic_vars: false,
        is_explicit: false,
        wait_here: wait,
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn enter_prereqs(
    ctx: &crate::execctx::ExecContext,
    mut deps: Vec<DepNode>,
    stem: Option<&[u8]>,
) -> Vec<DepNode> {
    if deps.is_empty() {
        return deps;
    }
    if let Some(stem_bytes) = stem {
        let pattern: *const ::core::ffi::c_char = b"%\0" as *const u8 as *const ::core::ffi::c_char;
        let mut kept: Vec<DepNode> = Vec::with_capacity(deps.len());
        for mut d in deps.into_iter() {
            if d.needs_second_expansion {
                kept.push(d);
                continue;
            }
            // Mutable, NUL-terminated copy of the name for the in-place
            // `find_percent` rewrite.
            let mut nm: Vec<u8> = d.name.clone().into_bytes();
            nm.push(0);
            let nm_ptr = nm.as_mut_ptr() as *mut ::core::ffi::c_char;
            let percent = find_percent(nm_ptr);
            if !percent.is_null() {
                // NUL-terminated stem for the C patsubst/expand helpers.
                let mut stem_c: Vec<u8> = stem_bytes.to_vec();
                stem_c.push(0);
                let o: *mut ::core::ffi::c_char;
                if stem_bytes.is_empty() {
                    memmove(
                        percent as *mut ::core::ffi::c_void,
                        percent.offset(1_i32 as isize) as *const ::core::ffi::c_void,
                        strlen(percent),
                    );
                    o = variable_buffer_output(
                        ctx,
                        ctx.variable_buffer.ptr(),
                        nm_ptr,
                        (strlen(nm_ptr) as size_t).wrapping_add(1),
                    );
                } else {
                    o = patsubst_expand_pat(
                        ctx,
                        ctx.variable_buffer.ptr(),
                        stem_c.as_ptr() as *const ::core::ffi::c_char,
                        pattern,
                        nm_ptr,
                        pattern.offset(1_i32 as isize),
                        percent.offset(1_i32 as isize),
                    );
                }
                if *ctx.variable_buffer.ptr().offset(0_i32 as isize) as i32 == 0 {
                    // Expanded to nothing: drop this prerequisite.
                    continue;
                } else {
                    let result = ::core::slice::from_raw_parts(
                        ctx.variable_buffer.ptr() as *const u8,
                        o.offset_from(ctx.variable_buffer.ptr()) as usize,
                    );
                    d.name = String::from_utf8_lossy(result).into_owned();
                }
            }
            d.stem = Some(String::from_utf8_lossy(stem_bytes).into_owned());
            d.static_pattern = true;
            kept.push(d);
        }
        deps = kept;
    }
    // Resolve targets to FileIds for non-second-expansion deps.
    for d in deps.iter_mut() {
        if !d.needs_second_expansion {
            let name_bytes = d.name.clone().into_bytes();
            let fid = lookup_file(ctx, &name_bytes).unwrap_or_else(|| enter_file(ctx, &name_bytes));
            d.file = Some(fid);
            d.static_pattern = false;
            // The c2rust graph nulled `name` once `file` was resolved; we keep
            // `name` (cheap owned String) for diagnostics and dep_name parity.
            if stem.is_none() {
                if let Some(node) = ctx.filenodes.get(fid) {
                    node.lock().expect("file node lock poisoned").is_explicit = true;
                }
            }
        }
    }
    deps
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_deps(
    ctx: &crate::execctx::ExecContext,
    f: FileId,
) -> Result<(), crate::build_result::BuildError> {
    let Some(node) = ctx.filenodes.get(f) else {
        return Ok(());
    };

    // Latch `snapped` and snapshot the current deps + the file's own stem. The
    // guard is dropped before any reentrant variable/expansion call.
    let (deps, file_stem): (Vec<DepNode>, Option<Vec<u8>>) = {
        let mut n = node.lock().expect("file node lock poisoned");
        if n.snapped {
            return Ok(());
        }
        n.snapped = true;
        (
            std::mem::take(&mut n.deps),
            n.stem.as_ref().map(|s| s.clone().into_bytes()),
        )
    };

    let mut initialized = false;
    let mut changed_dep = false;
    // Rebuilt dep list: untouched deps are pushed through verbatim; a
    // second-expansion dep is replaced by its expansion's resolved deps.
    let mut rebuilt: Vec<DepNode> = Vec::with_capacity(deps.len());

    // Walked by iterator rather than `drain` so that a rejected expansion can
    // hand the untouched tail back to the node: the deps were moved out of the
    // `FileNode` above, and since #442 this loop can stop early, so the list has
    // to be put back rather than silently truncated (the cleanup-paths contract
    // from #561).
    let mut rest = deps.into_iter();
    let mut rejected = None;
    for d in rest.by_ref() {
        if d.name.is_empty() || !d.needs_second_expansion {
            rebuilt.push(d);
            continue;
        }

        // For a static-pattern dep, rewrite every `%` token in the name to
        // `$*` so the upcoming expansion substitutes the stem.
        let mut name_bytes = d.name.clone().into_bytes();
        if d.static_pattern && name_bytes.contains(&b'%') {
            name_bytes = rewrite_static_pattern_name(&name_bytes);
        }

        if !initialized {
            if let Err(e) = initialize_file_variables(ctx, f, 0) {
                rejected = Some(e);
                rebuilt.push(d);
                break;
            }
            initialized = true;
        }
        let stem: Option<&[u8]> = match &d.stem {
            Some(s) => Some(s.as_bytes()),
            None => file_stem.as_deref(),
        };
        set_file_variables(ctx, f, stem)?;

        // Second-expansion string expansion in this target's variable context,
        // via the FileId form of `expand_string_for_file`.
        let mut name_c = name_bytes.clone();
        name_c.push(0);
        let mut expanded = expand_string_for_file(ctx, &name_c, f)?;

        let mut new = split_prereqs(ctx, expanded.as_mut_ptr() as *mut ::core::ffi::c_char)?;
        changed_dep = true;
        if new.is_empty() {
            continue;
        }
        let fstem = d.stem.clone();
        for nd in new.iter_mut() {
            let nm = nd.name.clone().into_bytes();
            let fid = lookup_file(ctx, &nm).unwrap_or_else(|| enter_file(ctx, &nm));
            nd.file = Some(fid);
            nd.stem = fstem.clone();
            if fstem.is_none() {
                if let Some(fnode) = ctx.filenodes.get(fid) {
                    fnode.lock().expect("file node lock poisoned").is_explicit = true;
                }
            }
        }
        rebuilt.append(&mut new);
    }

    rebuilt.extend(rest);
    {
        let mut n = node.lock().expect("file node lock poisoned");
        n.deps = rebuilt;
    }
    if let Some(e) = rejected {
        return Err(e);
    }
    if changed_dep {
        crate::shuffle::shuffle_deps_recursive(ctx, f);
    }
    Ok(())
}

/// Rewrite a static-pattern prerequisite name for second expansion: each `%`
/// token is replaced by `$*` so the subsequent variable expansion substitutes
/// the stem. Operates on raw name bytes (the former in-place `xmalloc`/`mempcpy`
/// dance over the c2rust `(*d).name`).
fn rewrite_static_pattern_name(name: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(name.len() + 2);
    let mut i = 0usize;
    while i < name.len() {
        if name[i] == b'%' {
            // Replace the `%` with `$*`, then copy the rest of the current
            // whitespace-delimited token verbatim (the C copied through the end
            // of the token before scanning for the next `%`).
            out.push(b'$');
            out.push(b'*');
            i += 1;
            while i < name.len() && !name[i].is_ascii_whitespace() {
                out.push(name[i]);
                i += 1;
            }
        } else {
            out.push(name[i]);
            i += 1;
        }
    }
    out
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_extra_prereqs(
    ctx: &crate::execctx::ExecContext,
    extra: *const variable,
) -> Result<Vec<DepNode>, crate::build_result::BuildError> {
    if extra.is_null() {
        return Ok(Vec::new());
    }
    // Expand the `.EXTRA_PREREQS` value, then split it into prerequisites.
    //
    // The result is *borrowed*, not owned: `expand_string_buf` with a null
    // `buf` writes into `ctx.variable_buffer` and hands back a cursor into it,
    // exactly as the C `variable_expand` returns the shared `variable_buffer`
    // and leaves ownership with the caller's context. The `allocated_*`
    // wrappers are the ones that swap the buffer out and transfer ownership.
    //
    // Since #442 a malformed reference in `.EXTRA_PREREQS` — an unterminated
    // `$(`, or a bad builtin call — comes back as a `BuildError` rather than
    // ending the process; `snap_deps` already returns `Result`, so it carries
    // out from here with no bridge.
    let expanded = expand_string_buf(
        ctx,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        (*extra).value,
        SIZE_MAX as size_t,
    )?;
    let mut prereqs = split_prereqs(ctx, expanded)?;
    // Resolve each prerequisite to a target and flag it so automatic variables
    // are ignored when it is evaluated.
    for d in prereqs.iter_mut() {
        let name_bytes = d.name.clone().into_bytes();
        let fid = lookup_file(ctx, &name_bytes).unwrap_or_else(|| enter_file(ctx, &name_bytes));
        d.file = Some(fid);
        d.ignore_automatic_vars = true;
    }
    Ok(prereqs)
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn snap_file(
    ctx: &crate::execctx::ExecContext,
    f: FileId,
    deps: &[DepNode],
) -> Result<(), crate::build_result::BuildError> {
    let Some(node) = ctx.filenodes.get(f) else {
        return Ok(());
    };

    // First pass over the node's flags: reset `updating`, fold in the global
    // `.SECONDARY`/`.NOTINTERMEDIATE` defaults, read `is_target`, snapshot the
    // node's name, the existing deps, and the `.EXTRA_PREREQS` value (if any).
    // The guard is dropped before any reentrant arena call below.
    let (fname, is_target, extra_value, has_variables) = {
        let mut n = node.lock().expect("file node lock poisoned");
        if !second_expansion(ctx) {
            n.updating = false;
        }
        if ctx.all_secondary.get() && !n.notintermediate {
            n.intermediate = true;
        }
        if ctx.no_intermediates.get() && !n.intermediate && !n.secondary {
            n.notintermediate = true;
        }
        let has_variables = !n.variables.is_empty();
        let extra_value = n
            .variables
            .iter()
            .find(|v| v.name == b".EXTRA_PREREQS")
            .map(|v| v.value.clone());
        (n.name.clone(), n.is_target, extra_value, has_variables)
    };

    // Compute the prerequisites to add. With per-target variables, take
    // `.EXTRA_PREREQS`; otherwise a target file copies the shared `deps`.
    let mut prereqs: Vec<DepNode> = if has_variables {
        let pre = match &extra_value {
            Some(value) => expand_extra_prereqs_value(ctx, value)?,
            None => Vec::new(),
        };
        if second_expansion(ctx) {
            let mut pre = pre;
            for d in pre.iter_mut() {
                // The owned `name` is always populated in the node model, so the
                // former "name was nulled, copy it back from file" path is moot.
                d.needs_second_expansion = true;
            }
            pre
        } else {
            pre
        }
    } else if is_target {
        deps.to_vec()
    } else {
        Vec::new()
    };

    if prereqs.is_empty() {
        return Ok(());
    }

    // Skip circular dependencies: if any prereq names this file, drop the whole
    // batch (matching the C early-break + free_dep_chain).
    let circular = prereqs.iter().any(|d| dep_name_bytes(d) == fname);
    if circular {
        return Ok(());
    }

    let mut n = node.lock().expect("file node lock poisoned");
    n.deps.append(&mut prereqs);
    Ok(())
}

/// Expand an `.EXTRA_PREREQS` variable value, split it into prerequisites, and
/// resolve each to a target — the value-taking companion to
/// [`expand_extra_prereqs`] used by the per-target path in [`snap_file`], where
/// the value lives on the [`FileNode`] (`Vec<TargetVariable>`) rather than behind
/// a legacy `*const variable`.
unsafe fn expand_extra_prereqs_value(
    ctx: &crate::execctx::ExecContext,
    value: &[u8],
) -> Result<Vec<DepNode>, crate::build_result::BuildError> {
    let mut value_c: Vec<u8> = value.to_vec();
    value_c.push(0);
    let expanded = expand_string_buf(
        ctx,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value_c.as_ptr() as *const ::core::ffi::c_char,
        SIZE_MAX as size_t,
    )?;
    // Borrowed from `ctx.variable_buffer`, not owned — see the note in
    // `expand_extra_prereqs`.
    let mut prereqs = split_prereqs(ctx, expanded)?;
    for d in prereqs.iter_mut() {
        let name_bytes = d.name.clone().into_bytes();
        let fid = lookup_file(ctx, &name_bytes).unwrap_or_else(|| enter_file(ctx, &name_bytes));
        d.file = Some(fid);
        d.ignore_automatic_vars = true;
    }
    Ok(prereqs)
}

/// The name of a dependency as owned bytes: the [`DepNode`] keeps its `name`
/// populated, so this is just the name's bytes.
fn dep_name_bytes(d: &DepNode) -> Vec<u8> {
    d.name.clone().into_bytes()
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub fn snap_deps(ctx: &crate::execctx::ExecContext) -> Result<(), crate::build_result::BuildError> {
    crate::entry::mark_snapped_deps(ctx);

    // `.PRECIOUS`: mark each prereq target precious.
    for fid in special_dep_targets(ctx, b".PRECIOUS") {
        apply_to_file_and_double_colon(ctx, fid, |n| n.precious = true);
    }

    // `.LOW_RESOLUTION_TIME`: mark each prereq target.
    for fid in special_dep_targets(ctx, b".LOW_RESOLUTION_TIME") {
        apply_to_file_and_double_colon(ctx, fid, |n| n.low_resolution_time = true);
    }

    // `.PHONY`: mark each prereq target as a phony, nonexistent target.
    for fid in special_dep_targets(ctx, b".PHONY") {
        apply_to_file_and_double_colon(ctx, fid, |n| {
            n.phony = true;
            n.is_target = true;
            n.last_mtime = NONEXISTENT_MTIME as u64;
            n.mtime_before_update = NONEXISTENT_MTIME as u64;
        });
    }

    // `.NOTINTERMEDIATE`: with deps, mark each; with no deps, mark all files.
    match special_target_state(ctx, b".NOTINTERMEDIATE") {
        SpecialTargetState::WithDeps(targets) => {
            for fid in targets {
                apply_to_file_and_double_colon(ctx, fid, |n| n.notintermediate = true);
            }
        }
        SpecialTargetState::NoDeps => ctx.no_intermediates.set(true),
        SpecialTargetState::Absent => {}
    }

    // `.INTERMEDIATE`: mark each prereq target intermediate (fatal if it is
    // also `.NOTINTERMEDIATE`).
    for fid in special_dep_targets(ctx, b".INTERMEDIATE") {
        let conflict = apply_to_file_and_double_colon_checked(ctx, fid, |n| {
            if n.notintermediate {
                return Some(n.name.clone());
            }
            n.intermediate = true;
            None
        });
        if let Some(name) = conflict {
            // SAFETY: `fatal_special_conflict` only formats `ctx`/`name`/`kinds`
            // into a diagnostic; no raw-pointer precondition beyond that.
            unsafe { fatal_special_conflict(ctx, &name, b".NOTINTERMEDIATE and .INTERMEDIATE") }?;
        }
    }

    // `.SECONDARY`: with deps, mark each both secondary and intermediate (fatal
    // if also `.NOTINTERMEDIATE`); with no deps, mark all files secondary.
    match special_target_state(ctx, b".SECONDARY") {
        SpecialTargetState::WithDeps(targets) => {
            for fid in targets {
                let conflict = apply_to_file_and_double_colon_checked(ctx, fid, |n| {
                    if n.notintermediate {
                        return Some(n.name.clone());
                    }
                    n.secondary = true;
                    n.intermediate = n.secondary;
                    None
                });
                if let Some(name) = conflict {
                    // SAFETY: see the `.INTERMEDIATE` conflict call above.
                    unsafe {
                        fatal_special_conflict(ctx, &name, b".NOTINTERMEDIATE and .SECONDARY")
                    }?;
                }
            }
        }
        SpecialTargetState::NoDeps => ctx.all_secondary.set(true),
        SpecialTargetState::Absent => {}
    }

    if ctx.no_intermediates.get() && ctx.all_secondary.get() {
        // SAFETY: `fatal_err` only formats a NUL-terminated string literal and
        // `ctx` into a diagnostic; no other raw-pointer precondition applies.
        return Err(unsafe {
            fatal_err(
                ctx,
                ::core::ptr::null_mut::<Floc>(),
                0,
                b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            )
        });
    }

    // `.EXPORT_ALL_VARIABLES`: a target presence enables global export.
    if special_target_is_target(ctx, b".EXPORT_ALL_VARIABLES") {
        with_options(ctx, |o| o.export_all_variables.set(true));
    }

    // `.IGNORE`: with deps, set per-target NOERROR; with no deps, global.
    match special_target_command_flag_state(ctx, b".IGNORE") {
        SpecialTargetState::WithDeps(targets) => {
            for fid in targets {
                apply_to_file_and_double_colon(ctx, fid, |n| n.command_flags |= COMMANDS_NOERROR);
            }
        }
        SpecialTargetState::NoDeps => crate::entry::set_ignore_errors_mirror(ctx, true),
        SpecialTargetState::Absent => {}
    }

    // `.SILENT`: with deps, set per-target SILENT; with no deps, global.
    match special_target_command_flag_state(ctx, b".SILENT") {
        SpecialTargetState::WithDeps(targets) => {
            for fid in targets {
                apply_to_file_and_double_colon(ctx, fid, |n| n.command_flags |= COMMANDS_SILENT);
            }
        }
        SpecialTargetState::NoDeps => with_options(ctx, |o| o.run_silent.set(true)),
        SpecialTargetState::Absent => {}
    }

    // `.NOTPARALLEL`: with deps, mark each prereq target's own deps (after the
    // first) `wait_here`; with no deps, disable parallelism globally.
    match special_target_command_flag_state(ctx, b".NOTPARALLEL") {
        SpecialTargetState::WithDeps(targets) => {
            for fid in targets {
                mark_notparallel(ctx, fid);
            }
        }
        SpecialTargetState::NoDeps => crate::entry::set_not_parallel(ctx),
        SpecialTargetState::Absent => {}
    }

    // Global `.EXTRA_PREREQS`: expand once, then offer to every snapped file.
    // SAFETY: `lookup_variable` is passed a NUL-terminated string literal and
    // its exact byte length; `expand_extra_prereqs` only reads the `variable`
    // it returns.
    let prereqs: Vec<DepNode> = unsafe {
        // Bound before the call rather than `?`-ed inside its argument list, so
        // the pointer handed to `expand_extra_prereqs` is only ever a value the
        // `Result` has already yielded.
        let extra = lookup_variable(
            ctx,
            b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
        )?;
        expand_extra_prereqs(ctx, extra)?
    };
    // Snapshot the arena's files, then snap each. Matching the C `hash_dump`,
    // any files entered while snapping are not themselves re-processed here.
    let filedump: Vec<FileId> = ctx
        .filenodes
        .0
        .lock()
        .expect("file arena poisoned")
        .keys()
        .copied()
        .collect();
    for fid in filedump {
        // SAFETY: `snap_file` is the c2rust-inherited per-file snap step;
        // `ctx`/`fid`/`prereqs` are all valid owned/arena-backed values.
        unsafe { snap_file(ctx, fid, &prereqs)? };
    }
    Ok(())
}

/// Outcome of inspecting a special target (`.PHONY`, `.SECONDARY`, …): it may
/// be absent, present but with no prerequisites (the "applies to everything"
/// form), or present with a list of prerequisite target [`FileId`]s.
enum SpecialTargetState {
    Absent,
    NoDeps,
    WithDeps(Vec<FileId>),
}

/// Resolve the prerequisite target [`FileId`]s of a special target, walking the
/// special target's own double-colon entries and resolving each prereq's
/// `file`. Names that have not been entered are skipped (matching the C, where a
/// prereq always has a resolved `file` by snap time). Returns an empty `Vec`
/// when the special target is absent or has no prereqs.
///
/// Locking discipline: the head node is locked only to clone out its `deps` and
/// double-colon entries' `deps`; the guard is dropped before each prereq name is
/// resolved through the arena.
fn special_dep_targets(ctx: &crate::execctx::ExecContext, name: &[u8]) -> Vec<FileId> {
    match special_target_state(ctx, name) {
        SpecialTargetState::WithDeps(t) => t,
        _ => Vec::new(),
    }
}

/// Like [`special_dep_targets`] but distinguishing "absent" from "present, no
/// deps" so the no-argument forms of `.NOTINTERMEDIATE`/`.SECONDARY` can latch
/// their global flags.
fn special_target_state(ctx: &crate::execctx::ExecContext, name: &[u8]) -> SpecialTargetState {
    let Some(head_id) = lookup_file(ctx, name) else {
        return SpecialTargetState::Absent;
    };
    let Some(node) = ctx.filenodes.get(head_id) else {
        return SpecialTargetState::Absent;
    };
    // Collect every dep name across the head and its double-colon entries,
    // dropping the guard before resolving any name through the arena.
    let dep_names: Vec<Vec<u8>> = {
        let n = node.lock().expect("file node lock poisoned");
        let mut names: Vec<Vec<u8>> = Vec::new();
        for d in &n.deps {
            names.push(dep_name_bytes(d));
        }
        for dc in &n.double_colon {
            for d in &dc.deps {
                names.push(dep_name_bytes(d));
            }
        }
        names
    };
    if dep_names.is_empty() {
        return SpecialTargetState::NoDeps;
    }
    let mut targets: Vec<FileId> = Vec::with_capacity(dep_names.len());
    for nm in dep_names {
        if let Some(fid) = lookup_file(ctx, &nm) {
            targets.push(fid);
        }
    }
    SpecialTargetState::WithDeps(targets)
}

/// `.IGNORE`/`.SILENT`/`.NOTPARALLEL` only act when the special target was
/// actually mentioned as a target (`is_target`). Returns `Absent` when it was
/// merely referenced as a prerequisite name, mirroring the C `f->is_target`
/// guard.
fn special_target_command_flag_state(
    ctx: &crate::execctx::ExecContext,
    name: &[u8],
) -> SpecialTargetState {
    if !special_target_is_target(ctx, name) {
        return SpecialTargetState::Absent;
    }
    special_target_state(ctx, name)
}

/// Whether the named special target exists and was mentioned as a target.
fn special_target_is_target(ctx: &crate::execctx::ExecContext, name: &[u8]) -> bool {
    let Some(head_id) = lookup_file(ctx, name) else {
        return false;
    };
    let Some(node) = ctx.filenodes.get(head_id) else {
        return false;
    };
    let n = node.lock().expect("file node lock poisoned");
    n.is_target
}

/// Apply `f` to the file `fid` and each of its inline double-colon entries (the
/// former `for (f2 = ...; f2; f2 = f2->prev)` chain walk). A single guard is held
/// for the whole node, which is sound: `f` does not re-enter the arena.
fn apply_to_file_and_double_colon(
    ctx: &crate::execctx::ExecContext,
    fid: FileId,
    mut f: impl FnMut(&mut FileNode),
) {
    if let Some(node) = ctx.filenodes.get(fid) {
        let mut n = node.lock().expect("file node lock poisoned");
        f(&mut n);
        for dc in n.double_colon.iter_mut() {
            f(dc);
        }
    }
}

/// Like [`apply_to_file_and_double_colon`] but `f` may return `Some(name)` to
/// signal a conflict, which short-circuits and is returned to the caller (used
/// for the `.INTERMEDIATE`/`.SECONDARY` vs `.NOTINTERMEDIATE` fatal checks).
fn apply_to_file_and_double_colon_checked(
    ctx: &crate::execctx::ExecContext,
    fid: FileId,
    mut f: impl FnMut(&mut FileNode) -> Option<Vec<u8>>,
) -> Option<Vec<u8>> {
    let node = ctx.filenodes.get(fid)?;
    let mut n = node.lock().expect("file node lock poisoned");
    if let Some(conflict) = f(&mut n) {
        return Some(conflict);
    }
    for dc in n.double_colon.iter_mut() {
        if let Some(conflict) = f(dc) {
            return Some(conflict);
        }
    }
    None
}

/// `.NOTPARALLEL` with explicit prereqs: for the named target `fid`, mark every
/// one of its own prerequisites *after the first* as `wait_here` (the C
/// `f2->deps->next` walk), across the head and its double-colon entries.
fn mark_notparallel(ctx: &crate::execctx::ExecContext, fid: FileId) {
    if let Some(node) = ctx.filenodes.get(fid) {
        let mut n = node.lock().expect("file node lock poisoned");
        for d in n.deps.iter_mut().skip(1) {
            d.wait_here = true;
        }
        for dc in n.double_colon.iter_mut() {
            for d in dc.deps.iter_mut().skip(1) {
                d.wait_here = true;
            }
        }
    }
}

/// Emit the byte-identical fatal diagnostic for a target that is both
/// `.NOTINTERMEDIATE` and `.INTERMEDIATE`/`.SECONDARY`.
unsafe fn fatal_special_conflict(
    ctx: &crate::execctx::ExecContext,
    name: &[u8],
    kinds: &[u8],
) -> Result<(), crate::build_result::BuildError> {
    let mut name_c = name.to_vec();
    name_c.push(0);
    let mut msg = b"%s cannot be both ".to_vec();
    msg.extend_from_slice(kinds);
    msg.push(0);
    Err(fatal_err(
        ctx,
        ::core::ptr::null_mut::<Floc>(),
        name.len() as size_t,
        msg.as_ptr() as *const ::core::ffi::c_char,
        &[FmtArg::Str(name_c.as_ptr() as *const ::core::ffi::c_char)],
    ))
}

#[cfg(test)]
mod fatal_special_conflict_tests {
    use super::fatal_special_conflict;

    /// [`fatal_special_conflict`] returns `BuildError::Failure` instead of
    /// aborting the process, and marks the context dying (#432 Phase B,
    /// #539).
    #[test]
    fn returns_failure_and_marks_dying() {
        let ctx = crate::execctx::ExecContext::default();
        assert!(!ctx.dying.0.load(::std::sync::atomic::Ordering::Relaxed));

        let result =
            unsafe { fatal_special_conflict(&ctx, b"target", b".NOTINTERMEDIATE and .SECONDARY") };

        assert_eq!(result, Err(crate::build_result::BuildError::Failure));
        assert!(ctx.dying.0.load(::std::sync::atomic::Ordering::Relaxed));
    }
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
        Some(dt) => {
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                dt.year(),
                dt.month(),
                dt.day(),
                dt.hour(),
                dt.minute(),
                dt.second(),
            )
        }
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
pub unsafe fn print_prereqs(deps: &[DepNode]) {
    // Print one prerequisite: optional `.WAIT ` marker plus its name.
    unsafe fn print_one(d: &DepNode, leading: &[u8]) {
        let name = dep_name_bytes(d);
        let wait: &[u8] = if d.wait_here { b".WAIT " } else { b"" };
        crate::output::trace_parts(&[leading, wait, &name]);
    }
    // Normal prerequisites first; the first order-only prereq starts the `|`
    // block.
    let mut first_ood: Option<usize> = None;
    for (i, d) in deps.iter().enumerate() {
        if !d.ignore_mtime {
            print_one(d, b" ");
        } else if first_ood.is_none() {
            first_ood = Some(i);
        }
    }
    if let Some(start) = first_ood {
        print_one(&deps[start], b" | ");
        for d in &deps[start + 1..] {
            if d.ignore_mtime {
                print_one(d, b" ");
            }
        }
    }
    crate::output::trace_out(b"\n");
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_file(ctx: &crate::execctx::ExecContext, fid: FileId) {
    let Some(node) = ctx.filenodes.get(fid) else {
        return;
    };
    // Snapshot the head and each inline double-colon entry, then print each in
    // order (the former `prev`-chain recursion). The guard is dropped before any
    // printing so reentrant variable lookups never deadlock on this node.
    let (head, entries): (FileNode, Vec<FileNode>) = {
        let n = node.lock().expect("file node lock poisoned");
        (n.clone(), n.double_colon.clone())
    };
    print_file_node(ctx, fid, &head, !entries.is_empty());
    for e in &entries {
        print_file_node(ctx, fid, e, false);
    }
}

/// Print one file node (`make -p` database entry). `fid` is the head's arena id,
/// used for the per-target/per-file variable dumps (inline double-colon entries
/// share the head's identity). `has_double_colon` controls the `::`/`:` after
/// the target name.
unsafe fn print_file_node(
    ctx: &crate::execctx::ExecContext,
    fid: FileId,
    f: &FileNode,
    has_double_colon: bool,
) {
    if crate::entry::opt_no_builtin_rules(ctx) && f.builtin {
        return;
    }
    crate::output::trace_out(b"\n");
    if let Some(recipe) = f.recipe.as_ref() {
        if recipe.recipe_prefix as i32 != crate::entry::opt_cmd_prefix(ctx) as i32 {
            crate::output::trace_out(b".RECIPEPREFIX = ");
            let new_prefix = recipe.recipe_prefix as ::core::ffi::c_char;
            with_options(ctx, |o| o.cmd_prefix.set(new_prefix));
            if new_prefix as i32 != RECIPEPREFIX_DEFAULT {
                crate::output::trace_out(&[new_prefix as u8]);
            }
            crate::output::trace_out(b"\n");
        }
    }
    if !f.variables.is_empty() {
        print_target_variables(ctx, fid);
    }
    if !f.is_target {
        crate::output::trace_out(b"# Not a target:\n");
    }
    crate::output::trace_parts(&[&f.name, if has_double_colon { b"::" } else { b":" }]);
    print_prereqs(&f.deps);
    if f.precious {
        crate::output::trace_out(b"#  Precious file (prerequisite of .PRECIOUS).\n");
    }
    if f.phony {
        crate::output::trace_out(b"#  Phony target (prerequisite of .PHONY).\n");
    }
    if f.cmd_target {
        crate::output::trace_out(b"#  Command line target.\n");
    }
    if f.dontcare {
        crate::output::trace_out(b"#  A default, MAKEFILES, or -include/sinclude makefile.\n");
    }
    if f.builtin {
        crate::output::trace_out(b"#  Builtin rule\n");
    }
    crate::output::trace_out(if f.tried_implicit {
        b"#  Implicit rule search has been done.\n"
    } else {
        b"#  Implicit rule search has not been done.\n"
    });
    if let Some(stem) = f.stem.as_ref() {
        crate::output::trace_parts(&[
            b"#  Implicit/static pattern stem: '",
            stem.as_bytes(),
            b"'\n",
        ]);
    }
    if f.intermediate {
        crate::output::trace_out(b"#  File is an intermediate prerequisite.\n");
    }
    if f.notintermediate {
        crate::output::trace_out(b"#  File is a prerequisite of .NOTINTERMEDIATE.\n");
    }
    if f.secondary {
        crate::output::trace_out(b"#  File is secondary (prerequisite of .SECONDARY).\n");
    }
    if f.is_explicit {
        crate::output::trace_out(b"#  File is explicitly mentioned.\n");
    }
    if !f.also_make.is_empty() {
        crate::output::trace_out(b"#  Also makes:");
        for d in &f.also_make {
            let nm = dep_name_bytes(d);
            crate::output::trace_parts(&[b" ", &nm]);
        }
        crate::output::trace_out(b"\n");
    }
    if f.last_mtime == UNKNOWN_MTIME as u64 {
        crate::output::trace_out(b"#  Modification time never checked.\n");
    } else if f.last_mtime == NONEXISTENT_MTIME as u64 {
        crate::output::trace_out(b"#  File does not exist.\n");
    } else if f.last_mtime == OLD_MTIME as u64 {
        crate::output::trace_out(b"#  File is very old.\n");
    } else {
        let stamp = file_timestamp_string(f.last_mtime);
        crate::output::trace_parts(&[b"#  Last modified ", stamp.as_bytes(), b"\n"]);
    }
    crate::output::trace_out(if f.updated {
        b"#  File has been updated.\n"
    } else {
        b"#  File has not been updated.\n"
    });
    match f.command_state {
        CommandState::Running => {
            crate::output::trace_out(b"#  Recipe currently running (THIS IS A BUG).\n");
        }
        CommandState::DepsRunning => {
            crate::output::trace_out(b"#  Dependencies recipe running (THIS IS A BUG).\n");
        }
        CommandState::NotStarted | CommandState::Finished => {
            match f.update_status {
                UpdateStatus::Success => {
                    crate::output::trace_out(b"#  Successfully updated.\n");
                }
                UpdateStatus::Question => {
                    if crate::entry::opt_question(ctx) {
                    } else {
                        panic!("assertion failed: question_flag");
                    };
                    crate::output::trace_out(b"#  Needs to be updated (-q is set).\n");
                }
                _ => {}
            }
        }
    }
    if !f.variables.is_empty() {
        print_file_variables(ctx, fid);
    }
    if let Some(recipe) = f.recipe.as_ref() {
        print_commands(ctx, recipe);
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_file_data_base(ctx: &crate::execctx::ExecContext) {
    crate::output::trace_out(b"\n# Files\n");
    // Snapshot the arena's `FileId`s under the map lock, drop it, then print
    // each (`print_file` walks the node's inline double-colon entries). Any
    // files entered while printing are not re-processed.
    let ids: Vec<FileId> = ctx
        .filenodes
        .0
        .lock()
        .expect("file arena poisoned")
        .keys()
        .copied()
        .collect();
    let count = ids.len();
    ids.into_iter().for_each(|fid| print_file(ctx, fid));
    crate::output::trace_parts(&[
        b"\n# ",
        (count as ::core::ffi::c_ulong).to_string().as_bytes(),
        b" files in the file table.\n",
    ]);
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
    crate::output::trace_parts(&[name, b"\n"]);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_targets(ctx: &crate::execctx::ExecContext) {
    // Snapshot the nodes from the arena, then print each over its `FileNode`
    // fields (the c2rust `print_target` walker is dead once the store is the
    // arena). Lock the map, grab the `Arc` handles, release the map lock, then
    // read `is_target`/`suffix`/`name` under each node's own lock.
    let nodes: Vec<::std::sync::Arc<::std::sync::Mutex<FileNode>>> = ctx
        .filenodes
        .0
        .lock()
        .expect("file arena poisoned")
        .values()
        .map(::std::sync::Arc::clone)
        .collect();
    nodes.iter().for_each(|node| print_one_target(node));
}

/// Print a single target's name for `print_targets` (the `make -p` `# Files`
/// stanza's target list). Skips non-targets, suffix-rule files, and the
/// built-in special targets (a dot followed by all-uppercase letters).
fn print_one_target(node: &::std::sync::Mutex<FileNode>) {
    let name = {
        let n = node.lock().expect("file node lock poisoned");
        if !n.is_target || n.suffix {
            return;
        }
        n.name.clone()
    };
    // Skip built-in special targets, whose names are a dot followed by one
    // or more all-uppercase letters (e.g. `.SUFFIXES`, `.PHONY`).
    if name.len() >= 2 && name[0] == b'.' && name[1..].iter().all(u8::is_ascii_uppercase) {
        return;
    }
    crate::output::trace_parts(&[&name, b"\n"]);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file(_ctx: &crate::execctx::ExecContext, _f: &FileNode) {
    // In the c2rust graph this checked that every name/stem field on a `*mut file`
    // and its deps was interned in the strcache (a raw `*const c_char` was only
    // valid if cached). In the `FileNode` model these fields are owned `Vec<u8>`/
    // `String`, so they are well-formed by construction and there is no strcache
    // pointer to verify. The walk is retained as a structural no-op so the
    // `make --debug` consistency pass keeps the same call shape.
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file_data_base(ctx: &crate::execctx::ExecContext) {
    // Snapshot the nodes under the map lock, drop it, then verify each under its
    // own lock.
    let nodes: Vec<::std::sync::Arc<::std::sync::Mutex<FileNode>>> = ctx
        .filenodes
        .0
        .lock()
        .expect("file arena poisoned")
        .values()
        .map(::std::sync::Arc::clone)
        .collect();
    nodes.iter().for_each(|node| {
        let n = node.lock().expect("file node lock poisoned");
        verify_file(ctx, &n);
    });
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn build_target_list(
    ctx: &crate::execctx::ExecContext,
    mut value: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    // Snapshot the targets from the arena: lock the map, grab `Arc` handles,
    // release the map lock, then read each node's `is_target`/`name` under its
    // own lock and drop the guard.
    let nodes: Vec<::std::sync::Arc<::std::sync::Mutex<FileNode>>> = ctx
        .filenodes
        .0
        .lock()
        .expect("file arena poisoned")
        .values()
        .map(::std::sync::Arc::clone)
        .collect();
    let fill = nodes.len() as ::core::ffi::c_ulong;
    if fill != ctx.last_targ_count.get() {
        let target_names: Vec<Vec<u8>> = nodes
            .iter()
            .filter_map(|node| {
                let n = node.lock().expect("file node lock poisoned");
                if n.is_target {
                    Some(n.name.clone())
                } else {
                    None
                }
            })
            .collect();
        let mut max: size_t = (strlen(value) as size_t)
            .wrapping_div(500)
            .wrapping_add(1)
            .wrapping_mul(500);
        let mut len: size_t;
        let mut p: *mut ::core::ffi::c_char;
        value = xrealloc(value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
        p = value;
        len = 0;
        for name in &target_names {
            let l: size_t = name.len() as size_t;
            len = len.wrapping_add(l.wrapping_add(1));
            if len > max {
                let off: size_t = p.offset_from(value) as ::core::ffi::c_long as size_t;
                max = max.wrapping_add(
                    l.wrapping_add(1)
                        .wrapping_div(500)
                        .wrapping_add(1)
                        .wrapping_mul(500),
                );
                value =
                    xrealloc(value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
                p = value.offset(off as isize) as *mut ::core::ffi::c_char;
            }
            p = mempcpy(
                p as *mut ::core::ffi::c_void,
                name.as_ptr() as *const ::core::ffi::c_void,
                l as size_t,
            ) as *mut ::core::ffi::c_char;
            let fresh4 = p;
            p = p.offset(1_i32 as isize);
            *fresh4 = ' ' as i32 as ::core::ffi::c_char;
        }
        *p.offset(-(1_i32 as isize)) = 0;
        ctx.last_targ_count.set(fill);
    }
    value
}
pub const FILE_TIMESTAMP_HI_RES: i32 = 1;

#[cfg(test)]
mod tests {
    use {super::*, crate::entry::initialize_stopchar_map, std::sync::Mutex};

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
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            let fid = enter_file(&ctx, b"snap_plain_probe");
            {
                let node = ctx.filenodes.get(fid).unwrap();
                node.lock().unwrap().updating = true;
            }
            snap_file(&ctx, fid, &[]).expect("snap_file on a fixed dep set cannot fail");
            let node = ctx.filenodes.get(fid).unwrap();
            assert!(
                !node.lock().unwrap().updating,
                "updating cleared when not 2nd-expanding"
            );
        }
    }

    /// For a target file with no per-target variables, `snap_file` copies the
    /// shared dep list (here a single prereq whose name matches the target, so
    /// the self-match break path runs and nothing is appended).
    #[test]
    fn snap_file_target_copies_extra_prereqs() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            let fid = enter_file(&ctx, b"snapself");
            {
                let node = ctx.filenodes.get(fid).unwrap();
                node.lock().unwrap().is_target = true;
            }
            // A one-element shared dep list whose dep name equals the target name.
            let deps = vec![dep_node_from_name(b"snapself".to_vec(), false, false)];
            snap_file(&ctx, fid, &deps).expect("snap_file on a fixed dep set cannot fail");
            // The self-referential prereq is dropped, so deps stays empty.
            let node = ctx.filenodes.get(fid).unwrap();
            assert!(
                node.lock().unwrap().deps.is_empty(),
                "self-prereq is not appended"
            );
        }
    }

    /// `expand_extra_prereqs` returns an empty list for a NULL variable and,
    /// for a real one, expands the value and resolves each word to a file.
    ///
    /// Since #442 it returns `Result`, because a malformed reference in
    /// `.EXTRA_PREREQS` travels out through `snap_deps` rather than ending the
    /// process. Driving it at all only became possible once the borrowed
    /// expansion buffer stopped being freed — before that fix any call reached
    /// a double free and aborted the test binary.
    #[test]
    fn expand_extra_prereqs_resolves_each_word() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _b = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            crate::function::hash_init_function_table(&ctx);
            crate::variable::init_hash_global_variable_set(&ctx);

            // A NULL variable is "no extra prerequisites", not an error.
            assert!(
                expand_extra_prereqs(&ctx, ::core::ptr::null())
                    .expect("a NULL variable is not an error")
                    .is_empty(),
                "a NULL .EXTRA_PREREQS yields no prereqs"
            );

            // A real value is expanded and split; each word becomes a resolved
            // dep flagged to ignore automatic variables.
            let name = ::std::ffi::CString::new(".EXTRA_PREREQS").unwrap();
            let value = ::std::ffi::CString::new("alpha beta").unwrap();
            let v = crate::variable::define_variable_in_set(
                &ctx,
                name.as_ptr(),
                strlen(name.as_ptr()),
                value.as_ptr(),
                crate::variable::o_file,
                0,
                ::core::ptr::null_mut(),
                ::core::ptr::null::<crate::floc::Floc>(),
            )
            .expect("test fixture defines a well-formed name");
            let prereqs = expand_extra_prereqs(&ctx, v).expect("well-formed value");
            let names: Vec<Vec<u8>> = prereqs.iter().map(dep_name_bytes).collect();
            assert_eq!(names, vec![b"alpha".to_vec(), b"beta".to_vec()]);
            assert!(
                prereqs.iter().all(|d| d.ignore_automatic_vars),
                "extra prereqs ignore automatic variables"
            );
            assert!(
                prereqs.iter().all(|d| d.file.is_some()),
                "each extra prereq resolves to a file"
            );
        }
    }

    /// `enter_prereqs(deps, None)` resolves each prerequisite to a file via
    /// `enter_file` and (with no stem) marks the entered file explicit. Drives
    /// the common no-pattern path.
    #[test]
    fn enter_prereqs_resolves_files_for_plain_deps() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();

            let deps = vec![dep_node_from_name(
                b"enter_prereqs_probe_target".to_vec(),
                false,
                false,
            )];
            let out = enter_prereqs(&ctx, deps, None);
            assert_eq!(out.len(), 1, "the chain length is unchanged");
            assert!(out[0].file.is_some(), "prereq resolved to a file");
            assert!(
                lookup_file(&ctx, b"enter_prereqs_probe_target").is_some(),
                "the prerequisite file is now in the table"
            );
        }
    }

    /// `enter_prereqs([], _)` is a no-op returning an empty list.
    #[test]
    fn enter_prereqs_null_is_noop() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let ctx = crate::execctx::ExecContext::default();
            assert!(enter_prereqs(&ctx, Vec::new(), None).is_empty());
        }
    }

    /// The file table is owned per-`ExecContext`, not a process global: a file
    /// entered in one context is found there but is invisible to an independent
    /// context. Guards against the table regressing to a `static mut`.
    #[test]
    fn file_table_is_per_context_not_global() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _ctx = crate::entry::install_default_exec_context_for_test();
        initialize_stopchar_map();
        let a = crate::execctx::ExecContext::default();
        let b = crate::execctx::ExecContext::default();

        let f = enter_file(&a, b"per_ctx_probe_target");
        assert_eq!(
            lookup_file(&a, b"per_ctx_probe_target"),
            Some(f),
            "and found again in context a"
        );
        assert!(
            lookup_file(&b, b"per_ctx_probe_target").is_none(),
            "an independent context shares no global file table"
        );
    }

    /// With a non-null stem, `enter_prereqs` walks the static-pattern block. A
    /// prerequisite name with no `%` finds no percent, so it keeps its name but
    /// is tagged with the stem, then resolved to a file.
    #[test]
    fn enter_prereqs_static_pattern_without_percent() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();

            let deps = vec![dep_node_from_name(
                b"enter_prereqs_static_probe".to_vec(),
                false,
                false,
            )];
            let out = enter_prereqs(&ctx, deps, Some(b"thestem"));
            assert_eq!(out.len(), 1);
            // The dep was tagged with the stem (staticpattern path ran) and then
            // resolved: file entered, static_pattern reset to false.
            assert_eq!(
                out[0].stem.as_deref(),
                Some("thestem"),
                "stem recorded on the static pattern"
            );
            assert!(out[0].file.is_some(), "prereq resolved to a file");
            assert!(
                !out[0].static_pattern,
                "static_pattern is reset after resolution"
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
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            crate::expand::initialize_variable_output(&ctx);

            let deps = vec![dep_node_from_name(b"%.o".to_vec(), false, false)];
            let out = enter_prereqs(&ctx, deps, Some(b"epp_stem"));
            assert_eq!(out.len(), 1);
            // `%` expanded to the stem and the dep resolved to a file named
            // "epp_stem.o".
            assert!(out[0].file.is_some(), "prereq resolved to a file");
            assert!(
                lookup_file(&ctx, b"epp_stem.o").is_some(),
                "the expanded prerequisite file was entered"
            );
        }
    }

    /// When a `%` prerequisite expands to the empty string (a bare `%` with an
    /// empty stem: the percent is dropped and nothing remains), `enter_prereqs`
    /// removes that prerequisite from the chain. With a single such dep the
    /// chain collapses to empty.
    #[test]
    fn enter_prereqs_drops_prereq_that_expands_empty() {
        let _g = FILE_GRAPH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let _b = crate::expand::VARIABLE_BUFFER_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        unsafe {
            initialize_stopchar_map();
            let ctx = crate::execctx::ExecContext::default();
            crate::expand::initialize_variable_output(&ctx);

            let deps = vec![dep_node_from_name(b"%".to_vec(), false, false)];
            // The bare `%` with an empty stem expands to "", so the dep is
            // dropped; the returned chain is empty.
            let out = enter_prereqs(&ctx, deps, Some(b""));
            assert!(out.is_empty(), "the empty-expanding prereq is removed");
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
    /// reads shared output state.
    static TIMESTAMP_ERR_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// A stamp below the encodable range (`s <= OLD_MTIME`) drives the
    /// out-of-range substitution branch: it formats the clamped timestamp and
    /// calls `error()` ("timestamp out of range: substituting"), then returns
    /// the substituted value `ORDINARY_MTIME_MIN`. The default context's null
    /// `program` name falls back to the plain "make" prefix.
    #[test]
    fn file_timestamp_cons_low_out_of_range_substitutes() {
        let _g = TIMESTAMP_ERR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
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

    /// `enter_file` interns a fresh node and is idempotent on the name;
    /// `lookup_file` finds it (after normalization) and misses otherwise.
    #[test]
    fn enter_and_lookup_filenode_round_trip_on_the_arena() {
        initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();

        assert!(lookup_file(&ctx, b"foo.o").is_none());

        let id = enter_file(&ctx, b"foo.o");
        assert_eq!(ctx.filenodes.len(), 1);
        // Re-entering the same (and the "./"-prefixed) name reuses the head.
        assert_eq!(enter_file(&ctx, b"foo.o"), id);
        assert_eq!(enter_file(&ctx, b"./foo.o"), id);
        assert_eq!(ctx.filenodes.len(), 1);

        assert_eq!(lookup_file(&ctx, b"foo.o"), Some(id));
        assert_eq!(lookup_file(&ctx, b"./foo.o"), Some(id));
        assert!(lookup_file(&ctx, b"bar.o").is_none());

        let node = ctx.filenodes.get(id).expect("interned");
        assert_eq!(node.lock().unwrap().name, b"foo.o");
    }

    /// Double-colon entries live inline on the head (they share its name, so
    /// they cannot be distinct name-derived `FileId`s) and the head is marked.
    #[test]
    fn double_colon_entries_are_inline_on_the_head() {
        initialize_stopchar_map();
        let ctx = crate::execctx::ExecContext::default();

        let id = enter_file(&ctx, b"all");
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
            f_append,
            f_append_value,
            f_bogus,
            f_expand,
            f_recursive,
            f_shell,
            f_simple,
            o_automatic,
            o_command,
            o_default,
            o_env,
            o_env_override,
            o_file,
            o_invalid,
            o_override,
            v_default,
            v_export,
            v_ifset,
            v_noexport,
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
        assert_eq!(FileId::from_bytes(b"foo.o"), FileId::from_bytes(b"foo.o"));
    }

    /// `FileId::from(&FileNode)` (the `id_wireformat!` `<-` form) content-hashes
    /// the *whole* node, deterministically and sensitive to every field — unlike
    /// `FileNode::id()` (`FileId::from_bytes(&hname)`), which is the graph's
    /// live identity and depends on the name alone. Mirrors `DepId <- DepNode`
    /// (dep.rs).
    #[test]
    fn file_id_from_node_hashes_whole_struct() {
        let a = FileNode::new(b"a.o".to_vec());
        let a_again = FileNode::new(b"a.o".to_vec());
        let b = FileNode::new(b"b.o".to_vec());
        assert_eq!(FileId::from(&a), FileId::from(&a_again));
        assert_ne!(FileId::from(&a), FileId::from(&b));

        // Two nodes with the same name but different mutable state hash
        // differently under `From<&FileNode>` — the opposite of `.id()`,
        // which stays fixed as long as `hname` is unchanged.
        let mut c = FileNode::new(b"c.o".to_vec());
        let before = FileId::from(&c);
        assert_eq!(c.id(), FileNode::new(b"c.o".to_vec()).id());
        c.precious = true;
        let after = FileId::from(&c);
        assert_ne!(before, after);
        assert_eq!(c.id(), FileId::from_bytes(b"c.o"));
    }
}

#[cfg(test)]
mod arena_helper_coverage_tests {
    use super::*;

    /// `File::new_named` stamps both `name` and `hname` from its argument and
    /// starts at `us_none` update status.
    #[test]
    fn new_named_sets_name_hname_and_status() {
        let nm = c"arena-coverage-new-named";
        let f = File::new_named(nm.as_ptr());
        assert_eq!(f.name, nm.as_ptr());
        assert_eq!(f.hname, nm.as_ptr());
        assert_eq!(f.update_status(), us_none);
    }

    /// The raw-pointer `set_command_state` sets the file's state; with no
    /// `also_make` peers the sibling loop is a no-op.
    #[test]
    fn set_command_state_sets_state() {
        let mut f = File::default();
        unsafe {
            set_command_state(&raw mut f, CommandState::Running);
        }
        assert_eq!(f.command_state, CommandState::Running);
    }

    /// `verify_file_data_base` walks every arena node (a structural no-op now);
    /// here it must run cleanly over a populated arena.
    #[test]
    fn verify_file_data_base_walks_the_arena() {
        let ctx = crate::execctx::ExecContext::default();
        let _ = enter_file(&ctx, b"verify-fdb-coverage");
        unsafe {
            verify_file_data_base(&ctx);
        }
    }
}
