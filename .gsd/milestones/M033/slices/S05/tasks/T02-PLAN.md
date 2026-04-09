---
estimated_steps: 4
estimated_files: 6
skills_used:
  - antfu/skills@vitepress
  - postgresql-database-engineering
---

# T02: Add the canonical S05 verifier and final acceptance replay

Why: R038 only closes once one public command replays the assembled proof stack and mechanically fails when docs drift away from the real boundary.

Steps:
1. Add `scripts/verify-m033-s05.sh` using the same failure-reporting pattern as the existing slice verifiers plus the docs-truth style from `reference-backend/scripts/verify-production-proof-surface.sh`, with a dedicated `.tmp/m033-s05/verify` artifact directory and named phase logs.
2. Make the wrapper run the cheap docs gate first (`npm --prefix website run build`), then an exact-string Python sweep over `website/docs/docs/databases/index.md`, then `bash scripts/verify-m033-s02.sh`, `bash scripts/verify-m033-s03.sh`, and `bash scripts/verify-m033-s04.sh` serially. Do not parallelize: the Postgres-backed proof surfaces share host port `5432`.
3. In the docs-truth sweep, require the exact neutral API names, PG-only API names, honest boundary wording, SQLite-later wording, Mesher-backed file references, and canonical proof commands that the public docs are supposed to stand behind.
4. Tighten `website/docs/docs/databases/index.md` as needed so the final public page includes the new canonical `bash scripts/verify-m033-s05.sh` command and the exact phrases the verifier enforces, without turning the page into zero-raw marketing.

Must-Haves:
- [ ] `scripts/verify-m033-s05.sh` becomes the canonical S05 acceptance command and preserves serial execution across the existing live-Postgres verifiers.
- [ ] The Python docs-truth sweep fails on missing API names, boundary wording, Mesher file anchors, or proof commands instead of silently allowing docs drift.
- [ ] The final docs page and the new script agree on the exact public contract, including the honest leftover / escape-hatch story and the SQLite-later seam.

## Inputs

- `website/docs/docs/databases/index.md`
- `scripts/verify-m033-s02.sh`
- `scripts/verify-m033-s03.sh`
- `scripts/verify-m033-s04.sh`
- `reference-backend/scripts/verify-production-proof-surface.sh`
- `website/package.json`

## Expected Output

- `scripts/verify-m033-s05.sh`
- `website/docs/docs/databases/index.md`

## Verification

bash scripts/verify-m033-s05.sh

## Observability Impact

- Signals added/changed: `scripts/verify-m033-s05.sh` should emit named phase boundaries and per-phase logs under `.tmp/m033-s05/verify/`.
- How a future agent inspects this: rerun `bash scripts/verify-m033-s05.sh` or inspect `.tmp/m033-s05/verify/*.log` to see whether docs build, docs truth, S02, S03, or S04 failed first.
- Failure state exposed: missing public strings, docs build errors, or the first failing underlying verifier should be surfaced without parallel log interleaving or secret-bearing DSNs.
