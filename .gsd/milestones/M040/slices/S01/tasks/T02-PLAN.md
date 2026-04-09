---
estimated_steps: 4
estimated_files: 4
skills_used:
  - test
  - debug-like-expert
---

# T02: Prove keyed idempotence and status visibility with a slice verifier

**Slice:** S01 — Keyed Submit/Status Contract on the Existing Proof Rail
**Milestone:** M040

## Description

Turn the keyed contract into a durable proof surface. This task should reuse the existing M039 `cluster-proof` harness patterns rather than inventing a second orchestration model, but it must exercise the new operator flow: submit keyed work, poll keyed status, and prove that a healthy cluster does not leak duplicate completion when the same key is retried.

The output should be a named Rust e2e file plus a repo-root verifier script that leaves behind enough artifacts for S02/S03 to compare healthy keyed behavior with later degraded/failover behavior.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `cluster-proof` build and package tests | stop immediately and preserve the failing build/test log instead of claiming the contract is proven | fail the verifier with the last captured phase/log output | reject missing binaries or red prerequisite tests as proof-surface drift |
| submit/status HTTP probes against spawned nodes | capture the raw request/response plus node logs, then fail closed | bound polling and persist the last status body instead of hanging forever | reject missing `request_key`, `attempt_id`, assignment, or phase/result fields instead of inferring success |
| artifact-copy and verification script checks | fail the slice verifier if evidence files are missing or empty | bound artifact collection and keep partial logs for diagnosis | reject malformed JSON or incomplete copied logs rather than treating the run as green |

## Load Profile

- **Shared resources**: two spawned `cluster-proof` processes, repeated submit/status polling, and `.tmp/m040-s01/verify/` artifact storage.
- **Per-operation cost**: one package build, one named Rust e2e run, repeated HTTP submit/status probes, and per-node log capture.
- **10x breakpoint**: repeated polling/artifact volume will grow before CPU; keep the verifier bounded and copy only the contract evidence needed for future slices.

## Negative Tests

- **Malformed inputs**: keyed submit with invalid JSON, status lookup for an unknown key, and retrying a known key with a conflicting payload.
- **Error paths**: node exits during the proof, status never reaches the expected phase, or duplicate same-key retry creates a second completion snapshot.
- **Boundary conditions**: healthy two-node cluster, truthful single-node/placeholder visibility, and same-key retry after the first completion is already visible.

## Steps

1. Add `compiler/meshc/tests/e2e_m040_s01.rs` by reusing the M039 process-spawn and HTTP helper patterns, but switch the exercised flow to keyed submit plus keyed status polling.
2. Assert the healthy-cluster contract: stable `request_key`, distinct `attempt_id`, truthful owner/replica visibility, and no duplicate completion leakage when the same key is retried with the same payload.
3. Assert the negative keyed paths: unknown-key status lookup and conflicting same-key resubmit must fail closed with contract-visible evidence.
4. Implement `scripts/verify-m040-s01.sh` to replay the named proof from repo root and archive submit/status JSON plus per-node stdout/stderr under `.tmp/m040-s01/verify/`.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m040_s01.rs` proves submit/status behavior against a real spawned `cluster-proof` runtime, not just helper functions.
- [ ] The proof asserts stable request-vs-attempt identity plus truthful owner/replica visibility on both healthy-cluster and single-node paths.
- [ ] `scripts/verify-m040-s01.sh` fail-closes on malformed JSON, missing artifacts, duplicate completion leakage, or conflicting same-key reuse.

## Verification

- `cargo test -p meshc --test e2e_m040_s01 -- --nocapture`
- `bash scripts/verify-m040-s01.sh`

## Observability Impact

- Signals added/changed: the verifier persists submit/status JSON, phase markers, and copied per-node stdout/stderr for keyed proof runs.
- How a future agent inspects this: rerun `bash scripts/verify-m040-s01.sh`, then inspect `.tmp/m040-s01/verify/` and the named Rust e2e failure output.
- Failure state exposed: malformed keyed responses, duplicate-completion leakage, conflicting same-key reuse, and per-node runtime failures are all captured as durable artifacts.

## Inputs

- `cluster-proof/main.mpl` — final keyed route wiring from T01.
- `cluster-proof/work.mpl` — keyed submit/status contract and observability fields from T01.
- `compiler/meshc/tests/e2e_m039_s02.rs` — existing cluster-proof harness patterns for remote/local proof.
- `compiler/meshc/tests/e2e_m039_s03.rs` — existing cluster lifecycle/log capture patterns for artifact copying.
- `scripts/verify-m039-s02.sh` — prior repo-root verifier structure to mirror for phase reports and status artifacts.

## Expected Output

- `compiler/meshc/tests/e2e_m040_s01.rs` — named Rust e2e contract proof for keyed submit/status behavior.
- `scripts/verify-m040-s01.sh` — repo-root slice verifier with durable artifact capture.
- `cluster-proof/main.mpl` — any small route/handler adjustments required by the e2e proof.
- `cluster-proof/work.mpl` — any contract/observability adjustments required by the e2e proof.
