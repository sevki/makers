//! Server-wide job slot scheduler with per-tenant shares (#609).
//!
//! Job slots are POSIX jobserver fds today, scoped to one process (blocker
//! 8 on #598): N tenants each honouring their own `-j` oversubscribes the
//! host by construction. This module is the mechanism half of the fix --
//! one pool sized to the host, tenants acquire/release slots against it,
//! and a waiting tenant is admitted fairly as slots free up. It does not
//! yet decide *how* fair (Q4 is scoped to the mechanism only): the only
//! [`FairnessStrategy`] shipped is FIFO, which reproduces today's
//! single-tenant semantics when there is exactly one tenant.
//!
//! Sans-io: this pool does not touch the jobserver pipe/fds itself. A
//! caller integrating this into `src/job.rs` translates a granted
//! [`TenantId`] into the existing POSIX jobserver token protocol at the
//! FFI boundary.

use std::collections::VecDeque;

/// Identifies a tenant across the scheduler and control plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TenantId(pub u64);

/// Decides which waiting tenant to admit next when a slot frees up, and
/// (optionally) which held tenants to preempt to make room for one.
///
/// The default `preempt` never preempts -- an empty result means "no
/// involuntary release", which is the only policy #609 needs today. A
/// future policy slice can implement weighted or priority-based selection
/// without touching [`SlotPool`] itself.
pub trait FairnessStrategy {
    /// Index into `waiting` of the tenant to admit next, if any.
    fn select_next(&mut self, waiting: &VecDeque<TenantId>) -> Option<usize>;

    /// Choose tenants to preempt so that `needed` additional slots become
    /// available. `held` is `(tenant, slots_held)` for every tenant
    /// currently holding at least one slot.
    fn preempt(&mut self, held: &[(TenantId, usize)], needed: usize) -> Vec<TenantId> {
        let _ = (held, needed);
        Vec::new()
    }
}

/// First-come, first-served. The only strategy #609 ships: with a single
/// tenant it behaves exactly like today's single-process `-j` pool.
#[derive(Debug, Default)]
pub struct FifoStrategy;

impl FairnessStrategy for FifoStrategy {
    fn select_next(&mut self, waiting: &VecDeque<TenantId>) -> Option<usize> {
        if waiting.is_empty() {
            None
        } else {
            Some(0)
        }
    }
}

/// One server-wide pool of job slots, shared across tenants.
///
/// Invariant: total slots held across all tenants never exceeds
/// `capacity`. A tenant that cannot be granted a slot immediately is
/// queued and admitted by `strategy` as capacity frees up, so a starved
/// tenant still makes progress rather than being starved forever (subject
/// to the strategy's fairness, which FIFO guarantees for a bounded number
/// of competitors).
pub struct SlotPool<S: FairnessStrategy = FifoStrategy> {
    capacity: usize,
    held: Vec<(TenantId, usize)>,
    waiting: VecDeque<TenantId>,
    strategy: S,
}

impl SlotPool<FifoStrategy> {
    /// A pool of `capacity` slots using the default FIFO strategy.
    pub fn new(capacity: usize) -> Self {
        Self::with_strategy(capacity, FifoStrategy)
    }
}

