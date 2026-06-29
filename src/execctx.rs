//! Process configuration and execution context.
//!
//! This is the owned root of make's runtime state. The c2rust translation kept
//! that state in process-global `static mut`s; we are replacing them with state
//! owned by `main` and threaded explicitly down the call graph by reference, so
//! functions stay pure (no ambient globals) and become safe to run on multiple
//! threads later. Think of `main` as the root of a tree and the context as
//! passed depth-first into every node that needs it — never reached through a
//! global or thread-local.
//!
//! [`Config`] holds values fixed once during startup (read-only thereafter);
//! [`ExecContext`] owns it plus the mutable per-build runtime state. Readers
//! take `&ExecContext` (or `&mut` when they update it); there is no global
//! accessor and no singleton.

/// Immutable process configuration: values fixed once during startup and read
/// for the rest of the run.
#[derive(Debug, Default, Clone)]
pub struct Config {
    /// `$(MAKELEVEL)` — the recursion depth of *this* make process. Parsed once
    /// from the `MAKELEVEL` environment variable during startup (0 at the top
    /// level, N inside a recursive `$(MAKE)`), then immutable.
    pub makelevel: u32,
}

/// The owned execution context, created in `main` and threaded by reference
/// into the call graph. Holds the immutable [`Config`] plus (as the migration
/// proceeds) the mutable runtime state that used to live in `static mut`s.
#[derive(Debug, Default, Clone)]
pub struct ExecContext {
    /// Read-only process configuration.
    pub config: Config,

    /// `f_mtime`'s future-timestamp cache: the most recently sampled "adjusted
    /// now" (`file_timestamp_now` plus the timestamp resolution slack). When a
    /// file's mtime is at or before this value it is known not to be in the
    /// future without re-reading the system clock, so the warning check only
    /// re-samples the clock when a file's mtime is past the cache. Per-run
    /// mutable state — interior mutability keeps readers on `&ExecContext`.
    pub mtime_adjusted_now: ::core::cell::Cell<crate::ffi_types::uintmax_t>,

    /// `f_mtime`'s clock-skew latch: set once when a file's modification time is
    /// found to lie in the future (right after the "in the future" warning is
    /// emitted), and read at the end of `main_0` to print the "Clock skew
    /// detected" notice. Per-run mutable state, set and read on the same
    /// build-phase `&ExecContext` as [`Self::mtime_adjusted_now`]; the former
    /// `static` atomic `CLOCK_SKEW_DETECTED`. Interior mutability keeps readers
    /// on `&ExecContext`.
    pub clock_skew_detected: ::core::cell::Cell<bool>,

    /// `load_too_high`'s per-second job-weighting cache: the wall-clock second
    /// of the previous load sample. When a new second begins, the running-job
    /// estimate folds in the jobs counted during the second just elapsed (see
    /// [`Self::load_prev_weight`]) and resets the per-second counter. Per-run
    /// mutable state — interior mutability keeps readers on `&ExecContext`.
    pub load_sample_second: ::core::cell::Cell<crate::ffi_types::time_t>,

    /// `load_too_high`'s per-second job-weighting cache: the job weight carried
    /// from the immediately preceding second (`LOAD_WEIGHT_B * jobs`), or `0`
    /// when more than one second has elapsed. Added to the current second's
    /// weight to estimate system load between real `getloadavg` samples.
    pub load_prev_weight: ::core::cell::Cell<::core::ffi::c_double>,

    /// `.NOTINTERMEDIATE` (no-argument) latch — set in `snap_deps` when a bare
    /// `.NOTINTERMEDIATE` target is seen, marking every file non-intermediate
    /// for this run. Per-run mutable state; the former `static mut
    /// no_intermediates` global. Interior mutability keeps readers on
    /// `&ExecContext`.
    pub no_intermediates: ::core::cell::Cell<bool>,
    /// `.SECONDARY` (no-argument) latch — set in `snap_deps` when a bare
    /// `.SECONDARY` target is seen, marking every file secondary for this run.
    /// Read alongside [`Self::no_intermediates`]; the former
    /// `file::ALL_SECONDARY` global.
    pub all_secondary: ::core::cell::Cell<bool>,

