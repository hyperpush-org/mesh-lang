---
estimated_steps: 4
estimated_files: 5
skills_used:
  - bash-scripting
  - test
---

# T02: Align the starter surfaces and verifier with the bounded one-public-URL contract

**Slice:** S01 — One-public-URL starter ingress truth
**Milestone:** M054

## Description

Turn the new ingress proof into the serious starter's bounded public contract instead of leaving it as a hidden repo-only rail. The generated Postgres scaffold, committed `examples/todo-postgres` README, and assembled verifier must all say the same thing: one public app URL may front multiple nodes, Mesh runtime ownership is still inspected through `meshc cluster`, and this slice does not introduce frontend-aware node selection or a Fly-specific product promise.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Postgres scaffold template and committed example parity | Fail the task on the first template/example mismatch and keep the generated-vs-committed diff instead of hand-waving the drift. | Treat a stalled materialization or scaffold test as template drift; abort before copying any retained bundle. | Fail closed if README markers, generated file lists, or example parity manifests are missing or inconsistent. |
| assembled `scripts/verify-m054-s01.sh` replay | Stop on the first prerequisite failure and preserve the phase log rather than copying stale artifacts. | Kill the wrapper on timeout, record which phase stalled, and avoid reusing prior `latest-proof-bundle.txt` pointers. | Fail closed if the wrapper sees malformed JSON, empty bundle pointers, or incomplete copied manifests. |
| older starter-boundary rails reused for re-verification | Fail the task if `e2e_m047_s04` or scaffold tests disagree with the new wording, rather than weakening those existing boundaries. | Treat slow prerequisite rails as replay-cost issues; keep the wrapper ordered and bounded instead of parallelizing fragile checks. | Fail closed if reused rails start returning malformed logs or 0-test outputs. |

## Load Profile

- **Shared resources**: `compiler/mesh-pkg` scaffold template output, committed `examples/todo-postgres` files, `.tmp/m054-s01/verify/`, and the retained proof bundle copied from T01.
- **Per-operation cost**: one scaffold/template test run, one example-materializer parity check, one contract test, and one ordered shell-wrapper replay of the new cargo rail.
- **10x breakpoint**: cumulative replay time and artifact-copy churn before CPU, memory, or DB load.

## Negative Tests

- **Malformed inputs**: missing `DATABASE_URL`, stale or empty `latest-proof-bundle.txt`, malformed retained manifests, and missing README markers in generated or committed starter surfaces.
- **Error paths**: wrapper phase failure, cargo rail failure, materializer diff, scaffold template test failure, and secret leakage in copied proof artifacts.
- **Boundary conditions**: rerun with an existing `.tmp/m054-s01/verify/` tree, public URL wording updated in the committed example but not the scaffold template, and proof bundle copy succeeding only partially.

## Steps

1. Update `compiler/mesh-pkg/src/scaffold.rs` so the generated Postgres README and staged replay wording treat the public base URL as a proxy/platform ingress fronting multiple nodes while `meshc cluster` remains the inspection path; keep SQLite explicitly local and do not promise frontend-aware node selection or a Fly-specific product contract.
2. Materialize the committed `examples/todo-postgres/README.md` from the scaffold template and adjust `compiler/meshc/tests/e2e_m054_s01.rs` where needed so the one-public-URL proof bundle and starter README stay aligned.
3. Add `scripts/tests/verify-m054-s01-contract.test.mjs` plus `scripts/verify-m054-s01.sh`; make the wrapper replay the new cargo rail, `cargo test -p mesh-pkg m049_s01_postgres_scaffold_ -- --nocapture`, `node scripts/tests/verify-m049-s03-materialize-examples.mjs --check`, and the bounded starter-split rail from `e2e_m047_s04`, then copy the fresh retained bundle into `.tmp/m054-s01/verify/`.
4. Make the wrapper fail closed on stale bundle pointers, missing retained files, 0-test outputs, and secret leaks, and publish `status.txt`, `current-phase.txt`, `phase-report.txt`, and `latest-proof-bundle.txt` as the stable inspection surfaces.

## Must-Haves

- [ ] The generated Postgres scaffold and committed example describe the same bounded one-public-URL contract and keep SQLite explicitly local-only.
- [ ] `scripts/verify-m054-s01.sh` is the assembled retained proof surface for S01 and it republishes a fresh bundle instead of depending on ad-hoc local state.
- [ ] The wrapper and contract test fail closed on stale pointers, missing retained artifacts, README/scaffold drift, and secret leakage.

## Verification

- `node scripts/tests/verify-m054-s01-contract.test.mjs`
- `DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m054-s01.sh`

## Observability Impact

- Signals added/changed: `.tmp/m054-s01/verify/status.txt`, `current-phase.txt`, `phase-report.txt`, `latest-proof-bundle.txt`, and a retained bundle manifest for the one-public-URL proof.
- How a future agent inspects this: run `bash scripts/verify-m054-s01.sh`, then start from `.tmp/m054-s01/verify/phase-report.txt` and the republished `latest-proof-bundle.txt` pointer.
- Failure state exposed: stale bundle pointers, missing retained files, README/scaffold contract drift, and redaction failures in the copied proof bundle.

## Inputs

- `compiler/mesh-pkg/src/scaffold.rs` — generator source for the public Postgres starter README and staged replay scripts.
- `examples/todo-postgres/README.md` — committed serious-starter README that must stay in sync with the scaffold template.
- `compiler/meshc/tests/e2e_m054_s01.rs` — T01 rail whose retained bundle shape and README assertions the wrapper must replay.
- `compiler/meshc/tests/e2e_m047_s04.rs` — existing starter-boundary rail that guards the Postgres-vs-SQLite public split.
- `scripts/verify-m053-s01.sh` — prior staged-deploy retained wrapper whose phase/pointer pattern S01 should follow.
- `scripts/verify-m053-s02.sh` — prior staged-failover retained wrapper whose bundle-copy/redaction pattern S01 should reuse.
- `scripts/tests/verify-m049-s03-materialize-examples.mjs` — example-materializer parity command for committed `examples/` output.

## Expected Output

- `compiler/mesh-pkg/src/scaffold.rs` — generated Postgres starter wording updated to the bounded one-public-URL contract.
- `examples/todo-postgres/README.md` — committed example README materialized from the updated scaffold template.
- `scripts/verify-m054-s01.sh` — assembled retained verifier for the S01 one-public-URL proof rail.
- `scripts/tests/verify-m054-s01-contract.test.mjs` — cheap contract test covering README/wrapper wording and retained-bundle expectations.
- `compiler/meshc/tests/e2e_m054_s01.rs` — cargo rail updated where needed so the wrapper and starter contract stay aligned.
