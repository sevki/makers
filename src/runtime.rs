//! Per-tenant execution slots.
//!
//! Phase E (#598) puts every tenant of the build server in *one process* —
//! there is no supervisor forking a `make` per client — and runs everything
//! on tokio. That leaves one question this module answers: what shape does a
//! tenant's task have?
//!
//! [`ExecContext`](crate::execctx::ExecContext) is neither `Send` nor `Sync`.
//! `!Sync` is by design (a tenant is single-owner; the context is full of
//! `Cell`/`RefCell`). `!Send` is incidental — the context still holds raw
//! `*mut c_char` fields, and a struct holding a raw pointer is `!Send` by
//! construction. `tokio::spawn` needs `Send + 'static`, so a tenant cannot be
//! an ordinary task on the multi-threaded runtime today.
//!
//! So a tenant is **pinned to a thread**: one OS thread, one current-thread
//! runtime, one context lineage that never crosses a thread boundary. Async
//! I/O — the control plane, worker sockets, the SIGCHLD reaper — works
//! exactly as it would on the shared runtime; the only thing given up is
//! work-stealing across tenants, which a tenant's own `-j` fan-out does not
//! need.
//!
//! This is deliberately the near-term shape, not the end state. When the raw
//! pointer fields retire and `ExecContext` becomes `Send`, tenants can move
//! to shared-runtime tasks without changing this API: callers hold a
//! [`TenantRuntime`] and await on it either way.

use std::future::Future;
use std::io;
use std::thread;

/// One tenant's execution slot: a current-thread tokio runtime that its
/// tenant — and nothing else — runs on.
///
/// The command-line `make` is the N=1 case: one tenant, one slot, the process
/// exiting when it finishes.
#[derive(Debug)]
pub struct TenantRuntime {
    runtime: tokio::runtime::Runtime,
}

impl TenantRuntime {
    /// Build a slot. Fails only if the OS refuses the runtime's resources.
    pub fn new() -> io::Result<Self> {
        Ok(TenantRuntime {
            runtime: tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?,
        })
    }

    /// Drive `future` to completion on this tenant's runtime.
    ///
    /// Takes `&self` rather than consuming the slot so a tenant can run
    /// several builds in sequence — the point of a long-lived session.
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }
}

/// Start a tenant on its own thread with its own [`TenantRuntime`], handing
/// the slot to `tenant`.
///
/// The closure — not the runtime — is what has to be `Send`: the slot is
/// built on the new thread and never leaves it, which is the whole point of
/// pinning.
pub fn spawn_tenant<F, T>(name: impl Into<String>, tenant: F) -> io::Result<thread::JoinHandle<T>>
where
    F: FnOnce(&TenantRuntime) -> T + Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new().name(name.into()).spawn(move || {
        let slot = TenantRuntime::new().expect("tenant runtime");
        tenant(&slot)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn block_on_runs_to_completion() {
        let slot = TenantRuntime::new().expect("slot");
        assert_eq!(slot.block_on(async { 6 * 7 }), 42);
    }

    #[test]
    fn one_slot_serves_repeated_builds() {
        let slot = TenantRuntime::new().expect("slot");
        let total: usize = (0..4).map(|n| slot.block_on(async move { n })).sum();
        assert_eq!(total, 6);
    }

    /// The phase acceptance in miniature: two tenants making progress
    /// concurrently in one process, each on its own thread and runtime.
    #[test]
    fn tenants_run_concurrently_and_independently() {
        let done = Arc::new(AtomicUsize::new(0));
        let handles: Vec<_> = (0..2)
            .map(|id| {
                let done = Arc::clone(&done);
                spawn_tenant(format!("tenant-{id}"), move |slot| {
                    slot.block_on(async move {
                        done.fetch_add(1, Ordering::SeqCst);
                        (id, thread::current().id())
                    })
                })
                .expect("spawn tenant")
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(done.load(Ordering::SeqCst), 2);
        assert_eq!(
            results.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
            [0, 1]
        );
        // Each tenant ran on its own thread — no slot was shared.
        assert_ne!(results[0].1, results[1].1);
    }
}
