---
id: T02
parent: S02
milestone: M050
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/tooling/index.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "scripts/tests/verify-m050-s02-first-contact-contract.test.mjs", "compiler/meshc/tests/e2e_m047_s05.rs", "compiler/meshc/tests/e2e_m047_s06.rs", "scripts/verify-m047-s06.sh", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Clustered Example now hands off to `/docs/distributed-proof/` instead of listing retained M047 verifier commands inline.", "The stale GitHub README/runbook link fix was applied across the M047-owned docs surfaces so the retained docs rails can pin the current repo identity consistently."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-owned verification passed with the shared first-contact Node contract plus the targeted retained M047 Rust rails. I also ran `bash -n scripts/verify-m047-s06.sh` after changing the wrapper guards so the shell contract itself is syntactically valid. T03-owned slice closeout surfaces are still absent on disk (`compiler/meshc/tests/e2e_m050_s02.rs`, `scripts/verify-m050-s02.sh`), so the assembled slice replay remains future work rather than something this task can partially pass."
completed_at: 2026-04-04T02:01:39.932Z
blocker_discovered: false
---

# T02: Rewrote Clustered Example around scaffold-first truth and moved retained M047 rail discoverability onto the proof page.

> Rewrote Clustered Example around scaffold-first truth and moved retained M047 rail discoverability onto the proof page.

## What Happened
---
id: T02
parent: S02
milestone: M050
key_files:
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - compiler/meshc/tests/e2e_m047_s06.rs
  - scripts/verify-m047-s06.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Clustered Example now hands off to `/docs/distributed-proof/` instead of listing retained M047 verifier commands inline.
  - The stale GitHub README/runbook link fix was applied across the M047-owned docs surfaces so the retained docs rails can pin the current repo identity consistently.
duration: ""
verification_result: passed
completed_at: 2026-04-04T02:01:39.933Z
blocker_discovered: false
---

# T02: Rewrote Clustered Example around scaffold-first truth and moved retained M047 rail discoverability onto the proof page.

**Rewrote Clustered Example around scaffold-first truth and moved retained M047 rail discoverability onto the proof page.**

## What Happened

Rewrote `website/docs/docs/getting-started/clustered-example/index.md` so it now stays on the route-free clustered scaffold first, then hands off in one dedicated follow-on section to the honest local SQLite starter, the serious shared/deployable Postgres starter, and `reference-backend`. Retargeted `compiler/meshc/tests/e2e_m047_s05.rs`, `compiler/meshc/tests/e2e_m047_s06.rs`, and `scripts/verify-m047-s06.sh` so Clustered Example only needs scaffold markers, current repo URLs, starter-split wording, and proof-page discoverability while the heavier direct rail commands remain pinned on the proof-heavy docs. Extended `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` so the slice-owned first-contact contract now covers Clustered Example ordering, current repo-link truth, and rejection of direct proof-rail creep, and updated the related M047-owned docs surfaces to the current `snowdamiz/mesh-lang` GitHub URLs.

## Verification

Task-owned verification passed with the shared first-contact Node contract plus the targeted retained M047 Rust rails. I also ran `bash -n scripts/verify-m047-s06.sh` after changing the wrapper guards so the shell contract itself is syntactically valid. T03-owned slice closeout surfaces are still absent on disk (`compiler/meshc/tests/e2e_m050_s02.rs`, `scripts/verify-m050-s02.sh`), so the assembled slice replay remains future work rather than something this task can partially pass.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` | 0 | ✅ pass | 448ms |
| 2 | `bash -n scripts/verify-m047-s06.sh` | 0 | ✅ pass | 14ms |
| 3 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_public_clustered_surfaces_use_source_first_names_and_todo_template -- --nocapture` | 0 | ✅ pass | 790ms |
| 4 | `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` | 0 | ✅ pass | 851ms |


## Deviations

In addition to the planned Clustered Example rewrite, I updated the same stale `hyperpush-org/hyperpush-mono` README/runbook links in `website/docs/docs/tooling/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/distributed/index.md` so the retained M047 docs rails could enforce the current repo identity consistently instead of leaving three adjacent public lies behind.

## Known Issues

`compiler/meshc/tests/e2e_m050_s02.rs` and `scripts/verify-m050-s02.sh` are still absent because T03 owns the assembled slice verifier. This task leaves the task-owned T02 rails green, but the slice-level closeout wrapper does not exist yet.

## Files Created/Modified

- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/meshc/tests/e2e_m047_s06.rs`
- `scripts/verify-m047-s06.sh`
- `.gsd/KNOWLEDGE.md`


## Deviations
In addition to the planned Clustered Example rewrite, I updated the same stale `hyperpush-org/hyperpush-mono` README/runbook links in `website/docs/docs/tooling/index.md`, `website/docs/docs/distributed-proof/index.md`, and `website/docs/docs/distributed/index.md` so the retained M047 docs rails could enforce the current repo identity consistently instead of leaving three adjacent public lies behind.

## Known Issues
`compiler/meshc/tests/e2e_m050_s02.rs` and `scripts/verify-m050-s02.sh` are still absent because T03 owns the assembled slice verifier. This task leaves the task-owned T02 rails green, but the slice-level closeout wrapper does not exist yet.
