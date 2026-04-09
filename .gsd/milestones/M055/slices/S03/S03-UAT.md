# S03: `mesh-lang` Public Surface & Starter Contract Consolidation — UAT

**Milestone:** M055
**Written:** 2026-04-07T03:09:33.227Z

# S03: `mesh-lang` Public Surface & Starter Contract Consolidation — UAT

**Milestone:** M055
**Written:** 2026-04-07

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice ships docs/scaffold/workflow/verifier surfaces, but the acceptance story also depends on retained wrapper replays that prove those public claims still coexist with the historical compatibility chain. The honest seam is therefore source-contract tests plus the assembled shell wrappers and retained proof bundle.

## Preconditions

1. Start from the `mesh-lang` repo root.
2. `node`, `npm`, `cargo`, `python3`, `bash`, and `docker` are installed.
3. `docker info` succeeds before running the retained wrapper chain.
4. Website and packages dependencies are installed so `npm --prefix website run build` and `npm --prefix packages-website run build` can complete.
5. There is enough local disk for Cargo/Docker temporary artifacts, because the retained M047/M051 wrappers replay historical bundle-building rails.

## Smoke Test

1. Run `node --test scripts/tests/verify-m055-s03-contract.test.mjs`.
2. **Expected:** 4 tests pass. The source contract proves the language-only deploy/public workflow graph and the assembled S03 wrapper contract are present on disk.

## Test Cases

### 1. First-contact public ladder stays scaffold/examples-first

1. Run `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`.
2. Run `cargo test -p meshc --test e2e_m049_s03 -- --nocapture`.
3. Run `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`.
4. Run `node --test scripts/tests/verify-m050-s02-first-contact-contract.test.mjs`.
5. **Expected:** generated examples still match checked-in examples; README / Getting Started / Clustered Example / Tooling no longer teach local `mesher/...` paths or direct `verify-m051-*` commands as the public next step; the SQLite-local vs PostgreSQL-deployable split remains explicit.

### 2. Public-secondary proof pages hand off across the repo boundary

1. Run `node --test scripts/tests/verify-m050-s03-secondary-surfaces.test.mjs`.
2. Run `node --test scripts/tests/verify-m053-s04-contract.test.mjs`.
3. Run `bash scripts/verify-production-proof-surface.sh`.
4. Run `npm --prefix website run build`.
5. **Expected:** Distributed / Distributed Proof / Production Backend Proof use the Hyperpush repo handoff instead of mesh-lang-local product-source paths, keep the named retained proof map only on the proof page, and preserve the SQLite-local vs PostgreSQL-deployable boundary in rendered docs.

### 3. Generic guides, clustering skill, and retained docs wrappers stay aligned

1. Run `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`.
2. Run `bash scripts/verify-m047-s06.sh`.
3. Run `bash scripts/verify-m051-s04.sh`.
4. **Expected:** the clustering skill and generic guide callouts use the same Production Backend Proof -> Hyperpush handoff as the public docs, and the retained M047/M051 wrapper stack stays green without reintroducing the deleted `reference-backend` public story or direct mesh-lang-local product runbooks.

### 4. mesh-lang deploy/public workflow graph is language-only

1. Run `node --test scripts/tests/verify-m034-s05-contract.test.mjs`.
2. Run `bash scripts/verify-m034-s05-workflows.sh`.
3. Run `node --test scripts/tests/verify-m053-s03-contract.test.mjs`.
4. Run `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .`.
5. Run `npm --prefix packages-website run build`.
6. **Expected:** `.github/workflows/deploy-services.yml` contains only the mesh-lang-owned registry + packages/public-surface graph, landing deploy/health-check steps are explicitly forbidden, the shared public-surface helper remains the single health-check implementation, and `packages-website` still builds.

### 5. Assembled slice wrapper proves the full mesh-lang-only public/starter contract

