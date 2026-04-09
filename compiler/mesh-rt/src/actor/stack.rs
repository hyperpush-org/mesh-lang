//! Corosensei-based stackful coroutine management for Mesh actors.
//!
//! Each actor runs as a stackful coroutine with a 64 KiB stack. The coroutine
//! yields when its reduction counter is exhausted, allowing the scheduler to
//! run other actors on the same OS thread.
//!
//! ## Thread-local State
//!
//! Three thread-locals track the current execution context:
//! - `CURRENT_YIELDER`: pointer to the active coroutine's Yielder (for yield on reduction exhaustion)
//! - `CURRENT_PID`: the PID of the currently running actor (for `mesh_actor_self()`)
//! - `STACK_BASE`: base address of the coroutine stack (for GC stack scanning bounds)

use corosensei::stack::DefaultStack;
use corosensei::{Coroutine, CoroutineResult, Yielder};

use super::process::{ProcessId, DEFAULT_STACK_SIZE};

use std::cell::Cell;

// ---------------------------------------------------------------------------
// Thread-local current-actor context
// ---------------------------------------------------------------------------

thread_local! {
    /// Raw pointer to the current coroutine's Yielder.
    ///
    /// Set before resuming a coroutine, cleared after it yields or completes.
    /// Used by `mesh_reduction_check()` to yield the current actor.
    ///
    /// Safety: The pointer is valid only while the coroutine is running.
    /// We store it as `*const ()` to erase the lifetime; the Yielder is
    /// borrowed from within the coroutine body and remains valid for the
    /// duration of that resume.
    pub static CURRENT_YIELDER: Cell<Option<*const ()>> = const { Cell::new(None) };

    /// PID of the currently executing actor on this thread.
    pub static CURRENT_PID: Cell<Option<ProcessId>> = const { Cell::new(None) };

    /// Base address of the current coroutine's stack (highest address).
    ///
    /// Captured at the very start of the coroutine body. The GC uses this
    /// as `stack_bottom` when scanning for roots (stack grows downward, so
    /// the base is the highest address).
    pub static STACK_BASE: Cell<*const u8> = const { Cell::new(std::ptr::null()) };
}

/// Set the current actor PID on this thread.
pub fn set_current_pid(pid: ProcessId) {
    CURRENT_PID.with(|c| c.set(Some(pid)));
}

/// Get the current actor PID on this thread.
pub fn get_current_pid() -> Option<ProcessId> {
    CURRENT_PID.with(|c| c.get())
}

/// Clear the current actor PID on this thread.
pub fn clear_current_pid() {
    CURRENT_PID.with(|c| c.set(None));
}

/// Get the base address of the current coroutine's stack.
///
/// Returns null if not running inside a coroutine.
pub fn get_stack_base() -> *const u8 {
    STACK_BASE.with(|c| c.get())
}

/// Set the base address of the current coroutine's stack.
pub fn set_stack_base(base: *const u8) {
    STACK_BASE.with(|c| c.set(base));
}

// ---------------------------------------------------------------------------
// Yield support
// ---------------------------------------------------------------------------

/// Yield the current coroutine (called from `mesh_reduction_check`).
///
/// After `suspend()` returns (coroutine is resumed), we re-install the yielder
/// into the thread-local because another coroutine may have run on this thread
/// in between and overwritten it.
///
/// # Safety
///
/// Must only be called from within a running coroutine (i.e., CURRENT_YIELDER
/// is set). Panics if called outside of a coroutine context.
pub fn yield_current() {
    CURRENT_YIELDER.with(|c| {
        let ptr = c
            .get()
            .expect("yield_current called outside of coroutine context");
        // Safety: The pointer is valid because we are inside the coroutine body
        // that set it, and the Yielder is borrowed for the duration of the body.
        let yielder: &Yielder<(), ()> = unsafe { &*(ptr as *const Yielder<(), ()>) };
        yielder.suspend(());
        // Re-install the yielder after resume. Another coroutine may have
        // overwritten the thread-local while we were suspended.
        c.set(Some(ptr));
    });
}

// ---------------------------------------------------------------------------
// CoroutineHandle
// ---------------------------------------------------------------------------

/// A handle wrapping a corosensei `Coroutine` for an actor.
///
/// The coroutine runs the actor's entry function on a dedicated 64 KiB stack.
/// It yields when its reduction counter is exhausted and resumes later.
///
/// `CoroutineHandle` is `!Send` because corosensei coroutines cannot be moved
/// across threads. The scheduler ensures coroutines stay on the thread that
/// created them.
pub struct CoroutineHandle {
    coro: Coroutine<(), (), ()>,
}

