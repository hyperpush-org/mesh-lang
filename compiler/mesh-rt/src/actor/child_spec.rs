//! Child specification types for Mesh supervision.
//!
//! Defines the types used to configure supervised children: restart policy,
//! shutdown behavior, child type, and the full child specification struct.
//! These types are used by the supervisor runtime (`supervisor.rs`) to manage
//! child actor lifecycles.

use super::process::ProcessId;

// ---------------------------------------------------------------------------
// Strategy
// ---------------------------------------------------------------------------

/// Supervision restart strategy.
///
/// Determines which children are restarted when one child exits abnormally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    /// Restart only the failed child. Siblings are unaffected.
    OneForOne,
    /// Terminate and restart ALL children when any one fails.
    OneForAll,
    /// Terminate and restart the failed child and all children started after it.
    RestForOne,
    /// Like OneForOne but for dynamic children added via `start_child`.
    /// Uses a template child spec for all children.
    SimpleOneForOne,
}

// ---------------------------------------------------------------------------
// RestartType
// ---------------------------------------------------------------------------

/// When a child should be restarted after exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartType {
    /// Always restart, regardless of exit reason (even Normal/Shutdown).
    Permanent,
    /// Restart only on abnormal exit (Error, Killed, Custom, Linked).
    /// Normal and Shutdown exits are not restarted.
    Transient,
    /// Never restart. The child is removed from the supervisor on exit.
    Temporary,
}

// ---------------------------------------------------------------------------
// ShutdownType
// ---------------------------------------------------------------------------

/// How a child should be terminated during ordered shutdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownType {
    /// Immediately kill the child process without waiting.
    BrutalKill,
    /// Send a Shutdown exit signal and wait up to the specified number of
    /// milliseconds for the child to exit. If the child does not exit
    /// within the timeout, it is forcefully killed.
    Timeout(u64),
}

impl Default for ShutdownType {
    /// Default shutdown timeout: 5000 milliseconds (5 seconds).
    /// Matches the Erlang/OTP default for worker children.
    fn default() -> Self {
        ShutdownType::Timeout(5000)
    }
}

// ---------------------------------------------------------------------------
// ChildType
// ---------------------------------------------------------------------------

/// Whether a supervised child is a regular worker or a nested supervisor.
///
/// This distinction affects shutdown behavior: supervisor-type children
/// may be given longer (or infinite) shutdown timeouts to allow their
/// own children to shut down gracefully.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildType {
    /// A regular actor that performs application work.
    Worker,
    /// A nested supervisor managing its own children.
    Supervisor,
}

// ---------------------------------------------------------------------------
// ChildSpec
// ---------------------------------------------------------------------------

/// Specification for a child actor under supervision.
///
/// Contains all the information needed to start, monitor, and restart a child.
/// The `start_fn` and `start_args_ptr` are raw pointers to the compiled Mesh
/// function and its serialized arguments, respectively.
#[derive(Debug, Clone)]
pub struct ChildSpec {
    /// Unique identifier for this child within the supervisor.
    pub id: String,
    /// Function pointer to the spawn function (extern "C" fn(*const u8)).
    pub start_fn: *const u8,
    /// Pointer to serialized initial arguments for the start function.
    pub start_args_ptr: *const u8,
    /// Size of the arguments in bytes.
    pub start_args_size: u64,
    /// When this child should be restarted after exit.
    pub restart_type: RestartType,
    /// How this child should be terminated during shutdown.
    pub shutdown: ShutdownType,
    /// Whether this child is a worker or a nested supervisor.
    pub child_type: ChildType,
    /// Optional target node name for remote spawning (e.g., "worker@192.168.1.2:9000").
    /// When set, the supervisor spawns this child on the remote node via mesh_node_spawn.
    /// When None, the supervisor spawns locally (existing behavior unchanged).
    pub target_node: Option<String>,
    /// Function name for remote spawning (required when target_node is Some).
    /// Used by mesh_node_spawn to look up the function on the remote node.
    pub start_fn_name: Option<String>,
}

// Safety: ChildSpec's fn ptrs are owned by the runtime and valid for the
// supervisor's lifetime. The supervisor is the only entity that uses these
// pointers, and it runs on the scheduler's worker threads.
unsafe impl Send for ChildSpec {}
unsafe impl Sync for ChildSpec {}

// ---------------------------------------------------------------------------
// ChildState
// ---------------------------------------------------------------------------

/// Runtime state of a supervised child.
///
/// Combines the static specification with dynamic runtime information
/// (current PID and running status).
#[derive(Debug, Clone)]
pub struct ChildState {
    /// The child's specification (restart policy, shutdown type, etc.).
    pub spec: ChildSpec,
    /// Current PID of the running child, or None if not started/terminated.
    pub pid: Option<ProcessId>,
    /// Whether this child is currently running.
    pub running: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutdown_type_default() {
        let shutdown = ShutdownType::default();
        assert_eq!(shutdown, ShutdownType::Timeout(5000));
    }

    #[test]
    fn test_strategy_variants() {
        // Verify all strategy variants exist and are distinct.
        let strategies = [
            Strategy::OneForOne,
            Strategy::OneForAll,
            Strategy::RestForOne,
            Strategy::SimpleOneForOne,
        ];
        for (i, a) in strategies.iter().enumerate() {
            for (j, b) in strategies.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_restart_type_variants() {
        assert_ne!(RestartType::Permanent, RestartType::Transient);
        assert_ne!(RestartType::Transient, RestartType::Temporary);
        assert_ne!(RestartType::Permanent, RestartType::Temporary);
    }

    #[test]
    fn test_child_type_variants() {
        assert_ne!(ChildType::Worker, ChildType::Supervisor);
    }

    #[test]
    fn test_child_spec_creation() {
        let spec = ChildSpec {
            id: "worker1".to_string(),
            start_fn: std::ptr::null(),
            start_args_ptr: std::ptr::null(),
            start_args_size: 0,
            restart_type: RestartType::Permanent,
            shutdown: ShutdownType::default(),
            child_type: ChildType::Worker,
            target_node: None,
            start_fn_name: None,
        };
        assert_eq!(spec.id, "worker1");
        assert_eq!(spec.restart_type, RestartType::Permanent);
        assert_eq!(spec.shutdown, ShutdownType::Timeout(5000));
        assert_eq!(spec.child_type, ChildType::Worker);
    }

    #[test]
    fn test_child_state_creation() {
        let spec = ChildSpec {
            id: "worker1".to_string(),
            start_fn: std::ptr::null(),
            start_args_ptr: std::ptr::null(),
            start_args_size: 0,
            restart_type: RestartType::Transient,
            shutdown: ShutdownType::BrutalKill,
            child_type: ChildType::Worker,
            target_node: None,
            start_fn_name: None,
        };
        let state = ChildState {
            spec,
            pid: None,
            running: false,
        };
        assert!(state.pid.is_none());
        assert!(!state.running);
        assert_eq!(state.spec.restart_type, RestartType::Transient);
        assert_eq!(state.spec.shutdown, ShutdownType::BrutalKill);
    }
}
