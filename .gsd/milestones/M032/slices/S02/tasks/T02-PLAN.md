---
estimated_steps: 4
estimated_files: 5
skills_used:
  - debug-like-expert
  - test
---

# T02: Dogfood the repaired inferred export in mesher and replay automation

**Slice:** S02 — Cross-module and inferred-export blocker retirement
**Milestone:** M032

## Description

Close the loop in real product code. This task uses the repaired inferred-export path inside mesher by moving a real batch-write helper into `Storage.Writer` and importing it from `Services.Writer`, which is a truthful dogfood surface instead of a synthetic demo. It also updates the public proof script and comment surface so the repo stops claiming `xmod_identity` is still a live blocker and `mesher/storage/writer.mpl` says only what is actually still true.

## Steps

1. Extract `flush_batch` and any storage-local helper it needs from `mesher/services/writer.mpl` into `mesher/storage/writer.mpl`, keeping its inferred collection parameter intact so the repaired compiler path is exercised by real mesher code.
2. Update `mesher/services/writer.mpl` to import and call the storage helper while leaving retry policy, service state transitions, and timer/ticker behavior unchanged.
3. Rewrite the top-of-file comment block in `mesher/storage/writer.mpl` so it drops the stale `main.mpl` / service-export limitation claim but preserves the honest raw-SQL / JSONB boundary rationale.
4. Flip `scripts/verify-m032-s01.sh` so `xmod_identity` is expected to build and run with exact stdout `7\npoly\n`, then rerun the script plus the mesher fmt/build gates until the repaired path is the new public truth.

## Must-Haves

- [ ] `mesher/storage/writer.mpl` exports a real inferred-parameter helper used by `mesher/services/writer.mpl`
- [ ] `mesher/storage/writer.mpl` comment wording is current and precise about the remaining raw-SQL boundary
- [ ] `scripts/verify-m032-s01.sh` treats `xmod_identity` as a supported path while preserving the other retained-limit checks

## Verification

- `rg -n "^pub fn flush_batch|flush_batch\(" mesher/storage/writer.mpl mesher/services/writer.mpl`
- `bash scripts/verify-m032-s01.sh`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`

## Observability Impact

- Signals added/changed: `verify-m032-s01.sh` now reports `xmod_identity` as a supported-path drift if it regresses, while mesher build failures localize dogfood breakage to `Storage.Writer` / `Services.Writer`
- How a future agent inspects this: `bash scripts/verify-m032-s01.sh`, `cargo run -q -p meshc -- build mesher`, and the import/export sites in `mesher/storage/writer.mpl` and `mesher/services/writer.mpl`
- Failure state exposed: exact replay-step name or mesher compile failure when the repaired inferred export stops working in product code

## Inputs

- `compiler/meshc/tests/e2e.rs` — repaired `m032_inferred_*` regression surface from T01
- `mesher/storage/writer.mpl` — current mixed-truth comment block and storage-only insert helper
- `mesher/services/writer.mpl` — current local `flush_batch` implementation and real dogfood call site
- `scripts/verify-m032-s01.sh` — current replay script that still treats `xmod_identity` as a retained failure
- `.tmp/m032-s01/xmod_identity/main.mpl` — canonical repaired caller fixture whose stdout the replay script must now enforce
- `.tmp/m032-s01/xmod_identity/utils.mpl` — canonical repaired callee fixture used by the replay script

## Expected Output

- `mesher/storage/writer.mpl` — inferred-parameter batch-write helper and corrected limitation wording
- `mesher/services/writer.mpl` — real import/call site using the repaired inferred export
- `scripts/verify-m032-s01.sh` — replay script updated so `xmod_identity` is verified as supported behavior
