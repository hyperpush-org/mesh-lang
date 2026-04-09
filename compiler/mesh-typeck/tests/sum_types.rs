//! Integration tests for sum type (ADT) type checking.
//!
//! These tests exercise:
//! - Sum type definitions and variant constructor registration
//! - Qualified variant construction (Shape.Circle(5.0))
//! - Generic sum types (Option<T>, custom generics)
//! - Nullary constructors (Shape.Point)
//! - Constructor patterns in match arms
//! - Or-patterns and as-patterns
//! - Error cases: unknown variants, binding mismatches

use mesh_typeck::error::TypeError;
use mesh_typeck::ty::{Ty, TyCon};
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
#[allow(dead_code)]
fn assert_has_error<F: Fn(&TypeError) -> bool>(result: &TypeckResult, pred: F, desc: &str) {
    assert!(
        result.errors.iter().any(|e| pred(e)),
        "expected error matching `{}`, got errors: {:?}",
        desc,
        result.errors
    );
}

// ── Sum Type Registration ────────────────────────────────────────────

/// Test 1: Simple sum type definition registers and nullary constructor
/// resolves to the sum type.
#[test]
fn test_sum_type_nullary_constructor() {
    let result = check_source("type Color do\n  Red\n  Green\n  Blue\nend\nColor.Red");
    assert_result_type(
        &result,
        Ty::App(Box::new(Ty::Con(TyCon::new("Color"))), vec![]),
    );
}

/// Test 2: Sum type with positional field constructor.
/// Shape.Circle(5.0) should type-check to Shape.
#[test]
fn test_sum_type_positional_constructor() {
    let result = check_source("type Shape do\n  Circle(Float)\n  Point\nend\nShape.Circle(5.0)");
    assert_result_type(
        &result,
        Ty::App(Box::new(Ty::Con(TyCon::new("Shape"))), vec![]),
    );
}

/// Test 3: Generic sum type -- Option.Some(42) should infer Option<Int>.
#[test]
fn test_generic_sum_type_constructor() {
    let result =
        check_source("type MyOption<T> do\n  MySome(T)\n  MyNone\nend\nMyOption.MySome(42)");
    assert_result_type(&result, Ty::option_like("MyOption", Ty::int()));
}

/// Test 4: Nullary generic constructor -- MyOption.MyNone with annotation.
#[test]
fn test_generic_sum_type_nullary_with_annotation() {
    let result = check_source(
        "type MyOption<T> do\n  MySome(T)\n  MyNone\nend\nlet x :: MyOption<Int> = MyOption.MyNone\nx",
    );
    assert_result_type(&result, Ty::option_like("MyOption", Ty::int()));
}

/// Test 5: Unqualified constructor access (e.g. Some(42) from builtins still works).
#[test]
fn test_builtin_option_still_works() {
    let result = check_source("Some(42)");
    assert_result_type(&result, Ty::option(Ty::int()));
}

/// Test 6: Wrong argument type to variant constructor produces error.
#[test]
fn test_sum_type_wrong_arg_type() {
    let result = check_source("type Shape do\n  Circle(Float)\nend\nShape.Circle(\"bad\")");
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::Mismatch { .. }),
        "Mismatch (wrong constructor arg type)",
    );
}

/// Test 7: Multiple field constructor.
#[test]
fn test_sum_type_multiple_fields() {
    let result =
        check_source("type Shape do\n  Rect(Float, Float)\n  Point\nend\nShape.Rect(3.0, 4.0)");
    assert_result_type(
        &result,
        Ty::App(Box::new(Ty::Con(TyCon::new("Shape"))), vec![]),
    );
}

// ── Constructor Pattern Tests ─────────────────────────────────────────

/// Test 8: Case match on sum type with constructor patterns.
/// The constructor pattern `Circle(r)` binds `r` to the Float field.
#[test]
fn test_sum_type_case_match() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(r) -> r\n  Point -> 0.0\nend",
    );
    assert_result_type(&result, Ty::float());
}

