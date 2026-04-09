---
id: S05
parent: M044
milestone: M044
provides:
  - A `cluster-proof` proof app that follows the same public clustered-app bootstrap and declared-handler contract as ordinary Mesh apps.
  - A single authoritative local closeout command (`bash scripts/verify-m044-s05.sh`) for the scaffold/operator + failover + docs story.
  - Scaffold-first public docs that teach `meshc init --clustered` and `meshc cluster` first, with `cluster-proof` as the deeper proof/runbook layer.
  - A final proof surface that fails closed on legacy route/env wording and stale docs/verifier drift.
requires:
  - slice: S01
    provides: The manifest-declared clustered-app contract and typed continuity/authority surfaces that cluster-proof now consumes directly.
  - slice: S02
    provides: Runtime-owned declared-handler execution and the dogfood declared work boundary used by `cluster-proof`.
  - slice: S03
    provides: `meshc init --clustered`, the public `MESH_*` bootstrap story, and the read-only `meshc cluster` operator surfaces that S05 promotes in docs and the closeout rail.
  - slice: S04
    provides: The bounded automatic promotion/recovery failover contract and retained artifact rail that S05 replays as part of final closeout.
affects:
  []
key_files:
  - cluster-proof/config.mpl
  - cluster-proof/main.mpl
  - cluster-proof/work_continuity.mpl
  - cluster-proof/README.md
  - README.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/tooling/index.md
  - compiler/meshc/tests/e2e_m044_s05.rs
  - compiler/meshc/tests/e2e_m044_s04.rs
  - scripts/verify-m044-s05.sh
  - scripts/verify-m044-s04.sh
key_decisions:
  - Keep `cluster-proof` on the public clustered-app `MESH_*` contract directly and do not preserve compatibility aliases for the old `CLUSTER_PROOF_*` bootstrap names.
  - Treat `cluster-proof` as the deeper dogfood proof consumer behind `meshc init --clustered` + `meshc cluster`, not as the first abstraction ordinary users should learn.
  - Make `scripts/verify-m044-s05.sh` the authoritative closeout command by replaying the S03 and S04 product rails instead of checking docs in isolation.
  - Repair stale verifier seams at the proof surface when the runtime truth is already green instead of weakening the acceptance contract or reopening unrelated runtime code.
patterns_established:
  - Assembled closeout verifiers should replay earlier product rails and retain copied evidence bundles rather than inventing a new docs-only acceptance path.
  - Repo-local verifier snippets should use non-login shells unless they truly require login-shell state; otherwise user dotfiles can turn a green proof red.
  - Pointer artifact files used by downstream wrappers must contain only the data payload they claim to point at, not banner text or command echoes.
observability_surfaces:
  - `GET /membership` for runtime-owned membership and authority truth.
  - `GET /work/:request_key` for keyed continuity truth after submit, failover, and rejoin.
  - `meshc cluster status|continuity|diagnostics --json` as the public read-only operator inspection CLI.
  - Retained same-image failover artifacts under `.tmp/m044-s04/continuity-api-failover-promotion-rejoin-*`.
  - Assembled closeout status, phase logs, copied bundles, and source/docs truth artifacts under `.tmp/m044-s05/verify/`.
drill_down_paths:
  - .gsd/milestones/M044/slices/S05/tasks/T01-SUMMARY.md
  - .gsd/milestones/M044/slices/S05/tasks/T02-SUMMARY.md
  - .gsd/milestones/M044/slices/S05/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-30T08:19:36.087Z
blocker_discovered: false
---

# S05: Cluster-Proof Rewrite, Docs, and Final Closeout

**Rewrote `cluster-proof` onto the public clustered-app contract, removed its legacy explicit clustering path, and closed M044 with a scaffold-first docs and verifier story.**

## What Happened

S05 finished the M044 transition from proof-app folklore to a first-class clustered-app product story.

