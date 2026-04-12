---
estimated_steps: 4
estimated_files: 3
skills_used:
  - test
---

# T01: Publish the backend-expansion handoff in the canonical inventory and maintainer docs

**Slice:** S04 — Canonical maintainer handoff
**Milestone:** M061

## Description

Update the maintainer-facing prose before changing the shell verifier shape so later tasks have a clear contract to lock. Add a final handoff section to `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` that tells backend maintainers how to use the gap map, what order to expand seams in, and which commands to rerun when a row changes. Refresh both supporting READMEs so they point at the canonical inventory plus the final root-level closeout command, and remove the stale product-root description that still treats `mesher/client` as mock-data-only.

## Steps

1. Add a stable `## Maintainer handoff` section to `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` with explicit `### Backend expansion order` and `### Proof commands to rerun` subheadings.
2. Update `../hyperpush-mono/mesher/client/README.md` so it stays a workflow companion, points to the canonical handoff section, and advertises the final root-level closeout wrapper alongside the package-local verifier.
3. Update `../hyperpush-mono/README.md` so the product root points maintainers at the canonical client inventory and no longer describes `mesher/client` as mock-data-only.
4. Keep wording actionable and durable: future backend slices should be able to pick their next seam from these docs without reopening the dashboard code first.

## Must-Haves

- [ ] `ROUTE-INVENTORY.md` contains a stable maintainer handoff section with backend expansion order and proof rerun guidance.
- [ ] `mesher/client/README.md` and the product-root `README.md` point to the canonical inventory plus the final root wrapper command.
- [ ] The root README no longer overstates the client as mock-data-only or sends maintainers to a stale workflow surface.

## Verification

- `python3 - <<'PY'
from pathlib import Path
inventory = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
client_readme = Path('../hyperpush-mono/mesher/client/README.md').read_text()
root_readme = Path('../hyperpush-mono/README.md').read_text()
assert '## Maintainer handoff' in inventory
assert '### Backend expansion order' in inventory
assert '### Proof commands to rerun' in inventory
assert 'bash scripts/verify-m061-s04.sh' in client_readme
assert 'bash scripts/verify-m061-s04.sh' in root_readme
assert 'mock-data TanStack dashboard' not in root_readme
PY`

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical inventory that needs the final handoff section.
- `../hyperpush-mono/mesher/client/README.md` — package workflow companion that must point to the canonical handoff and root wrapper.
- `../hyperpush-mono/README.md` — product-root maintainer surface that still has stale client wording.

## Expected Output

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — final maintainer handoff guidance lives beside the canonical inventory.
- `../hyperpush-mono/mesher/client/README.md` — package workflow points to the canonical handoff and root closeout command.
- `../hyperpush-mono/README.md` — product root accurately surfaces the client truth inventory and closeout command.
