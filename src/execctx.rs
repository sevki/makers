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

/// Which shell "personality" governs recipe-line quoting/escaping. The C
/// original tracks this as two independent int flags (`unixy_shell`,
/// `batch_mode_shell`) whose only meaningful combinations are "unixy",
/// "W32/DOS batch", or neither (the fatal "!unixy && !batch_mode_shell" case
/// `construct_command_argv_internal` guards against) — never both at once.
/// Modeling the pair as one enum makes that impossible fourth combination
/// unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShellKind {
    /// A POSIX-style ("unixy") shell. The only outcome this POSIX port ever
    /// produces.
    #[default]
    Unixy,
    /// W32/DOS batch-mode shell (COMMAND.COM/CMD.EXE). Unreachable in this
    /// POSIX-only port; kept so the type carries the same state space as the
    /// C original for a future non-POSIX target.
    Batch,
    /// Neither unixy nor batch shell context — the C original's fatal
    /// "Bad shell context" case.
    Other,
}

/// Immutable process configuration: values fixed once during startup and read
/// for the rest of the run.
#[derive(Debug, Clone)]
pub struct Config {
    /// `$(MAKELEVEL)` — the recursion depth of *this* make process. Parsed once
    /// from the `MAKELEVEL` environment variable during startup (0 at the top
    /// level, N inside a recursive `$(MAKE)`), then immutable.
    pub makelevel: u32,

    /// Which shell personality is in effect. The C original resolves this at
    /// startup from the detected shell; this POSIX-only port always resolves
    /// to [`ShellKind::Unixy`], but it lives as owned `Config` state rather
    /// than a fixed `const` so a future non-POSIX target can set it per
    /// session instead of reintroducing global state.
    pub shell_kind: ShellKind,

    /// Default system include directories searched by `-I` when not disabled
    /// (`-I-`). Fixed at compile time in the C original via autoconf
    /// substitution (`INCLUDEDIR` et al.); this port hardcodes the same
    /// defaults, but keeps them as owned `Config` state rather than a bare
    /// module-level `const` so nothing resembling build configuration lives
    /// outside `ExecContext`.
    pub default_include_directories: [&'static [u8]; 3],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            makelevel: 0,
            shell_kind: ShellKind::default(),
            default_include_directories: [
                b"/usr/gnu/include",
                b"/usr/local/include",
                b"/usr/include",
            ],
        }
    }
}

/// The owned execution context, created in `main` and threaded by reference
/// into the call graph. Holds the immutable [`Config`] plus (as the migration
/// proceeds) the mutable runtime state that used to live in `static mut`s.
#[derive(Debug, Default, Clone)]
pub struct ExecContext {
    /// Read-only process configuration.
    pub config: Config,

    /// The session salsa database ([`crate::makedb::MakeDb`]) — hosts the
    /// string interner, the parser's interned AST nodes, and dependency-graph
    /// inputs in one database with shared revisions. Owned here (not in
    /// `OnceLock` statics) so two sessions in one process never share
    /// interner state. Interning and input creation need only `&MakeDb`, so
    /// no interior-mutability wrapper is required.
    pub db: crate::makedb::MakeDb,

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

    /// The pattern-rule database itself — owned rules in definition order, the
    /// former `rule::PATTERN_RULES` `thread_local!`. Populated after the
    /// build-phase context rebuild in `main_0` (makefile reading, suffix-rule
    /// conversion, and builtin-rule installation all run later), so it needs no
    /// carry-over there. Accessed through `rule::with_pattern_rules{,_mut}`,
    /// which now borrow from the caller's `&ExecContext` — a server hosting
    /// concurrent sessions gets one rule DB per session context instead of one
    /// per thread.
    pub rules: ::core::cell::RefCell<Vec<crate::rule::Rule>>,

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

    /// Path of the temporary file holding piped-in stdin makefile text, the
    /// former main.rs `static mut temp_stdin_name`. Written during argument
    /// decoding (`--temp-stdin`, or when `-f -` spools stdin to a temp file)
    /// and read by `temp_stdin_unlink` on the `die`/fatal-signal paths, which
    /// carry only `&ExecContext`. Carried across the `main_0` build-phase
    /// context rebuild: the name is recorded before the rebuild and must
    /// survive until cleanup. Interned or 'static storage backs the pointer.
    pub temp_stdin_name: PtrCell,

    /// The working directory before any `-C` chdir, the former main.rs
    /// `pub static mut directory_before_chdir` — restored on re-exec and in
    /// `die`, both of which carry only `&ExecContext`. Set once during
    /// startup (before the build-phase context rebuild), so it is carried
    /// across it. Heap storage from `xstrdup` backs the pointer.
    pub directory_before_chdir: MutPtrCell,

    /// The program name — `basename(argv[0])`, or `"make"` when argv[0] is
    /// empty — the former main.rs `pub static mut program`. Prefixes every
    /// message/error line and the usage text. Set once at startup (before the
    /// build-phase context rebuild, so it is carried across it); argv or
    /// 'static storage backs the pointer.
    pub program: PtrCell,

    /// The temporary-file directory (`$MAKE_TMPDIR`, `$TMPDIR`, or the
    /// default), the former `misc.rs` function-local `static mut tmpdir` in
    /// `get_tmpdir`. Computed once at startup (before the build-phase context
    /// rebuild) and cached; carried across the rebuild so later temp-file
    /// users don't re-probe the environment and re-warn about an invalid
    /// value. Interned, `'static`, or `xstrdup`'d storage backs the pointer
    /// (never freed, matching the former static's lifetime).
    pub tmpdir: PtrCell,

    /// `--shuffle` mode/seed and the PRNG state it drives (see
    /// [`crate::shuffle::ShuffleState`]) — the former shuffle.rs `static
    /// CONFIG` and misc.rs `static MK_STATE`. Configured once from the
    /// command line before the build-phase context rebuild (`main_0`'s
    /// `decode_switches`/`set_mode` call), so it is carried across it like
    /// `program`/`tmpdir`; every later shuffle call reads and re-seeds it.
    pub shuffle: ::core::cell::Cell<crate::shuffle::ShuffleState>,

    /// The remote-execution backend (the former `remote_stub.rs` `static
    /// REMOTE: StubRemote` singleton). Owned per-context instead of a process
    /// global so a future multi-tenant host can run different backends (or
    /// differently configured ones) per session. Defaults to the
    /// never-remote [`crate::remote_stub::StubRemote`]; cheap to clone (an
    /// `Arc` bump) since [`ExecContext`] itself derives `Clone`.
    pub remote_backend: RemoteBackendCell,

    /// Make's recorded starting working directory (after any `-C` chdirs), the
    /// former main.rs `pub static mut starting_directory` — read by the
    /// `Entering/Leaving directory` lines and `$(abspath)`. Null when `getcwd`
    /// failed. Set after the build-phase context rebuild, into `main_0`'s
    /// `current_directory` buffer, which outlives the run.
    pub starting_directory: MutPtrCell,

    /// `load_too_high`'s last-reported `getloadavg` failure errno (the former
    /// function-local `static mut lossage`), used to suppress repeating the
    /// same "cannot enforce load limit" warning.
    pub load_lossage: LoadLossage,

    /// The `SHELL` variable as it came from the environment, the former main.rs
    /// `pub static mut shell_var` — recorded (name/length/value, via `xstrdup`)
    /// when the startup environment scan meets `SHELL`, and appended by
    /// `target_environment` to every child's environment when the walk did not
    /// already export a `SHELL`. Unset (null name/value) when the environment
    /// had no `SHELL`. Written during startup, before the `main_0` build-phase
    /// context rebuild, and read during the build, so it is carried across the
    /// rebuild; heap storage from `xstrdup` backs the pointers.
    pub shell_var: ShellVar,

