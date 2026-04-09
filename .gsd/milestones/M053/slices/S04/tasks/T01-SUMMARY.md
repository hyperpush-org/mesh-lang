---
id: T01
parent: S04
milestone: M053
provides: []
requires: []
affects: []
key_files: ["README.md", "website/docs/docs/getting-started/index.md", "website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/tooling/index.md", "scripts/tests/verify-m050-s02-first-contact-contract.test.mjs", "scripts/verify-m050-s02.sh", "compiler/meshc/tests/e2e_m047_s05.rs", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Keep first-contact pages starter-first by naming SQLite as local-only while routing staged deploy/failover/packages proof through the generated Postgres starter and proof pages.", "Treat the retained M047 docs contract as part of the first-contact verification surface because `scripts/verify-m050-s02.sh` replays it transitively."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`, a clean timed rerun of `bash scripts/verify-m050-s02.sh`, `bash scripts/verify-production-proof-surface.sh`, and `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`. The assembled M050 verifier wrote a green bundle under `.tmp/m050-s02/verify/` with `status.txt=ok`, `current-phase.txt=complete`, and all phase markers passed. The slice-level T03 verifier files (`scripts/tests/verify-m053-s04-contract.test.mjs`, `scripts/verify-m053-s04.sh`) are not present yet, so they remain future-task surfaces rather than regressions from T01."
completed_at: 2026-04-05T21:36:52.271Z
blocker_discovered: false
---

# T01: Aligned first-contact starter docs and verifiers with the M053 SQLite-local/Postgres-proof contract.

> Aligned first-contact starter docs and verifiers with the M053 SQLite-local/Postgres-proof contract.

## What Happened
---
id: T01
parent: S04
milestone: M053
key_files:
  - README.md
  - website/docs/docs/getting-started/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/tooling/index.md
  - scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - scripts/verify-m050-s02.sh
  - compiler/meshc/tests/e2e_m047_s05.rs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep first-contact pages starter-first by naming SQLite as local-only while routing staged deploy/failover/packages proof through the generated Postgres starter and proof pages.
  - Treat the retained M047 docs contract as part of the first-contact verification surface because `scripts/verify-m050-s02.sh` replays it transitively.
duration: ""
verification_result: passed
completed_at: 2026-04-05T21:36:52.273Z
blocker_discovered: false
---

# T01: Aligned first-contact starter docs and verifiers with the M053 SQLite-local/Postgres-proof contract.

**Aligned first-contact starter docs and verifiers with the M053 SQLite-local/Postgres-proof contract.**

## What Happened

Updated `README.md`, Getting Started, Clustered Example, and Tooling so the public starter ladder remains scaffold/examples-first while explicitly calling SQLite local-only/single-node only and routing staged deploy + failover plus hosted packages/public-surface proof through the generated PostgreSQL starter and proof pages. Tightened the task-owned first-contact guardrails in `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` and `scripts/verify-m050-s02.sh` to pin the new wording and built-HTML order. During verification, `bash scripts/verify-m050-s02.sh` exposed a stale retained assertion in `compiler/meshc/tests/e2e_m047_s05.rs`; I aligned that Rust docs contract with the new Tooling wording so the assembled verifier and the public docs now agree. Recorded the verifier dependency in `.gsd/KNOWLEDGE.md` for future slices.

## Verification

Passed `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`, a clean timed rerun of `bash scripts/verify-m050-s02.sh`, `bash scripts/verify-production-proof-surface.sh`, and `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`. The assembled M050 verifier wrote a green bundle under `.tmp/m050-s02/verify/` with `status.txt=ok`, `current-phase.txt=complete`, and all phase markers passed. The slice-level T03 verifier files (`scripts/tests/verify-m053-s04-contract.test.mjs`, `scripts/verify-m053-s04.sh`) are not present yet, so they remain future-task surfaces rather than regressions from T01.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` | 0 | ✅ pass | 270ms |
| 2 | `bash scripts/verify-m050-s02.sh` | 0 | ✅ pass | 33800ms |
| 3 | `bash scripts/verify-production-proof-surface.sh` | 0 | ✅ pass | 13800ms |
| 4 | `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` | 0 | ✅ pass | 9100ms |


## Deviations

Updated `compiler/meshc/tests/e2e_m047_s05.rs` in addition to the planned M050 verifier files because `bash scripts/verify-m050-s02.sh` transitively replays that retained docs contract and it still expected the old Tooling wording.

## Known Issues

None.

## Files Created/Modified

- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `scripts/verify-m050-s02.sh`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `.gsd/KNOWLEDGE.md`


## Deviations
Updated `compiler/meshc/tests/e2e_m047_s05.rs` in addition to the planned M050 verifier files because `bash scripts/verify-m050-s02.sh` transitively replays that retained docs contract and it still expected the old Tooling wording.

## Known Issues
None.
