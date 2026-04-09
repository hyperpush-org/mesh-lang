---
id: T03
parent: S01
milestone: M032
provides:
  - Published the authoritative S01 limitation matrix with exact proof surfaces, mixed-truth guidance, and downstream slice owners.
key_files:
  - .gsd/milestones/M032/slices/S01/S01-SUMMARY.md
  - .gsd/milestones/M032/slices/S01/S01-PLAN.md
  - .gsd/milestones/M032/slices/S01/tasks/T03-PLAN.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Assigned `xmod_identity` to S02 as the priority real blocker, `from_json` comment surgery to S04, and the retained keep-sites to S05 instead of flattening them into one generic cleanup bucket.
patterns_established:
  - Every M032 matrix entry now records mesher site, status, proof surface, likely owner, and next-slice handoff in one place.
observability_surfaces:
  - test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md
  - rg '^## ' .gsd/milestones/M032/slices/S01/S01-SUMMARY.md
  - cargo test -p meshc --test e2e m032_ -- --nocapture
  - cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture
  - bash scripts/verify-m032-s01.sh
duration: 0h 40m
verification_result: passed
completed_at: 2026-03-24T16:33:03-0400
blocker_discovered: false
---

# T03: Publish the mesher limitation matrix and handoff

**Published the S01 limitation matrix with exact proof surfaces, mixed-truth guidance, and downstream slice owners.**

## What Happened

I first fixed the pre-flight artifact gaps so the task contract matched the real diagnostic surface: `S01-PLAN.md` now includes an explicit route-closure failure-path verification step, and `T03-PLAN.md` now has an `## Observability Impact` section describing how future agents inspect drift.

From there I rebuilt the handoff from live repo state instead of reusing the planner snapshot. I rescanned `mesher/` with `rg`, reread the M032 proof surfaces in `compiler/meshc/tests/e2e.rs`, `compiler/meshc/tests/e2e_stdlib.rs`, and `scripts/verify-m032-s01.sh`, and wrote `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` as the authoritative matrix.

The summary now has the exact sections the slice needed:

- `## Stale Folklore` for the request-query, cross-module `from_json`, service-call `case`, and cast-handler `if/else` families
- `## Real Blockers` for the actual S02 target: `xmod_identity`
- `## Real Keep-Sites` for route closures, nested `&&`, timer-to-service-cast, and parser-bound case-arm helpers
- `## Mixed-Truth Comments` for the tricky `storage/writer`, `storage/queries`, and `event_processor` clusters where stale wording sits next to real rationale
- `## Next-Slice Handoff` with explicit owners for S02, S03, S04, and S05

The rescan also surfaced one non-obvious ledger detail the earlier research did not foreground: the real single-expression case-arm keep-sites extend beyond `event_processor` and `fingerprint` into `api/team.mpl`, `services/retention.mpl`, and `ingestion/pipeline.mpl`. I recorded that in `.gsd/KNOWLEDGE.md` so S05 does not under-count retained parser-bound comments.

## Verification

Task-level verification passed:

- The new slice artifact exists and has the required section layout.
- `bash scripts/verify-m032-s01.sh` still passes against the repo state described by the matrix.

Final slice-level verification also passed:

- all M032 CLI e2e proofs pass
- both M032 stdlib route proofs pass
- the explicit route-closure failure-path check passes
- `meshc fmt --check mesher` and `meshc build mesher` both stay green

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md` | 0 | ✅ pass | 0.09s |
| 2 | `rg '^## ' .gsd/milestones/M032/slices/S01/S01-SUMMARY.md` | 0 | ✅ pass | 0.05s |
| 3 | `cargo test -p meshc --test e2e m032_ -- --nocapture` | 0 | ✅ pass | 13.89s |
| 4 | `cargo test -p meshc --test e2e m032_limit -- --nocapture` | 0 | ✅ pass | 8.49s |
| 5 | `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture` | 0 | ✅ pass | 8.58s |
| 6 | `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture` | 0 | ✅ pass | 7.92s |
| 7 | `bash scripts/verify-m032-s01.sh` | 0 | ✅ pass | 90.76s |
| 8 | `cargo run -q -p meshc -- fmt --check mesher` | 0 | ✅ pass | 7.13s |
| 9 | `cargo run -q -p meshc -- build mesher` | 0 | ✅ pass | 13.45s |

## Diagnostics

The durable inspection path for this task is now:

- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` for the current classification, proof surface, and next-slice owner per family
- `compiler/meshc/tests/e2e.rs` and `compiler/meshc/tests/e2e_stdlib.rs` for stable named proofs when one family regresses
- `bash scripts/verify-m032-s01.sh` plus `.tmp/m032-s01/verify/` for direct replay commands and failure artifacts
- `.gsd/KNOWLEDGE.md` for the widened case-arm keep-site inventory that the initial research snapshot did not fully foreground

## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` — added the authoritative limitation matrix and downstream handoff for S02/S03/S04/S05.
- `.gsd/milestones/M032/slices/S01/S01-PLAN.md` — added the explicit route-closure failure-path verification step required by the pre-flight check.
- `.gsd/milestones/M032/slices/S01/tasks/T03-PLAN.md` — added the missing `## Observability Impact` section.
- `.gsd/KNOWLEDGE.md` — recorded the broader M032 case-arm keep-site inventory so later slices do not miss real parser-bound comments.
- `.gsd/milestones/M032/slices/S01/tasks/T03-SUMMARY.md` — recorded the task outcome and full verification evidence.