    /// Head of the command-line variable-definition list (`V=x` arguments), the
    /// former file-scoped main.rs `static mut command_variables` — pushed by
    /// `handle_non_switch_argument` as switches are decoded and walked to build
    /// `-*-command-variables-*-`/`MAKEOVERRIDES`. Switch decoding runs both
    /// before the `main_0` build-phase context rebuild (argv, `MAKEFLAGS`) and
    /// after it (re-reading `MAKEFLAGS` once makefiles are parsed), so the list
    /// is carried across the rebuild. `xmalloc`ed nodes back the pointers.
    pub command_variables: CommandVariables,

    /// The `.DEFAULT_GOAL` variable record, the former main.rs `pub static mut
    /// default_goal_var` — defined once in `main_0` (after the build-phase
    /// context rebuild, so no carry-over) and consulted by `record_files` while
    /// parsing (only targets seen while it is still empty become the default
    /// goal) and by `main_0`'s goal selection. Points into the global variable
    /// set, whose records are updated in place, so the pointer stays valid
    /// across later `.DEFAULT_GOAL` assignments.
    pub default_goal_var: DefaultGoalVar,

    /// The set of fatal signals make traps, the former main.rs `pub static mut
    /// fatal_signal_set` — built once during startup (`sigemptyset`, then one
    /// `sigaddset` per signal `install_fatal_signal` traps) and passed to
    /// `sigprocmask` by job.rs's `block_sigs`/`unblock_sigs` to keep child
    /// bookkeeping atomic against the fatal-signal handler. Written before the
    /// `main_0` build-phase context rebuild and read during the build, so it is
    /// carried across the rebuild. `SigsetT` is a plain `Copy` record; sites
    /// copy it out of the `Cell`, hand the local to the libc call, and (for
    /// the setup writes) store it back.
    pub fatal_signal_set: FatalSignalSet,

    /// The run's own output-sync context (the former main.rs `static mut
    /// make_sync`): the `output` record that captures make's *own* messages
    /// into temp files while `-O` output sync is active. Boxed so its heap
    /// address is stable — [`Self::output_context`] captures the address
    /// before the `main_0` build-phase context rebuild and `die` compares
    /// against it after, so `main_0` carries the same allocation across the
    /// rebuild rather than letting a fresh default reset it.
    pub make_sync: MakeSync,

    /// The active output-sync target (the former output.rs `static mut
    /// output_context`): null when writes go straight to stdio, otherwise
    /// pointing at [`Self::make_sync`] or at a running child's `output`
    /// record. Readers and writers reach it through
    /// [`crate::output::output_context`]/[`crate::output::set_output_context`],
    /// which resolve the *live* context over the `CTX_PTR` borrow channel —
    /// the printers can be handed a throwaway `ExecContext` (plugin ABI,
    /// signal handler) and must still see the real run's sync state.
    /// Written before the `main_0` rebuild and read after, so it is carried
    /// across alongside [`Self::make_sync`].
    pub output_context: OutputContext,

    /// `pid2str`'s scratch buffer (the former `static mut pidstring`), read
    /// back by the caller through the returned pointer before the next
    /// `pid2str` call overwrites it — exactly the former static's contract,
    /// now with a stable address on the owned context instead of process
    /// memory. `Cell::as_ptr` hands out that address directly.
    pub pid_string: PidString,

    /// Head of the live-children chain (`struct child` list), the former
    /// job.rs `static mut children`. Every launched recipe child is pushed
    /// here and popped by `reap_children`; the fatal-signal handler reaches
    /// the chain through the `CTX_PTR` borrow channel (it cannot take an
    /// `&ExecContext`), so killing and partially-built-target cleanup walk
    /// the *live* run's children rather than a throwaway context's (#468).
    /// Jobs only start after the `main_0` build-phase rebuild, so the chain
    /// is empty at rebuild time and needs no carry.
    pub children: ChildChain,
    /// Head of the load-limited postponed-jobs chain, the former job.rs
    /// `static mut waiting_jobs`. Shares [`Self::children`]'s lifecycle.
    pub waiting_jobs: ChildChain,

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

    /// make's central file store: a [`FileId`](crate::file::FileId)-keyed map of
    /// `Arc<Mutex<FileNode>>`, where build-graph nodes are shared by `Copy`
    /// handle instead of raw `*mut file`. This is the SOLE file store (the former
    /// raw-pointer `FileTable`/`ctx.files` has been removed):
    /// `lookup_file`/`enter_file`/`rehash_file`/`rename_file` and the
    /// `make -p` / target-list / intermediate-cleanup walks reach it through the
    /// `&ExecContext` they carry. Populated during makefile parsing and reused
    /// through the build, so `main_0` hands it across the build-phase context
    /// rebuild rather than letting it reset.
    pub filenodes: FileArena,

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

    /// `parse_file_seq`'s reused unquoting scratch buffer (the former
    /// function-local `static mut tmpbuf`/`tmpbuf_len`) — grown to fit the
    /// longest name seen so far and reused on every call, exactly as the
    /// former statics were. A plain owned `Vec<u8>`, not an FFI pointer/size
    /// pair: `parse_file_seq` borrows it mutably just long enough to grow it
    /// and take `as_mut_ptr()`, casting to `*mut c_char` only at that single
    /// FFI boundary (the same treatment `PidString` got in #475). Called from
    /// both the parse phase and the build phase (`function.rs`, `implicit.rs`),
    /// so `main_0` carries it across the context rebuild like
    /// [`Self::read_dirstream_buf`].
    pub file_seq_tmpbuf: ::core::cell::RefCell<Vec<u8>>,

    /// Goals accumulated while reading makefiles (the former `static mut
    /// read_files` linked-list replacement) — the pointer-free `Vec` that
    /// `read_all_makefiles`/`eval_makefile`/`eval` push onto as each
    /// (possibly nested, via `include`) makefile is read. Fully drained by
    /// `read_all_makefiles` (via `RefCell::take`, mirroring the former
    /// `mem::take(&mut read_files)`) before it returns, so unlike
    /// [`Self::file_seq_tmpbuf`] this never needs to survive the `main_0`
    /// context rebuild — it starts and ends each call empty.
    pub read_files: ::core::cell::RefCell<Vec<crate::dep::GoalDepNode>>,

    /// The active `ifdef`/`ifeq`/`else`/`endif` nesting stack (the former
    /// module-scope `static mut toplevel_conditionals`/`conditionals` pair) —
    /// one [`ConditionalsFrame`] per makefile-reading scope. `eval_buffer` and
    /// `eval`'s `include` handling install a fresh empty frame for the nested
    /// scope and restore the enclosing one afterward (`RefCell::replace`
    /// mirrors the former `install_conditionals`/`restore_conditionals`
    /// pointer swap; dropping the old frame is the former manual `free`).
    /// Never needs to survive the `main_0` rebuild: every makefile read
    /// balances its own `if`/`endif` nesting before `eval` returns.
    pub conditionals: ::core::cell::RefCell<ConditionalsFrame>,

    /// The jobserver token pipe/fifo's `[read end, write end]` fds, the
    /// former posixos.rs `static mut job_fds`, defaulting to `[-1, -1]` (no
    /// fds open) like the original static's initializer. `jobserver_clear`
    /// runs synchronously from `fatal_error_signal` (a real signal handler),
    /// so — like [`Self::children`] — this stays a `Cell`, not a `RefCell`:
    /// a signal that interrupts a held `RefCell` borrow would panic
    /// mid-handler. Reached through the `CTX_PTR` borrow channel from that
    /// path, and through the ordinary `&ExecContext` parameter everywhere
    /// else.
    pub job_fds: JobFds,

    /// The fifo jobserver's path, the former posixos.rs `static mut
    /// fifo_name`. `xmalloc`/`xstrdup`'d and `free`'d exactly as the
    /// original static was — signal-reachable (`jobserver_clear` runs from
    /// `fatal_error_signal`), so like [`Self::temp_stdin_name`] it stays a
    /// raw-pointer `Cell` rather than an owned `RefCell<Vec<u8>>`, for the
    /// same panic-in-signal-handler reason as [`Self::job_fds`].
    pub fifo_name: MutPtrCell,

