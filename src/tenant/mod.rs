//! Tenant-facing building blocks for Phase E (#598): a server-wide job
//! slot scheduler (#609) and a tenant control plane (#610).
//!
//! Both are sans-io: pure state machines with no fds, sockets, or async
//! runtime dependency. Neither is wired into `ExecContext` or a real
//! server yet -- that requires #607 (`ExecContext: Send`) and the async
//! runtime / `make-srv` crate (#600, #592), none of which have landed in
//! this repository yet. These modules exist so that integration is a
//! matter of plumbing once that foundation exists, rather than a design
//! exercise done under time pressure then.
//!
//! Decisions this code assumes (recorded on #598):
//! - **Q2 (trust boundary): cooperative tenants.** One org, many
//!   repos/branches. Isolation here is for correctness (no cross-talk),
//!   not for security against an adversarial tenant -- no sandboxing of
//!   job execution is in scope.
//! - **Q4 (resource policy, #609): mechanism only.** The slot pool and its
//!   acquire/release/preemption plumbing are decided now; the fairness
//!   *policy* (weights, priorities) is left pluggable via
//!   [`scheduler::FairnessStrategy`], with FIFO as the only strategy
//!   shipped today.
//! - **Q5 (control plane, #610): sans-io core, transport deferred.** The
//!   control plane is a pure command/event state machine
//!   ([`control_plane::ControlPlane`]); how it's exposed (websocket frames
//!   vs. a separate admin protocol) is a transport-adapter decision for
//!   later. Authentication is explicitly out of scope for this slice --
//!   [`control_plane::TenantIdentity`] is trusted input, not verified here.

pub mod control_plane;
pub mod scheduler;
