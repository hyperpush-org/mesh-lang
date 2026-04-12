# S04: Canonical maintainer handoff

**Goal:** Package the canonical Mesher client inventory, backend gap map, and proof rail into a maintainer-facing handoff that is surfaced from the product root, rerunnable without stale-backend drift, and actionable for later backend expansion planning.
**Demo:** The canonical inventory and backend gap map live beside `mesher/client` with a rerunnable drift-proof rail.

## Must-Haves

- Add a final maintainer handoff section to `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` that explains how to read the support-status vocabulary, which backend gaps should be expanded first, and which proof commands must stay green when route or surface rows change.
- Refresh `../hyperpush-mono/mesher/client/README.md` and `../hyperpush-mono/README.md` so they surface the canonical inventory plus the final root-level closeout command, and stop framing `mesher/client` as a mock-data-only dashboard.
- Add `../hyperpush-mono/scripts/verify-m061-s04.sh` as the product-root closeout wrapper that delegates to `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, then fails closed on missing `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt` artifacts.
- Extend `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` and `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` so the package verifier emits a retained proof-bundle pointer, the structural contract locks the new handoff/readme/root-wrapper/CI markers, and drift names the exact missing section or marker.
- Harden `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` so a stray backend already listening on the chosen port is not silently trusted as authoritative unless reuse is explicitly requested.

## Threat Surface

- **Abuse**: a stale local Mesher already bound to the seed port could make the proof rail bless the wrong runtime, and a drifted root wrapper could report success without proving the canonical inventory against the intended app state.
- **Data exposure**: retained proof bundles and logs touch maintainer-only diagnostics around settings, team, API key, and alert surfaces, so the bundle must retain scripts, logs, and file pointers only — never copied secret values, raw API key reveals, or credential-bearing env vars.
- **Input trust**: the verifier and seed scripts trust local `DATABASE_URL`, `BASE_URL`, and `MESHER_*` env configuration plus same-origin seeded API responses; they must fail closed when those inputs point at the wrong runtime or return malformed state.

## Requirement Impact

- **Requirements touched**: `R170`, `R171`
- **Re-verify**: the canonical handoff headings in `ROUTE-INVENTORY.md`, root/client README command surfaces, the root wrapper and retained proof-bundle pointer contract, the seed-live-issue isolation path, and the full delegated dev/prod route-inventory replay.
- **Decisions revisited**: `D524`, `D526`, `D527`, `D531`, `D534`

## Proof Level

- This slice proves: final-assembly
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`

## Observability / Diagnostics

- Runtime signals: `status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, per-phase verifier logs, and retained Playwright `test-results/` artifacts.
- Inspection surfaces: `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`, `../hyperpush-mono/.tmp/m061-s04/verify/`, and the delegated package verifier artifacts under `../hyperpush-mono/mesher/.tmp/`.
- Failure visibility: the wrapper names the failing phase, the delegated verifier keeps per-phase logs, and the retained proof-bundle pointer makes the last successful or failing evidence set easy to inspect after reruns.
- Redaction constraints: keep bundles path-and-log based; do not snapshot one-time API key reveals, copied secrets, or raw credential-bearing env values.

## Integration Closure

- Upstream surfaces consumed: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/client/README.md`, `../hyperpush-mono/README.md`, `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, `../hyperpush-mono/mesher/scripts/seed-live-issue.sh`, `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, `../hyperpush-mono/scripts/verify-m051-s01.sh`, and `../hyperpush-mono/.github/workflows/ci.yml`.
- New wiring introduced in this slice: a product-root closeout wrapper, retained proof-bundle pointering for the package verifier, root README/client README surfacing for the canonical handoff, a lightweight CI acknowledgement of the structural contract, and isolated-by-default live-issue seeding.
- What remains before the milestone is truly usable end-to-end: nothing if the structural contract and `bash ../hyperpush-mono/scripts/verify-m061-s04.sh` both pass.

## Tasks

- [x] **T01: Publish the backend-expansion handoff in the canonical inventory and maintainer docs** `est:70m`
  - Why: R171 is only closed when future backend maintainers can start from the canonical inventory and product-root docs instead of reopening the dashboard code to infer what to build next.
  - Files: `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`, `../hyperpush-mono/mesher/client/README.md`, `../hyperpush-mono/README.md`
  - Do: Add a stable `## Maintainer handoff` section to `ROUTE-INVENTORY.md` with `### Backend expansion order` and `### Proof commands to rerun` subheadings, then refresh the client README and product-root README so they point to the canonical handoff and the final root wrapper command while removing stale mock-only wording.
  - Verify: `python3 - <<'PY'
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
  - Done when: backend maintainers can read the inventory and the two README surfaces and know what to expand next plus which commands must stay green.
- [x] **T02: Add the product-root closeout wrapper and lock the closeout contract markers** `est:100m`
  - Why: R170 requires a fail-closed proof rail and root-level surfacing, not just more prose in the canonical inventory.
  - Files: `../hyperpush-mono/scripts/verify-m061-s04.sh`, `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`, `../hyperpush-mono/.github/workflows/ci.yml`
  - Do: Add a root wrapper modeled on `../hyperpush-mono/scripts/verify-m051-s01.sh`, extend the package verifier to retain a proof bundle and `latest-proof-bundle.txt`, lock the new handoff/README/wrapper/CI markers in the node:test contract, and update CI to acknowledge the structural route-inventory contract truthfully.
  - Verify: `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
  - Done when: the structural contract fails closed on missing handoff headings, stale wrapper markers, missing proof-bundle pointers, or CI drift, and a root-level closeout command exists.
- [ ] **T03: Harden live-issue seeding and prove the closeout rail end to end** `est:110m`
  - Why: the last reproduced repeatability hazard is stale backend reuse in `seed-live-issue.sh`; until that is fixed and the root wrapper passes, the handoff is not actually rerunnable.
  - Files: `../hyperpush-mono/mesher/scripts/seed-live-issue.sh`, `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh`, `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`, `../hyperpush-mono/scripts/verify-m061-s04.sh`
  - Do: Mirror the isolated-port behavior from `seed-live-admin-ops.sh`, keep backend reuse opt-in only, align the delegated verifier and root wrapper around the retained proof-bundle/artifact paths, then rerun the root wrapper and fix only the reproduced setup/runtime drift needed to make it pass.
  - Verify: `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`
  - Done when: the closeout wrapper passes, `seed-live-issue.sh` no longer silently trusts a random running backend, and the final proof-bundle pointer resolves to inspectable evidence.

## Files Likely Touched

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/client/README.md`
- `../hyperpush-mono/README.md`
- `../hyperpush-mono/scripts/verify-m061-s04.sh`
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`
- `../hyperpush-mono/.github/workflows/ci.yml`
- `../hyperpush-mono/mesher/scripts/seed-live-issue.sh`
- `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh`
