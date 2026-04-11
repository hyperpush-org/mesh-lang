---
id: T03
parent: S02
milestone: M050
key_files:
  - website/docs/docs/tooling/index.md
  - scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - scripts/verify-m050-s02.sh
  - compiler/meshc/tests/e2e_m050_s02.rs
  - scripts/verify-m049-s05.sh
  - scripts/tests/verify-m049-s05-contract.test.mjs
  - compiler/meshc/tests/e2e_m049_s05.rs
  - compiler/meshc/tests/e2e_m047_s04.rs
  - scripts/tests/verify-m049-s04-onboarding-contract.test.mjs
  - tools/editors/neovim-mesh/README.md
  - website/docs/public/install.sh
  - website/docs/public/install.ps1
key_decisions:
  - Expose `bash scripts/verify-m050-s02.sh` as the focused first-contact/docs truth replay on the Tooling page instead of folding that responsibility into the heavier release/proof sections.
  - Retain the full `.tmp/m050-s02/verify` bundle inside `scripts/verify-m049-s05.sh` under `retained-m050-s02-verify` so the new docs preflight is inspectable instead of being an orphan wrapper step.
duration: 
verification_result: mixed
completed_at: 2026-04-04T02:23:48.471Z
blocker_discovered: false
---

# T03: Reordered Tooling, added the M050 S02 first-contact docs verifier, and wired it into the retained M049 assembled replay.

**Reordered Tooling, added the M050 S02 first-contact docs verifier, and wired it into the retained M049 assembled replay.**

## What Happened

Rewrote `website/docs/docs/tooling/index.md` so the public flow now stays install/update -> starter choice -> clustered inspection order -> editor support before it drops into the release and retained proof runbooks, and added a new `## Assembled first-contact docs verifier` section that exposes `bash scripts/verify-m050-s02.sh`. Extended `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` so the slice-owned source contract now fails on Tooling drift as well as README / Getting Started drift. Added `scripts/verify-m050-s02.sh` as the focused docs replay: it runs the first-contact source contract, the retained M047 docs rails, the retained M048/M036 tooling contracts, performs a serial VitePress build, and copies built HTML snapshots for Getting Started, Clustered Example, and Tooling into `.tmp/m050-s02/verify/` with a summary JSON. Added `compiler/meshc/tests/e2e_m050_s02.rs` to pin the new verifier’s phase order, built-site evidence, and retained bundle shape. Updated `scripts/verify-m049-s05.sh`, `scripts/tests/verify-m049-s05-contract.test.mjs`, and `compiler/meshc/tests/e2e_m049_s05.rs` so the assembled M049 replay now runs `bash scripts/verify-m050-s02.sh` immediately after the existing M050 S01 docs preflight and retains the copied S02 proof bundle. I also repaired adjacent retained contracts that still pinned older Tooling / Clustered Example wording so the new verifier stack could run truthfully.

## Verification

Task-owned source and contract rails are green: the updated M050 S02 first-contact source contract, the retained M036/M048 tooling contracts, the new Rust verifier contract `e2e_m050_s02`, the updated Rust wrapper contract `e2e_m049_s05`, the retained M047 S04 docs contract, the retained M049 onboarding contract, the retained M050 S01 preflight, and the focused assembled verifier `bash scripts/verify-m050-s02.sh` all passed. The remaining red rail is the final assembled `bash scripts/verify-m049-s05.sh` wrapper, which now gets past the new M050 S02 preflight and the older docs contracts and then stops in the retained `e2e_m049_s01` replay because the configured local Postgres admin database is unreachable on this host.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` | 0 | ✅ pass | 334ms |
| 2 | `node --test scripts/tests/verify-m048-s05-contract.test.mjs` | 0 | ✅ pass | 302ms |
| 3 | `node --test scripts/tests/verify-m036-s03-contract.test.mjs` | 0 | ✅ pass | 763ms |
| 4 | `cargo test -p meshc --test e2e_m050_s02 -- --nocapture` | 0 | ✅ pass | 11000ms |
| 5 | `node --test scripts/tests/verify-m049-s05-contract.test.mjs` | 0 | ✅ pass | 328ms |
| 6 | `cargo test -p meshc --test e2e_m049_s05 -- --nocapture` | 0 | ✅ pass | 3700ms |
| 7 | `cargo test -p meshc --test e2e_m047_s04 -- --nocapture` | 0 | ✅ pass | 4900ms |
| 8 | `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs` | 0 | ✅ pass | 310ms |
| 9 | `bash scripts/verify-m050-s01.sh` | 0 | ✅ pass | 21100ms |
| 10 | `bash scripts/verify-m050-s02.sh` | 0 | ✅ pass | 23000ms |
| 11 | `bash scripts/verify-m049-s05.sh` | 1 | ❌ fail | 75100ms |

## Deviations

In addition to the planned Tooling/page/verifier work, I repaired adjacent retained contracts that still pinned older Tooling / Clustered Example wording (`compiler/meshc/tests/e2e_m047_s04.rs`, `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`, `tools/editors/neovim-mesh/README.md`, and the public installer repo strings) so the new verifier stack could run truthfully instead of inheriting known stale red rails.

## Known Issues

`bash scripts/verify-m049-s05.sh` still fails in the retained `cargo test -p meshc --test e2e_m049_s01 -- --nocapture` replay because the configured local Postgres admin database is unreachable on this host (`connection refused` while clearing isolated databases). Resume by standing up a reachable admin database on the configured `DATABASE_URL` host/port or updating the repo `.env` / `.tmp/m049-s01/local-postgres/connection.env`, then rerun `bash scripts/verify-m049-s05.sh`. No additional docs or wrapper-code failure is currently indicated by the retained bundle.

## Files Created/Modified

- `website/docs/docs/tooling/index.md`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `scripts/verify-m050-s02.sh`
- `compiler/meshc/tests/e2e_m050_s02.rs`
- `scripts/verify-m049-s05.sh`
- `scripts/tests/verify-m049-s05-contract.test.mjs`
- `compiler/meshc/tests/e2e_m049_s05.rs`
- `compiler/meshc/tests/e2e_m047_s04.rs`
- `scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`
- `tools/editors/neovim-mesh/README.md`
- `website/docs/public/install.sh`
- `website/docs/public/install.ps1`