    /// The output-sync lock file's path, the former posixos.rs `static mut
    /// osync_tmpfile`. Same signal-reachability rationale as
    /// [`Self::fifo_name`] (`osync_clear` also runs from
    /// `fatal_error_signal`).
    pub osync_tmpfile: MutPtrCell,

    /// `library_search`'s `-l` directory-search cache — the former
    /// `remake.rs` function-local `static mut buf`/`buflen`/
    /// `libdir_maxlen`/`std_dirs`. Populated on first use per run and reused
    /// across every `-lfoo` prerequisite lookup within it (`libdir_maxlen`/
    /// `std_dirs` never change after the first pattern; `buf`/`buflen` only
    /// grow to fit the longest library basename seen so far). `f_mtime`
    /// (`library_search`'s sole caller) only runs during the build phase,
    /// so — like [`Self::read_files`] — this never needs to survive the
    /// `main_0` build-phase context rebuild.
    pub library_search_cache: ::core::cell::RefCell<LibrarySearchCache>,

    /// Count of job slots currently in use, the former job.rs `static
    /// JOB_SLOTS_USED` atomic. `reap_children` (which decrements this) runs
    /// from `fatal_error_signal` (a real signal handler) as well as the
    /// ordinary build loop, so — like [`Self::job_fds`] — this must stay
    /// interrupt-safe; unlike `job_fds`'s plain set/get, the updates here are
    /// a genuine read-modify-write, so only [`AtomicU32Cell`] (not a `Cell`)
    /// avoids a torn increment under a concurrent signal.
    pub job_slots_used: AtomicU32Cell,

    /// Jobs started since the load average was last sampled, the former
    /// job.rs `static JOB_COUNTER` atomic. Same signal-reentrancy rationale
    /// as [`Self::job_slots_used`] (`reap_children` decrements it).
    pub job_counter: AtomicU64Cell,

    /// Jobserver tokens this make instance currently holds, the former
    /// job.rs `static JOBSERVER_TOKENS` atomic. `pub` because `main.rs`'s
    /// `clean_jobserver` drains it directly on exit. Same signal-reentrancy
    /// rationale as [`Self::job_slots_used`].
    pub jobserver_tokens: AtomicU32Cell,

    /// Children reaped by the `SIGCHLD` handler and not yet processed by the
    /// reap loop, the former job.rs `static DEAD_CHILDREN` atomic.
    /// `child_handler` — the real `SIGCHLD` handler — increments this
    /// directly, so unlike every other signal-reachable `ExecContext` field
    /// this is written from genuinely asynchronous signal delivery, not just
    /// from a normal call path a fatal-signal handler also happens to invoke;
    /// only an atomic (not a `Cell`) avoids a torn read-modify-write.
    pub dead_children: AtomicU32Cell,

    /// Set while a fatal signal is being handled, the former `commands.rs`
    /// `static HANDLING_FATAL_SIGNAL` atomic. `fatal_error_signal` — the real
    /// fatal-signal handler — sets this through the `CTX_PTR` borrow channel
    /// (like [`Self::dead_children`]), so it stays a per-context atomic
    /// rather than a process-wide one: a future multi-tenant host running
    /// several sessions must not have one session's fatal signal look like
    /// another's.
    pub handling_fatal_signal: AtomicBoolCell,

    /// PID of the running `$(shell)` child, or `0` when none, the former
    /// `function.rs` `static SHELL_FUNCTION_PID` atomic. Written by
    /// `func_shell_base` and by the `shell_completed` reaper callback, and
    /// read by `reap_children`, both of which run from `fatal_error_signal`
    /// as well as the ordinary build loop — same signal-reentrancy rationale
    /// as [`Self::dead_children`], and same multi-tenant isolation rationale
    /// as [`Self::handling_fatal_signal`].
    pub shell_function_pid: AtomicI32Cell,

    /// The `$(shell)` completion flag: `0` while the child is still running,
    /// `1` on success, `-1` when the shell could not be started, the former
    /// `function.rs` `static SHELL_FUNCTION_COMPLETED` atomic. Set by
    /// `shell_completed` and spin-waited on by `func_shell_base`. Same
    /// signal-reentrancy and multi-tenant isolation rationale as
    /// [`Self::shell_function_pid`].
    pub shell_function_completed: AtomicI32Cell,

    /// The active jobserver style, the former `posixos.rs` `static JS_TYPE`
    /// atomic. `jobserver_clear` — reached from `fatal_error_signal` through
    /// the `CTX_PTR` borrow channel — resets this, so it needs the same
    /// per-context-atomic treatment as [`Self::handling_fatal_signal`].
    pub js_type: AtomicU8Cell,

    /// True in the process that created the jobserver (and so owns the
    /// fifo), the former `posixos.rs` `static JOB_ROOT` atomic. Read by
    /// `jobserver_clear`, same fatal-signal-reachability rationale as
    /// [`Self::js_type`].
    pub job_root: AtomicBoolCell,

    /// A private dup of the jobserver's read side, closed by a fatal signal
    /// to wake a blocked acquire — the former `posixos.rs` `static JOB_RFD`
    /// atomic. `jobserver_signal` (the real `SIGCHLD` handler's helper,
    /// reached directly with no `&ExecContext` parameter) and
    /// `jobserver_clear` (reached from `fatal_error_signal`) both touch this
    /// through the `CTX_PTR` borrow channel, so — like [`Self::dead_children`]
    /// — only an atomic avoids a torn update under a concurrent signal.
    pub job_rfd: FdSentinelCell,

    /// The output-sync mutex's fd, or `-1` when output sync is off, the
    /// former `posixos.rs` `static OSYNC_HANDLE` atomic. `osync_clear` —
    /// reached from `fatal_error_signal` — resets this, same rationale as
    /// [`Self::job_rfd`].
    pub osync_handle: FdSentinelCell,

    /// True in the process that created the output-sync lock file (and so
    /// unlinks it), the former `posixos.rs` `static SYNC_ROOT` atomic. Read
    /// by `osync_clear`, same fatal-signal-reachability rationale as
    /// [`Self::osync_handle`].
    pub sync_root: AtomicBoolCell,

    /// A read fd that always reports EOF, handed to non-interactive children
    /// as stdin and cached after the first creation — the former
    /// `posixos.rs` `get_bad_stdin` function-local `static BAD_STDIN`
    /// atomic. Not signal-reachable; the atomic is only for the
    /// compare-exchange race between concurrent first-callers within a
    /// session (see `get_bad_stdin`), same as the original.
    pub bad_stdin: FdSentinelCell,

    /// Whether `O_TMPFILE` has been observed to work in this session's temp
    /// dir, the former `posixos.rs` `os_anontmp` function-local `static
    /// TMPFILE_WORKS` atomic (defaults to `true`; `os_anontmp` clears it
    /// after the first `O_TMPFILE` failure so later calls skip straight to
    /// the `tmpfile()` fallback). Not signal-reachable.
    pub tmpfile_works: TrueAtomicBoolCell,

    /// `build_target_list`'s memoized target-list length, the former
    /// `file.rs` function-local `static mut last_targ_count`.
    pub last_targ_count: ::core::cell::Cell<::core::ffi::c_ulong>,

    /// One-shot `.WAIT`-as-a-prerequisite warning flags, the former
    /// `read.rs` function-local `static WPRE`/`static WCMD` atomics inside
    /// `check_special_file`.
    pub wpre_warned: AtomicBoolCell,
    pub wcmd_warned: AtomicBoolCell,

    /// `reap_children`'s one-time "Waiting for unfinished jobs" notice
    /// guard, the former `job.rs` function-local `static PRINTED` atomic.
    pub reap_children_printed: AtomicBoolCell,

    /// `reap_children`'s memoized `.DELETE_ON_ERROR` lookup (`-1` = not yet
    /// computed, `0`/`1` = the answer), the former `job.rs` function-local
    /// `static DELETE_ON_ERROR` atomic. Uses [`FdSentinelCell`] purely for
    /// its `-1`-default `AtomicI32`; this isn't an fd.
    pub delete_on_error: FdSentinelCell,

    /// `func_call`'s saved `$(call)` recursion-depth argument count, the
    /// former `function.rs` function-local `static MAX_ARGS` atomic.
    pub max_args: AtomicU32Cell,

