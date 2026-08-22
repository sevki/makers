//! Process-global state that stops two tenants sharing one process.
//!
//! Phase E (#598) puts every tenant of the build server in *one* process. This
//! crate is a c2rust translation of GNU make, which was written as one process
//! serving one build, so a good deal of its state is process-global by
//! inheritance rather than by choice. Nothing about that is visible from a
//! single-tenant run — every other test in the suite passes with it — which is
//! exactly why it needs pinning here.
//!
//! Each hazard gets a test asserting what a *multi-tenant* process needs, and
//! each arrives in the same pull request as the fix that makes it pass, as two
//! commits: the test alone (red), then the fix (green). AGENTS.md asks for a
//! red run because "a test that has never been seen red proves nothing" —
//! keeping the pair in one PR means no finished fix waits on an unrelated one.
//!
//! This file covers the umask hazard (#608) and the workspace-root hazard
//! (#605). The remaining sibling arrives with its own fix: the session-scoped
//! interner in #607.
//!
//! Two hazards from #598 are deliberately not covered anywhere:
//!
//! * **The `waitpid(-1)` reaper** (blocker 1, the sharpest one). `reap_children`
//!   only enters its loop with a populated `ctx.children` chain, which needs the
//!   job machinery a test cannot reasonably stand up. It belongs to #604, next
//!   to the code that fixes it.
//! * **`ExecContext: !Send`** (blocker 7). This cannot be a runtime test: it is
//!   a compile error, which is the point. Today `ExecContext` is `!Send` because
//!   of `Rc<RefCell<StdoutSink>>`, `Rc<RefCell<StderrSink>>` and its raw
//!   `*mut c_char` fields, so the compiler stops you sharing one context between
//!   threads and every test here builds a fresh context per thread. When #607
//!   retires those fields it should add the one-line `assert_send::<ExecContext>()`
//!   this module cannot write yet.

use std::ffi::{CStr, CString};
use std::os::unix::fs::PermissionsExt as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;

use make_sys::execctx::ExecContext;

/// These tests mutate process-global state (umask, cwd), so they must not run
/// against each other inside this binary.
fn hazard_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn current_umask() -> libc::mode_t {
    // Reading the umask means setting it; put it straight back.
    unsafe {
        let m = libc::umask(0);
        libc::umask(m);
        m
    }
}

/// One tenant's worth of temp-file churn: the `umask(0o77)` … `umask(mask)`
/// window in `misc::open_named_tmpfd` (`src/misc.rs:758`).
fn churn_temp_files(count: usize) {
    let ctx = ExecContext::default();
    for _ in 0..count {
        let (fd, name) = unsafe { make_sys::misc::open_named_tmpfd(&ctx) };
        if fd >= 0 {
            unsafe { libc::close(fd) };
        }
        if !name.is_null() {
            unsafe {
                libc::unlink(name);
                libc::free(name.cast());
            }
        }
    }
}

