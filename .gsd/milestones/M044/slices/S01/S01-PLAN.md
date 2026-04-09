# S01: Clustered Declarations & Typed Public Surface

**Goal:** Productize the first public clustered-app contract by adding an optional manifest opt-in, validating declared clustered boundaries, and replacing stringly continuity APIs with typed Mesh-facing surfaces consumed by `cluster-proof`.
**Demo:** After this: After this: a Mesh app can opt into clustered mode in `mesh.toml`, declare clustered handlers, and compile against typed continuity/authority values without continuity JSON parsing in app code.

## Tasks
- [x] **T01: Added optional clustered manifest declarations with shared compiler/LSP validation and the first M044 contract tests.** — ---
estimated_steps: 4
estimated_files: 4
skills_used:
  - rust-best-practices
  - test
---

Define the public clustered-app declaration contract at the manifest/compiler boundary first so later runtime and proof-app work do not guess at handler naming or accidentally make `mesh.toml` mandatory for all builds.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Shared `mesh.toml` parsing in `mesh-pkg` | Fail the build with the manifest path and parser reason; do not silently ignore an invalid clustered config. | N/A — parsing is synchronous. | Reject partial clustered config instead of defaulting to a broader or looser clustered boundary. |
| Export collection in `meshc` | Stop before MIR lowering and report the unresolved/private target explicitly. | N/A — export collection is synchronous. | Treat unknown handler kinds or bad target strings as contract errors, not as local-only fallbacks. |
| Project-aware LSP analysis | Keep editor diagnostics aligned with `meshc build` and surface drift as a test failure. | Fail the LSP test instead of returning stale single-file truth. | Reject manifest-backed clustered apps that analyze differently than the compiler path. |

## Load Profile

- **Shared resources**: manifest parsing, project-wide export discovery, and project-aware LSP analysis.
- **Per-operation cost**: one optional manifest parse plus one export-validation pass per build/analyze request.
- **10x breakpoint**: broad project analysis and repeated validation work would drift first if the clustered declaration resolver is duplicated instead of shared.

## Negative Tests

- **Malformed inputs**: missing `[cluster]` fields, unknown handler kinds, bad target strings, and empty declaration arrays.
- **Error paths**: unknown targets, private targets, or service-method mismatches must fail closed.
- **Boundary conditions**: manifest absent, clustered section absent, and manifest present on a project that still has to build locally without clustered declarations.

## Steps

1. Extend `compiler/mesh-pkg/src/manifest.rs` with an optional clustered-app section that keeps existing package/dependency parsing intact and leaves non-manifest builds valid.
2. Teach `compiler/meshc/src/main.rs` to load the optional manifest when present, validate clustered declarations after export collection and before MIR lowering, and emit explicit diagnostics for unknown/private/mismatched targets.
3. Mirror the same manifest-aware clustered validation in `compiler/mesh-lsp/src/analysis.rs` so editor truth stays aligned with the compiler path.
4. Create `compiler/meshc/tests/e2e_m044_s01.rs` with `manifest_`/`clustered_manifest_`-prefixed coverage and add matching parser/LSP tests so the later verifier can target them fail-closed.

## Must-Haves

- [ ] Clustered mode is an app-level opt-in in `mesh.toml`, not a mandatory compiler prerequisite.
- [ ] The declaration boundary is explicit and narrow: service calls, service casts, and public work functions only.
- [ ] Invalid declarations fail before codegen and show the exact bad target/reason in both compiler and LSP proof rails.
  - Estimate: 2h
  - Files: compiler/mesh-pkg/src/manifest.rs, compiler/meshc/src/main.rs, compiler/mesh-lsp/src/analysis.rs, compiler/meshc/tests/e2e_m044_s01.rs
  - Verify: `cargo test -p mesh-pkg clustered_manifest_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 manifest_ -- --nocapture`
`cargo test -p mesh-lsp clustered_manifest_ -- --nocapture`
- [x] **T02: Replace stringly Continuity results with typed builtins across typeck, MIR, and runtime** — ---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - test
---