    /// `decode_switches`'s re-entrancy guard, the former `main.rs`
    /// function-local `static USING_GETOPT` atomic.
    pub using_getopt: AtomicBoolCell,

    /// `print_version`'s one-shot-per-run guard, the former `main.rs`
    /// function-local `static PRINTED_VERSION` atomic.
    pub printed_version: AtomicBoolCell,

    /// `die`'s re-entrancy guard (only the first caller actually runs the
    /// cleanup/exit path), the former `main.rs` function-local `static
    /// DYING` atomic.
    pub dying: AtomicBoolCell,

    /// `setup_tmpfile`'s re-entrancy guard, the former `output.rs`
    /// function-local `static IN_SETUP` atomic.
    pub output_in_setup: AtomicBoolCell,

    /// Saved `O_APPEND` flags for stdout/stderr, restored by `output_close`,
    /// the former `output.rs` `static STDOUT_FLAGS`/`static STDERR_FLAGS`
    /// atomics. Both default to `-1` (the "unset" sentinel `fd_set_append`
    /// itself uses), hence [`FdSentinelCell`] rather than [`AtomicI32Cell`].
    pub stdout_flags: FdSentinelCell,
    pub stderr_flags: FdSentinelCell,

    /// `check_io_state`'s memoized stdio-validity bitmask, the former
    /// `posixos.rs` function-local `static IO_STATE` atomic.
    pub io_state: IoStateCell,

    /// Depth of the in-progress environment-variable expansion (used to
    /// detect self-referential recursion), the former `variable.rs` `pub
    /// static ENV_RECURSION` atomic.
    pub env_recursion: AtomicU64Cell,

    /// Monotonic counter bumped whenever the global variable set changes;
    /// used to invalidate the cached `.VARIABLES` value in
    /// `lookup_special_var`, the former `variable.rs` `static
    /// VARIABLE_CHANGENUM` atomic.
    pub variable_changenum: AtomicU64Cell,

    /// `lookup_special_var`'s memoized `.VARIABLES` rebuild point, the
    /// former `variable.rs` function-local `static LAST_CHANGENUM` atomic.
    pub last_changenum: AtomicU64Cell,

    /// Head of the pattern-variable list (`--pattern-target%suffix: ...`
    /// definitions), the former `variable.rs` `static mut pattern_vars`.
    pub pattern_vars: PatternVarsCell,

    /// `create_pattern_var`'s per-target-length fast-insert cache, the
    /// former `variable.rs` `static mut last_pattern_vars` array.
    pub last_pattern_vars: LastPatternVarsCell,

    /// The makefile-remaking goal set consulted by `show_goal_error`, the
    /// former `remake.rs` thread-local `GOAL_LIST`. Populated fresh at the
    /// start of each `update_goal_chain` call, so unlike
    /// [`Self::pattern_vars`] this never needs to survive the `main_0`
    /// context rebuild.
    pub goal_list: ::core::cell::RefCell<Vec<crate::dep::GoalDepNode>>,

    /// The goal currently being processed in `update_goal_chain` (target
    /// `FileId` and resolution flags), the former `remake.rs` thread-local
    /// `GOAL_DEP`.
    pub goal_dep: ::core::cell::Cell<Option<(Option<crate::file::FileId>, crate::dep::DepFlags)>>,

    /// The user-defined/built-in function table, the former `function.rs`
    /// `static mut function_table`. Populated once by
    /// `hash_init_function_table` before the `main_0` context rebuild, so
    /// `main_0` carries it across like [`Self::pattern_vars`].
    pub function_table: FunctionTableCell,

    /// The chain built from `vpath` directives, the former `vpath.rs`
    /// `static mut vpaths`. Rebuilt by `build_vpath_lists` after the
    /// `main_0` context rebuild, so unlike [`Self::function_table`] this
    /// never needs to survive it.
    pub vpaths: VpathChain,

    /// The pseudo-vpath built from the `VPATH` variable, the former
    /// `vpath.rs` `static mut general_vpath`.
    pub general_vpath: VpathChain,

    /// The pseudo-vpath built from the `GPATH` variable, the former
    /// `vpath.rs` `static mut gpaths`.
    pub gpaths: VpathChain,

    /// The `--warn`/`.WARNINGS` configuration, the former `warning.rs`
    /// `static STATE`. Set up by `warning::init` before the `main_0` context
    /// rebuild, so like [`Self::function_table`] it must be carried across
    /// that rebuild — otherwise the configured defaults would be lost and
    /// every later `is_active`/`action` check would see raw
    /// `Action::Unset`. Pure POD content (no pointers), so `Cell` gets a
    /// sound auto-derived `Clone`.
    pub warning_state: ::core::cell::Cell<crate::warning::State>,

    /// The `-d`/`--debug` bitmask, the former `main.rs` `static DB_LEVEL`
    /// atomic. The process reads/writes this from a single thread (option
    /// parsing, then the build), so a plain `Cell` preserves the original
    /// `Relaxed`-ordering semantics with no synchronization cost.
    /// `debug_signal_handler` (a real `SIGUSR1` handler, C-ABI, no `ctx`
    /// parameter) reaches this through the `CTX_PTR` borrow channel, the
    /// same mechanism `job_fds`/`handling_fatal_signal` use for their
    /// signal-handler paths. `decode_debug_flags` (the only setter from
    /// command-line/`MAKEFLAGS` parsing) runs after the `main_0` context
    /// rebuild, so unlike `warning_state`/`function_table` this never needs
    /// to survive it.
    pub db_level: ::core::cell::Cell<i32>,
}

/// [`ExecContext::library_search_cache`]'s fields, split out only because
/// `ExecContext`'s derive needs one `Default` impl per field and this is a
/// single cohesive cache. `buf` is scratch space sized to `libdir_maxlen +
/// buflen + 2`, its contents overwritten on every lookup. `buflen` tracks
/// the longest library basename seen so far, which is distinct from
/// `buf.len()` since `buf` also pads for the directory prefix. `libdir_maxlen`
/// and `std_dirs` are a one-time summary of the fixed search-directory table.
#[derive(Debug, Clone, Default)]
pub struct LibrarySearchCache {
    pub buf: Vec<u8>,
    pub buflen: usize,
    pub libdir_maxlen: usize,
    pub std_dirs: u32,
}

/// A `Cell<[i32; 2]>` that defaults to `[-1, -1]` (the jobserver's "no fds
/// open" sentinel), for the same reason [`PtrCell`]/[`MutPtrCell`] exist:
/// `ExecContext`'s derive needs a `Default` impl, and the sentinel isn't
/// `[i32; 2]`'s own `Default`.
#[derive(Debug, Clone)]
pub struct JobFds(pub ::core::cell::Cell<[i32; 2]>);

impl Default for JobFds {
    fn default() -> Self {
        Self(::core::cell::Cell::new([-1, -1]))
    }
}

/// An owned `AtomicU32` for counters a real signal handler updates
/// concurrently with the main path. Unlike a plain `Cell`, whose
/// non-atomic read-modify-write (`set(get() + 1)`) could tear if a signal
/// interrupts it mid-update and lose an increment, an atomic `fetch_add`/
/// `fetch_sub` is safe under that interruption. `ExecContext` clones
/// snapshot the current value rather than sharing the same atomic cell,
/// since each clone is an independent runtime state, not an alias.
#[derive(Debug, Default)]
pub struct AtomicU32Cell(pub ::core::sync::atomic::AtomicU32);

impl Clone for AtomicU32Cell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicU32::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// The `u64` counterpart of [`AtomicU32Cell`].
#[derive(Debug, Default)]
pub struct AtomicU64Cell(pub ::core::sync::atomic::AtomicU64);

impl Clone for AtomicU64Cell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicU64::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// The `bool` counterpart of [`AtomicU32Cell`], for flags a real signal
/// handler sets (rather than counters it increments).
#[derive(Debug, Default)]
pub struct AtomicBoolCell(pub ::core::sync::atomic::AtomicBool);

