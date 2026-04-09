# M055 — S05 Research

**Date:** 2026-04-07

## Summary

S05 is a **validation-remediation slice**, not another split-design slice. The repo already has the product-side extraction/materialization contract and the language-side/public-surface contract in source. The live break is narrower:

- The current fast contract surface is **25/26 green**.
- The **only reproducible live source failure** is `scripts/tests/verify-m055-s01-contract.test.mjs`, which reports:
  - `.gsd/PROJECT.md missing "M055 is now the active split-contract milestone."`
- That one drift cascades through the assembled chain:
  - `bash scripts/verify-m055-s01.sh` fails in `m055-s01-contract`
  - `bash scripts/verify-m055-s03.sh` fails immediately in `m055-s01-wrapper`
  - `bash scripts/verify-m055-s04.sh` cannot complete the language-side handoff, so it never publishes the promised repo-attribution outputs

The strongest evidence is already captured in the current repo:

- `.tmp/m055-s03/verify/full-contract.log` shows the nested failure chain.
- `.tmp/m055-s03/verify/m055-s01-wrapper.log` shows the exact S01 contract failure and points at `.gsd/PROJECT.md`.
- `.gsd/milestones/M055/M055-VALIDATION.md` already calls out the same root cause and the same remediation sequence.

The product side is **not** the blocker:

- `scripts/lib/m055-workspace.sh`, `scripts/materialize-hyperpush-mono.mjs`, `scripts/verify-m051-s01.sh`, and the S03/S04 contract tests are currently green.
- The staged product-root proof described in S02/S04 remains the healthy seam.

One operational caveat matters for planning: the current `.tmp/m055-s04/verify/` tree is **not authoritative** even though `status.txt` says `ok`. It is missing the core completion artifacts promised by S04:

- `latest-proof-bundle.txt`
- `language-repo.meta.json`
- `product-repo.meta.json`
- `language-proof-bundle.txt`
- `product-proof-bundle.txt`

Its `phase-report.txt` stops at `language-m055-s03-wrapper\tstarted`. Treat that tree as a stale/interleaved artifact, not as green evidence. This matches the existing M055 knowledge rule: do **not** run `node scripts/materialize-hyperpush-mono.mjs --check` in parallel with `bash scripts/verify-m055-s04.sh`.

## Requirements Focus

This remediation slice is the proof-closeout layer for the M055 contract already established by S01–S04.

### Directly supported table-stakes
- **R116** — examples remain the public start surface; validated indirectly through the retained language-side S03 bundle.
- **R117 / R118** — docs stay evaluator-facing and keep one clear clustered path; validated through `bash scripts/verify-m055-s03.sh` and its retained bundles.
- **R119** — Mesher remains the maintained deeper reference app; validated through the product-root `verify-m051-s01.sh` path preserved in S02/S04.
- **R120** — landing, docs, packages, and product tell one coherent story; this is the main slice-level closeout requirement because the milestone-level evidence chain is currently broken.
- **R121** — packages/public-surface deploy ownership remains part of the normal contract; validated through the retained S03 workflow/public-surface bundle.
- **R122** — SQLite-local vs Postgres-deployable starter split remains honest; validated through the retained language-side docs/starter proofs, not by new product work.

### Candidate M055 requirements this slice actually closes
- Blessed sibling-repo workspace layout
- Repo-local GSD authority plus coordination boundary
- No hidden monorepo path assumptions in public/generated/verifier surfaces
- Cross-repo hosted/local evidence chain with explicit per-repo attribution
- Truthful local Mesher/Hyperpush toolchain contract outside repo-root folklore
- One canonical public repo-identity source

## Recommendation

Treat S05 as **three tight tasks in order**:

1. **Repair the current-state contract** in `.gsd/PROJECT.md`.
   - This is the only live source drift found in fast verification.
   - Do not broaden this into new split work.
   - The file must describe M055 as the **active split-contract milestone**, not complete.

2. **Re-close the language-side verifier chain**.
   - Re-run the fast contract tests.
   - Re-run `bash scripts/verify-m055-s01.sh`.
   - Then re-run `bash scripts/verify-m055-s03.sh`.
   - No additional code changes are likely unless the fresh rerun exposes a second live failure after the `.gsd/PROJECT.md` fix.

3. **Re-close the two-repo evidence assembly**.
   - Run `bash scripts/verify-m055-s04.sh` **in isolation**.
   - Do not run `node scripts/materialize-hyperpush-mono.mjs --check` concurrently.
   - Validate the actual published artifacts, not just `status.txt`.

This should stay a small validation slice. If the `.gsd/PROJECT.md` repair makes S01/S03 green, S04 should be approached as **fresh bundle publication**, not a redesign of materialization, repo identity, or product-root proof ownership.

## Implementation Landscape

### 1. `.gsd/PROJECT.md`
**Role:** current-state project contract.

