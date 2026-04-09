---
estimated_steps: 4
estimated_files: 9
skills_used:
  - debug-like-expert
---

# T03: Publish the mesher limitation matrix and handoff

**Slice:** S01 — Limitation Truth Audit and Repro Matrix
**Milestone:** M032

## Description

Write the slice artifact that later executors can actually use. The tests and script from T01/T02 are only valuable if the slice also leaves a precise matrix of which mesher workaround families are stale, which are still real, what proves each classification, and which later slice should act on them. This task turns the current investigation into an authoritative handoff for S02, S03, and the retained-limit keep-list.

## Steps

1. Use `.gsd/milestones/M032/slices/S01/S01-RESEARCH.md`, the finalized `e2e_m032_*` tests, and `scripts/verify-m032-s01.sh` as the proof inventory, then re-scan `mesher/` with `rg` so the summary covers the actual current comment sites rather than only the ones easiest to remember.
2. Write `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` with clear sections for stale folklore, real blockers, real keep-sites, mixed-truth comments, and next-slice handoff. For each entry, record the mesher path(s), status, proof command or test name, likely owning subsystem, and the next slice that should touch it.
3. Be explicit about the tricky cases: `mesher/storage/writer.mpl` is mixed truth, `mesher/storage/queries.mpl` keeps real ORM-boundary rationale after the stale `from_json` wording is removed, `mesher/ingestion/routes.mpl` route closures require live-request proof, and `xmod_identity` is the priority S02 blocker rather than the stale `from_json` family.
4. Verify the summary file exists and the slice script still passes against the repo state the summary describes.

## Must-Haves

- [ ] `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` classifies the audited mesher families as `stale`, `real blocker`, or `real keep`
- [ ] Every matrix entry points to a concrete proof command or named test, not just prose
- [ ] The summary names the next-slice owner for each family so S02/S03 can execute without re-auditing S01

## Verification

- `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md`
- `rg '^## ' .gsd/milestones/M032/slices/S01/S01-SUMMARY.md`
- `bash scripts/verify-m032-s01.sh`

## Inputs

- `.gsd/milestones/M032/slices/S01/S01-RESEARCH.md` — research findings and repro matrix to distill into the slice handoff
- `compiler/meshc/tests/e2e.rs` — finalized stale-supported and real-limit CLI proofs
- `compiler/meshc/tests/e2e_stdlib.rs` — finalized live-request route proof
- `scripts/verify-m032-s01.sh` — authoritative matrix replay script
- `mesher/ingestion/routes.mpl` — query-string and route-closure comment sites
- `mesher/services/event_processor.mpl` — stale `from_json` wording plus real case-arm keep-site
- `mesher/services/stream_manager.mpl` — stale cast-handler folklore and real nested-`&&` blocker
- `mesher/storage/writer.mpl` — mixed-truth inferred-export comment family
- `mesher/storage/queries.mpl` — mixed-truth raw-SQL / `from_json` rationale

## Expected Output

- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` — authoritative limitation matrix and handoff for S02/S03/S05

## Observability Impact

- Primary inspection surface becomes `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md`: future agents should be able to map any retained or stale mesher comment directly to a named `e2e_m032_*` test, a `scripts/verify-m032-s01.sh` replay step, or a source-proof note.
- No new runtime signals are introduced, but this task must preserve the existing failure-path visibility from T02 by recording the exact failing test names and runtime symptoms for `xmod_identity`, nested `&&`, route closures, timer-service cast mismatch, and single-expression `case` arms.
- If the matrix drifts from reality, the failure should be inspectable through one of three surfaces: the summary entry’s proof command no longer passes, the named Rust test fails, or `bash scripts/verify-m032-s01.sh` stops on the exact repro family and leaves logs under `.tmp/m032-s01/verify/`.