impl Clone for AtomicBoolCell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicBool::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// The signed-`i32` counterpart of [`AtomicU32Cell`], for values like a PID
/// or a tri-state completion flag that are naturally signed.
#[derive(Debug, Default)]
pub struct AtomicI32Cell(pub ::core::sync::atomic::AtomicI32);

impl Clone for AtomicI32Cell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicI32::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// The `u8` counterpart of [`AtomicU32Cell`], for small enums stored as a
/// byte (e.g. the jobserver style) that a fatal-signal handler resets.
#[derive(Debug, Default)]
pub struct AtomicU8Cell(pub ::core::sync::atomic::AtomicU8);

impl Clone for AtomicU8Cell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicU8::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// An `AtomicI32` defaulting to `-1` (the "no fd" sentinel), for the same
/// reason [`JobFds`] exists: `ExecContext`'s derive needs a `Default` impl,
/// and `-1` isn't `AtomicI32`'s own default.
#[derive(Debug)]
pub struct FdSentinelCell(pub ::core::sync::atomic::AtomicI32);

impl Default for FdSentinelCell {
    fn default() -> Self {
        Self(::core::sync::atomic::AtomicI32::new(-1))
    }
}

impl Clone for FdSentinelCell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicI32::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// The `bool` counterpart of [`FdSentinelCell`]: an `AtomicBool` defaulting
/// to `true` rather than `AtomicBool`'s own `false` default.
#[derive(Debug)]
pub struct TrueAtomicBoolCell(pub ::core::sync::atomic::AtomicBool);

impl Default for TrueAtomicBoolCell {
    fn default() -> Self {
        Self(::core::sync::atomic::AtomicBool::new(true))
    }
}

impl Clone for TrueAtomicBoolCell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicBool::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// An `AtomicU32` defaulting to [`crate::posixos::IO_UNKNOWN`] (the
/// "not yet computed" sentinel `check_io_state` checks for), rather than
/// `AtomicU32`'s own `0` default — `0` would be misread as an already-cached
/// "nothing is OK" result.
#[derive(Debug)]
pub struct IoStateCell(pub ::core::sync::atomic::AtomicU32);

impl Default for IoStateCell {
    fn default() -> Self {
        Self(::core::sync::atomic::AtomicU32::new(
            crate::posixos::IO_UNKNOWN as u32,
        ))
    }
}

impl Clone for IoStateCell {
    fn clone(&self) -> Self {
        Self(::core::sync::atomic::AtomicU32::new(
            self.0.load(::core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

/// A `Cell<*mut pattern_var>` that defaults to null, for the same reason
/// [`ChildChain`] exists: raw pointers have no `Default`.
#[derive(Debug, Clone)]
pub struct PatternVarsCell(pub ::core::cell::Cell<*mut crate::variable::pattern_var>);

impl Default for PatternVarsCell {
    fn default() -> Self {
        Self(::core::cell::Cell::new(::core::ptr::null_mut()))
    }
}

/// A `Cell<[*mut pattern_var; 256]>` that defaults to all-null:
/// `create_pattern_var`'s per-target-length fast-insert cache.
#[derive(Debug, Clone)]
pub struct LastPatternVarsCell(
    pub ::core::cell::Cell<[*mut crate::variable::pattern_var; 256]>,
);

impl Default for LastPatternVarsCell {
    fn default() -> Self {
        Self(::core::cell::Cell::new(
            [::core::ptr::null_mut(); 256],
        ))
    }
}

/// One frame of the `ifdef`/`ifeq` conditional-nesting stack (the former C
/// `struct conditionals`'s `ignoring`/`seen_else` parallel arrays, sized by a
/// separate `if_cmds`/`allocated` pair). Here `if_cmds` is simply
/// `ignoring.len()` (`== seen_else.len()`, the two always grow and shrink in
/// lockstep) — genuine `Vec::push`/`pop` replaces the manual
/// `xmalloc`/`xrealloc`-by-fives growth, so there is no separate capacity
/// field to track.
///
/// `ignoring[i]`: `0` = this level's lines are active, `1` = ignoring because
/// this level's condition was false, `2` = ignoring because an enclosing
/// level is ignoring (or this level's `if`/`else` branch already ran).
/// `seen_else[i]`: whether this level has already seen its one allowed
/// `else`.
#[derive(Debug, Clone, Default)]
pub struct ConditionalsFrame {
    pub ignoring: Vec<u8>,
    pub seen_else: Vec<u8>,
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
    pub ::core::cell::RefCell<rustc_hash::FxHashMap<Box<[u8]>, Box<crate::dir::directory>>>,
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

/// make's central file table, idiomatic edition: a
/// [`FileId`](crate::file::FileId)-keyed `FxHashMap` of `Arc<Mutex<FileNode>>`,
/// behind a `Mutex` for the interior mutability `&ExecContext` needs. This is
/// the SOLE file store (the raw-pointer file table has been removed) — nodes are
/// owned by the arena (reference-counted, shared safely) and referenced
/// elsewhere by the `Copy` [`FileId`] handle, so there is no `*mut file`.
pub struct FileArena(
    pub  ::std::sync::Mutex<
        rustc_hash::FxHashMap<
            crate::file::FileId,
            ::std::sync::Arc<::std::sync::Mutex<crate::file::FileNode>>,
        >,
    >,
);

impl Default for FileArena {
    fn default() -> Self {
        Self(::std::sync::Mutex::new(rustc_hash::FxHashMap::default()))
    }
}

impl Clone for FileArena {
    fn clone(&self) -> Self {
        // A real clone (unlike the raw-pointer `FileTable`, whose clone had to
        // return empty): the entries are `Arc<Mutex<FileNode>>`, so cloning the
        // map clones the `Arc` handles and the cloned arena shares the very same
        // nodes — sound and meaningful.
        Self(::std::sync::Mutex::new(
            self.0.lock().expect("file arena poisoned").clone(),
        ))
    }
}

impl ::core::fmt::Debug for FileArena {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileArena")
            .field(&self.0.lock().expect("file arena poisoned").len())
            .finish()
    }
}

impl FileArena {
    /// Look up a node by its [`FileId`](crate::file::FileId) handle, returning a
    /// cloned `Arc` so the caller can lock it without holding the map guard.
    pub fn get(
        &self,
        id: crate::file::FileId,
    ) -> Option<::std::sync::Arc<::std::sync::Mutex<crate::file::FileNode>>> {
        self.0
            .lock()
            .expect("file arena poisoned")
            .get(&id)
            .map(::std::sync::Arc::clone)
    }

    /// Intern `node`, returning its [`FileId`](crate::file::FileId) handle. An
    /// existing entry under the same id is left untouched (interning is
    /// idempotent on identity).
    pub fn intern(&self, node: crate::file::FileNode) -> crate::file::FileId {
        let id = node.id();
        self.0
            .lock()
            .expect("file arena poisoned")
            .entry(id)
            .or_insert_with(|| ::std::sync::Arc::new(::std::sync::Mutex::new(node)));
        id
    }

    /// Get the node under `id`, inserting `f()` if absent. Returns the (cloned)
    /// `Arc` either way. The node is interned under the caller-supplied `id`
    /// (not recomputed from the created node), so callers that derived `id`
    /// from the byte-exact hash name stay consistent even for names that do not
    /// round-trip through `String`.
    pub fn get_or_insert_with(
        &self,
        id: crate::file::FileId,
        f: impl FnOnce() -> crate::file::FileNode,
    ) -> ::std::sync::Arc<::std::sync::Mutex<crate::file::FileNode>> {
        ::std::sync::Arc::clone(
            self.0
                .lock()
                .expect("file arena poisoned")
                .entry(id)
                .or_insert_with(|| ::std::sync::Arc::new(::std::sync::Mutex::new(f()))),
        )
    }

    /// Number of interned files.
    pub fn len(&self) -> usize {
        self.0.lock().expect("file arena poisoned").len()
    }

    /// Whether the arena holds no files.
    pub fn is_empty(&self) -> bool {
        self.0.lock().expect("file arena poisoned").is_empty()
    }
}

/// Wrapper giving [`ExecContext::load_proc_fd`] its `-2` ("not yet probed")
/// initial value while `ExecContext` keeps deriving `Default`.
#[derive(Debug, Clone)]
pub struct LoadProcFd(pub ::core::cell::Cell<i32>);

/// A `Cell<*const c_char>` that defaults to null — raw pointers have no
/// `Default`, so `ExecContext`'s derive needs the wrapper.
#[derive(Debug, Clone)]
pub struct PtrCell(pub ::core::cell::Cell<*const ::core::ffi::c_char>);

impl Default for PtrCell {
    fn default() -> Self {
        PtrCell(::core::cell::Cell::new(::core::ptr::null()))
    }
}

/// A `Cell<*mut c_char>` that defaults to null, for the same reason.
#[derive(Debug, Clone)]
pub struct MutPtrCell(pub ::core::cell::Cell<*mut ::core::ffi::c_char>);

impl Default for MutPtrCell {
    fn default() -> Self {
        MutPtrCell(::core::cell::Cell::new(::core::ptr::null_mut()))
    }
}

/// An `Arc<dyn RemoteBackend>` that defaults to the stub backend — trait
/// objects have no `Default`, so `ExecContext`'s derive needs the wrapper.
/// `Arc` (rather than `Box`) so cloning `ExecContext` is a cheap refcount
/// bump instead of requiring `RemoteBackend: Clone` (which trait objects
/// can't derive).
#[derive(Clone)]
pub struct RemoteBackendCell(
    pub ::std::sync::Arc<dyn crate::remote_stub::RemoteBackend + Send + Sync>,
);

impl Default for RemoteBackendCell {
    fn default() -> Self {
        Self(::std::sync::Arc::new(crate::remote_stub::StubRemote))
    }
}

impl ::core::fmt::Debug for RemoteBackendCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemoteBackendCell").finish_non_exhaustive()
    }
}

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

/// A `Cell<variable>` holding [`ExecContext::shell_var`]. `variable` is a
/// `Copy` c2rust record, so sites read the whole record out, update fields,
/// and store it back. Defaults to the unset record (null name/value, zero
/// location) the former `static mut` initializer produced.
#[derive(Clone)]
pub struct ShellVar(pub ::core::cell::Cell<crate::variable::variable>);

impl Default for ShellVar {
    fn default() -> Self {
        Self(::core::cell::Cell::new(crate::variable::variable {
            name: ::core::ptr::null_mut(),
            value: ::core::ptr::null_mut(),
            fileinfo: crate::floc::Floc {
                filenm: ::core::ptr::null(),
                lineno: 0,
                offset: 0,
            },
            length: 0,
            recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export:
                [0; 4],
        }))
    }
}

impl ::core::fmt::Debug for ShellVar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        // `variable` has no `Debug`; whether SHELL was seen is the useful bit.
        f.debug_tuple("ShellVar")
            .field(&!self.0.get().value.is_null())
            .finish()
    }
}

/// A `Cell<*mut command_variable>` (list head) that defaults to null, for
/// [`ExecContext::command_variables`] — raw pointers have no `Default`.
#[derive(Debug, Clone)]
pub struct CommandVariables(pub ::core::cell::Cell<*mut crate::make_main::command_variable>);

impl Default for CommandVariables {
    fn default() -> Self {
        Self(::core::cell::Cell::new(::core::ptr::null_mut()))
    }
}

/// A `Cell<*mut variable>` that defaults to null, for
/// [`ExecContext::default_goal_var`].
#[derive(Debug, Clone)]
pub struct DefaultGoalVar(pub ::core::cell::Cell<*mut crate::variable::variable>);

impl Default for DefaultGoalVar {
    fn default() -> Self {
        Self(::core::cell::Cell::new(::core::ptr::null_mut()))
    }
}

/// A `Cell<SigsetT>` holding [`ExecContext::fatal_signal_set`], defaulting to
/// the empty set the former `static mut` initializer produced.
#[derive(Clone)]
pub struct FatalSignalSet(pub ::core::cell::Cell<crate::make_main::SigsetT>);

impl Default for FatalSignalSet {
    fn default() -> Self {
        Self(::core::cell::Cell::new(crate::make_main::SigsetT {
            __val: [0; 16],
        }))
    }
}

impl ::core::fmt::Debug for FatalSignalSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        // `SigsetT` has no `Debug`; the raw mask words identify the set.
        f.debug_tuple("FatalSignalSet")
            .field(&self.0.get().__val)
            .finish()
    }
}

