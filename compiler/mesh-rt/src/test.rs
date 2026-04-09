//! Test runtime support functions for the Mesh testing framework (Phase 138).
//!
//! Provides `extern "C"` functions called by compiled `*.test.mpl` programs.
//! The test harness (lowered by Plan 03) calls these functions to:
//!   - Begin each test (`mesh_test_begin`)
//!   - Record pass/fail outcomes (`mesh_test_pass`, `mesh_test_fail_msg`)
//!   - Assert conditions (`mesh_test_assert`, `mesh_test_assert_eq`,
//!     `mesh_test_assert_ne`, `mesh_test_assert_raises`)
//!   - Print summary at the end of the run (`mesh_test_summary`)
//!   - Clean up mock actors (`mesh_test_cleanup_actors`)
//!
//! ## State model
//!
//! All state is kept in `thread_local!` statics. Tests run single-threaded
//! from the generated `main`, so no locking is required.
//!
//! ## Failure output
//!
//! Failures print inline as each test fails, and are also accumulated in
//! `FAIL_MESSAGES`. `mesh_test_summary` reprints all failures in a
//! `Failures:` section before the final count line.
//!
//! ## assert_raises mechanism
//!
//! `mesh_test_assert_raises` uses a flag-based mechanism to detect whether
//! a closure "raised" (i.e., triggered a failing assertion). This avoids
//! panicking through `extern "C"` closures, which aborts in Rust 1.73+.
//! See the `IN_ASSERT_RAISES` / `ASSERT_RAISES_TRIGGERED` thread-locals.

use std::cell::{Cell, RefCell};
use std::io::Write as _;

use crate::string::MeshString;

// ── ANSI color codes ─────────────────────────────────────────────────────────

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

// ── Per-process test state ────────────────────────────────────────────────────

thread_local! {
    static PASS_COUNT: Cell<i64> = Cell::new(0);
    static FAIL_COUNT: Cell<i64> = Cell::new(0);
    static CURRENT_TEST: RefCell<String> = RefCell::new(String::new());
    static QUIET_MODE: Cell<bool> = Cell::new(false);
    /// Accumulates failure messages for the end-of-run `Failures:` reprint.
    static FAIL_MESSAGES: RefCell<Vec<String>> = RefCell::new(Vec::new());
    /// Pids of mock actors spawned during the run; drained by cleanup_actors.
    static MOCK_ACTOR_PIDS: RefCell<Vec<i64>> = RefCell::new(Vec::new());

    // ── assert_raises flag-based mechanism ───────────────────────────────────
    //
    // When mesh_test_assert_raises calls a closure, it sets IN_ASSERT_RAISES.
    // mesh_test_assert checks this flag:
    //   - If IN_ASSERT_RAISES is true: set ASSERT_RAISES_TRIGGERED = true and
    //     return normally (do NOT record a failure or panic).
    //   - If IN_ASSERT_RAISES is false: record failure normally and return.
    //
    // This avoids panicking through extern "C" closures (which would abort in
    // Rust 1.73+). Test bodies do not early-exit on failure; all assertions
    // in a body run to completion.
    static IN_ASSERT_RAISES: Cell<bool> = Cell::new(false);
    static ASSERT_RAISES_TRIGGERED: Cell<bool> = Cell::new(false);
}

// ── Helper: read a MeshString as a &str ──────────────────────────────────────

/// Extract a `&str` from a pointer to a `MeshString`.
///
/// # Safety
///
/// `s` must be a valid, non-null pointer to an initialised `MeshString`
/// whose data bytes are valid UTF-8.
unsafe fn mesh_str<'a>(s: *const MeshString) -> &'a str {
    (*s).as_str()
}

/// Helper: create a Rust `String` failure message and forward it to
/// `mesh_test_fail_msg` via a temporary `MeshString` on the stack.
///
/// This is used by the assert helpers to avoid duplicating the
/// fail-msg accumulation / print logic.
fn fail_with(msg: &str) {
    // We need a MeshString to pass to mesh_test_fail_msg.
    // Allocate one from the GC so the pointer remains valid.
    let ms = crate::string::mesh_string_new(msg.as_ptr(), msg.len() as u64);
    unsafe { mesh_test_fail_msg(ms) };
}

// ── Public extern "C" functions ───────────────────────────────────────────────

