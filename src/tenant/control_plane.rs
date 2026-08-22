//! Tenant control plane (#610): create, attach, list and cancel a tenant,
//! with identity carried on every command.
//!
//! Sans-io by design (Q5): this is a pure command/event state machine with
//! no websocket frames, no HTTP, no networking at all. How it's exposed --
//! folded into #447's websocket protocol or served as a separate admin
//! API -- is a transport-adapter decision left for once `make-srv` (#592)
//! and the async runtime (#600) actually exist in this crate; either
//! transport can drive this same core by turning wire messages into
//! [`Command`]s and [`Event`]s back into wire messages.
//!
//! Authentication is explicitly deferred (Q5 follow-up): [`TenantIdentity`]
//! is trusted input here, not verified. A transport adapter is responsible
//! for authenticating a caller before constructing a [`Command`] on their
//! behalf.
//!
//! Isolation is for correctness, not security (Q2: cooperative tenants) --
//! this module does not defend against a malicious tenant impersonating
//! another; it only keeps well-behaved tenants from cross-talking.

use std::collections::HashMap;

use super::scheduler::TenantId;

/// A tenant's identity, carried on every command rather than inferred from
/// a connection (#610's "identity on every frame"). Trusted input: nothing
/// in this module verifies it belongs to the caller.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TenantIdentity {
    pub tenant: TenantId,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantState {
    Attached,
    Building,
    Cancelled,
}

#[derive(Debug, Clone)]
struct TenantInfo {
    identity: TenantIdentity,
    workspace: String,
    state: TenantState,
}

/// A snapshot of one tenant, as returned by [`Command::List`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantSummary {
    pub tenant: TenantId,
    pub label: String,
    pub state: TenantState,
}

/// Requests the control plane accepts.
#[derive(Debug, Clone)]
pub enum Command {
    Create {
        identity: TenantIdentity,
        workspace: String,
    },
    Attach {
        identity: TenantIdentity,
    },
    List,
    Cancel {
        identity: TenantIdentity,
    },
}

/// Why a command was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    /// `Create` for a tenant id that already exists.
    AlreadyExists,
    /// `Attach`/`Cancel` for a tenant id that was never created.
    UnknownTenant,
}

/// Outcomes the control plane produces. A transport adapter maps these
/// onto whatever wire format it speaks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Created(TenantIdentity),
    Attached(TenantIdentity),
    Listed(Vec<TenantSummary>),
    Cancelled(TenantIdentity),
    Rejected {
        identity: TenantIdentity,
        reason: RejectReason,
    },
}

/// The tenant control plane's state machine: create, attach, list and
/// cancel a tenant. Pure and synchronous -- [`Self::handle`] takes a
/// [`Command`] and returns the resulting [`Event`], with no I/O.
#[derive(Debug, Default)]
pub struct ControlPlane {
    tenants: HashMap<TenantId, TenantInfo>,
}

impl ControlPlane {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle(&mut self, command: Command) -> Event {
        match command {
            Command::Create {
                identity,
                workspace,
            } => self.create(identity, workspace),
            Command::Attach { identity } => self.attach(identity),
            Command::List => self.list(),
            Command::Cancel { identity } => self.cancel(identity),
        }
    }

    fn create(&mut self, identity: TenantIdentity, workspace: String) -> Event {
        if self.tenants.contains_key(&identity.tenant) {
            return Event::Rejected {
                identity,
                reason: RejectReason::AlreadyExists,
            };
        }
        self.tenants.insert(
            identity.tenant,
            TenantInfo {
                identity: identity.clone(),
                workspace,
                state: TenantState::Attached,
            },
        );
        Event::Created(identity)
    }

    fn attach(&mut self, identity: TenantIdentity) -> Event {
        if self.tenants.contains_key(&identity.tenant) {
            Event::Attached(identity)
        } else {
            Event::Rejected {
                identity,
                reason: RejectReason::UnknownTenant,
            }
        }
    }

    fn list(&self) -> Event {
        let mut tenants: Vec<TenantSummary> = self
            .tenants
            .values()
            .map(|info| TenantSummary {
                tenant: info.identity.tenant,
                label: info.identity.label.clone(),
                state: info.state,
            })
            .collect();
        tenants.sort_by_key(|t| t.tenant);
        Event::Listed(tenants)
    }

    /// The workspace root recorded for `tenant` at `Create`, if it exists.
    pub fn workspace(&self, tenant: TenantId) -> Option<&str> {
        self.tenants
            .get(&tenant)
            .map(|info| info.workspace.as_str())
    }