**Current problem:** it says `M055 is now complete.` even though the milestone validation is `needs-remediation` and S05 is still pending.

**Why it matters:** `scripts/tests/verify-m055-s01-contract.test.mjs` explicitly expects:
- `M055 is now the active split-contract milestone.`
- followed by the two-repo target wording and the repo-local `.gsd` authority wording.

This is the first task seam and the only currently reproducible source drift.

### 2. `scripts/tests/verify-m055-s01-contract.test.mjs`
**Role:** source-of-truth contract for S01 boundary language.

**What it checks:**
- `WORKSPACE.md`
- `README.md`
- `CONTRIBUTING.md`
- `.gsd/PROJECT.md`
- repo identity
- installer parity
- packages/landing/editor ownership surfaces
- `scripts/verify-m055-s01.sh` phase and replay contract

**Current live result:** 14/15 passing within this file; only the `.gsd/PROJECT.md` M055-active marker fails.

### 3. `scripts/verify-m055-s01.sh`
**Role:** authoritative S01 wrapper.

**Phases:**
- `m055-s01-contract`
- `m055-s01-local-docs`
- `m055-s01-packages-build`
- `m055-s01-landing-build`
- `m055-s01-gsd-regression`

**Current live behavior:** red only because `m055-s01-contract` fails.

**Important:** no wrapper-source drift was found in fast contract tests. The task here is to make the existing wrapper green again, not rewrite it.

### 4. `scripts/verify-m055-s03.sh`
**Role:** assembled language-side proof wrapper.

**Depends on:**
- `bash scripts/verify-m055-s01.sh`
- `bash scripts/verify-m050-s02.sh`
- `bash scripts/verify-m050-s03.sh`
- `bash scripts/verify-m051-s04.sh`
- `bash scripts/verify-m034-s05-workflows.sh`
- `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .`
- `npm --prefix packages-website run build`

**Publishes:**
- `.tmp/m055-s03/verify/status.txt`
- `.tmp/m055-s03/verify/current-phase.txt`
- `.tmp/m055-s03/verify/phase-report.txt`
- `.tmp/m055-s03/verify/full-contract.log`
- `.tmp/m055-s03/verify/latest-proof-bundle.txt`
- `.tmp/m055-s03/verify/retained-proof-bundle/`

**Current live behavior:** fails immediately because the nested S01 wrapper is red.

**Natural task seam:** once `.gsd/PROJECT.md` is fixed, re-run this wrapper before touching S04.

### 5. `scripts/verify-m055-s04.sh`
**Role:** final two-repo assembly wrapper.

**Order of operations:**
1. `node scripts/materialize-hyperpush-mono.mjs --check`
2. product-root `bash scripts/verify-m051-s01.sh`
3. product-root `bash scripts/verify-landing-surface.sh`
4. language-root `M055_HYPERPUSH_ROOT=<staged-product> bash scripts/verify-m055-s03.sh`
5. copy language/product verify trees
6. copy pointed child proof bundles
7. generate `language-repo.meta.json` and `product-repo.meta.json`
8. publish language/product proof-bundle pointer files
9. assert retained-bundle shape

**Promised outputs:**
- `.tmp/m055-s04/verify/latest-proof-bundle.txt`
- `.tmp/m055-s04/verify/language-repo.meta.json`
- `.tmp/m055-s04/verify/product-repo.meta.json`
- `.tmp/m055-s04/verify/language-proof-bundle.txt`
- `.tmp/m055-s04/verify/product-proof-bundle.txt`
- `.tmp/m055-s04/verify/retained-proof-bundle/`

**Current live state:** these outputs are missing because the chain stops at `language-m055-s03-wrapper`.

**Natural task seam:** do not debug S04 until S03 is green. If S04 still fails afterward, the likely hotspots are:
- `copy_pointed_bundle_or_fail(...)`
- `capture_repo_metadata_or_fail(...)`
- pointer/bundle-shape assertions

### 6. `scripts/lib/m055-workspace.sh`
**Role:** shared resolver for:
- sibling `hyperpush-mono` root
- canonical language repo slug from `scripts/lib/repo-identity.json`

**Status:** fast S04 contract tests show no live drift here.

**Implication for planning:** do not reopen origin-remote fallback or in-repo `mesher/` shortcuts unless a post-fix rerun produces new evidence.

### 7. `scripts/materialize-hyperpush-mono.mjs`
**Role:** explicit allowlist materializer for the staged `hyperpush-mono` repo.

**Status:** fast contracts are green.

**Operational note:** it shares the staged workspace path used by `bash scripts/verify-m055-s04.sh`. Serialized execution matters.

### 8. `scripts/verify-m051-s01.sh` + staged product verifiers
**Role:** product-side compatibility wrapper and retained Mesher proof chain.

**Status:** validation evidence and S04 contract tests both show this side is healthy.

**Implication:** S05 should not spend time redesigning the product-root proof story unless a fresh post-S01 rerun produces new failures there.