Move the public continuity API onto the existing typed-builtin pattern so app code can field-access authority and record values directly, and later clustered-handler work can build on a real Mesh-facing contract instead of JSON shims.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Typechecker builtin-module and struct registration | Fail compilation with a type error instead of silently leaving `Continuity.*` as `String ! String`. | N/A — compile-time only. | Reject partial struct registration that would let some fields type-check and others drift. |
| MIR/intrinsic/runtime ABI alignment | Stop at compile or runtime tests with the exact symbol/layout mismatch; do not paper over it with string re-encoding. | Fail the e2e run and keep the build/run logs. | Treat wrong payload boxing or field layout as an ABI regression, not a decode fallback. |
| `mesh-rt` staticlib freshness in compiler e2e | Rebuild `mesh-rt` before linking temp projects so stale symbols cannot fake a compiler/runtime failure. | N/A — one explicit build step. | Reject missing/new continuity symbols as stale-artifact drift rather than changing tests to match the stale library. |

## Load Profile

- **Shared resources**: type maps, MIR struct tables, runtime result boxing, and temp-project compiler e2e builds.
- **Per-operation cost**: one `mesh-rt` build plus typed compile/run checks over the new builtin structs.
- **10x breakpoint**: ABI and payload-layout mismatches break before performance matters; the expensive failure mode is repeated temp-project rebuild/link churn.

## Negative Tests

- **Malformed inputs**: wrong arity, wrong field name, and wrong result-shape usage of `Continuity.*` values.
- **Error paths**: runtime promotion rejection and request-key-not-found must still surface as `Err(String)` without decode wrappers.
- **Boundary conditions**: nested typed payloads (`submit().record.*`), completion/ack record updates, and authority/status reads in both primary and standby roles.

## Steps

1. Register builtin continuity struct shapes and typed `Continuity.*` signatures in `compiler/mesh-typeck/src/infer.rs`, following the existing `HttpResponse` precedent instead of inventing a new ad hoc path.
2. Mirror those structs and result payload shapes in `compiler/mesh-codegen/src/mir/lower.rs` and `compiler/mesh-codegen/src/codegen/intrinsics.rs` so field access and lowering agree on layout.
3. Change `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/lib.rs` to return typed Mesh payloads directly from the exported continuity functions rather than JSON strings.
4. Extend `compiler/meshc/tests/e2e_m044_s01.rs` with `typed_continuity_` coverage and either retire or rewrite the old stringly continuity assertions in `compiler/meshc/tests/e2e_m043_s02.rs` so the workspace no longer depends on JSON decoding as the public contract.

## Must-Haves

- [ ] `Continuity.authority_status()`, `status()`, `submit()`, `promote()`, `mark_completed()`, and `acknowledge_replica()` all expose typed Mesh results.
- [ ] The typed continuity structs are registered in both typeck and MIR, with matching runtime payload boxing/layout.
- [ ] Compiler e2e coverage proves both the happy path and compile-time failures for bad typed API usage.
  - Estimate: 3h
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/lib.rs, compiler/meshc/tests/e2e_m044_s01.rs, compiler/meshc/tests/e2e_m043_s02.rs
  - Verify: `cargo test -p meshc --test e2e_m044_s01 typed_continuity_ -- --nocapture`
`cargo test -p meshc --test e2e_m044_s01 continuity_compile_fail_ -- --nocapture`
- [x] **T03: Stopped T03 after confirming the typed continuity builtin seam from T02 never landed; wrote precise resume notes instead of shipping a fake cluster-proof rewrite.** — ---
estimated_steps: 4
estimated_files: 6
skills_used:
  - test
---

