---
estimated_steps: 4
estimated_files: 7
skills_used:
  - rust-best-practices
  - test
---

# T02: Move declared-work completion into the runtime path and shrink the scaffold example

**Slice:** S02 — Tiny End-to-End Clustered Example
**Milestone:** M045

## Description

Keep the public clustered example tiny by making successful declared work complete through the runtime/codegen seam, then rewrite the scaffold around that contract so `meshc init --clustered` no longer needs app-owned completion, placement, or status helpers.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Continuity completion path in `compiler/mesh-rt/src/dist/node.rs` / `compiler/mesh-rt/src/dist/continuity.rs` | Surface an explicit failure and keep the continuity record inspectable; do not fake success in the scaffold. | Bound polls in tests and fail with retained continuity/log artifacts. | Treat mismatched attempt IDs or missing execution-node truth as a runtime contract failure. |
| Clustered scaffold generator in `compiler/mesh-pkg/src/scaffold.rs` and its tooling/source-contract rails | Fail tests if generated source grows example-owned status or completion logic back in. | N/A — generation is synchronous and bounded. | Reject stale `Continuity.mark_completed`, placement helpers, or proof-app literals instead of tolerating them. |

## Load Profile

- **Shared resources**: continuity registry, spawned work actors, temporary scaffold project dirs, local ports, and CLI continuity polling.
- **Per-operation cost**: one declared-work execution plus one completion update and one scaffold init/build per proof case.
- **10x breakpoint**: pending records and process cleanup flake before raw throughput; diagnostics must show whether work ran but completion failed.

## Negative Tests

- **Malformed inputs**: stale attempt ID, missing execution-node truth, and generated source that still contains `Continuity.mark_completed`, placement helpers, or app-owned status routes.
- **Error paths**: work runs but continuity never completes, completion update rejects, or the generated scaffold build/source rails drift.
- **Boundary conditions**: local-owner completion, remote-owner completion, and a generated app that still builds while staying tiny.

## Steps

1. Extend the runtime/codegen declared-work path so a successful work execution records completion with the truthful execution node instead of leaving the record pending.
2. Rewrite the scaffolded `work.mpl`/`main.mpl`/README contract around that runtime-owned completion path, keeping only bootstrap, `/health`, and submit logic local.
3. Update tooling and source-contract rails so the generated example is pinned to the tiny runtime-owned shape instead of the older incomplete behavior.
4. Extend `compiler/meshc/tests/e2e_m045_s02.rs` with scaffold contract and completion assertions that guard R077, R079, and R080 directly.

## Must-Haves

- [ ] Scaffolded declared work reaches `phase=completed` without app-owned `Continuity.mark_completed(...)` glue.
- [ ] The generated example stays tiny and language-first.
- [ ] The scaffold README points users at runtime-owned `meshc cluster status` / `meshc cluster continuity` truth.
- [ ] Source/tooling rails fail if placement/status/completion logic leaks back into the example.

## Verification

- `cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture`
- `cargo test -p meshc --test e2e_m045_s02 m045_s02_scaffold_runtime_completion_ -- --nocapture`

## Observability Impact

- Signals added/changed: continuity completion truth for scaffolded work plus any runtime execution/completion log surface kept for debugging.
- How a future agent inspects this: `meshc cluster continuity <node> <request_key> --json`, scaffold process stdout/stderr, and the new completion-focused e2e artifacts.
- Failure state exposed: work-ran-but-not-completed vs completion-succeeded remains distinguishable.

## Inputs

- `compiler/mesh-codegen/src/declared.rs` — declared-work wrapper/body shape from T01.
- `compiler/mesh-rt/src/dist/node.rs` — runtime dispatch path that must record completion.
- `compiler/mesh-rt/src/dist/continuity.rs` — continuity completion/update contract.
- `compiler/mesh-pkg/src/scaffold.rs` — generated clustered example surface and README text.
- `compiler/meshc/tests/tooling_e2e.rs` — scaffold init contract.
- `compiler/meshc/tests/e2e_m045_s01.rs` — S01 scaffold/bootstrap contract that must stay green.
- `compiler/meshc/tests/e2e_m045_s02.rs` — new S02 proof file extended with completion/source assertions.

## Expected Output

- `compiler/mesh-codegen/src/declared.rs` — runtime-owned completion seam stays attached to declared work.
- `compiler/mesh-rt/src/dist/node.rs` — successful declared work records truthful execution completion.
- `compiler/mesh-rt/src/dist/continuity.rs` — completion contract remains explicit and inspectable.
- `compiler/mesh-pkg/src/scaffold.rs` — generated app stays tiny and free of app-owned placement/status/completion logic.
- `compiler/meshc/tests/tooling_e2e.rs` — init contract guards the new tiny example shape.
- `compiler/meshc/tests/e2e_m045_s01.rs` — upstream bootstrap/source contract remains protected.
- `compiler/meshc/tests/e2e_m045_s02.rs` — new scaffold completion/source assertions cover R077, R079, and R080.
