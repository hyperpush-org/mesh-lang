//! End-to-end tests for the Phase 3 type system success criteria.
//!
//! These 5 tests verify the definitive success criteria for the entire
//! type system phase:
//!
//! 1. Let-polymorphism: identity function used at multiple types
//! 2. Occurs check: infinite type x(x) rejected
//! 3. Structs, Option, Result: struct definition/usage, Option?, Result!
//! 4. Traits: interface + impl + where clause enforcement
//! 5. Error locations: type errors carry source span information

use mesh_typeck::error::TypeError;
use mesh_typeck::ty::Ty;
use mesh_typeck::TypeckResult;

// ── Helpers ────────────────────────────────────────────────────────────

/// Parse Mesh source and run the type checker.
fn check_source(src: &str) -> TypeckResult {
    let parse = mesh_parser::parse(src);
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

// ── Success Criterion 1: Let-Polymorphism ──────────────────────────────

/// The identity function `let id = fn (x) -> x end` should be usable
/// at both Int and String types without error (let-polymorphism).
#[test]
fn test_success_criterion_1_let_polymorphism() {
    let result = check_source(
        "let id = fn (x) -> x end\n\
         let a = id(42)\n\
         let b = id(\"hello\")\n\
         b",
    );
    assert_result_type(&result, Ty::string());
}

/// Polymorphic `id` can be used with Bool too.
#[test]
fn test_success_criterion_1_let_polymorphism_bool() {
    let result = check_source(
        "let id = fn (x) -> x end\n\
         id(true)",
    );
    assert_result_type(&result, Ty::bool());
}

// ── Success Criterion 2: Occurs Check ──────────────────────────────────

/// The self-application `fn (x) -> x(x) end` should be rejected with
/// an InfiniteType error (occurs check).
#[test]
fn test_success_criterion_2_occurs_check() {
    let result = check_source("fn (x) -> x(x) end");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::InfiniteType { .. }),
        "InfiniteType",
    );
}

/// Verify the infinite type error message mentions the recursive type.
#[test]
fn test_success_criterion_2_occurs_check_diagnostic() {
    let src = "fn (x) -> x(x) end";
    let result = check_source(src);
    assert!(!result.errors.is_empty(), "expected an error");

    let rendered = result.render_errors(
        src,
        "test.mpl",
        &mesh_typeck::diagnostics::DiagnosticOptions::colorless(),
    );
    assert!(!rendered.is_empty(), "expected rendered diagnostics");
    let first = &rendered[0];
    assert!(
        first.contains("infinite type"),
        "expected 'infinite type' in error: {}",
        first
    );
}

// ── Success Criterion 3: Structs, Option, Result ───────────────────────

/// Struct definition + literal construction + field access.
#[test]
fn test_success_criterion_3_struct() {
    let result = check_source(
        "struct Point do\n  x :: Int\n  y :: Int\nend\n\
         let p = Point { x: 1, y: 2 }\n\
         p.x",
    );
    assert_result_type(&result, Ty::int());
}

/// Option type: Some wraps, None is polymorphic.
#[test]
fn test_success_criterion_3_option() {
    let result = check_source("Some(42)");
    assert_result_type(&result, Ty::option(Ty::int()));
}

/// Result type: Ok wraps a success value.
#[test]
fn test_success_criterion_3_result() {
    let result = check_source("Ok(42)");
    // Result<Int, ?N> -- the error type is a fresh variable
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
    let actual = result.result_type.as_ref().expect("expected a result type");
    let actual_str = format!("{}", actual);
    assert!(
        actual_str.starts_with("Result<Int"),
        "expected Result<Int, ...>, got `{}`",
        actual_str
    );
}

/// Option sugar: `Int?` type annotation resolves to Option<Int>.
#[test]
fn test_success_criterion_3_option_sugar() {
    let result = check_source("let x :: Int? = Some(42)\nx");
    assert_result_type(&result, Ty::option(Ty::int()));
}

// ── Success Criterion 4: Traits ────────────────────────────────────────

/// Interface definition + impl block should type-check without errors.
#[test]
fn test_success_criterion_4_traits_basic() {
    let result = check_source(
        "interface Printable do\n  fn to_string(self) -> String\nend\n\
         impl Printable for Int do\n  fn to_string(self) -> String do\n    \"int\"\n  end\nend",
    );
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
}

/// Where clause should enforce trait constraints at call sites.
/// Calling a function with `where T: Eq` on a type that implements Eq should succeed.
#[test]
fn test_success_criterion_4_where_clause_satisfied() {
    let result = check_source("fn check<T>(x :: T) -> T where T: Eq do\n  x\nend\ncheck(42)");
    assert_result_type(&result, Ty::int());
}

