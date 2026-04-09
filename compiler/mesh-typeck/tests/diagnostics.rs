//! Snapshot tests for Mesh type error diagnostics.
//!
//! Each test triggers a specific type error, renders it through the ariadne
//! diagnostic pipeline, and snapshots the output with insta. These verify
//! that error messages are terse, include dual-span labels, and show fix
//! suggestions when plausible.

use mesh_typeck::diagnostics::{render_diagnostic, DiagnosticOptions};
use mesh_typeck::error::TypeError;
use mesh_typeck::TypeckResult;

// ── Helpers ────────────────────────────────────────────────────────────

/// Colorless options for deterministic snapshot output.
fn opts() -> DiagnosticOptions {
    DiagnosticOptions::colorless()
}

/// Parse Mesh source and run the type checker.
fn check_source(src: &str) -> TypeckResult {
    let parse = mesh_parser::parse(src);
    mesh_typeck::check(&parse)
}

/// Render the first error from a type check result as a diagnostic string.
fn render_first_error(src: &str) -> String {
    let result = check_source(src);
    assert!(
        !result.errors.is_empty(),
        "expected at least one error for source: {:?}",
        src
    );
    render_diagnostic(&result.errors[0], src, "test.mpl", &opts(), None)
}

/// Render all errors from a type check result as diagnostic strings.
fn render_all_errors(src: &str) -> Vec<String> {
    let result = check_source(src);
    result.render_errors(src, "test.mpl", &opts())
}

// ── Diagnostic Snapshot Tests ──────────────────────────────────────────

