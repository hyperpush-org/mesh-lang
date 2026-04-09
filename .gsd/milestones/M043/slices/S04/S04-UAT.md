# S04: Public Proof Surface and Operator Contract Truth — UAT

**Milestone:** M043
**Written:** 2026-03-29T12:23:18.468Z

# S04: Public Proof Surface and Operator Contract Truth — UAT

**Milestone:** M043
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice ships mechanical docs/verifier rails plus a canonical destructive local authority. The right acceptance bar is: the proof-surface verifier is green, the docs site still builds, the Fly helper stays read-only and fail-closed, and the same-image failover authority still proves promotion plus stale-primary fencing.

## Preconditions

- Run from the repo root: `/Users/sn0w/Documents/dev/mesh-lang`
- `bash`, `python3`, `rg`, `npm`, Cargo, and Docker are installed
- Docker daemon is running before replaying `bash scripts/verify-m043-s03.sh`
- No Fly credentials or deployed app are required for the help-path and negative-path checks below
- Optional live Fly inspection requires an existing deployed app plus:
  - `CLUSTER_PROOF_FLY_APP=<existing-app>`
  - `CLUSTER_PROOF_BASE_URL=https://<existing-app>.fly.dev`
  - optional `CLUSTER_PROOF_REQUEST_KEY=<existing-request-key>` if you want keyed status inspection

## Smoke Test

Run:

```bash
bash scripts/verify-m043-s04-proof-surface.sh
```

**Expected:** exit code 0, `.tmp/m043-s04/proof-surface/status.txt` contains `ok`, `.tmp/m043-s04/proof-surface/current-phase.txt` contains `complete`, and the verifier does not report stale M042 script names or wording.

## Test Cases

### 1. Public proof surfaces are mechanically aligned to the M043 failover contract

1. Run `bash scripts/verify-m043-s04-proof-surface.sh`.
2. Open `.tmp/m043-s04/proof-surface/phase-report.txt`.
3. Confirm the report contains passed phases for `proof-page-contract`, `guide-routing`, `readme-routing`, `runbook-contract`, `command-list`, and `stale-wording`.
4. **Expected:** the proof page, runbook, distributed guide, README, and sidebar all agree on the M043 script names, the explicit `/promote` boundary, the runtime-owned `cluster_role` / `promotion_epoch` / `replication_health` fields, same-image local authority, stale-primary fencing, and bounded Fly scope.

### 2. The Fly helper stays read-only and points back to the real local authority

1. Run `bash scripts/verify-m043-s04-fly.sh --help`.
2. Read the help text.
3. Confirm it names `bash scripts/verify-m043-s03.sh` as the destructive local authority.
4. Confirm the help text explicitly bans deploys, restarts/scale changes, secret writes, `POST /work`, and `POST /promote`.
5. **Expected:** the Fly helper presents itself as a read-only sanity/config/log/probe rail and does not imply it proves destructive failover on Fly.

### 3. The Fly helper fails closed and leaves localized artifacts on malformed input

1. Run `bash scripts/verify-m043-s04-fly.sh` with no environment variables.
2. Open `.tmp/m043-s04/fly/status.txt`, `.tmp/m043-s04/fly/current-phase.txt`, and `.tmp/m043-s04/fly/input-validation.log`.
3. Run `CLUSTER_PROOF_FLY_APP=mesh-cluster-proof CLUSTER_PROOF_BASE_URL=https://wrong.fly.dev bash scripts/verify-m043-s04-fly.sh`.
4. **Expected:** both runs exit non-zero at `input-validation`; the first reports missing `CLUSTER_PROOF_FLY_APP`, the second reports the host/app mismatch; the artifact bundle makes the failure phase obvious without any secret leakage.

### 4. The docs site still builds after the contract rewrite

1. Run `npm --prefix website run build`.
2. Watch for markdown/frontmatter/link/build errors.
3. **Expected:** the build completes successfully and the updated distributed proof pages render without breaking the VitePress site.

### 5. The public docs still point at a green destructive same-image authority

1. Run `bash scripts/verify-m043-s03.sh`.
2. Watch the output for the failover checkpoints.
3. Confirm it reaches `promoted standby membership truth`, `old primary fenced membership after rejoin`, `stale-primary same-key guard truth`, and a final `verify-m043-s03: ok`.
4. **Expected:** the same-image local rail still proves the actual contract the docs now advertise: explicit promotion, runtime-owned authority truth, stale-primary fencing, and fenced rejoin.

## Edge Cases

### Exact markdown command drift in the Fly live-mode block

1. If `bash scripts/verify-m043-s04-proof-surface.sh` fails at `command-list`, open `.tmp/m043-s04/proof-surface/command-list.content-check.log`.
2. Compare the proof page and runbook against the verifier’s expected Fly block.
3. **Expected:** the docs use the exact literal continued command form, including doubled trailing backslashes on the `CLUSTER_PROOF_FLY_APP=...` and `CLUSTER_PROOF_BASE_URL=...` lines.

### Optional live deployed Fly inspection stays read-only

1. Against an already-deployed app, run:
   ```bash
   CLUSTER_PROOF_FLY_APP=<app> \
   CLUSTER_PROOF_BASE_URL=https://<app>.fly.dev \
   bash scripts/verify-m043-s04-fly.sh
   ```
2. If you already know a valid request key, rerun with `CLUSTER_PROOF_REQUEST_KEY=<existing-request-key>`.
3. **Expected:** the helper only performs `fly status --json`, `fly config show`, `fly logs --no-tail`, `GET /membership`, and optional `GET /work/:request_key`; successful live runs archive summaries that include `cluster_role`, `promotion_epoch`, and `replication_health` instead of the older M042 payload shape.

## Failure Signals

- `bash scripts/verify-m043-s04-proof-surface.sh` exits non-zero or leaves `.tmp/m043-s04/proof-surface/status.txt=failed`
- `.tmp/m043-s04/proof-surface/current-phase.txt` stops at `proof-page-contract`, `command-list`, or `stale-wording`
- `bash scripts/verify-m043-s04-fly.sh --help` omits the read-only restrictions or fails to mention `bash scripts/verify-m043-s03.sh`
- `bash scripts/verify-m043-s04-fly.sh` accepts malformed input instead of failing at `input-validation`
- `npm --prefix website run build` reports markdown/frontmatter/build errors
- `bash scripts/verify-m043-s03.sh` fails to reach the promotion/fencing checkpoints or does not end with `verify-m043-s03: ok`

## Requirements Proved By This UAT

- R053 — the public distributed failover claims are now mechanically tied to the canonical M043 verifier pair, proof page, runbook, and local failover authority instead of drifting independently.

## Not Proven By This UAT

- Live Fly evidence for a specific deployed `cluster-proof` app; that requires real `CLUSTER_PROOF_FLY_APP` / `CLUSTER_PROOF_BASE_URL` values.
- Destructive failover on Fly. This slice intentionally keeps the Fly rail read-only and leaves destructive proof to `bash scripts/verify-m043-s03.sh`.
- Active-active writes, automatic promotion, or any topology broader than one primary plus one standby.

## Notes for Tester

Use the proof-surface verifier as the first stop for wording drift and the Fly helper only for help/read-only sanity or optional deployed-app inspection. If a live environment contradicts the docs, rerun the matching verifier first; if the contradiction is about actual failover behavior rather than public wording, fall back to `bash scripts/verify-m043-s03.sh` as the destructive authority.
