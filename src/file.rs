use crate::content_hash::ContentHash;

pub use crate::ffi_types::{
    __clockid_t, __off64_t, __off_t, __suseconds_t, __syscall_slong_t, __time_t, clockid_t,
    intmax_t, size_t, time_t, uintmax_t,
};
use crate::id_wireformat;
use {
    crate::{
        misc::{copy_dep_chain, end_of_token, free_ns_chain, xcalloc, xmalloc, xrealloc, xstrdup},
        stdio::FILE,
        strcache::{strcache_add_len, strcache_iscached},
    },
    c2rust_bitfields,
    libc::{
        __errno_location, abort, free, printf, putchar, puts, sprintf, strchr, strcmp, strcpy,
        unlink,
    },
};
extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
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
    pub command_flags: ::core::ffi::c_int,
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
        File {
            name: ::core::ptr::null(),
            hname: ::core::ptr::null(),
            vpath: ::core::ptr::null(),
            deps: ::core::ptr::null_mut(),
            cmds: ::core::ptr::null_mut(),
            stem: ::core::ptr::null(),
            also_make: ::core::ptr::null_mut(),
            prev: ::core::ptr::null_mut(),
            last: ::core::ptr::null_mut(),
            renamed: ::core::ptr::null_mut(),
            variables: ::core::ptr::null_mut(),
            pat_variables: ::core::ptr::null_mut(),
            parent: ::core::ptr::null_mut(),
            double_colon: ::core::ptr::null_mut(),
            last_mtime: 0,
            mtime_before_update: 0,
            considered: 0,
            command_flags: 0,
            update_status: UpdateStatus::default(),
            command_state: CommandState::default(),
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

/// State of a file's recipe execution.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CommandState {
    #[default]
    NotStarted = 0,
    DepsRunning = 1,
    Running = 2,
    Finished = 3,
}