/// Type mismatch: annotation says Int but expression is a String.
#[test]
fn test_diag_type_mismatch() {
    let src = "let x :: Int = \"hello\"";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Type mismatch in if branches: then branch Int, else branch String.
#[test]
fn test_diag_if_branch_mismatch() {
    let src = "if true do 1 else \"hello\" end";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Arity mismatch: function expects 2 args, called with 1.
#[test]
fn test_diag_arity_mismatch() {
    let src = "let f = fn (x, y) -> x end\nf(1)";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Unbound variable: name not defined.
#[test]
fn test_diag_unbound_variable() {
    let src = "x + 1";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Not a function: trying to call an Int.
#[test]
fn test_diag_not_a_function() {
    let src = "let x = 42\nx(1)";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Missing field in struct literal.
#[test]
fn test_diag_missing_field() {
    let src = "struct Point do\n  x :: Int\n  y :: Int\nend\nPoint { x: 1 }";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Unknown field in struct literal.
#[test]
fn test_diag_unknown_field() {
    let src = "struct Point do\n  x :: Int\n  y :: Int\nend\nPoint { x: 1, z: 2 }";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Trait not satisfied: String used with +.
#[test]
fn test_diag_trait_not_satisfied() {
    let src = "\"hello\" + \"world\"";
    let output = render_first_error(src);
    insta::assert_snapshot!(output);
}

/// Render errors method returns Vec of rendered diagnostics.
#[test]
fn test_render_errors_multiple() {
    let src = "x + y";
    let errors = render_all_errors(src);
    assert!(!errors.is_empty(), "expected at least one rendered error");
    for e in &errors {
        assert!(
            e.contains("Error"),
            "rendered error should contain 'Error': {}",
            e
        );
    }
}

/// Error code appears in diagnostic output.
#[test]
fn test_error_codes_present() {
    let src = "let x :: Int = \"hello\"";
    let output = render_first_error(src);
    assert!(
        output.contains("E0001"),
        "expected E0001 in mismatch diagnostic: {}",
        output
    );

    let src2 = "x + 1";
    let output2 = render_first_error(src2);
    assert!(
        output2.contains("E0004"),
        "expected E0004 in unbound var diagnostic: {}",
        output2
    );
}

// ── Phase 4 Diagnostic Tests ────────────────────────────────────────

#[test]
fn test_diag_non_exhaustive_match() {
    let src = "case x do Some(v) -> v end";
    let err = TypeError::NonExhaustiveMatch {
        scrutinee_type: "Option<Int>".to_string(),
        missing_patterns: vec!["None".to_string()],
        span: rowan::TextRange::new(0.into(), 26.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_non_exhaustive_match_multiple() {
    let src = "case s do Circle(r) -> r end";
    let err = TypeError::NonExhaustiveMatch {
        scrutinee_type: "Shape".to_string(),
        missing_patterns: vec!["Rect".to_string(), "Point".to_string()],
        span: rowan::TextRange::new(0.into(), 28.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    assert!(output.contains("E0012"), "expected E0012 code: {}", output);
    assert!(
        output.contains("Rect"),
        "expected Rect in missing: {}",
        output
    );
    assert!(
        output.contains("Point"),
        "expected Point in missing: {}",
        output
    );
    assert!(
        output.contains("non-exhaustive"),
        "expected non-exhaustive message: {}",
        output
    );
}

#[test]
fn test_diag_redundant_arm() {
    let src = "case x do _ -> 1\n  _ -> 2 end";
    let err = TypeError::RedundantArm {
        arm_index: 1,
        span: rowan::TextRange::new(18.into(), 24.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_redundant_arm_is_warning() {
    let src = "case x do _ -> 1\n  true -> 2 end";
    let err = TypeError::RedundantArm {
        arm_index: 1,
        span: rowan::TextRange::new(18.into(), 27.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    assert!(
        output.contains("Warning"),
        "expected Warning kind: {}",
        output
    );
    assert!(output.contains("W0001"), "expected W0001 code: {}", output);
    assert!(
        output.contains("unreachable"),
        "expected 'unreachable' label: {}",
        output
    );
}

#[test]
fn test_diag_invalid_guard_expression() {
    let src = "case x do n when f(n) -> n end";
    let err = TypeError::InvalidGuardExpression {
        reason: "function calls not allowed in guards".to_string(),
        span: rowan::TextRange::new(16.into(), 20.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_unknown_variant() {
    let src = "type Shape do Circle(Float) end\ncase s do Triangle(a) -> a end";
    let err = TypeError::UnknownVariant {
        name: "Triangle".to_string(),
        span: rowan::TextRange::new(42.into(), 54.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    assert!(output.contains("E0010"), "expected E0010 code: {}", output);
    assert!(
        output.contains("Triangle"),
        "expected 'Triangle' in output: {}",
        output
    );
}

#[test]
fn test_diag_or_pattern_binding_mismatch() {
    let src = "case x do a | (b, c) -> a end";
    let err = TypeError::OrPatternBindingMismatch {
        expected_bindings: vec!["a".to_string()],
        found_bindings: vec!["b".to_string(), "c".to_string()],
        span: rowan::TextRange::new(10.into(), 20.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    assert!(output.contains("E0011"), "expected E0011 code: {}", output);
    assert!(
        output.contains("bind"),
        "expected binding-related message: {}",
        output
    );
}

// ── Phase 6 Actor Diagnostic Tests ──────────────────────────────────

#[test]
fn test_diag_send_type_mismatch() {
    let src = "send(pid, 42)";
    let err = TypeError::SendTypeMismatch {
        expected: mesh_typeck::ty::Ty::string(),
        found: mesh_typeck::ty::Ty::int(),
        span: rowan::TextRange::new(0.into(), 13.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_self_outside_actor() {
    let src = "let me = self()";
    let result = check_source(src);
    assert!(!result.errors.is_empty(), "expected SelfOutsideActor error");
    let output = render_diagnostic(&result.errors[0], src, "test.mpl", &opts(), None);
    assert!(output.contains("E0015"), "expected E0015 code: {}", output);
    assert!(
        output.contains("self()"),
        "expected 'self()' in output: {}",
        output
    );
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_spawn_non_function() {
    let src = "spawn(42)";
    let err = TypeError::SpawnNonFunction {
        found: mesh_typeck::ty::Ty::int(),
        span: rowan::TextRange::new(0.into(), 9.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    assert!(output.contains("E0016"), "expected E0016 code: {}", output);
    assert!(
        output.contains("function"),
        "expected 'function' in output: {}",
        output
    );
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_receive_outside_actor() {
    let src = "receive do\nn -> n\nend";
    let result = check_source(src);
    assert!(
        !result.errors.is_empty(),
        "expected ReceiveOutsideActor error"
    );
    let output = render_diagnostic(&result.errors[0], src, "test.mpl", &opts(), None);
    assert!(output.contains("E0017"), "expected E0017 code: {}", output);
    assert!(
        output.contains("receive"),
        "expected 'receive' in output: {}",
        output
    );
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_send_type_mismatch_details() {
    let src = "send(pid, \"hello\")";
    let err = TypeError::SendTypeMismatch {
        expected: mesh_typeck::ty::Ty::int(),
        found: mesh_typeck::ty::Ty::string(),
        span: rowan::TextRange::new(0.into(), 18.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    assert!(
        output.contains("Int"),
        "expected 'Int' type in output: {}",
        output
    );
    assert!(
        output.contains("String"),
        "expected 'String' type in output: {}",
        output
    );
    assert!(
        output.contains("Pid"),
        "expected 'Pid' in help text: {}",
        output
    );
}

// ── Phase 10 JSON + Multi-span Diagnostic Tests ────────────────────

#[test]
fn test_json_output_mode() {
    let src = "let x :: Int = \"hello\"";
    let result = check_source(src);
    assert!(!result.errors.is_empty());
    let json_opts = DiagnosticOptions::json_mode();
    let output = render_diagnostic(&result.errors[0], src, "test.mpl", &json_opts, None);
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .unwrap_or_else(|e| panic!("invalid JSON output: {}\n{}", e, output));
    assert_eq!(parsed["code"], "E0001");
    assert_eq!(parsed["severity"], "error");
    assert!(parsed["message"].as_str().unwrap().contains("expected"));
    assert!(!parsed["spans"].as_array().unwrap().is_empty());
}

#[test]
fn test_json_one_line() {
    let src = "x + 1";
    let result = check_source(src);
    assert!(!result.errors.is_empty());
    let json_opts = DiagnosticOptions::json_mode();
    let output = render_diagnostic(&result.errors[0], src, "test.mpl", &json_opts, None);
    assert!(
        !output.contains('\n'),
        "JSON output should be one line: {}",
        output
    );
}

#[test]
fn test_not_a_function_fix_suggestion() {
    // Directly construct a NotAFunction error to test the fix suggestion.
    let src = "let x = 42\nx(1)";
    let err = TypeError::NotAFunction {
        ty: mesh_typeck::ty::Ty::int(),
        span: rowan::TextRange::new(11.into(), 15.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    assert!(
        output.contains("did you mean to call it"),
        "expected fix suggestion for not-a-function: {}",
        output
    );
}

// ── Phase 32 AmbiguousMethod Diagnostic Tests ──────────────────────

#[test]
fn test_diag_ambiguous_method_deterministic_order() {
    use mesh_typeck::ty::Ty;

    let src = "x.to_string()";
    let err = TypeError::AmbiguousMethod {
        method_name: "to_string".to_string(),
        candidate_traits: vec!["Displayable".to_string(), "Printable".to_string()],
        ty: Ty::int(),
        span: rowan::TextRange::new(0.into(), 13.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    insta::assert_snapshot!(output);
}

#[test]
fn test_diag_ambiguous_method_help_text() {
    use mesh_typeck::ty::Ty;

    let src = "point.to_string()";
    let err = TypeError::AmbiguousMethod {
        method_name: "to_string".to_string(),
        candidate_traits: vec!["Display".to_string(), "Printable".to_string()],
        ty: Ty::Con(mesh_typeck::ty::TyCon::new("Point")),
        span: rowan::TextRange::new(0.into(), 17.into()),
    };
    let output = render_diagnostic(&err, src, "test.mpl", &opts(), None);
    insta::assert_snapshot!(output);
}
