# S01: Clustered Declarations & Typed Public Surface — UAT

**Milestone:** M044
**Written:** 2026-03-29T19:21:20.704Z

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: This slice shipped compiler/LSP/runtime contract work plus a proof-app rewrite, so the truthful user-acceptance surface is the named verifier and its retained logs, not an unstructured manual click-through.

## Preconditions

- Run from the repo root with the normal Rust toolchain available.
- No stale local edits should be hiding failures in `compiler/`, `cluster-proof/`, or `scripts/verify-m044-s01.sh`.
- Remove or ignore older `.tmp/m044-s01/verify/` output; the verifier rewrites that directory each run.

## Smoke Test

1. Run `bash scripts/verify-m044-s01.sh`.
2. Inspect `.tmp/m044-s01/verify/status.txt` and `.tmp/m044-s01/verify/phase-report.txt`.
3. **Expected:** The command ends with `verify-m044-s01: ok`, `status.txt` contains `ok`, and every named phase in `phase-report.txt` ends in `passed`.

## Test Cases

### 1. Manifest opt-in stays optional while clustered declarations fail closed

1. Run `cargo test -p mesh-pkg m044_s01_clustered_manifest_ -- --nocapture`.
2. Run `cargo test -p mesh-lsp m044_s01_clustered_manifest_ -- --nocapture`.
3. Run `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`.
4. **Expected:** The parser rail runs 11 tests, the LSP rail runs 4 tests, and the meshc manifest rail runs 6 tests. Manifestless builds and manifests without `[cluster]` still pass, while private work targets, service kind mismatches, bad target shapes, disabled cluster sections, empty declarations, blank targets, and service start helpers fail with explicit `mesh.toml` reasons.

### 2. Typed continuity runtime truth is available to Mesh code

1. Run `cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture`.
2. Inspect the generated `.tmp/m044-s01/continuity-api-*` artifact directories if the test fails.
3. **Expected:** The rail runs 2 tests and passes. The runtime round-trip proves typed `Continuity.authority_status()`, `submit()`, `acknowledge_replica()`, and `promote()` field access from Mesh code, including standby→primary promotion truth and primary-side promotion rejection without falling back to JSON decoding.

### 3. Bad typed-API usage fails at compile time instead of silently drifting

1. Run `cargo test -p meshc --test e2e_m044_s01 m044_s01_continuity_compile_fail_ -- --nocapture`.
2. Inspect `.tmp/m044-s01/continuity-api-*` build logs if the rail fails.
3. **Expected:** The rail runs 2 tests and passes. `Continuity.promote("extra")` is rejected for wrong arity, and treating `Continuity.authority_status()` like an arithmetic value fails with a type/result-shape error.

### 4. `cluster-proof` consumes the typed public surface instead of runtime JSON shims

1. Run `cargo run -q -p meshc -- build cluster-proof`.
2. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
3. Search `cluster-proof/work_continuity.mpl` for `ContinuityAuthorityStatus.from_json|ContinuitySubmitDecision.from_json|WorkRequestRecord.from_json|parse_authority_status_json|parse_continuity_submit_response|parse_continuity_record`.
4. **Expected:** The build and package tests pass, and the search finds no matches. `cluster-proof/mesh.toml` contains `[cluster] enabled = true` plus the declared `WorkContinuity.*` and `WorkLegacy.handle_work_probe` boundary.

## Edge Cases

### Invalid manifest target shape

1. Re-run the manifest rails above or inspect the `m044_s01_manifest_service_target_bad_shape_fails_before_codegen` test case in `compiler/meshc/tests/e2e_m044_s01.rs`.
2. **Expected:** Service targets missing the `<ModulePath>.<Service>.<method>` shape fail before codegen and point back to the manifest contract instead of compiling locally.

### Private or ambiguous work declarations

1. Re-run the parser/compiler rails above or inspect the `m044_s01_clustered_manifest_rejects_private_work_target` / `m044_s01_clustered_manifest_rejects_ambiguous_work_target` coverage.
2. **Expected:** Private functions and overloaded public work names are rejected as clustered targets; the system does not silently broaden the clustered boundary.

### Authority outage surface stays explicit

1. Inspect `cluster-proof/main.mpl` and `cluster-proof/work_continuity.mpl` or rerun the package tests if you suspect drift.
2. **Expected:** Authority read failures still map to explicit `authority_status_unavailable:*` payloads/status codes and `[cluster-proof] Runtime authority unavailable error=...` logs instead of fabricated role/epoch truth.

## Failure Signals

- `bash scripts/verify-m044-s01.sh` exits non-zero, `status.txt` says `failed`, or `current-phase.txt` points at a specific failed rail.
- Any retained `*.test-count.log` file reports missing `running N test` output or `0` tests.
- `cluster-proof/work_continuity.mpl` regains any of the forbidden `parse_*_json` / `*.from_json(...)` continuity shims.
- `meshc build cluster-proof` fails with authority field-access drift or temp-project continuity linkage errors after a continuity field/layout change.

## Requirements Proved By This UAT

- R061 — Clustered mode is an app-level opt-in declared in `mesh.toml` using the standard clustered-app contract.
- R062 — Declared clustered handlers compile against typed public continuity and authority surfaces with no app-level continuity JSON parsing.
- R063 (advance only) — The declared clustered boundary is explicit and fail-closed even though runtime execution semantics land in S02.

## Not Proven By This UAT

- Runtime-owned execution, placement, and failover of declared clustered handlers; that is S02/S04 work.
- Built-in operator/CLI surfaces and `meshc init --clustered`; that is S03 work.
- Removal of the remaining `WorkLegacy` proof-app bridge; that is S05 work.

## Notes for Tester

Treat `.tmp/m044-s01/verify/` as the authoritative evidence bundle for this slice. If the acceptance rail goes red after a continuity-field change, start by comparing the typeck builtin registrations, MIR pre-seeded struct layout, codegen intrinsic declarations, and `mesh-rt` `MeshContinuity*` payload structs before reopening higher-level `cluster-proof` logic.
