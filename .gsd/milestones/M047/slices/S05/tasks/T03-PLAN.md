---
estimated_steps: 1
estimated_files: 6
skills_used: []
---

# T03: Land zero-ceremony `@cluster` declared-work wrappers

Implement the prerequisite compiler/runtime seam that T02 proved is still missing. Remove the public `(request_key, attempt_id)` requirement from declared-work validation/lowering for ordinary `@cluster` functions, generate internal adapters that receive runtime continuity metadata and invoke the user-authored function without exposing those args in source, and keep runtime-owned continuity completion/diagnostic surfaces truthful. Extend the M047 compiler/runtime rails so a minimal `@cluster pub fn add() -> Int` build passes while `meshc cluster continuity` still reports internal request/attempt metadata.

## Inputs

- `.gsd/milestones/M047/slices/S05/tasks/T01-SUMMARY.md`
- `.gsd/milestones/M047/slices/S05/tasks/T02-SUMMARY.md`
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`

## Expected Output

- `Updated declared-work validation/lowering for no-ceremony `@cluster` functions`
- `Compiler/runtime regression rails proving hidden continuity metadata still survives internally`

## Verification

cargo test -p meshc --test e2e_m047_s01 -- --nocapture && cargo test -p meshc --test e2e_m047_s02 -- --nocapture

## Observability Impact

- Signals added/changed: the generated Todo starter should log bootstrap mode, schema init, HTTP start, rate-limit rejection, and DB startup failures.
- How a future agent inspects this: generate the template, run the binary, and read stdout/stderr plus the generated README/Dockerfile.
- Failure state exposed: missing DB path/schema or rate-limit overflow becomes an explicit startup error or 429/5xx response instead of a silent no-op.
