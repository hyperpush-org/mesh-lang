---
id: S03
parent: M046
milestone: M046
provides:
  - A repo-owned `tiny-cluster/` package that stays route-free, source-first, and visibly trivial (`1 + 1`).
  - A runtime-owned `startup_dispatch_window` observability seam plus CLI-only proof of startup dedupe, automatic promotion/recovery/completion, and fenced rejoin.
  - A direct local verifier (`scripts/verify-m046-s03.sh`) and retained artifact bundle shape that downstream slices can reuse when rebuilding `cluster-proof/` and aligning scaffold/docs surfaces.
requires:
  - slice: S01
    provides: The shared source/manifest clustered-work declaration path and declared-handler runtime identity that S03 consumes for the real package contract and runtime-name discovery.
  - slice: S02
    provides: The runtime-owned startup trigger/status contract and `declared_handler_runtime_name` CLI surfaces that S03 reuses for the real-package startup and failover proofs.
affects:
  - S04
  - S05
  - S06
key_files:
  - tiny-cluster/mesh.toml
  - tiny-cluster/main.mpl
  - tiny-cluster/work.mpl
  - tiny-cluster/tests/work.test.mpl
  - tiny-cluster/README.md
  - compiler/meshc/tests/e2e_m046_s03.rs
  - compiler/mesh-rt/src/dist/node.rs
  - scripts/verify-m046-s03.sh
  - .gsd/milestones/M046/slices/S03/S03-PLAN.md
  - .gsd/PROJECT.md
key_decisions:
  - Keep `tiny-cluster/` source-first and route-free: app code only marks clustered work and boots with `Node.start_from_env()`, while runtime/tooling surfaces own startup, failover, and status truth.
  - Move the failover-observation pending window out of package code and runtime-env guidance into a language-owned `startup_dispatch_window` seam in `mesh-rt`.
  - For enduring plan artifacts that preserve historical task text, assert an explicit current-state override line instead of treating every superseded term as drift.
patterns_established:
  - Use package-owned smoke rails that read on-disk source files to enforce source-first, route-free clustered proof contracts before Rust e2e rails run.
  - For real clustered proof rails, archive the package, verifier contract, pre/post-kill CLI JSON, and per-node logs into one `.tmp/m046-s03/...` bundle so failures stay diagnosable from a single retained directory.
  - When deterministic request ownership matters in a failover test, choose node identities/ports that hash the startup request onto the intended owner instead of relying on speculative retries.
  - For enduring plan artifacts that intentionally preserve historical task text, add and assert a current-state override line instead of trying to ban every superseded term from the whole file.
observability_surfaces:
  - `meshc cluster status --json` for truthful membership and authority during route-free startup, failover, and rejoin.
  - `meshc cluster continuity --json` list/single-record output for startup record discovery, mirrored pending/completed state, owner/replica truth, and `declared_handler_runtime_name`.
  - `meshc cluster diagnostics --json` for `startup_dispatch_window`, `startup_trigger`, `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, `startup_completed`, and `fenced_rejoin` transitions.
  - Retained `.tmp/m046-s03/tiny-cluster-failover-runtime-truth-*` bundles with `scenario-meta.json`, pre/post-kill status/continuity/diagnostics snapshots, and per-node stdout/stderr logs.
  - Direct verifier artifacts under `.tmp/m046-s03/verify/` including `phase-report.txt`, `status.txt`, `current-phase.txt`, `latest-proof-bundle.txt`, and copied retained bundles.
drill_down_paths:
  - .gsd/milestones/M046/slices/S03/tasks/T01-SUMMARY.md
  - .gsd/milestones/M046/slices/S03/tasks/T02-SUMMARY.md
  - .gsd/milestones/M046/slices/S03/tasks/T03-SUMMARY.md
  - .gsd/milestones/M046/slices/S03/tasks/T04-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-31T21:41:37.207Z
blocker_discovered: false
---

# S03: `tiny-cluster/` local no-HTTP proof

**`tiny-cluster/` is now a real repo-owned, route-free clustered proof that keeps the workload at `1 + 1` while proving startup dedupe, failover promotion/recovery/completion, and fenced rejoin entirely through Mesh runtime/CLI surfaces.**

## What Happened

S03 closed the local proof half of M046. The repo now ships `tiny-cluster/` as a real package with a package-only `mesh.toml`, a route-free `main.mpl` that only calls `Node.start_from_env()`, a single source `clustered(work)` declaration, and a package smoke rail that fails closed if `[cluster]`, `HTTP.serve(...)`, `/work`, `/status`, `/health`, explicit continuity calls, or app-owned timing hooks reappear. The work body stays visibly trivial (`1 + 1`), so the remaining complexity on the proof path is Mesh-owned orchestration rather than app code.

`compiler/meshc/tests/e2e_m046_s03.rs` now builds the real repo package, archives the package plus plan/verifier contract into `.tmp/m046-s03/...`, and proves both the two-node startup and destructive failover story entirely through `meshc cluster status|continuity|diagnostics`. The startup rail dedupes one logical record by `declared_handler_runtime_name == "Work.execute_declared_work"`. The failover rail observes a runtime-owned pending window, kills the owner, proves `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, and completion on the standby, then restarts the stale primary and proves `fenced_rejoin` plus post-rejoin continuity truth without any HTTP route or app-owned control plane.

