# S01: Onboarding Graph & Retained Rail Reset — UAT

**Milestone:** M050
**Written:** 2026-04-04T01:18:15.252Z

# S01: Onboarding Graph & Retained Rail Reset — UAT

**Milestone:** M050
**Mode:** Docs-graph and retained-rail verification

## Purpose

Validate the structural public docs reset that this slice shipped:

- `Getting Started` and `Clustered Example` are the only primary first-contact entries.
- `Distributed Proof` and `Production Backend Proof` stay public, but only as secondary proof surfaces.
- `Clustered Example` no longer self-links in the VitePress footer.
- Historical M047 and production-proof docs rails now guard the new structure instead of stale proof-first copy.
- The active M049 assembled wrapper now runs the new M050 docs preflight first.

## Preconditions

- Run from the repo root.
- Node.js, npm, Rust, and Cargo are available.
- No concurrent docs verifiers or VitePress builds are running. `bash scripts/verify-m050-s01.sh` writes `website/docs/.vitepress/dist`, and overlapping docs builds can create false drift.

## Test Cases

### 1. Source-level onboarding graph contract

1. Run `node --test scripts/tests/verify-m050-s01-onboarding-graph.test.mjs`.
2. **Expected:** 5 passing tests.
3. **Expected contract details:**
   - `website/docs/.vitepress/config.mts` keeps `Getting Started` limited to `/docs/getting-started/` and `/docs/getting-started/clustered-example/`.
   - `Proof Surfaces` is the final `/docs/` sidebar group and contains exactly `/docs/distributed-proof/` and `/docs/production-backend-proof/`.
   - Both proof sidebar items carry `includeInFooter: false`.
   - `website/docs/.vitepress/theme/composables/usePrevNext.ts` uses exact normalized page equality instead of prefix matching.
   - Computed footer flow is `Getting Started -> Clustered Example -> Language Basics`, and neither proof page participates in footer navigation.
4. **Expected negative coverage:** the same Node test also fail-closes on four temp mutations:
   - proof page moved back into a primary sidebar group
   - footer resolver regresses to prefix matching
   - proof pages lose footer opt-out markers
   - proof-surface route is typoed

### 2. Retained M047 and production-proof docs rails

1. Run `cargo test -p meshc --test e2e_m047_s04 -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m047_s06 m047_s06_ -- --nocapture`.
3. Run `bash reference-backend/scripts/verify-production-proof-surface.sh`.
4. **Expected:** all three pass.
5. **Expected behavior:**
   - the retained M047 rails now accept the new public graph and bounded proof-page discoverability markers
   - the production proof page stays public-secondary, footer-opted-out, and linked behind `Getting Started -> Clustered Example`
   - none of these rails require the old proof-first intro wording to stay green

### 3. Built-site docs-graph verifier

1. Run `bash scripts/verify-m050-s01.sh`.
2. **Expected:** the command ends with `verify-m050-s01: ok` and reports `artifacts: .tmp/m050-s01/verify`.
3. Inspect:
   - `.tmp/m050-s01/verify/status.txt`
   - `.tmp/m050-s01/verify/current-phase.txt`
   - `.tmp/m050-s01/verify/phase-report.txt`
   - `.tmp/m050-s01/verify/latest-proof-bundle.txt`
4. **Expected:**
   - `status.txt` is `ok`
   - `current-phase.txt` is `complete`
   - `latest-proof-bundle.txt` points at `.tmp/m050-s01/verify`
   - `phase-report.txt` contains passed markers for `m050-s01-onboarding-graph`, `m047-s04-docs-contract`, `m047-s06-docs-contract`, `production-proof-surface`, `docs-build`, `retain-built-html`, `built-html`, and `m050-s01-bundle-shape`
5. Inspect `.tmp/m050-s01/verify/built-html/summary.json`.
6. **Expected built HTML footer truth:**
   - `getting_started.footer_links == ["/docs/getting-started/clustered-example/"]`
   - `clustered_example.footer_links == ["/docs/getting-started/", "/docs/language-basics/"]`
   - `distributed_proof.footer_links == []`
   - `production_backend_proof.footer_links == []`

### 4. M049 wrapper preflight wiring

1. Run `node --test scripts/tests/verify-m049-s05-contract.test.mjs`.
2. Run `cargo test -p meshc --test e2e_m049_s05 -- --nocapture`.
3. **Expected:** both pass.
4. **Expected behavior:** the M049 assembled wrapper now fail-closes if `bash scripts/verify-m050-s01.sh` disappears, moves later in the replay order, or stops being the first fast docs-graph preflight before the heavier scaffold/example rails.

## Edge Cases

### Prefix-matching regression

1. Do not patch source manually; the Node contract already exercises this case through a temp mutation.
2. **Expected:** if `usePrevNext.ts` ever falls back to `startsWith(...)` semantics again, `node --test scripts/tests/verify-m050-s01-onboarding-graph.test.mjs` must fail closed instead of allowing `Clustered Example` to self-link.

### Proof-page footer opt-out regression

1. Do not patch source manually; the Node contract already removes the markers in a temp copy.
2. **Expected:** removing `includeInFooter: false`, `prev: false`, or `next: false` from a proof page must fail the source contract, and a real built-site replay must also fail because the proof-page footer becomes visible again.

### Overlapping docs builds

1. Start `bash scripts/verify-m050-s01.sh`.
2. While it is still running, start another VitePress build or another docs verifier that writes `website/docs/.vitepress/dist` or `.tmp/m050-s01/verify`.
3. **Expected:** this is not a supported operating mode. If output drifts or phases fail unpredictably, kill the overlapping runs and rerun once serially.
4. **Why this matters:** this slice's proof depends on one clean built-site snapshot and one clean retained artifact tree.

## Failure Signals

- `status.txt` is not `ok` or `current-phase.txt` is not `complete` after a supposed green verifier run.
- `phase-report.txt` stops before `built-html` or `m050-s01-bundle-shape`.
- `summary.json` shows any proof-page footer links or shows `Clustered Example` linking to itself.
- The retained M047 or production-proof rails start requiring old proof-first copy again.
- The M049 wrapper contract no longer includes `bash scripts/verify-m050-s01.sh` as the first preflight.

## Requirements Advanced By This UAT

- **R117** — public docs are no longer structurally proof-first; proof pages stay public-secondary and the graph is mechanically verified.
- **R118** — clustered-app guidance now has one obvious first path (`Getting Started` -> `Clustered Example`) before lower-level proof surfaces.
- **R120** — public docs and README routing now share one coherent onboarding graph instead of competing first-contact proof links.

## Notes for Tester

- Debug from `.tmp/m050-s01/verify/phase-report.txt` and `.tmp/m050-s01/verify/built-html/summary.json` first; only then open the copied HTML snapshots or retained docs rails.
- If the failure is in the M049 wrapper, inspect its contract test before replaying the full assembled wrapper; S01 intentionally gives later slices one cheap docs-graph preflight to diagnose first.
