# S02: Tiny End-to-End Clustered Example — UAT

**Milestone:** M045
**Written:** 2026-03-30T21:14:10.085Z

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice shipped a runtime/codegen seam change, a smaller scaffold contract, a real two-node runtime proof, and an assembled verifier with retained evidence. The honest acceptance surface is therefore a mix of focused compiler/runtime rails plus one end-to-end verifier replay.

## Preconditions

- Run from the repo root: `/Users/sn0w/Documents/dev/mesh-lang`.
- Rust/Cargo toolchain must be available.
- No other process should be holding the temporary ports used by the clustered scaffold tests.
- Do **not** run `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture` in parallel with `bash scripts/verify-m045-s02.sh`; they share the same two-node proof surface.
- Use the checked-in tests and verifier directly; they already fail closed on zero-test and stale-artifact drift.

## Smoke Test

Run:

```bash
cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture
```

**Expected:** the test passes and proves the runtime chose a remote owner for at least one keyed submit, completed continuity reached `phase=completed`, and the owner node actually executed the declared work.

## Test Cases

### 1. Manifest-approved declared wrappers stay remotely spawnable without widening internal helpers

1. Run `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`.
2. Confirm the target runs a real test (`running 1 test`) and passes.
3. **Expected:** manifest-approved declared work/service wrappers remain remote-spawnable, but undeclared/internal `__*` helpers are still absent from the public execution surface.

### 2. The generated clustered scaffold stays tiny and runtime-owned

1. Run `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`.
2. Run `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`.
3. **Expected:** the generated source builds, stays free of app-owned `Continuity.mark_completed(...)`, placement helpers, and status helpers, and the runtime completion rail reports `phase=completed` without any scaffold-owned completion shim.

### 3. One tiny local example proves runtime-chosen remote execution and stable completion truth

1. Run `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`.
2. Confirm all three tests pass.
3. **Expected:**
   - the scaffold contract test stays green,
   - the single-node runtime-completion test reaches `completed` without app glue,
   - the two-node rail finds a request where `owner_node != ingress_node`, then confirms `meshc cluster continuity --json` reports the same completed truth on both ingress and owner nodes,
   - duplicate submit after completion stays stable instead of reopening the work.

### 4. The assembled verifier replays prerequisites and retains a fresh proof bundle

1. Run `bash scripts/verify-m045-s02.sh`.
2. Read `.tmp/m045-s02/verify/status.txt`.
3. Read `.tmp/m045-s02/verify/phase-report.txt`.
4. Read `.tmp/m045-s02/verify/latest-proof-bundle.txt` and confirm that directory exists.
5. **Expected:** the script prints `verify-m045-s02: ok`, `status.txt` contains `ok`, every phase in `phase-report.txt` ends in `passed`, and the retained bundle path exists under `.tmp/m045-s02/verify/retained-m045-s02-artifacts`.

## Edge Cases

### `meshc cluster continuity --json` returns a direct record, not an `ok` wrapper

1. Run `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_reaches_completed_without_app_glue -- --nocapture`.
2. **Expected:** the test passes by reading `phase`, `owner_node`, and related fields directly from the top-level JSON record. Any expectation of an `{ "ok": ... }` wrapper is a contract bug in the caller.

### Duplicate submit after completion stays truthful

1. Run `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture`.
2. Inspect the retained artifacts if needed.
3. **Expected:** once the remote-owned request reaches `completed`, a duplicate submit does not re-run work locally or reopen the continuity record.

## Failure Signals

- `running 0 tests` or a missing named target where the verifier expects a real rail.
- HTTP `503` submit responses containing `declared_work_remote_spawn_failed`.
- Owner/ingress `meshc cluster continuity --json` disagreement, or continuity stuck in `submitted` / `pending` / `rejected` instead of `completed`.
- Generated scaffold source containing `Continuity.mark_completed`, placement helpers, or proof-app literals.
- `scripts/verify-m045-s02.sh` exiting non-zero, missing `latest-proof-bundle.txt`, or a retained bundle that fails the bundle-shape check.

## Requirements Advanced By This UAT

- R078 — proves cluster formation and runtime-chosen remote execution on one tiny local example; S03 still needs to add failover on that same surface.
- R080 — makes `meshc init --clustered` a credible docs-grade clustered example surface by proving the tiny scaffold end to end.
- R077 — further shrinks the primary example by moving remote execution/completion glue into runtime/codegen.
- R079 — keeps completion and status truth on runtime-owned surfaces instead of app-owned helpers.

## Not Proven By This UAT

- The failover half of R078; S03 still owns primary-loss truth on this same tiny example.
- R081’s docs-ordering promise; S05 still needs to make the public docs teach this example first.
- The final legacy-example cleanup promised by S04.

## Notes for Tester

Use `bash scripts/verify-m045-s02.sh` as the terminal acceptance rail. The focused test commands are useful for localizing failures, but the assembled verifier is the authoritative slice contract because it replays the prerequisite bootstrap/declared-work rails, reruns the full S02 proof, and retains the evidence bundle the next slices will build on.