The final slice cleanup removed the remaining user-visible timing seam from the public story. Instead of package-owned delay code or runtime-env guidance, `mesh-rt` now exposes a bounded language-owned `startup_dispatch_window` diagnostic for runtime-owned startup work, while `scripts/verify-m046-s03.sh` fails closed if `tiny-cluster/`, the e2e rail, the README, or the enduring plan drift back toward package/user timing knobs. The verifier replays the prerequisite M046/S02 startup rail plus all S03 package/failover checks, then copies a fresh retained bundle under `.tmp/m046-s03/verify/retained-m046-s03-artifacts` with `phase-report.txt`, `status.txt`, `current-phase.txt`, and `latest-proof-bundle.txt` for downstream debugging.

### Operational Readiness

- **Health signal:** `meshc cluster status --json` shows membership/authority, `meshc cluster continuity --json` shows the startup record and mirrored failover state by `declared_handler_runtime_name`, and `meshc cluster diagnostics --json` records `startup_dispatch_window`, `startup_trigger`, `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, `startup_completed`, and `fenced_rejoin`. The direct verifier also emits `.tmp/m046-s03/verify/status.txt=ok` and a fully passed `phase-report.txt`.
- **Failure signal:** any reintroduction of `[cluster]`, HTTP/status routes, app-owned timing hooks, duplicate startup records, missing promotion/recovery/fenced-rejoin transitions, or missing retained bundle files fails either the package smoke rail, the e2e contract checks, or `scripts/verify-m046-s03.sh` with archived raw artifacts.
- **Recovery procedure:** rerun `cargo test -p mesh-rt startup_work_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`, and `bash scripts/verify-m046-s03.sh`; then inspect `.tmp/m046-s03/verify/phase-report.txt`, `.tmp/m046-s03/verify/04-m046-s03-artifacts.txt`, and the referenced `tiny-cluster-failover-runtime-truth-*` bundle (especially `scenario-meta.json`, pre/post-kill status/continuity/diagnostics JSON, and node stdout/stderr logs).
- **Monitoring gaps:** S03 proves the local `tiny-cluster/` route-free startup/failover/status story, but the packaged `cluster-proof/` rebuild, scaffold parity, and public-doc alignment are still open to S04–S06.

## Verification

Ran the exact slice verification contract and all rails passed.

- `cargo test -p mesh-rt startup_work_ -- --nocapture` — 6 passed
- `cargo run -q -p meshc -- build tiny-cluster` — succeeded
- `cargo run -q -p meshc -- test tiny-cluster/tests` — 3 passed
- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture` — 2 passed
- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture` — 1 passed
- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture` — 3 passed
- `bash scripts/verify-m046-s03.sh` — passed; replayed the focused M046/S02 startup rail, wrote `.tmp/m046-s03/verify/status.txt=ok`, `.tmp/m046-s03/verify/current-phase.txt=complete`, a fully passed `.tmp/m046-s03/verify/phase-report.txt`, and retained `.tmp/m046-s03/verify/retained-m046-s03-artifacts/tiny-cluster-failover-runtime-truth-1774993077181749000`.

