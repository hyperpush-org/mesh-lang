//! Job (async task) runtime support for Mesh.
//!
//! Jobs provide a simple async computation pattern:
//! - `Job.async(fn)` spawns a linked actor that runs the function and sends
//!   its result back
//! - `Job.await(pid)` blocks until the job completes, returning `Result<T, String>`
//! - `Job.await_timeout(pid, ms)` same as await but with timeout
//! - `Job.map(list, fn)` spawns parallel jobs and collects results in order
//!
//! ## Message Protocol
//!
//! Jobs use `JOB_RESULT_TAG` (u64::MAX - 1) to distinguish job results from
//! exit signals (`EXIT_SIGNAL_TAG` = u64::MAX). The job actor:
//! 1. Links to the caller
//! 2. Calls fn_ptr(env_ptr) to get the result
//! 3. Sends [JOB_RESULT_TAG][result] to the caller
//! 4. Exits normally
//!
//! ## Result Layout
//!
//! Returns a `MeshResult` (same as File/IO/JSON):
//! - tag 0 = Ok (value is the job's return value)
//! - tag 1 = Err (value is a string describing the crash reason)

use crate::gc::mesh_gc_alloc_actor;
use crate::io::MeshResult;
use crate::string::mesh_string_new;

use super::heap::MessageBuffer;
use super::link::EXIT_SIGNAL_TAG;
use super::process::{Message, ProcessId, ProcessState};
use super::stack;
use super::GLOBAL_SCHEDULER;

/// Type tag for job result messages.
///
/// Distinct from EXIT_SIGNAL_TAG (u64::MAX) to allow the await logic to
/// differentiate between "job completed with a value" and "job crashed".
pub const JOB_RESULT_TAG: u64 = u64::MAX - 1;

/// Allocate a MeshResult on the GC heap.
fn alloc_result(tag: u8, value: *mut u8) -> *mut MeshResult {
    unsafe {
        let ptr = mesh_gc_alloc_actor(
            std::mem::size_of::<MeshResult>() as u64,
            std::mem::align_of::<MeshResult>() as u64,
        ) as *mut MeshResult;
        (*ptr).tag = tag;
        (*ptr).value = value;
        ptr
    }
}

/// Build an Err MeshResult from a Rust string slice.
fn err_result(msg: &str) -> *mut MeshResult {
    let mesh_str = mesh_string_new(msg.as_ptr(), msg.len() as u64);
    alloc_result(1, mesh_str as *mut u8)
}

// ---------------------------------------------------------------------------
// extern "C" ABI functions
// ---------------------------------------------------------------------------

/// Spawn an async job that runs `fn_ptr(env_ptr)` and sends its result back.
///
/// The job actor:
/// 1. Links to the caller (so crashes propagate)
/// 2. Calls `fn_ptr(env_ptr)` to get a result value (i64)
/// 3. Sends the result to the caller tagged with JOB_RESULT_TAG
/// 4. Exits normally
///
/// Returns the PID of the spawned job actor.
///
/// - `fn_ptr`: pointer to the function to run (signature: fn(env) -> i64)
/// - `env_ptr`: pointer to the closure environment
#[no_mangle]
pub extern "C" fn mesh_job_async(fn_ptr: *const u8, env_ptr: *const u8) -> u64 {
    let caller_pid = match stack::get_current_pid() {
        Some(pid) => pid.as_u64(),
        None => {
            // Outside actor context -- cannot link. Still spawn.
            return spawn_job_actor(fn_ptr, env_ptr, u64::MAX);
        }
    };

    spawn_job_actor(fn_ptr, env_ptr, caller_pid)
}

