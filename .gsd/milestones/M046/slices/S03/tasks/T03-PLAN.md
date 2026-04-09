---
estimated_steps: 4
estimated_files: 5
skills_used:
  - rust-best-practices
  - rust-testing
---

# T03: Prove route-free failover/rejoin and close the slice with `verify-m046-s03.sh`

**Slice:** S03 — `tiny-cluster/` local no-HTTP proof
**Milestone:** M046

## Description

Extend the S03 e2e rail into the destructive failover/rejoin proof and close the slice with a direct verifier that replays consumed prerequisites, copies fresh `.tmp/m046-s03` bundles, and fails closed on missing tests or missing evidence.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `tiny-cluster/work.mpl` failover delay window | Fail with archived pre-kill snapshots if the startup record never becomes observable instead of inventing a route or second submit path. | Bound the wait for a mirrored pending window and archive the last continuity/status snapshot before failing. | Treat malformed delay input as zero/invalid and fail the pre-kill discovery rail honestly. |
| `compiler/meshc/tests/e2e_m045_s03.rs` helper port | Reuse only CLI/process helpers and fail with archived last observations instead of falling back to HTTP helpers. | Bound promotion/recovery/rejoin polling windows and record which CLI surface failed. | Reject malformed JSON and missing diagnostic transitions as proof failures. |
| `scripts/verify-m046-s03.sh` direct wrapper | Fail on timeout, zero tests, or missing fresh artifact bundles instead of reporting a false green wrapper. | Mark the current phase as failed and keep the last command log plus artifact hint. | Treat malformed phase reports, pointers, or bundle manifests as verifier failures. |

## Load Profile

- **Shared resources**: Two node processes, one forced owner kill, one rejoin, repeated CLI polling, and copied proof bundles under `.tmp/m046-s03/verify`.
- **Per-operation cost**: Startup discovery, destructive failover, promotion/recovery/rejoin observation, plus one direct verifier replay of the prerequisite commands.
- **10x breakpoint**: Pending-window timing drift and artifact bundle size will break first; raw CPU is not the bottleneck.

## Negative Tests

- **Malformed inputs**: Missing or invalid work-delay env values, missing runtime-name discovery, or a continuity record that never reaches mirrored pending state before kill.
- **Error paths**: Missing `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, or `fenced_rejoin` transitions must fail with retained evidence instead of retrying indefinitely.
- **Boundary conditions**: The stale primary must rejoin fenced as standby after promotion, and the package’s default no-delay behavior must remain fast/trivial outside the failover harness.

## Steps

1. Use the package-local delay env only inside the failover harness to keep the startup record pending long enough to discover through `meshc cluster continuity` list mode, then treat the returned record as the single source of truth.
2. Port only the CLI/process wait/assert helpers needed from `compiler/meshc/tests/e2e_m045_s03.rs` into `compiler/meshc/tests/e2e_m046_s03.rs` to kill the owner, prove standby promotion/recovery/completion, and assert fenced rejoin with no HTTP routes or app submit/status contract.
3. Retain scenario metadata, pre/post-kill CLI snapshots, and node logs under `.tmp/m046-s03/...` so promotion/recovery drift is diagnosable from one bundle.
4. Add `scripts/verify-m046-s03.sh` with direct prerequisite commands (`cargo build -q -p mesh-rt`, the focused S02 startup rail, `meshc build/test` on `tiny-cluster/`, and the S03 e2e rail), named-test-count assertions, artifact snapshot/copy logic, and bundle-shape checks without nested wrapper recursion.

## Must-Haves

- [ ] The failover/rejoin proof uses only runtime-owned CLI surfaces; no route or app-owned control seam is added.
- [ ] The default package behavior stays fast and trivial; the delay env is test-only and does not change the public workload story.
- [ ] Promotion, recovery, completion, and fenced rejoin are all asserted from retained `meshc cluster status|continuity|diagnostics` evidence.
- [ ] `scripts/verify-m046-s03.sh` replays direct prerequisite commands, proves named test filters ran, and retains a fresh copied failover bundle fail-closed.

## Verification

- `cargo test -p meshc --test e2e_m046_s03 m046_s03_tiny_cluster_failover_ -- --nocapture`
- `bash scripts/verify-m046-s03.sh`

## Observability Impact

- Signals added/changed: failover bundles with `scenario-meta.json`, pre/post-kill status/continuity/diagnostics snapshots, and verifier `phase-report.txt`, `status.txt`, `current-phase.txt`, plus `latest-proof-bundle.txt`.
- How a future agent inspects this: open `.tmp/m046-s03/verify/phase-report.txt`, follow `.tmp/m046-s03/verify/latest-proof-bundle.txt`, and inspect the copied node logs/JSON snapshots.
- Failure state exposed: the exact last phase, failing command log, and copied failover evidence directory remain on disk after verifier failure.

## Inputs

- `tiny-cluster/work.mpl` — package-local delay hook consumed only by the failover harness.
- `compiler/meshc/tests/e2e_m045_s03.rs` — CLI failover/rejoin wait/assert patterns to port without the HTTP helpers.
- `compiler/meshc/tests/e2e_m046_s03.rs` — startup/package rail that the destructive failover proof extends.
- `scripts/verify-m045-s03.sh` — direct-verifier artifact snapshot/copy/bundle-shape pattern to adapt for S03.
- `compiler/meshc/tests/e2e_m046_s02.rs` — focused consumed-contract startup rail that the new verifier must replay directly.

## Expected Output

- `compiler/meshc/tests/e2e_m046_s03.rs` — failover/rejoin rails plus retained `.tmp/m046-s03` evidence checks.
- `scripts/verify-m046-s03.sh` — direct slice verifier with prerequisite replay and artifact retention.