/// Box-owned `Cell<output>` holding [`ExecContext::make_sync`]. The `Cell`
/// gives the same interior mutability the other context fields use; the `Box`
/// pins the record's address for the pointer-identity uses
/// (`output_context == make_sync`). Defaults to the zeroed record the former
/// `static mut` initializer produced (`output_init` configures it at
/// startup). `Clone` allocates a fresh box — a cloned context is a new run
/// with its own sync record, never an alias of the original's.
#[derive(Clone)]
pub struct MakeSync(pub Box<::core::cell::Cell<crate::output::output>>);

impl MakeSync {
    /// The stable address of the owned `output` record, for the pointer-based
    /// `output_init`/`output_close` calls and the `output_context` identity
    /// compare.
    pub fn as_ptr(&self) -> *mut crate::output::output {
        self.0.as_ptr()
    }
}

impl Default for MakeSync {
    fn default() -> Self {
        Self(Box::new(::core::cell::Cell::new(crate::output::output {
            out: 0,
            err: 0,
            syncout: [0; 1],
            c2rust_padding: [0; 3],
        })))
    }
}

impl ::core::fmt::Debug for MakeSync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        // `output` has no `Debug`; the descriptors and the syncout bit are
        // the useful state.
        let o = self.0.get();
        f.debug_struct("MakeSync")
            .field("out", &o.out)
            .field("err", &o.err)
            .field("syncout", &(o.syncout[0] & 1))
            .finish()
    }
}

/// A `Cell<*mut output>` that defaults to null (no active sync target), for
/// [`ExecContext::output_context`] — raw pointers have no `Default`.
#[derive(Debug, Clone)]
pub struct OutputContext(pub ::core::cell::Cell<*mut crate::output::output>);

impl Default for OutputContext {
    fn default() -> Self {
        Self(::core::cell::Cell::new(::core::ptr::null_mut()))
    }
}

/// Box-free `Cell<[u8; 100]>` holding [`ExecContext::pid_string`]. Plain byte
/// array, not a pointer: `Cell::as_ptr` gives `pid2str` a stable address to
/// `sprintf` into and return (cast to `*mut c_char` only at that FFI
/// boundary), without the indirection `PtrCell` would need.
#[derive(Clone)]
pub struct PidString(pub ::core::cell::Cell<[u8; 100]>);

impl Default for PidString {
    fn default() -> Self {
        Self(::core::cell::Cell::new([0; 100]))
    }
}

impl ::core::fmt::Debug for PidString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let bytes = self.0.get();
        let nul = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        f.debug_tuple("PidString")
            .field(&::core::str::from_utf8(&bytes[..nul]).unwrap_or("<invalid>"))
            .finish()
    }
}

/// A `Cell<*mut child>` list head that defaults to null (an empty chain), for
/// [`ExecContext::children`] and [`ExecContext::waiting_jobs`] — raw pointers
/// have no `Default`.
#[derive(Debug, Clone)]
pub struct ChildChain(pub ::core::cell::Cell<*mut crate::job::child>);

impl Default for ChildChain {
    fn default() -> Self {
        Self(::core::cell::Cell::new(::core::ptr::null_mut()))
    }
}

/// A `Cell<*mut Vpath>` list head that defaults to null, for
/// [`ExecContext::vpaths`]/[`ExecContext::general_vpath`]/
/// [`ExecContext::gpaths`] — raw pointers have no `Default`.
#[derive(Debug)]
pub struct VpathChain(pub ::core::cell::Cell<*mut crate::vpath::Vpath>);

