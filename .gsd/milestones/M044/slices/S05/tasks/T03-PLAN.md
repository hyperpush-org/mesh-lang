---
estimated_steps: 24
estimated_files: 6
skills_used: []
---

# T03: Publish the scaffold-first closeout verifier and docs story

Close the milestone by making the docs and proof surfaces say the same thing the code now proves. This task adds a dedicated `scripts/verify-m044-s05.sh` final-assembly rail that replays the earlier clustered-app and failover proofs, then rewrites the public runbooks/pages so `meshc init --clustered` + `meshc cluster` are the primary clustered-app story and `cluster-proof` is the deeper dogfood/runbook proof consumer.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Assembled verifier in `scripts/verify-m044-s05.sh` | Stop at the first failing phase with retained phase/status/source-truth artifacts. | Mark the exact replay phase that stalled and keep the copied artifact paths. | Fail closed on missing `running N test`, malformed JSON/artifacts, or stale source/docs literals. |
| Prior behavior rails `scripts/verify-m044-s03.sh` and `scripts/verify-m044-s04.sh` | Surface the upstream failing rail directly instead of masking it behind docs-only success. | Preserve the failing prior phase and copied bundle path for debugging. | Reject malformed retained bundle state instead of passing the closeout rail on incomplete evidence. |
| Public docs/build surfaces | Fail the docs truth/build step rather than letting stale M043/S04-only wording or legacy route/env text ship. | N/A — local file/build path. | Reject missing markers, stale command blocks, or mixed clustered-app vs proof-app stories. |

## Load Profile

- **Shared resources**: replayed verifier artifact directories, docs build output, and the final S05 closeout artifact bundle.
- **Per-operation cost**: one S03 replay, one S04 replay, one S05 e2e/package replay, and one docs build/truth sweep.
- **10x breakpoint**: repeated verifier reruns churn artifact directories before code paths saturate; the closeout script must stay deterministic and fail fast.

## Negative Tests

- **Malformed inputs**: missing retained bundle files, missing `running N test` evidence, and stale docs/source literals such as `GET /work`, `CLUSTER_PROOF_COOKIE`, `remain later distributed slices`, or `M043 failover contract`.
- **Error paths**: S03 scaffold/operator drift, S04 failover drift, docs that still present `cluster-proof` as the first abstraction, or source absence checks that miss a reintroduced legacy helper.
- **Boundary conditions**: scaffold-first docs still route readers to the deeper `cluster-proof` runbook for the bounded failover details, and the Fly story remains clearly read-only.

## Steps

1. Add `scripts/verify-m044-s05.sh` as the authoritative final-assembly rail: replay `scripts/verify-m044-s03.sh` and `scripts/verify-m044-s04.sh`, run the named `e2e_m044_s05` filter plus `cluster-proof` build/tests, fail closed on zero-test runs, and retain `.tmp/m044-s05/verify/` status/source/docs-truth artifacts.
2. Rewrite `cluster-proof/README.md`, `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, `website/docs/docs/tooling/index.md`, and repo `README.md` so the primary clustered story starts with the scaffold/operator surfaces and treats `cluster-proof` as the deeper dogfood proof consumer.
3. Keep the public contract exact: no legacy `GET /work` probe or proof-specific bootstrap envs in docs, no stale “later slices” or “M043 failover contract” wording, and the Fly proof remains clearly read-only.
4. Rebuild the docs and run the full S05 verifier so the shipped runbooks, website pages, and retained closeout artifacts all agree on the final M044 story.

## Must-Haves

- [ ] `scripts/verify-m044-s05.sh` is the authoritative local closeout command and replays the S03/S04 product rails instead of checking docs in isolation.
- [ ] Public docs teach `meshc init --clustered` + `meshc cluster` as the primary clustered-app story and route failover depth to `cluster-proof` without treating it as the first abstraction.
- [ ] Source/docs truth checks fail on legacy route/env wording and on stale “later slice” / “M043 contract” language.

## Inputs

- `scripts/verify-m044-s03.sh`
- `scripts/verify-m044-s04.sh`
- `compiler/meshc/tests/e2e_m044_s05.rs`
- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`

## Expected Output

- `scripts/verify-m044-s05.sh`
- `README.md`
- `cluster-proof/README.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`

## Verification

cargo test -p meshc --test e2e_m044_s05 -- --nocapture
bash scripts/verify-m044-s05.sh
npm --prefix website run build

## Observability Impact

- Signals added/changed: `.tmp/m044-s05/verify/{status.txt,current-phase.txt,phase-report.txt,full-contract.log}` plus copied source-absence/docs-truth logs and the latest retained failover bundle path.
- How a future agent inspects this: start with `status.txt`, `current-phase.txt`, and `phase-report.txt`; if an upstream replay failed, jump straight to the copied S03/S04 artifact path named by the closeout rail.
- Failure state exposed: whether the break is in scaffold/operator truth, failover truth, package behavior, or docs/source drift.
