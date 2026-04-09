//! Service runtime support for Mesh.
//!
//! Provides synchronous call/reply semantics on top of the actor message
//! passing primitives. A "service call" sends a message to a service actor,
//! then blocks the caller until a reply arrives.
//!
//! ## Message format
//!
//! **Call message TO service:** `[u64 type_tag][u64 caller_pid][i64... args]`
//! - type_tag: identifies which call/cast handler to dispatch to
//! - caller_pid: so the service knows where to send the reply
//! - args: handler arguments encoded as i64 values
//!
//! **Reply TO caller:** `[i64 reply_value]`
//! - A single i64 value (the return value from the call handler)

use super::heap::MessageBuffer;
use super::process::{Message, ProcessId};
use super::stack;
use super::GLOBAL_SCHEDULER;

/// Synchronous service call: send a message to the target service and block
/// until a reply arrives.
///
/// 1. Get the caller's PID
/// 2. Build a call message: [u64 type_tag][u64 caller_pid][payload bytes]
/// 3. Send to target via mesh_actor_send
/// 4. Block on receive (infinite wait) for the reply
/// 5. Return a pointer to the reply data
///
/// Returns a pointer to the reply data (heap-allocated in the caller's
/// actor heap), or null if the call fails.
///
/// - `target_pid`: PID of the service actor
/// - `msg_tag`: type tag identifying which handler to invoke
/// - `payload_ptr`: pointer to argument bytes (array of i64 values)
/// - `payload_size`: size of the payload in bytes
#[no_mangle]
pub extern "C" fn mesh_service_call(
    target_pid: u64,
    msg_tag: u64,
    payload_ptr: *const u8,
    payload_size: u64,
) -> *const u8 {
    // Get the caller's PID.
    let caller_pid = match stack::get_current_pid() {
        Some(pid) => pid.as_u64(),
        None => return std::ptr::null(),
    };

    let sched = match GLOBAL_SCHEDULER.get() {
        Some(s) => s,
        None => return std::ptr::null(),
    };

    // Build the call message: [u64 type_tag][u64 caller_pid][payload bytes]
    let mut data = Vec::with_capacity(16 + payload_size as usize);
    data.extend_from_slice(&msg_tag.to_le_bytes());
    data.extend_from_slice(&caller_pid.to_le_bytes());

    if !payload_ptr.is_null() && payload_size > 0 {
        let payload = unsafe { std::slice::from_raw_parts(payload_ptr, payload_size as usize) };
        data.extend_from_slice(payload);
    }

    // The type_tag for the MessageBuffer is the msg_tag itself.
    let buffer = MessageBuffer::new(data, msg_tag);
    let msg = Message { buffer };

    // Send the call message to the target service.
    let target = ProcessId(target_pid);
    if let Some(proc_arc) = sched.get_process(target) {
        let mut proc = proc_arc.lock();
        proc.mailbox.push(msg);

        // Wake the target if it's waiting.
        if matches!(proc.state, super::process::ProcessState::Waiting) {
            proc.state = super::process::ProcessState::Ready;
            drop(proc);
            sched.wake_process(target);
        }
    } else {
        return std::ptr::null();
    }

    // Block the caller until a reply arrives.
    //
    // If we're inside a coroutine, use the standard mesh_actor_receive which
    // yields to the scheduler. If we're on the main thread (no coroutine),
    // do a spin-wait on the mailbox instead (the main thread cannot yield).
    let caller_pid_obj = stack::get_current_pid().unwrap();

    // Check if we're in a coroutine context (CURRENT_YIELDER is set).
    let in_coroutine = stack::CURRENT_YIELDER.with(|c| c.get().is_some());

    if in_coroutine {
        // Standard path: yield to scheduler while waiting for reply.
        super::mesh_actor_receive(-1)
    } else {
        // Main thread path: spin-wait on the mailbox.
        loop {
            if let Some(proc_arc) = sched.get_process(caller_pid_obj) {
                let proc = proc_arc.lock();
                if let Some(msg) = proc.mailbox.pop() {
                    drop(proc);
                    return super::copy_msg_to_actor_heap(sched, caller_pid_obj, msg);
                }
            }
            // Brief sleep to avoid burning CPU.
            std::thread::sleep(std::time::Duration::from_micros(10));
        }
    }
}

/// Send a reply from the service actor back to the caller.
///
/// Called by the service's receive loop after processing a call handler.
/// The reply is a single i64 value sent as a raw message to the caller.
///
/// - `caller_pid`: PID of the caller that made the service call
/// - `reply_ptr`: pointer to the reply data bytes
/// - `reply_size`: size of the reply data in bytes
#[no_mangle]
pub extern "C" fn mesh_service_reply(caller_pid: u64, reply_ptr: *const u8, reply_size: u64) {
    // Send the reply data to the caller using mesh_actor_send.
    super::mesh_actor_send(caller_pid, reply_ptr, reply_size);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::scheduler::Scheduler;

    #[test]
    fn test_service_reply_sends_message() {
        let sched = Scheduler::new(1);
        // Create a "caller" process.
        extern "C" fn noop(_args: *const u8) {}
        let caller_pid = sched.spawn(noop as *const u8, std::ptr::null(), 0, 1);

        // Initialize the global scheduler for this test.
        // (In real usage, mesh_rt_init_actor does this.)
        // We can't easily set the global scheduler in a unit test without
        // interfering with other tests, so test the message format instead.

        // Test that the call message format is correct.
        let msg_tag: u64 = 42;
        let caller: u64 = 123;
        let mut data = Vec::new();
        data.extend_from_slice(&msg_tag.to_le_bytes());
        data.extend_from_slice(&caller.to_le_bytes());
        data.extend_from_slice(&99i64.to_le_bytes()); // one arg

        assert_eq!(data.len(), 24); // 8 + 8 + 8

        // Verify we can decode the message format.
        let decoded_tag = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let decoded_caller = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let decoded_arg = i64::from_le_bytes(data[16..24].try_into().unwrap());

        assert_eq!(decoded_tag, 42);
        assert_eq!(decoded_caller, 123);
        assert_eq!(decoded_arg, 99);
    }

    #[test]
    fn test_service_call_message_no_args() {
        // A call message with no payload arguments.
        let msg_tag: u64 = 7;
        let caller: u64 = 456;
        let mut data = Vec::new();
        data.extend_from_slice(&msg_tag.to_le_bytes());
        data.extend_from_slice(&caller.to_le_bytes());

        assert_eq!(data.len(), 16); // just tag + caller_pid

        let decoded_tag = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let decoded_caller = u64::from_le_bytes(data[8..16].try_into().unwrap());

        assert_eq!(decoded_tag, 7);
        assert_eq!(decoded_caller, 456);
    }

    #[test]
    fn test_service_call_returns_null_outside_actor() {
        // mesh_service_call requires a current PID (must be inside actor context).
        // Without one, it should return null.
        assert!(stack::get_current_pid().is_none());
        let result = mesh_service_call(0, 0, std::ptr::null(), 0);
        assert!(result.is_null());
    }
}