impl Default for VpathChain {
    fn default() -> Self {
        Self(::core::cell::Cell::new(::core::ptr::null_mut()))
    }
}

impl Clone for VpathChain {
    fn clone(&self) -> Self {
        // Per-run build state rebuilt fresh by `build_vpath_lists` after each
        // `main_0` context rebuild, never carried across it or genuinely
        // cloned; deriving `Clone` would copy the raw `*mut Vpath` by value,
        // aliasing the same chain across two contexts with no way to tell
        // which owns it — the same hazard Codex found in `FunctionTableCell`
        // on PR #498. A fresh null pointer is the right (and only sound)
        // snapshot.
        Self::default()
    }
}

/// The user-defined/built-in function table (`$(call)`-able functions,
/// including those registered by `gmk_add_function`), the former
/// `function.rs` `static mut function_table`. `hash_table` is a `Copy`
/// c2rust record; call sites take `.0.as_ptr()` for the `*mut hash_table`
/// the `hash.rs` FFI-shaped insert/lookup functions expect, the same
/// raw-pointer-into-`Cell`-interior treatment [`Self::pattern_vars`] gets.
pub struct FunctionTableCell(pub ::core::cell::Cell<crate::hash::hash_table>);

impl Default for FunctionTableCell {
    fn default() -> Self {
        Self(::core::cell::Cell::new(crate::hash::hash_table {
            ht_vec: ::core::ptr::null_mut(),
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
        }))
    }
}

impl Clone for FunctionTableCell {
    fn clone(&self) -> Self {
        // Per-run build state handed across the `main_0` context rebuild by
        // `mem::take`, never by clone; deriving `Clone` would copy the
        // `hash_table` record's `ht_vec` pointer by value, aliasing the same
        // slot array across two contexts with independently-mutable
        // `ht_fill`/`ht_size` counters. A fresh empty table is the right (and
        // only sound) snapshot, matching `DirNameTable`/`DirContentsTable`.
        Self::default()
    }
}