/// Called by the test harness immediately before each test body.
///
/// Stores the test name for subsequent `pass`/`fail_msg` calls and,
/// in verbose mode, prints `"  running: <name>"` (no trailing newline)
/// so that the pass/fail line can overwrite it with `\r`.
#[no_mangle]
pub extern "C" fn mesh_test_begin(name: *const MeshString) {
    let name_str = unsafe { mesh_str(name) }.to_owned();
    CURRENT_TEST.with(|ct| *ct.borrow_mut() = name_str.clone());

    if !QUIET_MODE.with(|q| q.get()) {
        print!("  {DIM}running:{RESET} {name_str}");
        let _ = std::io::stdout().flush();
    }
}

/// Called by the test harness after the test body returns without panicking.
///
/// Increments `PASS_COUNT` and, in verbose mode, overwrites the
/// `running:` line with a green checkmark.
#[no_mangle]
pub extern "C" fn mesh_test_pass() {
    PASS_COUNT.with(|c| c.set(c.get() + 1));

    if !QUIET_MODE.with(|q| q.get()) {
        let name = CURRENT_TEST.with(|ct| ct.borrow().clone());
        println!("\r  {GREEN}✓{RESET} {name}");
    }
}

/// Called when an assertion fails (or when the test harness catches a panic
/// from one of the assert helpers).
///
/// Increments `FAIL_COUNT`, prints the failure inline (verbose mode), and
/// accumulates the failure message for the end-of-run `Failures:` section.
#[no_mangle]
pub unsafe extern "C" fn mesh_test_fail_msg(msg: *const MeshString) {
    FAIL_COUNT.with(|c| c.set(c.get() + 1));
    let msg_str = mesh_str(msg).to_owned();
    let name = CURRENT_TEST.with(|ct| ct.borrow().clone());

    if !QUIET_MODE.with(|q| q.get()) {
        println!("\r  {RED}✗{RESET} {name}");
        println!("    {RED}{msg_str}{RESET}");
    }

    let entry = format!("  {RED}{BOLD}✗{RESET} {name}\n    {RED}{msg_str}{RESET}");
    FAIL_MESSAGES.with(|fm| fm.borrow_mut().push(entry));
}

/// Assert that `cond` is non-zero.
///
/// On failure, records the failure message (unless inside `assert_raises`).
///
/// Note: this function does NOT panic on failure. Panicking through
/// `extern "C"` Mesh closures is undefined behaviour (hard abort in Rust
/// 1.73+). Instead, failures are recorded via the fail-count / FAIL_MESSAGES
/// state; test bodies continue running after a failed assert.
///
/// When called from inside `mesh_test_assert_raises`, the failure is
/// silently intercepted: `ASSERT_RAISES_TRIGGERED` is set and no failure
/// is recorded, allowing `assert_raises` to verify that the closure "raised".
#[no_mangle]
pub unsafe extern "C" fn mesh_test_assert(
    cond: i8,
    expr_src: *const MeshString,
    _file: *const u8,
    _file_len: i64,
    _line: i64,
) {
    if cond == 0 {
        if IN_ASSERT_RAISES.with(|f| f.get()) {
            // We are inside an assert_raises closure. Signal that a raise
            // occurred without recording it as a test failure.
            ASSERT_RAISES_TRIGGERED.with(|f| f.set(true));
            return;
        }
        let src = mesh_str(expr_src);
        let msg = format!("assert failed: {src}");
        fail_with(&msg);
    }
}

/// Assert that `lhs` and `rhs` (already converted to strings by the lowerer)
/// are equal. Fails with an `expected`/`actual` diagnostic.
///
/// Does NOT panic on failure (see `mesh_test_assert` for rationale).
#[no_mangle]
pub unsafe extern "C" fn mesh_test_assert_eq(
    lhs: *const MeshString,
    rhs: *const MeshString,
    expr_src: *const MeshString,
    _file: *const u8,
    _file_len: i64,
    _line: i64,
) {
    let l = mesh_str(lhs);
    let r = mesh_str(rhs);
    if l != r {
        if IN_ASSERT_RAISES.with(|f| f.get()) {
            ASSERT_RAISES_TRIGGERED.with(|f| f.set(true));
            return;
        }
        let src = mesh_str(expr_src);
        let msg = format!("assert_eq failed: {src}\n  left:  {l}\n  right: {r}");
        fail_with(&msg);
    }
}

