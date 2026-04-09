---
id: S04
parent: M046
milestone: M046
provides:
  - A rebuilt `cluster-proof/` package that matches the tiny route-free clustered contract: source-owned `clustered(work)`, one `Node.start_from_env()` bootstrap path, stable runtime name `Work.execute_declared_work`, and trivial `1 + 1` work.
  - A shared `compiler/meshc/tests/support/m046_route_free.rs` harness plus `compiler/meshc/tests/e2e_m046_s04.rs` / `scripts/verify-m046-s04.sh` proof surface that downstream slices can reuse for packaged route-free verification.
  - Historical M044/M045 wrapper rails that now fail closed on the new packaged verifier and its retained artifacts instead of reviving deleted `/work`, delay-hook, or Fly HTTP package claims.
requires:
  - slice: S01
    provides: The shared source/manifest clustered-work declaration path and stable declared-handler runtime identity (`Work.execute_declared_work`) that the packaged proof reuses.
  - slice: S02
    provides: The runtime-owned startup trigger/status contract and `declared_handler_runtime_name` CLI surfaces that S04 reuses for route-free packaged startup and status proof.
affects:
  - S05
  - S06
key_files:
  - cluster-proof/mesh.toml
  - cluster-proof/main.mpl
  - cluster-proof/work.mpl
  - cluster-proof/tests/work.test.mpl
  - cluster-proof/README.md
  - cluster-proof/Dockerfile
  - cluster-proof/fly.toml
  - compiler/meshc/tests/support/m046_route_free.rs
  - compiler/meshc/tests/e2e_m046_s04.rs
  - scripts/verify-m046-s04.sh
  - compiler/meshc/tests/e2e_m044_s05.rs
  - compiler/meshc/tests/e2e_m045_s04.rs
  - compiler/meshc/tests/e2e_m045_s05.rs
  - scripts/verify-m044-s05.sh
  - scripts/verify-m045-s04.sh
  - scripts/verify-m045-s05.sh
  - .gsd/PROJECT.md
key_decisions:
  - D247: rebuild `cluster-proof/` around the tiny route-free source contract (`clustered(work)` + `Node.start_from_env()`) instead of preserving the legacy routeful/Fly HTTP proof shape.
  - D248: package `cluster-proof/` as a direct-binary Docker/Fly artifact with no entrypoint wrapper, `PORT`, or `http_service` story.
  - D249: share the route-free package harness through `compiler/meshc/tests/support/m046_route_free.rs`, requiring temp `meshc build --output` targets outside package directories with pre-created parents plus retained `build-meta.json`.
  - D250: keep the M044/M045 historical wrapper rails as thin delegates to `scripts/verify-m046-s04.sh` with retained phase/bundle checks instead of replaying deleted `/work`, delay-hook, or Fly HTTP assertions.
patterns_established:
  - Use package-owned smoke rails that read on-disk source, README, Docker, and Fly files directly so route/proxy/delay drift fails before deeper Rust or runtime proof rails run.
  - Build packaged proof binaries to temp `meshc build --output` paths outside the package directory, require a pre-created parent, and archive `build-meta.json` plus tracked-binary snapshots so proof rails do not churn tracked binaries like `cluster-proof/cluster-proof`.
  - Share route-free build/spawn/CLI polling helpers across local and packaged proof rails in `compiler/meshc/tests/support/` instead of forking near-duplicate harness stacks for `tiny-cluster/` and `cluster-proof/`.
  - Keep historical wrapper rails as thin delegates to the current authoritative verifier and assert retained bundle/status artifacts rather than obsolete product-contract strings.