impl ::core::fmt::Debug for FunctionTableCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        // `hash_table` has no `Debug`; the fill count is the useful bit.
        f.debug_tuple("FunctionTableCell")
            .field(&self.0.get().ht_fill)
            .finish()
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

    /// Which shell personality is in effect (always [`ShellKind::Unixy`] in
    /// this POSIX port).
    pub fn shell_kind(&self) -> ShellKind {
        self.config.shell_kind
    }

    /// Default system include directories searched by `-I` when not disabled.
    pub fn default_include_directories(&self) -> [&'static [u8]; 3] {
        self.config.default_include_directories
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ConditionalsFrame, ExecContext, FileArena};

    #[test]
    fn context_exposes_makelevel() {
        let ctx = ExecContext::new(Config { makelevel: 3, ..Default::default() });
        assert_eq!(ctx.makelevel(), 3);
        // Cloning yields an independent copy of the owned state.
        assert_eq!(ctx.clone().makelevel(), 3);
    }

    #[test]
    fn default_makelevel_is_zero() {
        assert_eq!(ExecContext::default().makelevel(), 0);
    }

    /// `make_sync`'s address is captured by `output_context` before the
    /// `main_0` build-phase rebuild and identity-compared after, so the
    /// carried Box must keep pointing at the same heap record through the
    /// `mem::take` + struct-update dance the rebuild performs. A clone, by
    /// contrast, is a new run and must get its own record.
    #[test]
    fn make_sync_address_survives_the_rebuild_carry() {
        let mut ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
        let addr = ctx.make_sync.as_ptr();
        ctx.output_context.0.set(addr);

        // Mirror main_0's rebuild: take the carried fields, move them into a
        // rebuilt context.
        let carried_make_sync = ::core::mem::take(&mut ctx.make_sync);
        let carried_output_context = ::core::mem::take(&mut ctx.output_context);
        let ctx = ExecContext {
            make_sync: carried_make_sync,
            output_context: carried_output_context,
            ..ExecContext::new(Config { makelevel: 2, ..Default::default() })
        };
        assert_eq!(ctx.make_sync.as_ptr(), addr, "carry must not move the record");
        assert_eq!(
            ctx.output_context.0.get(),
            ctx.make_sync.as_ptr(),
            "the captured pointer still identifies the carried record"
        );

        assert_ne!(
            ctx.clone().make_sync.as_ptr(),
            addr,
            "a cloned context owns a fresh record, never an alias"
        );
    }

    /// The sync record starts as the zeroed struct the former `static mut`
    /// initializer produced (startup's `output_init` configures it), and the
    /// sync target starts unset.
    #[test]
    fn output_sync_state_starts_like_the_former_statics() {
        let ctx = ExecContext::default();
        let ms = ctx.make_sync.0.get();
        assert_eq!((ms.out, ms.err, ms.syncout[0]), (0, 0, 0));
        assert!(ctx.output_context.0.get().is_null());
    }

    /// `pid2str` writes through `Cell::as_ptr()` and returns that same
    /// address; the wrapper must round-trip bytes and keep the address
    /// stable across gets (the former static's address never moved either).
    #[test]
    fn pid_string_round_trips_and_has_a_stable_address() {
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
        assert_eq!(ctx.pid_string.0.get()[0], 0, "starts empty");
        let addr = ctx.pid_string.0.as_ptr();

        let mut buf = ctx.pid_string.0.get();
        buf[0] = b'4';
        buf[1] = b'2';
        ctx.pid_string.0.set(buf);

        assert_eq!(ctx.pid_string.0.as_ptr(), addr, "address is stable");
        assert_eq!(ctx.pid_string.0.get()[0], b'4');
        assert_eq!(ctx.pid_string.0.get()[1], b'2');
    }

    /// The children and postponed-jobs chains (former job.rs `static mut
    /// children`/`waiting_jobs`) start empty and are per-run mutable state:
    /// pushing a head is observable through the shared `&ExecContext`, which
    /// is how `start_waiting_job`/`reap_children` and the fatal-signal
    /// handler (via the `CTX_PTR` channel) all see one chain.
    #[test]
    fn child_chains_start_empty_and_mutate_in_place() {
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
        assert!(ctx.children.0.get().is_null());
        assert!(ctx.waiting_jobs.0.get().is_null());

        let head = 0x1000usize as *mut crate::job::child;
        ctx.children.0.set(head);
        ctx.waiting_jobs.0.set(head);
        assert_eq!(ctx.children.0.get(), head);
        assert_eq!(ctx.waiting_jobs.0.get(), head);
        // A fresh context (a sub-build or a test double) starts with its own
        // empty chains, not the process-wide list the statics used to share.
        assert!(ExecContext::default().children.0.get().is_null());
    }

    #[test]
    fn load_sample_cache_starts_zeroed() {
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
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
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
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
            ..ExecContext::new(Config { makelevel: 1, ..Default::default() })
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
            ..ExecContext::new(Config { makelevel: 1, ..Default::default() })
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

    /// `parse_file_seq`'s reused scratch buffer (the former function-local
    /// `static mut tmpbuf`/`tmpbuf_len`) starts empty and survives the
    /// `main_0` carry-over the same way [`read_dirstream_buffer_starts_empty_and_survives_carry_over`]
    /// verifies for `read_dirstream_buf`.
    #[test]
    fn file_seq_tmpbuf_starts_empty_and_survives_carry_over() {
        let ctx = ExecContext::default();
        assert!(ctx.file_seq_tmpbuf.borrow().is_empty());

        let mut populated = ExecContext::default();
        populated.file_seq_tmpbuf.borrow_mut().extend_from_slice(b"scratch");

        let carried_buf = ::core::mem::take(&mut populated.file_seq_tmpbuf);
        let rebuilt = ExecContext {
            file_seq_tmpbuf: carried_buf,
            ..ExecContext::new(Config { makelevel: 1, ..Default::default() })
        };
        assert_eq!(&*rebuilt.file_seq_tmpbuf.borrow(), b"scratch");
        assert!(populated.file_seq_tmpbuf.borrow().is_empty());
        assert_eq!(rebuilt.makelevel(), 1);

        assert!(ExecContext::default().file_seq_tmpbuf.borrow().is_empty());
    }

    /// The variable trio (`shell_var` / `command_variables` /
    /// `default_goal_var`, the former main.rs statics) starts unset, and the
    /// two members written before the `main_0` build-phase context rebuild
    /// (`shell_var`, `command_variables`) survive the carry-over the same way
    /// the directory cache does.
    #[test]
    fn variable_trio_starts_unset_and_survives_carry_over() {
        let ctx = ExecContext::default();
        assert!(ctx.shell_var.0.get().name.is_null());
        assert!(ctx.shell_var.0.get().value.is_null());
        assert_eq!(ctx.shell_var.0.get().length, 0);
        assert!(ctx.command_variables.0.get().is_null());
        assert!(ctx.default_goal_var.0.get().is_null());
        // The Debug impl reports whether SHELL was seen (manual impl:
        // `variable` itself has no `Debug`).
        assert_eq!(format!("{:?}", ctx.shell_var), "ShellVar(false)");

        // Simulate the startup environment scan recording SHELL and a `V=x`
        // switch pushing a command variable, then the rebuild hand-off.
        let mut populated = ExecContext::default();
        let mut sv = populated.shell_var.0.get();
        sv.name = c"SHELL".as_ptr() as *mut ::core::ffi::c_char;
        sv.length = 5;
        sv.value = c"/bin/sh".as_ptr() as *mut ::core::ffi::c_char;
        populated.shell_var.0.set(sv);
        populated
            .command_variables
            .0
            .set(0x1 as *mut crate::make_main::command_variable);

        let carried_shell_var = ::core::mem::take(&mut populated.shell_var);
        let carried_command_variables = ::core::mem::take(&mut populated.command_variables);
        let rebuilt = ExecContext {
            shell_var: carried_shell_var,
            command_variables: carried_command_variables,
            ..ExecContext::new(Config { makelevel: 1, ..Default::default() })
        };
        assert_eq!(rebuilt.shell_var.0.get().length, 5);
        assert!(!rebuilt.shell_var.0.get().value.is_null());
        assert_eq!(format!("{:?}", rebuilt.shell_var), "ShellVar(true)");
        assert!(!rebuilt.command_variables.0.get().is_null());
        // The source fields reset to unset; a fresh context inherits nothing.
        assert!(populated.shell_var.0.get().value.is_null());
        assert!(populated.command_variables.0.get().is_null());
        // `default_goal_var` is only defined after the rebuild, so the rebuilt
        // context correctly starts without one.
        assert!(rebuilt.default_goal_var.0.get().is_null());
    }

    /// The fatal-signal set (the former main.rs `pub static mut
    /// fatal_signal_set`) starts empty, holds the mask words the startup
    /// `sigaddset` calls write, and survives the `main_0` build-phase
    /// carry-over so `block_sigs`/`unblock_sigs` mask the real set during the
    /// build.
    #[test]
    fn fatal_signal_set_starts_empty_and_survives_carry_over() {
        let ctx = ExecContext::default();
        assert!(ctx.fatal_signal_set.0.get().__val.iter().all(|&w| w == 0));
        assert_eq!(
            format!("{:?}", ctx.fatal_signal_set),
            format!("FatalSignalSet({:?})", [0u64; 16])
        );

        // Simulate `install_fatal_signal` adding SIGINT (bit 1 of word 0, as
        // `sigaddset(set, 2)` does on Linux), then the rebuild hand-off.
        let mut populated = ExecContext::default();
        let mut set = populated.fatal_signal_set.0.get();
        set.__val[0] |= 1 << 1;
        populated.fatal_signal_set.0.set(set);

        let carried = ::core::mem::take(&mut populated.fatal_signal_set);
        let rebuilt = ExecContext {
            fatal_signal_set: carried,
            ..ExecContext::new(Config { makelevel: 1, ..Default::default() })
        };
        assert_eq!(rebuilt.fatal_signal_set.0.get().__val[0], 1 << 1);
        // The source field reset to empty; a fresh context inherits nothing.
        assert!(populated
            .fatal_signal_set
            .0
            .get()
            .__val
            .iter()
            .all(|&w| w == 0));
    }

    /// The `.NOTINTERMEDIATE`/`.SECONDARY` latches start unset and are per-run
    /// (a fresh context, e.g. a new make invocation, sees them `false` again),
    /// replacing the former process-global `no_intermediates`/`ALL_SECONDARY`.
    #[test]
    fn intermediate_latches_start_unset_and_are_per_run() {
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
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
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
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
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
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
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
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
        let ctx = ExecContext::new(Config { makelevel: 0, ..Default::default() });
        assert_eq!(ctx.commands_started.get(), 0);
        assert_eq!(ctx.considered.get(), 0);

        // The bump idiom `update_goal_chain` / `start_job_command` use.
        ctx.commands_started
            .set(ctx.commands_started.get().wrapping_add(1));
        ctx.considered.set(ctx.considered.get().wrapping_add(1));
        assert_eq!(ctx.commands_started.get(), 1);
        assert_eq!(ctx.considered.get(), 1);

        // Per-run: a fresh context does not inherit the counts.
        assert_eq!(ExecContext::default().commands_started.get(), 0);
        assert_eq!(ExecContext::default().considered.get(), 0);
    }

    /// The idiomatic file arena interns a [`FileNode`](crate::file::FileNode)
    /// and hands back a `FileId` handle that round-trips through `get`. Unlike
    /// the raw-pointer `FileTable` (whose `Clone` had to return empty), the
    /// arena's `Clone` is real and meaningful: the clone shares the very same
    /// `Arc<Mutex<FileNode>>` nodes, so a mutation through one arena is visible
    /// through the other.
    #[test]
    fn file_arena_interns_and_clone_shares_nodes() {
        use crate::file::FileNode;

        let arena = FileArena::default();
        assert!(arena.is_empty());

        let id = arena.intern(FileNode::new(b"foo.o".to_vec()));
        assert_eq!(arena.len(), 1);
        assert_eq!(id, FileNode::new(b"foo.o".to_vec()).id());
        assert_eq!(
            arena
                .get(id)
                .expect("interned node present")
                .lock()
                .unwrap()
                .name,
            b"foo.o"
        );

        // A real clone: same underlying node, shared by `Arc`.
        let clone = arena.clone();
        assert_eq!(clone.len(), 1);
        clone
            .get(id)
            .expect("clone shares the node")
            .lock()
            .unwrap()
            .phony = true;
        assert!(
            arena.get(id).unwrap().lock().unwrap().phony,
            "mutation through the clone is visible through the original — a genuine shared clone"
        );
    }

    /// `ctx.conditionals` starts empty (no open `if`/`ifdef`), and
    /// `RefCell::replace` gives exactly the former `install_conditionals`/
    /// `restore_conditionals` swap: installing a fresh frame for a nested
    /// scope (e.g. `include`) returns the enclosing frame, and replacing it
    /// back drops the nested frame's `Vec`s.
    #[test]
    fn conditionals_frame_install_and_restore_round_trips() {
        let ctx = ExecContext::default();
        assert!(ctx.conditionals.borrow().ignoring.is_empty());
        assert!(ctx.conditionals.borrow().seen_else.is_empty());

        // Simulate an open `ifeq` (mid-file) before a nested `include`.
        ctx.conditionals.borrow_mut().ignoring.push(0);
        ctx.conditionals.borrow_mut().seen_else.push(0);

        let enclosing = ctx
            .conditionals
            .replace(ConditionalsFrame::default());
        assert_eq!(enclosing.ignoring, vec![0]);
        assert!(
            ctx.conditionals.borrow().ignoring.is_empty(),
            "the nested scope starts with its own independent, empty frame"
        );

        // The nested scope opens (and properly closes) its own conditional;
        // restoring hands the enclosing frame back untouched.
        ctx.conditionals.borrow_mut().ignoring.push(1);
        ctx.conditionals.replace(enclosing);
        assert_eq!(ctx.conditionals.borrow().ignoring, vec![0]);
    }
}
