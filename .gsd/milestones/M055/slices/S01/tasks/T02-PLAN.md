---
estimated_steps: 4
estimated_files: 7
skills_used:
  - bash-scripting
  - powershell-windows
  - test
---

# T02: Centralize repo identity and installer ownership around one contract

**Slice:** S01 — Two-Repo Boundary & GSD Authority Contract
**Milestone:** M055

## Description

Create the durable repo-identity source and make the language-owned installer surfaces validate against it instead of hand-copying repo slug assumptions. This task should keep the language repo identity explicit without pretending the product repo shares the same public URLs.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/lib/repo-identity.json` | fail on missing mesh-lang or hyperpush-mono identity fields before touching consumers | N/A for source assertions | treat malformed JSON or missing URL fields as a real contract break |
| Installer source/copy pair | stop on the first parity mismatch between `tools/install/` and `website/docs/public/` | N/A for source assertions | treat stale slug or diverged copy text as installer-ownership drift |
| `scripts/lib/m034_public_surface_contract.py` | keep the existing local-docs surface green while moving repo-identity data out of hand-copied constants | bounded local command only | treat fallback hardcoding or mismatched expected URLs as public-surface drift |

## Load Profile

- **Shared resources**: the installer source/copy pairs, the local-docs contract helper, and the slice-owned Node contract.
- **Per-operation cost**: one JSON contract plus a handful of source assertions and parity edits.
- **10x breakpoint**: repeated installer-copy diffs and local-docs replays dominate long before file size or CPU becomes relevant.

## Negative Tests

- **Malformed inputs**: stale `snowdamiz/mesh-lang` hardcoding where the new contract should own the value, missing `hyperpush-mono` identity fields, or diverged `tools/install` vs `website/docs/public` bytes.
- **Error paths**: the JSON contract exists, but `scripts/lib/m034_public_surface_contract.py` still embeds a second repo-identity copy and silently disagrees with it.
- **Boundary conditions**: `tools/install/install.{sh,ps1}` stay the editable source pair while `website/docs/public/install.{sh,ps1}` remain mirrored public copies with exact parity.

## Steps

1. Add `scripts/lib/repo-identity.json` with the canonical language-repo and product-repo slugs, repo URLs, issue URLs, installer roots, docs roots, and blob-base values that later slices can consume.
2. Update `scripts/lib/m034_public_surface_contract.py` so the local-docs contract reads or validates against that canonical identity data instead of owning another hand-copied repo slug table.
3. Keep `tools/install/install.sh` and `tools/install/install.ps1` as the editable installer sources, update the mirrored `website/docs/public/install.sh` and `website/docs/public/install.ps1` copies in the same task, and fail closed on parity drift.
4. Extend `scripts/tests/verify-m055-s01-contract.test.mjs` so it catches malformed repo-identity data, stale hardcoded installer slugs, and source/copy mismatches.

## Must-Haves

- [ ] `scripts/lib/repo-identity.json` is the only new canonical repo-identity data file introduced by this slice.
- [ ] `scripts/lib/m034_public_surface_contract.py` no longer owns a conflicting repo-identity copy for installer/docs metadata.
- [ ] `tools/install/install.{sh,ps1}` and `website/docs/public/install.{sh,ps1}` stay byte-parity and match the canonical identity contract.
- [ ] The slice-owned Node contract fails on missing mesh-lang or hyperpush-mono repo identity markers.

## Verification

- `diff -u tools/install/install.sh website/docs/public/install.sh`
- `diff -u tools/install/install.ps1 website/docs/public/install.ps1`
- `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .`
- `node --test scripts/tests/verify-m055-s01-contract.test.mjs`

## Inputs

- `scripts/lib/m034_public_surface_contract.py` — current local-docs/public-surface contract helper with hand-copied repo identity
- `tools/install/install.sh` — editable Unix installer source
- `tools/install/install.ps1` — editable Windows installer source
- `website/docs/public/install.sh` — docs-served Unix installer copy
- `website/docs/public/install.ps1` — docs-served Windows installer copy
- `scripts/tests/verify-m055-s01-contract.test.mjs` — slice-owned contract rail seeded by T01

## Expected Output

- `scripts/lib/repo-identity.json` — canonical language-repo vs product-repo identity contract
- `scripts/lib/m034_public_surface_contract.py` — local-docs contract aligned to the canonical identity data
- `tools/install/install.sh` — installer source aligned to the canonical language repo identity
- `tools/install/install.ps1` — installer source aligned to the canonical language repo identity
- `website/docs/public/install.sh` — public installer copy kept in source parity
- `website/docs/public/install.ps1` — public installer copy kept in source parity
- `scripts/tests/verify-m055-s01-contract.test.mjs` — extended identity and installer-parity assertions
