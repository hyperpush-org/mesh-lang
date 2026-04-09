---
id: S01
parent: M044
milestone: M044
provides:
  - Optional `[cluster]` app metadata with a fail-closed declaration contract for clustered service call handlers, service cast handlers, and public work functions.
  - Typed Mesh-facing `ContinuityAuthorityStatus`, `ContinuityRecord`, and `ContinuitySubmitDecision` values backed by aligned compiler/runtime ABI support.
  - A `cluster-proof` consumer that opts into clustered mode through `mesh.toml` and consumes typed continuity data directly.
  - A single slice-level verifier (`bash scripts/verify-m044-s01.sh`) with retained artifacts that localize manifest, ABI, and proof-app regressions.
requires:
  []
affects:
  - S02
  - S03
  - S05
key_files:
  - compiler/mesh-pkg/src/manifest.rs
  - compiler/meshc/src/main.rs
  - compiler/mesh-lsp/src/analysis.rs
  - compiler/mesh-typeck/src/builtins.rs
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - cluster-proof/work_continuity.mpl
  - scripts/verify-m044-s01.sh
key_decisions:
  - Use optional `[cluster]` metadata in `mesh.toml` with fully-qualified `{ kind, target }` declarations and fail-closed validation against a shared clustered export surface.
  - Keep the first typed public continuity API as String/Int/Bool-backed builtin structs rather than expanding the slice into builtin enum registration and matching changes.
  - Consume typed `ContinuityAuthorityStatus`, `ContinuityRecord`, and `ContinuitySubmitDecision` directly inside `cluster-proof` and keep JSON shaping only at the HTTP request/response boundary.
  - Make `bash scripts/verify-m044-s01.sh` the authoritative local slice rail, with named non-zero test-filter checks and retained phase logs under `.tmp/m044-s01/verify/`.
patterns_established:
  - Validate clustered boundaries against one shared compiler-discovered export surface and reuse that exact contract in both `meshc` and `mesh-lsp` instead of maintaining parallel target-parsing logic.
  - When a runtime-backed Mesh API becomes typed, update the seam in lockstep across typeck builtins, module registration, MIR struct pre-seeding, codegen intrinsic declarations, and runtime payload boxing/layout.
  - In proof apps, keep JSON only at the real transport boundary; delete stringly decode shims once the public Mesh-facing API becomes typed.
observability_surfaces:
  - `.tmp/m044-s01/verify/{status.txt,phase-report.txt,full-contract.log,*.log}` records per-phase verifier state and the first failing rail for slice-level debugging.
  - `cluster-proof/main.mpl` logs typed runtime authority readiness and unavailability (`[cluster-proof] Runtime authority ready ...` / `Runtime authority unavailable error=...`).
  - The typed authority/status HTTP payload helpers in `cluster-proof/work_continuity.mpl` now surface `cluster_role`, `promotion_epoch`, and `replication_health` without app-side continuity JSON parsing.
drill_down_paths:
  - .gsd/milestones/M044/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M044/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M044/slices/S01/tasks/T03-SUMMARY.md
  - .gsd/milestones/M044/slices/S01/tasks/T04-SUMMARY.md
  - .gsd/milestones/M044/slices/S01/tasks/T05-SUMMARY.md
  - .gsd/milestones/M044/slices/S01/tasks/T06-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-29T19:21:20.703Z
blocker_discovered: false
---

# S01: Clustered Declarations & Typed Public Surface

**Optional clustered declarations in `mesh.toml`, typed `Continuity` builtins, and a typed `cluster-proof` consumer now form the first public clustered-app contract.**

## What Happened

S01 productized the first public clustered-app contract in four seams. First, T01 added optional `[cluster]` metadata to `mesh.toml` and a shared clustered declaration validator in `mesh-pkg`; `meshc` now loads that manifest only when present, keeps manifestless builds valid, and rejects bad targets before MIR lowering, while `mesh-lsp` mirrors the same surface so editor diagnostics and compiler truth stay aligned. The declaration boundary is intentionally narrow and explicit: clustered service call handlers, service cast handlers, and public work functions only.

Second, the slice had to correct its own assumptions. An attempted proof-app rewrite exposed that the public `Continuity.*` API was still stringly at the Mesh boundary, so T04 restored the typed contract properly instead of papering over it. `compiler/mesh-typeck` now exposes `ContinuityAuthorityStatus`, `ContinuityRecord`, and `ContinuitySubmitDecision` as builtin structs, MIR lowering pre-seeds matching layouts for field access, codegen maps the Mesh calls to typed `mesh_continuity_*` intrinsics, and `mesh-rt` now returns boxed typed payload structs instead of JSON strings.

