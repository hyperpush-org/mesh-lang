# S03: Tiny Example Failover Truth — UAT

**Milestone:** M045
**Written:** 2026-03-30T23:14:32.437Z

# S03: Tiny Example Failover Truth — UAT

**Milestone:** M045
**Written:** 2026-03-28

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice is a runtime-behavior change on a local two-node clustered example, so the truthful acceptance surface is the assembled verifier plus the retained runtime-owned artifacts it copies out of `.tmp/m045-s03/`.

## Preconditions

- Run from the repo root.
- No parallel runs of `cargo test -p meshc --test e2e_m045_s02 ...`, `cargo test -p meshc --test e2e_m045_s03 ...`, or `bash scripts/verify-m045-s03.sh`.
- Local build toolchain is available (`cargo`, Rust test dependencies, and the repo’s normal Mesh build prerequisites).
- `.tmp/m045-s03/` can be rewritten by the verifier.

## Smoke Test

Run:

```bash
bash scripts/verify-m045-s03.sh
```

**Expected:** the command exits 0, prints `verify-m045-s03: ok`, writes `.tmp/m045-s03/verify/status.txt` with `ok`, and records a retained bundle path in `.tmp/m045-s03/verify/latest-proof-bundle.txt`.

## Test Cases

### 1. Clustered scaffold contract stays tiny

1. Run:
   ```bash
   cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
   ```
2. Inspect the generated clustered scaffold artifacts if needed.
3. **Expected:** the test passes and the scaffold still exposes `Node.start_from_env()`, declared work submission, and `meshc cluster status|continuity|diagnostics` documentation without `CLUSTER_PROOF_*`, manual promote routes, or app-owned status handlers.

### 2. Single-node clustered scaffold still completes locally

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture
   ```
2. If the rail fails, inspect the retained `.tmp/m045-s02/scaffold-runtime-completion-*` bundle.
3. **Expected:** the tests pass; the scaffold completes continuity locally with `replica_node == ""`, `execution_node == local node`, and no app-owned `Continuity.mark_completed(...)` glue.

### 3. Two-node scaffold failover survives primary loss on the same example

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m045_s03 m045_s03_failover_ -- --nocapture
   ```
2. Open the newest `.tmp/m045-s03/scaffold-failover-runtime-truth-*` directory.
3. Check `pre-kill-continuity-standby.json`.
4. **Expected:** the retained pre-kill record shows the selected request in `phase="submitted"`, `result="pending"`, `owner_node=primary`, `replica_node=standby`, `replica_status="preparing"` or `"mirrored"`, `cluster_role="standby"`, and `execution_node=""`.

### 4. Promotion, recovery, and rejoin truth stay runtime-owned

1. In the same retained bundle, inspect:
   - `post-kill-status-standby.json`
   - `post-kill-diagnostics-standby.json`
   - `post-kill-continuity-standby-completed.json`
   - `post-rejoin-status-primary.json`
   - `post-rejoin-status-standby.json`
   - `post-rejoin-continuity-primary.json`
   - `post-rejoin-continuity-standby.json`
   - `post-rejoin-stale-submit-primary.json`
2. Check the node logs:
   - `primary-run1.stderr.log`
   - `standby-run1.stderr.log`
   - `primary-run2.stderr.log`
3. **Expected:**
   - standby promotes to `cluster_role="primary"` with `promotion_epoch=1` after primary loss,
   - standby diagnostics contain `automatic_promotion`, `recovery_rollover`, and `automatic_recovery`,
   - the recovered attempt completes on the standby,
   - the rejoined primary reports `cluster_role="standby"` and the selected request’s continuity now points at the standby-owned recovered attempt,
   - the stale-primary duplicate submit returns `outcome="duplicate"` with the recovered attempt ID,
   - no retained log shows the recovered attempt completing on the stale primary.

## Edge Cases

### Fresh-bundle selection is fail-closed

1. Run `bash scripts/verify-m045-s03.sh` twice.
2. Inspect `.tmp/m045-s03/verify/03-m045-s03-artifacts.txt` and `.tmp/m045-s03/verify/latest-proof-bundle.txt` from the second run.
3. **Expected:** the verifier copies only fresh artifact directories from the second replay, points `latest-proof-bundle.txt` at `.tmp/m045-s03/verify/retained-m045-s03-artifacts`, and fails if the copied bundle is missing `scenario-meta.json`, the required JSON proof files, or node stdout/stderr logs.

### Rejoin diagnostics are generic, selected-request truth is continuity-based

1. Inspect `post-rejoin-diagnostics-primary.json` alongside `post-rejoin-continuity-primary.json`.
2. **Expected:** a `fenced_rejoin` entry exists on the rejoined primary, but the exact diagnostics request key may differ from the selected recovered request; the authoritative selected-request truth remains the post-rejoin continuity JSON plus the stale-primary duplicate-submit response.

## Failure Signals

- `bash scripts/verify-m045-s03.sh` exits non-zero or prints `verification drift:`.
- `.tmp/m045-s03/verify/status.txt` is `failed` or `current-phase.txt` stops before `complete`.
- The retained bundle is missing `scenario-meta.json`, `pre-kill-continuity-standby.json`, `post-kill-diagnostics-standby.json`, post-rejoin continuity/status JSON, or node stdout/stderr logs.
- Standby diagnostics show `automatic_promotion_rejected:no_mirrored_state`, `replica_prepare_timeout`, or the recovered attempt completes on the stale primary.

## Requirements Proved By This UAT

- R078 — proves one tiny local clustered example can form a cluster, run declared work, survive primary loss, and retain truthful runtime-owned failover evidence end to end on the same scaffold-first surface.

## Not Proven By This UAT

- R081 — this UAT does not prove the public docs now teach the scaffold-first example first; that remains S05.
- Full legacy-example cleanup outside the scaffold-first surface; S04 still owns collapsing the remaining `cluster-proof`-style residue.

## Notes for Tester

- If the rail fails, start with `.tmp/m045-s03/verify/current-phase.txt`, then inspect the copied retained bundle under `.tmp/m045-s03/verify/retained-m045-s03-artifacts/` before rerunning anything.
- Do not run the S02 and S03 clustered rails in parallel; they contend on the same proof surfaces and can create false red failures.
- The generated scaffold intentionally keeps a brief fixed work delay so the local failover window is observable; that is part of the current proof surface, not a leftover manual operator control.
