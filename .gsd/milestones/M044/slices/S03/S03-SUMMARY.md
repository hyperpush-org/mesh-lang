---
id: S03
parent: M044
milestone: M044
provides:
  - Runtime-owned transient operator query transport plus public `meshc cluster` inspection commands.
  - `meshc init --clustered` scaffold on the generic `MESH_*` clustered-app contract.
  - An assembled S03 verifier and public docs surface for the read-only operator/bootstrap story.
requires:
  - slice: S02
    provides: Manifest-declared clustered execution metadata, runtime-owned declared handler submission, and the `cluster-proof` declared-work surface.
affects:
  - S04
  - S05
key_files:
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/operator.rs
  - compiler/mesh-rt/src/lib.rs
  - compiler/meshc/src/cluster.rs
  - compiler/meshc/src/main.rs
  - compiler/meshc/tests/e2e_m044_s03.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/mesh-pkg/src/lib.rs
  - scripts/verify-m044-s03.sh
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/tooling/index.md
  - .gsd/PROJECT.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Live operator inspection now uses a transient authenticated query channel that never registers a peer session.
  - The public clustered scaffold stays narrow: one declared work target, the generic `MESH_*` env contract, and read-only operator inspection via `meshc cluster`.
  - The S03 scaffold acceptance surface is generate/build/start/status inspection, not a deeper continuity-route replay.
patterns_established:
  - Use runtime-owned transport truth first, then expose CLI and scaffold layers on top of that seam.
  - Prove read-only operator behavior against a live clustered runtime and generated scaffold, not just unit-level Rust helpers.
  - When dual-stack listener reachability varies locally, choose the reachable live operator target from runtime truth instead of assuming every advertised address is equally queryable.
observability_surfaces:
  - `meshc cluster status --json`
  - `meshc cluster continuity --json`
  - `meshc cluster diagnostics --json`
  - `.tmp/m044-s03/verify/phase-report.txt`
  - `.tmp/m044-s03/*` retained operator/scaffold e2e artifacts
  - Structured continuity/operator diagnostics in `mesh-rt`
drill_down_paths:
  - .gsd/milestones/M044/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M044/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M044/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M044/slices/S03/tasks/T04-SUMMARY.md
  - .gsd/milestones/M044/slices/S03/tasks/T05-SUMMARY.md
  - .gsd/milestones/M044/slices/S03/tasks/T06-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-30T02:32:18.034Z
blocker_discovered: false
---

# S03: Built-in Operator Surfaces & Clustered Scaffold

**Runtime-owned cluster inspection and the first public clustered-app scaffold now ship together: `mesh-rt` answers transient authenticated operator queries without joining the inspected cluster, `meshc cluster` exposes those read-only surfaces, and `meshc init --clustered` generates a buildable app on the generic `MESH_*` contract.**

## What Happened

S03 closed the gap between the S02 declared-handler runtime seam and an ordinary operator-facing/public bootstrap story.

On the runtime side, `mesh-rt` no longer requires the inspector to join the cluster just to ask membership or continuity questions. `compiler/mesh-rt/src/dist/node.rs` and `compiler/mesh-rt/src/dist/operator.rs` now support a transient authenticated operator query path: the client opens TLS, proves the shared cookie, sends a single operator query, receives a reply, and closes without registering a peer session, exchanging peer lists, or syncing continuity state. The named `operator_query_` and `operator_diagnostics_` rails now prove both the zero-record truth surface and the non-registering behavior.

On the CLI side, `compiler/meshc/src/cluster.rs` adds the public read-only operator commands: `meshc cluster status`, `meshc cluster continuity`, and `meshc cluster diagnostics`. Those commands authenticate with `MESH_CLUSTER_COOKIE`, fail closed on unreachable/auth/malformed responses, and emit either human output or stable JSON for retained artifacts. The live `m044_s03_operator_` rail proves zero-record membership/authority truth, proves the CLI does not appear as a cluster peer, proves continuity lookup after real declared work submit, and proves post-fault diagnostics against a live two-node `cluster-proof` runtime.

On the bootstrap side, `compiler/mesh-pkg/src/scaffold.rs` and `compiler/meshc/src/main.rs` now support `meshc init --clustered`. The generated project is intentionally narrow and public-facing: it declares `Work.execute_declared_work` in `mesh.toml`, ships a small clustered app on the generic `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME`, `MESH_DISCOVERY_SEED`, `MESH_CLUSTER_PORT`, `MESH_CONTINUITY_ROLE`, and `MESH_CONTINUITY_PROMOTION_EPOCH` contract, and points operators at `meshc cluster` instead of proof-app-specific operator routes or `CLUSTER_PROOF_*` env names. The new tooling and `m044_s03_scaffold_` rails prove the scaffold generates, builds, starts in clustered mode, and answers `meshc cluster status` against the generated app.