Third, T05 moved `cluster-proof` onto that real public seam. The proof app now has its own `cluster-proof/mesh.toml` clustered opt-in, `work_continuity.mpl` consumes typed continuity results directly, the old `parse_*_json` and `*.from_json(...)` continuity adapters are gone, and only the HTTP request/response wrappers remain locally JSON-shaped. Package tests were updated to assert typed authority/status/submit payload behavior instead of preserving the deprecated stringly path.

Finally, T06 turned the slice into one authoritative acceptance story. `scripts/verify-m044-s01.sh` refreshes `mesh-rt`, replays the named manifest/parser/LSP/compiler/runtime rails, proves the `cluster-proof` build and package-test path, and fails closed if any named filter runs zero tests or if any stale continuity shim literals reappear. The retained artifact bundle under `.tmp/m044-s01/verify/` is now the slice-level diagnostic surface for manifest drift, typed continuity ABI drift, and proof-app consumer regressions.

## Verification

Verified with `bash scripts/verify-m044-s01.sh` during slice closeout. The retained phase report at `.tmp/m044-s01/verify/phase-report.txt` shows all nine phases passed: `mesh-rt-staticlib`, `manifest-parser`, `lsp-clustered-manifest`, `meshc-manifest`, `meshc-typed-runtime`, `meshc-typed-compile-fail`, `cluster-proof-build`, `cluster-proof-tests`, and `cluster-proof-shim-absence`. The verifier also retained explicit non-zero test-count evidence for each named filter: manifest parser `[11]`, LSP `[4]`, meshc manifest `[6]`, typed runtime `[2]`, and typed compile-fail `[2]`. `cluster-proof/work_continuity.mpl` passed the six-literal stale-shim absence check, and both `cargo run -q -p meshc -- build cluster-proof` and `cargo run -q -p meshc -- test cluster-proof/tests` passed inside the assembled rail.

## Requirements Advanced

- R063 — S01 defined the explicit declared-handler boundary in `mesh.toml` and enforced it through shared compiler/LSP validation, so S02 can attach runtime semantics to a narrow, already-proven contract instead of widening the clustered claim ad hoc.
- R069 — S01 moved `cluster-proof` onto the new public clustered-app inputs (manifest opt-in plus typed continuity values), which advances the eventual full proof-app rewrite even though the legacy execution/operator path is still present.

## Requirements Validated

- R061 — Validated by the green manifest rails in `cargo test -p mesh-pkg m044_s01_clustered_manifest_ -- --nocapture`, `cargo test -p mesh-lsp m044_s01_clustered_manifest_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s01 m044_s01_manifest_ -- --nocapture`, plus the assembled `bash scripts/verify-m044-s01.sh` replay.
- R062 — Validated by the typed runtime and compile-fail rails (`cargo test -p meshc --test e2e_m044_s01 m044_s01_typed_continuity_ -- --nocapture` and `m044_s01_continuity_compile_fail_`), the green `cluster-proof` build/package tests, and the retained shim-absence check over `cluster-proof/work_continuity.mpl` in `bash scripts/verify-m044-s01.sh`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Operational Readiness

- **Health signal**: `bash scripts/verify-m044-s01.sh` ends green with `.tmp/m044-s01/verify/status.txt=ok`, and the live proof app now surfaces typed runtime authority truth through `cluster-proof/main.mpl` startup logs plus the `/membership` and `/promote` payload helpers (`cluster_role`, `promotion_epoch`, `replication_health`).
- **Failure signal**: invalid clustered declarations now surface explicit `mesh.toml:` diagnostics in both `meshc` and `mesh-lsp`; runtime authority outages surface `authority_status_unavailable:*` payloads/status codes plus `[cluster-proof] Runtime authority unavailable error=...` logs; the acceptance bundle points at the first broken phase via `current-phase.txt` and the per-phase logs.
- **Recovery**: fix the manifest contract or continuity ABI drift, rebuild `mesh-rt`, and rerun `bash scripts/verify-m044-s01.sh`. If the proof app regresses after a continuity field change, re-align the typeck builtins, MIR pre-seeded structs, intrinsic declarations, and `mesh-rt` `MeshContinuity*` payload layout before changing higher-level `cluster-proof` logic.
- **Monitoring gaps**: operator truth is still app-owned inside `cluster-proof`; there is no built-in runtime/CLI clustered inspector yet, and declared handlers are not runtime-executed until S02, so long-running production monitoring still depends on the proof app’s HTTP/log wrappers and the slice verifier rather than a platform-native cluster admin surface.

## Deviations

T03 discovered the planned proof-app rewrite would have been fake because the public `Continuity.*` seam was still `Result<String, String>` under the hood. The slice therefore reopened that compiler/runtime seam as T04, landed the typed builtin contract first, and only then completed the `cluster-proof` rewrite and final acceptance rail.

## Known Limitations

