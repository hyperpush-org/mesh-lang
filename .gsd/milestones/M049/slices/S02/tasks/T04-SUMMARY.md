---
id: T04
parent: S02
milestone: M049
provides: []
requires: []
affects: []
key_files: ["README.md", "website/docs/docs/tooling/index.md", "website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/distributed/index.md", "website/docs/docs/distributed-proof/index.md", "compiler/meshc/tests/e2e_m047_s06.rs", "scripts/verify-m047-s06.sh", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Spell the todo-api starter modes explicitly with `--db sqlite` and `--db postgres`, keep `meshc init --clustered` as the canonical minimal clustered scaffold, and describe `scripts/verify-m047-s05.sh` as a retained historical rail instead of a public starter proof surface.", "When M047 S06 docs wording changes, update both `compiler/meshc/tests/e2e_m047_s06.rs` and `scripts/verify-m047-s06.sh` in the same task."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`bash -n scripts/verify-m047-s06.sh` passed, proving the updated wrapper still parses after the new content-guard rules. `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` passed, proving the docs-contract rail now requires the explicit SQLite/Postgres split and the retained historical M047 wording. `npm --prefix website run build` passed, proving the edited VitePress docs still render and build cleanly."
completed_at: 2026-04-03T00:10:12.815Z
blocker_discovered: false
---

# T04: Split the public todo-api docs into explicit SQLite-local and Postgres-clustered guidance, and retargeted the M047 S06 docs-contract rails to fail on stale clustered-SQLite wording.

> Split the public todo-api docs into explicit SQLite-local and Postgres-clustered guidance, and retargeted the M047 S06 docs-contract rails to fail on stale clustered-SQLite wording.

## What Happened
---
id: T04
parent: S02
milestone: M049
key_files:
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - compiler/meshc/tests/e2e_m047_s06.rs
  - scripts/verify-m047-s06.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Spell the todo-api starter modes explicitly with `--db sqlite` and `--db postgres`, keep `meshc init --clustered` as the canonical minimal clustered scaffold, and describe `scripts/verify-m047-s05.sh` as a retained historical rail instead of a public starter proof surface.
  - When M047 S06 docs wording changes, update both `compiler/meshc/tests/e2e_m047_s06.rs` and `scripts/verify-m047-s06.sh` in the same task.
duration: ""
verification_result: passed
completed_at: 2026-04-03T00:10:12.817Z
blocker_discovered: false
---

# T04: Split the public todo-api docs into explicit SQLite-local and Postgres-clustered guidance, and retargeted the M047 S06 docs-contract rails to fail on stale clustered-SQLite wording.

**Split the public todo-api docs into explicit SQLite-local and Postgres-clustered guidance, and retargeted the M047 S06 docs-contract rails to fail on stale clustered-SQLite wording.**

## What Happened

Rewrote the repo landing page and the M047-facing VitePress pages so they no longer talk about `meshc init --template todo-api` as one generic clustered story. The public contract now shows the honest local path as `meshc init --template todo-api --db sqlite`, points serious shared/deployable guidance at `meshc init --template todo-api --db postgres`, and keeps `meshc init --clustered` as the canonical minimal clustered scaffold. I also reframed the M047 S05/S06 wording so the retained historical clustered Todo rail is described as a bounded fixture-backed proof surface rather than as the live public starter contract, then updated both the Rust docs-contract rail and the assembled shell wrapper to fail closed on stale generic or clustered-SQLite wording.

## Verification

`bash -n scripts/verify-m047-s06.sh` passed, proving the updated wrapper still parses after the new content-guard rules. `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` passed, proving the docs-contract rail now requires the explicit SQLite/Postgres split and the retained historical M047 wording. `npm --prefix website run build` passed, proving the edited VitePress docs still render and build cleanly.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m047-s06.sh` | 0 | ✅ pass | 10ms |
| 2 | `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` | 0 | ✅ pass | 10720ms |
| 3 | `npm --prefix website run build` | 0 | ✅ pass | 50840ms |


## Deviations

The written task only named the Rust docs-contract rail, but the assembled `scripts/verify-m047-s06.sh` wrapper carries its own hard-coded content guards over the same wording. I updated that shell verifier in the same task so the next assembled S06 replay does not fail on stale docs strings after the README/site split lands.

## Known Issues

`cargo test -p meshc --test e2e_m047_s06 -- --nocapture` still emits pre-existing `dead_code` warnings from unrelated support modules. The rail passes, and this task did not change those support crates.

## Files Created/Modified

- `README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/verify-m047-s06.sh`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
The written task only named the Rust docs-contract rail, but the assembled `scripts/verify-m047-s06.sh` wrapper carries its own hard-coded content guards over the same wording. I updated that shell verifier in the same task so the next assembled S06 replay does not fail on stale docs strings after the README/site split lands.

## Known Issues
`cargo test -p meshc --test e2e_m047_s06 -- --nocapture` still emits pre-existing `dead_code` warnings from unrelated support modules. The rail passes, and this task did not change those support crates.