    /// Resolved `-B`/`--always-make` for this run: `Options::always_make` gated
    /// by the restart count — a restarting sub-make (`restarts != 0`) does not
    /// force-remake on the first pass. Set in `main_0`; the former `static mut
    /// always_make_flag`. Read by `set_file_variables` and `update_file_1` via
    /// the `&ExecContext` they already carry.
    pub always_make_flag: ::core::cell::Cell<bool>,

    /// Pattern-rule database statistics, recomputed by `snap_implicit_rules`
    /// after the makefiles are read and consumed by `pattern_search` to size its
    /// scratch allocations — the former `static` atomics `NUM_PATTERN_RULES` /
    /// `MAX_PATTERN_TARGETS` / `MAX_PATTERN_DEPS` / `MAX_PATTERN_DEP_LENGTH`.
    /// `num_pattern_rules` counts the pattern rules; `max_pattern_targets` is the
    /// most targets any one rule has; `max_pattern_deps` the most prerequisites
    /// (also bumped by `pattern_search` itself when a rule expands to more deps
    /// than any seen before); `max_pattern_dep_length` the longest prerequisite
    /// name. Producers and consumers carry the same `&ExecContext`, so interior
    /// mutability keeps readers on `&ExecContext`.
    pub num_pattern_rules: ::core::cell::Cell<::core::ffi::c_uint>,
    pub max_pattern_targets: ::core::cell::Cell<::core::ffi::c_uint>,
    pub max_pattern_deps: ::core::cell::Cell<::core::ffi::c_uint>,
    pub max_pattern_dep_length: ::core::cell::Cell<crate::ffi_types::size_t>,

    /// The goal-chain walk's per-pass tracking counters, the former `static`
    /// atomics `COMMANDS_STARTED` / `CONSIDERED`. `commands_started` counts
    /// recipes launched so far — bumped by `start_job_command` and the
    /// `notice_finished_file` touch path — and `update_goal_chain` snapshots it
    /// around each goal to tell whether a pass made progress. `considered` is a
    /// generation marker bumped once per `update_goal_chain` pass; each file
    /// records the generation it was last considered in (`update_file`) so it is
    /// not walked twice in one pass. Producers and consumers carry the same
    /// `&ExecContext`, so interior mutability keeps readers on `&ExecContext`.
    pub commands_started: ::core::cell::Cell<::core::ffi::c_uint>,
    pub considered: ::core::cell::Cell<::core::ffi::c_uint>,

    /// Whether the terminal's "good" stdin is currently held by a running job,
    /// the former `static GOOD_STDIN_USED` atomic. Only one local job at a time
    /// may inherit the real stdin; `start_job_command` sets this when it hands a
    /// job the good stdin and clears it when a remote hand-off declines stdin,
    /// and `reap_children` clears it when that job is reaped. Lives on
    /// `ExecContext` (not `Options`): it is per-run job-execution state read and
    /// written only on the build walk (`reap_children` / `start_job_command`),
    /// never on the `gmk_eval` throwaway-context path, and every site already
    /// carries the same `&ExecContext`.
    pub good_stdin_used: ::core::cell::Cell<bool>,

    /// Count of `DIR*` streams the directory cache currently holds open, the
    /// former file-scoped `static mut dir::open_directories`. `find_directory`
    /// bumps it when it opens a new stream and, on reaching
    /// `MAX_OPEN_DIRECTORIES`, drains that directory immediately to bound open
    /// file descriptors; `dir_contents_file_exists_p` and
    /// `clear_directory_contents` decrement it when a stream is exhausted or
    /// discarded. It only governs *when* a directory is read to completion
    /// (eagerly vs lazily), never which files are found, so threading it on the
    /// per-run `&ExecContext` is behavior-preserving. The glob `open_dirstream`
    /// callback cannot carry the build context (C-ABI) and already runs against
    /// a throwaway default context, as it does for the `make[N]:` prefix.
    pub open_directories: ::core::cell::Cell<u32>,