observability_surfaces:
  - `meshc cluster status --json` on packaged `cluster-proof` nodes for membership and authority truth during the route-free startup proof.
  - `meshc cluster continuity --json` list and single-record output for startup-record discovery, `declared_handler_runtime_name` truth, and completed record inspection on the packaged proof.
  - `meshc cluster diagnostics --json` retained in the packaged proof bundle so startup-trigger / dispatch-window / completion truth stays inspectable without package-owned routes.
  - Retained `.tmp/m046-s04/cluster-proof-startup-two-node-*` evidence with `scenario-meta.json`, `build.log`, `build-meta.json`, `tracked-binary-snapshots.json`, status/continuity/diagnostics JSON, human CLI output, and per-node stdout/stderr logs.
  - Direct verifier artifacts under `.tmp/m046-s04/verify/` including `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt` pointing at `.tmp/m046-s04/verify/retained-m046-s04-artifacts`.
  - The retained packaged-proof bundle shape itself: `cluster-proof-helper-build-meta-*`, `cluster-proof-helper-preflight-*`, `cluster-proof-package-contract-*`, `cluster-proof-package-build-and-test-*`, and `cluster-proof-startup-two-node-*` directories copied under `.tmp/m046-s04/verify/retained-m046-s04-artifacts/`.
drill_down_paths:
  - .gsd/milestones/M046/slices/S04/tasks/T01-SUMMARY.md
  - .gsd/milestones/M046/slices/S04/tasks/T02-SUMMARY.md
  - .gsd/milestones/M046/slices/S04/tasks/T03-SUMMARY.md
  - .gsd/milestones/M046/slices/S04/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T00:10:49.544Z
blocker_discovered: false
---

# S04: Rebuild `cluster-proof/` as tiny packaged proof

**Rebuilt `cluster-proof/` as a tiny packaged route-free proof app on the same `clustered(work)` + `Node.start_from_env()` contract as `tiny-cluster/`, proved it through Mesh-owned CLI/runtime surfaces, and repointed older wrapper rails at the new packaged verifier.**

## What Happened

S04 finished the packaged half of M046 by deleting the old `cluster-proof/` proof-app shape and rebuilding it as the packaged route-free sibling of `tiny-cluster/`. The package now has a package-only `mesh.toml`, a `main.mpl` that only calls `Node.start_from_env()` and logs runtime bootstrap success/failure, and a `work.mpl` that owns one source `clustered(work)` declaration with the stable runtime name `Work.execute_declared_work` and a visible `1 + 1` body. The old `cluster.mpl`, `config.mpl`, `work_continuity.mpl`, `tests/config.test.mpl`, and `docker-entrypoint.sh` seams are gone instead of being preserved as wrappers.

The package contract is now locked in the same route-free style as `tiny-cluster/`. `cluster-proof/tests/work.test.mpl` reads the on-disk source, README, Dockerfile, and Fly config directly and fails closed if `[cluster]`, `declarations`, `HTTP.serve(...)`, `/work`, `/membership`, `Continuity.*`, delay knobs, `docker-entrypoint.sh`, `PORT`, or `http_service` drift back in. `cluster-proof/README.md` now points operators at `meshc cluster status|continuity|diagnostics`, `cluster-proof/Dockerfile` copies only the built binary into the runtime image, and `cluster-proof/fly.toml` keeps only the direct-binary process/build env contract.

S04 also added the shared route-free proof harness under `compiler/meshc/tests/support/m046_route_free.rs`. That helper layer now owns temp-path package builds, `build-meta.json`, tracked-binary snapshot checks, node spawn/log capture, and CLI JSON polling for `meshc cluster status|continuity|diagnostics`. `compiler/meshc/tests/e2e_m046_s04.rs` uses the harness to prove that `cluster-proof` builds to a temp output path outside the package directory, boots two nodes, dedupes one startup record discovered by `declared_handler_runtime_name == "Work.execute_declared_work"`, completes through Mesh-owned CLI/runtime surfaces only, and retains the full proof bundle under `.tmp/m046-s04/...`. The direct verifier `scripts/verify-m046-s04.sh` replays the S03 regression, the package smoke/build commands, the packaged e2e rail, and a retained bundle-shape check so downstream slices can trust one authoritative packaged proof surface.

