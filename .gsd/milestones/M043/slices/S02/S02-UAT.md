# S02: Standby Promotion and Stale-Primary Fencing — UAT

**Milestone:** M043
**Written:** 2026-03-29T09:48:05.428Z

# S02: Standby Promotion and Stale-Primary Fencing — UAT

**Milestone:** M043
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: the slice contract is a destructive runtime failover sequence, so the honest acceptance path is the retained artifact bundle plus the real local runtime replay that generated it.

## Preconditions

- Build artifacts are available from the repo root.
- No stale `cluster-proof` processes are holding the ephemeral ports chosen by the verifier.
- Run from the repo root so `.tmp/m043-s02/verify/` is created in the expected location.

## Smoke Test

1. Run `bash scripts/verify-m043-s02.sh`.
2. Confirm it exits 0 and writes `.tmp/m043-s02/verify/status.txt` with `ok`.
3. **Expected:** `.tmp/m043-s02/verify/phase-report.txt` contains `runtime-continuity`, `cluster-proof-tests`, `build-cluster-proof`, `s01-contract`, `m042-rejoin`, `m043-api`, `failover-contract`, and `failover-artifacts` all marked `passed`.

## Test Cases

### 1. Primary loss promotes the standby without losing request truth

1. Run `bash scripts/verify-m043-s02.sh`.
2. Open `.tmp/m043-s02/verify/07-failover-artifacts/` and locate the copied `continuity-api-failover-promotion-rejoin-*` directory.
3. Inspect `scenario-meta.json`, `pending-primary.json`, `pending-standby.json`, `degraded-membership-standby.json`, `promote-standby.json`, and `promoted-owner-lost-status.json`.
4. **Expected:** the same `request_key` and original `attempt_id` exist on both pre-failover status payloads, standby becomes self-only and `degraded` after primary loss, `/promote` returns `cluster_role=primary`, `promotion_epoch=1`, `replication_health=unavailable`, and the promoted standby reports the old attempt as `replica_status=owner_lost` before retry.

### 2. Same-key retry rolls forward on the promoted standby and completes there

1. In the copied failover artifact directory, inspect `failover-retry.json`, `failover-pending-status.json`, and `failover-completed-standby.json`.
2. Compare the retry `attempt_id` in `failover-retry.json` to the original `attempt_id` from `scenario-meta.json`.
3. Inspect `standby-run1.stderr.log` for `transition=recovery_rollover` and `standby-run1.stdout.log` for `work executed request_key=... attempt_id=... execution=standby@[::1]:...`.
4. **Expected:** the promoted standby issues a new attempt id, owns the retried work locally with `replica_status=unassigned`, and the completion log shows execution on the standby node, not on the failed primary.

### 3. Restarted old primary rejoins fenced and cannot reclaim authority

1. In the copied failover artifact directory, inspect `membership-primary-run2.json`, `post-rejoin-primary-status.json`, `post-rejoin-standby-status.json`, and `stale-guard-primary.json`.
2. Inspect `primary-run2.stderr.log` for `transition=fenced_rejoin` and `primary-run2.stdout.log` for the final keyed status line.
3. Confirm `primary-run2.stdout.log` does not contain a `work executed ... execution=primary@127.0.0.1:...` line for the promoted attempt.
4. **Expected:** the restarted old primary reports `cluster_role=standby`, `promotion_epoch=1`, and the newer standby-owned attempt as the authoritative completed record; a same-key submit against the old primary replays the promoted standby result instead of starting a new execution.

## Edge Cases

### Pre-submit membership truth stays local-only until mirrored continuity exists

1. Inspect the copied `membership-primary-run1.json` and `membership-standby-run1.json` files from the failover bundle.
2. Compare them to `submit-primary.json`.
3. **Expected:** both membership payloads report `replication_health=local_only` before any keyed work is mirrored; health becomes `healthy` on the submit/status payloads only after runtime continuity is actually mirrored.

### Older single-cluster rejoin regression still replays after explicit role/epoch config became mandatory

1. Inspect `.tmp/m043-s02/verify/04-m042-rejoin.log`.
2. Confirm it contains `running 1 test` and `test continuity_api_same_identity_rejoin_preserves_newer_attempt_truth ... ok`.
3. **Expected:** the older M042 same-identity rejoin rail still passes inside the assembled S02 verifier, proving the S02 work did not silently regress single-cluster attempt fencing while adding the primary/standby failover path.

## Failure Signals

- `scripts/verify-m043-s02.sh` exits non-zero or `.tmp/m043-s02/verify/status.txt` is not `ok`.
- `promote-standby.json` does not report `promotion_epoch: 1` / `cluster_role: primary`.
- `failover-retry.json` reuses the old `attempt_id` instead of rolling a new one.
- `primary-run2.stderr.log` lacks `transition=fenced_rejoin` or `primary-run2.stdout.log` shows execution on the old primary after rejoin.
- The copied artifact directory is missing the scenario metadata or the expected JSON/log bundle.

## Requirements Proved By This UAT

- R051 — full loss of the active primary cluster remains survivable through live replication to a standby cluster, explicit promotion, and fenced stale-primary rejoin.

## Not Proven By This UAT

- The same-image two-cluster packaged operator rail from S03.
- Public docs/help/Fly proof-surface reconciliation from S04.

## Notes for Tester

Use the copied bundle under `.tmp/m043-s02/verify/07-failover-artifacts/` as the source of truth. It preserves the exact JSON payloads and node logs from the passing destructive replay, which is more reliable for review than trying to infer the sequence from console output alone.