1. Confirm `docker info` succeeds.
2. Run `bash scripts/verify-m055-s03.sh`.
3. Open `.tmp/m055-s03/verify/status.txt` and confirm it contains `ok`.
4. Open `.tmp/m055-s03/verify/current-phase.txt` and confirm it contains `complete`.
5. Open `.tmp/m055-s03/verify/phase-report.txt` and confirm it contains `passed` markers for:
   - `m055-s01-wrapper`
   - `m050-s02-wrapper`
   - `m050-s03-wrapper`
   - `m051-s04-wrapper`
   - `m034-s05-workflows`
   - `local-docs`
   - `packages-build`
   - `retain-m055-s01-verify`
   - `retain-m050-s02-verify`
   - `retain-m050-s03-verify`
   - `retain-m051-s04-verify`
   - `retain-m034-s05-workflows`
   - `m055-s03-bundle-shape`
6. Read `.tmp/m055-s03/verify/latest-proof-bundle.txt` and confirm the pointed directory exists.
7. **Expected:** the wrapper passes end to end, republishes retained sub-verifier state, and leaves one auditable retained bundle for the mesh-lang-only public/starter contract.

## Edge Cases

### Landing drift is rejected before the heavy wrapper runs

1. Temporarily reintroduce a landing deploy job or landing-specific health-check step into `.github/workflows/deploy-services.yml`.
2. Run `node --test scripts/tests/verify-m055-s03-contract.test.mjs` or `bash scripts/verify-m034-s05-workflows.sh`.
3. **Expected:** the verifier fails closed on the forbidden landing marker instead of passing because the required language-owned jobs are still present.

### Retained docs-wrapper drift is caught separately from public-docs drift

1. Keep public docs/source contracts green.
2. Temporarily stale out a retained wrapper expectation in `scripts/verify-m047-s06.sh` or `scripts/verify-m051-s04.sh`.
3. Run the corresponding wrapper.
4. **Expected:** the wrapper fails even though the direct public-docs tests still pass, which proves the retained compatibility chain is independently guarded.

### Repo-identity handoff drift is caught at the source level

1. Remove or rename the `productHandoff` mapping in `scripts/lib/repo-identity.json`, or break the scaffold placeholder/binding in `compiler/mesh-pkg/src/scaffold.rs`.
2. Run `node --test scripts/tests/verify-m049-s04-onboarding-contract.test.mjs`.
3. **Expected:** the contract fails closed on the missing repo-boundary handoff marker instead of letting public docs/scaffold output silently fall back to local product paths.

## Failure Signals

- Any source contract test reports stale local `mesher/...`, `reference-backend/README.md`, or direct `verify-m051-*` public handoff markers.
- `bash scripts/verify-m034-s05-workflows.sh` or `python3 scripts/lib/m034_public_surface_contract.py local-docs --root .` reports a forbidden landing job/check or a malformed language-only graph.
- `bash scripts/verify-m047-s06.sh`, `bash scripts/verify-m051-s04.sh`, or `bash scripts/verify-m055-s03.sh` fails and leaves a non-`ok` status/current-phase marker under its `.tmp/.../verify/` tree.
- `.tmp/m055-s03/verify/latest-proof-bundle.txt` points to a missing directory or the retained bundle is missing copied sub-verifier state.

## Requirements Proved By This UAT

- R008 — the public docs/examples/starter path stays production-oriented and no longer teaches local proof-app paths as the evaluator-facing follow-on step.

## Not Proven By This UAT

- Actual extraction of `hyperpush-mono` into a separate repo.
- Hosted remote freshness across the future two-repo split.
- Product-repo runtime behavior after extraction; that belongs to S04 and the product-side evidence chain.

## Notes for Tester

- Start with `.tmp/m055-s03/verify/phase-report.txt` if the assembled wrapper fails; it names the first broken phase cleanly.
- If the failure sits inside a retained wrapper, inspect that wrapper’s own `.tmp/.../verify/phase-report.txt` before changing docs or workflow source.
- The local source/build/public contract is only done when the assembled S03 wrapper is green; passing the individual Node tests alone is not sufficient.
