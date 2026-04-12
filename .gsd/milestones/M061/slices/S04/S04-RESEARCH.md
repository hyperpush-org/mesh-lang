# S04 — Research

**Date:** 2026-04-12

## Summary

S04 is now a **targeted closeout slice** for the two remaining active requirements: **R170** (repeatable evidence-backed proof rail) and **R171** (actionable backend-planning handoff). S01–S03 already delivered the canonical inventory, mixed-surface map, backend gap map, and the fast fail-closed structural contract. What is still missing is the final maintainer-facing packaging: a clear backend-expansion handoff, root-level surfacing, and a closeout verifier/bundle pattern that makes the client truth surface easy to rerun and consume later.

The existing technical seams are already strong. `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` is canonical, `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` is the right fail-closed contract rail, and `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` already emits phase/status logs and reruns the structural + Playwright proof surfaces. The missing handoff is mostly around **what backend maintainers do next** and **how the product root exposes and retains this proof surface**. Right now the inventory has no explicit backend implementation order, the product root still describes `mesher/client` as “the mock-data TanStack dashboard”, and CI only builds the client package instead of acknowledging the route-inventory closeout rail.

Following the loaded **playwright-best-practices** skill, the useful debugging signal is to isolate the failing proof rather than broadening waits everywhere. I confirmed the current prod alert lifecycle proof passes in isolation:

- `env PLAYWRIGHT_PROJECT=prod npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=prod --grep "admin and ops live alerts acknowledge and resolve a real Mesher alert through same-origin refreshes"` ✅

That means the remaining repeatability hazard is not the alert mutation seam itself. The one local rerun hazard I reproduced is earlier in setup: `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` got stuck in `seed-live-issue` because `seed-live-issue.sh` will silently reuse **any** already-running Mesher on its chosen port. `seed-live-admin-ops.sh` already has the better isolation pattern (`pick_available_port` / `configure_backend_endpoint`); `seed-live-issue.sh` does not.

## Recommendation

Treat S04 as a **closeout wrapper + handoff** slice, not another parser/inventory-model slice.

1. **Keep `ROUTE-INVENTORY.md` canonical** and add the missing maintainer handoff content there: a short final section that says how to use the backend gap map, what order to expand backend support in, and what proof commands must stay green when statuses change. That directly closes **R171**.
2. **Add a milestone-level/root-level wrapper** modeled on `verify-maintainer-surface.sh` + `scripts/verify-m051-s01.sh` instead of bloating the existing package verifier with unrelated root concerns. Let the package verifier stay route-inventory-focused; let the new wrapper delegate it, verify root integration markers, and emit a retained proof-bundle pointer. That is the repo’s established closeout pattern and best matches **R170**.
3. **Harden `seed-live-issue.sh` isolation** before treating the wrapper as fully repeatable. The bash-scripting skill’s defensive-pattern guidance fits exactly here: explicit environment ownership, no accidental reuse of stray background services, clear failure modes, and retained artifacts. Mirror the admin/ops seed script’s isolated-port behavior or make reuse opt-in only.
4. **Extend the existing node:test contract** instead of inventing a second drift checker. `verify-client-route-inventory.test.mjs` is already the fail-closed home for cross-file contract markers; use it to lock the new handoff headings, wrapper markers, README/root README references, and any CI or bundle pointer contract.

## Implementation Landscape

