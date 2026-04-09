---
id: T04
parent: S04
milestone: M046
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/e2e_m044_s05.rs", "compiler/meshc/tests/e2e_m045_s04.rs", "compiler/meshc/tests/e2e_m045_s05.rs", "scripts/verify-m044-s05.sh", "scripts/verify-m045-s04.sh", "scripts/verify-m045-s05.sh", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D250: historical M044/M045 cluster-proof wrapper rails now delegate directly to scripts/verify-m046-s04.sh and fail closed on retained phase/bundle artifacts instead of replaying deleted HTTP/package/docs steps."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the three focused Rust verification targets individually, passed the exact slice verification command from the task plan, and passed live happy-path executions of all three rewritten wrapper scripts (verify-m045-s04.sh, verify-m045-s05.sh, and verify-m044-s05.sh) with their delegated M046 retained-artifact checks."
completed_at: 2026-03-31T23:15:05.883Z
blocker_discovered: false
---

# T04: Repointed the stale M044/M045 cluster-proof wrapper rails at the M046 route-free packaged verifier and removed the deleted routeful contract checks.

> Repointed the stale M044/M045 cluster-proof wrapper rails at the M046 route-free packaged verifier and removed the deleted routeful contract checks.

## What Happened
---
id: T04
parent: S04
milestone: M046
key_files:
  - compiler/meshc/tests/e2e_m044_s05.rs
  - compiler/meshc/tests/e2e_m045_s04.rs
  - compiler/meshc/tests/e2e_m045_s05.rs
  - scripts/verify-m044-s05.sh
  - scripts/verify-m045-s04.sh
  - scripts/verify-m045-s05.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D250: historical M044/M045 cluster-proof wrapper rails now delegate directly to scripts/verify-m046-s04.sh and fail closed on retained phase/bundle artifacts instead of replaying deleted HTTP/package/docs steps.
duration: ""
verification_result: passed
completed_at: 2026-03-31T23:15:05.886Z
blocker_discovered: false
---

# T04: Repointed the stale M044/M045 cluster-proof wrapper rails at the M046 route-free packaged verifier and removed the deleted routeful contract checks.

**Repointed the stale M044/M045 cluster-proof wrapper rails at the M046 route-free packaged verifier and removed the deleted routeful contract checks.**

## What Happened

Rewrote compiler/meshc/tests/e2e_m045_s04.rs, compiler/meshc/tests/e2e_m045_s05.rs, and compiler/meshc/tests/e2e_m044_s05.rs from stale routeful/package/docs assertions into focused source-contract guards that only prove the new historical-wrapper truth: each retained rail must delegate to scripts/verify-m046-s04.sh, retain the delegated verify bundle, and reject /work, /membership, delay-hook, and Fly HTTP/package drift as current truth. Replaced the three historical shell verifiers with thin delegated wrappers that run scripts/verify-m046-s04.sh, copy .tmp/m046-s04/verify/ into a retained local bundle, require status.txt, current-phase.txt, phase-report.txt, full-contract.log, and latest-proof-bundle.txt, and republish the retained bundle pointer before running their own focused Rust content-targets. Recorded D250 for the new historical-wrapper pattern and appended a project knowledge note so future work does not pull README/docs/package-smoke assertions back into these M044/M045 aliases while S05 owns broader scaffold/docs parity.

## Verification

Passed the three focused Rust verification targets individually, passed the exact slice verification command from the task plan, and passed live happy-path executions of all three rewritten wrapper scripts (verify-m045-s04.sh, verify-m045-s05.sh, and verify-m044-s05.sh) with their delegated M046 retained-artifact checks.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture` | 0 | ✅ pass | 7272ms |
| 2 | `cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` | 0 | ✅ pass | 7096ms |
| 3 | `cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` | 0 | ✅ pass | 5867ms |
| 4 | `cargo test -p meshc --test e2e_m044_s05 m044_s05_ -- --nocapture && cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture && cargo test -p meshc --test e2e_m045_s05 m045_s05_ -- --nocapture` | 0 | ✅ pass | 4114ms |
| 5 | `bash scripts/verify-m045-s04.sh` | 0 | ✅ pass | 144539ms |
| 6 | `bash scripts/verify-m045-s05.sh` | 0 | ✅ pass | 147664ms |
| 7 | `bash scripts/verify-m044-s05.sh` | 0 | ✅ pass | 143422ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/e2e_m044_s05.rs`
- `compiler/meshc/tests/e2e_m045_s04.rs`
- `compiler/meshc/tests/e2e_m045_s05.rs`
- `scripts/verify-m044-s05.sh`
- `scripts/verify-m045-s04.sh`
- `scripts/verify-m045-s05.sh`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
None.
