//! Integration tests for the Mesh type inference engine.
//!
//! These tests parse Mesh source code, run type checking via `mesh_typeck::check()`,
//! and assert on the inferred types and errors. They exercise the core behaviors
//! of Algorithm J inference: literals, let-bindings, let-polymorphism, occurs check,
//! if-branches, function application, closures, arithmetic, and error detection.

use mesh_typeck::error::TypeError;
use mesh_typeck::ty::Ty;
use mesh_typeck::TypeckResult;

// ── Helpers ────────────────────────────────────────────────────────────

/// Parse Mesh source and run the type checker.
fn check_source(src: &str) -> TypeckResult {
    let parse = mesh_parser::parse(src);
    // Uncomment to debug parse failures:
    // if !parse.ok() {
    //     panic!("parse errors: {:?}", parse.errors());
    // }
    mesh_typeck::check(&parse)
}

/// Assert that the result has no errors and the final expression type
/// matches the expected type.
fn assert_result_type(result: &TypeckResult, expected: Ty) {
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
    // The result_type field holds the type of the last expression in the program.
    let actual = result
        .result_type
        .as_ref()
        .expect("expected a result type from inference");
    let actual_str = format!("{}", actual);
    let expected_str = format!("{}", expected);
    assert_eq!(
        actual_str, expected_str,
        "expected type `{}`, got `{}`",
        expected_str, actual_str
    );
}

/// Assert that the result contains an error matching the given predicate.
fn assert_has_error<F: Fn(&TypeError) -> bool>(result: &TypeckResult, pred: F, desc: &str) {
    assert!(
        result.errors.iter().any(|e| pred(e)),
        "expected error matching `{}`, got errors: {:?}",
        desc,
        result.errors
    );
}

// ── Literal Inference ──────────────────────────────────────────────────

#[test]
fn test_integer_literal_is_int() {
    let result = check_source("42");
    assert_result_type(&result, Ty::int());
}

#[test]
fn test_float_literal_is_float() {
    let result = check_source("3.14");
    assert_result_type(&result, Ty::float());
}

#[test]
fn test_string_literal_is_string() {
    let result = check_source("\"hello\"");
    assert_result_type(&result, Ty::string());
}

#[test]
fn test_bool_literal_is_bool() {
    let result = check_source("true");
    assert_result_type(&result, Ty::bool());
}

// ── Let Binding Inference ──────────────────────────────────────────────

#[test]
fn test_let_binding_inference() {
    let result = check_source("let x = 42");
    // After `let x = 42`, the type of the binding should be Int.
    // The result type is the type of the last item/expression.
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    // x should be inferred as Int in the type table.
    // We check the result_type which should be the type of the let-binding's initializer.
    let ty = result.result_type.as_ref().expect("expected a result type");
    assert_eq!(format!("{}", ty), "Int");
}

#[test]
fn test_let_binding_with_usage() {
    let result = check_source("let x = 1\nx + 2");
    assert_result_type(&result, Ty::int());
}

// ── Function Inference ─────────────────────────────────────────────────

#[test]
fn test_function_identity() {
    let result = check_source("let id = fn (x) -> x end\nid(1)");
    assert_result_type(&result, Ty::int());
}

/// SUCCESS CRITERION #1: Let-polymorphism
/// The identity function can be used at multiple types.
#[test]
fn test_let_polymorphism() {
    let result = check_source("let id = fn (x) -> x end\nlet a = id(1)\nlet b = id(\"hello\")\nb");
    assert!(
        result.errors.is_empty(),
        "let-polymorphism should not produce errors, got: {:?}",
        result.errors
    );
    // b should be String (the last binding).
    assert_result_type(&result, Ty::string());
}

// ── Occurs Check ───────────────────────────────────────────────────────

/// SUCCESS CRITERION #2: Occurs check rejects self-application.
#[test]
fn test_occurs_check_rejection() {
    let result = check_source("fn (x) -> x(x) end");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::InfiniteType { .. }),
        "InfiniteType",
    );
}

// ── If Expression ──────────────────────────────────────────────────────

#[test]
fn test_if_branch_mismatch() {
    let result = check_source("if true do 1 else \"hello\" end");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::Mismatch { .. }),
        "Mismatch (if-branches)",
    );
}

#[test]
fn test_if_branches_same_type() {
    let result = check_source("if true do 1 else 2 end");
    assert_result_type(&result, Ty::int());
}

// ── Arity and Unbound Variable Errors ──────────────────────────────────

#[test]
fn test_function_application_wrong_arity() {
    let result = check_source("let f = fn (x, y) -> x end\nf(1)");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::ArityMismatch { .. }),
        "ArityMismatch",
    );
}

#[test]
fn test_unbound_variable() {
    let result = check_source("x + 1");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::UnboundVariable { name, .. } if name == "x"),
        "UnboundVariable(x)",
    );
}

// ── Arithmetic and Comparison ──────────────────────────────────────────

#[test]
fn test_arithmetic_int() {
    let result = check_source("1 + 2 * 3");
    assert_result_type(&result, Ty::int());
}

#[test]
fn test_comparison_returns_bool() {
    let result = check_source("1 < 2");
    assert_result_type(&result, Ty::bool());
}

// ── Nested Function Inference ──────────────────────────────────────────

#[test]
fn test_nested_function_inference() {
    let result = check_source("let apply = fn (f, x) -> f(x) end\napply(fn (n) -> n + 1 end, 42)");
    assert_result_type(&result, Ty::int());
}