impl CoroutineHandle {
    /// Create a new coroutine that will call `entry_fn(args_ptr)`.
    ///
    /// The entry function signature matches the Mesh actor ABI:
    /// `extern "C" fn(args: *const u8)`.
    ///
    /// The coroutine installs its Yielder into the thread-local before
    /// calling the entry function, so `mesh_reduction_check()` can yield.
    pub fn new(entry_fn: *const u8, args_ptr: *const u8) -> Self {
        let stack =
            DefaultStack::new(DEFAULT_STACK_SIZE).expect("failed to allocate coroutine stack");

        // Capture the function pointer and args for the closure.
        let fn_ptr = entry_fn as usize;
        let args = args_ptr as usize;

        let coro = Coroutine::with_stack(stack, move |yielder: &Yielder<(), ()>, _input: ()| {
            // Capture the stack base at the very start of the coroutine body.
            // This local variable is near the base of the coroutine stack, so
            // its address serves as the upper bound for GC stack scanning.
            let stack_anchor: u64 = 0;
            let _ = std::hint::black_box(&stack_anchor);
            STACK_BASE.with(|c| {
                c.set(&stack_anchor as *const u64 as *const u8);
            });

            // Also store the stack base on the process for cross-context access.
            if let Some(pid) = CURRENT_PID.with(|c| c.get()) {
                if let Some(sched) = crate::actor::GLOBAL_SCHEDULER.get() {
                    if let Some(proc_arc) = sched.get_process(pid) {
                        proc_arc.lock().stack_base = &stack_anchor as *const u64 as *const u8;
                    }
                }
            }

            // Install yielder in thread-local so mesh_reduction_check can access it.
            CURRENT_YIELDER.with(|c| {
                c.set(Some(yielder as *const Yielder<(), ()> as *const ()));
            });

            // Call the actor entry function.
            // Safety: The fn_ptr was provided by the scheduler from a valid extern "C" fn.
            let func: extern "C" fn(*const u8) =
                unsafe { std::mem::transmute::<usize, extern "C" fn(*const u8)>(fn_ptr) };
            func(args as *const u8);

            // No need to clear CURRENT_YIELDER here -- the scheduler clears
            // the thread-local context after resume returns (whether yield or
            // completion). Clearing here would interfere with the next coroutine
            // on this thread if it ran between our yield and resume.
        });

        CoroutineHandle { coro }
    }

    /// Resume the coroutine.
    ///
    /// Returns `true` if the coroutine yielded (still has work to do),
    /// `false` if it completed (returned from entry function).
    pub fn resume(&mut self) -> bool {
        match self.coro.resume(()) {
            CoroutineResult::Yield(()) => true,
            CoroutineResult::Return(()) => false,
        }
    }

    /// Check whether the coroutine has finished.
    pub fn done(&self) -> bool {
        self.coro.done()
    }
}

impl std::fmt::Debug for CoroutineHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoroutineHandle")
            .field("done", &self.done())
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn test_coroutine_runs_to_completion() {
        // Use a test-specific counter to avoid interference from concurrent tests.
        static COMPLETION_COUNTER: AtomicU64 = AtomicU64::new(0);

        extern "C" fn completion_entry(_args: *const u8) {
            COMPLETION_COUNTER.fetch_add(1, Ordering::SeqCst);
        }

        COMPLETION_COUNTER.store(0, Ordering::SeqCst);
        let mut handle = CoroutineHandle::new(completion_entry as *const u8, std::ptr::null());

        let yielded = handle.resume();
        // Simple function should run to completion without yielding
        assert!(!yielded, "simple function should complete without yielding");
        assert!(handle.done());
        assert_eq!(COMPLETION_COUNTER.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_coroutine_yield_and_resume() {
        // Use a test-specific counter to avoid interference from concurrent tests.
        static YIELD_COUNTER: AtomicU64 = AtomicU64::new(0);

        extern "C" fn yield_entry(_args: *const u8) {
            YIELD_COUNTER.fetch_add(1, Ordering::SeqCst);
            yield_current();
            YIELD_COUNTER.fetch_add(1, Ordering::SeqCst);
        }

        YIELD_COUNTER.store(0, Ordering::SeqCst);
        let mut handle = CoroutineHandle::new(yield_entry as *const u8, std::ptr::null());

        // First resume: runs to yield point
        let yielded = handle.resume();
        assert!(yielded);
        assert!(!handle.done());
        assert_eq!(YIELD_COUNTER.load(Ordering::SeqCst), 1);

        // Second resume: completes
        let yielded = handle.resume();
        assert!(!yielded);
        assert!(handle.done());
        assert_eq!(YIELD_COUNTER.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_current_pid_thread_local() {
        assert!(get_current_pid().is_none());
        let pid = ProcessId::next();
        set_current_pid(pid);
        assert_eq!(get_current_pid().unwrap(), pid);
        clear_current_pid();
        assert!(get_current_pid().is_none());
    }
}