Finally, S03 adds the assembled verifier and public text. `scripts/verify-m044-s03.sh` now replays the S02 prerequisite, reruns the mesh-rt/operator/scaffold rails, checks the generated scaffold contract, and checks the S03 doc markers. `README.md`, `website/docs/docs/getting-started/index.md`, and `website/docs/docs/tooling/index.md` now teach `meshc init --clustered` plus the read-only `meshc cluster` commands without claiming bounded automatic promotion or a finished `cluster-proof` rewrite.

## Verification

Verified the slice through the exact slice-plan rails plus the docs build: `cargo test -p mesh-rt operator_query_ -- --nocapture`, `cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`, `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`, `bash scripts/verify-m044-s03.sh`, and `npm --prefix website run build`. The assembled verifier retained the S03 proof bundle under `.tmp/m044-s03/verify/`, including named phase status, the generated scaffold replay, and the operator/scaffold e2e logs.

## Requirements Advanced

- R065 — Added the runtime-owned transient operator query path and public `meshc cluster status|continuity|diagnostics` commands, with live proof that inspection is read-only and does not mutate membership.
- R066 — Added `meshc init --clustered`, the generated public clustered app scaffold, and the assembled scaffold-contract verifier on the generic `MESH_*` runtime contract.

## Requirements Validated

- R065 — Validated by `cargo test -p mesh-rt operator_query_ -- --nocapture`, `cargo test -p mesh-rt operator_diagnostics_ -- --nocapture`, `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`, and `bash scripts/verify-m044-s03.sh`.
- R066 — Validated by `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`, `cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture`, and `bash scripts/verify-m044-s03.sh`.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The generated scaffold originally tried to prove a deeper continuity lookup path in the same e2e, but that extra route pressure was not required by the slice contract and introduced harness-only listener timing noise. The final S03 scaffold proof stays on the written contract: generate, build, start in clustered mode, and inspect through `meshc cluster status`.

## Known Limitations

`meshc cluster` is read-only only in S03; there is no manual promotion or mutation surface. `cluster-proof` still exists as the dogfood proof app and still carries its own proof-app HTTP/operator routes; S05 is still responsible for rewriting it fully onto the public clustered-app model. The zero-record authority surface can still report `replication_health=local_only` even after membership convergence; that is truthful runtime state, not a regression.

## Follow-ups

S04 should add bounded automatic promotion on top of the now-public operator/bootstrap model. S05 should rewrite `cluster-proof` onto the public clustered-app contract and reconcile the final docs/proof surface.

## Files Created/Modified

- `compiler/mesh-rt/src/dist/node.rs` — Added the transient authenticated operator query transport and server-side non-registering handling.
- `compiler/mesh-rt/src/dist/operator.rs` — Added remote transient operator query helpers and coverage for non-registering status queries.
- `compiler/mesh-rt/src/lib.rs` — Exported the new transient operator helper surface.
- `compiler/meshc/src/cluster.rs` — Added the public read-only `meshc cluster` command family.
- `compiler/meshc/src/main.rs` — Wired `meshc cluster` and `meshc init --clustered` into the CLI.
- `compiler/meshc/tests/e2e_m044_s03.rs` — Added the live operator and scaffold proof rails.
- `compiler/meshc/tests/tooling_e2e.rs` — Added the clustered scaffold tooling proof.
- `compiler/mesh-pkg/src/scaffold.rs` — Added the clustered scaffold generator and tests.
- `compiler/mesh-pkg/src/lib.rs` — Exported the clustered scaffold entrypoint.
- `scripts/verify-m044-s03.sh` — Added the assembled S03 verifier.
- `README.md` — Documented `meshc init --clustered` and the public `meshc cluster` commands.
- `website/docs/docs/getting-started/index.md` — Documented the clustered scaffold path.
- `website/docs/docs/tooling/index.md` — Documented the clustered scaffold and read-only cluster CLI surface.
- `.gsd/PROJECT.md` — Updated current-state project summary to record M044/S03 as complete.
- `.gsd/KNOWLEDGE.md` — Recorded the zero-record operator `local_only` truth and the longer S03 submit-path timeout expectation for future harness work.