Finally, S04 closed the historical drift trap. The M044/M045 wrapper Rust tests and shell verifiers no longer replay deleted routeful package steps or assert Fly HTTP/package/docs claims as current truth. They now delegate to `scripts/verify-m046-s04.sh`, copy the delegated verify directory locally, require `status.txt`, `current-phase.txt`, `phase-report.txt`, `full-contract.log`, and `latest-proof-bundle.txt`, and only assert the route-free packaged contract remains the current story. Downstream slices can now treat `cluster-proof/` as the packaged route-free sibling of `tiny-cluster/`, and treat `.tmp/m046-s04/verify/latest-proof-bundle.txt` plus the retained `cluster-proof-startup-two-node-*` bundle as the authoritative packaged proof evidence.

## Verification

Ran every slice-plan verification command and all passed.

- `cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl` — passed in 8876 ms.
- `cargo run -q -p meshc -- test cluster-proof/tests && docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .` — passed in 216456 ms.
- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && bash scripts/verify-m046-s04.sh` — passed in 211080 ms. This also confirmed the packaged observability surfaces: `.tmp/m046-s04/verify/status.txt=ok`, `.tmp/m046-s04/verify/current-phase.txt=complete`, `.tmp/m046-s04/verify/phase-report.txt` with all phases passed, `.tmp/m046-s04/verify/latest-proof-bundle.txt` pointing at `.tmp/m046-s04/verify/retained-m046-s04-artifacts`, and a retained `cluster-proof-startup-two-node-*` bundle containing status/continuity/diagnostics JSON, `build-meta.json`, tracked-binary snapshots, and per-node logs.
- `cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture && cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` — passed in 8826 ms.

Operational readiness for the packaged proof is now explicit: the health signal is the retained verifier status/phase files plus `meshc cluster status|continuity|diagnostics` JSON on the packaged nodes; the failure signal is any contract-guard drift, temp-build preflight failure, duplicate/missing startup record, malformed build metadata, or wrapper-bundle mismatch; the recovery procedure is to rerun `bash scripts/verify-m046-s04.sh` and inspect `phase-report.txt`, `latest-proof-bundle.txt`, the retained `cluster-proof-startup-two-node-*` directory, and node logs; the remaining monitoring gap is cross-surface scaffold/docs alignment and assembled milestone replay, which stay open to S05/S06.

## Requirements Advanced

- R086 — `cluster-proof/` now only denotes clustered work in source and boots through `Node.start_from_env()`, while runtime/tooling surfaces own startup inspection, record discovery, and packaged proof verification instead of app-owned cluster/status modules.
- R087 — The packaged proof now auto-runs clustered work on startup and is exercised entirely through `meshc cluster status|continuity|diagnostics` plus retained verifier artifacts, with no `/work` route or explicit app-side `Continuity.submit_declared_work(...)` call left in package code.
- R091 — S04 brings the packaged proof onto the same Mesh-owned CLI/runtime inspection surfaces as the local proof, adding retained packaged status/continuity/diagnostics evidence that S06 can assemble into the final multi-surface operator story.
- R093 — `cluster-proof/work.mpl` now keeps the workload at literal `1 + 1` and removes delay/timing helpers, so the packaged proof stays intentionally trivial and any remaining orchestration complexity is attributable to Mesh.

## Requirements Validated

- R089 — Validated by `cargo run -q -p meshc -- build cluster-proof && test ! -e cluster-proof/cluster.mpl && test ! -e cluster-proof/config.mpl && test ! -e cluster-proof/work_continuity.mpl`, `cargo run -q -p meshc -- test cluster-proof/tests && docker build -f cluster-proof/Dockerfile -t mesh-cluster-proof:m046-s04-local .`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_ -- --nocapture && cargo test -p meshc --test e2e_m046_s04 m046_s04_ -- --nocapture && bash scripts/verify-m046-s04.sh`, and `cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture && cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture`, which together prove the packaged app has no app-owned clustering, failover, routing, or status logic.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

T01 pulled the obsolete `cluster-proof/tests/config.test.mpl` deletion forward into the source reset so `meshc test cluster-proof/tests` would not stay pointed at deleted modules. T03 also corrected the carried S03 plan/verifier wording to the live completed text and narrowed one brittle README drift guard so the packaged verifier matched the actual route-free contract instead of false-positive substring matches.