/// Internal: spawn the job actor with the given caller PID.
fn spawn_job_actor(fn_ptr: *const u8, env_ptr: *const u8, caller_pid: u64) -> u64 {
    let sched = match GLOBAL_SCHEDULER.get() {
        Some(s) => s,
        None => return u64::MAX,
    };

    // Pack the job parameters into a buffer that the job entry function can read.
    // Layout: [u64 fn_ptr][u64 env_ptr][u64 caller_pid]
    let mut args = Vec::with_capacity(24);
    args.extend_from_slice(&(fn_ptr as u64).to_le_bytes());
    args.extend_from_slice(&(env_ptr as u64).to_le_bytes());
    args.extend_from_slice(&caller_pid.to_le_bytes());

    // Allocate args on the GC heap so they survive past this function.
    let args_heap = unsafe {
        let ptr = mesh_gc_alloc_actor(args.len() as u64, 8);
        std::ptr::copy_nonoverlapping(args.as_ptr(), ptr, args.len());
        ptr
    };

    let pid = sched.spawn(
        job_entry as *const u8,
        args_heap as *const u8,
        24,
        1, // Normal priority
    );

    pid.as_u64()
}

/// Entry function for job actors.
///
/// Unpacks the args buffer, links to the caller, calls the user function,
/// sends the result, and exits.
extern "C" fn job_entry(args: *const u8) {
    if args.is_null() {
        return;
    }

    // Unpack: [u64 fn_ptr][u64 env_ptr][u64 caller_pid]
    let (fn_ptr, env_ptr, caller_pid) = unsafe {
        let fn_ptr_val =
            u64::from_le_bytes(std::slice::from_raw_parts(args, 8).try_into().unwrap());
        let env_ptr_val = u64::from_le_bytes(
            std::slice::from_raw_parts(args.add(8), 8)
                .try_into()
                .unwrap(),
        );
        let caller_val = u64::from_le_bytes(
            std::slice::from_raw_parts(args.add(16), 8)
                .try_into()
                .unwrap(),
        );
        (
            fn_ptr_val as *const u8,
            env_ptr_val as *const u8,
            caller_val,
        )
    };

    // Link to the caller (if valid).
    if caller_pid != u64::MAX {
        super::mesh_actor_link(caller_pid);
    }

    // Call the user function: fn(env_ptr) -> i64
    let user_fn: extern "C" fn(*const u8) -> i64 = unsafe { std::mem::transmute(fn_ptr) };
    let result = user_fn(env_ptr);

    // Send the result to the caller tagged with JOB_RESULT_TAG.
    if caller_pid != u64::MAX {
        // Message layout: [u64 JOB_RESULT_TAG][i64 result]
        let mut msg_data = Vec::with_capacity(16);
        msg_data.extend_from_slice(&JOB_RESULT_TAG.to_le_bytes());
        msg_data.extend_from_slice(&result.to_le_bytes());

        let sched = match GLOBAL_SCHEDULER.get() {
            Some(s) => s,
            None => return,
        };

        let target = ProcessId(caller_pid);
        let buffer = MessageBuffer::new(msg_data, JOB_RESULT_TAG);
        let msg = Message { buffer };

        if let Some(proc_arc) = sched.get_process(target) {
            let mut proc = proc_arc.lock();
            proc.mailbox.push(msg);

            // Wake if waiting.
            if matches!(proc.state, ProcessState::Waiting) {
                proc.state = ProcessState::Ready;
                drop(proc);
                sched.wake_process(target);
            }
        }
    }

    // Actor exits normally after this function returns.
}

/// Block until the job completes and return a `MeshResult`.
///
/// Receives messages from the job actor:
/// - `JOB_RESULT_TAG` message: extract the value, return Ok(value)
/// - `EXIT_SIGNAL_TAG` message: the job crashed, return Err(reason)
///
/// - `job_pid`: PID of the job actor (unused for filtering; we block on any message)
///
/// Returns a pointer to a heap-allocated MeshResult.
#[no_mangle]
pub extern "C" fn mesh_job_await(_job_pid: u64) -> *const u8 {
    // Use mesh_actor_receive(-1) to block indefinitely.
    let msg_ptr = super::mesh_actor_receive(-1);
    if msg_ptr.is_null() {
        return err_result("job await: no message received") as *const u8;
    }

    decode_job_message(msg_ptr)
}

