//! Type checking tests for actor constructs: Pid<M>, spawn, send, receive, self(), link.
//!
//! These tests verify that:
//! - Pid<M> is a valid type with correct message type tracking
//! - spawn() returns Pid<M> where M is inferred from the actor
//! - send() validates message types at compile time for typed Pid<M>
//! - send() accepts any message for untyped Pid (escape hatch)
//! - receive infers message type from arm patterns
//! - self() returns Pid<M> inside actor, errors outside
//! - Typed Pid is assignable to untyped Pid

use mesh_typeck::error::TypeError;
use mesh_typeck::ty::Ty;
use mesh_typeck::TypeckResult;

// ── Helpers ────────────────────────────────────────────────────────────

fn check_source(src: &str) -> TypeckResult {
    let parse = mesh_parser::parse(src);
    mesh_typeck::check(&parse)
}

fn assert_no_errors(result: &TypeckResult) {
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
}

fn assert_has_error<F: Fn(&TypeError) -> bool>(result: &TypeckResult, pred: F, desc: &str) {
    assert!(
        result.errors.iter().any(|e| pred(e)),
        "expected error matching `{}`, got errors: {:?}",
        desc,
        result.errors
    );
}

// ── Pid Type Representation ────────────────────────────────────────────

#[test]
fn test_pid_type_constructors() {
    // Typed Pid<Int>
    let typed = Ty::pid(Ty::int());
    assert_eq!(format!("{}", typed), "Pid<Int>");

    // Untyped Pid
    let untyped = Ty::untyped_pid();
    assert_eq!(format!("{}", untyped), "Pid");
}

// ── Actor Definition ───────────────────────────────────────────────────

#[test]
fn test_actor_def_basic() {
    // Simple actor that receives messages and recurses with updated state.
    // The message type is inferred from the pattern usage (n used in + with Int state).
    let result = check_source(
        "actor counter(state :: Int) do\nreceive do\nn -> counter(state + n)\nend\nend",
    );
    assert_no_errors(&result);
}

#[test]
fn test_actor_def_registers_in_env() {
    // After defining an actor, we should be able to reference it as a function.
    let result = check_source(
        "actor counter(state :: Int) do\nreceive do\nn -> counter(state + n)\nend\nend\nlet f = counter",
    );
    assert_no_errors(&result);
}

// ── Spawn Returns Pid<M> ──────────────────────────────────────────────

#[test]
fn test_spawn_returns_pid() {
    let result = check_source(
        "actor counter(state :: Int) do\nreceive do\nn -> counter(state + n)\nend\nend\nlet p = spawn(counter, 0)\np",
    );
    assert_no_errors(&result);
    // The result type should be Pid<...>.
    let result_ty = result.result_type.as_ref().expect("expected result type");
    let ty_str = format!("{}", result_ty);
    assert!(
        ty_str.starts_with("Pid<"),
        "expected Pid<...>, got: {}",
        ty_str
    );
}

// ── Send Type Validation ──────────────────────────────────────────────

#[test]
fn test_send_typed_pid_correct_type() {
    // Send an Int to a Pid<Int> -- should succeed.
    let result = check_source(
        "actor counter(state :: Int) do\nreceive do\nn -> counter(state + n)\nend\nend\nlet pid = spawn(counter, 0)\nsend(pid, 42)",
    );
    assert_no_errors(&result);
}

#[test]
fn test_send_typed_pid_wrong_type() {
    // Send a String to a Pid<Int> -- should produce type error.
    let result = check_source(
        "actor counter(state :: Int) do\nreceive do\nn -> counter(state + n)\nend\nend\nlet pid = spawn(counter, 0)\nsend(pid, \"hello\")",
    );
    // Should produce a type error -- sending String to Pid<Int>.
    assert!(
        !result.errors.is_empty(),
        "expected type error for sending wrong type, got no errors"
    );
}

#[test]
fn test_send_untyped_pid_any_type() {
    // Untyped Pid should accept any message type (escape hatch).
    let result = check_source(
        "actor counter(state :: Int) do\nreceive do\nn -> counter(state + n)\nend\nend\nlet pid :: Pid = spawn(counter, 0)\nsend(pid, \"hello\")",
    );
    assert_no_errors(&result);
}

