---
id: T02
parent: S03
milestone: M050
key_files:
  - website/docs/docs/production-backend-proof/index.md
  - website/docs/docs/web/index.md
  - website/docs/docs/databases/index.md
  - website/docs/docs/testing/index.md
  - website/docs/docs/concurrency/index.md
  - reference-backend/scripts/verify-production-proof-surface.sh
  - scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Keep `reference-backend/scripts/verify-production-proof-surface.sh` focused on the production proof page/sidebar/runbook seam and move cross-page routing checks into `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`.
duration: 
verification_result: passed
completed_at: 2026-04-04T03:54:10.344Z
blocker_discovered: false
---

# T02: Routed secondary docs through Production Backend Proof and added the S03 secondary-surface contract.

**Routed secondary docs through Production Backend Proof and added the S03 secondary-surface contract.**

## What Happened

Kept `website/docs/docs/production-backend-proof/index.md` compact but made its public-secondary role explicit, added clickable guide/runbook routing there, and updated `website/docs/docs/web/index.md`, `website/docs/docs/databases/index.md`, `website/docs/docs/testing/index.md`, and `website/docs/docs/concurrency/index.md` so each guide now routes through Production Backend Proof before the deeper `reference-backend/README.md` runbook. Rewrote `reference-backend/scripts/verify-production-proof-surface.sh` so it now proves the proof page/sidebar/runbook seam instead of acting like a second onboarding contract, then added `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs` to fail closed on cross-page role and routing drift across the S03 secondary docs surfaces. Also replayed the carried-forward M047 docs cargo rail so slice-level verification status is current.

## Verification

Passed `node --test scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`, `bash reference-backend/scripts/verify-production-proof-surface.sh`, and `npm --prefix website run build`. Also replayed the carried-forward M047 slice rail with `cargo test -p meshc --test e2e_m047_s04 -- --nocapture && cargo test -p meshc --test e2e_m047_s05 -- --nocapture && cargo test -p meshc --test e2e_m047_s06 -- --nocapture`, which passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs` | 0 | ✅ pass | 510ms |
| 2 | `bash reference-backend/scripts/verify-production-proof-surface.sh` | 0 | ✅ pass | 1508ms |
| 3 | `npm --prefix website run build` | 0 | ✅ pass | 21200ms |
| 4 | `cargo test -p meshc --test e2e_m047_s04 -- --nocapture && cargo test -p meshc --test e2e_m047_s05 -- --nocapture && cargo test -p meshc --test e2e_m047_s06 -- --nocapture` | 0 | ✅ pass | 70600ms |

## Deviations

Replayed the carried-forward T01 M047 cargo sequence during T02 closeout so slice-level verification status is current. The implementation scope otherwise stayed inside the written T02 docs/verifier boundary.

## Known Issues

None.

## Files Created/Modified

- `website/docs/docs/production-backend-proof/index.md`
- `website/docs/docs/web/index.md`
- `website/docs/docs/databases/index.md`
- `website/docs/docs/testing/index.md`
- `website/docs/docs/concurrency/index.md`
- `reference-backend/scripts/verify-production-proof-surface.sh`
- `scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`
- `.gsd/KNOWLEDGE.md`