/// Outcome of the last attempt to update a file.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum UpdateStatus {
    #[default]
    Success = 0,
    None = 1,
    Question = 2,
    Failed = 3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSetList {
    pub next: *mut VariableSetList,
    pub set: *mut VariableSet,
    pub next_is_parent: ::core::ffi::c_int,
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

/// Stable identity for a dep edge: content-hash of the full (immutable) `DepNode`.
id_wireformat!(DepId[HASH_SIZE] <- DepNode);

/// Stable identity for a file: derived from its canonical name only.
/// Mutable runtime state (timestamps, flags, command state) does not
/// contribute to the key, so a file's identity survives updates.
id_wireformat!(FileId[HASH_SIZE] |f: String| f.as_str());

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DepFlags: u32 {
        const NONE = 0;
        // fill these from GNU make’s DEP_* flags
        // const SOME_FLAG = 1 << 0;
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
use crate::hash::{
    hash_delete, hash_deleted_item, hash_dump, hash_find_item, hash_find_slot, hash_init,
    hash_insert_at, hash_map, hash_print_stats, jhash_string,
};
use crate::make_main::{
    cmd_prefix, db_level, export_all_variables, ignore_errors_flag, just_print_flag,
    no_builtin_rules_flag, no_intermediates, not_parallel, question_flag, run_silent,
    second_expansion, stopchar_map, touch_flag, verify_flag,
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
    fn bool_value(value: bool) -> ::core::ffi::c_uint {
        value as ::core::ffi::c_uint
    }

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

    pub fn is_target(&self) -> ::core::ffi::c_uint {
        self.is_target as ::core::ffi::c_uint
    }

    pub fn set_is_target(&mut self, value: ::core::ffi::c_uint) {
        self.is_target = value != 0;
    }

    pub fn suffix(&self) -> ::core::ffi::c_uint {
        self.suffix as ::core::ffi::c_uint
    }

    pub fn set_suffix(&mut self, value: ::core::ffi::c_uint) {
        self.suffix = value != 0;
    }

    pub fn precious(&self) -> ::core::ffi::c_uint {
        Self::bool_value(self.precious)
    }

    pub fn loaded(&self) -> ::core::ffi::c_uint {
        Self::bool_value(self.loaded)
    }

    pub fn set_loaded(&mut self, value: ::core::ffi::c_uint) {
        self.loaded = value != 0;
    }

    pub fn set_unloaded(&mut self, value: ::core::ffi::c_uint) {
        self.unloaded = value != 0;
    }

    pub fn updating(&self) -> ::core::ffi::c_uint {
        Self::bool_value(self.updating)
    }

    pub fn set_updating(&mut self, value: ::core::ffi::c_uint) {
        self.updating = value != 0;
    }

    pub fn updated(&self) -> ::core::ffi::c_uint {
        Self::bool_value(self.updated)
    }

    pub fn set_updated(&mut self, value: ::core::ffi::c_uint) {
        self.updated = value != 0;
    }

    pub fn phony(&self) -> ::core::ffi::c_uint {
        Self::bool_value(self.phony)
    }

    pub fn dontcare(&self) -> ::core::ffi::c_uint {
        Self::bool_value(self.dontcare)
    }

    pub fn set_dontcare(&mut self, value: ::core::ffi::c_uint) {
        self.dontcare = value != 0;
    }

    pub fn no_diag(&self) -> ::core::ffi::c_uint {
        Self::bool_value(self.no_diag)
    }

    pub fn set_no_diag(&mut self, value: ::core::ffi::c_uint) {
        self.no_diag = value != 0;
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
}

impl crate::content_hash::ContentHash for DepFlags {
    fn hash(&self, state: &mut impl crate::content_hash::DigestUpdate) {
        state.update(&self.bits().to_le_bytes());
    }
}

pub type hash_map_func_t = crate::hash::hash_map_func_t;
pub type qsort_cmp_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const INTSTR_LENGTH: usize = 53_usize
    .wrapping_mul(::core::mem::size_of::<uintmax_t>() as usize)
    .wrapping_div(22_usize)
    .wrapping_add(3_usize);
pub const RECIPEPREFIX_DEFAULT: ::core::ffi::c_int = '\t' as i32;
pub const COMMANDS_SILENT: ::core::ffi::c_int = 2;
pub const COMMANDS_NOERROR: ::core::ffi::c_int = 4;
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
pub const UNKNOWN_MTIME: ::core::ffi::c_int = 0;
pub const NONEXISTENT_MTIME: ::core::ffi::c_int = 1;
pub const OLD_MTIME: ::core::ffi::c_int = 2;
pub const ORDINARY_MTIME_MIN: ::core::ffi::c_int = OLD_MTIME + 1;
pub static mut snapped_deps: ::core::ffi::c_int = 0;
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_hash_1(key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    let mut _key_: *const ::core::ffi::c_uchar =
        (*(key as *const File)).hname as *const ::core::ffi::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as ::core::ffi::c_ulong);
    _result_
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_hash_2(mut _key: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    let mut _result_: ::core::ffi::c_ulong = 0;
    _result_
}
unsafe fn file_hash_cmp(
    x: *const ::core::ffi::c_void,
    y: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if (*(x as *const File)).hname == (*(y as *const File)).hname {
        0
    } else {
        strcmp((*(x as *const File)).hname, (*(y as *const File)).hname)
    }
}
static mut files: hash_table = hash_table {
    ht_vec: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
    ht_in_map: [0; 1],
    c2rust_padding: [0; 3],
};
static mut rehashed_files: *mut *mut File = ::core::ptr::null::<*mut File>() as *mut *mut File;
static mut rehashed_files_len: size_t = 0;
pub const REHASHED_FILES_INCR: ::core::ffi::c_int = 5;
const MAP_DIRSEP: ::core::ffi::c_int = 0x8000;
const STOPCHAR_MAP_LEN: usize = 256;
static mut all_secondary: ::core::ffi::c_int = 0;
fn is_dirsep(ch: u8) -> bool {
    ch == b'/' || cfg!(windows) && ch == b'\\'
}

unsafe fn normalize_lookup_name(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    assert!(
        !name.is_null() && *name != 0,
        "lookup_file name must be non-empty"
    );
    let name_with_nul = ::core::ffi::CStr::from_ptr(name).to_bytes_with_nul();
    let name_bytes = &name_with_nul[..name_with_nul.len() - 1];
    let mut offset = 0usize;
    while name_bytes.get(offset) == Some(&b'.')
        && name_bytes.get(offset + 1).is_some_and(|ch| is_dirsep(*ch))
        && offset + 2 < name_bytes.len()
    {
        offset += 2;
        while name_bytes.get(offset).is_some_and(|ch| is_dirsep(*ch)) {
            offset += 1;
        }
    }

    if offset == name_bytes.len() {
        b"./\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        name.add(offset)
    }
}

#[no_mangle]
pub unsafe extern "C" fn lookup_file(name: *const ::core::ffi::c_char) -> *mut File {
    let f: *mut File;
    let mut file_key: File = File::default();
    let name = normalize_lookup_name(name);
    file_key.hname = name;
    f = hash_find_item(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut File;
    f
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn enter_file(name: *const ::core::ffi::c_char) -> *mut file {
    let f: *mut file;
    let new: *mut file;
    let file_slot: *mut *mut file;
    let mut file_key: file = File::default();
    if *name as ::core::ffi::c_int != 0 {
    } else {
        panic!("assertion failed: *name != '\'");
    };
    if verify_flag == 0 || strcache_iscached(name) != 0 {
    } else {
        panic!("assertion failed: ! verify_flag || strcache_iscached (name)");
    };
    file_key.hname = name;
    let file_slot: *mut *mut File = hash_find_slot(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut *mut File;
    let f: *mut File = *file_slot;
    if !(f.is_null() || std::ptr::eq(f as *mut ::core::ffi::c_void, hash_deleted_item))
        && (*f).double_colon.is_null()
    {
        (*f).builtin = false;
        return f;
    }
    let new: *mut File = xcalloc(::core::mem::size_of::<File>() as size_t) as *mut File;
    (*new).hname = name;
    (*new).name = (*new).hname;
    (*new).update_status = UpdateStatus::None;
    if f.is_null() || std::ptr::eq(f as *mut ::core::ffi::c_void, hash_deleted_item) {
        (*new).last = new;
        hash_insert_at(
            &raw mut files,
            new as *const ::core::ffi::c_void,
            file_slot as *const ::core::ffi::c_void,
        );
    } else {
        (*new).double_colon = f;
        (*(*f).last).prev = new;
        (*f).last = new;
    }
    new
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn rehash_file(mut from_file: *mut file, to_hname: *const ::core::ffi::c_char) {
    let mut file_key: file = File::default();
    let file_slot: *mut *mut File;
    let to_file: *mut File;
    let deleted_file: *mut File;
    let mut f: *mut File;
    (*from_file).builtin = false;
    file_key.hname = to_hname;
    if file_hash_cmp(
        from_file as *const ::core::ffi::c_void,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) == 0
    {
        return;
    }
    file_key.hname = (*from_file).hname;
    while !(*from_file).renamed.is_null() {
        from_file = (*from_file).renamed;
    }
    if file_hash_cmp(
        from_file as *const ::core::ffi::c_void,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) != 0
    {
        abort();
    }
    let deleted_file: *mut File =
        hash_delete(&raw mut files, from_file as *const ::core::ffi::c_void) as *mut File;
    if deleted_file != from_file {
        abort();
    }
    file_key.hname = to_hname;
    let file_slot: *mut *mut File = hash_find_slot(
        &raw mut files,
        &raw mut file_key as *const ::core::ffi::c_void,
    ) as *mut *mut File;
    let to_file: *mut File = *file_slot;
    (*from_file).hname = to_hname;
    f = (*from_file).double_colon;
    while !f.is_null() {
        (*f).hname = to_hname;
        f = (*f).prev;
    }
    if to_file.is_null() || std::ptr::eq(to_file as *mut ::core::ffi::c_void, hash_deleted_item) {
        hash_insert_at(
            &raw mut files,
            from_file as *const ::core::ffi::c_void,
            file_slot as *const ::core::ffi::c_void,
        );
        return;
    }
    if !(*from_file).cmds.is_null() {
        if (*to_file).cmds.is_null() {
            (*to_file).cmds = (*from_file).cmds;
        } else if (*from_file).cmds != (*to_file).cmds {
            if !(*(*to_file).cmds).fileinfo.filenm.is_null() {
                error(
                    &raw mut (*(*from_file).cmds).fileinfo,
                    b"recipe was specified for file '%s' at %s:%lu,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        FmtArg::Str(((*from_file).name) as *const ::core::ffi::c_char),
                        FmtArg::Str(
                            ((*(*from_file).cmds).fileinfo.filenm) as *const ::core::ffi::c_char,
                        ),
                        FmtArg::Uint(((*(*from_file).cmds).fileinfo.lineno) as u64),
                    ],
                );
            } else {
                error(
                    &raw mut (*(*from_file).cmds).fileinfo,
                    b"recipe for file '%s' was found by implicit rule search,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[FmtArg::Str(
                        ((*from_file).name) as *const ::core::ffi::c_char,
                    )],
                );
            }
            error(
                &raw mut (*(*from_file).cmds).fileinfo,
                b"but '%s' is now considered the same file as '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str(((*from_file).name) as *const ::core::ffi::c_char),
                    FmtArg::Str((to_hname) as *const ::core::ffi::c_char),
                ],
            );
            error(
                &raw mut (*(*from_file).cmds).fileinfo,
                b"recipe for '%s' will be ignored in favor of the one for '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str(((*from_file).name) as *const ::core::ffi::c_char),
                    FmtArg::Str((to_hname) as *const ::core::ffi::c_char),
                ],
            );
        }
    }
    if (*to_file).deps.is_null() {
        (*to_file).deps = (*from_file).deps;
    } else {
        let mut deps: *mut Dep = (*to_file).deps;
        while !(*deps).next.is_null() {
            deps = (*deps).next;
        }
        (*deps).next = (*from_file).deps;
    }
    merge_variable_set_lists(&raw mut (*to_file).variables, (*from_file).variables);
    if !(*to_file).double_colon.is_null()
        && (*from_file).is_target
        && (*from_file).double_colon.is_null()
    {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            b"can't rename single-colon '%s' to double-colon '%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                FmtArg::Str(((*from_file).name) as *const ::core::ffi::c_char),
                FmtArg::Str((to_hname) as *const ::core::ffi::c_char),
            ],
        );
    }
    if (*to_file).double_colon.is_null() && !(*from_file).double_colon.is_null() {
        if (*to_file).is_target {
            fatal(
                ::core::ptr::null_mut::<Floc>(),
                b"can't rename double-colon '%s' to single-colon '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str(((*from_file).name) as *const ::core::ffi::c_char),
                    FmtArg::Str((to_hname) as *const ::core::ffi::c_char),
                ],
            );
        } else {
            (*to_file).double_colon = (*from_file).double_colon;
        }
    }
    if (*from_file).last_mtime > (*to_file).last_mtime {
        (*to_file).last_mtime = (*from_file).last_mtime;
    }
    (*to_file).mtime_before_update = (*from_file).mtime_before_update;
    (*to_file).precious |= (*from_file).precious;
    (*to_file).loaded |= (*from_file).loaded;
    (*to_file).tried_implicit |= (*from_file).tried_implicit;
    (*to_file).updating |= (*from_file).updating;
    (*to_file).updated |= (*from_file).updated;
    (*to_file).is_target |= (*from_file).is_target;
    (*to_file).cmd_target |= (*from_file).cmd_target;
    (*to_file).phony |= (*from_file).phony;
    (*to_file).is_explicit |= (*from_file).is_explicit;
    (*to_file).secondary |= (*from_file).secondary;
    (*to_file).notintermediate |= (*from_file).notintermediate;
    (*to_file).ignore_vpath |= (*from_file).ignore_vpath;
    (*to_file).snapped |= (*from_file).snapped;
    (*to_file).suffix |= (*from_file).suffix;
    (*to_file).builtin = false;
    (*from_file).renamed = to_file;
    if rehashed_files_len.wrapping_rem(REHASHED_FILES_INCR as size_t) == 0 {
        rehashed_files = xrealloc(
            rehashed_files as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut File>() as size_t)
                .wrapping_mul(rehashed_files_len.wrapping_add(REHASHED_FILES_INCR as size_t)),
        ) as *mut *mut File;
    }
    let fresh2 = rehashed_files_len;
    rehashed_files_len = rehashed_files_len.wrapping_add(1);
    let fresh3 = &mut (*rehashed_files.offset(fresh2 as isize));
    *fresh3 = from_file;
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn rename_file(mut from_file: *mut file, to_hname: *const ::core::ffi::c_char) {
    rehash_file(from_file, to_hname);
    while !from_file.is_null() {
        (*from_file).name = (*from_file).hname;
        from_file = (*from_file).prev;
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn remove_intermediates(sig: ::core::ffi::c_int) {
    let mut file_slot: *mut *mut file;
    let file_end: *mut *mut file;
    let mut doneany: ::core::ffi::c_int = 0;
    if question_flag != 0 || touch_flag != 0 || all_secondary != 0 || no_intermediates != 0 {
        return;
    }
    if sig != 0 && just_print_flag != 0 {
        return;
    }
    file_slot = files.ht_vec as *mut *mut File;
    file_end = file_slot.offset(files.ht_size as isize);
    while file_slot < file_end {
        if !((*file_slot).is_null()
            || *file_slot as *mut ::core::ffi::c_void
                == hash_deleted_item as *mut ::core::ffi::c_void)
        {
            let f: *mut File = *file_slot;
            if (*f).intermediate
                && ((*f).dontcare || !(*f).precious)
                && !(*f).secondary
                && !(*f).notintermediate
                && !(*f).cmd_target
            {
                let status: ::core::ffi::c_int;
                if (*f).update_status as ::core::ffi::c_int
                    != UpdateStatus::None as ::core::ffi::c_int
                {
                    // ENOENT from unlink means the file was already gone: skip the
                    // diagnostic/bookkeeping below (the C code `continue`d here).
                    let skip: bool;
                    if just_print_flag != 0 {
                        status = 0;
                        skip = false;
                    } else {
                        status = unlink((*f).name);
                        skip = status < 0 && *__errno_location() == ENOENT;
                    }
                    if !skip && !(*f).dontcare {
                        if sig != 0 {
                            error(
                                ::core::ptr::null_mut::<Floc>(),
                                b"*** deleting intermediate file '%s'\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[FmtArg::Str(((*f).name) as *const ::core::ffi::c_char)],
                            );
                        } else {
                            if doneany == 0 && 0x1 as ::core::ffi::c_int & db_level != 0 {
                                printf(
                                    b"Removing intermediate files...\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                fflush(stdout);
                            }
                            if run_silent == 0 {
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
                                b"unlink: \0" as *const u8 as *const ::core::ffi::c_char,
                                (*f).name,
                            );
                            doneany = 0;
                        }
                    }
                }
            }
        }
        file_slot = file_slot.offset(1 as ::core::ffi::c_int as isize);
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
pub unsafe fn split_prereqs(mut p: *mut ::core::ffi::c_char) -> *mut dep {
    let mut new: *mut dep = parse_file_seq(
        &raw mut p,
        0x100 as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0x40 as ::core::ffi::c_int,
    );
    if *p != 0 {
        let mut ood: *mut Dep;
        p = p.offset(1 as ::core::ffi::c_int as isize);
        ood = parse_file_seq::<Dep>(
            &raw mut p,
            0x1 as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0x40 as ::core::ffi::c_int,
        );
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
pub unsafe fn enter_prereqs(mut deps: *mut dep, stem: *const ::core::ffi::c_char) -> *mut dep {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let _d1: *mut Dep;
    if deps.is_null() {
        return ::core::ptr::null_mut::<Dep>();
    }
    if !stem.is_null() {
        let pattern: *const ::core::ffi::c_char = b"%\0" as *const u8 as *const ::core::ffi::c_char;
        let mut dp: *mut Dep = deps;
        let mut dl: *mut Dep = ::core::ptr::null_mut::<Dep>();
        while let Some(dp_ref) = dp.as_mut() {
            let percent: *mut ::core::ffi::c_char;
            let nl: size_t = (strlen(dp_ref.name) as size_t).wrapping_add(1);
            alloca_allocations.push(::std::vec::from_elem(0, nl as usize));
            let nm: *mut ::core::ffi::c_char =
                alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_char;
            memcpy(
                nm as *mut ::core::ffi::c_void,
                dp_ref.name as *const ::core::ffi::c_void,
                nl as size_t,
            );
            percent = find_percent(nm);
            if !percent.is_null() {
                let o: *mut ::core::ffi::c_char;
                if *stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 {
                    memmove(
                        percent as *mut ::core::ffi::c_void,
                        percent.offset(1 as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
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
                        pattern.offset(1 as ::core::ffi::c_int as isize),
                        percent.offset(1 as ::core::ffi::c_int as isize),
                    );
                }
                if *variable_buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0
                {
                    let df: *mut Dep = dp;
                    if dp == deps {
                        deps = dp_ref.next;
                        dp = deps;
                    } else {
                        let dl_ref = dl
                            .as_mut()
                            .expect("previous dependency is null while unlinking");
                        dl_ref.next = dp_ref.next;
                        dp = dl_ref.next;
                    }
                    free_dep(df);
                    continue;
                } else {
                    dp_ref.name = strcache_add_len(
                        variable_buffer,
                        o.offset_from(variable_buffer) as ::core::ffi::c_long as size_t,
                    );
                }
            }
            dp_ref.stem = stem;
            dp_ref.staticpattern = true;
            dl = dp;
            dp = dp_ref.next;
        }
    }
    for d1 in seq_iter(deps) {
        if !((*d1).need_2nd_expansion) {
            (*d1).file = lookup_file((*d1).name);
            if (*d1).file.is_null() {
                (*d1).file = enter_file((*d1).name);
            }
            (*d1).staticpattern = false;
            (*d1).name = ::core::ptr::null::<::core::ffi::c_char>();
            if stem.is_null() {
                (*(*d1).file).is_explicit = true;
            }
        }
    }
    deps
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn expand_deps(f: *mut file) {
    let mut d: *mut dep;
    let mut dp: *mut *mut dep;
    let mut fstem: *const ::core::ffi::c_char;
    let mut initialized: ::core::ffi::c_int = 0;
    let mut changed_dep: ::core::ffi::c_int = 0;
    if (*f).snapped {
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
                    cs = cs.offset(1 as ::core::ffi::c_int as isize);
                }
                if nperc != 0 {
                    let slen: size_t = (strlen((*d).name) as size_t)
                        .wrapping_add(nperc)
                        .wrapping_add(1);
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
                        s = s.offset(1 as ::core::ffi::c_int as isize);
                        *fresh0 = '$' as i32 as ::core::ffi::c_char;
                        let fresh1 = s;
                        s = s.offset(1 as ::core::ffi::c_int as isize);
                        *fresh1 = '*' as i32 as ::core::ffi::c_char;
                        cs = cs.offset(1 as ::core::ffi::c_int as isize);
                        pcs = cs;
                        cs = strchr(end_of_token(cs), '%' as i32);
                    }
                    strcpy(s, pcs);
                    free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
                    (*d).name = name;
                }
            }
            if initialized == 0 {
                initialize_file_variables(f, 0);
                initialized = 1;
            }
            set_file_variables(
                f,
                if !(*d).stem.is_null() {
                    (*d).stem
                } else {
                    (*f).stem
                },
            );
            p = expand_string_for_file((*d).name, f);
            free((*d).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            new = split_prereqs(p);
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
                    (*d).file = lookup_file((*d).name);
                    if (*d).file.is_null() {
                        (*d).file = enter_file((*d).name);
                    }
                    (*d).name = ::core::ptr::null::<::core::ffi::c_char>();
                    (*d).stem = fstem;
                    if fstem.is_null() {
                        (*(*d).file).is_explicit = true;
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
pub unsafe fn expand_extra_prereqs(extra: *const variable) -> *mut dep {
    let mut d: *mut dep;
    let prereqs: *mut dep = if !extra.is_null() {
        split_prereqs(expand_string_buf(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            (*extra).value,
            SIZE_MAX as size_t,
        ))
    } else {
        ::core::ptr::null_mut::<Dep>()
    };
    for d in seq_iter(prereqs) {
        (*d).file = lookup_file((*d).name);
        if (*d).file.is_null() {
            (*d).file = enter_file((*d).name);
        }
        (*d).name = ::core::ptr::null::<::core::ffi::c_char>();
        (*d).ignore_automatic_vars = true;
    }
    prereqs
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn snap_file(f: *mut file, deps: *const dep) {
    let mut prereqs: *mut dep = ::core::ptr::null_mut::<dep>();
    let mut d: *mut dep;
    if second_expansion == 0 {
        (*f).updating = false;
    }
    if all_secondary != 0 && !(*f).notintermediate {
        (*f).intermediate = true;
    }
    if no_intermediates != 0 && !(*f).intermediate && !(*f).secondary {
        (*f).notintermediate = true;
    }
    if !(*f).variables.is_null() {
        prereqs = expand_extra_prereqs(lookup_variable_in_set(
            b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
            (*(*f).variables).set,
        ));
        if second_expansion != 0 {
            for d in seq_iter(prereqs) {
                if (*d).name.is_null() {
                    (*d).name = xstrdup((*(*d).file).name);
                }
                (*d).need_2nd_expansion = true;
            }
        }
    } else if (*f).is_target {
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
            let fname = (*f).name;
            let same = match (dname.as_ref(), fname.as_ref()) {
                (Some(&db), Some(&fb)) => {
                    fb as ::core::ffi::c_int == db as ::core::ffi::c_int
                        && (fb as ::core::ffi::c_int == 0
                            || strcmp(fname.offset(1), dname.offset(1)) == 0)
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
        } else if (*f).deps.is_null() {
            (*f).deps = prereqs;
        } else {
            d = (*f).deps;
            while !(*d).next.is_null() {
                d = (*d).next;
            }
            (*d).next = prereqs;
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn snap_deps() {
    let mut f: *mut file;
    let mut f2: *mut file;
    let mut d: *mut dep;
    snapped_deps = 1;
    f = lookup_file(b".PRECIOUS\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        for d in seq_iter((*f).deps) {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).precious = true;
                f2 = (*f2).prev;
            }
        }
        f = (*f).prev;
    }
    f = lookup_file(b".LOW_RESOLUTION_TIME\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        for d in seq_iter((*f).deps) {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).low_resolution_time = true;
                f2 = (*f2).prev;
            }
        }
        f = (*f).prev;
    }
    f = lookup_file(b".PHONY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        for d in seq_iter((*f).deps) {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).phony = true;
                (*f2).is_target = true;
                (*f2).last_mtime = NONEXISTENT_MTIME as uintmax_t;
                (*f2).mtime_before_update = NONEXISTENT_MTIME as uintmax_t;
                f2 = (*f2).prev;
            }
        }
        f = (*f).prev;
    }
    f = lookup_file(b".NOTINTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        if !(*f).deps.is_null() {
            for d in seq_iter((*f).deps) {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).notintermediate = true;
                    f2 = (*f2).prev;
                }
            }
        } else {
            no_intermediates = 1;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".INTERMEDIATE\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        for d in seq_iter((*f).deps) {
            f2 = (*d).file;
            while !f2.is_null() {
                if (*f2).notintermediate {
                    fatal(
                        ::core::ptr::null_mut::<Floc>(),
                        b"%s cannot be both .NOTINTERMEDIATE and .INTERMEDIATE\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[FmtArg::Str(((*f2).name) as *const ::core::ffi::c_char)],
                    );
                } else {
                    (*f2).intermediate = true;
                }
                f2 = (*f2).prev;
            }
        }
        f = (*f).prev;
    }
    f = lookup_file(b".SECONDARY\0" as *const u8 as *const ::core::ffi::c_char);
    while !f.is_null() {
        if !(*f).deps.is_null() {
            for d in seq_iter((*f).deps) {
                f2 = (*d).file;
                while !f2.is_null() {
                    if (*f2).notintermediate {
                        fatal(
                            ::core::ptr::null_mut::<Floc>(),
                            b"%s cannot be both .NOTINTERMEDIATE and .SECONDARY\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[FmtArg::Str(((*f2).name) as *const ::core::ffi::c_char)],
                        );
                    } else {
                        let rhs = {
                            (*f2).secondary = true;
                            (*f2).secondary
                        } as ::core::ffi::c_uint;
                        (*f2).intermediate = (rhs) != 0;
                    }
                    f2 = (*f2).prev;
                }
            }
        } else {
            all_secondary = 1;
        }
        f = (*f).prev;
    }
    if no_intermediates != 0 && all_secondary != 0 {
        fatal(
            ::core::ptr::null_mut::<Floc>(),
            b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    f = lookup_file(b".EXPORT_ALL_VARIABLES\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target {
        export_all_variables = 1;
    }
    f = lookup_file(b".IGNORE\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target {
        if (*f).deps.is_null() {
            ignore_errors_flag = 1;
        } else {
            for d in seq_iter((*f).deps) {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= COMMANDS_NOERROR;
                    f2 = (*f2).prev;
                }
            }
        }
    }
    f = lookup_file(b".SILENT\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target {
        if (*f).deps.is_null() {
            run_silent = 1;
        } else {
            for d in seq_iter((*f).deps) {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= COMMANDS_SILENT;
                    f2 = (*f2).prev;
                }
            }
        }
    }
    f = lookup_file(b".NOTPARALLEL\0" as *const u8 as *const ::core::ffi::c_char);
    if !f.is_null() && (*f).is_target {
        let mut d2: *mut Dep;
        if (*f).deps.is_null() {
            not_parallel = 1;
        } else {
            for d in seq_iter((*f).deps) {
                f2 = (*d).file;
                while !f2.is_null() {
                    if !(*f2).deps.is_null() {
                        d2 = (*(*f2).deps).next;
                        while !d2.is_null() {
                            (*d2).wait_here = true;
                            d2 = (*d2).next;
                        }
                    }
                    f2 = (*f2).prev;
                }
            }
        }
    }
    let prereqs: *mut Dep = expand_extra_prereqs(lookup_variable(
        b".EXTRA_PREREQS\0" as *const u8 as *const ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 15]>() as size_t).wrapping_sub(1),
    ));
    let filedump: *mut *mut ::core::ffi::c_void = hash_dump(
        &raw mut files,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        None,
    );
    let mut filep: *mut *mut ::core::ffi::c_void = filedump;
    while !(*filep).is_null() {
        snap_file(*filep as *mut File, prereqs);
        filep = filep.offset(1 as ::core::ffi::c_int as isize);
    }
    free(filedump as *mut ::core::ffi::c_void);
    free_dep_chain(prereqs);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn set_command_state(file: *mut file, state: cmd_state) {
    let mut d: *mut dep;
    (*file).set_command_state(state as cmd_state as cmd_state);
    d = (*file).also_make;
    while !d.is_null() {
        if state as ::core::ffi::c_uint > (*(*d).file).command_state as ::core::ffi::c_uint {
            (*(*d).file).command_state = state as CommandState as CommandState;
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_timestamp_cons(
    fname: *const ::core::ffi::c_char,
    stamp: time_t,
    ns: ::core::ffi::c_long,
) -> uintmax_t {
    let offset: ::core::ffi::c_int = (ORDINARY_MTIME_MIN as ::core::ffi::c_long
        + (if FILE_TIMESTAMP_HI_RES != 0 { ns } else { 0 }))
        as ::core::ffi::c_int;
    let s: uintmax_t = stamp as uintmax_t;
    let product: uintmax_t = s << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 });
    let mut ts: uintmax_t = product.wrapping_add(offset as uintmax_t);
    if !(s
        <= ((!(0 as ::core::ffi::c_int as uintmax_t))
            .wrapping_sub(
                if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                    0 as ::core::ffi::c_int as uintmax_t
                } else {
                    !(0 as ::core::ffi::c_int as uintmax_t)
                        << (::core::mem::size_of::<uintmax_t>() as usize)
                            .wrapping_mul(8 as usize)
                            .wrapping_sub(1 as usize)
                },
            )
            .wrapping_sub((2 + 1) as uintmax_t)
            >> (if 1 != 0 { 30 } else { 0 })
            << (if 1 != 0 { 30 } else { 0 }))
        .wrapping_add((2 + 1) as uintmax_t)
        .wrapping_add(
            (if 1 != 0 {
                1000000000 as ::core::ffi::c_int
            } else {
                1
            }) as uintmax_t,
        )
        .wrapping_sub(1 as uintmax_t)
        .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
            >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
        && product <= ts
        && ts
            <= ((!(0 as ::core::ffi::c_int as uintmax_t))
                .wrapping_sub(
                    if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                        0 as ::core::ffi::c_int as uintmax_t
                    } else {
                        !(0 as ::core::ffi::c_int as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8 as usize)
                                .wrapping_sub(1 as usize)
                    },
                )
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
                << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
            .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000 as ::core::ffi::c_int
                } else {
                    1
                }) as uintmax_t,
            )
            .wrapping_sub(1 as uintmax_t))
    {
        let mut buf: [::core::ffi::c_char; 43] = [0; 43];
        let f: *const ::core::ffi::c_char = if !fname.is_null() {
            fname
        } else {
            b"Current time\0" as *const u8 as *const ::core::ffi::c_char
        };
        ts = if s <= OLD_MTIME as uintmax_t {
            ORDINARY_MTIME_MIN as uintmax_t
        } else {
            ((!(0 as ::core::ffi::c_int as uintmax_t))
                .wrapping_sub(
                    if !(-(1 as ::core::ffi::c_int) as uintmax_t <= 0 as uintmax_t) {
                        0 as ::core::ffi::c_int as uintmax_t
                    } else {
                        !(0 as ::core::ffi::c_int as uintmax_t)
                            << (::core::mem::size_of::<uintmax_t>() as usize)
                                .wrapping_mul(8 as usize)
                                .wrapping_sub(1 as usize)
                    },
                )
                .wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
                >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })
                << (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 }))
            .wrapping_add(ORDINARY_MTIME_MIN as uintmax_t)
            .wrapping_add(
                (if FILE_TIMESTAMP_HI_RES != 0 {
                    1000000000 as ::core::ffi::c_int
                } else {
                    1
                }) as uintmax_t,
            )
            .wrapping_sub(1 as uintmax_t)
        };
        file_timestamp_sprintf(&raw mut buf as *mut ::core::ffi::c_char, ts);
        error(
            ::core::ptr::null_mut::<Floc>(),
            b"%s: timestamp out of range: substituting %s\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                FmtArg::Str((f) as *const ::core::ffi::c_char),
                FmtArg::Str(
                    (&raw mut buf as *mut ::core::ffi::c_char) as *const ::core::ffi::c_char,
                ),
            ],
        );
    }
    ts
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_timestamp_now(resolution: *mut ::core::ffi::c_int) -> uintmax_t {
    let r: ::core::ffi::c_int;
    let s: time_t;
    let ns: ::core::ffi::c_int;
    let mut timespec: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_REALTIME, &raw mut timespec) == 0 {
        r = 1;
        s = timespec.tv_sec as time_t;
        ns = timespec.tv_nsec as ::core::ffi::c_int;
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
            r = 1000 as ::core::ffi::c_int;
            s = timeval.tv_sec as time_t;
            ns = (timeval.tv_usec * 1000 as __suseconds_t) as ::core::ffi::c_int;
        } else {
            r = 1000000000 as ::core::ffi::c_int;
            s = time(::core::ptr::null_mut::<time_t>());
            ns = 0;
        }
    }
    *resolution = r;
    file_timestamp_cons(
        ::core::ptr::null::<::core::ffi::c_char>(),
        s,
        ns as ::core::ffi::c_long,
    )
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn file_timestamp_sprintf(mut p: *mut ::core::ffi::c_char, ts: uintmax_t) {
    let mut t: time_t = (ts.wrapping_sub(ORDINARY_MTIME_MIN as uintmax_t)
        >> (if FILE_TIMESTAMP_HI_RES != 0 { 30 } else { 0 })) as time_t;
    let tm: *mut tm = localtime(&raw mut t);
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
                as ::core::ffi::c_int,
        ) - 1) as isize,
    );
    while *p as ::core::ffi::c_int == '0' as i32 {
        p = p.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    p = p.offset((*p as ::core::ffi::c_int != '.' as i32) as ::core::ffi::c_int as isize);
    *p = 0;
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
                if (*deps).wait_here {
                    b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !(*deps).name.is_null() {
                    (*deps).name
                } else {
                    (*(*deps).file).name
                },
            );
        } else if ood.is_null() {
            ood = deps;
        }
        deps = (*deps).next;
    }
    if !ood.is_null() {
        let ood_ref = ood.as_ref().expect("order-only dependency is null");
        printf(
            b" | %s%s\0" as *const u8 as *const ::core::ffi::c_char,
            if ood_ref.wait_here {
                b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !ood_ref.name.is_null() {
                ood_ref.name
            } else {
                ood_ref
                    .file
                    .as_ref()
                    .expect("order-only dependency has a null file")
                    .name
            },
        );
        for ood in seq_iter(ood_ref.next) {
            if (*ood).ignore_mtime {
                printf(
                    b" %s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    if (*ood).wait_here {
                        b".WAIT \0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    if !(*ood).name.is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    },
                );
            }
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
    if no_builtin_rules_flag != 0 && (*f).builtin() as ::core::ffi::c_int != 0 {
        return;
    }
    putchar('\n' as i32);
    if !(*f).cmds.is_null()
        && (*(*f).cmds).recipe_prefix as ::core::ffi::c_int != cmd_prefix as ::core::ffi::c_int
    {
        fputs(
            b".RECIPEPREFIX = \0" as *const u8 as *const ::core::ffi::c_char,
            stdout,
        );
        cmd_prefix = (*(*f).cmds).recipe_prefix;
        if cmd_prefix as ::core::ffi::c_int != RECIPEPREFIX_DEFAULT {
            putchar(cmd_prefix as ::core::ffi::c_int);
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
    puts(if (*f).tried_implicit {
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
                    (*(*d).file).name
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
        let mut buf: [::core::ffi::c_char; 43] = [0; 43];
        file_timestamp_sprintf(&raw mut buf as *mut ::core::ffi::c_char, (*f).last_mtime);
        printf(
            b"#  Last modified %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    }
    puts(if (*f).updated {
        b"#  File has been updated.\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"#  File has not been updated.\0" as *const u8 as *const ::core::ffi::c_char
    });
    match (*f).command_state as ::core::ffi::c_int {
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
        0 | 3 => match (*f).update_status as ::core::ffi::c_int {
            0 => {
                puts(b"#  Successfully updated.\0" as *const u8 as *const ::core::ffi::c_char);
            }
            2 => {
                if question_flag != 0 {
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
pub unsafe fn print_file_data_base() {
    puts(b"\n# Files\0" as *const u8 as *const ::core::ffi::c_char);
    hash_map(&raw mut files, Some(print_file));
    fputs(
        b"\n# files hash-table stats:\n# \0" as *const u8 as *const ::core::ffi::c_char,
        stdout,
    );
    hash_print_stats(&raw mut files, stdout);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_target(item: *const ::core::ffi::c_void) {
    let f: *const file = item as *const file;
    if (*f).is_target() == 0 || (*f).suffix() as ::core::ffi::c_int != 0 {
        return;
    }
    if *(*f).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
        && *(*__ctype_b_loc()).offset(*(*f).name.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uchar as ::core::ffi::c_int
            as isize) as ::core::ffi::c_int
            & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        let mut cp: *const ::core::ffi::c_char = (*f).name.offset(1 as ::core::ffi::c_int as isize);
        loop {
            cp = cp.offset(1 as ::core::ffi::c_int as isize);
            if !(*cp as ::core::ffi::c_int != 0) {
                break;
            }
            if *(*__ctype_b_loc())
                .offset(*cp as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                == 0
            {
                break;
            }
        }
        if *cp as ::core::ffi::c_int == 0 {
            return;
        }
    }
    puts((*f).name);
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn print_targets() {
    hash_map(&raw mut files, Some(print_target));
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file(item: *const ::core::ffi::c_void) {
    let f: *const file = item as *const file;
    let mut d: *const dep;
    if !(*f).name.is_null()
        && *(*f).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).name) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                FmtArg::Str(((*f).name) as *const ::core::ffi::c_char),
                FmtArg::Str(
                    (b"name\0" as *const u8 as *const ::core::ffi::c_char)
                        as *const ::core::ffi::c_char,
                ),
                FmtArg::Str(((*f).name) as *const ::core::ffi::c_char),
            ],
        );
    }
    if !(*f).hname.is_null()
        && *(*f).hname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).hname) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                FmtArg::Str(((*f).name) as *const ::core::ffi::c_char),
                FmtArg::Str(
                    (b"hname\0" as *const u8 as *const ::core::ffi::c_char)
                        as *const ::core::ffi::c_char,
                ),
                FmtArg::Str(((*f).hname) as *const ::core::ffi::c_char),
            ],
        );
    }
    if !(*f).vpath.is_null()
        && *(*f).vpath.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).vpath) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                FmtArg::Str(((*f).name) as *const ::core::ffi::c_char),
                FmtArg::Str(
                    (b"vpath\0" as *const u8 as *const ::core::ffi::c_char)
                        as *const ::core::ffi::c_char,
                ),
                FmtArg::Str(((*f).vpath) as *const ::core::ffi::c_char),
            ],
        );
    }
    if !(*f).stem.is_null()
        && *(*f).stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        && strcache_iscached((*f).stem) == 0
    {
        error(
            ::core::ptr::null::<Floc>(),
            b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                FmtArg::Str(((*f).name) as *const ::core::ffi::c_char),
                FmtArg::Str(
                    (b"stem\0" as *const u8 as *const ::core::ffi::c_char)
                        as *const ::core::ffi::c_char,
                ),
                FmtArg::Str(((*f).stem) as *const ::core::ffi::c_char),
            ],
        );
    }
    for d in seq_iter((*f).deps) {
        if !(*d).need_2nd_expansion
            && !(*d).name.is_null()
            && *(*d).name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strcache_iscached((*d).name) == 0
        {
            error(
                ::core::ptr::null::<Floc>(),
                b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str(((*d).name) as *const ::core::ffi::c_char),
                    FmtArg::Str(
                        (b"name\0" as *const u8 as *const ::core::ffi::c_char)
                            as *const ::core::ffi::c_char,
                    ),
                    FmtArg::Str(((*d).name) as *const ::core::ffi::c_char),
                ],
            );
        }
        if !(*d).stem.is_null()
            && *(*d).stem.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strcache_iscached((*d).stem) == 0
        {
            error(
                ::core::ptr::null::<Floc>(),
                b"%s: field '%s' not cached: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    FmtArg::Str(((*d).name) as *const ::core::ffi::c_char),
                    FmtArg::Str(
                        (b"stem\0" as *const u8 as *const ::core::ffi::c_char)
                            as *const ::core::ffi::c_char,
                    ),
                    FmtArg::Str(((*d).stem) as *const ::core::ffi::c_char),
                ],
            );
        }
    }
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn verify_file_data_base() {
    hash_map(&raw mut files, Some(verify_file));
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn build_target_list(mut value: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut last_targ_count: ::core::ffi::c_ulong = 0;
    if files.ht_fill != last_targ_count {
        let mut max: size_t = (strlen(value) as size_t)
            .wrapping_div(500)
            .wrapping_add(1)
            .wrapping_mul(500);
        let mut len: size_t;
        let mut p: *mut ::core::ffi::c_char;
        let mut fp: *mut *mut File = files.ht_vec as *mut *mut File;
        let end: *mut *mut File = fp.offset(files.ht_size as isize) as *mut *mut File;
        value = xrealloc(value as *mut ::core::ffi::c_void, max) as *mut ::core::ffi::c_char;
        p = value;
        len = 0;
        while fp < end {
            if !((*fp).is_null()
                || *fp as *mut ::core::ffi::c_void == hash_deleted_item as *mut ::core::ffi::c_void)
                && (**fp).is_target
            {
                let f: *mut File = *fp;
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
                p = p.offset(1 as ::core::ffi::c_int as isize);
                *fresh4 = ' ' as i32 as ::core::ffi::c_char;
            }
            fp = fp.offset(1 as ::core::ffi::c_int as isize);
        }
        *p.offset(-(1 as ::core::ffi::c_int as isize)) = 0;
        last_targ_count = files.ht_fill;
    }
    value
}
/// # Safety
///
/// C-style API operating on raw pointers inherited from the c2rust
/// translation; all pointer arguments must be valid for the call.
pub unsafe fn init_hash_files() {
    hash_init(
        &raw mut files,
        1000 as ::core::ffi::c_ulong,
        Some(file_hash_1),
        Some(file_hash_2),
        Some(file_hash_cmp),
    );
}
pub const FILE_TIMESTAMP_HI_RES: ::core::ffi::c_int = 1;