## Known Limitations

S04 closes the packaged `cluster-proof/` contract, but equal-surface alignment across the scaffold, `tiny-cluster/`, and `cluster-proof/` is still open to S05. The packaged proof rail currently proves route-free startup/status/runtime inspection truth; the milestone-level assembled replay and public clustered-doc closeout still belong to S06. The historical M044/M045 wrappers now intentionally prove delegation plus retained bundle shape only—they do not re-own scaffold/docs parity or broader product-story assertions.

## Follow-ups

S05 should align `meshc init --clustered`, `tiny-cluster/`, and rebuilt `cluster-proof/` so docs, smoke rails, and verifier guards treat all three as equal clustered-example surfaces. S06 should assemble the local and packaged route-free proofs into one closeout verifier/docs replay and finish requirement-level milestone validation on the unified clustered story.

## Files Created/Modified

- `cluster-proof/mesh.toml` — Replaced the package manifest with a package-only route-free contract and removed manifest-owned cluster declarations.
- `cluster-proof/main.mpl` — Reduced package bootstrap to a single `Node.start_from_env()` path with route-free runtime bootstrap logging.
- `cluster-proof/work.mpl` — Kept the packaged workload at one source-owned `clustered(work)` handler returning `1 + 1` with runtime name `Work.execute_declared_work`.
- `cluster-proof/tests/work.test.mpl` — Locked the route-free package contract in smoke tests by reading source, README, Docker, and Fly files directly and failing closed on drift.
- `cluster-proof/README.md` — Rewrote the packaged runbook around Mesh-owned `meshc cluster status|continuity|diagnostics` inspection instead of package routes or proxy stories.
- `cluster-proof/Dockerfile` — Simplified packaging to a direct-binary multi-stage image that copies only the built `cluster-proof` binary into the runtime image.
- `cluster-proof/fly.toml` — Removed Fly HTTP proxy assumptions and kept only the direct-binary process/build env contract for the packaged proof.
- `compiler/meshc/tests/support/m046_route_free.rs` — Added the shared route-free build/spawn/CLI harness used by both the local and packaged M046 proof rails, including temp build metadata and retained artifact helpers.
- `compiler/meshc/tests/e2e_m046_s04.rs` — Added the packaged `cluster-proof` route-free e2e rail that builds to a temp output path, boots two nodes, verifies CLI/runtime truth, and retains `.tmp/m046-s04/...` evidence.
- `scripts/verify-m046-s04.sh` — Added the direct packaged-proof verifier with contract guards, S03 regression replay, retained bundle copying, and bundle-shape checks.
- `compiler/meshc/tests/e2e_m044_s05.rs` — Repointed the historical wrapper contract rails at the new packaged verifier so M044/M045 aliases fail closed on retained delegation artifacts instead of reviving deleted routeful checks.
- `compiler/meshc/tests/e2e_m045_s04.rs` — Repointed the historical wrapper contract rails at the new packaged verifier so M044/M045 aliases fail closed on retained delegation artifacts instead of reviving deleted routeful checks.
- `compiler/meshc/tests/e2e_m045_s05.rs` — Repointed the historical wrapper contract rails at the new packaged verifier so M044/M045 aliases fail closed on retained delegation artifacts instead of reviving deleted routeful checks.
- `scripts/verify-m044-s05.sh` — Replaced the old wrapper scripts with thin delegates that replay `scripts/verify-m046-s04.sh` and retain the delegated bundle/status artifacts locally.
- `scripts/verify-m045-s04.sh` — Replaced the old wrapper scripts with thin delegates that replay `scripts/verify-m046-s04.sh` and retain the delegated bundle/status artifacts locally.
- `scripts/verify-m045-s05.sh` — Replaced the old wrapper scripts with thin delegates that replay `scripts/verify-m046-s04.sh` and retain the delegated bundle/status artifacts locally.
- `.gsd/PROJECT.md` — Updated project state to record S04 as complete and narrow the remaining M046 work to scaffold/docs alignment and assembled replay.