T01 moved `cluster-proof` onto the same public `MESH_*` bootstrap contract that `meshc init --clustered` already teaches. The proof app now starts from `MESH_CLUSTER_COOKIE`, `MESH_NODE_NAME`, `MESH_DISCOVERY_SEED`, `MESH_CONTINUITY_ROLE`, and `MESH_CONTINUITY_PROMOTION_EPOCH`, with Fly identity fallback only when `MESH_NODE_NAME` is absent. The old proof-app bootstrap dialect is no longer part of the real clustered contract; `CLUSTER_PROOF_WORK_DELAY_MS` remains as the only proof-only timing knob. The live `e2e_m044_s05` public-contract rail now proves explicit node-name startup, Fly-derived identity fallback, malformed-input fail-closed behavior, and rejection of the old `CLUSTER_PROOF_*` env names.

T02 completed the dogfood rewrite inside `cluster-proof` itself. The old `GET /work` probe path is gone, `cluster-proof/work_legacy.mpl` has been deleted, and the app now exposes only the runtime-owned keyed submit/status boundary: `POST /work` and `GET /work/:request_key`. The remaining `cluster-proof` code now consumes the declared-handler/runtime continuity model ordinary clustered apps use instead of carrying app-owned placement and dispatch logic as a second path. The package tests and S05 legacy-cleanup e2e rail prove both the positive keyed contract and the absence of the old surface together.

T03 closed the public story. `scripts/verify-m044-s05.sh` is now the assembled closeout command: it replays the S03 scaffold/operator rail, the S04 automatic promotion/recovery rail, the full `e2e_m044_s05` closeout target, `cluster-proof` build/tests, source/docs truth checks, and the website build. The repo docs (`README.md`, `cluster-proof/README.md`, `website/docs/docs/distributed/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/tooling/index.md`) now teach `meshc init --clustered` plus the read-only `meshc cluster` inspection CLI as the primary clustered-app story, with `cluster-proof` framed as the deeper dogfood/runbook proof consumer.

Closeout also flushed two stale verifier seams that would have left the milestone falsely red after the code and docs were already correct: the S04 failover e2e still asserted a removed `continuity=runtime-native` log marker, and the S04 verifier stack still relied on a login-shell docs check plus a noisy bundle-pointer file that broke the S05 wrapper. Both were repaired at the proof-surface layer instead of weakening the acceptance contract.

## Verification

Passed the full slice matrix.

- `cargo run -q -p meshc -- test cluster-proof/tests/config.test.mpl`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s05 m044_s05_public_contract_ -- --nocapture`
- `cargo run -q -p meshc -- build cluster-proof`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo test -p meshc --test e2e_m044_s05 m044_s05_legacy_cleanup_ -- --nocapture`
- `test ! -e cluster-proof/work_legacy.mpl`
- `cargo test -p meshc --test e2e_m044_s05 -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture` (targeted rerun after fixing stale S04 proof expectations)
- `bash scripts/verify-m044-s04.sh`
- `bash scripts/verify-m044-s05.sh`
- `npm --prefix website run build`

All commands passed. The final assembled closeout rail ended green with `verify-m044-s05: ok`, `.tmp/m044-s05/verify/status.txt=ok`, and copied retained S03/S04 bundles under `.tmp/m044-s05/verify/`.

## Requirements Advanced

- R065 — S05 keeps the built-in runtime/CLI operator surfaces as the primary operator story by centering the docs and closeout verifier on `meshc cluster status|continuity|diagnostics --json` instead of app-authored admin routes.
- R066 — S05 turns the S03 scaffold into the public entrypoint by making `meshc init --clustered` the first clustered-app story in the README and docs and by replaying the scaffold/operator rail inside the final closeout command.
- R068 — S05 replays the destructive S04 automatic promotion/recovery rail from the final closeout verifier, so the rewritten proof package and public docs stay anchored to a still-green failover contract rather than drifting away from runtime truth.

## Requirements Validated

