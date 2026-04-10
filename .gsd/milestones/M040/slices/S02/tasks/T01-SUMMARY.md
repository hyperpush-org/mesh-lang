---
id: T01
parent: S02
milestone: M040
key_files:
  - compiler/mesh-codegen/src/mir/lower.rs
  - compiler/mesh-codegen/src/codegen/expr.rs
  - compiler/mesh-codegen/src/lib.rs
  - cluster-proof/work.mpl
  - cluster-proof/main.mpl
  - .gsd/milestones/M040/slices/S02/tasks/T01-SUMMARY.md
key_decisions:
  - Let non-root lexical bindings shadow root-scope function registrations during MIR lowering so local strings cannot be rewritten into function symbols.
  - Keep the keyed S01 /work contract intact and restore the old GET /work rail as a compatibility adapter instead of reverting the keyed API.
duration: 
verification_result: mixed
completed_at: 2026-03-28T19:04:17.458Z
blocker_discovered: false
---

# T01: Fixed the clustered startup shadowing crash and staged legacy /work compatibility, but final M039/S03 verification still needs rerun.

**Fixed the clustered startup shadowing crash and staged legacy /work compatibility, but final M039/S03 verification still needs rerun.**

## What Happened

Reproduced the exact clustered startup abort, traced it through cluster-proof startup and emitted LLVM, and found the real root cause in compiler lowering/codegen: a same-named local string binding was being resolved as a top-level function symbol and passed into mesh_string_length. Fixed lexical shadowing in compiler/mesh-codegen/src/mir/lower.rs, added a defensive local-first load in compiler/mesh-codegen/src/codegen/expr.rs, and added a regression test in compiler/mesh-codegen/src/lib.rs. After that, the exact M039/S03 repro advanced from crashing before readiness to both nodes reaching `work services ready`, which exposed the next red path: the old proof still expects GET /work with the legacy response/log shape. Staged a narrow compatibility adapter in cluster-proof/work.mpl and cluster-proof/main.mpl to preserve the keyed S01 API while restoring the old proof rail, but the context-budget warning arrived before I could run the final compile and e2e rerun on that staged patch.

## Verification

Verified the original red path with the exact M039/S03 command, then verified the compiler shadowing fix with a targeted mesh-codegen regression test. Rebuilt cluster-proof with emitted LLVM and confirmed the bad `@node_name` string-intrinsic pattern disappeared. Re-ran the exact M039/S03 proof after the compiler fix and confirmed the startup crash was retired because both nodes reached `work services ready`; the remaining failure moved to the legacy GET /work compatibility seam. Final build/e2e verification after the staged compatibility patch is still pending.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-codegen llvm_codegen_prefers_local_string_binding_over_same_named_function -- --nocapture` | 0 | ✅ pass | 15400ms |
| 2 | `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture` | 101 | ❌ fail | 18900ms |

## Deviations

The actual root cause was a compiler lowering/codegen shadowing bug rather than a narrow runtime string implementation bug, so the repair landed in mesh-codegen. After retiring the crash, I also had to begin a compatibility adapter for the legacy GET /work proof rail because the exact M039/S03 harness still depends on it.

## Known Issues

Final verification is incomplete. The staged cluster-proof/main.mpl and cluster-proof/work.mpl compatibility changes were not recompiled or rerun through `cargo run -q -p meshc -- build cluster-proof` and the exact `cargo test -p meshc --test e2e_m039_s03 e2e_m039_s03_rejoins_and_routes_to_peer_again_without_manual_repair -- --nocapture` command after the context-budget warning interrupted execution.

## Files Created/Modified

- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `cluster-proof/work.mpl`
- `cluster-proof/main.mpl`
- `.gsd/milestones/M040/slices/S02/tasks/T01-SUMMARY.md`
