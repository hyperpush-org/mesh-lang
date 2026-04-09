---
id: S02
parent: M045
milestone: M045
provides:
  - A tiny scaffold-first clustered example whose happy-path distributed behavior stays runtime-owned instead of example-owned.
  - Runtime/codegen-owned declared-work completion for scaffolded clustered apps.
  - A two-node local proof rail that shows runtime-chosen remote execution and completed continuity truth on both ingress and owner nodes.
  - A fail-closed assembled verifier with retained artifacts that downstream failover/docs slices can reuse.
requires:
  - slice: S01
    provides: Runtime-owned clustered bootstrap via `Node.start_from_env()`, typed `BootstrapStatus`, and the smaller scaffold/public bootstrap contract.
affects:
  - S03
  - S04
  - S05
key_files:
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/mesh-codegen/src/codegen/expr.rs
  - compiler/mesh-codegen/src/codegen/intrinsics.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/mesh-rt/src/dist/continuity.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - compiler/meshc/tests/tooling_e2e.rs
  - compiler/meshc/tests/e2e_m045_s02.rs
  - scripts/verify-m045-s02.sh
key_decisions:
  - Keep declared-handler remote execution narrow by registering only manifest-approved `__declared_work_*` wrapper symbols for remote spawn; other compiler-internal `__*` helpers stay hidden.
  - Record successful declared-work completion in the runtime/codegen wrapper path instead of adding scaffold-specific `Continuity.mark_completed(...)` or app-owned status/completion helpers.
  - Use ingress-side `meshc cluster status --json` plus dual-node `meshc cluster continuity --json` as the authoritative runtime-owned proof surface, and replay direct prerequisite commands in `scripts/verify-m045-s02.sh` instead of nesting older verifier wrappers.
patterns_established:
  - For clustered example work, keep remote declared-handler exposure narrow: only manifest-approved declared wrapper symbols should be remote-spawnable.
  - Record successful declared-work completion inside the runtime/codegen wrapper path so generated apps stay business-logic-first and do not reintroduce `Continuity.mark_completed(...)` glue.
  - For scaffold-first clustered proofs, trust runtime-owned CLI truth (`meshc cluster status` and `meshc cluster continuity`) plus retained evidence bundles instead of local placement prediction or app-owned status endpoints.
  - Assembled clustered verifiers should retain copied proof bundles and own the cluster exclusively; concurrent replays on the same surface create false red rails.
observability_surfaces:
  - `/health` on the generated clustered app still provides local service readiness.
  - `meshc cluster status <ingress-node> --json` is the runtime-owned cluster-formation and authority truth surface for the tiny example.
  - `meshc cluster continuity <node> <request_key> --json` on both ingress and owner nodes is the authoritative completion truth surface for remote-owner work.
  - `.tmp/m045-s02/verify/retained-m045-s02-artifacts/` plus `phase-report.txt`, `status.txt`, and the copied node logs are the durable diagnostic bundle when the two-node proof drifts.
  - Owner-node stderr still exposes actionable failure signals such as `mesh node spawn failed ...` and continuity transitions when remote execution regresses.
drill_down_paths:
  - .gsd/milestones/M045/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M045/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M045/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-30T21:14:10.084Z
blocker_discovered: false
---

# S02: Tiny End-to-End Clustered Example

**The clustered scaffold is now a tiny two-node example that proves runtime-chosen remote execution and completed continuity truth without app-owned placement, completion, or status helpers.**

## What Happened

S02 finished the tiny clustered example’s happy-path story and kept the ownership boundary honest. T01 repaired the remote declared-handler seam by registering only manifest-approved declared wrapper symbols for remote spawn, which let the owner node resolve `__declared_work_*` targets without widening all compiler-internal helpers into the public execution surface. The new focused `compiler/meshc/tests/e2e_m045_s02.rs` rail stopped trusting local placement prediction and instead proved remote execution from the submit response’s actual `owner_node`, then required completed continuity truth on both ingress and owner nodes plus owner-node execution evidence.

T02 moved successful declared-work completion into the runtime/codegen seam so the scaffold no longer needed app-owned completion glue. Generated declared-work wrappers now call a narrow runtime completion helper that records the truthful execution node, and the clustered scaffold was rewritten around that contract: `main.mpl` submits directly to the declared work target, `work.mpl` stays a pure work body, and the README points users at runtime-owned `meshc cluster status` / `meshc cluster continuity` truth instead of app-owned placement or status helpers. Source-contract rails now reject leaked `Continuity.mark_completed(...)`, stale proof-app literals, and app-owned status helpers.

T03 added the public proof surface for the slice. The two-node rail now initializes a clustered scaffold project, boots two local nodes, retries request keys until the runtime chooses a remote owner, checks runtime-owned `meshc cluster status --json` and dual-node `meshc cluster continuity --json` output, and retains the resulting HTTP/CLI/node-log artifacts under `.tmp/m045-s02`. `scripts/verify-m045-s02.sh` became the assembled stopping condition: it replays the direct prerequisite rails, reruns the full `m045_s02_` filter, copies the fresh proof bundle into `.tmp/m045-s02/verify/retained-m045-s02-artifacts/`, and fail-closes on stale artifact or bundle-shape drift.