Prove the new public surface is real by rewriting the live proof app onto it. S01 is not done if the compiler/runtime change lands but `cluster-proof` still parses continuity JSON or lacks a real clustered-app manifest.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` manifest opt-in and declaration contract | Fail the build with the exact manifest/declaration error instead of silently falling back to the old proof-app path. | N/A — static config. | Reject stale or misspelled declarations rather than broadening the clustered boundary implicitly. |
| Typed `Continuity.*` surface | Stop on compile/runtime failure and keep the proof-app logs; do not reintroduce `from_json` wrappers. | Fail the app tests and preserve logs/artifacts. | Treat any need to decode runtime continuity JSON as a regression in the public API contract. |
| Existing `cluster-proof` HTTP JSON responses | Keep HTTP shaping local to the app payload structs while avoiding builtin continuity JSON traits as a hidden dependency. | Fail the proof-app tests/build if response shaping drifts. | Reject malformed payloads or missing fields instead of hiding the typed/runtime boundary behind another opaque shim. |

## Load Profile

- **Shared resources**: `cluster-proof` build/test compilation, continuity runtime calls, and local HTTP JSON shaping.
- **Per-operation cost**: one package build, one package test run, and targeted grep checks over the continuity adapter layer.
- **10x breakpoint**: build/test churn and stale shims break before runtime load matters; the main risk is drift back to JSON decoding.

## Negative Tests

- **Malformed inputs**: missing clustered opt-in, undeclared handler targets, and stale helper names that still expect raw JSON strings.
- **Error paths**: runtime continuity rejection, authority unavailability, and status-not-found must still surface through the typed API without decode adapters.
- **Boundary conditions**: typed nested record reads, local HTTP payload encoding, and the line between clustered handlers and ordinary local code inside `cluster-proof`.

## Steps

1. Add `cluster-proof/mesh.toml` with the clustered opt-in and declared handler boundary defined in T01, keeping the package metadata valid for ordinary builds.
2. Rewrite `cluster-proof/work_continuity.mpl` to consume typed `Continuity` values directly, deleting the runtime continuity `parse_*_json` helpers and app-level `from_json` calls on authority/record/submit payloads.
3. Update `cluster-proof/work.mpl`, `cluster-proof/main.mpl`, and any package tests that still assume stringly continuity responses so the app boundary stays honest.
4. Extend the M044 proof rail with a `cluster_proof_`-prefixed build/test case or equivalent package assertions so the verifier can prove the rewritten consumer contract directly.

## Must-Haves

- [ ] `cluster-proof` has a real `mesh.toml` clustered-app opt-in and declared handler surface.
- [ ] Runtime continuity payloads are consumed as typed Mesh values; only HTTP response payloads remain locally JSON-shaped.
- [ ] No runtime continuity `from_json` helper survives in `cluster-proof/work_continuity.mpl`.
  - Estimate: 2h
  - Files: cluster-proof/mesh.toml, cluster-proof/work_continuity.mpl, cluster-proof/work.mpl, cluster-proof/main.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m044_s01.rs
  - Verify: `cargo run -q -p meshc -- build cluster-proof`
`cargo run -q -p meshc -- test cluster-proof/tests`
`! rg -n 'ContinuityAuthorityStatus\.from_json|ContinuitySubmitDecision\.from_json|WorkRequestRecord\.from_json|parse_authority_status_json|parse_continuity_submit_response|parse_continuity_record' cluster-proof/work_continuity.mpl`
  - Blocker: `Continuity.*` still exposes `String ! String` to Mesh code; `cluster-proof/work_continuity.mpl` still uses runtime JSON decode helpers; `compiler/meshc/tests/e2e_m043_s02.rs` still preserves the deprecated stringly public contract; `cluster-proof/mesh.toml` does not exist yet.
- [x] **T04: Restored the typed Continuity compiler/runtime seam and documented that the dedicated M044 proof tests still need migration.** — Re-do the unlanded T02 work explicitly instead of pretending it already exists.

1. Register the typed `Continuity` structs and function signatures in both `compiler/mesh-typeck/src/infer.rs` and `compiler/mesh-typeck/src/builtins.rs`, following the existing builtin-struct path rather than leaving the Mesh surface at `String ! String`.
2. Mirror those typed shapes through `compiler/mesh-codegen/src/mir/lower.rs` and `compiler/mesh-codegen/src/codegen/intrinsics.rs` so field access, lowering, and runtime ABI all agree on layout and result payloads.
3. Change `compiler/mesh-rt/src/dist/continuity.rs` and `compiler/mesh-rt/src/lib.rs` to return typed Mesh payloads directly from the exported continuity functions instead of JSON strings.
4. Rewrite the stale stringly public-contract coverage in `compiler/meshc/tests/e2e_m044_s01.rs` and `compiler/meshc/tests/e2e_m043_s02.rs` so compiler/runtime truth no longer depends on app-side JSON decoding.
  - Estimate: 3h
  - Files: compiler/mesh-typeck/src/infer.rs, compiler/mesh-typeck/src/builtins.rs, compiler/mesh-codegen/src/mir/lower.rs, compiler/mesh-codegen/src/codegen/intrinsics.rs, compiler/mesh-rt/src/dist/continuity.rs, compiler/mesh-rt/src/lib.rs, compiler/meshc/tests/e2e_m044_s01.rs, compiler/meshc/tests/e2e_m043_s02.rs
  - Verify: cargo test -p meshc --test e2e_m044_s01 typed_continuity_ -- --nocapture
cargo test -p meshc --test e2e_m044_s01 continuity_compile_fail_ -- --nocapture
- [x] **T05: Rewrote `cluster-proof` onto typed continuity values, added a real clustered manifest, and pinned the rewrite to named `cluster_proof_` proof rails.** — Now that the typed public API is real, migrate the proof app onto it and delete the stringly shim path.

1. Add `cluster-proof/mesh.toml` with the clustered opt-in and declared handler boundary from T01, keeping package metadata valid for ordinary package build/test flows.
2. Rewrite `cluster-proof/work_continuity.mpl` to consume typed `Continuity` values directly and delete the runtime continuity `parse_*_json` helpers plus `*.from_json(...)` adapters for authority, submit, and record payloads.
3. Update `cluster-proof/work.mpl`, `cluster-proof/main.mpl`, and package tests so only app HTTP payload shaping remains local JSON work while continuity/authority data stays typed Mesh values end-to-end.
4. Extend the M044 proof rail so `cluster-proof` consumer coverage lives in the slice’s named compiler/package tests rather than as an undocumented manual rewrite.
  - Estimate: 2h
  - Files: cluster-proof/mesh.toml, cluster-proof/work_continuity.mpl, cluster-proof/work.mpl, cluster-proof/main.mpl, cluster-proof/tests/work.test.mpl, compiler/meshc/tests/e2e_m044_s01.rs
  - Verify: cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
! rg -n 'ContinuityAuthorityStatus\.from_json|ContinuitySubmitDecision\.from_json|WorkRequestRecord\.from_json|parse_authority_status_json|parse_continuity_submit_response|parse_continuity_record' cluster-proof/work_continuity.mpl
- [x] **T06: Added the fail-closed M044/S01 acceptance rail and moved the typed continuity compiler proofs into the M044 suite.** — Finish the slice only after one authoritative verifier replays the restored contract in order.

1. Add `scripts/verify-m044-s01.sh` as the single repo-root acceptance command that runs the named manifest/parser/compiler/LSP rails, the typed continuity compiler e2e, the `cluster-proof` build/test replay, and the stale-continuity-shim absence check in order.
2. Make the wrapper fail closed on missing `running N test` evidence or 0-test filters, reset `.tmp/m044-s01/verify/` phase state per run, and preserve per-phase logs/status markers for postmortem inspection.
3. Align verifier-targeted test prefixes and coverage in `compiler/meshc/tests/e2e_m044_s01.rs`, `compiler/mesh-lsp/src/analysis.rs`, and `compiler/mesh-pkg/src/manifest.rs` so the wrapper depends on stable named rails instead of broad ad hoc filters.
4. Treat `bash scripts/verify-m044-s01.sh` as the slice’s stopping condition and keep the artifact bundle specific enough to distinguish manifest-validation drift, typed ABI regressions, and `cluster-proof` consumer drift.
  - Estimate: 90m
  - Files: scripts/verify-m044-s01.sh, compiler/meshc/tests/e2e_m044_s01.rs, compiler/mesh-lsp/src/analysis.rs, compiler/mesh-pkg/src/manifest.rs
  - Verify: bash scripts/verify-m044-s01.sh
