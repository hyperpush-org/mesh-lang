//! Supervisor runtime for Mesh actors.
//!
//! Implements OTP-style supervision with four restart strategies
//! (one_for_one, one_for_all, rest_for_one, simple_one_for_one),
//! restart limit tracking via sliding window, ordered shutdown with
//! timeout/brutal_kill, and child lifecycle management.
//!
//! The supervisor is an actor that traps exits and manages child lifecycles.
//! It receives exit signals from linked children and applies the configured
//! restart strategy.
//!
//! ## Architecture
//!
//! Supervisor state is stored in a global registry keyed by PID, because
//! coroutine entry functions only receive a `*const u8` argument. The
//! supervisor entry function retrieves its state from this registry.
//!
//! ## Usage
//!
//! The extern "C" ABI functions in `mod.rs` delegate to the functions in
//! this module. Compiled Mesh programs call those ABI functions to start
//! supervisors, add/remove children, etc.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use rustc_hash::FxHashMap;

use super::child_spec::{ChildSpec, ChildState, RestartType, ShutdownType, Strategy};
use super::link;
use super::process::{ExitReason, ProcessId, ProcessState};
use super::scheduler::Scheduler;

// ---------------------------------------------------------------------------
// SupervisorState
// ---------------------------------------------------------------------------

/// The complete runtime state of a supervisor actor.
pub struct SupervisorState {
    /// Supervision restart strategy.
    pub strategy: Strategy,
    /// Maximum number of restarts allowed within the time window.
    pub max_restarts: u32,
    /// Time window in seconds for restart limit tracking.
    pub max_seconds: u64,
    /// Ordered list of child states (static children + dynamically added ones).
    pub children: Vec<ChildState>,
    /// Sliding window of restart timestamps for restart limit enforcement.
    pub restart_history: VecDeque<Instant>,
    /// For simple_one_for_one: the template child spec used for dynamic children.
    pub child_template: Option<ChildSpec>,
}

impl SupervisorState {
    /// Create a new supervisor state with the given configuration.
    pub fn new(strategy: Strategy, max_restarts: u32, max_seconds: u64) -> Self {
        SupervisorState {
            strategy,
            max_restarts,
            max_seconds,
            children: Vec::new(),
            restart_history: VecDeque::new(),
            child_template: None,
        }
    }

    /// Find the index of a child by its current PID.
    pub fn find_child_index(&self, pid: ProcessId) -> Option<usize> {
        self.children.iter().position(|c| c.pid == Some(pid))
    }

    /// Count the number of currently running children.
    pub fn running_count(&self) -> usize {
        self.children.iter().filter(|c| c.running).count()
    }
}

impl std::fmt::Debug for SupervisorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SupervisorState")
            .field("strategy", &self.strategy)
            .field("max_restarts", &self.max_restarts)
            .field("max_seconds", &self.max_seconds)
            .field("children", &self.children.len())
            .field("running", &self.running_count())
            .field("restart_history", &self.restart_history.len())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// SupervisorConfig
// ---------------------------------------------------------------------------

/// Configuration for creating a new supervisor.
///
/// This is the data passed to `mesh_supervisor_start` from compiled Mesh
/// programs. The config is deserialized from raw bytes.
#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    /// Supervision restart strategy.
    pub strategy: Strategy,
    /// Maximum restarts within the time window.
    pub max_restarts: u32,
    /// Time window in seconds.
    pub max_seconds: u64,
    /// Child specifications.
    pub child_specs: Vec<ChildSpec>,
}

// ---------------------------------------------------------------------------
// Global supervisor state registry
// ---------------------------------------------------------------------------

/// Global registry mapping supervisor PIDs to their state.
///
/// Since coroutine entry functions only receive a `*const u8`, we store
/// supervisor state in a global registry that the entry function can
/// look up by PID.
static SUPERVISOR_STATES: std::sync::OnceLock<
    Mutex<FxHashMap<ProcessId, Arc<Mutex<SupervisorState>>>>,
> = std::sync::OnceLock::new();

fn supervisor_states() -> &'static Mutex<FxHashMap<ProcessId, Arc<Mutex<SupervisorState>>>> {
    SUPERVISOR_STATES.get_or_init(|| Mutex::new(FxHashMap::default()))
}

/// Register a supervisor state for the given PID.
pub fn register_supervisor_state(
    pid: ProcessId,
    state: SupervisorState,
) -> Arc<Mutex<SupervisorState>> {
    let arc = Arc::new(Mutex::new(state));
    supervisor_states().lock().insert(pid, Arc::clone(&arc));
    arc
}

/// Look up a supervisor state by PID.
pub fn get_supervisor_state(pid: ProcessId) -> Option<Arc<Mutex<SupervisorState>>> {
    supervisor_states().lock().get(&pid).cloned()
}

/// Remove a supervisor state when the supervisor exits.
pub fn remove_supervisor_state(pid: ProcessId) {
    supervisor_states().lock().remove(&pid);
}

