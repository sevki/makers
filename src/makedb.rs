//! The session salsa database.
//!
//! One `#[salsa::db]` for the whole session, owned by
//! [`crate::execctx::ExecContext`] — the unification of the three
//! formerly module-local salsa islands (`strcache.rs`'s interner db,
//! `parser.rs`'s AST-node db, `depgraph.rs`'s graph-query db). Hosting the
//! string interner, the parser's interned AST nodes, and the dependency-graph
//! input in a single database gives them shared revisions, so parse→graph
//! derivation can become incremental end-to-end; owning it on the context
//! (instead of `OnceLock` statics) means two sessions in one process do not
//! share interner state.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// The unified session database. Counts query executions (salsa
/// `WillExecute` events) so memoization is observable — a cache hit answers
/// without a new execution.
#[salsa::db]
#[derive(Clone)]
pub struct MakeDb {
    storage: salsa::Storage<Self>,
    executions: Arc<AtomicU64>,
}

impl Default for MakeDb {
    fn default() -> Self {
        let executions = Arc::new(AtomicU64::new(0));
        let counter = Arc::clone(&executions);
        MakeDb {
            storage: salsa::Storage::new(Some(Box::new(move |event| {
                if matches!(event.kind, salsa::EventKind::WillExecute { .. }) {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            }))),
            executions,
        }
    }
}

// `salsa::Storage` carries no useful `Debug`; the context derives `Debug`,
// so give the database an opaque representation.
impl std::fmt::Debug for MakeDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MakeDb")
            .field("executions", &self.executions.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

#[salsa::db]
impl salsa::Database for MakeDb {}

impl MakeDb {
    /// How many tracked queries have actually executed (as opposed to being
    /// answered from cache) over this database's lifetime.
    pub fn executions(&self) -> u64 {
        self.executions.load(Ordering::Relaxed)
    }
}