/// Test 9: Constructor pattern with qualified name in case arm.
#[test]
fn test_sum_type_qualified_constructor_pattern() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Shape.Circle(r) -> r\n  Shape.Point -> 0.0\nend",
    );
    assert_result_type(&result, Ty::float());
}

/// Test 10: Nested constructor patterns.
#[test]
fn test_nested_constructor_pattern() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\nend\n\
         let s = Some(Shape.Circle(3.14))\n\
         case s do\n  Some(Circle(r)) -> r\n  None -> 0.0\nend",
    );
    assert_result_type(&result, Ty::float());
}

/// Test 11: Constructor pattern with wildcard sub-pattern.
#[test]
fn test_constructor_pattern_with_wildcard() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(_) -> 1\n  Point -> 2\nend",
    );
    assert_result_type(&result, Ty::int());
}

/// Test 12: Unknown variant in constructor pattern produces error.
#[test]
fn test_unknown_variant_pattern() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Triangle(a) -> a\nend",
    );
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::UnknownVariant { .. }),
        "UnknownVariant",
    );
}

// ── As Pattern Tests ─────────────────────────────────────────────────

/// Test 13: As-pattern binds the whole matched value.
#[test]
fn test_as_pattern_binds_whole_value() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(r) as shape -> r\n  Point -> 0.0\nend",
    );
    assert_result_type(&result, Ty::float());
}

/// Test 14: As-pattern with literal.
#[test]
fn test_as_pattern_with_literal() {
    let result = check_source(
        "let x = 42\n\
         case x do\n  n as val -> val\nend",
    );
    assert_result_type(&result, Ty::int());
}

// ── Or Pattern Tests ─────────────────────────────────────────────────

/// Test 15: Or-pattern with nullary constructors.
#[test]
fn test_or_pattern_nullary() {
    let result = check_source(
        "type Color do\n  Red\n  Green\n  Blue\nend\n\
         let c = Color.Red\n\
         case c do\n  Red | Green -> 1\n  Blue -> 2\nend",
    );
    assert_result_type(&result, Ty::int());
}

/// Test 16: Or-pattern with literal alternatives.
#[test]
fn test_or_pattern_literals() {
    let result = check_source(
        "let x = 1\n\
         case x do\n  1 | 2 | 3 -> true\n  _ -> false\nend",
    );
    assert_result_type(&result, Ty::bool());
}

// ── Phase 4 End-to-End: Option/Result as Sum Types ──────────────────

/// Test 17: Option is registered as a proper sum type.
/// Some(42) and None both type-check, and qualified access works.
#[test]
fn test_option_is_sum_type() {
    // Unqualified Some(42) still works
    let result = check_source("Some(42)");
    assert_result_type(&result, Ty::option(Ty::int()));

    // Qualified Option.Some(42) also works
    let result2 = check_source("Option.Some(42)");
    assert_result_type(&result2, Ty::option(Ty::int()));

    // None still works
    let result3 = check_source("let x :: Option<Int> = None\nx");
    assert_result_type(&result3, Ty::option(Ty::int()));

    // Qualified Option.None works
    let result4 = check_source("let x :: Option<Int> = Option.None\nx");
    assert_result_type(&result4, Ty::option(Ty::int()));
}

/// Test 18: Result is registered as a proper sum type.
/// Ok(42) and Err("oops") both type-check, and qualified access works.
#[test]
fn test_result_is_sum_type() {
    // Unqualified Ok(42) still works
    let result = check_source("Ok(42)");
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
    let actual_str = format!("{}", result.result_type.as_ref().unwrap());
    assert!(
        actual_str.starts_with("Result<Int"),
        "expected Result<Int, ...>, got `{}`",
        actual_str
    );

    // Qualified Result.Ok(42) works
    let result2 = check_source("Result.Ok(42)");
    assert!(
        result2.errors.is_empty(),
        "expected no errors, got: {:?}",
        result2.errors
    );
    let actual_str2 = format!("{}", result2.result_type.as_ref().unwrap());
    assert!(
        actual_str2.starts_with("Result<Int"),
        "expected Result<Int, ...>, got `{}`",
        actual_str2
    );

    // Qualified Result.Err("oops") works
    let result3 = check_source("Result.Err(\"oops\")");
    assert!(
        result3.errors.is_empty(),
        "expected no errors, got: {:?}",
        result3.errors
    );
    let actual_str3 = format!("{}", result3.result_type.as_ref().unwrap());
    assert!(
        actual_str3.contains("Result"),
        "expected Result<...>, got `{}`",
        actual_str3
    );
}

