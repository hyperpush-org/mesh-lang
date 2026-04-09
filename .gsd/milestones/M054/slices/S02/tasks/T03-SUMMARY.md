---
id: T03
parent: S02
milestone: M054
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-pkg/src/scaffold.rs", "examples/todo-postgres/README.md", "scripts/verify-m054-s02.sh", "scripts/tests/verify-m054-s02-contract.test.mjs", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["D424: copy the delegated .tmp/m054-s01/verify directory unchanged into the S02 proof bundle, and copy the fresh staged bundle referenced by the new S02 artifact into retained-staged-bundle/ so the assembled verifier stays self-contained without mutating S01."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`node --test scripts/tests/verify-m054-s02-contract.test.mjs` passed and exercised both stale-doc and stale-wrapper failure modes. `bash scripts/verify-m054-s02.sh` passed against a live local Postgres admin URL, which means the wrapper delegated S01 successfully, replayed the direct-correlation S02 e2e, retained the copied S01 verify markers, published `.tmp/m054-s02/verify/latest-proof-bundle.txt`, and validated the copied direct-correlation bundle shape plus redaction guard. I also reran the exact failing S01 e2e against the live container port before the final wrapper replay to confirm the only issue was the stale fallback published port, not the new code."
completed_at: 2026-04-06T15:44:45.752Z
blocker_discovered: false
---

# T03: Added direct-correlation starter guidance and an assembled M054 S02 verifier that delegates S01 and retains a self-contained proof bundle.

> Added direct-correlation starter guidance and an assembled M054 S02 verifier that delegates S01 and retains a self-contained proof bundle.

## What Happened
---
id: T03
parent: S02
milestone: M054
key_files:
  - compiler/mesh-pkg/src/scaffold.rs
  - examples/todo-postgres/README.md
  - scripts/verify-m054-s02.sh
  - scripts/tests/verify-m054-s02-contract.test.mjs
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - D424: copy the delegated .tmp/m054-s01/verify directory unchanged into the S02 proof bundle, and copy the fresh staged bundle referenced by the new S02 artifact into retained-staged-bundle/ so the assembled verifier stays self-contained without mutating S01.
duration: ""
verification_result: passed
completed_at: 2026-04-06T15:44:45.753Z
blocker_discovered: false
---

# T03: Added direct-correlation starter guidance and an assembled M054 S02 verifier that delegates S01 and retains a self-contained proof bundle.

**Added direct-correlation starter guidance and an assembled M054 S02 verifier that delegates S01 and retains a self-contained proof bundle.**

## What Happened

Updated the Postgres starter README template in compiler/mesh-pkg/src/scaffold.rs so the public contract now teaches the runtime-owned X-Mesh-Continuity-Request-Key response header as the operator/debug seam for clustered GET /todos and GET /todos/:id traffic. The generated copy now tells operators to take that request key directly to meshc cluster continuity <node-name@host:port> <request-key> --json, while still reserving continuity-list discovery for startup records or general manual investigation and keeping the boundary against frontend-aware routing, sticky-session claims, and Fly-specific product promises. I rebuilt meshc and re-materialized examples/todo-postgres/README.md through the materializer so the committed example stayed generator-owned.

Added scripts/verify-m054-s02.sh as the assembled S02 wrapper. It delegates scripts/verify-m054-s01.sh, replays cargo test -p meshc --test e2e_m054_s02 -- --nocapture, copies the delegated S01 verify/ directory unchanged, retains the fresh direct-correlation artifact directory, copies the staged bundle referenced by the new S02 artifact into retained-staged-bundle/, and fail-closes on redaction or retained-bundle drift. Added scripts/tests/verify-m054-s02-contract.test.mjs as the cheap contract rail that checks the starter wording, the bounded operator framing, the S01 delegation markers, the retained direct-correlation artifact names, and the no-secret/no-.env wrapper boundaries.

During verification, the first wrapper replay failed before startup because the saved fallback .tmp/m049-s01/local-postgres/connection.env still pointed at an old published Docker port. I reran the exact failing e2e_m054_s01 rail against the live container port derived from container-meta.txt, confirmed the runtime rail was otherwise healthy, reran the assembled wrapper with that corrected base URL, and recorded the fallback-port rule in .gsd/KNOWLEDGE.md.

## Verification

`node --test scripts/tests/verify-m054-s02-contract.test.mjs` passed and exercised both stale-doc and stale-wrapper failure modes. `bash scripts/verify-m054-s02.sh` passed against a live local Postgres admin URL, which means the wrapper delegated S01 successfully, replayed the direct-correlation S02 e2e, retained the copied S01 verify markers, published `.tmp/m054-s02/verify/latest-proof-bundle.txt`, and validated the copied direct-correlation bundle shape plus redaction guard. I also reran the exact failing S01 e2e against the live container port before the final wrapper replay to confirm the only issue was the stale fallback published port, not the new code.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m054-s02-contract.test.mjs` | 0 | ✅ pass | 303ms |
| 2 | `DATABASE_URL=<live local Postgres admin URL> bash scripts/verify-m054-s02.sh` | 0 | ✅ pass | 53400ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/mesh-pkg/src/scaffold.rs`
- `examples/todo-postgres/README.md`
- `scripts/verify-m054-s02.sh`
- `scripts/tests/verify-m054-s02-contract.test.mjs`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
None.