### 9. `.gsd/milestones/M055/M055-VALIDATION.md`
**Role:** already-written milestone validation verdict.

**Why it matters:** it already names the remediation scope precisely:
1. restore truthful pre-completion M055 current-state wording
2. repair `bash scripts/verify-m055-s03.sh`
3. rerun/repair `bash scripts/verify-m055-s04.sh` until repo metadata and bundle pointers publish
4. rerun milestone validation

Use it as the acceptance contract; do not invent a second remediation story.

## Constraints

- **Current-state files must describe current state only.** This comes from the repo’s hard rule and is exactly why `.gsd/PROJECT.md` is failing.
- **Do not treat existing `.tmp/m055-s04/verify/` as authoritative** if it lacks the pointer/meta files promised by the wrapper, even if `status.txt` says `ok`.
- **Do not run `node scripts/materialize-hyperpush-mono.mjs --check` in parallel with `bash scripts/verify-m055-s04.sh`.** The knowledge base explicitly calls this out as a false-red/stale-artifact hazard.
- **Do not reintroduce repo-root product assumptions.** The correct ownership helpers are already in `scripts/lib/m055-workspace.sh` and `scripts/lib/repo-identity.json`.
- **Product ref attribution must remain staged-manifest based** (`materialized:<fingerprint>`) until Hyperpush lives in a real second checkout; the language side should keep using real git SHA attribution.

## Common Pitfalls

- **Editing tests instead of the current-state doc.** The failing source contract is catching real drift.
- **Starting from S04 when S01 is already red.** S04’s language-side failure is downstream of the S01 contract break.
- **Trusting `status.txt` alone.** For S03/S04, pointer files and retained-bundle directories are part of the actual contract.
- **Debugging product-root staging first.** Current evidence says product-root proof is the stable side.
- **Running materialization and final assembly concurrently.** That can create misleading missing-artifact states.

## Verification Approach

### Fast preflight
Run the source contracts first:

- `node --test scripts/tests/verify-m055-s01-contract.test.mjs scripts/tests/verify-m055-s03-contract.test.mjs scripts/tests/verify-m055-s04-contract.test.mjs`

Target result after remediation: **all 26 tests pass**.

### S01 closeout
- `bash scripts/verify-m055-s01.sh`

Required artifacts:
- `.tmp/m055-s01/verify/status.txt` = `ok`
- `.tmp/m055-s01/verify/current-phase.txt` = `complete`
- `.tmp/m055-s01/verify/phase-report.txt` contains all five phase markers as `passed`

### S03 closeout
- `bash scripts/verify-m055-s03.sh`

Required artifacts:
- `.tmp/m055-s03/verify/status.txt` = `ok`
- `.tmp/m055-s03/verify/current-phase.txt` = `complete`
- `.tmp/m055-s03/verify/latest-proof-bundle.txt` exists and points to a real directory
- `.tmp/m055-s03/verify/retained-proof-bundle/` exists

### S04 closeout
Run **serially**:
- `bash scripts/verify-m055-s04.sh`

Required artifacts:
- `.tmp/m055-s04/verify/status.txt` = `ok`
- `.tmp/m055-s04/verify/current-phase.txt` = `complete`
- `.tmp/m055-s04/verify/latest-proof-bundle.txt` exists
- `.tmp/m055-s04/verify/language-repo.meta.json` exists
- `.tmp/m055-s04/verify/product-repo.meta.json` exists
- `.tmp/m055-s04/verify/language-proof-bundle.txt` exists
- `.tmp/m055-s04/verify/product-proof-bundle.txt` exists
- `.tmp/m055-s04/verify/retained-proof-bundle/` exists

Expected `phase-report.txt` markers:
- `init`
- `materialize-hyperpush`
- `product-m051-wrapper`
- `product-landing-wrapper`
- `language-m055-s03-wrapper`
- `retain-language-m055-s03-verify`
- `retain-language-m055-s03-proof-bundle`
- `retain-product-m051-s01-verify`
- `retain-product-m051-s01-proof-bundle`
- `retain-product-landing-surface-verify`
- `repo-metadata`
- `m055-s04-bundle-shape`

### Milestone revalidation
After the three wrappers are green, rerun milestone validation so `M055-VALIDATION.md` no longer records the remediation-round-0 failure state.

## Skills Discovered

| Technology | Skill | Status | Relevance |
| --- | --- | --- | --- |
| Bash verifier assembly | `bash-scripting` | available | Relevant for fail-closed wrapper structure, strict mode, and explicit artifact publication. |
| Workflow/verifier ownership | `github-workflows` | available | Relevant because S03 validates language-owned workflow boundaries and hosted-evidence expectations. |
| Root-cause investigation | `debug-like-expert` | available | Relevant because this slice is a verification-chain regression, not new feature work. |

No additional skill installation was needed.