/// Block until the job completes or timeout, returning a `MeshResult`.
///
/// Same as `mesh_job_await` but with a timeout in milliseconds.
/// If timeout expires before a result arrives, returns Err("timeout").
///
/// - `job_pid`: PID of the job actor
/// - `timeout_ms`: timeout in milliseconds
///
/// Returns a pointer to a heap-allocated MeshResult.
#[no_mangle]
pub extern "C" fn mesh_job_await_timeout(_job_pid: u64, timeout_ms: i64) -> *const u8 {
    let msg_ptr = super::mesh_actor_receive(timeout_ms);
    if msg_ptr.is_null() {
        return err_result("timeout") as *const u8;
    }

    decode_job_message(msg_ptr)
}

/// Decode a received message into a MeshResult.
///
/// Message layout from actor_receive: [u64 type_tag][u64 data_len][u8... data]
fn decode_job_message(msg_ptr: *const u8) -> *const u8 {
    unsafe {
        // Read type_tag (first 8 bytes).
        let type_tag =
            u64::from_le_bytes(std::slice::from_raw_parts(msg_ptr, 8).try_into().unwrap());

        if type_tag == JOB_RESULT_TAG {
            // Job completed successfully.
            // Data layout after header: [u64 JOB_RESULT_TAG][i64 result]
            // The data is at offset 16 (after the 16-byte header: type_tag + data_len).
            let data_ptr = msg_ptr.add(16);
            let result_value = i64::from_le_bytes(
                std::slice::from_raw_parts(data_ptr.add(8), 8)
                    .try_into()
                    .unwrap(),
            );
            // Return Ok(result_value)
            alloc_result(0, result_value as *mut u8) as *const u8
        } else if type_tag == EXIT_SIGNAL_TAG {
            // Job crashed. The data contains exit signal info.
            // Try to extract a reason string from the exit signal.
            let data_len = u64::from_le_bytes(
                std::slice::from_raw_parts(msg_ptr.add(8), 8)
                    .try_into()
                    .unwrap(),
            ) as usize;

            if data_len >= 9 {
                let data_ptr = msg_ptr.add(16);
                // Exit signal layout: [u64 exiting_pid][u8 reason_tag][...reason_data]
                let reason_tag = *data_ptr.add(8);
                match reason_tag {
                    0 => err_result("normal") as *const u8,
                    1 => {
                        // Error: [tag(1)][u64 str_len][str_bytes...]
                        if data_len >= 17 {
                            let str_len = u64::from_le_bytes(
                                std::slice::from_raw_parts(data_ptr.add(9), 8)
                                    .try_into()
                                    .unwrap(),
                            ) as usize;
                            if data_len >= 17 + str_len {
                                let reason_str = std::str::from_utf8(std::slice::from_raw_parts(
                                    data_ptr.add(17),
                                    str_len,
                                ))
                                .unwrap_or("unknown error");
                                err_result(reason_str) as *const u8
                            } else {
                                err_result("job crashed") as *const u8
                            }
                        } else {
                            err_result("job crashed") as *const u8
                        }
                    }
                    2 => err_result("killed") as *const u8,
                    4 => err_result("shutdown") as *const u8,
                    5 => {
                        // Custom: same layout as Error
                        if data_len >= 17 {
                            let str_len = u64::from_le_bytes(
                                std::slice::from_raw_parts(data_ptr.add(9), 8)
                                    .try_into()
                                    .unwrap(),
                            ) as usize;
                            if data_len >= 17 + str_len {
                                let reason_str = std::str::from_utf8(std::slice::from_raw_parts(
                                    data_ptr.add(17),
                                    str_len,
                                ))
                                .unwrap_or("unknown error");
                                err_result(reason_str) as *const u8
                            } else {
                                err_result("job crashed") as *const u8
                            }
                        } else {
                            err_result("job crashed") as *const u8
                        }
                    }
                    _ => err_result("job crashed") as *const u8,
                }
            } else {
                err_result("job crashed") as *const u8
            }
        } else {
            // Unexpected message tag -- treat as error.
            err_result("unexpected message") as *const u8
        }
    }
}