### Key Files

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` — canonical route inventory, mixed-surface tables, backend gap map, and invariants. **Currently missing** any explicit backend expansion order / maintainer handoff section.
- `../hyperpush-mono/mesher/client/README.md` — workflow companion for the client package. It already points to `ROUTE-INVENTORY.md` and `verify:route-inventory`, but still frames the inventory mainly as route classification/proof coverage rather than the final backend-planning handoff.
- `../hyperpush-mono/README.md` — product-root maintainer surface. It is currently stale: it still says `mesher/client` is “the mock-data TanStack dashboard” and does not point to the canonical route inventory or client proof rail.
- `../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh` — current package-owned verifier. It already has strict mode, phase/status files, and filtered dev/prod Playwright runs, but **does not** emit a retained proof bundle pointer and still depends on the weaker `seed-live-issue.sh` isolation story.
- `../hyperpush-mono/mesher/scripts/seed-live-issue.sh` — repeatability weak point. `ensure_backend()` immediately reuses any backend that answers on `BASE_URL`, so stale local processes can make the wrapper seed against the wrong runtime or hang in readback.
- `../hyperpush-mono/mesher/scripts/seed-live-admin-ops.sh` — reference implementation for safer seeding. It has `pick_available_port()` and `configure_backend_endpoint()` and should be the model if `seed-live-issue.sh` is hardened.
- `../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs` — canonical fail-closed contract rail. It already locks inventory row parity, recognized proof suites, README/package markers, and verifier phases. This is the right place to add handoff/root-wrapper/root-README/CI marker assertions.
- `../hyperpush-mono/.github/workflows/ci.yml` — currently installs and builds `mesher/client`, but does not acknowledge the route-inventory handoff/proof rail.
- `../hyperpush-mono/scripts/verify-m051-s01.sh` — minimal root wrapper pattern: delegate the package verifier, require `status/current-phase/phase-report/latest-proof-bundle`, and fail closed on drift.
- `../hyperpush-mono/mesher/scripts/verify-maintainer-surface.sh` — full closeout verifier pattern: package contract checks + retained proof bundle + `latest-proof-bundle.txt`. This is the best in-repo reference for S04’s final wrapper shape.
- `../hyperpush-mono/mesher/client/playwright.config.ts` — already follows the right isolation pattern for the browser suites (`workers: 1`, explicit sibling config path, same-origin Mesher backend). Do not broaden or redesign this unless wrapper proof still fails after seed-script isolation is fixed.

### Build Order

1. **Define the handoff content first** in `ROUTE-INVENTORY.md` (and supporting README text): recommended backend expansion order, how to interpret the four support statuses, and what to rerun when a row changes. This is the actual R171 deliverable.
2. **Lock the new contract next** by extending `verify-client-route-inventory.test.mjs`. That gives a fast fail-closed rail for headings, wrapper markers, root README references, and any new bundle-pointer contract before touching slower shell/browser paths.
3. **Add the product-root closeout wrapper** (new file is likely the right move, analogous to `scripts/verify-m051-s01.sh`) plus any product-root README / workflow references it needs.
4. **Harden `seed-live-issue.sh`** if the delegated wrapper still reuses stale backends. This is the most likely setup-level root cause for “repeatable rail” drift in local maintainer reruns.
5. **Only then rerun the full delegated rail** and, if desired, decide whether CI should call the new root wrapper or just lock its markers. The local wrapper/value-add is the important part; CI wiring is optional but useful if the environment cost is acceptable.

### Verification Approach

Fast contract gate:

- `node --test ../hyperpush-mono/mesher/scripts/tests/verify-client-route-inventory.test.mjs`

Current known-good isolated runtime proof:

- `env PLAYWRIGHT_PROJECT=prod npm --prefix ../hyperpush-mono/mesher/client exec -- playwright test --config ../hyperpush-mono/mesher/client/playwright.config.ts --project=prod --grep "admin and ops live alerts acknowledge and resolve a real Mesher alert through same-origin refreshes"`

Delegated package rail after setup hardening:

- `bash ../hyperpush-mono/mesher/scripts/verify-client-route-inventory.sh`

Expected closeout rail if S04 adds a root wrapper:

- `bash ../hyperpush-mono/scripts/verify-m061-s04.sh`

For the closeout wrapper, verification should explicitly confirm:

- delegated verifier ends with `status.txt=ok` and `current-phase.txt=complete`
- required phase markers are present in `phase-report.txt`
- a retained proof-bundle pointer exists (`latest-proof-bundle.txt`)
- README/root README/wrapper markers match the canonical handoff contract

## Constraints

- Keep `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md` as the **single canonical truth surface**. Do not create a parallel handoff registry.
- Preserve `readRouteInventory()` / `parseRouteInventoryMarkdown()` as top-level wrappers unless a new structured handoff table genuinely needs parser support. Most S04 handoff text can stay human-readable and be locked with contract assertions.
- Keep split-workspace execution truthful: from `mesh-lang`, always use the sibling client’s explicit Playwright config path.
- The browser rail already has the correct isolation defaults (`workers: 1`, same-origin boot, explicit config). Prefer fixing seed/setup drift over adding sleeps or weakening assertions.

## Common Pitfalls

- **`seed-live-issue.sh` backend reuse** — unlike `seed-live-admin-ops.sh`, it will reuse any backend already listening on `BASE_URL`. This is the clearest repeatability hazard for local reruns.
- **Stale product-root wording** — `../hyperpush-mono/README.md` still describes `mesher/client` as mock-data-only, which undercuts the final handoff.
- **Assuming CI already proves the handoff rail** — current `ci.yml` only builds `mesher/client`; it does not exercise or even reference the route-inventory closeout surface.
- **Over-extending the parser** — S03 already established the parser contract. For S04, prefer stable headings/tables + node:test assertions over another abstraction layer unless the handoff data truly needs machine-readable structure.

## Open Risks

- If S04 decides to run the full route-inventory wrapper in GitHub Actions, that is a broader environment lift than a docs-only closeout: the job will need Node, Playwright browser install, PostgreSQL client/runtime access, and the sibling `mesh-lang` checkout / `meshc` contract, similar to the Mesher maintainer-surface job.
- If the wrapper stays local-only, the planner should still lock the root README/wrapper markers in `verify-client-route-inventory.test.mjs` so the handoff cannot silently drift back to prose.

## Skills Discovered

| Technology | Skill | Status |
|------------|-------|--------|
| Playwright | `playwright-best-practices` | available |
| Bash scripting | `bash-scripting` | available |
| Node.js `node:test` | none found (`npx skills find "node:test"`) | none found |
