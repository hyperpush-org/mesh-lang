---
id: T02
parent: S03
milestone: M054
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m054-s03.sh", "compiler/meshc/tests/e2e_m054_s03.rs", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M054/slices/S03/tasks/T02-SUMMARY.md"]
key_decisions: ["Kept exact public prose in the Node source-contract rail and limited the shell verifier to built-HTML fragments, test-count enforcement, and retained-bundle shape checks."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran node --test scripts/tests/verify-m054-s03-contract.test.mjs, cargo test -p meshc --test e2e_m054_s03 -- --nocapture, and DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh. The assembled replay delegated bash scripts/verify-m054-s02.sh, reran generate:og and the VitePress build, wrote .tmp/m054-s03/verify/built-html-summary.json, and finished with status.txt=ok, current-phase.txt=complete, and latest-proof-bundle.txt pointing to .tmp/m054-s03/proof-bundles/retained-public-docs-proof-1775493715927587000."
completed_at: 2026-04-06T16:44:28.537Z
blocker_discovered: false
---

# T02: Hardened the S03 docs verifier to replay the Cargo contract, retain built-HTML evidence, and republish its own proof bundle after S02.

> Hardened the S03 docs verifier to replay the Cargo contract, retain built-HTML evidence, and republish its own proof bundle after S02.

## What Happened
---
id: T02
parent: S03
milestone: M054
key_files:
  - scripts/verify-m054-s03.sh
  - compiler/meshc/tests/e2e_m054_s03.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M054/slices/S03/tasks/T02-SUMMARY.md
key_decisions:
  - Kept exact public prose in the Node source-contract rail and limited the shell verifier to built-HTML fragments, test-count enforcement, and retained-bundle shape checks.
duration: ""
verification_result: passed
completed_at: 2026-04-06T16:44:28.538Z
blocker_discovered: false
---

# T02: Hardened the S03 docs verifier to replay the Cargo contract, retain built-HTML evidence, and republish its own proof bundle after S02.

**Hardened the S03 docs verifier to replay the Cargo contract, retain built-HTML evidence, and republish its own proof bundle after S02.**

## What Happened

The local tree already had the initial S03 guard files from the prior task, but the assembled verifier was still short of the written contract: it replayed the Node source contract and S02, yet skipped the Cargo verifier target and did not emit a built-HTML assertion summary. I tightened the implementation instead of recreating it. On the Rust side, compiler/meshc/tests/e2e_m054_s03.rs now archives the verifier and public-doc source surfaces into task artifacts, pins the wrapper’s command order and retained-bundle markers, and adds fail-closed mutation tests for missing Rust-phase replay plus summary/bundle/redaction drift. On the shell side, scripts/verify-m054-s03.sh now reruns cargo test -p meshc --test e2e_m054_s03 -- --nocapture, enforces that the Cargo target actually ran tests, writes .tmp/m054-s03/verify/built-html-summary.json, copies that summary plus the Rust-contract log into the retained proof bundle, and validates that the copied S02 verify tree still has a live latest-proof-bundle.txt pointer. I also recorded the non-obvious contract split in .gsd/KNOWLEDGE.md: exact homepage/proof prose belongs in the Node source-contract rail, while the shell verifier should stick to built-HTML fragments and retained-bundle shape.

## Verification

Ran node --test scripts/tests/verify-m054-s03-contract.test.mjs, cargo test -p meshc --test e2e_m054_s03 -- --nocapture, and DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh. The assembled replay delegated bash scripts/verify-m054-s02.sh, reran generate:og and the VitePress build, wrote .tmp/m054-s03/verify/built-html-summary.json, and finished with status.txt=ok, current-phase.txt=complete, and latest-proof-bundle.txt pointing to .tmp/m054-s03/proof-bundles/retained-public-docs-proof-1775493715927587000.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m054-s03-contract.test.mjs` | 0 | ✅ pass | 929ms |
| 2 | `cargo test -p meshc --test e2e_m054_s03 -- --nocapture` | 0 | ✅ pass | 2378ms |
| 3 | `DATABASE_URL=<redacted> bash scripts/verify-m054-s03.sh` | 0 | ✅ pass | 108100ms |


## Deviations

The planned S03 files already existed locally from T01, so this task hardened them to meet the written contract instead of creating them from scratch. I also used a disposable local Docker Postgres container because the repo had no preconfigured DATABASE_URL, but the assembled verifier only needed a local throwaway admin database.

## Known Issues

None.

## Files Created/Modified

- `scripts/verify-m054-s03.sh`
- `compiler/meshc/tests/e2e_m054_s03.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M054/slices/S03/tasks/T02-SUMMARY.md`


## Deviations
The planned S03 files already existed locally from T01, so this task hardened them to meet the written contract instead of creating them from scratch. I also used a disposable local Docker Postgres container because the repo had no preconfigured DATABASE_URL, but the assembled verifier only needed a local throwaway admin database.

## Known Issues
None.