/// Missing impl for where clause should produce a TraitNotSatisfied error.
#[test]
fn test_success_criterion_4_where_clause_unsatisfied() {
    let result = check_source(
        "interface Serializable do\n  fn serialize(self) -> String\nend\n\
         fn save<T>(x :: T) -> T where T: Serializable do\n  x\nend\nsave(42)",
    );
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::TraitNotSatisfied { .. }),
        "TraitNotSatisfied",
    );
}

/// Compiler-known arithmetic operators should work via trait dispatch.
#[test]
fn test_success_criterion_4_operator_traits() {
    let result = check_source("1 + 2 * 3");
    assert_result_type(&result, Ty::int());
}

// ── Success Criterion 5: Error Locations ───────────────────────────────

/// Type errors should carry source span information that can be rendered
/// as ariadne diagnostics with labeled source locations.
#[test]
fn test_success_criterion_5_error_locations() {
    let src = "let x :: Int = \"hello\"";
    let result = check_source(src);

    // Should have a type mismatch error.
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::Mismatch { .. }),
        "Mismatch",
    );

    // Render the diagnostic and check it contains source location information.
    let rendered = result.render_errors(
        src,
        "test.mpl",
        &mesh_typeck::diagnostics::DiagnosticOptions::colorless(),
    );
    assert!(!rendered.is_empty(), "expected rendered diagnostics");
    let first = &rendered[0];

    // The diagnostic should show both types.
    assert!(
        first.contains("Int") && first.contains("String"),
        "expected both 'Int' and 'String' in diagnostic: {}",
        first
    );

    // The diagnostic should show a line/column label.
    assert!(
        first.contains("expected") && first.contains("found"),
        "expected 'expected' and 'found' labels in diagnostic: {}",
        first
    );
}

/// Error diagnostic for unbound variable should show the variable name
/// and its location in the source.
#[test]
fn test_success_criterion_5_unbound_var_location() {
    let src = "foo + bar";
    let result = check_source(src);

    assert_has_error(
        &result,
        |e| matches!(e, TypeError::UnboundVariable { name, .. } if name == "foo"),
        "UnboundVariable(foo)",
    );

    let rendered = result.render_errors(
        src,
        "test.mpl",
        &mesh_typeck::diagnostics::DiagnosticOptions::colorless(),
    );
    assert!(!rendered.is_empty());
    assert!(
        rendered[0].contains("foo"),
        "expected 'foo' in diagnostic: {}",
        rendered[0]
    );
}

/// Error diagnostic for arity mismatch should show expected vs found argument count.
#[test]
fn test_success_criterion_5_arity_location() {
    let src = "let f = fn (x, y) -> x end\nf(1)";
    let result = check_source(src);

    assert_has_error(
        &result,
        |e| {
            matches!(
                e,
                TypeError::ArityMismatch {
                    expected: 2,
                    found: 1,
                    ..
                }
            )
        },
        "ArityMismatch(2, 1)",
    );

    let rendered = result.render_errors(
        src,
        "test.mpl",
        &mesh_typeck::diagnostics::DiagnosticOptions::colorless(),
    );
    assert!(!rendered.is_empty());
    let first = &rendered[0];
    assert!(
        first.contains("2") && first.contains("1"),
        "expected '2' and '1' in arity diagnostic: {}",
        first
    );
}

// ── Phase 12-03: Pipe-Aware Call Inference ───────────────────────────────

/// Pipe with multi-arg function: `5 |> add(10)` should type check as `add(5, 10)`.
#[test]
fn test_pipe_call_arity() {
    let result = check_source(
        "fn add(x :: Int, y :: Int) -> Int do x + y end\n\
         5 |> add(10)",
    );
    assert!(
        result.errors.is_empty(),
        "expected no errors for pipe+call, got: {:?}",
        result.errors
    );
    assert_result_type(&result, Ty::int());
}

/// Pipe with closure arg: `5 |> apply(fn x -> x * 2 end)` should type check.
/// Uses untyped `f` parameter to let inference determine the closure type.
#[test]
fn test_pipe_call_with_closure() {
    let result = check_source(
        "fn apply(x, f) do f(x) end\n\
         5 |> apply(fn (x) -> x * 2 end)",
    );
    assert!(
        result.errors.is_empty(),
        "expected no errors for pipe+closure call, got: {:?}",
        result.errors
    );
    assert_result_type(&result, Ty::int());
}

/// Bare pipe (no call, just function ref) still works: `5 |> double`.
#[test]
fn test_pipe_bare_function_ref() {
    let result = check_source(
        "fn double(x :: Int) -> Int do x * 2 end\n\
         5 |> double",
    );
    assert!(
        result.errors.is_empty(),
        "expected no errors for bare pipe, got: {:?}",
        result.errors
    );
    assert_result_type(&result, Ty::int());
}
