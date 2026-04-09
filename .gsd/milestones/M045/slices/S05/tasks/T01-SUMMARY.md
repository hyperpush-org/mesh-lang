---
id: T01
parent: S05
milestone: M045
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/getting-started/index.md", "website/docs/.vitepress/config.mts", ".gsd/milestones/M045/slices/S05/tasks/T01-SUMMARY.md"]
key_decisions: ["Keep the getting-started clustered story scaffold-first and point deeper failover/operator proof details at `/docs/distributed-proof/` instead of teaching `cluster-proof`-specific surfaces as the primary contract."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level docs verification passed: `npm --prefix website run build` succeeded, and the required `rg` marker sweep found the new route, sidebar entry, and runtime CLI commands in the expected files. I also previewed the built docs locally and verified in the browser that `/docs/getting-started/` renders the clustered callout/link and `/docs/getting-started/clustered-example/` renders the scaffold-first tutorial with the runtime CLI surfaces and distributed-proof pointer. Slice-level verification was run to record the current boundary honestly: `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` fails because the `e2e_m045_s05` target does not exist yet, and `bash scripts/verify-m045-s05.sh` fails because the wrapper verifier does not exist yet; both are expected T02-owned reds."
completed_at: 2026-03-31T01:43:48.980Z
blocker_discovered: false
---

# T01: Added a dedicated Getting Started clustered tutorial and routed clustered readers to it from the intro page and sidebar.

> Added a dedicated Getting Started clustered tutorial and routed clustered readers to it from the intro page and sidebar.

## What Happened
---
id: T01
parent: S05
milestone: M045
key_files:
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/getting-started/index.md
  - website/docs/.vitepress/config.mts
  - .gsd/milestones/M045/slices/S05/tasks/T01-SUMMARY.md
key_decisions:
  - Keep the getting-started clustered story scaffold-first and point deeper failover/operator proof details at `/docs/distributed-proof/` instead of teaching `cluster-proof`-specific surfaces as the primary contract.
duration: ""
verification_result: mixed
completed_at: 2026-03-31T01:43:48.982Z
blocker_discovered: false
---

# T01: Added a dedicated Getting Started clustered tutorial and routed clustered readers to it from the intro page and sidebar.

**Added a dedicated Getting Started clustered tutorial and routed clustered readers to it from the intro page and sidebar.**

## What Happened

Added `website/docs/docs/getting-started/clustered-example/index.md` as the first-class clustered tutorial and kept it aligned to the real `meshc init --clustered` scaffold in `compiler/mesh-pkg/src/scaffold.rs`. The page teaches the generated files, `Node.start_from_env()`, `Work.execute_declared_work`, the app-owned `POST /work/:request_key` route, the runtime-owned `meshc cluster status|continuity|diagnostics` inspection commands, and a concise same-example failover walkthrough that points deeper bounded failover details at `/docs/distributed-proof/` instead of teaching `cluster-proof` surfaces as the primary contract. Updated `website/docs/docs/getting-started/index.md` to remove the old inline clustered digression and route clustered readers directly to the new page, and updated `website/docs/.vitepress/config.mts` so the Getting Started sidebar exposes `Clustered Example` as a first-class entry.

## Verification

Task-level docs verification passed: `npm --prefix website run build` succeeded, and the required `rg` marker sweep found the new route, sidebar entry, and runtime CLI commands in the expected files. I also previewed the built docs locally and verified in the browser that `/docs/getting-started/` renders the clustered callout/link and `/docs/getting-started/clustered-example/` renders the scaffold-first tutorial with the runtime CLI surfaces and distributed-proof pointer. Slice-level verification was run to record the current boundary honestly: `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` fails because the `e2e_m045_s05` target does not exist yet, and `bash scripts/verify-m045-s05.sh` fails because the wrapper verifier does not exist yet; both are expected T02-owned reds.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix website run build` | 0 | ✅ pass | 54473ms |
| 2 | `rg -n '/docs/getting-started/clustered-example/|meshc init --clustered|meshc cluster status|meshc cluster continuity|meshc cluster diagnostics' website/docs/.vitepress/config.mts website/docs/docs/getting-started/index.md website/docs/docs/getting-started/clustered-example/index.md` | 0 | ✅ pass | 31ms |
| 3 | `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` | 101 | ❌ fail | 1421ms |
| 4 | `bash scripts/verify-m045-s05.sh` | 127 | ❌ fail | 37ms |


## Deviations

None.

## Known Issues

`cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` currently fails with `no test target named e2e_m045_s05`, and `bash scripts/verify-m045-s05.sh` currently fails with `No such file or directory`. Those slice-level rails are still pending T02 rather than indicating a T01 docs regression.

## Files Created/Modified

- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/.vitepress/config.mts`
- `.gsd/milestones/M045/slices/S05/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
`cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` currently fails with `no test target named e2e_m045_s05`, and `bash scripts/verify-m045-s05.sh` currently fails with `No such file or directory`. Those slice-level rails are still pending T02 rather than indicating a T01 docs regression.
