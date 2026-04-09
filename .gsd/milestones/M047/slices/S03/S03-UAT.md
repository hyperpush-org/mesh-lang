# S03: Clustered HTTP route wrapper — UAT

**Milestone:** M047
**Written:** 2026-04-01T08:50:08.145Z

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: the slice did not ship a live user-facing wrapper feature. The honest acceptance question is whether the repo now records the blocker truthfully, preserves the surrounding clustered/route control rails, and avoids teaching a fake `HTTP.clustered(...)` story.

## Preconditions

- Run from the repo root.
- `cargo` is available.
- No existing `e2e_m047_s03` target or `HTTP.clustered(...)` implementation has been added since this slice summary was written.

## Smoke Test

Run `cargo test -p meshc --test e2e_m047_s03 -- --nocapture`.

**Expected:** the command fails closed with `error: no test target named 'e2e_m047_s03' in 'meshc' package`, proving the wrapper rail still does not exist and the slice is not claiming otherwise.

## Test Cases

### 1. Missing clustered-route wrapper is still reported honestly

1. Run `rg -n 'HTTP\.clustered|http_clustered' compiler/mesh-typeck/src compiler/mesh-codegen/src compiler/mesh-rt/src compiler/meshc/tests -g '*.rs'`.
2. Run `cargo test -p meshc --test e2e_m047_s03 -- --nocapture`.
3. **Expected:** the grep returns no matches, and Cargo fails with `no test target named 'e2e_m047_s03'`. This is the authoritative blocker signal for the missing wrapper surface.

### 2. Ordinary `@cluster` behavior is still intact while S03 is blocked

1. Run `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`.
2. **Expected:** the target passes (currently 2 tests), proving the S02 runtime-name and replication-count truth for ordinary clustered declarations still works while the HTTP wrapper remains absent.

### 3. Retained M032 route-limit controls are self-contained and still truthful

1. Run `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`.
3. **Expected:** both commands pass. The bare-function control serves a live 200 response, and the closure route still fails only at live request time. Neither test should require any `.tmp/m032-s01/...` fixture tree to compile.

### 4. Typeck/LSP guardrails still describe the current tree without pretending S03 exists

1. Run `cargo test -p mesh-typeck m047_s03 -- --nocapture`.
2. Run `cargo test -p mesh-lsp -- --nocapture`.
3. Run `cargo test -p mesh-codegen m047_s03 -- --nocapture`.
4. Run `cargo test -p mesh-rt m047_s03 -- --nocapture`.
5. **Expected:** mesh-lsp passes its suite, while the `m047_s03`-filtered typeck/codegen/runtime commands exit 0 but report zero matching tests. Treat those as presence checks only, not as proof that the wrapper exists.

## Edge Cases

### Zero-test green filters

1. Inspect the output from `cargo test -p mesh-typeck m047_s03 -- --nocapture`, `cargo test -p mesh-codegen m047_s03 -- --nocapture`, and `cargo test -p mesh-rt m047_s03 -- --nocapture`.
2. **Expected:** each command reports zero executed tests. If a future closeout treats those green exits as implementation proof without checking the `running 0 tests` / filtered count signal, the verification story has regressed.

### Transient fixture independence for the retained route controls

1. Confirm there is no `.tmp/m032-s01/...` fixture tree available, or ignore it if present.
2. Run the two `e2e_stdlib` M032 control commands again.
3. **Expected:** both controls still compile and pass because their Mesh sources now live directly in `compiler/meshc/tests/e2e_stdlib.rs`.

## Failure Signals

- `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` stops passing.
- Either retained M032 control stops compiling or starts depending on transient `.tmp` fixtures again.
- `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` starts succeeding without a corresponding real `HTTP.clustered(...)` implementation and live route proof.
- Future reviewers ignore the zero-test `m047_s03` filter outputs and claim the wrapper exists.

## Requirements Proved By This UAT

- none — this UAT proves blocker truth and control-rail integrity, not a requirement status transition.

## Not Proven By This UAT

- A live clustered HTTP route wrapper.
- Route-local replication-count truth for `HTTP.clustered(...)`.
- Runtime request/reply transport for clustered `Request -> Response` handlers.
- Any scaffold, docs, or public migration surface that depends on the wrapper existing.

## Notes for Tester

This is an intentionally negative UAT for a blocked slice. A green run means the repo is being honest about what did and did not land: the wrapper is still missing, the positive S02 control still works, and the retained M032 route-limit controls stay self-contained and truthful.