// ── Self Expression ───────────────────────────────────────────────────

#[test]
fn test_self_inside_actor() {
    let result = check_source(
        "actor pinger(state :: Int) do\nlet me = self()\nreceive do\nn -> pinger(state + n)\nend\nend",
    );
    assert_no_errors(&result);
}

#[test]
fn test_self_outside_actor_error() {
    let result = check_source("let me = self()");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::SelfOutsideActor { .. }),
        "SelfOutsideActor",
    );
}

// ── Receive Expression ────────────────────────────────────────────────

#[test]
fn test_receive_outside_actor_error() {
    let result = check_source("receive do\nn -> n\nend");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::ReceiveOutsideActor { .. }),
        "ReceiveOutsideActor",
    );
}

#[test]
fn test_receive_infers_message_type() {
    // The receive patterns should constrain the actor's message type.
    let result = check_source(
        "actor echo(state :: Int) do\nreceive do\nn -> echo(n)\nend\nend\nlet pid = spawn(echo, 0)\nsend(pid, 42)",
    );
    assert_no_errors(&result);
}

// ── Typed Pid Assignable to Untyped Pid ───────────────────────────────

#[test]
fn test_typed_pid_to_untyped_pid() {
    // Assigning a Pid<Int> to a variable annotated as Pid should work.
    let result = check_source(
        "actor counter(state :: Int) do\nreceive do\nn -> counter(state + n)\nend\nend\nlet typed_pid = spawn(counter, 0)\nlet untyped :: Pid = typed_pid",
    );
    assert_no_errors(&result);
}

// ── Link Expression ──────────────────────────────────────────────────

#[test]
fn test_link_returns_unit() {
    let result = check_source(
        "actor worker(state :: Int) do\nreceive do\nn -> worker(n)\nend\nend\nactor supervisor(state :: Int) do\nlet pid = spawn(worker, 0)\nlink(pid)\nreceive do\nn -> supervisor(n)\nend\nend",
    );
    assert_no_errors(&result);
}

// ── Unification: Pid Escape Hatch ────────────────────────────────────

#[test]
fn test_unify_pid_typed_with_untyped() {
    use mesh_typeck::error::ConstraintOrigin;
    use mesh_typeck::unify::InferCtx;

    let mut ctx = InferCtx::new();

    // Unifying Pid (untyped) with Pid<Int> (typed) should succeed.
    let untyped = Ty::untyped_pid();
    let typed = Ty::pid(Ty::int());
    assert!(
        ctx.unify(untyped, typed, ConstraintOrigin::Builtin).is_ok(),
        "untyped Pid should unify with Pid<Int>"
    );
}

#[test]
fn test_unify_pid_typed_with_untyped_reverse() {
    use mesh_typeck::error::ConstraintOrigin;
    use mesh_typeck::unify::InferCtx;

    let mut ctx = InferCtx::new();

    // Reverse direction: Pid<Int> with Pid should also succeed.
    let typed = Ty::pid(Ty::int());
    let untyped = Ty::untyped_pid();
    assert!(
        ctx.unify(typed, untyped, ConstraintOrigin::Builtin).is_ok(),
        "Pid<Int> should unify with untyped Pid"
    );
}

#[test]
fn test_unify_pid_typed_same_msg() {
    use mesh_typeck::error::ConstraintOrigin;
    use mesh_typeck::unify::InferCtx;

    let mut ctx = InferCtx::new();

    // Pid<Int> with Pid<Int> should succeed.
    let a = Ty::pid(Ty::int());
    let b = Ty::pid(Ty::int());
    assert!(
        ctx.unify(a, b, ConstraintOrigin::Builtin).is_ok(),
        "Pid<Int> should unify with Pid<Int>"
    );
}

#[test]
fn test_unify_pid_typed_different_msg() {
    use mesh_typeck::error::ConstraintOrigin;
    use mesh_typeck::unify::InferCtx;

    let mut ctx = InferCtx::new();

    // Pid<Int> with Pid<String> should fail.
    let a = Ty::pid(Ty::int());
    let b = Ty::pid(Ty::string());
    assert!(
        ctx.unify(a, b, ConstraintOrigin::Builtin).is_err(),
        "Pid<Int> should not unify with Pid<String>"
    );
}