    fn cancel(&mut self, identity: TenantIdentity) -> Event {
        match self.tenants.get_mut(&identity.tenant) {
            Some(info) => {
                info.state = TenantState::Cancelled;
                Event::Cancelled(identity)
            }
            None => Event::Rejected {
                identity,
                reason: RejectReason::UnknownTenant,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(id: u64, label: &str) -> TenantIdentity {
        TenantIdentity {
            tenant: TenantId(id),
            label: label.to_string(),
        }
    }

    #[test]
    fn create_then_list_shows_the_tenant() {
        let mut plane = ControlPlane::new();
        let alice = identity(1, "alice");

        assert_eq!(
            plane.handle(Command::Create {
                identity: alice.clone(),
                workspace: "/ws/a".into()
            }),
            Event::Created(alice.clone())
        );

        assert_eq!(
            plane.handle(Command::List),
            Event::Listed(vec![TenantSummary {
                tenant: TenantId(1),
                label: "alice".into(),
                state: TenantState::Attached,
            }])
        );
    }

    #[test]
    fn duplicate_create_is_rejected() {
        let mut plane = ControlPlane::new();
        let alice = identity(1, "alice");
        plane.handle(Command::Create {
            identity: alice.clone(),
            workspace: "/ws/a".into(),
        });

        assert_eq!(
            plane.handle(Command::Create {
                identity: alice.clone(),
                workspace: "/ws/a2".into()
            }),
            Event::Rejected {
                identity: alice,
                reason: RejectReason::AlreadyExists
            }
        );
    }

    #[test]
    fn attach_unknown_tenant_is_rejected() {
        let mut plane = ControlPlane::new();
        let ghost = identity(404, "ghost");

        assert_eq!(
            plane.handle(Command::Attach {
                identity: ghost.clone()
            }),
            Event::Rejected {
                identity: ghost,
                reason: RejectReason::UnknownTenant
            }
        );
    }

    #[test]
    fn attach_known_tenant_succeeds() {
        let mut plane = ControlPlane::new();
        let alice = identity(1, "alice");
        plane.handle(Command::Create {
            identity: alice.clone(),
            workspace: "/ws/a".into(),
        });

        assert_eq!(
            plane.handle(Command::Attach {
                identity: alice.clone()
            }),
            Event::Attached(alice)
        );
    }

    #[test]
    fn cancel_unknown_tenant_is_rejected() {
        let mut plane = ControlPlane::new();
        let ghost = identity(404, "ghost");

        assert_eq!(
            plane.handle(Command::Cancel {
                identity: ghost.clone()
            }),
            Event::Rejected {
                identity: ghost,
                reason: RejectReason::UnknownTenant
            }
        );
    }

    #[test]
    fn cancel_reflects_in_list_state() {
        let mut plane = ControlPlane::new();
        let alice = identity(1, "alice");
        plane.handle(Command::Create {
            identity: alice.clone(),
            workspace: "/ws/a".into(),
        });

        assert_eq!(
            plane.handle(Command::Cancel {
                identity: alice.clone()
            }),
            Event::Cancelled(alice)
        );

        assert_eq!(
            plane.handle(Command::List),
            Event::Listed(vec![TenantSummary {
                tenant: TenantId(1),
                label: "alice".into(),
                state: TenantState::Cancelled,
            }])
        );
    }

    #[test]
    fn workspace_is_recorded_at_create() {
        let mut plane = ControlPlane::new();
        let alice = identity(1, "alice");
        plane.handle(Command::Create {
            identity: alice.clone(),
            workspace: "/ws/a".into(),
        });

        assert_eq!(plane.workspace(alice.tenant), Some("/ws/a"));
        assert_eq!(plane.workspace(TenantId(404)), None);
    }

    #[test]
    fn two_tenants_do_not_cross_talk() {
        let mut plane = ControlPlane::new();
        let alice = identity(1, "alice");
        let bob = identity(2, "bob");
        plane.handle(Command::Create {
            identity: alice.clone(),
            workspace: "/ws/a".into(),
        });
        plane.handle(Command::Create {
            identity: bob.clone(),
            workspace: "/ws/b".into(),
        });

        plane.handle(Command::Cancel {
            identity: alice.clone(),
        });

        let Event::Listed(tenants) = plane.handle(Command::List) else {
            panic!("expected Listed");
        };
        assert_eq!(tenants.len(), 2);
        assert_eq!(tenants[0].state, TenantState::Cancelled);
        assert_eq!(tenants[1].state, TenantState::Attached);
    }
}
