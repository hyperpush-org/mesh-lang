# S03: Same-Image Two-Cluster Operator Rail — UAT

**Milestone:** M043
**Written:** 2026-03-29T11:26:44.517Z

# S03: Same-Image Two-Cluster Operator Rail — UAT

**Milestone:** M043
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: the slice delivers a packaged operator verifier that runs a live destructive Docker failover and then proves the result from retained artifacts. Both the live command outcome and the copied JSON/log bundle need to be checked.

## Preconditions

- Docker Desktop / Docker Engine is running locally and can build images.
- The repo is checked out at the S03 closeout state and Cargo can build the workspace.
- No manual peer-list editing is required; the rail should derive node identity from the same image plus env.

## Smoke Test

1. Run `bash scripts/verify-m043-s03.sh` from the repo root.
2. Confirm the command ends with `verify-m043-s03: ok`.
3. Open `.tmp/m043-s03/verify/status.txt` and `.tmp/m043-s03/verify/current-phase.txt`.
4. **Expected:** `status.txt` contains `ok` and `current-phase.txt` contains `complete`.

## Test Cases

### 1. Packaged same-image failover rail replays cleanly

1. Run `bash scripts/verify-m043-s03.sh`.
2. Open `.tmp/m043-s03/verify/phase-report.txt`.
3. Verify the report contains these passed phases in order: `runtime-continuity`, `cluster-proof-tests`, `build-cluster-proof`, `s02-contract`, `same-image-contract`, `entrypoint-misconfig`, `same-image-artifacts`.
4. **Expected:** every phase is marked `passed`, proving the wrapper replayed the prerequisite continuity rails before validating the same-image contract.

### 2. Retained artifact bundle proves promotion and fenced rejoin

1. Open `.tmp/m043-s03/verify/05-same-image-bundle-values.txt` and note the request key, original attempt, failover attempt, primary node, and standby node.
2. Open `.tmp/m043-s03/verify/05-same-image-artifacts/<selected-bundle>/scenario-meta.json` and confirm it records the same request key plus `image_tag = mesh-cluster-proof:m043-s03-local`.
3. Open `.tmp/m043-s03/verify/05-same-image-artifacts/<selected-bundle>/promoted-membership-standby.json` and confirm `cluster_role = primary`, `promotion_epoch = 1`, and `self = standby@standby:4370`.
4. Open `.tmp/m043-s03/verify/05-same-image-artifacts/<selected-bundle>/membership-primary-run2.json` and confirm `self = primary@primary:4370` but `cluster_role = standby` with `promotion_epoch = 1`.
5. Open `.tmp/m043-s03/verify/05-same-image-artifacts/<selected-bundle>/post-rejoin-primary-status.json`.
6. **Expected:** the copied bundle proves the same key rolled from `attempt-0` to `attempt-1`, the standby became primary at epoch 1, and the old primary rejoined deposed instead of regaining authority.

### 3. Stale-primary same-key requests stay fenced after rejoin

1. Open `.tmp/m043-s03/verify/05-same-image-artifacts/<selected-bundle>/stale-guard-primary.json`.
2. Confirm `attempt_id` is the failover attempt (`attempt-1`), `cluster_role = standby`, `promotion_epoch = 1`, `owner_node = standby@standby:4370`, and `execution_node = standby@standby:4370`.
3. Open `.tmp/m043-s03/verify/full-contract.log` and confirm it contains `stale-primary same-key guard truth: keyed payload truth ok`.
4. **Expected:** the restarted old primary serves the promoted standby-owned result and does not execute or complete the request locally.

## Edge Cases

### Invalid continuity env fails before runtime startup

1. Open `.tmp/m043-s03/verify/04a-invalid-continuity.log`.
2. Confirm it contains `[cluster-proof] Config error: Invalid continuity topology: standby role requires promotion epoch 0 before promotion`.
3. Confirm the log does not contain the normal runtime startup line used by the verifier to detect a booted package.
4. **Expected:** contradictory same-image continuity env fails immediately at the entrypoint boundary and does not leak cookie material into the retained log.

## Failure Signals

- `bash scripts/verify-m043-s03.sh` exits non-zero or does not print `verify-m043-s03: ok`.
- `.tmp/m043-s03/verify/status.txt` is not `ok` or `current-phase.txt` is not `complete`.
- `phase-report.txt` is missing a phase or shows any phase other than `passed`.
- The copied same-image bundle is missing `scenario-meta.json`, membership/status JSON, or per-container logs.
- `promoted-membership-standby.json` does not show standby promoted to epoch 1, or `membership-primary-run2.json` does not show the old primary rejoined as `cluster_role = standby`.
- `stale-guard-primary.json` shows execution/ownership returning to the old primary.
- `04a-invalid-continuity.log` lacks the config error or shows a runtime-start line.

## Requirements Proved By This UAT

- R052 — proves the local same-image Docker operator rail: one `cluster-proof` image, small env-driven topology input, no manual peer list, retained failover artifacts, and fail-closed startup on contradictory continuity env.

## Not Proven By This UAT

- The public README/help/docs contract for the disaster-continuity path.
- Live Fly deployment state or destructive failover on Fly.

## Notes for Tester

- Replace `<selected-bundle>` with the directory named in `.tmp/m043-s03/verify/05-same-image.selection.txt` or the only copied bundle under `.tmp/m043-s03/verify/05-same-image-artifacts/` that contains `scenario-meta.json`.
- The promoted standby's post-rejoin `replication_health` may be `local_only` or `healthy`; treat ownership, epoch, and stale-primary fencing fields as the authoritative truth.