Net result: the scaffold-first clustered example is now honestly tiny and end to end for the happy path. Remote-owner execution, completion, and status truth all come from runtime/codegen plus public runtime inspection surfaces, while `cluster-proof` is left as the deeper secondary rail rather than the place users must learn the core clustered story from.

## Verification

I reran every slice-plan verification command and the slice is green:

- `cargo test -p meshc --test e2e_m044_s02 m044_s02_declared_work_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_declared_work_remote_spawn_ -- --nocapture`
- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture`
- `bash scripts/verify-m045-s02.sh`

The final verifier pass also confirmed the runtime/diagnostic surfaces this slice depends on: `/health` for local readiness, `meshc cluster status --json` for cluster formation/authority truth, `meshc cluster continuity --json` on both ingress and owner nodes for completed continuity truth, and the retained proof bundle under `.tmp/m045-s02/verify/retained-m045-s02-artifacts/` with `phase-report.txt`, `status.txt`, and `latest-proof-bundle.txt` all in the expected shape.

## Requirements Advanced

- R078 — The tiny scaffold-first example now proves cluster formation and runtime-chosen remote execution on two local nodes, leaving only failover on that same example for S03.
- R080 — `meshc init --clustered` is now a credible docs-grade clustered example surface because the generated app stays small while still proving remote-owner execution and runtime-owned completion end to end.
- R077 — The primary example surface got smaller again: remote-spawn registration and declared-work completion moved into runtime/codegen, so the scaffold stays closer to business logic plus minimal ingress/work code.
- R079 — The scaffold no longer carries app-owned completion or status truth for successful declared work; the proof rail relies on runtime CLI surfaces instead of placement/status helpers.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Instead of nesting `scripts/verify-m045-s01.sh` and the older M044 wrappers literally, the terminal S02 verifier replays their direct prerequisite commands. On this host the nested wrapper stack can hang after `cluster-proof/tests` already logged green, so flattening the replay preserved the intended prerequisite coverage without leaving the acceptance rail stuck.

## Known Limitations

S02 proves the tiny example’s happy path — cluster formation, runtime-chosen remote execution, runtime-owned completion, and duplicate-submit stability — but it does not yet prove primary loss and failover on that same example; S03 still owns that half of R078. Public docs still have not been rewritten to lead with the scaffold-first example, so R081 remains open for S05. The older nested verifier chain remains flaky on this host, but the new top-level S02 verifier avoids it by replaying the direct prerequisite commands.

## Follow-ups

- S03 should reuse this exact scaffold-first example and retained bundle shape to prove primary-loss/failover truth on the same surface instead of switching back to a proof-app-only rail.
- S04 should collapse the remaining legacy example-side continuity/config residue now that bootstrap, remote-owner execution, and completion are runtime-owned on the tiny example.
- S05 should make the public docs teach this scaffold-first example before `cluster-proof`, while keeping `scripts/verify-m045-s02.sh` as the local happy-path acceptance rail.

## Files Created/Modified

- `compiler/mesh-codegen/src/codegen/mod.rs` — Startup registration now includes only manifest-approved declared wrapper executable symbols in the remote-spawn registry.
- `compiler/mesh-codegen/src/codegen/expr.rs` — Generated declared-work actor wrappers now call the runtime completion seam after successful execution.
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — Codegen exposes the runtime completion intrinsic used by declared-work wrappers.
- `compiler/mesh-rt/src/dist/node.rs` — Remote declared-work dispatch remains fail-closed and records truthful execution-node completion through the runtime path.
- `compiler/mesh-rt/src/dist/continuity.rs` — Continuity completion helpers now let successful declared work complete without app-owned `Continuity.mark_completed(...)` glue.
- `compiler/mesh-pkg/src/scaffold.rs` — The clustered scaffold now stays tiny and runtime-owned, with submit/work flow and README text aligned to public CLI truth surfaces.
- `compiler/meshc/tests/tooling_e2e.rs` — Tooling rails now reject leaked placement, completion, and status-helper logic in `meshc init --clustered` output.
- `compiler/meshc/tests/e2e_m045_s02.rs` — The new slice e2e rail proves scaffold source shape, runtime-owned completion, two-node remote-owner execution, duplicate-submit stability, and retained artifact truth.
- `compiler/meshc/tests/e2e_m044_s02.rs` — The older declared-work regression still protects the manifest gate while the remote-spawn seam changed.
- `compiler/meshc/tests/e2e_m044_s01.rs` — A stale manifest fixture was updated to the current public declared-work contract so the S02 verifier prerequisites stay truthful.
- `scripts/verify-m045-s02.sh` — Added the fail-closed assembled slice verifier with retained bundle and bundle-shape checks.