/// Test 19: Option pattern matching (Some + None = exhaustive).
#[test]
fn test_option_exhaustive_pattern_match() {
    let result = check_source(
        "let opt = Some(42)\n\
         case opt do\n  Some(x) -> x\n  None -> 0\nend",
    );
    assert_result_type(&result, Ty::int());
}

/// Test 20: Result pattern matching (Ok + Err = exhaustive).
#[test]
fn test_result_exhaustive_pattern_match() {
    let result = check_source(
        "let r = Ok(42)\n\
         case r do\n  Ok(x) -> x\n  Err(e) -> 0\nend",
    );
    assert_result_type(&result, Ty::int());
}

/// Test 21: Full sum type lifecycle -- define, construct, match, extract.
#[test]
fn test_sum_type_full_lifecycle() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Rect(Float, Float)\n  Point\nend\n\
         let s = Shape.Circle(3.14)\n\
         case s do\n\
           Circle(r) -> r\n\
           Rect(w, h) -> w\n\
           Point -> 0.0\n\
         end",
    );
    assert_result_type(&result, Ty::float());
}

/// Test 22: Generic sum type with nested pattern matching.
#[test]
fn test_generic_sum_type_nested_patterns() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Some(Shape.Circle(3.14))\n\
         case s do\n\
           Some(Circle(r)) -> r\n\
           Some(Point) -> 0.0\n\
           None -> 0.0\n\
         end",
    );
    assert_result_type(&result, Ty::float());
}

/// Test 23: Or-pattern in sum type match.
#[test]
fn test_or_pattern_sum_type() {
    let result = check_source(
        "type Color do\n  Red\n  Green\n  Blue\nend\n\
         let c = Color.Red\n\
         case c do\n\
           Red | Green | Blue -> 1\n\
         end",
    );
    assert_result_type(&result, Ty::int());
}

/// Test 24: Option sugar (Int?) still works with sum type registration.
#[test]
fn test_option_sugar_with_sum_types() {
    // Int? = Option<Int>
    let result = check_source("let x :: Int? = Some(42)\nx");
    assert_result_type(&result, Ty::option(Ty::int()));
}

/// Test 25: Existing Phase 3 Option/Result tests still pass.
/// This is a regression check for the migration.
#[test]
fn test_option_result_backward_compat() {
    // Some(42) -> Option<Int>
    let result = check_source("Some(42)");
    assert_result_type(&result, Ty::option(Ty::int()));

    // None with annotation -> Option<Int>
    let result2 = check_source("let x :: Option<Int> = None\nx");
    assert_result_type(&result2, Ty::option(Ty::int()));

    // Ok(42) -> Result<Int, ?>
    let result3 = check_source("Ok(42)");
    assert!(result3.errors.is_empty());

    // Err("bad") -> Result<?, String>
    let result4 = check_source("Err(\"bad\")");
    assert!(result4.errors.is_empty());
}

/// Test 26: Nested generic sum types work correctly.
/// Option<Option<Int>> with nested pattern matching.
#[test]
fn test_nested_generic_sum_types() {
    let result = check_source(
        "let x = Some(Some(42))\n\
         case x do\n\
           Some(Some(n)) -> n\n\
           Some(None) -> 0\n\
           None -> 0\n\
         end",
    );
    assert_result_type(&result, Ty::int());
}

// ── Helper for generic non-builtin types ──────────────────────────────

trait TyExt {
    fn option_like(name: &str, inner: Ty) -> Ty;
}

impl TyExt for Ty {
    fn option_like(name: &str, inner: Ty) -> Ty {
        Ty::App(Box::new(Ty::Con(TyCon::new(name))), vec![inner])
    }
}
