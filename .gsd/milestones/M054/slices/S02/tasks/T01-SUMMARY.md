---
id: T01
parent: S02
milestone: M054
provides: []
requires: []
affects: []
key_files: ["compiler/mesh-rt/src/http/server.rs", "compiler/meshc/tests/e2e_m047_s07.rs", ".gsd/DECISIONS.md"]
key_decisions: ["D423: expose clustered HTTP request correlation through a runtime-owned `X-Mesh-Continuity-Request-Key` response header and reuse `meshc cluster continuity <node> <request_key> --json` for direct lookup.", "Keep the older continuity-list diff logic only as isolated guardrail unit coverage; the end-to-end proof now treats the response header as the request-selection seam."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-owned verification passed: `cargo test -p mesh-rt m054_s02_ -- --nocapture` proved the runtime injects the correlation header on clustered success and rejection responses without dropping handler headers, and `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` proved the low-level clustered HTTP rail can jump from one response header to one continuity record on both nodes. I also ran the remaining slice-level verification commands once; they currently fail because the later T02/T03 targets and scripts do not exist yet in this tree."
completed_at: 2026-04-06T15:13:21.991Z
blocker_discovered: false
---

# T01: Added a runtime-owned clustered HTTP correlation header and switched the low-level route proof to direct continuity lookup.

> Added a runtime-owned clustered HTTP correlation header and switched the low-level route proof to direct continuity lookup.

## What Happened
---
id: T01
parent: S02
milestone: M054
key_files:
  - compiler/mesh-rt/src/http/server.rs
  - compiler/meshc/tests/e2e_m047_s07.rs
  - .gsd/DECISIONS.md
key_decisions:
  - D423: expose clustered HTTP request correlation through a runtime-owned `X-Mesh-Continuity-Request-Key` response header and reuse `meshc cluster continuity <node> <request_key> --json` for direct lookup.
  - Keep the older continuity-list diff logic only as isolated guardrail unit coverage; the end-to-end proof now treats the response header as the request-selection seam.
duration: ""
verification_result: mixed
completed_at: 2026-04-06T15:13:21.993Z
blocker_discovered: false
---

# T01: Added a runtime-owned clustered HTTP correlation header and switched the low-level route proof to direct continuity lookup.

**Added a runtime-owned clustered HTTP correlation header and switched the low-level route proof to direct continuity lookup.**

## What Happened

Updated the clustered HTTP runtime seam so response construction now preserves handler headers while injecting `X-Mesh-Continuity-Request-Key` on successful clustered responses and on runtime-generated `503` rejection responses when a continuity key exists. Added focused `mesh-rt` unit coverage for successful header preservation and rejection-path correlation, then rewired the low-level two-node clustered HTTP e2e to read the response header from retained raw HTTP, write a sibling `.request-key.txt` artifact, and use the header value directly with `meshc cluster continuity <node> <request_key> --json` on both nodes instead of diffing continuity lists. Recorded D423 so downstream slice work extends the same operator-facing correlation seam.

## Verification

Task-owned verification passed: `cargo test -p mesh-rt m054_s02_ -- --nocapture` proved the runtime injects the correlation header on clustered success and rejection responses without dropping handler headers, and `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` proved the low-level clustered HTTP rail can jump from one response header to one continuity record on both nodes. I also ran the remaining slice-level verification commands once; they currently fail because the later T02/T03 targets and scripts do not exist yet in this tree.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-rt m054_s02_ -- --nocapture` | 0 | ✅ pass | 45819ms |
| 2 | `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` | 0 | ✅ pass | 50673ms |
| 3 | `cargo test -p meshc --test e2e_m054_s02 -- --nocapture` | 101 | ❌ fail | 773ms |
| 4 | `node --test scripts/tests/verify-m054-s02-contract.test.mjs` | 1 | ❌ fail | 380ms |
| 5 | `DATABASE_URL=placeholder bash scripts/verify-m054-s02.sh` | 127 | ❌ fail | 34ms |


## Deviations

Added retained `.request-key.txt` artifacts next to the raw clustered HTTP response captures so header mismatches are inspectable without reparsing the `.http` files. Otherwise the task followed the written plan.

## Known Issues

Slice-level follow-on surfaces are not present yet, so `cargo test -p meshc --test e2e_m054_s02 -- --nocapture`, `node --test scripts/tests/verify-m054-s02-contract.test.mjs`, and `bash scripts/verify-m054-s02.sh` still fail immediately because the target/script files do not exist. That is expected until T02/T03 land.

## Files Created/Modified

- `compiler/mesh-rt/src/http/server.rs`
- `compiler/meshc/tests/e2e_m047_s07.rs`
- `.gsd/DECISIONS.md`


## Deviations
Added retained `.request-key.txt` artifacts next to the raw clustered HTTP response captures so header mismatches are inspectable without reparsing the `.http` files. Otherwise the task followed the written plan.

## Known Issues
Slice-level follow-on surfaces are not present yet, so `cargo test -p meshc --test e2e_m054_s02 -- --nocapture`, `node --test scripts/tests/verify-m054-s02-contract.test.mjs`, and `bash scripts/verify-m054-s02.sh` still fail immediately because the target/script files do not exist. That is expected until T02/T03 land.