    /// `load_too_high`'s lazily-probed `/proc/loadavg` descriptor cache, the
    /// former function-local `static mut proc_fd`. Only touched on the
    /// build-phase context (job scheduling), so the startup context
    /// re-initialization never discards an open fd.
    pub load_proc_fd: LoadProcFd,
    /// `load_too_high`'s last-reported `getloadavg` failure errno (the former
    /// function-local `static mut lossage`), used to suppress repeating the
    /// same "cannot enforce load limit" warning.
    pub load_lossage: LoadLossage,

    /// The directory cache's name-keyed table (`struct directory` entries), the
    /// former file-scoped `static mut dir::directories`. Owned per-run so there
    /// is no process-global hash table; `find_directory` and
    /// `print_dir_data_base` reach it through the `&ExecContext` they carry. The
    /// glob `open_dirstream` callback (C-ABI, no `ctx` parameter) reaches it
    /// through the `CTX_PTR` borrow channel, mirroring `with_options`. Populated
    /// during makefile parsing (`$(wildcard)`, vpath, includes) and reused
    /// through the build, so `main_0` hands it across the build-phase context
    /// rebuild rather than letting the cache reset (see its use site).
    /// Idiomatic Rust [`rustc_hash::FxHashMap`] keyed by the directory name
    /// bytes, replacing the c2rust FFI `hash_table` and its `directory_hash_*`
    /// callbacks.
    pub directories: DirNameTable,
    /// The directory cache's dev/inode-keyed contents table (`struct
    /// directory_contents`), the former file-scoped `static mut
    /// dir::directory_contents`. Shares [`Self::directories`]' lifetime and
    /// hand-off. Idiomatic Rust [`rustc_hash::FxHashMap`] keyed by `(dev, ino)`,
    /// replacing the c2rust FFI `hash_table` and its hash/compare callbacks.
    pub directory_contents: DirContentsTable,

    /// make's central file table (`struct file` records keyed by hash-name), the
    /// former file-scoped `static mut file::files`. Owned per-run so there is no
    /// process-global hash table; `lookup_file`/`enter_file`/`rehash_file` and the
    /// `make -p` / target-list / intermediate-cleanup walks reach it through the
    /// `&ExecContext` they carry. Populated during makefile parsing and reused
    /// through the build, so `main_0` hands it across the build-phase context
    /// rebuild rather than letting it reset (see its use site). Idiomatic Rust
    /// [`rustc_hash::FxHashMap`] keyed by the file's hash-name bytes, replacing
    /// the c2rust FFI `hash_table` and its `file_hash_*` callbacks.
    pub files: FileTable,

    /// `read_dirstream`'s reused dirent scratch buffer — the heap block (and its
    /// size) that the glob `gl_readdir` callback rewrites for each enumerated
    /// file and returns a pointer into. These were the function-local process
    /// globals `static mut buf`/`bufsz`; like the directory cache they now live
    /// on the owned per-run context. The C-ABI callback can't carry an
    /// `&ExecContext`, so it reaches these through the same `CTX_PTR` borrow
    /// channel as `open_dirstream`. The buffer only grows (never shrinks); its
    /// contents are scratch (overwritten every call), but `main_0` hands it
    /// across the build-phase context rebuild alongside the directory cache so a
    /// single block serves the whole run, exactly as the former static did.
    pub read_dirstream_buf: ::core::cell::Cell<*mut ::core::ffi::c_char>,
    pub read_dirstream_bufsz: ::core::cell::Cell<crate::ffi_types::size_t>,
}

