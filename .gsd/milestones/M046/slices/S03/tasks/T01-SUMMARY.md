---
id: T01
parent: S03
milestone: M046
provides: []
requires: []
affects: []
key_files: ["tiny-cluster/mesh.toml", "tiny-cluster/main.mpl", "tiny-cluster/work.mpl", "tiny-cluster/tests/work.test.mpl", "tiny-cluster/README.md", ".gsd/KNOWLEDGE.md", ".gsd/DECISIONS.md"]
key_decisions: ["D241: keep `tiny-cluster` source-first and route-free, using only a package-local `TINY_CLUSTER_WORK_DELAY_MS` hook clamped to 5000 ms for later failover observation.", "Use package-level `File.read` smoke tests to fail fast on manifest/source/control-surface drift before the later Rust e2e rails run."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed with `cargo run -q -p meshc -- build tiny-cluster` and `cargo run -q -p meshc -- test tiny-cluster/tests`. Intermediate slice-level verification also passed for `cargo build -q -p mesh-rt` and `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture`. The T02/T03-owned rails are still expected red in this task state: `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` fails because the test target does not exist yet, and `bash scripts/verify-m046-s03.sh` fails because the verifier script does not exist yet."
completed_at: 2026-03-31T20:12:18.575Z
blocker_discovered: false
---

# T01: Created the real `tiny-cluster` package with a bounded local delay hook and file-backed smoke tests.

> Created the real `tiny-cluster` package with a bounded local delay hook and file-backed smoke tests.

## What Happened
---
id: T01
parent: S03
milestone: M046
key_files:
  - tiny-cluster/mesh.toml
  - tiny-cluster/main.mpl
  - tiny-cluster/work.mpl
  - tiny-cluster/tests/work.test.mpl
  - tiny-cluster/README.md
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - D241: keep `tiny-cluster` source-first and route-free, using only a package-local `TINY_CLUSTER_WORK_DELAY_MS` hook clamped to 5000 ms for later failover observation.
  - Use package-level `File.read` smoke tests to fail fast on manifest/source/control-surface drift before the later Rust e2e rails run.
duration: ""
verification_result: mixed
completed_at: 2026-03-31T20:12:18.577Z
blocker_discovered: false
---

# T01: Created the real `tiny-cluster` package with a bounded local delay hook and file-backed smoke tests.

**Created the real `tiny-cluster` package with a bounded local delay hook and file-backed smoke tests.**

## What Happened

Created the real repo-owned `tiny-cluster/` package from the S02 temp fixture. Added a package-only `mesh.toml`, a route-free `main.mpl` that only boots through `Node.start_from_env()`, a source-declared `clustered(work)` function in `work.mpl`, and a bounded local-only `TINY_CLUSTER_WORK_DELAY_MS` hook that clamps malformed/negative/oversized values to an honest 0..5000 ms window. Added `tiny-cluster/tests/work.test.mpl` as a package-owned smoke rail that proves the trivial `1 + 1` work result, verifies delay normalization, and reads the on-disk package files to fail fast if `[cluster]`, routes, or explicit continuity control calls drift back into source. Wrote `tiny-cluster/README.md` as a local-only runbook that keeps operators on `meshc cluster status|continuity|diagnostics` instead of inventing package-owned routes.

## Verification

Task-level verification passed with `cargo run -q -p meshc -- build tiny-cluster` and `cargo run -q -p meshc -- test tiny-cluster/tests`. Intermediate slice-level verification also passed for `cargo build -q -p mesh-rt` and `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture`. The T02/T03-owned rails are still expected red in this task state: `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` fails because the test target does not exist yet, and `bash scripts/verify-m046-s03.sh` fails because the verifier script does not exist yet.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- build tiny-cluster` | 0 | ✅ pass | 7089ms |
| 2 | `cargo run -q -p meshc -- test tiny-cluster/tests` | 0 | ✅ pass | 7889ms |
| 3 | `cargo build -q -p mesh-rt` | 0 | ✅ pass | 47080ms |
| 4 | `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture` | 0 | ✅ pass | 50676ms |
| 5 | `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture` | 101 | ❌ fail | 1000ms |
| 6 | `bash scripts/verify-m046-s03.sh` | 127 | ❌ fail | 92ms |


## Deviations

Exported small pure delay helper functions from `tiny-cluster/work.mpl` so the package smoke rail could verify malformed/negative/oversized delay behavior directly, and hardened the package smoke rail with repo-root/package-root/tests-root file-read fallbacks after confirming that `meshc test <dir>` does not guarantee one stable working directory.

## Known Issues

`compiler/meshc/tests/e2e_m046_s03.rs` is still absent, so the new slice-specific Rust proof rail is red until T02 lands. `scripts/verify-m046-s03.sh` is still absent, so the assembled slice verifier is red until T03 lands.

## Files Created/Modified

- `tiny-cluster/mesh.toml`
- `tiny-cluster/main.mpl`
- `tiny-cluster/work.mpl`
- `tiny-cluster/tests/work.test.mpl`
- `tiny-cluster/README.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`


## Deviations
Exported small pure delay helper functions from `tiny-cluster/work.mpl` so the package smoke rail could verify malformed/negative/oversized delay behavior directly, and hardened the package smoke rail with repo-root/package-root/tests-root file-read fallbacks after confirming that `meshc test <dir>` does not guarantee one stable working directory.

## Known Issues
`compiler/meshc/tests/e2e_m046_s03.rs` is still absent, so the new slice-specific Rust proof rail is red until T02 lands. `scripts/verify-m046-s03.sh` is still absent, so the assembled slice verifier is red until T03 lands.
