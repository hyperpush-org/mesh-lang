//! Type checking tests for supervisor constructs: strategy validation,
//! child spec validation (start fn, restart type, shutdown value), and
//! duplicate child name detection.

use mesh_typeck::error::TypeError;
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

// ── Valid Supervisor ─────────────────────────────────────────────────

#[test]
fn test_supervisor_valid_one_for_one() {
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: one_for_one\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: 5000\n\
         end\n\
         end",
    );
    assert_no_errors(&result);
}

#[test]
fn test_supervisor_valid_one_for_all() {
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: one_for_all\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: 5000\n\
         end\n\
         end",
    );
    assert_no_errors(&result);
}

// ── Invalid Strategy ─────────────────────────────────────────────────

#[test]
fn test_supervisor_invalid_strategy() {
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: round_robin\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: 5000\n\
         end\n\
         end",
    );
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::InvalidStrategy { found, .. } if found == "round_robin"),
        "InvalidStrategy for round_robin",
    );
}

// ── Invalid Child Start (no spawn) ──────────────────────────────────

#[test]
fn test_supervisor_invalid_child_start() {
    // The start function does not use spawn(), so it doesn't return Pid.
    // fn -> 42 end  is a closure returning Int, not Pid.
    let result = check_source(
        "supervisor MySup do\n\
         strategy: one_for_one\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child bad do\n\
         start: fn -> 42 end\n\
         restart: permanent\n\
         shutdown: 5000\n\
         end\n\
         end",
    );
    assert_has_error(
        &result,
        |e| matches!(e, TypeError::InvalidChildStart { child_name, .. } if child_name == "bad"),
        "InvalidChildStart for child 'bad'",
    );
}

// ── Invalid Restart Type ────────────────────────────────────────────

#[test]
fn test_supervisor_invalid_restart_type() {
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: one_for_one\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: always\n\
         shutdown: 5000\n\
         end\n\
         end",
    );
    assert_has_error(
        &result,
        |e| {
            matches!(e, TypeError::InvalidRestartType { found, child_name, .. }
                     if found == "always" && child_name == "w1")
        },
        "InvalidRestartType for 'always'",
    );
}

// ── Invalid Shutdown Value ──────────────────────────────────────────

#[test]
fn test_supervisor_invalid_shutdown() {
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: one_for_one\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: fast\n\
         end\n\
         end",
    );
    assert_has_error(
        &result,
        |e| {
            matches!(e, TypeError::InvalidShutdownValue { found, child_name, .. }
                     if found == "fast" && child_name == "w1")
        },
        "InvalidShutdownValue for 'fast'",
    );
}

// ── Valid Child Start Returns Pid ───────────────────────────────────

#[test]
fn test_supervisor_child_start_returns_pid() {
    // spawn(worker) returns Pid -- this should be accepted.
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: one_for_one\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: 5000\n\
         end\n\
         end",
    );
    assert_no_errors(&result);
}

// ── Valid Strategies ────────────────────────────────────────────────

#[test]
fn test_supervisor_all_valid_strategies() {
    for strategy in &[
        "one_for_one",
        "one_for_all",
        "rest_for_one",
        "simple_one_for_one",
    ] {
        let src = format!(
            "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
             supervisor MySup do\n\
             strategy: {}\n\
             max_restarts: 3\n\
             max_seconds: 5\n\
             child w1 do\n\
             start: fn -> spawn(worker) end\n\
             restart: permanent\n\
             shutdown: 5000\n\
             end\n\
             end",
            strategy
        );
        let result = check_source(&src);
        assert!(
            !result
                .errors
                .iter()
                .any(|e| matches!(e, TypeError::InvalidStrategy { .. })),
            "strategy '{}' should be valid but got InvalidStrategy error",
            strategy
        );
    }
}

// ── Duplicate Child Names ───────────────────────────────────────────

#[test]
fn test_supervisor_duplicate_child_names() {
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: one_for_one\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: 5000\n\
         end\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: 5000\n\
         end\n\
         end",
    );
    // Should detect duplicate child name "w1".
    assert!(
        !result.errors.is_empty(),
        "expected error for duplicate child names, got none"
    );
}

// ── Valid Restart Types ─────────────────────────────────────────────

#[test]
fn test_supervisor_all_valid_restart_types() {
    for restart in &["permanent", "transient", "temporary"] {
        let src = format!(
            "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
             supervisor MySup do\n\
             strategy: one_for_one\n\
             max_restarts: 3\n\
             max_seconds: 5\n\
             child w1 do\n\
             start: fn -> spawn(worker) end\n\
             restart: {}\n\
             shutdown: 5000\n\
             end\n\
             end",
            restart
        );
        let result = check_source(&src);
        assert!(
            !result
                .errors
                .iter()
                .any(|e| matches!(e, TypeError::InvalidRestartType { .. })),
            "restart type '{}' should be valid but got InvalidRestartType error",
            restart
        );
    }
}

// ── Valid Shutdown Values ───────────────────────────────────────────

#[test]
fn test_supervisor_brutal_kill_shutdown() {
    let result = check_source(
        "actor worker() do\nreceive do\nm -> worker()\nend\nend\n\
         supervisor MySup do\n\
         strategy: one_for_one\n\
         max_restarts: 3\n\
         max_seconds: 5\n\
         child w1 do\n\
         start: fn -> spawn(worker) end\n\
         restart: permanent\n\
         shutdown: brutal_kill\n\
         end\n\
         end",
    );
    assert_no_errors(&result);
}
