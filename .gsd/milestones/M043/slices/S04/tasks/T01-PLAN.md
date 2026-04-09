---
estimated_steps: 4
estimated_files: 7
skills_used:
  - flyio-cli-public
---

# T01: Add the M043 proof-surface and read-only Fly verifier rails

**Slice:** S04 — Public Proof Surface and Operator Contract Truth
**Milestone:** M043

## Description

Create the fail-closed M043 verifier pair before touching public prose so the slice has an executable contract instead of ad hoc markdown edits.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `fly` CLI / read-only app inspection | Fail closed with the captured phase log and the last saved stdout/stderr artifact. | Mark the phase as failed, preserve partial artifacts, and report which read-only probe never converged. | Reject mismatched hosts, malformed JSON, or missing `cluster_role` / `promotion_epoch` / `replication_health` fields before claiming live truth. |
| M042 verifier templates | Stop and report the stale contract point instead of silently inheriting M042 wording into the new M043 rail. | N/A | Reject copied assertions that still look for M042-only command names or payload shapes. |

## Load Profile

- **Shared resources**: Local artifact directory, optional Fly API/logs, and live `cluster-proof` HTTP responses.
- **Per-operation cost**: One proof-surface string sweep plus read-only `fly status`, `fly config show`, `fly logs --no-tail`, `GET /membership`, and optional `GET /work/:request_key`.
- **10x breakpoint**: Fly API latency and log volume fail before the verifier logic changes.

## Negative Tests

- **Malformed inputs**: Missing `CLUSTER_PROOF_FLY_APP`, host/app mismatches in `CLUSTER_PROOF_BASE_URL`, and malformed optional request keys must fail in input validation.
- **Error paths**: Missing M043 script names, stale M042 wording, or live JSON without authority fields must stop the verifier with an artifact hint.
- **Boundary conditions**: Live Fly mode stays read-only even when `CLUSTER_PROOF_REQUEST_KEY` is set; the verifier must not submit work, restart machines, or mutate config.

## Steps

1. Clone the structure of `scripts/verify-m042-s04-proof-surface.sh` and `scripts/verify-m042-s04-fly.sh` into named M043 verifier scripts.
2. Upgrade the proof-surface contract to require M043 commands, explicit `/promote` authority wording, same-image local authority, stale-primary fencing, supported topology, and non-goals.
3. Upgrade the Fly verifier to keep the read-only command set but require `cluster_role`, `promotion_epoch`, and `replication_health` on `/membership` and optional keyed-status JSON.
4. Keep both scripts writing retained phase/state/full-log artifacts under `.tmp/m043-s04/` and failing closed on stale help text, config drift, or malformed live payloads.

## Must-Haves

- [ ] `scripts/verify-m043-s04-proof-surface.sh` exists and bans stale M042 command/wiring text from the public surfaces.
- [ ] `scripts/verify-m043-s04-fly.sh` help/live text names `bash scripts/verify-m043-s03.sh` as the destructive local authority and keeps Fly read-only.
- [ ] Live JSON checks require authority fields instead of the older M042 membership/status shape.
- [ ] Retained verifier artifacts point to the failing phase/log without leaking secrets.

## Verification

- `bash scripts/verify-m043-s04-fly.sh --help`
- `bash -n scripts/verify-m043-s04-proof-surface.sh scripts/verify-m043-s04-fly.sh`

## Observability Impact

- Signals added/changed: `.tmp/m043-s04/proof-surface/` and `.tmp/m043-s04/fly/` phase/status/full-log artifacts for public-contract drift and live authority-shape failures.
- How a future agent inspects this: rerun the named verifier, then inspect the retained phase/status/current-phase files and captured help/log/HTTP artifacts under `.tmp/m043-s04/`.
- Failure state exposed: stale command/help text, config drift, malformed live JSON, and the exact verifier phase that failed.

## Inputs

- `scripts/verify-m042-s04-proof-surface.sh` — M042 public-contract gate to fork into the M043 script.
- `scripts/verify-m042-s04-fly.sh` — M042 read-only Fly verifier to upgrade to M043 authority fields.
- `scripts/verify-m043-s03.sh` — current destructive local authority the new scripts must point at.
- `scripts/lib/m043_cluster_proof.sh` — shared M043 JSON assertions to reuse where they fit.
- `cluster-proof/main.mpl` — confirms `/promote` is mounted and authority truth is public.
- `cluster-proof/work_continuity.mpl` — confirms the keyed and promotion payload fields the verifier must require.
- `cluster-proof/fly.toml` — confirms the supported read-only Fly operator seam.

## Expected Output

- `scripts/verify-m043-s04-proof-surface.sh` — M043 documentation-truth verifier.
- `scripts/verify-m043-s04-fly.sh` — M043 read-only Fly verifier.
