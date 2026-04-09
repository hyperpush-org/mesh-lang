//! Integration tests for exhaustiveness/redundancy checking wired into case/match.
//!
//! These tests exercise:
//! - Non-exhaustive match produces a hard error (NonExhaustiveMatch)
//! - Exhaustive match produces no error
//! - Redundant arm produces a warning (RedundantArm)
//! - Guards excluded from exhaustiveness (guarded arms don't count)
//! - Guard expression validation (only comparisons, booleans, names, literals)
//! - Bool exhaustiveness
//! - Wildcard covers all

use mesh_typeck::error::TypeError;
use mesh_typeck::TypeckResult;

// ── Helpers ────────────────────────────────────────────────────────────

/// Parse Mesh source and run the type checker.
fn check_source(src: &str) -> TypeckResult {
    let parse = mesh_parser::parse(src);
    mesh_typeck::check(&parse)
}

/// Assert that the result has no errors.
fn assert_no_errors(result: &TypeckResult) {
    assert!(
        result.errors.is_empty(),
        "expected no errors, got: {:?}",
        result.errors
    );
}

/// Assert that the result has no warnings.
fn assert_no_warnings(result: &TypeckResult) {
    assert!(
        result.warnings.is_empty(),
        "expected no warnings, got: {:?}",
        result.warnings
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

/// Assert that the result contains a warning matching the given predicate.
fn assert_has_warning<F: Fn(&TypeError) -> bool>(result: &TypeckResult, pred: F, desc: &str) {
    assert!(
        result.warnings.iter().any(|e| pred(e)),
        "expected warning matching `{}`, got warnings: {:?}",
        desc,
        result.warnings
    );
}

// ── Non-exhaustive match produces hard error ──────────────────────────

/// A sum type match missing a variant should produce NonExhaustiveMatch.
#[test]
fn test_non_exhaustive_sum_type() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(_) -> 1\nend",
    );
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::NonExhaustiveMatch { .. }),
        "NonExhaustiveMatch (missing Point)",
    );
}

/// A Bool match missing `false` should produce NonExhaustiveMatch.
#[test]
fn test_non_exhaustive_bool() {
    let result = check_source(
        "let b = true\n\
         case b do\n  true -> 1\nend",
    );
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::NonExhaustiveMatch { .. }),
        "NonExhaustiveMatch (missing false)",
    );
}

// ── Exhaustive match produces no error ────────────────────────────────

/// A sum type match covering all variants should have no errors.
#[test]
fn test_exhaustive_sum_type() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(_) -> 1\n  Point -> 2\nend",
    );
    assert_no_errors(&result);
    assert_no_warnings(&result);
}

/// A Bool match covering both true and false should have no errors.
#[test]
fn test_exhaustive_bool() {
    let result = check_source(
        "let b = true\n\
         case b do\n  true -> 1\n  false -> 2\nend",
    );
    assert_no_errors(&result);
}

/// A wildcard arm makes any match exhaustive.
#[test]
fn test_wildcard_makes_exhaustive() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  _ -> 1\nend",
    );
    assert_no_errors(&result);
}

/// An integer match with wildcard is exhaustive.
#[test]
fn test_int_with_wildcard_exhaustive() {
    let result = check_source(
        "let x = 42\n\
         case x do\n  1 -> 10\n  2 -> 20\n  _ -> 0\nend",
    );
    assert_no_errors(&result);
}

/// An integer match without wildcard is NOT exhaustive.
#[test]
fn test_int_without_wildcard_non_exhaustive() {
    let result = check_source(
        "let x = 42\n\
         case x do\n  1 -> 10\n  2 -> 20\nend",
    );
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::NonExhaustiveMatch { .. }),
        "NonExhaustiveMatch (Int without wildcard)",
    );
}

// ── Redundant arm produces warning ────────────────────────────────────

/// A wildcard followed by a specific pattern: the second arm is redundant.
#[test]
fn test_redundant_arm_after_wildcard() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  _ -> 1\n  Circle(_) -> 2\nend",
    );
    assert_has_warning(
        &result,
        |e| matches!(e, TypeError::RedundantArm { .. }),
        "RedundantArm (arm after wildcard)",
    );
}

/// Duplicate pattern arms: the second identical arm is redundant.
#[test]
fn test_redundant_duplicate_arm() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(_) -> 1\n  Circle(_) -> 2\n  Point -> 3\nend",
    );
    assert_has_warning(
        &result,
        |e| matches!(e, TypeError::RedundantArm { arm_index, .. } if *arm_index == 1),
        "RedundantArm (duplicate Circle arm at index 1)",
    );
}

/// No redundancy when each arm covers a different variant.
#[test]
fn test_no_redundancy() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(_) -> 1\n  Point -> 2\nend",
    );
    assert_no_warnings(&result);
}

// ── Guards excluded from exhaustiveness ───────────────────────────────

/// A guarded arm is excluded from exhaustiveness checking.
/// So `Circle(_) when true -> 1` does NOT count as covering Circle.
#[test]
fn test_guarded_arm_excluded_from_exhaustiveness() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(_) when true -> 1\n  Point -> 2\nend",
    );
    // Circle is only covered by a guarded arm, so the match is non-exhaustive.
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::NonExhaustiveMatch { .. }),
        "NonExhaustiveMatch (guarded arm excluded)",
    );
}

/// A guarded arm with an unguarded fallback is exhaustive.
#[test]
fn test_guarded_with_unguarded_fallback_exhaustive() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(_) when true -> 1\n  Circle(_) -> 2\n  Point -> 3\nend",
    );
    assert_no_errors(&result);
}

// ── Guard expression validation ──────────────────────────────────────

/// Valid guard with comparison should not produce errors.
#[test]
fn test_valid_guard_comparison() {
    let result = check_source(
        "let x = 42\n\
         case x do\n  n when n > 0 -> 1\n  _ -> 0\nend",
    );
    assert_no_errors(&result);
}

/// Valid guard with boolean operator should not produce errors.
#[test]
fn test_valid_guard_boolean_op() {
    let result = check_source(
        "let x = 42\n\
         case x do\n  n when n > 0 and n < 100 -> 1\n  _ -> 0\nend",
    );
    assert_no_errors(&result);
}

// ── Variable binding in guard ────────────────────────────────────────

/// Guard can reference pattern bindings.
#[test]
fn test_guard_references_pattern_binding() {
    let result = check_source(
        "type Shape do\n  Circle(Float)\n  Point\nend\n\
         let s = Shape.Circle(5.0)\n\
         case s do\n  Circle(r) when r > 3.0 -> r\n  Circle(_) -> 0.0\n  Point -> 0.0\nend",
    );
    assert_no_errors(&result);
}

// ── Or-pattern exhaustiveness ────────────────────────────────────────

/// Or-pattern covering all variants is exhaustive.
#[test]
fn test_or_pattern_covers_all_variants() {
    let result = check_source(
        "type Color do\n  Red\n  Green\n  Blue\nend\n\
         let c = Color.Red\n\
         case c do\n  Red | Green | Blue -> 1\nend",
    );
    assert_no_errors(&result);
}