/// The directory cache's name-keyed table: an idiomatic Rust
/// [`rustc_hash::FxHashMap`] from the directory name bytes (the interned name,
/// less its NUL) to an owned, heap-stable [`directory`](crate::dir::directory),
/// replacing the c2rust FFI `hash_table` (and its `directory_hash_*` callbacks).
///
/// Entries are `Box`ed so a `*mut directory` returned by `find_directory` stays
/// valid across later inserts/rehashes — the map may move the `Box`, never the
/// heap block it owns. `RefCell` gives the interior mutability the former
/// `Cell<hash_table>` provided on the shared `&ExecContext`.
pub struct DirNameTable(
    pub  ::core::cell::RefCell<
        rustc_hash::FxHashMap<Box<[u8]>, Box<crate::dir::directory>>,
    >,
);

impl Default for DirNameTable {
    fn default() -> Self {
        Self(::core::cell::RefCell::new(rustc_hash::FxHashMap::default()))
    }
}

impl Clone for DirNameTable {
    fn clone(&self) -> Self {
        // Per-run build state handed across the build-phase rebuild by move
        // (`mem::take`), never by clone; the `Clone` impl exists only to keep
        // `ExecContext`'s derive working, and the entries hold raw
        // back-pointers that must not be duplicated, so a fresh empty table is
        // the right (and only sound) snapshot.
        Self::default()
    }
}

impl ::core::fmt::Debug for DirNameTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DirNameTable")
            .field(&self.0.borrow().len())
            .finish()
    }
}

/// The directory cache's dev/inode-keyed contents table: an idiomatic Rust
/// [`rustc_hash::FxHashMap`] from `(dev, ino)` to an owned, heap-stable
/// [`directory_contents`](crate::dir::directory_contents), replacing the
/// c2rust FFI `hash_table` (and its `directory_contents_hash_*` callbacks).
///
/// Entries are `Box`ed so a `*mut directory_contents` taken for a
/// [`directory`](crate::dir::directory)'s `contents` (and for the glob
/// dirstream) stays valid across later inserts/rehashes — the map may move the
/// `Box`, never the heap block it owns. `RefCell` gives the interior mutability
/// the former `Cell<hash_table>` provided on the shared `&ExecContext`.
pub struct DirContentsTable(
    pub  ::core::cell::RefCell<
        rustc_hash::FxHashMap<
            (crate::ffi_types::dev_t, crate::ffi_types::ino_t),
            Box<crate::dir::directory_contents>,
        >,
    >,
);

impl Default for DirContentsTable {
    fn default() -> Self {
        Self(::core::cell::RefCell::new(rustc_hash::FxHashMap::default()))
    }
}

impl Clone for DirContentsTable {
    fn clone(&self) -> Self {
        // The directory cache is per-run build state handed across the
        // build-phase rebuild by move (`mem::take`), never by clone; the
        // `Clone` impl exists only to keep `ExecContext`'s derive working, so a
        // fresh empty table is the right (and only sound) snapshot — the entries
        // hold raw back-pointers that must not be duplicated.
        Self::default()
    }
}

impl ::core::fmt::Debug for DirContentsTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DirContentsTable")
            .field(&self.0.borrow().len())
            .finish()
    }
}

/// make's central file table: an idiomatic Rust [`rustc_hash::FxHashMap`] from a
/// file's hash-name bytes (the interned `hname`, less its NUL) to its
/// heap-stable `*mut file` record, replacing the c2rust FFI `hash_table` (and its
/// `file_hash_*` callbacks).
///
/// Unlike the directory cache, the value is a raw `*mut file`, not an owned
/// `Box`: `file` records are interned for the whole run and referenced by raw
/// pointer all over the codebase (deps, goals, the build graph), so the table
/// only *indexes* them — it does not own them. `RefCell` gives the interior
/// mutability the former `Cell<hash_table>` provided on the shared
/// `&ExecContext`.
pub struct FileTable(
    pub  ::core::cell::RefCell<rustc_hash::FxHashMap<Box<[u8]>, *mut crate::file::File>>,
);