/// Assert that `lhs` and `rhs` are NOT equal. Fails when they are equal.
///
/// Does NOT panic on failure (see `mesh_test_assert` for rationale).
#[no_mangle]
pub unsafe extern "C" fn mesh_test_assert_ne(
    lhs: *const MeshString,
    rhs: *const MeshString,
    expr_src: *const MeshString,
    _file: *const u8,
    _file_len: i64,
    _line: i64,
) {
    let l = mesh_str(lhs);
    let r = mesh_str(rhs);
    if l == r {
        if IN_ASSERT_RAISES.with(|f| f.get()) {
            ASSERT_RAISES_TRIGGERED.with(|f| f.set(true));
            return;
        }
        let src = mesh_str(expr_src);
        let msg = format!("assert_ne failed: {src}\n  both sides equal: {l}");
        fail_with(&msg);
    }
}

/// Assert that calling the closure `fn_ptr(env_ptr)` raises (i.e., triggers a
/// failing assertion inside the closure body).
///
/// This uses a flag-based mechanism rather than panic/catch_unwind, because
/// Mesh closures are compiled with `extern "C"` ABI and panicking through an
/// `extern "C"` boundary causes an abort in Rust 1.73+.
///
/// Mechanism:
/// 1. Set `IN_ASSERT_RAISES = true` before calling the closure.
/// 2. `mesh_test_assert` (and eq/ne variants) check this flag: when true they
///    set `ASSERT_RAISES_TRIGGERED = true` and return without recording failure.
/// 3. After the closure returns, check `ASSERT_RAISES_TRIGGERED`.
///    - If triggered → passes (the closure "raised" as expected).
///    - If not triggered → records a test failure.
///
/// The closure ABI matches the Mesh runtime closure convention:
/// `extern "C" fn(*const u8) -> i64`.
#[no_mangle]
pub unsafe extern "C" fn mesh_test_assert_raises(
    fn_ptr: *const u8,
    env_ptr: *const u8,
    _file: *const u8,
    _file_len: i64,
    _line: i64,
) {
    // Save previous state in case of nested assert_raises calls.
    let prev_in_raises = IN_ASSERT_RAISES.with(|f| f.get());
    let prev_triggered = ASSERT_RAISES_TRIGGERED.with(|f| f.get());

    IN_ASSERT_RAISES.with(|f| f.set(true));
    ASSERT_RAISES_TRIGGERED.with(|f| f.set(false));

    let f: extern "C" fn(*const u8) -> i64 = std::mem::transmute(fn_ptr);
    f(env_ptr);

    let triggered = ASSERT_RAISES_TRIGGERED.with(|f| f.get());

    // Restore previous state.
    IN_ASSERT_RAISES.with(|f| f.set(prev_in_raises));
    ASSERT_RAISES_TRIGGERED.with(|f| f.set(prev_triggered));

    if !triggered {
        // The closure returned normally without triggering any assertion failure.
        let msg = "assert_raises failed: expression did not raise";
        fail_with(msg);
    }
    // If triggered: the closure raised as expected — no failure to record.
}

/// Print the run summary and exit the process with the appropriate code.
///
/// First reprints all accumulated failure messages in a `Failures:` section,
/// then prints the final `N passed` / `N failed, M passed` count line.
///
/// Exits with code `0` when all tests passed, `1` when any tests failed.
/// This lets the outer `meshc test` runner detect test failures via exit code.
///
/// The harness (`Plan 03`) passes the elapsed time as milliseconds.
#[no_mangle]
pub extern "C" fn mesh_test_summary(passed: i64, failed: i64, elapsed_ms: i64) {
    // Reprint accumulated failures at the bottom of the run.
    FAIL_MESSAGES.with(|fm| {
        let messages = fm.borrow();
        if !messages.is_empty() {
            println!("\n{BOLD}Failures:{RESET}");
            for msg in messages.iter() {
                println!("{msg}");
            }
        }
    });

    let elapsed = elapsed_ms as f64 / 1000.0;
    if failed > 0 {
        println!("\n{RED}{BOLD}{failed} failed{RESET}, {passed} passed in {elapsed:.2}s");
        std::process::exit(1);
    } else {
        println!("\n{GREEN}{BOLD}{passed} passed{RESET} in {elapsed:.2}s");
        std::process::exit(0);
    }
}

/// Clean up any mock actors registered during the test run.
///
/// Drains `MOCK_ACTOR_PIDS` and calls `mesh_actor_exit` for each Pid.
/// Plan 03 populates `MOCK_ACTOR_PIDS` when `Test.mock_actor` is called.
#[no_mangle]
pub extern "C" fn mesh_test_cleanup_actors() {
    let pids: Vec<i64> = MOCK_ACTOR_PIDS.with(|p| std::mem::take(&mut *p.borrow_mut()));
    for pid in pids {
        // Reason tag 0 = normal exit (same convention as actor/mod.rs).
        crate::actor::mesh_actor_exit(pid as u64, 0);
    }
}

