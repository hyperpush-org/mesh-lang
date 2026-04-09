# S04: Remove Legacy Example-Side Cluster Logic — UAT

**Milestone:** M045
**Written:** 2026-03-31T01:14:31.343Z

# S04: Remove Legacy Example-Side Cluster Logic — UAT

**Milestone:** M045
**Written:** 2026-03-31

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: this slice is half source-contract cleanup and half assembled runtime/docs verification, so the truthful acceptance surface is a mix of package tests, destructive runtime rails, and the assembled verifier that retains upstream proof artifacts.

## Preconditions

- Run from the repo root.
- Do not run `cargo test -p meshc --test e2e_m045_s02 ...`, `bash scripts/verify-m045-s02.sh`, or `bash scripts/verify-m045-s04.sh` in parallel with each other.
- Local build toolchain is available (`cargo`, Rust test dependencies, `npm`, and the repo’s normal Mesh build prerequisites).
- `.tmp/m045-s02/`, `.tmp/m045-s03/`, and `.tmp/m045-s04/` can be rewritten by the verifier scripts.

## Smoke Test

Run:

```bash
cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture
```

**Expected:** the command exits 0 and all three S04 contract tests pass: the cleaned `cluster-proof` target shape is real, public docs point at the M045 closeout rail, and `scripts/verify-m045-s04.sh` is the current assembled verifier.

## Test Cases

### 1. `cluster-proof` no longer teaches dead placement or wrapper-owned completion

1. Run:
   ```bash
   cargo run -q -p meshc -- build cluster-proof
   ```
2. Then run:
   ```bash
   cargo run -q -p meshc -- test cluster-proof/tests
   ```
3. **Expected:** the package build and tests pass. `cluster-proof/tests/work.test.mpl` proves declared work lives in `Work`, membership payloads keep runtime authority/discovery truth, and legacy placement fields are absent. `cluster-proof/tests/config.test.mpl` proves topology and durability validation on the live config seam.

### 2. The old M044 failover rail still stays green after the target move

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s04 m044_s04_auto_promotion_ -- --nocapture
   ```
2. Inspect the test output if needed.
3. **Expected:** both tests pass. One proves the declared-handler contract now lives in `Work`; the other proves automatic promotion/recovery still completes without retry and without wrapper-owned `Continuity.mark_completed(...)` glue.

### 3. The current public clustered closeout rail is M045 S04

1. Run:
   ```bash
   bash scripts/verify-m045-s04.sh
   ```
2. Inspect:
   - `.tmp/m045-s04/verify/status.txt`
   - `.tmp/m045-s04/verify/current-phase.txt`
   - `.tmp/m045-s04/verify/phase-report.txt`
   - `.tmp/m045-s04/verify/latest-failover-bundle.txt`
3. **Expected:** the script exits 0, prints `verify-m045-s04: ok`, records `ok` in `status.txt`, shows all phases as `passed`, and writes a failover bundle pointer that resolves to a copied S03 artifact root with the expected runtime-owned JSON/log files.

### 4. Public docs/readmes now point at M045 instead of M044 as the live closeout story

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m045_s04 m045_s04_docs_story_contract -- --nocapture
   ```
2. If you want an additional historical check, run:
   ```bash
   cargo test -p meshc --test e2e_m044_s05 m044_s05_closeout_ -- --nocapture
   ```
3. **Expected:** the S04 docs-story contract passes, and the optional M044 historical contract also passes without requiring the current docs to keep claiming M044 as the live clustered closeout rail.

## Edge Cases

### Nested verifier runtime prebuild is fail-closed

1. Run:
   ```bash
   bash scripts/verify-m045-s02.sh
   ```
2. Inspect `.tmp/m045-s02/verify/phase-report.txt` and `.tmp/m045-s02/verify/02-cluster-proof-tests.log`.
3. **Expected:** the script passes and the nested `cluster-proof` package-test phase does not fail with a linker error about missing `target/debug/libmesh_rt.a`; the verifier explicitly prebuilds `mesh-rt` before the nested package-test path.

### Fresh failover bundle retention stays strict

1. Run `bash scripts/verify-m045-s04.sh` twice.
2. Inspect the second run’s `.tmp/m045-s04/verify/latest-failover-bundle.txt` and the copied bundle it points to.
3. **Expected:** the pointer resolves to a retained S03 artifact root that contains exactly one `scenario-meta.json`-backed failover bundle plus the required pre-kill, post-kill, and post-rejoin runtime-owned JSON/log files. The verifier should fail if the pointer is malformed or the copied bundle shape drifts.

## Failure Signals

- `cargo test -p meshc --test e2e_m045_s04 m045_s04_ -- --nocapture` exits non-zero or reports 0 tests.
- `bash scripts/verify-m045-s04.sh` exits non-zero, prints `verification drift:`, or leaves `.tmp/m045-s04/verify/status.txt` as `failed`.
- `cargo run -q -p meshc -- test cluster-proof/tests` fails with missing legacy-field assertions, wrong declared-work ownership, or topology/durability contract drift.
- Nested verifier logs show `ld: library '.../target/debug/libmesh_rt.a' not found`, `declared_work_remote_spawn_failed`, `write_error_after_reconnect`, or malformed retained failover bundle pointers.

## Requirements Proved By This UAT

- R079 — proves the remaining example-owned distributed seams are gone from `cluster-proof`: dead placement helpers are deleted, declared work lives in `Work`, and wrapper-side manual completion is absent while the runtime-owned failover rail still passes.

## Not Proven By This UAT

- Full S05 docs-first ordering and teaching emphasis across every public clustered doc surface; S04 only moves the current closeout/readme contract onto M045.
- Any hosted or Fly deployment surface; this UAT is local-repo and local-runtime only.

## Notes for Tester

- If the assembled rail fails, start with `.tmp/m045-s04/verify/current-phase.txt`, then inspect the leaf logs in `.tmp/m045-s02/verify/` or the retained failover bundle path from `.tmp/m045-s04/verify/latest-failover-bundle.txt` before rerunning anything.
- The nested S02 replay can still surface transient remote-owner `write_error` / `write_error_after_reconnect` failures on this host. Treat the verifier as fail-closed, but confirm whether the direct `e2e_m045_s02` rail is also red before changing source code.
- Do not run the S02/S04 clustered rails in parallel; they contend on the same local proof surfaces and can create false red failures.
