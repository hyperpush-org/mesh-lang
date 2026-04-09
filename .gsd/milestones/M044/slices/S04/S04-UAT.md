# S04: Bounded Automatic Promotion — UAT

**Milestone:** M044
**Written:** 2026-03-30T05:20:00.696Z

# S04: Bounded Automatic Promotion — UAT

**Milestone:** M044
**Written:** 2026-03-30

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: S04 is a runtime/compiler/proof-surface slice. The truthful proof is the destructive same-image failover rail plus retained artifacts and docs/build checks, not a browser walkthrough alone.

## Preconditions

- Run from the repo root.
- Rust toolchain and Cargo are available.
- Node/npm dependencies for `website/` are installed.
- No stale `cluster-proof` listeners are already occupying the ephemeral ports the `e2e_m044_s04` harness chooses.

## Smoke Test

Run the assembled acceptance rail:

```bash
bash scripts/verify-m044-s04.sh
```

**Expected:** the script exits 0, `.tmp/m044-s04/verify/status.txt` contains `ok`, and `.tmp/m044-s04/verify/latest-proof-bundle.txt` points at a retained `continuity-api-failover-promotion-rejoin-*` artifact directory.

## Test Cases

### 1. Safe automatic promotion and automatic recovery complete on the standby without a second submit

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture
   ```
2. Read the latest bundle path from:
   ```bash
   cat .tmp/m044-s04/verify/latest-proof-bundle.txt
   ```
3. Open `auto-recovery-pending-standby.json` from that bundle.
4. **Expected:** the JSON reports the promoted standby as `owner_node`, `cluster_role` is `primary`, `promotion_epoch` is `1`, `phase` is `submitted`, `result` is `pending`, and the new `attempt_id` differs from the original attempt.
5. Open `auto-recovery-completed-standby.json` from the same bundle.
6. **Expected:** the same new `attempt_id` is now `completed` / `succeeded` with `execution_node` equal to the standby and no client-side retry or second keyed submit required.

### 2. Stale-primary rejoin stays fenced after the promoted standby completes the recovered attempt

1. Reuse the latest S04 proof bundle.
2. Open `post-rejoin-primary-status.json` and `stale-guard-primary.json`.
3. **Expected:** the old primary reports `cluster_role` as `standby`, `promotion_epoch` as `1`, and the status payload reflects the standby-owned promoted attempt rather than a resumed local-primary attempt.
4. Inspect `primary-run2.stderr.log`.
5. **Expected:** it contains the `fenced_rejoin` continuity transition for the promoted attempt.
6. Inspect `primary-run2.stdout.log`.
7. **Expected:** it does **not** contain a `work executed` log for the promoted attempt on the old primary.

### 3. Manual authority mutation stays disabled at compile time

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s04 m044_s04_manual_surface_ -- --nocapture
   ```
2. **Expected:** both tests pass, confirming stale `Continuity.promote()` call sites fail closed with an explicit automatic-only diagnostic instead of compiling or linking.

### 4. The proof app and public docs match the auto-only failover contract

1. Run:
   ```bash
   cargo run -q -p meshc -- build cluster-proof
   cargo run -q -p meshc -- test cluster-proof/tests
   npm --prefix website run build
   ```
2. Run:
   ```bash
   rg -n '/promote|Continuity\.promote' README.md cluster-proof/README.md website/docs/docs/distributed/index.md website/docs/docs/distributed-proof/index.md
   ```
3. **Expected:** the build/test commands pass, the docs build passes, and the `rg` command returns no matches.

## Edge Cases

### Ambiguous promotion stays fail-closed instead of promoting anyway

1. Run:
   ```bash
   cargo test -p mesh-rt automatic_promotion_ -- --nocapture
   ```
2. **Expected:** the suite passes, including the rejection case for missing mirrored state, proving the runtime keeps ambiguity on the rejected path instead of inventing a promotion.

### Recovery rollover remains runtime-owned and fenced

1. Run:
   ```bash
   cargo test -p mesh-rt automatic_recovery_ -- --nocapture
   ```
2. **Expected:** the suite passes and the output includes `recovery_rollover` / `submit` transitions showing a new `attempt_id` owned by the surviving node.

## Failure Signals

- `bash scripts/verify-m044-s04.sh` exits non-zero or leaves `.tmp/m044-s04/verify/status.txt` as anything other than `ok`.
- Any named Cargo filter exits 0 while reporting `running 0 tests`.
- The retained proof bundle is missing `auto-recovery-pending-standby.json`, `auto-recovery-completed-standby.json`, `stale-guard-primary.json`, `standby-run1.stderr.log`, or `primary-run2.stderr.log`.
- Any doc file reintroduces stale manual authority wording.

## Requirements Proved By This UAT

- R067 — proves automatic promotion is auto-only, bounded, and fail-closed on ambiguity.
- R068 — proves declared clustered work survives primary loss through safe promotion, automatic recovery, and stale-primary fencing.

## Not Proven By This UAT

- Hosted destructive failover on Fly; the Fly surface remains read-only inspection only.
- Multi-standby quorum semantics, active-active writes, or any topology stronger than one primary plus one standby.
- The full S05 `cluster-proof` rewrite onto the generic clustered-app scaffold.

## Notes for Tester

If the destructive same-image rail fails, start with the bundle path recorded in `.tmp/m044-s04/verify/latest-proof-bundle.txt` and inspect `standby-run1.stderr.log`, `primary-run2.stderr.log`, and the JSON status snapshots before changing code. Those artifacts are the authoritative truth surface for S04.