/// Register a mock actor Pid for cleanup at the end of the run.
///
/// Called by the Plan 03 `Test.mock_actor` implementation.
#[allow(dead_code)]
pub fn register_mock_actor_pid(pid: i64) {
    MOCK_ACTOR_PIDS.with(|p| p.borrow_mut().push(pid));
}

/// Return the current pass count (for use in test harness summary).
#[no_mangle]
pub extern "C" fn mesh_test_pass_count() -> i64 {
    PASS_COUNT.with(|c| c.get())
}

/// Return the current fail count (for use in test harness summary).
#[no_mangle]
pub extern "C" fn mesh_test_fail_count() -> i64 {
    FAIL_COUNT.with(|c| c.get())
}

/// Run a test body closure and record the pass/fail outcome.
///
/// The test harness calls this for every `test(...)` block. The closure
/// ABI matches the Mesh runtime closure convention:
/// `extern "C" fn(*const u8) -> i64`.
///
/// Outcome detection uses the fail-count snapshot approach:
/// - Record `FAIL_COUNT` before calling the closure.
/// - Call the closure (any assert failures increment `FAIL_COUNT` directly).
/// - After the closure returns, if `FAIL_COUNT` increased → a test failure
///   was recorded; otherwise call `mesh_test_pass()`.
///
/// This avoids panicking through `extern "C"` closures (which aborts in
/// Rust 1.73+). The trade-off is that all assertions in a test body run to
/// completion even after a failure (no early-exit on first assert failure).
#[no_mangle]
pub unsafe extern "C" fn mesh_test_run_body(fn_ptr: *const u8, env_ptr: *const u8) {
    let fail_before = FAIL_COUNT.with(|c| c.get());

    let f: extern "C" fn(*const u8) -> i64 = std::mem::transmute(fn_ptr);
    f(env_ptr);

    let fail_after = FAIL_COUNT.with(|c| c.get());
    if fail_after == fail_before {
        mesh_test_pass();
    }
    // If fail_after > fail_before: a failure was already recorded by an assert
    // helper; mesh_test_fail_msg already incremented FAIL_COUNT. No extra
    // counting needed here.
}

/// Spawn a mock actor whose body is the given closure.
///
/// The spawned actor runs the closure for every message it receives.
/// The Pid is tracked in `MOCK_ACTOR_PIDS` for cleanup between tests.
///
/// Closure ABI: `extern "C" fn(env_ptr: *const u8) -> i64`.
///
/// Requires `mesh_rt_init_actor` to have been called first.
#[no_mangle]
pub unsafe extern "C" fn mesh_test_mock_actor(fn_ptr: *const u8, env_ptr: *const u8) -> i64 {
    // Build a small args block: {fn_ptr, env_ptr} so the spawned actor
    // has access to the closure. The actor entry function is the standard
    // closure dispatch shim at mesh_actor_closure_runner (from actor/mod.rs).
    // Since we don't have a dedicated closure-runner entry point exposed here,
    // we use mesh_actor_spawn with the fn_ptr directly.
    //
    // Pack fn_ptr and env_ptr into a heap-allocated args block.
    #[repr(C)]
    struct MockArgs {
        fn_ptr: *const u8,
        env_ptr: *const u8,
    }

    // Leak args so the actor thread can read them.
    let args = Box::new(MockArgs { fn_ptr, env_ptr });
    let args_ptr = Box::into_raw(args) as *const u8;
    let args_size = std::mem::size_of::<MockArgs>() as u64;

    // The actor entry function: reads MockArgs and calls fn_ptr(env_ptr) in a loop.
    // We use a generic wrapper defined below.
    extern "C" fn mock_actor_entry(args: *const u8) {
        unsafe {
            let mock_args = &*(args as *const MockArgs);
            let f: extern "C" fn(*const u8) -> i64 = std::mem::transmute(mock_args.fn_ptr);
            // Run the closure once per message received.
            // In a real implementation this would loop on receive; for test mocks
            // we run the closure once and exit.
            loop {
                let msg_ptr = crate::actor::mesh_actor_receive(100); // 100ms timeout
                if msg_ptr.is_null() {
                    break; // no message in time window — exit actor
                }
                f(mock_args.env_ptr);
            }
        }
    }

    let pid = crate::actor::mesh_actor_spawn(
        mock_actor_entry as *const u8,
        args_ptr,
        args_size,
        1, // Normal priority
    ) as i64;

    MOCK_ACTOR_PIDS.with(|p| p.borrow_mut().push(pid));
    pid
}