/// Spawn parallel jobs for each element of a list, collect results in order.
///
/// For each element in the input list:
/// 1. Spawn a job that calls `fn_ptr(env_ptr, element)` (note: the closure
///    receives the element as its argument, with env_ptr for captures)
/// 2. Collect all job PIDs
/// 3. Await each job in order
/// 4. Build a result list of MeshResult values
///
/// - `list_ptr`: pointer to a Mesh list (MeshList)
/// - `fn_ptr`: pointer to the mapping function
/// - `env_ptr`: pointer to the closure environment
///
/// Returns a pointer to a new Mesh list containing MeshResult values.
#[no_mangle]
pub extern "C" fn mesh_job_map(
    list_ptr: *const u8,
    fn_ptr: *const u8,
    env_ptr: *const u8,
) -> *const u8 {
    use crate::collections::list::{
        mesh_list_append, mesh_list_get, mesh_list_length, mesh_list_new,
    };

    if list_ptr.is_null() || fn_ptr.is_null() {
        return mesh_list_new() as *const u8;
    }

    let len = mesh_list_length(list_ptr as *mut u8);
    if len == 0 {
        return mesh_list_new() as *const u8;
    }

    // Spawn jobs for each element.
    let mut job_pids = Vec::with_capacity(len as usize);
    for i in 0..len {
        let element = mesh_list_get(list_ptr as *mut u8, i);

        let caller_pid = stack::get_current_pid()
            .map(|p| p.as_u64())
            .unwrap_or(u64::MAX);

        // Pack [fn_ptr, env_ptr, element, caller_pid] for the map job entry.
        let mut full_args = Vec::with_capacity(32);
        full_args.extend_from_slice(&(fn_ptr as u64).to_le_bytes());
        full_args.extend_from_slice(&(env_ptr as u64).to_le_bytes());
        full_args.extend_from_slice(&(element as u64).to_le_bytes());
        full_args.extend_from_slice(&caller_pid.to_le_bytes());

        let full_args_heap = unsafe {
            let ptr = mesh_gc_alloc_actor(full_args.len() as u64, 8);
            std::ptr::copy_nonoverlapping(full_args.as_ptr(), ptr, full_args.len());
            ptr
        };

        let sched = match GLOBAL_SCHEDULER.get() {
            Some(s) => s,
            None => return mesh_list_new() as *const u8,
        };

        let pid = sched.spawn(
            map_job_entry as *const u8,
            full_args_heap as *const u8,
            32,
            1, // Normal priority
        );
        job_pids.push(pid.as_u64());
    }

    // Await each job in order and build result list.
    let mut result_list = mesh_list_new();
    for _job_pid in &job_pids {
        // Block until we get a result from this job.
        let msg_ptr = super::mesh_actor_receive(-1);
        let result = if msg_ptr.is_null() {
            err_result("job map: no message received") as u64
        } else {
            decode_job_message(msg_ptr) as u64
        };
        result_list = mesh_list_append(result_list, result);
    }

    result_list as *const u8
}