- R069 — Validated by `cargo test -p meshc --test e2e_m044_s05 -- --nocapture`, `cargo run -q -p meshc -- build cluster-proof`, `cargo run -q -p meshc -- test cluster-proof/tests`, `test ! -e cluster-proof/work_legacy.mpl`, and `bash scripts/verify-m044-s05.sh`, which together prove cluster-proof now consumes the public clustered-app standard and no longer carries the old explicit clustering path.
- R070 — Validated by `cargo test -p meshc --test e2e_m044_s05 -- --nocapture`, `bash scripts/verify-m044-s05.sh`, and `npm --prefix website run build`, plus the exact docs/source truth checks over the README, distributed proof pages, tooling page, and cluster-proof runbook that require scaffold-first `meshc init --clustered` + `meshc cluster` guidance and reject legacy route/env folklore.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The product work stayed inside plan, but the final closeout uncovered two stale verifier seams outside the intended feature scope: `compiler/meshc/tests/e2e_m044_s04.rs` was still asserting the removed `continuity=runtime-native` marker, and `scripts/verify-m044-s04.sh` was still using a login-shell docs step plus a noisy `latest-proof-bundle.txt` capture that made the S05 wrapper fail even after the runtime, package, and docs rails were green. Those proof-surface repairs were necessary to make the final assembled acceptance command truthful.

## Known Limitations

The public clustered-app contract remains intentionally bounded. The supported failover topology is still one active primary plus one standby; there is no manual promotion path, no active-active story, no consensus-backed control plane, and no exactly-once execution claim. Fly remains a read-only inspection rail rather than a destructive failover proof surface. The verifier stack is also intentionally text- and artifact-shape-sensitive: exact docs markers and a single-line failover bundle pointer are part of the contract, so careless script or docs edits can make the closeout rail red even when runtime behavior is unchanged.

## Follow-ups

- Use `bash scripts/verify-m044-s05.sh` as the terminal acceptance command for M044 milestone validation and closeout.
- When broader operator controls or wider failover topologies are planned later, treat them as new requirements rather than extending the bounded M044 contract implicitly through docs or verifier drift.

## Files Created/Modified

- `cluster-proof/config.mpl` — Retargeted cluster-proof bootstrap parsing to the public `MESH_*` clustered-app contract and fail-closed validation rules.
- `cluster-proof/main.mpl` — Removed the legacy probe route from the mounted HTTP surface and kept runtime-owned membership/authority handling on the public cluster contract.
- `cluster-proof/work_continuity.mpl` — Left only the keyed runtime-owned continuity submit/status path and current log/response shaping for declared work.
- `cluster-proof/work_legacy.mpl` — Deleted the old explicit clustering probe path entirely.
- `compiler/meshc/tests/e2e_m044_s05.rs` — Added the public-contract, legacy-cleanup, and closeout docs/verifier truth rails for S05.
- `compiler/meshc/tests/e2e_m044_s04.rs` — Updated stale failover log expectations so the destructive S04 rail matches the current cluster-proof startup markers.
- `scripts/verify-m044-s05.sh` — Added the assembled S05 closeout verifier that replays S03/S04, package rails, docs build, and source/docs truth checks.
- `scripts/verify-m044-s04.sh` — Fixed the docs-truth shell invocation and latest-proof-bundle pointer output so the S05 wrapper can consume the retained S04 artifacts truthfully.
- `README.md` — Rewrote the top-level distributed story around scaffold-first clustered apps and the final M044 closeout rail.
- `cluster-proof/README.md` — Repositioned cluster-proof as the deeper dogfood proof consumer, documented the public `MESH_*` contract, and removed the old route/env story.
- `website/docs/docs/distributed/index.md` — Adjusted the distributed guide to point readers at the scaffold-first clustered-app/operator proof story and the bounded failover contract.
- `website/docs/docs/distributed-proof/index.md` — Made the public distributed proof page scaffold-first and aligned it with the final S05 closeout command and bounded failover story.
- `website/docs/docs/tooling/index.md` — Documented `meshc init --clustered` and the read-only `meshc cluster` CLI as first-class tooling surfaces.