impl Default for FileTable {
    fn default() -> Self {
        Self(::core::cell::RefCell::new(rustc_hash::FxHashMap::default()))
    }
}

impl Clone for FileTable {
    fn clone(&self) -> Self {
        // Per-run build state handed across the build-phase rebuild by move
        // (`mem::take`), never by clone; the `Clone` impl exists only to keep
        // `ExecContext`'s derive working, and the entries are raw `*mut file`
        // back-references that must not be duplicated, so a fresh empty table is
        // the right (and only sound) snapshot.
        Self::default()
    }
}

impl ::core::fmt::Debug for FileTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileTable")
            .field(&self.0.borrow().len())
            .finish()
    }
}

/// Wrapper giving [`ExecContext::load_proc_fd`] its `-2` ("not yet probed")
/// initial value while `ExecContext` keeps deriving `Default`.
#[derive(Debug, Clone)]
pub struct LoadProcFd(pub ::core::cell::Cell<i32>);

impl Default for LoadProcFd {
    fn default() -> Self {
        Self(::core::cell::Cell::new(-2))
    }
}

/// Wrapper giving [`ExecContext::load_lossage`] its `-1` ("no failure reported
/// yet") initial value while `ExecContext` keeps deriving `Default`.
#[derive(Debug, Clone)]
pub struct LoadLossage(pub ::core::cell::Cell<i32>);

impl Default for LoadLossage {
    fn default() -> Self {
        Self(::core::cell::Cell::new(-1))
    }
}