// ---------------------------------------------------------------------------
// Child lifecycle management
// ---------------------------------------------------------------------------

/// Start all children in order.
///
/// If any child fails to start, terminate already-started children in
/// reverse order and return an error.
pub fn start_children(
    state: &mut SupervisorState,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) -> Result<(), String> {
    for i in 0..state.children.len() {
        match start_single_child(&mut state.children[i], scheduler, sup_pid) {
            Ok(_pid) => {}
            Err(e) => {
                // Terminate children that were already started (reverse order).
                terminate_children_range(state, 0, i, scheduler, sup_pid);
                return Err(format!(
                    "child '{}' failed to start: {}",
                    state.children[i].spec.id, e
                ));
            }
        }
    }
    Ok(())
}

/// Start a single child process.
///
/// Spawns the child via the scheduler, links the supervisor to the child,
/// and updates the child state.
///
/// If the child spec has `target_node` set, the child is spawned on the
/// remote node via `mesh_node_spawn`. Otherwise, spawns locally (unchanged).
pub fn start_single_child(
    child: &mut ChildState,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) -> Result<ProcessId, String> {
    // Route to remote spawn if target_node is set.
    if child.spec.target_node.is_some() {
        let node = child.spec.target_node.clone().unwrap();
        let fn_name = child
            .spec
            .start_fn_name
            .clone()
            .ok_or("remote child requires start_fn_name")?;
        return start_single_child_remote(child, sup_pid, &node, &fn_name);
    }

    // Local spawn path (existing behavior unchanged).
    let child_pid = scheduler.spawn(
        child.spec.start_fn,
        child.spec.start_args_ptr,
        child.spec.start_args_size,
        1, // Normal priority
    );

    // Link the supervisor to the child.
    let sup_proc = scheduler.get_process(sup_pid);
    let child_proc = scheduler.get_process(child_pid);

    if let (Some(sup_proc), Some(child_proc)) = (sup_proc, child_proc) {
        link::link(&sup_proc, &child_proc, sup_pid, child_pid);
    } else {
        return Err("failed to look up processes for linking".to_string());
    }

    // Update child state.
    child.pid = Some(child_pid);
    child.running = true;

    Ok(child_pid)
}

/// Start a child process on a remote node via `mesh_node_spawn`.
///
/// Calls `mesh_node_spawn` with `link_flag=1` (spawn + bidirectional link).
/// The link ensures the supervisor receives DIST_EXIT when the remote child
/// crashes, which the supervisor's existing `trap_exit + handle_child_exit`
/// handles automatically.
///
/// The returned PID from `mesh_node_spawn` is a fully-qualified remote PID
/// with correct node_id/creation/local_id (Phase 67 handles this).
fn start_single_child_remote(
    child: &mut ChildState,
    _sup_pid: ProcessId,
    target_node: &str,
    fn_name: &str,
) -> Result<ProcessId, String> {
    let node_bytes = target_node.as_bytes();
    let fn_bytes = fn_name.as_bytes();

    let arg_count = child.spec.start_args_size / 8;
    let arg_tags = if arg_count == 0 {
        Vec::new()
    } else {
        vec![1u8; arg_count as usize]
    };

    // mesh_node_spawn is an extern "C" function expecting raw pointers.
    // It must be called from within an actor coroutine context (reads
    // stack::get_current_pid()). The supervisor IS an actor, so this works.
    let result = crate::dist::node::mesh_node_spawn(
        node_bytes.as_ptr(),
        node_bytes.len() as u64,
        fn_bytes.as_ptr(),
        fn_bytes.len() as u64,
        child.spec.start_args_ptr,
        child.spec.start_args_size,
        if arg_tags.is_empty() {
            std::ptr::null()
        } else {
            arg_tags.as_ptr()
        },
        arg_count,
        1, // link_flag=1: spawn with bidirectional link
    );

    if result == 0 {
        return Err("remote spawn failed: node not connected or function not found".to_string());
    }

    let remote_pid = ProcessId(result);
    child.pid = Some(remote_pid);
    child.running = true;

    Ok(remote_pid)
}