Declared clustered handlers are only declared and validated in S01; runtime-owned execution, placement, and failover for those declarations remain S02 work. `cluster-proof` still carries `WorkLegacy` and app-owned placement/operator glue, built-in runtime/CLI operator surfaces do not exist yet, and `meshc init` still scaffolds only the ordinary hello-world app. The typed continuity payloads also keep phase/result/role/outcome as string fields for now rather than dedicated builtin enums.

## Follow-ups

S02 should connect the new `[cluster]` declarations to runtime-owned declared-handler execution and placement instead of leaving them as validation-only metadata. S03 should move authority/status/operator truth out of `cluster-proof`’s app-owned HTTP wrapper into built-in runtime and CLI surfaces and add clustered scaffolding. S05 should remove the remaining `WorkLegacy` bridge once the declared-handler runtime path is real.

## Files Created/Modified

- `compiler/mesh-pkg/src/manifest.rs` — Added optional `[cluster]` manifest parsing, shared clustered declaration validation, and manifest contract tests.
- `compiler/meshc/src/main.rs` — Loaded optional `mesh.toml` files during project builds and rejected invalid clustered declarations before MIR/codegen.
- `compiler/mesh-lsp/src/analysis.rs` — Mirrored manifest-backed clustered declaration validation into project-aware LSP analysis so editor diagnostics match `meshc build`.
- `compiler/mesh-typeck/src/builtins.rs` — Registered typed `Continuity` builtin functions and payload structs in the Mesh type environment and module registry.
- `compiler/mesh-typeck/src/infer.rs` — Registered typed continuity structs for field access and builtin inference.
- `compiler/mesh-codegen/src/mir/lower.rs` — Pre-seeded MIR struct layouts for `ContinuityAuthorityStatus`, `ContinuityRecord`, and `ContinuitySubmitDecision`, and mapped lowered calls onto the typed runtime intrinsics.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Declared the typed `mesh_continuity_*` runtime intrinsics with result payload signatures that match the public Mesh API.
- `compiler/mesh-rt/src/dist/continuity.rs` — Returned boxed typed continuity payload structs from the runtime ABI instead of JSON strings.
- `compiler/meshc/tests/e2e_m044_s01.rs` — Added the named M044 manifest, typed continuity, compile-fail, and cluster-proof consumer regression suite.
- `cluster-proof/mesh.toml` — Added a real clustered manifest for the proof app.
- `cluster-proof/work_continuity.mpl` — Deleted continuity JSON decode shims and rewired the proof app to consume typed continuity payloads directly while keeping JSON local to the HTTP boundary.
- `cluster-proof/main.mpl` — Updated runtime-authority logging and membership responses to consume typed continuity status directly.
- `cluster-proof/tests/work.test.mpl` — Reworked proof-app package tests around typed authority, status, and submit helpers.
- `scripts/verify-m044-s01.sh` — Added the fail-closed slice acceptance wrapper with named test-count checks and retained per-phase artifacts.

## Forward Intelligence

### What the next slice should know
- The clustered declaration contract is already shared between parser, compiler, and LSP. Reuse `ClusteredExportSurface` / `validate_cluster_declarations(...)` instead of inventing a second declaration parser when S02 starts executing declared handlers.
- `cluster-proof` is now a truthful typed consumer of `Continuity.*`; any new JSON work added below the HTTP boundary is a regression against the public API contract S01 just proved.

### What's fragile
- The typed continuity seam spans typeck builtins, MIR struct pre-seeding, intrinsic declarations, and the `mesh-rt` `MeshContinuity*` ABI layout — change one without the others and field access or temp-project linking will drift.
- `cluster-proof/main.mpl` authority helpers still need typed parameter annotations for imported runtime structs; otherwise `meshc build cluster-proof` can regress with the misleading bare diagnostic `Undefined variable 'to_string'`.

### Authoritative diagnostics
- `.tmp/m044-s01/verify/phase-report.txt` / `.tmp/m044-s01/verify/current-phase.txt` — first stop for slice-level regressions because they distinguish manifest-contract drift, typed ABI drift, and proof-app consumer drift.
- `compiler/meshc/tests/e2e_m044_s01.rs` — authoritative compiler/runtime contract cases for manifest validity, typed runtime truth, compile-fail behavior, and `cluster-proof` consumer coverage.
- `cluster-proof/work_continuity.mpl` plus `.tmp/m044-s01/verify/08-cluster-proof-shim-absence.log` — authoritative check that runtime continuity JSON shims did not creep back in.

### What assumptions changed
- Original assumption: the public typed `Continuity.*` seam already existed and `cluster-proof` just needed a consumer rewrite.
- What actually happened: T03 proved the Mesh-facing API was still stringly, so the slice had to land the compiler/runtime ABI seam first and only then complete the proof-app migration.
