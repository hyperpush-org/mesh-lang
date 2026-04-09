# S03: Single-Cluster Failure, Safe Degrade, and Rejoin — UAT

**Milestone:** M039
**Written:** 2026-03-28T12:16:44.430Z

# S03: Single-Cluster Failure, Safe Degrade, and Rejoin — UAT

**Milestone:** M039
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S03 shipped both live runtime behavior and a canonical replay wrapper. The honest acceptance path is to run the wrapper, then inspect the copied proof bundle it produced for the exact pre-loss, degraded, and post-rejoin signals.

## Preconditions

- Run from the repo root with the Rust workspace buildable.
- No manual cleanup of `.tmp/m039-s03/verify/` is required; the wrapper must handle reruns safely.
- Do not run concurrent `cluster-proof` build/test commands during this UAT; let the wrapper own the proof run.

## Smoke Test

1. Run `bash scripts/verify-m039-s03.sh`.
2. Open `.tmp/m039-s03/verify/status.txt`, `.tmp/m039-s03/verify/current-phase.txt`, and `.tmp/m039-s03/verify/phase-report.txt`.
3. **Expected:** the script exits 0, prints `verify-m039-s03: ok`, `status.txt` is `ok`, `current-phase.txt` is `complete`, and `phase-report.txt` shows `cluster-proof-tests`, `build-cluster-proof`, `s01-contract`, `s02-contract`, `s03-degrade`, and `s03-rejoin` all marked `passed`.

## Test Cases

### 1. Degraded cluster truthfully shrinks membership and serves locally after peer loss

1. After the smoke test, open `.tmp/m039-s03/verify/05-s03-degrade-artifacts.txt` and note the copied proof directory name.
2. Open the copied `pre-loss-node-a-membership.json` and `degraded-node-a-membership.json` from that directory.
3. Open the copied `pre-loss-work.json` and `degraded-work.json` from that directory.
4. **Expected:** `pre-loss-node-a-membership.json` lists both `node-a` and `node-b`, `degraded-node-a-membership.json` lists only `node-a` with an empty `peers` array, `pre-loss-work.json` reports `request_id: "work-0"` with `routed_remotely: true` and execution on `node-b`, and `degraded-work.json` reports `request_id: "work-1"` with `fell_back_locally: true` and execution on `node-a`.

### 2. Same-identity rejoin restores truthful membership and remote routing

1. Open `.tmp/m039-s03/verify/06-s03-rejoin-artifacts.txt` and note the copied proof directory name.
2. Open `post-rejoin-node-a-membership.json`, `post-rejoin-node-b-membership.json`, and `post-rejoin-work.json` from that copied directory.
3. Open `node-a-run1.stdout.log` and `node-b-run2.stdout.log` from the same directory.
4. **Expected:** both post-rejoin membership JSON files list `node-a` and `node-b` again, `post-rejoin-work.json` reports `request_id: "work-2"` with `routed_remotely: true` and execution on `node-b`, `node-a-run1.stdout.log` contains `work dispatched request_id=work-2 ... routed_remotely=true`, and `node-b-run2.stdout.log` contains `work executed execution=node-b...` for the restarted peer.

### 3. Restart evidence is preserved instead of overwritten

1. In the copied rejoin artifact directory, check that `node-b-run1.stdout.log`, `node-b-run1.stderr.log`, `node-b-run2.stdout.log`, and `node-b-run2.stderr.log` all exist.
2. Confirm the files are non-empty.
3. **Expected:** both run-1 and run-2 log pairs are present, showing the first crashed incarnation and the restarted incarnation separately. A rejoin proof that only leaves one `node-b` log file is a failure.

## Edge Cases

### Wrapper rerun against an existing verify directory

1. Run `bash scripts/verify-m039-s03.sh` a second time without deleting `.tmp/m039-s03/verify/`.
2. Re-open `status.txt`, `current-phase.txt`, and the two artifact-manifest files.
3. **Expected:** the second run also exits 0, leaves `status.txt=ok` and `current-phase.txt=complete`, and rewrites the copied degrade/rejoin manifests to point at the new proof directories rather than silently reusing stale evidence.

### Phase-specific request ids stay distinct across one cluster lifetime

1. In the copied degrade and rejoin artifact directories, compare `pre-loss-work.json`, `degraded-work.json`, and `post-rejoin-work.json`.
2. **Expected:** the request ids are `work-0`, `work-1`, and `work-2` respectively. Reuse of `work-0` after rejoin is a regression in the ingress-owned correlation contract.

## Failure Signals

- `bash scripts/verify-m039-s03.sh` exits non-zero or does not print `verify-m039-s03: ok`.
- `.tmp/m039-s03/verify/status.txt` is not `ok`, `.tmp/m039-s03/verify/current-phase.txt` is not `complete`, or `phase-report.txt` is missing a `passed` line for any phase.
- The copied degrade/rejoin manifest is missing expected JSON or run-numbered log files.
- `degraded-node-a-membership.json` still lists `node-b`, or either post-rejoin membership JSON fails to list both nodes.
- `degraded-work.json` does not fall back locally, or `post-rejoin-work.json` does not return to remote routing.
- `node-b-run2.stdout.log` is missing or empty after the rejoin proof.

## Requirements Proved By This UAT

- R048 — proves single-cluster safe degrade, continued service for new work, and same-identity clean rejoin without manual repair.
- R046 — advances the local proof that membership is truthful on loss and rejoin.
- R047 — advances the proof that runtime-native internal routing recovers after rejoin instead of only working on the happy path.

## Not Proven By This UAT

- One-image operator deployment or Fly-backed continuity replay; that belongs to S04.
- Cross-cluster disaster recovery, keyed at-least-once continuity, or any durability guarantee beyond this single two-node cluster proof.
- Rich distributed request metadata transport beyond the current scalar-safe contract; S03 intentionally avoids proving cross-node string/struct argument delivery through restart.

## Notes for Tester

The authoritative evidence is the copied `.tmp/m039-s03/verify/` bundle, not the transient temp directories created before the wrapper copies them. If the wrapper fails, inspect `full-contract.log`, `phase-report.txt`, and the phase-specific copied manifests before rerunning. The proof is intentionally local and narrow: it demonstrates truthful cluster continuity on `cluster-proof`, not the later operator or disaster-recovery story.
