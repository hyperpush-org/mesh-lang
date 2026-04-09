# S03: Public contract and guarded claims — UAT

**Milestone:** M054
**Written:** 2026-04-06T16:52:36.575Z

# S03: Public contract and guarded claims — UAT

**Milestone:** M054
**Written:** 2026-04-06

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: This slice only ships docs/metadata/asset/verifier surfaces. The truthful acceptance seam is the repo-owned contract tests plus the retained built-site/proof-bundle artifacts, not a separate live product interaction.

## Preconditions

- The repo has installed website dependencies so `npm --prefix website run generate:og` and `npm --prefix website run build` succeed.
- Cargo can run `compiler/meshc/tests/e2e_m054_s03.rs`.
- Docker is available **or** a disposable Postgres admin URL is available as `DATABASE_URL`, because the assembled S03 wrapper delegates `bash scripts/verify-m054-s02.sh`.
- Start from the repo root with the S03 files present.

## Smoke Test

Run:

```bash
node --test scripts/tests/verify-m054-s03-contract.test.mjs
```

**Expected:** 3 tests pass; the source contract confirms the bounded homepage/proof/OG wording is present and the stale generic load-balancing tagline is absent.

## Test Cases

### 1. Source contract catches the bounded public wording

1. Run `node --test scripts/tests/verify-m054-s03-contract.test.mjs`.
2. Confirm the test output shows 3 passing tests and 0 failures.
3. **Expected:** The current repo publishes the M054 S03 bounded homepage, proof-page, and OG contract; drift mutations in the test fixture fail closed.

### 2. Rendered docs and OG asset match the public contract

1. Run `npm --prefix website run generate:og`.
2. Run `npm --prefix website run build`.
3. Open `website/docs/.vitepress/dist/index.html` and confirm it contains `One public app URL fronts multiple Mesh nodes. Runtime placement stays server-side, and operator truth stays on meshc cluster.`
4. Confirm `website/docs/.vitepress/dist/index.html` does **not** contain `Built-in failover, load balancing, and exactly-once semantics`.
5. Open `website/docs/.vitepress/dist/docs/distributed-proof/index.html` and confirm it contains all of the following:
   - `A proxy/platform ingress may expose one public app URL in front of multiple nodes, but that is where the public routing story ends.`
   - `X-Mesh-Continuity-Request-Key`
   - `meshc cluster continuity <node-name@host:port> <request-key> --json` (HTML-escaped in the built file)
   - `If you are inspecting startup work or doing manual discovery without a request key yet`
   - `sticky sessions, frontend-aware routing, or client-visible topology claims`
6. Confirm `website/docs/public/og-image-v2.png` exists and is non-empty.
7. **Expected:** Built HTML and generated OG output match the bounded docs contract exactly; stale generic load-balancing copy is absent.

### 3. Assembled verifier repackages S02 evidence and publishes one retained bundle

1. Export a disposable Postgres admin URL as `DATABASE_URL`.
2. Run `bash scripts/verify-m054-s03.sh`.
3. Confirm `.tmp/m054-s03/verify/status.txt` contains `ok` and `.tmp/m054-s03/verify/current-phase.txt` contains `complete`.
4. Confirm `.tmp/m054-s03/verify/phase-report.txt` includes `passed` markers for:
   - `m054-s03-source-contract`
   - `m054-s03-rust-contract`
   - `m054-s03-s02-replay`
   - `m054-s03-generate-og`
   - `m054-s03-build-docs`
   - `m054-s03-built-html-assertions`
   - `m054-s03-retain-s02-verify`
   - `m054-s03-retain-source-and-logs`
   - `m054-s03-retain-site-evidence`
   - `m054-s03-retain-og-evidence`
   - `m054-s03-redaction-drift`
   - `m054-s03-bundle-shape`
5. Read `.tmp/m054-s03/verify/latest-proof-bundle.txt` and open the pointed retained bundle.
6. Confirm that retained bundle contains:
   - `retained-m054-s02-verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log,latest-proof-bundle.txt}`
   - `retained-site/index.html`
   - `retained-site/docs/distributed-proof/index.html`
   - `built-html-summary.json`
   - `retained-og-image-v2.png`
7. **Expected:** The wrapper replays S02 unchanged, republishes its own retained proof bundle, and keeps both built-site evidence and delegated proof lineage in one auditable place.

## Edge Cases

### Missing or invalid database preflight fails closed

1. Unset `DATABASE_URL` (or set it to a non-Postgres URL).
2. Run `bash scripts/verify-m054-s03.sh`.
3. **Expected:** The verifier fails during `m054-s03-db-env-preflight` instead of reporting green docs truth without the delegated S02 runtime proof.

### Stale public copy is rejected before the heavy wrapper runs

1. Temporarily replace the homepage description or OG subtitle with the old generic load-balancing claim.
2. Run `node --test scripts/tests/verify-m054-s03-contract.test.mjs`.
3. **Expected:** The source contract fails closed and names the missing/stale marker instead of allowing the drift to reach the built-site or retained-bundle rails.

## Failure Signals

- `node --test scripts/tests/verify-m054-s03-contract.test.mjs` reports missing bounded markers or stale generic copy.
- `npm --prefix website run build` succeeds but `website/docs/.vitepress/dist/index.html` or `.../docs/distributed-proof/index.html` is missing the bounded markers above.
- `.tmp/m054-s03/verify/status.txt` is not `ok`, `phase-report.txt` is missing a required `passed` phase, or `latest-proof-bundle.txt` does not point at a real retained bundle.
- The retained bundle is missing `retained-m054-s02-verify`, `built-html-summary.json`, retained site HTML, or `retained-og-image-v2.png`.
- The assembled verifier reports a redaction leak for `DATABASE_URL`.

## Requirements Proved By This UAT

- R123 — Public docs, metadata, and proof surfaces now explain the shipped one-public-URL/server-side runtime-placement model honestly, and repo-owned rails fail closed if the copy overclaims or the retained proof bundle drifts.

## Not Proven By This UAT

- This UAT does not prove new runtime routing behavior beyond the already-shipped S01/S02 ingress and request-correlation rails.
- This UAT does not prove live Fly deployment parity or any `mesher/landing` wording changes.

## Notes for Tester

Inspect `.tmp/m054-s03/verify/` first if the assembled rail fails. The phase report, built HTML summary, and retained proof-bundle pointer are the fastest trustworthy diagnostics, and the copied `retained-m054-s02-verify` tree preserves the delegated runtime-proof lineage without needing to re-derive it from source.