/// Making temp files leaves the process umask exactly as it was found.
///
/// This passed before #608 too, because the borrow was correctly restored on
/// the single-tenant path; it passes now because nothing is borrowed at all.
/// Keep it either way — it is the cheap check that no temp-file path has
/// started reaching for a process-wide setting again.
#[test]
fn a_single_tenant_restores_the_process_umask() {
    let _guard = hazard_lock().lock().unwrap_or_else(|e| e.into_inner());
    let entry = current_umask();
    unsafe { libc::umask(0o022) };

    churn_temp_files(200);

    let after = current_umask();
    unsafe { libc::umask(entry) };
    assert_eq!(
        after, 0o022,
        "one tenant alone must restore the umask it borrowed"
    );
}
/// What the umask borrow was reaching for, stated directly: the temp file is
/// private to its owner. Asserted under a deliberately permissive umask, so it
/// passes only if the guarantee comes from the file's own creation mode rather
/// than from a process-wide setting somebody else can change (#608).
#[test]
fn open_named_tmpfd_mode_is_private() {
    let _guard = hazard_lock().lock().unwrap_or_else(|e| e.into_inner());
    let entry = current_umask();
    unsafe { libc::umask(0) };

    let ctx = ExecContext::default();
    let (fd, name) = unsafe { make_sys::misc::open_named_tmpfd(&ctx) };
    assert!(fd >= 0, "could not create a temp file");
    let path = unsafe { CStr::from_ptr(name) }.to_str().unwrap().to_owned();
    let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;

    unsafe {
        libc::close(fd);
        libc::unlink(name);
        libc::free(name.cast());
        libc::umask(entry);
    }
    assert_eq!(
        mode, 0o600,
        "temp file created {mode:#o} under a permissive umask"
    );
}
/// `open_named_tmpfd` saves the old umask, sets `0o77`, and restores the saved
/// value. Two tenants interleaved in that window both save `0o77` and both
/// "restore" it, so the process is left permanently restrictive.
#[test]
fn concurrent_tenants_do_not_corrupt_the_process_umask() {
    let _guard = hazard_lock().lock().unwrap_or_else(|e| e.into_inner());
    let entry = current_umask();
    unsafe { libc::umask(0o022) };

    let tenants: Vec<_> = (0..4)
        .map(|_| thread::spawn(|| churn_temp_files(200)))
        .collect();
    for t in tenants {
        t.join().unwrap();
    }

    let after = current_umask();
    unsafe { libc::umask(entry) };
    assert_eq!(
        after, 0o022,
        "four tenants making temp files left the process umask at {after:#o}"
    );
}
/// The consequence that makes this more than housekeeping: while one tenant is
/// inside that window, *another* tenant's build outputs are created against the
/// borrowed `0o77`, so they come out `0o600` instead of `0o644`.
#[test]
fn one_tenants_temp_files_do_not_change_another_tenants_output_permissions() {
    let _guard = hazard_lock().lock().unwrap_or_else(|e| e.into_inner());
    let entry = current_umask();
    unsafe { libc::umask(0o022) };

    let stop = Arc::new(AtomicBool::new(false));
    let neighbour = {
        let stop = Arc::clone(&stop);
        thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                churn_temp_files(20);
            }
        })
    };

    // Own subdirectory: these tests must not scatter hundreds of transient
    // files through the shared temp dir, where other test binaries are working.
    let dir = std::env::temp_dir().join(format!("hazard-outputs-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut worst = 0o777;
    for i in 0..400 {
        let out = dir.join(format!("output-{i}"));
        std::fs::write(&out, b"built").unwrap();
        worst = worst.min(std::fs::metadata(&out).unwrap().permissions().mode() & 0o777);
        std::fs::remove_file(&out).unwrap();
    }

    stop.store(true, Ordering::Relaxed);
    neighbour.join().unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    unsafe { libc::umask(entry) };
    assert_eq!(
        worst, 0o644,
        "a build output was created with {worst:#o} because a neighbouring \
         tenant held the umask at 0o77"
    );
}

/// `-C` is a real `chdir(2)` (`src/main.rs:1825`), and cwd is process-wide, so
/// a tenant that starts while another tenant holds the working directory used
/// to resolve its own workspace against the wrong root. A session is now
/// *given* its root instead of inheriting whatever directory the process
/// happens to be sitting in, so neither tenant depends on the process cwd at
/// all — note this test never `chdir`s to tenant A's root.
///
/// Note what made this hazard easy to miss before: an *already-warm* context
/// kept answering correctly, because `dir.rs` had cached the listing of `.`
/// back when `.` meant its own root. That was not isolation, it was a stale
/// cache keyed by a name whose meaning changed underneath it — right by
/// accident, and wrong as soon as the tenant asked about a file in the
/// directory it had actually moved to. Keying by the resolved path fixes both
/// halves at once.
#[test]
fn a_tenant_starting_up_sees_its_own_root_not_another_tenants() {
    let _guard = hazard_lock().lock().unwrap_or_else(|e| e.into_inner());
    let entry = std::env::current_dir().unwrap();

    let root = std::env::temp_dir().join(format!("hazard-cwd-{}", std::process::id()));
    let (a, b) = (root.join("tenant-a"), root.join("tenant-b"));
    std::fs::create_dir_all(&a).unwrap();
    std::fs::create_dir_all(&b).unwrap();
    std::fs::write(a.join("only-in-a.mk"), b"all:;@:").unwrap();

    let name = CString::new("only-in-a.mk").unwrap();

    // Tenant A is running, rooted at `a` — without the process ever going there.
    let tenant_a = ExecContext::default();
    *tenant_a.workspace_root.borrow_mut() = a.clone();
    let a_sees_it = unsafe { make_sys::dir::file_exists_p(&tenant_a, name.as_ptr()) };

    // Tenant B starts and applies its own `-C`, which moves the whole process.
    std::env::set_current_dir(&b).unwrap();

    // A second session for tenant A — a new build on the same workspace.
    let tenant_a_again = ExecContext::default();
    *tenant_a_again.workspace_root.borrow_mut() = a.clone();
    let still_sees_it = unsafe { make_sys::dir::file_exists_p(&tenant_a_again, name.as_ptr()) };

    std::env::set_current_dir(&entry).unwrap();
    let _ = std::fs::remove_dir_all(&root);

    assert_eq!(
        a_sees_it.unwrap(),
        1,
        "tenant A should see its own makefile"
    );
    assert_eq!(
        still_sees_it.unwrap(),
        1,
        "a new session for tenant A could not find its own makefile, because \
         tenant B's -C had moved the process working directory"
    );
}