These rails confirm the real package contract, route-free startup dedupe, runtime-owned failover pending window, automatic promotion/recovery/completion, fenced rejoin, and retained local proof artifacts.

## Requirements Advanced

- R086 — `tiny-cluster/` proves an app can stay at `clustered(work)` plus `Node.start_from_env()` while runtime-owned startup, failover, recovery, and status semantics stay inside Mesh-owned code and CLI surfaces.
- R091 — S03 extends the route-free CLI contract from startup into failover truth by proving `meshc cluster status|continuity|diagnostics` plus retained verifier artifacts are sufficient to inspect owner/replica state, promotion, completion, and fenced rejoin with no app-owned status route.

## Requirements Validated

- R088 — Validated by `cargo run -q -p meshc -- build tiny-cluster`, `cargo run -q -p meshc -- test tiny-cluster/tests`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_package_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_startup_ -- --nocapture`, `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`, and `bash scripts/verify-m046-s03.sh`, which together prove `tiny-cluster/` exists as a repo-owned, route-free, local-first proof with no app-owned timing helpers.
- R093 — Validated by the same package, startup, failover, and verifier rails plus the package/e2e contract checks that assert `tiny-cluster/work.mpl` stays plain `1 + 1` while the failover pending window is runtime-owned (`startup_dispatch_window`) rather than app-owned.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice started with package-owned and then runtime-env-owned failover timing seams (`TINY_CLUSTER_WORK_DELAY_MS`, then `MESH_STARTUP_WORK_DELAY_MS`) in earlier task states, but S03 closed by moving the pending-window control into a language-owned `startup_dispatch_window` runtime seam so the public package, README, and e2e contract stay timing-seam-free. Because `S03-PLAN.md` preserves historical T01–T03 text, closeout added an explicit T04 current-state override section instead of rewriting history; the verifier and e2e contract now assert that override line directly.

## Known Limitations

- S03 only closes the local `tiny-cluster/` proof; the packaged `cluster-proof/` rebuild is still open to S04.
- Equal-surface scaffold/doc/verifier alignment across `tiny-cluster/`, rebuilt `cluster-proof/`, and `meshc init --clustered` is still open to S05/S06.
- The direct verifier proves the retained local artifact bundle shape, but milestone-level assembled replay and public-doc promotion still remain for later slices.

## Follow-ups

- S04 should rebuild `cluster-proof/` on the same tiny route-free `1 + 1` contract and reuse the S03 CLI-only failover/status proof shape.
- S05 should align `meshc init --clustered`, `tiny-cluster/`, and rebuilt `cluster-proof/` so contract guards, docs, and verifiers treat all three as equal clustered-example surfaces.
- S06 should assemble the local/package/scaffold replay into one closeout verifier and promote the now-truthful clustered story into the public docs layer.

## Files Created/Modified

- `tiny-cluster/mesh.toml` — Created the repo-owned local proof package manifest with no `[cluster]` declaration path and no app-owned control-plane config.
- `tiny-cluster/main.mpl` — Kept the package bootstrap route-free with a single `Node.start_from_env()` entrypoint.
- `tiny-cluster/work.mpl` — Reduced the proof workload to plain `1 + 1` and removed app-owned timing helpers from the declared work body.
- `tiny-cluster/tests/work.test.mpl` — Added package smoke rails that fail closed on source/README drift and timing/control-surface regressions.
- `tiny-cluster/README.md` — Wrote the local operator runbook around `meshc cluster status|continuity|diagnostics` instead of app routes.
- `compiler/meshc/tests/e2e_m046_s03.rs` — Added the real-package startup/failover/rejoin e2e rails and retained-artifact contract for S03.
- `compiler/mesh-rt/src/dist/node.rs` — Moved the pending failover observation window into a language-owned startup dispatch seam and surfaced the corresponding diagnostics.
- `scripts/verify-m046-s03.sh` — Added the direct slice verifier with contract guards, replay phases, and retained-bundle checks.
- `.gsd/milestones/M046/slices/S03/S03-PLAN.md` — Recorded the current-state T04 override so the enduring slice plan preserves history while still stating the live contract.
- `.gsd/PROJECT.md` — Updated project state to reflect S03 completion and the remaining M046 alignment work.