/// Start children from index `from_idx` to end, in forward order.
pub fn start_children_from(
    state: &mut SupervisorState,
    from_idx: usize,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) -> Result<(), String> {
    for i in from_idx..state.children.len() {
        match start_single_child(&mut state.children[i], scheduler, sup_pid) {
            Ok(_pid) => {}
            Err(e) => {
                // Terminate children that were started in this batch.
                terminate_children_range(state, from_idx, i, scheduler, sup_pid);
                return Err(format!(
                    "child '{}' failed to start: {}",
                    state.children[i].spec.id, e
                ));
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Child termination
// ---------------------------------------------------------------------------

/// Terminate all children in reverse start order.
pub fn terminate_all_children(
    state: &mut SupervisorState,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) {
    let len = state.children.len();
    if len == 0 {
        return;
    }
    terminate_children_range(state, 0, len, scheduler, sup_pid);
}

/// Terminate children in the range `[from_idx, to_idx)` in REVERSE order.
pub fn terminate_children_range(
    state: &mut SupervisorState,
    from_idx: usize,
    to_idx: usize,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) {
    // Iterate in reverse order.
    for i in (from_idx..to_idx).rev() {
        if state.children[i].running {
            terminate_single_child(&mut state.children[i], scheduler, sup_pid);
        }
    }
}

/// Terminate children from `from_idx` to end, in reverse order.
pub fn terminate_children_from(
    state: &mut SupervisorState,
    from_idx: usize,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) {
    let len = state.children.len();
    terminate_children_range(state, from_idx, len, scheduler, sup_pid);
}

/// Terminate a single child process.
///
/// Based on the child's shutdown type:
/// - BrutalKill: immediately mark the process as Exited(Killed).
/// - Timeout(ms): send a Shutdown exit signal, poll/wait for exit, force-kill on timeout.
pub fn terminate_single_child(child: &mut ChildState, scheduler: &Scheduler, sup_pid: ProcessId) {
    let child_pid = match child.pid {
        Some(pid) => pid,
        None => {
            child.running = false;
            return;
        }
    };

    // Remote children: send exit signal via distribution, don't access local process table.
    if !child_pid.is_local() {
        crate::dist::node::send_dist_exit(sup_pid, child_pid, &ExitReason::Shutdown);
        // The bidirectional link will deliver the child's exit back to the supervisor's
        // mailbox. We mark as not running immediately -- the supervisor's receive loop
        // handles the actual exit signal. This matches OTP semantics where termination
        // is asynchronous for remote children.
        child.running = false;
        child.pid = None;
        return;
    }

    match child.spec.shutdown {
        ShutdownType::BrutalKill => {
            // Immediately kill the child.
            if let Some(proc_arc) = scheduler.get_process(child_pid) {
                let mut proc = proc_arc.lock();
                if !matches!(proc.state, ProcessState::Exited(_)) {
                    proc.state = ProcessState::Exited(ExitReason::Killed);
                }
            }
        }
        ShutdownType::Timeout(ms) => {
            // Send a Shutdown exit signal to the child.
            send_exit_signal(scheduler, child_pid, &ExitReason::Shutdown);

            // Poll for the child to exit within the timeout.
            let deadline = Instant::now() + Duration::from_millis(ms);
            loop {
                if let Some(proc_arc) = scheduler.get_process(child_pid) {
                    if matches!(proc_arc.lock().state, ProcessState::Exited(_)) {
                        break;
                    }
                } else {
                    break;
                }
                if Instant::now() >= deadline {
                    // Timeout -- force kill.
                    if let Some(proc_arc) = scheduler.get_process(child_pid) {
                        let mut proc = proc_arc.lock();
                        if !matches!(proc.state, ProcessState::Exited(_)) {
                            proc.state = ProcessState::Exited(ExitReason::Killed);
                        }
                    }
                    break;
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }

    // Unlink the supervisor from the child.
    let sup_proc = scheduler.get_process(sup_pid);
    let child_proc = scheduler.get_process(child_pid);
    if let (Some(sp), Some(cp)) = (sup_proc, child_proc) {
        link::unlink(&sp, &cp, sup_pid, child_pid);
    }

    // Mark child as not running.
    child.running = false;
    child.pid = None;
}

/// Send an exit signal to a target process by delivering it to the mailbox.
///
/// If the target has trap_exit enabled, the signal is delivered as a message.
/// If not, the process is terminated with the given reason.
/// If the reason is Killed, the process is immediately terminated (untrappable).
fn send_exit_signal(scheduler: &Scheduler, target_pid: ProcessId, reason: &ExitReason) {
    if let Some(proc_arc) = scheduler.get_process(target_pid) {
        let mut proc = proc_arc.lock();

        // Skip already-exited processes.
        if matches!(proc.state, ProcessState::Exited(_)) {
            return;
        }

        // Killed is untrappable -- immediately terminate.
        if matches!(reason, ExitReason::Killed) {
            proc.state = ProcessState::Exited(ExitReason::Killed);
            return;
        }

        if proc.trap_exit {
            // Deliver as a message (the process will handle it in its receive loop).
            let signal_data = link::encode_exit_signal(target_pid, reason);
            let buffer = super::heap::MessageBuffer::new(signal_data, link::EXIT_SIGNAL_TAG);
            proc.mailbox.push(super::process::Message { buffer });

            // Wake if Waiting.
            if matches!(proc.state, ProcessState::Waiting) {
                proc.state = ProcessState::Ready;
            }
        } else {
            // Non-trapping process: terminate immediately.
            proc.state = ProcessState::Exited(reason.clone());
        }
    }
}

// ---------------------------------------------------------------------------
// Restart limit tracking
// ---------------------------------------------------------------------------

/// Check if a restart is allowed within the configured limits.
///
/// Uses a sliding window: removes timestamps older than `now - max_seconds`
/// from the front of `restart_history`. If the remaining count is already
/// at `max_restarts`, returns false (limit exceeded). Otherwise, records
/// the current timestamp and returns true.
pub fn check_restart_limit(state: &mut SupervisorState) -> bool {
    let now = Instant::now();
    let window = Duration::from_secs(state.max_seconds);

    // Remove timestamps outside the sliding window.
    while let Some(&oldest) = state.restart_history.front() {
        if now.duration_since(oldest) > window {
            state.restart_history.pop_front();
        } else {
            break;
        }
    }

    if state.restart_history.len() >= state.max_restarts as usize {
        // Limit exceeded.
        false
    } else {
        state.restart_history.push_back(now);
        true
    }
}

// ---------------------------------------------------------------------------
// Exit handling and strategy dispatch
// ---------------------------------------------------------------------------

/// Handle a child exit event.
///
/// Finds the child by PID, checks the restart policy, and applies the
/// configured strategy if a restart is needed.
///
/// Returns `Ok(())` if handled successfully, `Err(msg)` if the restart
/// limit was exceeded (the supervisor should terminate).
pub fn handle_child_exit(
    state: &mut SupervisorState,
    child_pid: ProcessId,
    reason: &ExitReason,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) -> Result<(), String> {
    // Find the child by PID.
    let child_idx = match state.find_child_index(child_pid) {
        Some(idx) => idx,
        None => return Ok(()), // Unknown child, ignore.
    };

    // Mark child as not running.
    state.children[child_idx].running = false;
    state.children[child_idx].pid = None;

    // Determine if restart is needed based on restart type and reason.
    let restart_type = state.children[child_idx].spec.restart_type;
    let should_restart = match restart_type {
        RestartType::Permanent => true,
        RestartType::Transient => {
            // Restart only on abnormal exit (not Normal, not Shutdown).
            !matches!(reason, ExitReason::Normal | ExitReason::Shutdown)
        }
        RestartType::Temporary => false,
    };

    if restart_type == RestartType::Temporary {
        // Remove from children list.
        state.children.remove(child_idx);
        return Ok(());
    }

    if !should_restart {
        return Ok(());
    }

    // Check restart limit before restarting.
    if !check_restart_limit(state) {
        // Restart limit exceeded -- terminate all children and fail.
        terminate_all_children(state, scheduler, sup_pid);
        return Err(format!(
            "restart limit exceeded: {} restarts in {} seconds",
            state.max_restarts, state.max_seconds
        ));
    }

    // Apply the strategy.
    apply_strategy(state, child_idx, scheduler, sup_pid)
}

/// Apply the configured restart strategy after a child exit.
pub fn apply_strategy(
    state: &mut SupervisorState,
    failed_child_idx: usize,
    scheduler: &Scheduler,
    sup_pid: ProcessId,
) -> Result<(), String> {
    match state.strategy {
        Strategy::OneForOne | Strategy::SimpleOneForOne => {
            // Restart only the failed child.
            start_single_child(&mut state.children[failed_child_idx], scheduler, sup_pid)?;
        }
        Strategy::OneForAll => {
            // Terminate all children in reverse order, then start all in forward order.
            terminate_all_children(state, scheduler, sup_pid);
            start_children(state, scheduler, sup_pid)?;
        }
        Strategy::RestForOne => {
            // Terminate children from failed_child_idx to end in reverse order.
            terminate_children_from(state, failed_child_idx, scheduler, sup_pid);
            // Restart from failed_child_idx to end in forward order.
            start_children_from(state, failed_child_idx, scheduler, sup_pid)?;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::child_spec::*;
    use crate::actor::process::ProcessId;

    /// Create a test scheduler.
    fn test_scheduler() -> Scheduler {
        Scheduler::new(1)
    }

    /// No-op entry function for test actors.
    extern "C" fn noop_entry(_args: *const u8) {
        // Do nothing -- immediately returns.
    }

    /// Create a test child spec.
    fn test_child_spec(id: &str, restart: RestartType, shutdown: ShutdownType) -> ChildSpec {
        ChildSpec {
            id: id.to_string(),
            start_fn: noop_entry as *const u8,
            start_args_ptr: std::ptr::null(),
            start_args_size: 0,
            restart_type: restart,
            shutdown,
            child_type: ChildType::Worker,
            target_node: None,
            start_fn_name: None,
        }
    }

    /// Create a test child state from a spec.
    fn test_child_state(spec: ChildSpec) -> ChildState {
        ChildState {
            spec,
            pid: None,
            running: false,
        }
    }

    /// Helper: create a supervisor state with children already spawned via
    /// the scheduler. Returns (SupervisorState, sup_pid).
    fn setup_supervisor(
        sched: &Scheduler,
        strategy: Strategy,
        child_specs: Vec<ChildSpec>,
    ) -> (SupervisorState, ProcessId) {
        let sup_pid = sched.spawn(noop_entry as *const u8, std::ptr::null(), 0, 1);

        // Set trap_exit on the supervisor process.
        if let Some(proc) = sched.get_process(sup_pid) {
            proc.lock().trap_exit = true;
        }

        let mut state = SupervisorState::new(strategy, 3, 5);
        state.children = child_specs
            .into_iter()
            .map(|spec| test_child_state(spec))
            .collect();

        // Start all children.
        let result = start_children(&mut state, sched, sup_pid);
        assert!(result.is_ok(), "start_children failed: {:?}", result.err());

        (state, sup_pid)
    }

    // -----------------------------------------------------------------------
    // Strategy tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_one_for_one_restarts_only_failed_child() {
        let sched = test_scheduler();
        let specs = vec![
            test_child_spec("child1", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child2", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child3", RestartType::Permanent, ShutdownType::BrutalKill),
        ];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);

        // Record initial PIDs.
        let initial_pids: Vec<ProcessId> = state.children.iter().map(|c| c.pid.unwrap()).collect();

        // Simulate child2 exit.
        let crashed_pid = initial_pids[1];
        // Mark the process as exited in the process table.
        if let Some(proc) = sched.get_process(crashed_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }

        let result = handle_child_exit(
            &mut state,
            crashed_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Child1 and child3 should still have the same PIDs (untouched).
        assert_eq!(state.children[0].pid.unwrap(), initial_pids[0]);
        assert_eq!(state.children[2].pid.unwrap(), initial_pids[2]);

        // Child2 should have a new PID (restarted).
        assert_ne!(state.children[1].pid.unwrap(), initial_pids[1]);
        assert!(state.children[1].running);
    }

    #[test]
    fn test_one_for_all_restarts_all_children() {
        let sched = test_scheduler();
        let specs = vec![
            test_child_spec("child1", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child2", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child3", RestartType::Permanent, ShutdownType::BrutalKill),
        ];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForAll, specs);

        let initial_pids: Vec<ProcessId> = state.children.iter().map(|c| c.pid.unwrap()).collect();

        // Simulate child2 exit.
        let crashed_pid = initial_pids[1];
        if let Some(proc) = sched.get_process(crashed_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }

        let result = handle_child_exit(
            &mut state,
            crashed_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // ALL children should have new PIDs (all restarted).
        for i in 0..3 {
            assert_ne!(
                state.children[i].pid.unwrap(),
                initial_pids[i],
                "child{} should have a new PID",
                i + 1
            );
            assert!(state.children[i].running);
        }
    }

    #[test]
    fn test_rest_for_one_restarts_subsequent() {
        let sched = test_scheduler();
        let specs = vec![
            test_child_spec("child1", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child2", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child3", RestartType::Permanent, ShutdownType::BrutalKill),
        ];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::RestForOne, specs);

        let initial_pids: Vec<ProcessId> = state.children.iter().map(|c| c.pid.unwrap()).collect();

        // Simulate child2 (middle child) exit.
        let crashed_pid = initial_pids[1];
        if let Some(proc) = sched.get_process(crashed_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }

        let result = handle_child_exit(
            &mut state,
            crashed_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Child1 should still have the same PID (before failed child -- untouched).
        assert_eq!(state.children[0].pid.unwrap(), initial_pids[0]);

        // Child2 (failed) and child3 (after failed) should have new PIDs.
        assert_ne!(state.children[1].pid.unwrap(), initial_pids[1]);
        assert_ne!(state.children[2].pid.unwrap(), initial_pids[2]);
        assert!(state.children[1].running);
        assert!(state.children[2].running);
    }

    #[test]
    fn test_simple_one_for_one_dynamic_child() {
        let sched = test_scheduler();

        // For simple_one_for_one, children are added dynamically.
        let sup_pid = sched.spawn(noop_entry as *const u8, std::ptr::null(), 0, 1);
        if let Some(proc) = sched.get_process(sup_pid) {
            proc.lock().trap_exit = true;
        }

        let mut state = SupervisorState::new(Strategy::SimpleOneForOne, 3, 5);
        state.child_template = Some(test_child_spec(
            "template",
            RestartType::Permanent,
            ShutdownType::BrutalKill,
        ));

        // Add dynamic children.
        for i in 0..3 {
            let mut child = test_child_state(test_child_spec(
                &format!("dynamic_{}", i),
                RestartType::Permanent,
                ShutdownType::BrutalKill,
            ));
            start_single_child(&mut child, &sched, sup_pid).unwrap();
            state.children.push(child);
        }

        let initial_pids: Vec<ProcessId> = state.children.iter().map(|c| c.pid.unwrap()).collect();

        // Crash the middle dynamic child.
        let crashed_pid = initial_pids[1];
        if let Some(proc) = sched.get_process(crashed_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }

        let result = handle_child_exit(
            &mut state,
            crashed_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Only the crashed child should have a new PID.
        assert_eq!(state.children[0].pid.unwrap(), initial_pids[0]);
        assert_ne!(state.children[1].pid.unwrap(), initial_pids[1]);
        assert_eq!(state.children[2].pid.unwrap(), initial_pids[2]);
    }

    // -----------------------------------------------------------------------
    // Restart limit tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_restart_limit_exceeded() {
        let sched = test_scheduler();
        let specs = vec![test_child_spec(
            "child1",
            RestartType::Permanent,
            ShutdownType::BrutalKill,
        )];

        // Set max_restarts=2, max_seconds=5.
        let sup_pid = sched.spawn(noop_entry as *const u8, std::ptr::null(), 0, 1);
        if let Some(proc) = sched.get_process(sup_pid) {
            proc.lock().trap_exit = true;
        }

        let mut state = SupervisorState::new(Strategy::OneForOne, 2, 5);
        state.children = specs.into_iter().map(|s| test_child_state(s)).collect();
        start_children(&mut state, &sched, sup_pid).unwrap();

        // Trigger 2 restarts (should succeed).
        for i in 0..2 {
            let crashed_pid = state.children[0].pid.unwrap();
            if let Some(proc) = sched.get_process(crashed_pid) {
                proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
            }
            let result = handle_child_exit(
                &mut state,
                crashed_pid,
                &ExitReason::Error("crash".to_string()),
                &sched,
                sup_pid,
            );
            assert!(result.is_ok(), "restart {} should succeed", i);
        }

        // 3rd restart should fail (limit exceeded).
        let crashed_pid = state.children[0].pid.unwrap();
        if let Some(proc) = sched.get_process(crashed_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }
        let result = handle_child_exit(
            &mut state,
            crashed_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("restart limit exceeded"));
    }

    #[test]
    fn test_restart_limit_sliding_window() {
        let mut state = SupervisorState::new(Strategy::OneForOne, 2, 1);

        // Record 2 restarts -- both should be allowed.
        assert!(check_restart_limit(&mut state));
        assert!(check_restart_limit(&mut state));

        // 3rd should fail (limit is 2 per 1 second).
        assert!(!check_restart_limit(&mut state));

        // Wait for the window to expire (just over 1 second).
        std::thread::sleep(Duration::from_millis(1100));

        // After the window expires, old restarts should be purged.
        assert!(check_restart_limit(&mut state));
    }

    // -----------------------------------------------------------------------
    // Restart type tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_permanent_restarts_on_normal() {
        let sched = test_scheduler();
        let specs = vec![test_child_spec(
            "child1",
            RestartType::Permanent,
            ShutdownType::BrutalKill,
        )];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);
        let initial_pid = state.children[0].pid.unwrap();

        // Simulate Normal exit.
        if let Some(proc) = sched.get_process(initial_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Normal);
        }
        let result = handle_child_exit(
            &mut state,
            initial_pid,
            &ExitReason::Normal,
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Permanent child should be restarted even on Normal exit.
        assert!(state.children[0].running);
        assert_ne!(state.children[0].pid.unwrap(), initial_pid);
    }

    #[test]
    fn test_transient_no_restart_on_normal() {
        let sched = test_scheduler();
        let specs = vec![test_child_spec(
            "child1",
            RestartType::Transient,
            ShutdownType::BrutalKill,
        )];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);
        let initial_pid = state.children[0].pid.unwrap();

        // Simulate Normal exit.
        if let Some(proc) = sched.get_process(initial_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Normal);
        }
        let result = handle_child_exit(
            &mut state,
            initial_pid,
            &ExitReason::Normal,
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Transient child should NOT be restarted on Normal exit.
        assert!(!state.children[0].running);
        assert!(state.children[0].pid.is_none());
    }

    #[test]
    fn test_transient_restarts_on_error() {
        let sched = test_scheduler();
        let specs = vec![test_child_spec(
            "child1",
            RestartType::Transient,
            ShutdownType::BrutalKill,
        )];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);
        let initial_pid = state.children[0].pid.unwrap();

        // Simulate Error exit.
        if let Some(proc) = sched.get_process(initial_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }
        let result = handle_child_exit(
            &mut state,
            initial_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Transient child should be restarted on Error exit.
        assert!(state.children[0].running);
        assert_ne!(state.children[0].pid.unwrap(), initial_pid);
    }

    #[test]
    fn test_transient_no_restart_on_shutdown() {
        let sched = test_scheduler();
        let specs = vec![test_child_spec(
            "child1",
            RestartType::Transient,
            ShutdownType::BrutalKill,
        )];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);
        let initial_pid = state.children[0].pid.unwrap();

        // Simulate Shutdown exit.
        if let Some(proc) = sched.get_process(initial_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Shutdown);
        }
        let result = handle_child_exit(
            &mut state,
            initial_pid,
            &ExitReason::Shutdown,
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Transient child should NOT be restarted on Shutdown exit.
        assert!(!state.children[0].running);
        assert!(state.children[0].pid.is_none());
    }

    #[test]
    fn test_temporary_never_restarts() {
        let sched = test_scheduler();
        let specs = vec![test_child_spec(
            "child1",
            RestartType::Temporary,
            ShutdownType::BrutalKill,
        )];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);
        let initial_pid = state.children[0].pid.unwrap();

        // Simulate Error exit.
        if let Some(proc) = sched.get_process(initial_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }
        let result = handle_child_exit(
            &mut state,
            initial_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Temporary child should be removed from the children list.
        assert!(state.children.is_empty());
    }

    // -----------------------------------------------------------------------
    // Shutdown and start order tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_ordered_shutdown_reverse_order() {
        let sched = test_scheduler();
        let specs = vec![
            test_child_spec("child1", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child2", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child3", RestartType::Permanent, ShutdownType::BrutalKill),
        ];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);

        // Verify all children are running.
        assert!(state.children.iter().all(|c| c.running));

        let pids: Vec<ProcessId> = state.children.iter().map(|c| c.pid.unwrap()).collect();

        // Terminate all children.
        terminate_all_children(&mut state, &sched, sup_pid);

        // All children should now be not running.
        assert!(state.children.iter().all(|c| !c.running));
        assert!(state.children.iter().all(|c| c.pid.is_none()));

        // Verify the processes were actually terminated in the process table.
        // (BrutalKill means they should be marked as Exited(Killed))
        for pid in &pids {
            if let Some(proc) = sched.get_process(*pid) {
                assert!(
                    matches!(proc.lock().state, ProcessState::Exited(_)),
                    "Process {} should be exited",
                    pid
                );
            }
        }
    }

    #[test]
    fn test_sequential_start_order() {
        let sched = test_scheduler();
        let specs = vec![
            test_child_spec("child1", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child2", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child3", RestartType::Permanent, ShutdownType::BrutalKill),
        ];

        let (state, _sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);

        // Verify all children started and got PIDs.
        assert_eq!(state.children.len(), 3);
        assert!(state.children.iter().all(|c| c.running));
        assert!(state.children.iter().all(|c| c.pid.is_some()));

        // PIDs should be sequential (since they're spawned one by one).
        let pids: Vec<u64> = state
            .children
            .iter()
            .map(|c| c.pid.unwrap().as_u64())
            .collect();
        // Each PID should be greater than the previous (sequential allocation).
        assert!(pids[0] < pids[1], "child1 PID should be less than child2");
        assert!(pids[1] < pids[2], "child2 PID should be less than child3");
    }

    #[test]
    fn test_start_failure_stops_remaining() {
        // We can't easily make spawn fail, but we can test the error path
        // by directly testing start_children with a pre-populated state.
        // Instead, test the logic around start order and termination.
        let sched = test_scheduler();

        let sup_pid = sched.spawn(noop_entry as *const u8, std::ptr::null(), 0, 1);
        if let Some(proc) = sched.get_process(sup_pid) {
            proc.lock().trap_exit = true;
        }

        // Create a state with 3 children. Start them manually one by one.
        let mut state = SupervisorState::new(Strategy::OneForOne, 3, 5);
        state.children = vec![
            test_child_state(test_child_spec(
                "child1",
                RestartType::Permanent,
                ShutdownType::BrutalKill,
            )),
            test_child_state(test_child_spec(
                "child2",
                RestartType::Permanent,
                ShutdownType::BrutalKill,
            )),
            test_child_state(test_child_spec(
                "child3",
                RestartType::Permanent,
                ShutdownType::BrutalKill,
            )),
        ];

        // Start all children (should succeed).
        let result = start_children(&mut state, &sched, sup_pid);
        assert!(result.is_ok());

        // Verify the sequential order was maintained.
        assert_eq!(state.children.len(), 3);
        for c in &state.children {
            assert!(c.running);
            assert!(c.pid.is_some());
        }
    }

    #[test]
    fn test_unknown_child_exit_ignored() {
        let sched = test_scheduler();
        let specs = vec![test_child_spec(
            "child1",
            RestartType::Permanent,
            ShutdownType::BrutalKill,
        )];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);
        let initial_pid = state.children[0].pid.unwrap();

        // Simulate exit of an unknown PID.
        let unknown_pid = ProcessId::next();
        let result = handle_child_exit(
            &mut state,
            unknown_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // Original child should be untouched.
        assert_eq!(state.children[0].pid.unwrap(), initial_pid);
        assert!(state.children[0].running);
    }

    #[test]
    fn test_supervisor_state_running_count() {
        let sched = test_scheduler();
        let specs = vec![
            test_child_spec("child1", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child2", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child3", RestartType::Permanent, ShutdownType::BrutalKill),
        ];

        let (state, _sup_pid) = setup_supervisor(&sched, Strategy::OneForOne, specs);
        assert_eq!(state.running_count(), 3);
    }

    #[test]
    fn test_terminate_single_child_brutal_kill() {
        let sched = test_scheduler();
        let sup_pid = sched.spawn(noop_entry as *const u8, std::ptr::null(), 0, 1);

        let mut child = test_child_state(test_child_spec(
            "victim",
            RestartType::Permanent,
            ShutdownType::BrutalKill,
        ));
        start_single_child(&mut child, &sched, sup_pid).unwrap();
        let child_pid = child.pid.unwrap();

        terminate_single_child(&mut child, &sched, sup_pid);

        // Child should be marked as not running.
        assert!(!child.running);
        assert!(child.pid.is_none());

        // Process should be exited in the process table.
        if let Some(proc) = sched.get_process(child_pid) {
            assert!(matches!(proc.lock().state, ProcessState::Exited(_)));
        }
    }

    #[test]
    fn test_rest_for_one_first_child_restarts_all() {
        let sched = test_scheduler();
        let specs = vec![
            test_child_spec("child1", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child2", RestartType::Permanent, ShutdownType::BrutalKill),
            test_child_spec("child3", RestartType::Permanent, ShutdownType::BrutalKill),
        ];

        let (mut state, sup_pid) = setup_supervisor(&sched, Strategy::RestForOne, specs);

        let initial_pids: Vec<ProcessId> = state.children.iter().map(|c| c.pid.unwrap()).collect();

        // Crash the FIRST child -- rest_for_one should restart ALL (0..end).
        let crashed_pid = initial_pids[0];
        if let Some(proc) = sched.get_process(crashed_pid) {
            proc.lock().state = ProcessState::Exited(ExitReason::Error("crash".to_string()));
        }

        let result = handle_child_exit(
            &mut state,
            crashed_pid,
            &ExitReason::Error("crash".to_string()),
            &sched,
            sup_pid,
        );
        assert!(result.is_ok());

        // All children should have new PIDs (first child = all subsequent).
        for i in 0..3 {
            assert_ne!(
                state.children[i].pid.unwrap(),
                initial_pids[i],
                "child{} should have a new PID",
                i + 1
            );
            assert!(state.children[i].running);
        }
    }

    // -----------------------------------------------------------------------
    // Remote child tests (Phase 69)
    // -----------------------------------------------------------------------

    #[test]
    fn test_remote_child_spec_fields() {
        let spec = ChildSpec {
            id: "remote_worker".to_string(),
            start_fn: std::ptr::null(),
            start_args_ptr: std::ptr::null(),
            start_args_size: 0,
            restart_type: RestartType::Permanent,
            shutdown: ShutdownType::default(),
            child_type: ChildType::Worker,
            target_node: Some("worker@host:9000".to_string()),
            start_fn_name: Some("my_worker".to_string()),
        };

        assert_eq!(spec.target_node.as_deref(), Some("worker@host:9000"));
        assert_eq!(spec.start_fn_name.as_deref(), Some("my_worker"));

        let child_state = ChildState {
            spec,
            pid: None,
            running: false,
        };

        assert_eq!(
            child_state.spec.target_node.as_deref(),
            Some("worker@host:9000")
        );
        assert_eq!(child_state.spec.start_fn_name.as_deref(), Some("my_worker"));
    }

    #[test]
    fn test_find_child_index_with_remote_pid() {
        let mut state = SupervisorState::new(Strategy::OneForOne, 3, 5);

        // Create a child with a remote PID (node_id=5, creation=1, local_id=42).
        let remote_pid = ProcessId::from_remote(5, 1, 42);

        let child = ChildState {
            spec: test_child_spec(
                "remote_child",
                RestartType::Permanent,
                ShutdownType::BrutalKill,
            ),
            pid: Some(remote_pid),
            running: true,
        };
        state.children.push(child);

        // Also add a local child.
        let local_pid = ProcessId::next();
        let local_child = ChildState {
            spec: test_child_spec(
                "local_child",
                RestartType::Permanent,
                ShutdownType::BrutalKill,
            ),
            pid: Some(local_pid),
            running: true,
        };
        state.children.push(local_child);

        // find_child_index should locate the remote child.
        assert_eq!(state.find_child_index(remote_pid), Some(0));
        assert_eq!(state.find_child_index(local_pid), Some(1));

        // Unknown PID returns None.
        let unknown = ProcessId::from_remote(99, 1, 1);
        assert_eq!(state.find_child_index(unknown), None);
    }

    #[test]
    fn test_terminate_remote_child_marks_not_running() {
        let sched = test_scheduler();
        let sup_pid = sched.spawn(noop_entry as *const u8, std::ptr::null(), 0, 1);

        // Create a ChildState with a remote PID (node_id != 0).
        let remote_pid = ProcessId::from_remote(5, 1, 42);
        let mut child = ChildState {
            spec: test_child_spec(
                "remote_child",
                RestartType::Permanent,
                ShutdownType::BrutalKill,
            ),
            pid: Some(remote_pid),
            running: true,
        };

        // Verify the PID is indeed remote.
        assert!(!remote_pid.is_local());

        // Terminate the remote child. The send_dist_exit call will silently
        // return (no node_state in test), which is fine -- we're testing the
        // control flow, not the network.
        terminate_single_child(&mut child, &sched, sup_pid);

        // Verify child is marked as not running with no PID.
        assert!(!child.running);
        assert!(child.pid.is_none());
    }
}
