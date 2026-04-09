---
estimated_steps: 3
estimated_files: 2
skills_used:
  - bash-scripting
  - test
---

# T03: Add a fail-closed S04 docs/reference contract verifier

Create one slice-owned verifier surface that builds the docs, runs the existing first-contact/proof-page checks, and asserts the new M053-specific wording across public docs plus retained Fly reference assets.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `npm --prefix website run build` | Fail the verifier immediately and keep the build log under `.tmp/m053-s04/verify/`; do not treat stale `dist/` output as evidence. | Stop the phase and mark the slice verifier failed instead of continuing with half-built docs. | Treat missing built HTML or missing summary files as drift and stop before later assertions run. |
| Existing contract rails (`scripts/verify-m050-s02.sh`, `scripts/verify-production-proof-surface.sh`, and `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`) | Stop on the first failing underlying rail and preserve its log path in the assembled verifier output. | Treat timeouts as contract drift; do not skip a slow rail and continue. | Fail closed if a wrapped rail stops producing the expected phase/status artifacts or command output shape. |
| New Node contract test (`scripts/tests/verify-m053-s04-contract.test.mjs`) | Fail closed on the first missing marker or stale string and report the offending file. | Stop the verifier if the test runner hangs; do not rely on partial TAP output. | Treat truncated/corrupted doc text or fixture copies as malformed input and fail with the exact surface name. |

## Load Profile

- **Shared resources**: VitePress build output under `website/docs/.vitepress/dist`, Node test runner state, Cargo test execution for the retained `cluster-proof` package, and `.tmp/m053-s04/verify/` artifact storage.
- **Per-operation cost**: one docs build, two existing shell verifiers, one retained fixture test run, and one new Node contract suite.
- **10x breakpoint**: docs build time and file-parse volume, not application throughput.

## Negative Tests

- **Malformed inputs**: missing M053 verifier names in `Distributed Proof`, retained Fly README still claiming equal canonical status, or corrupted built-doc output.
- **Error paths**: first-contact docs drift back toward proof-maze-first wording, retained Fly help text widens into a starter contract, or the assembled verifier stops before writing phase/status artifacts.
- **Boundary conditions**: duplicated trailing lines in docs, stale historical proof markers surviving beside M053 wording, and slice verifier output existing but missing the latest phase pointer.

## Steps

1. Implement `scripts/tests/verify-m053-s04-contract.test.mjs` as a fixture-backed contract suite that reads the targeted docs/reference files and rejects Fly-first, proof-maze-first, or SQLite/Postgres boundary drift.
2. Implement `scripts/verify-m053-s04.sh` so it writes `.tmp/m053-s04/verify/` phase/status artifacts, runs the docs build plus the existing proof rails, and then runs the new Node contract suite with failing-log hints.
3. Keep the verifier scoped to docs/reference surfaces only: it should not require live Fly credentials, mutate hosted infrastructure, or widen the generated starter contract.

## Must-Haves

- [ ] `scripts/tests/verify-m053-s04-contract.test.mjs` pins the M053 docs/reference wording across public docs and retained Fly assets.
- [ ] `scripts/verify-m053-s04.sh` assembles build + existing verifiers + the new test into one fail-closed surface under `.tmp/m053-s04/verify/`.
- [ ] Failure artifacts make it obvious whether drift came from first-contact docs, distributed-proof routing, retained Fly assets, or the docs build.

## Inputs

- `README.md` — first-contact repo-root copy updated by T01
- `website/docs/docs/getting-started/index.md` — primary evaluator-facing starter ladder updated by T01
- `website/docs/docs/getting-started/clustered-example/index.md` — clustered follow-on page updated by T01
- `website/docs/docs/tooling/index.md` — CLI/starter contract page updated by T01
- `website/docs/docs/distributed/index.md` — proof-page handoff updated by T02
- `website/docs/docs/distributed-proof/index.md` — M053 proof map updated by T02
- `scripts/fixtures/clustered/cluster-proof/README.md` — retained Fly reference asset updated by T02
- `scripts/verify-m043-s04-fly.sh` — Fly help surface updated by T02
- `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl` — retained fixture contract from T02
- `scripts/verify-m050-s02.sh` — existing first-contact assembled verifier
- `scripts/verify-production-proof-surface.sh` — existing proof-surface verifier

## Expected Output

- `scripts/tests/verify-m053-s04-contract.test.mjs` — fixture-backed M053 docs/reference contract suite
- `scripts/verify-m053-s04.sh` — assembled verifier with `.tmp/m053-s04/verify/` phase/status artifacts

## Verification

- `node --test scripts/tests/verify-m053-s04-contract.test.mjs`
- `bash scripts/verify-m053-s04.sh`

## Observability Impact

- Signals added/changed: `.tmp/m053-s04/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, and failing-log pointers
- How a future agent inspects this: run `bash scripts/verify-m053-s04.sh` and inspect the retained `.tmp/m053-s04/verify/` bundle
- Failure state exposed: docs build drift, first-contact wording drift, distributed-proof routing drift, retained Fly-asset drift, or missing verifier artifacts
