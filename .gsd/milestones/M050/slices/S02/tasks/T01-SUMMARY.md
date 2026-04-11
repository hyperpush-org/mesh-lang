---
id: T01
parent: S02
milestone: M050
key_files:
  - website/docs/docs/getting-started/index.md
  - README.md
  - scripts/tests/verify-m050-s02-first-contact-contract.test.mjs
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M050/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - Kept the literal README/Get Started clustered/proof next-step markers intact so the retained production-proof verifier stays green while the chooser copy moves above them.
  - Used a slice-owned Node contract test to fail closed on starter-command, repo-identity, and ordering drift instead of trying to infer onboarding truth from broader docs prose.
duration: 
verification_result: mixed
completed_at: 2026-04-04T01:46:51.665Z
blocker_discovered: false
---

# T01: Rewrote README and Getting Started around the explicit clustered/SQLite/Postgres starter chooser and added a first-contact docs contract.

**Rewrote README and Getting Started around the explicit clustered/SQLite/Postgres starter chooser and added a first-contact docs contract.**

## What Happened

Rewrote `website/docs/docs/getting-started/index.md` so the public path is now install -> verify -> hello-world -> explicit starter chooser, with proof pages pushed back to follow-on links and the stale source-build clone URL corrected to `https://github.com/snowdamiz/mesh-lang.git`. Tightened `README.md` to teach the same post-hello-world choice while preserving the literal clustered/proof next-step markers the retained production-proof verifier expects. Added `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` as the slice-owned source contract for starter commands, repo identity, and ordering drift, then captured the literal-marker constraint in `.gsd/KNOWLEDGE.md` for later docs work.

## Verification

Task-level verification passed with `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` and `bash reference-backend/scripts/verify-production-proof-surface.sh`. Early slice-level replay also passed the retained M047/M048/M049 rails relevant to this docs change (`e2e_m047_s05`, `e2e_m047_s06`, `verify-m048-s05-contract`, `verify-m049-s05-contract`, `e2e_m049_s05`). The later-task-owned `e2e_m050_s02` and `scripts/verify-m050-s02.sh` surfaces are still absent, and unrelated retained failures remain in `scripts/tests/verify-m036-s03-contract.test.mjs` and `bash scripts/verify-m049-s05.sh`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs` | 0 | ✅ pass | 354ms |
| 2 | `bash reference-backend/scripts/verify-production-proof-surface.sh` | 0 | ✅ pass | 679ms |
| 3 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_public_clustered_surfaces_use_source_first_names_and_todo_template -- --nocapture` | 0 | ✅ pass | 4550ms |
| 4 | `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture` | 0 | ✅ pass | 696ms |
| 5 | `node --test scripts/tests/verify-m048-s05-contract.test.mjs` | 0 | ✅ pass | 412ms |
| 6 | `node --test scripts/tests/verify-m036-s03-contract.test.mjs` | 1 | ❌ fail | 961ms |
| 7 | `cargo test -p meshc --test e2e_m050_s02 -- --nocapture` | 101 | ❌ fail | 207ms |
| 8 | `node --test scripts/tests/verify-m049-s05-contract.test.mjs` | 0 | ✅ pass | 345ms |
| 9 | `cargo test -p meshc --test e2e_m049_s05 -- --nocapture` | 0 | ✅ pass | 631ms |
| 10 | `bash scripts/verify-m050-s02.sh` | 127 | ❌ fail | 10ms |
| 11 | `bash scripts/verify-m049-s05.sh` | 1 | ❌ fail | 51457ms |

## Deviations

None.

## Known Issues

`node --test scripts/tests/verify-m036-s03-contract.test.mjs` is already red on this tree because the retained M036 contract expects Neovim/install-script markers that are currently missing. `cargo test -p meshc --test e2e_m050_s02 -- --nocapture` and `bash scripts/verify-m050-s02.sh` still fail because T03 owns those slice-level verification surfaces and they do not exist yet. `bash scripts/verify-m049-s05.sh` still stops in the retained `e2e_m049_s01` replay (`.tmp/m049-s05/verify/m049-s01-e2e.log`), which is outside this T01 docs rewrite.

## Files Created/Modified

- `website/docs/docs/getting-started/index.md`
- `README.md`
- `scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M050/slices/S02/tasks/T01-SUMMARY.md`