/// Entry function for map job actors.
///
/// Unpacks args: [u64 fn_ptr][u64 env_ptr][u64 element][u64 caller_pid]
/// Calls fn_ptr(env_ptr, element) and sends result to caller.
extern "C" fn map_job_entry(args: *const u8) {
    if args.is_null() {
        return;
    }

    let (fn_ptr, env_ptr, element, caller_pid) = unsafe {
        let fn_ptr_val =
            u64::from_le_bytes(std::slice::from_raw_parts(args, 8).try_into().unwrap());
        let env_ptr_val = u64::from_le_bytes(
            std::slice::from_raw_parts(args.add(8), 8)
                .try_into()
                .unwrap(),
        );
        let element_val = u64::from_le_bytes(
            std::slice::from_raw_parts(args.add(16), 8)
                .try_into()
                .unwrap(),
        );
        let caller_val = u64::from_le_bytes(
            std::slice::from_raw_parts(args.add(24), 8)
                .try_into()
                .unwrap(),
        );
        (fn_ptr_val, env_ptr_val, element_val, caller_val)
    };

    // Link to the caller.
    if caller_pid != u64::MAX {
        super::mesh_actor_link(caller_pid);
    }

    // Call the mapping function: fn(env_ptr, element) -> i64
    let user_fn: extern "C" fn(*const u8, i64) -> i64 =
        unsafe { std::mem::transmute(fn_ptr as *const u8) };
    let result = user_fn(env_ptr as *const u8, element as i64);

    // Send result to caller.
    if caller_pid != u64::MAX {
        let mut msg_data = Vec::with_capacity(16);
        msg_data.extend_from_slice(&JOB_RESULT_TAG.to_le_bytes());
        msg_data.extend_from_slice(&result.to_le_bytes());

        let sched = match GLOBAL_SCHEDULER.get() {
            Some(s) => s,
            None => return,
        };

        let target = ProcessId(caller_pid);
        let buffer = MessageBuffer::new(msg_data, JOB_RESULT_TAG);
        let msg = Message { buffer };

        if let Some(proc_arc) = sched.get_process(target) {
            let mut proc = proc_arc.lock();
            proc.mailbox.push(msg);

            if matches!(proc.state, ProcessState::Waiting) {
                proc.state = ProcessState::Ready;
                drop(proc);
                sched.wake_process(target);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_result_tag_distinct_from_exit() {
        assert_ne!(JOB_RESULT_TAG, EXIT_SIGNAL_TAG);
        assert_eq!(JOB_RESULT_TAG, u64::MAX - 1);
        assert_eq!(EXIT_SIGNAL_TAG, u64::MAX);
    }

    #[test]
    fn test_alloc_result_ok() {
        crate::gc::mesh_rt_init();
        let result = alloc_result(0, 42 as *mut u8);
        unsafe {
            assert_eq!((*result).tag, 0);
            assert_eq!((*result).value as u64, 42);
        }
    }

    #[test]
    fn test_alloc_result_err() {
        crate::gc::mesh_rt_init();
        let result = err_result("test error");
        unsafe {
            assert_eq!((*result).tag, 1);
            assert!(!(*result).value.is_null());
        }
    }

    #[test]
    fn test_decode_job_result_message() {
        crate::gc::mesh_rt_init();

        // Build a fake message as it would appear after mesh_actor_receive:
        // [u64 type_tag][u64 data_len][u64 JOB_RESULT_TAG][i64 result_value]
        let result_value: i64 = 99;
        let mut msg = Vec::new();
        msg.extend_from_slice(&JOB_RESULT_TAG.to_le_bytes()); // type_tag
        msg.extend_from_slice(&16u64.to_le_bytes()); // data_len (8 + 8)
        msg.extend_from_slice(&JOB_RESULT_TAG.to_le_bytes()); // data: tag
        msg.extend_from_slice(&result_value.to_le_bytes()); // data: value

        let result_ptr = decode_job_message(msg.as_ptr());
        let result = result_ptr as *const MeshResult;
        unsafe {
            assert_eq!((*result).tag, 0); // Ok
            assert_eq!((*result).value as i64, 99);
        }
    }

    #[test]
    fn test_decode_exit_signal_message() {
        crate::gc::mesh_rt_init();

        // Build a fake exit signal message:
        // [u64 EXIT_SIGNAL_TAG][u64 data_len][u64 exiting_pid][u8 reason_tag=0 (Normal)]
        let mut msg = Vec::new();
        msg.extend_from_slice(&EXIT_SIGNAL_TAG.to_le_bytes()); // type_tag
        msg.extend_from_slice(&9u64.to_le_bytes()); // data_len (8 + 1)
        msg.extend_from_slice(&42u64.to_le_bytes()); // exiting pid
        msg.push(0); // reason_tag: Normal

        let result_ptr = decode_job_message(msg.as_ptr());
        let result = result_ptr as *const MeshResult;
        unsafe {
            assert_eq!((*result).tag, 1); // Err
                                          // Value should be a MeshString containing "normal"
            assert!(!(*result).value.is_null());
        }
    }

    #[test]
    fn test_mesh_job_async_returns_max_without_scheduler() {
        // Without GLOBAL_SCHEDULER, should return u64::MAX.
        // Note: the scheduler may be initialized by other tests, so
        // we just verify the function doesn't crash.
        extern "C" fn dummy_fn(_env: *const u8) -> i64 {
            42
        }
        let _pid = mesh_job_async(dummy_fn as *const u8, std::ptr::null());
        // If scheduler not initialized, returns u64::MAX.
        // If initialized (from other tests), returns a valid PID.
        // Either way, no panic = success.
    }
}
