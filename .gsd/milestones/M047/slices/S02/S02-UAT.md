# S02: Replication-count semantics for clustered functions — UAT

**Milestone:** M047
**Written:** 2026-04-01T07:32:44.042Z

## UAT Type

- UAT mode: mixed
- Why this mode is sufficient: This slice changed three coupled seams — compiler registration metadata, runtime continuity behavior, and the user-facing `meshc cluster continuity` contract — so the honest acceptance story is a mix of unit rails for the registration/runtime seam and an end-to-end temp-project rail for real `@cluster` behavior.

## Preconditions

- Run from the repo root with the normal Rust toolchain available.
- Do not run other `e2e_m046_*` / `e2e_m047_*` route-free runtime tests in parallel.
- No extra env vars are required for the slice rails.

## Smoke Test

Run:

```bash
cargo test -p meshc --test e2e_m047_s02 -- --nocapture
```

**Expected:** 2 tests pass. The test leaves retained proof artifacts under `.tmp/m047-s02/`, including continuity JSON/human output and emitted LLVM for the ordinary `@cluster` fixture.

## Test Cases

### 1. Declared-handler registration keeps replication counts

1. Run `cargo test -p mesh-codegen m047_s02 -- --nocapture`.
2. Confirm all 6 `m047_s02` tests pass.
3. **Expected:** the codegen rail proves that default and explicit replication counts survive declared-handler planning, missing lowered symbols fail explicitly, service handlers are filtered out of startup-work registration, and emitted registration markers include the replication count argument.

### 2. Continuity records preserve runtime name and replication count

1. Run `cargo test -p mesh-rt m047_s02 -- --nocapture`.
2. Confirm all 6 `m047_s02` runtime tests pass.
3. **Expected:** the runtime rail proves that continuity records preserve `runtime_name` and public `replication_count`, required replica counts are derived from declared-handler metadata, and unsupported higher fanout is durably rejected instead of being silently clipped.

### 3. Ordinary `@cluster` functions surface truthful CLI continuity output

1. Run `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`.
2. Open the newest `.tmp/m047-s02/cli-source-cluster-counts-*/cluster-continuity-default-single-json.json` artifact.
3. Open the matching `.tmp/m047-s02/cli-source-cluster-counts-*/cluster-continuity-explicit-single-json.json` artifact.
4. **Expected:** the default record shows the generic runtime name `Work.handle_submit` with `replication_count = 2`, while the explicit-count record shows `Work.handle_retry` with `replication_count = 3` and a durable rejection path rather than a fake successful mirrored execution.

### 4. Diagnostics stay explicit for unsupported higher fanout

1. From the same retained `.tmp/m047-s02/cli-source-cluster-counts-*` bundle, open `cluster-diagnostics-json.json` and `runtime.stderr.log`.
2. Search for `unsupported_replication_count:3`.
3. **Expected:** the explicit-count fixture leaves an observable rejection reason in diagnostics/logs, so operators can tell the runtime rejected unsupported fanout instead of silently treating `@cluster(3)` as `@cluster` or local-only work.

### 5. Regression guard: route-free startup contract still works after count-bearing changes

1. Run `cargo test -p meshc --test e2e_m046_s02 m046_s02_ -- --nocapture`.
2. Confirm all 7 tests pass.
3. **Expected:** the earlier route-free startup/continuity contract still works, so S02 did not regress the runtime-owned clustered startup model while adding replication-count semantics.

## Edge Cases

### LLVM artifact location when using `--emit-llvm --output`

1. After running `cargo test -p meshc --test e2e_m047_s02 -- --nocapture`, open the newest `.tmp/m047-s02/llvm-registration-markers-*/project/` directory.
2. Check that `output.ll` exists beside the requested output binary path.
3. **Expected:** the LLVM artifact lives beside the explicit output target, not under a separate temp-project root; missing it usually means the harness looked in the wrong place, not that codegen stopped emitting LLVM.

### Single-node startup carveout for bare `@cluster`

1. From the retained `.tmp/m047-s02/cli-source-cluster-counts-*` bundle, inspect `cluster-continuity-default-single-human.log` and `cluster-status-ready.json`.
2. **Expected:** bare `@cluster` still produces a continuity record with `replication_count = 2` on a single node, but the runtime completes through the established single-node startup carveout instead of falsely claiming real mirrored execution.

## Failure Signals

- `cargo test -p meshc --test e2e_m047_s02 -- --nocapture` runs 0 tests, reports a missing target, or fails to leave `.tmp/m047-s02/...` artifacts.
- Continuity JSON/human output stops including `runtime_name` or `replication_count`.
- Emitted LLVM or continuity output reintroduces the legacy runtime name `Work.execute_declared_work` for the ordinary source-declared fixture.
- The explicit `@cluster(3)` case appears as successful mirrored execution instead of a durable rejection with an explicit reason.
- Runtime diagnostics stop surfacing `unsupported_replication_count:3` for the unsupported fanout case.

## Requirements Proved By This UAT

- R098 — proves that bare `@cluster` defaults to replication count `2`, explicit counts survive into runtime/CLI truth, and the count means replication rather than a vague execution width.

## Not Proven By This UAT

- HTTP route-local clustering syntax (`HTTP.clustered(...)`) — that remains S03 work.
- True multi-replica execution beyond one required replica — the current runtime still rejects higher fanout explicitly.
- Docs/scaffold/example migration away from `clustered(work)` and `.toml` clustering — that remains S04/S06 work.

## Notes for Tester

- The retained `.tmp/m047-s02/...` artifacts are the quickest way to debug a red rail; they preserve emitted LLVM, continuity JSON/human output, diagnostics, and runtime logs together.
- If the LLVM marker test fails on a missing file, check the explicit output directory first; `meshc build --emit-llvm --output ...` writes `output.ll` beside the binary target.
- The current GSD requirements DB does not recognize the M047 requirement family yet, so requirement status is tracked through the saved decision plus the checked-in rendered requirements file rather than through a successful `gsd_requirement_update` call.