impl ExecContext {
    /// Build a context over the given immutable [`Config`]. Mutable per-run
    /// caches start at their zero defaults.
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Self::default()
        }
    }

    /// `$(MAKELEVEL)` for this make process.
    pub fn makelevel(&self) -> u32 {
        self.config.makelevel
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ExecContext};

    #[test]
    fn context_exposes_makelevel() {
        let ctx = ExecContext::new(Config { makelevel: 3 });
        assert_eq!(ctx.makelevel(), 3);
        // Cloning yields an independent copy of the owned state.
        assert_eq!(ctx.clone().makelevel(), 3);
    }

    #[test]
    fn default_makelevel_is_zero() {
        assert_eq!(ExecContext::default().makelevel(), 0);
    }

    #[test]
    fn load_sample_cache_starts_zeroed() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert_eq!(ctx.load_sample_second.get(), 0);
        assert_eq!(ctx.load_prev_weight.get(), 0.0);
        // `..Self::default()` in `new` must not skip the cache fields.
        assert_eq!(ExecContext::default().load_sample_second.get(), 0);
    }

    /// `load_too_high`'s probe caches start at their non-zero sentinels
    /// (`proc_fd = -2` "not yet probed", `lossage = -1` "no failure reported"),
    /// matching the former function-local `static mut` initializers, and are
    /// per-run mutable state. The custom `Default` impls must survive
    /// `..Self::default()` in `new`.
    #[test]
    fn load_probe_caches_start_at_sentinels() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert_eq!(ctx.load_proc_fd.0.get(), -2);
        assert_eq!(ctx.load_lossage.0.get(), -1);
        assert_eq!(ExecContext::default().load_proc_fd.0.get(), -2);
        assert_eq!(ExecContext::default().load_lossage.0.get(), -1);

        // Per-run mutation is observable through the shared `&ExecContext`.
        ctx.load_proc_fd.0.set(7);
        ctx.load_lossage.0.set(0);
        assert_eq!(ctx.load_proc_fd.0.get(), 7);
        assert_eq!(ctx.load_lossage.0.get(), 0);
        // A fresh context is back at the sentinels (per-run, not global).
        assert_eq!(ExecContext::default().load_proc_fd.0.get(), -2);
    }

    /// The directory cache's two `FxHashMap`s start empty, and the carry-over in
    /// `main_0` (a struct update that moves these two fields onto a freshly built
    /// context) preserves the populated table while resetting the rest.
    #[test]
    fn dir_cache_tables_start_zeroed_and_survive_carry_over() {
        let ctx = ExecContext::default();
        assert!(ctx.directories.0.borrow().is_empty());
        assert!(ctx.directory_contents.0.borrow().is_empty());

        // Simulate a populated table, then the `main_0` build-phase rebuild that
        // hands the cache across.
        let populated = ExecContext::default();
        {
            // SAFETY: a zeroed `directory` is a valid (inert) entry; the test
            // only checks the map's size, never dereferences the name pointer.
            let dir: Box<crate::dir::directory> = Box::new(unsafe { ::core::mem::zeroed() });
            populated
                .directories
                .0
                .borrow_mut()
                .insert(Box::from(&b"d"[..]), dir);
        }
        assert_eq!(populated.directories.0.borrow().len(), 1);

        let mut populated = populated;
        let carried = ::core::mem::take(&mut populated.directories);
        let rebuilt = ExecContext {
            directories: carried,
            ..ExecContext::new(Config { makelevel: 1 })
        };
        // The carried table survived; the source field reset to empty.
        assert_eq!(rebuilt.directories.0.borrow().len(), 1);
        assert!(populated.directories.0.borrow().is_empty());
        // Unrelated per-run state on the rebuilt context is fresh.
        assert_eq!(rebuilt.makelevel(), 1);
    }

    /// `read_dirstream`'s reused dirent scratch buffer (the former process-global
    /// `static mut buf`/`bufsz`) starts empty, is per-run, and is handed across
    /// the `main_0` build-phase context rebuild the same way the directory cache
    /// is — a single heap block must serve the whole run, as the former static
    /// did.
    #[test]
    fn read_dirstream_buffer_starts_empty_and_survives_carry_over() {
        let ctx = ExecContext::default();
        assert!(ctx.read_dirstream_buf.get().is_null());
        assert_eq!(ctx.read_dirstream_bufsz.get(), 0);

        // Simulate a buffer allocated during makefile parsing (non-null pointer
        // + size marks it live), then the `main_0` rebuild that hands it across.
        let mut populated = ExecContext::default();
        populated
            .read_dirstream_buf
            .set(0x1 as *mut ::core::ffi::c_char);
        populated.read_dirstream_bufsz.set(128);

        let carried_buf = ::core::mem::take(&mut populated.read_dirstream_buf);
        let carried_bufsz = ::core::mem::take(&mut populated.read_dirstream_bufsz);
        let rebuilt = ExecContext {
            read_dirstream_buf: carried_buf,
            read_dirstream_bufsz: carried_bufsz,
            ..ExecContext::new(Config { makelevel: 1 })
        };
        // The carried buffer survived; the source fields reset to empty.
        assert_eq!(
            rebuilt.read_dirstream_buf.get(),
            0x1 as *mut ::core::ffi::c_char
        );
        assert_eq!(rebuilt.read_dirstream_bufsz.get(), 128);
        assert!(populated.read_dirstream_buf.get().is_null());
        assert_eq!(populated.read_dirstream_bufsz.get(), 0);
        assert_eq!(rebuilt.makelevel(), 1);

        // Per-run: a fresh context does not inherit the buffer.
        assert!(ExecContext::default().read_dirstream_buf.get().is_null());
        assert_eq!(ExecContext::default().read_dirstream_bufsz.get(), 0);
    }

    /// The `.NOTINTERMEDIATE`/`.SECONDARY` latches start unset and are per-run
    /// (a fresh context, e.g. a new make invocation, sees them `false` again),
    /// replacing the former process-global `no_intermediates`/`ALL_SECONDARY`.
    #[test]
    fn intermediate_latches_start_unset_and_are_per_run() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert!(!ctx.no_intermediates.get());
        assert!(!ctx.all_secondary.get());

        ctx.no_intermediates.set(true);
        ctx.all_secondary.set(true);
        assert!(ctx.no_intermediates.get());
        assert!(ctx.all_secondary.get());

        // A fresh context does not inherit the latch (no cross-run leakage).
        assert!(!ExecContext::default().no_intermediates.get());
        assert!(!ExecContext::default().all_secondary.get());
    }

    /// `always_make_flag` (resolved `-B`/`--always-make`) is per-run and starts
    /// unset, replacing the former process-global `always_make_flag`.
    #[test]
    fn always_make_flag_starts_unset_and_is_per_run() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert!(!ctx.always_make_flag.get());

        ctx.always_make_flag.set(true);
        assert!(ctx.always_make_flag.get());

        // A fresh context does not inherit it (no cross-run leakage).
        assert!(!ExecContext::default().always_make_flag.get());
    }

    /// The pattern-rule database statistics (`num_pattern_rules` etc., the former
    /// `static` atomics) start at 0, are per-run, and track the running maxima
    /// `snap_implicit_rules`/`pattern_search` compute.
    #[test]
    fn pattern_rule_stats_start_zero_and_track_maxima() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert_eq!(ctx.num_pattern_rules.get(), 0);
        assert_eq!(ctx.max_pattern_targets.get(), 0);
        assert_eq!(ctx.max_pattern_deps.get(), 0);
        assert_eq!(ctx.max_pattern_dep_length.get(), 0);

        // The running-max idiom `snap_implicit_rules` / `pattern_search` use.
        for len in [3usize, 9, 4, 9, 7] {
            if len > ctx.max_pattern_dep_length.get() {
                ctx.max_pattern_dep_length.set(len);
            }
        }
        assert_eq!(ctx.max_pattern_dep_length.get(), 9);
        ctx.num_pattern_rules
            .set(ctx.num_pattern_rules.get().wrapping_add(1));
        assert_eq!(ctx.num_pattern_rules.get(), 1);

        // Per-run: a fresh context does not inherit the computed stats.
        assert_eq!(ExecContext::default().max_pattern_dep_length.get(), 0);
        assert_eq!(ExecContext::default().num_pattern_rules.get(), 0);
    }

    /// `clock_skew_detected` starts false, latches true the way `f_mtime` sets
    /// it on a future-dated file, and is per-run, replacing the former
    /// process-global `CLOCK_SKEW_DETECTED`.
    #[test]
    fn clock_skew_detected_starts_unset_and_is_per_run() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert!(!ctx.clock_skew_detected.get(), "no skew yet");

        ctx.clock_skew_detected.set(true);
        assert!(ctx.clock_skew_detected.get(), "skew detected");

        // A fresh context does not inherit the latch (no cross-run leakage).
        assert!(!ExecContext::default().clock_skew_detected.get());
    }

    /// The goal-chain pass counters (`commands_started` / `considered`, the
    /// former `static` atomics) start at 0, bump monotonically, and are per-run.
    #[test]
    fn goal_chain_counters_start_zero_and_bump() {
        let ctx = ExecContext::new(Config { makelevel: 0 });
        assert_eq!(ctx.commands_started.get(), 0);
        assert_eq!(ctx.considered.get(), 0);

        // The bump idiom `update_goal_chain` / `start_job_command` use.
        ctx.commands_started.set(ctx.commands_started.get().wrapping_add(1));
        ctx.considered.set(ctx.considered.get().wrapping_add(1));
        assert_eq!(ctx.commands_started.get(), 1);
        assert_eq!(ctx.considered.get(), 1);

        // Per-run: a fresh context does not inherit the counts.
        assert_eq!(ExecContext::default().commands_started.get(), 0);
        assert_eq!(ExecContext::default().considered.get(), 0);
    }
}