impl<S: FairnessStrategy> SlotPool<S> {
    pub fn with_strategy(capacity: usize, strategy: S) -> Self {
        assert!(capacity > 0, "a slot pool needs at least one slot");
        Self {
            capacity,
            held: Vec::new(),
            waiting: VecDeque::new(),
            strategy,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn in_use(&self) -> usize {
        self.held.iter().map(|(_, n)| n).sum()
    }

    /// Request one slot for `tenant`. Returns `true` if granted
    /// immediately; otherwise `tenant` is queued and will be granted by a
    /// later `release`/`poll` once room and the strategy allow it.
    pub fn acquire(&mut self, tenant: TenantId) -> bool {
        if self.in_use() < self.capacity {
            self.grant(tenant);
            true
        } else {
            self.waiting.push_back(tenant);
            false
        }
    }

    fn grant(&mut self, tenant: TenantId) {
        match self.held.iter_mut().find(|(t, _)| *t == tenant) {
            Some(entry) => entry.1 += 1,
            None => self.held.push((tenant, 1)),
        }
    }

    /// Release one slot held by `tenant`, then admit waiting tenants into
    /// whatever room that freed up. Returns the tenants granted a slot as
    /// a result, in grant order.
    pub fn release(&mut self, tenant: TenantId) -> Vec<TenantId> {
        if let Some(idx) = self.held.iter().position(|(t, _)| *t == tenant) {
            self.held[idx].1 -= 1;
            if self.held[idx].1 == 0 {
                self.held.remove(idx);
            }
        }
        self.poll()
    }

    /// Admit as many waiting tenants as free capacity and the strategy
    /// allow. Called automatically by `release`; exposed so a caller can
    /// re-poll after e.g. a capacity change.
    pub fn poll(&mut self) -> Vec<TenantId> {
        let mut granted = Vec::new();
        while self.in_use() < self.capacity {
            match self.strategy.select_next(&self.waiting) {
                Some(idx) if idx < self.waiting.len() => {
                    let tenant = self.waiting.remove(idx).expect("idx checked above");
                    self.grant(tenant);
                    granted.push(tenant);
                }
                _ => break,
            }
        }
        granted
    }

    /// Ask the strategy to free `needed` slots by preempting held tenants.
    /// Releases exactly the tenants the strategy chooses (all their held
    /// slots); with the default `FifoStrategy` this is always empty.
    pub fn request_preemption(&mut self, needed: usize) -> Vec<TenantId> {
        let victims = self.strategy.preempt(&self.held, needed);
        for victim in &victims {
            self.held.retain(|(t, _)| t != victim);
        }
        victims
    }

    /// Slots currently held by `tenant`.
    pub fn held_by(&self, tenant: TenantId) -> usize {
        self.held
            .iter()
            .find(|(t, _)| *t == tenant)
            .map(|(_, n)| *n)
            .unwrap_or(0)
    }

    /// Tenants currently queued, in wait order.
    pub fn waiting(&self) -> impl Iterator<Item = &TenantId> {
        self.waiting.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_exceeds_capacity_across_tenants() {
        let mut pool = SlotPool::new(2);
        let a = TenantId(1);
        let b = TenantId(2);
        let c = TenantId(3);

        assert!(pool.acquire(a));
        assert!(pool.acquire(b));
        // Pool is full: c queues rather than oversubscribing the host.
        assert!(!pool.acquire(c));
        assert_eq!(pool.held_by(a) + pool.held_by(b) + pool.held_by(c), 2);
    }

    #[test]
    fn starved_tenant_still_makes_progress() {
        let mut pool = SlotPool::new(1);
        let a = TenantId(1);
        let b = TenantId(2);

        assert!(pool.acquire(a));
        assert!(!pool.acquire(b));
        assert_eq!(pool.waiting().copied().collect::<Vec<_>>(), vec![b]);

        let granted = pool.release(a);
        assert_eq!(granted, vec![b]);
        assert_eq!(pool.held_by(b), 1);
        assert!(pool.waiting().next().is_none());
    }

    #[test]
    fn single_tenant_never_queues_within_capacity() {
        // Q4: with one strategy/tenant, the pool degrades to today's
        // single-tenant `-j N` semantics -- every acquire up to capacity
        // succeeds immediately.
        let mut pool = SlotPool::new(4);
        let solo = TenantId(1);
        for _ in 0..4 {
            assert!(pool.acquire(solo));
        }
        assert!(!pool.acquire(solo));
        assert_eq!(pool.held_by(solo), 4);
    }

    #[test]
    fn fifo_admits_in_wait_order() {
        let mut pool = SlotPool::new(1);
        let a = TenantId(1);
        let b = TenantId(2);
        let c = TenantId(3);

        pool.acquire(a);
        pool.acquire(b);
        pool.acquire(c);

        assert_eq!(pool.release(a), vec![b]);
        assert_eq!(pool.release(b), vec![c]);
        assert_eq!(pool.release(c), Vec::<TenantId>::new());
    }

    #[test]
    fn default_strategy_never_preempts() {
        let mut pool = SlotPool::new(1);
        pool.acquire(TenantId(1));
        assert_eq!(pool.request_preemption(1), Vec::<TenantId>::new());
        // The held slot is untouched by a no-op preemption.
        assert_eq!(pool.held_by(TenantId(1)), 1);
    }
}
