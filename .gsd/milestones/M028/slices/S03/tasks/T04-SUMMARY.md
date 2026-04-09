---
id: T04
parent: S03
milestone: M028
provides:
  - Truthful public/backend/editor docs for the verified `meshc init` / `meshc fmt` / `meshc test` / `meshc lsp` workflow
  - A fixed formatter path for exported type aliases so `pub type ...` survives canonical formatting on `reference-backend`
  - A stale-string verification surface that fails named docs when command drift reappears
key_files:
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/testing/index.md
  - website/docs/docs/cheatsheet/index.md
  - tools/editors/vscode-mesh/README.md
  - reference-backend/README.md
  - compiler/mesh-fmt/src/walker.rs
  - compiler/mesh-fmt/src/lib.rs
key_decisions:
  - Document only the Mesh CLI/editor surfaces that now have live proof in S03, especially `meshc init`, `meshc fmt`, project-directory `meshc test`, honest unsupported coverage, and the JSON-RPC-proven LSP feature set.
  - Fix the exported type-alias formatter regression in production code instead of freezing docs around a temporarily broken backend file, because T04's verification bar includes real `fmt`/`test`/LSP checks on `reference-backend`.
patterns_established:
  - Doc-truth tasks should close with both live command reruns and a targeted negative `rg` sweep for known stale phrases, so drift fails mechanically instead of relying on manual reading.
observability_surfaces:
  - cargo run -p meshc -- fmt --check reference-backend
  - cargo run -p meshc -- test reference-backend
  - cargo test -p meshc --test e2e_lsp -- --nocapture
  - ! rg -n "meshc new|mesh fmt|meshc test \.|mesh-lang-0\.1\.0\.vsix|Coverage reporting is available as a stub" README.md website/docs/docs/tooling/index.md website/docs/docs/testing/index.md website/docs/docs/cheatsheet/index.md tools/editors/vscode-mesh/README.md reference-backend/README.md
  - .gsd/tmp/t04-verification-results.json
duration: 2h 20m
verification_result: passed
completed_at: 2026-03-23 15:43:48 EDT
blocker_discovered: false
---

# T04: Sync docs and editor instructions to the verified tooling contract

**Synced the public/backend/editor docs to the proven S03 command surface and fixed the exported-type formatter bug that still blocked the real backend verification path.**

## What Happened

I started by reading the T04 contract, prior slice summaries, and the target docs/editor files. The docs drift was exactly what the plan called out: public docs still advertised `mesh fmt` and `meshc new`, testing docs still used `meshc test .` and stub-coverage wording, the tooling docs still referenced `mesh-lang-0.1.0.vsix`, and the backend README documented tests/coverage but not the verified fmt/LSP daily-driver loop.

Before changing docs, I reran the task verification commands to confirm local reality. That immediately exposed that T03's known backend regressions were still live in this worktree: `reference-backend/types/job.mpl` had been formatter-damaged back to `pubtype JobStatus = String`, `reference-backend/tests/config.test.mpl` had also drifted, `meshc test reference-backend` failed with a parse error, and the backend-shaped LSP e2e test failed because `Types.Job` no longer exported `Job` cleanly.

I reproduced the formatter root cause instead of repeatedly hand-editing the backend file. `meshc fmt reference-backend/types/job.mpl` was collapsing `pub type` into `pubtype`, which meant the formatter itself could reintroduce invalid syntax. The bug was in `compiler/mesh-fmt/src/walker.rs`: `walk_type_alias_def(...)` handled the `type` token but did not add a separator after a `VISIBILITY` node, unlike the function/struct walkers. I fixed that production path, then added focused regressions in `compiler/mesh-fmt/src/walker.rs` and `compiler/mesh-fmt/src/lib.rs` for `pub type` idempotency plus the real `reference-backend/types/job.mpl` canonical file.

With the formatter fixed, I restored `reference-backend/types/job.mpl` to valid source, re-canonicalized `reference-backend/` through `meshc fmt`, and confirmed the previously failing backend test/LSP paths were green again. Only after the command surface was genuinely healthy did I update the docs.

I then updated:
- `README.md` to name `meshc init`, `meshc fmt`, and `meshc test <project-or-dir>` as the real tooling surface and to surface `meshc init` in Quick Start.
- `website/docs/docs/tooling/index.md` to use `meshc init`, document `meshc fmt --check`, describe truthful project-root/directory/file `meshc test` usage plus honest `--coverage` behavior, and limit LSP/editor claims to the JSON-RPC-proven feature set.
- `website/docs/docs/testing/index.md` to show real project-root/directory/file test invocations and explicit unsupported coverage behavior.
- `website/docs/docs/cheatsheet/index.md` to remove the stale `meshc test .` example.
- `tools/editors/vscode-mesh/README.md` to use the current `0.3.0` VSIX/install flow and the proven LSP feature set.
- `reference-backend/README.md` to add the canonical daily-driver edit loop for fmt/test/LSP alongside the existing build/run/migrate commands.

I also applied the required pre-flight plan fixes: S03's slice plan now includes the docs stale-string failure check in slice verification/observability, and T04's task plan now has an `## Observability Impact` section that explains exactly how future agents inspect doc-truth drift.

## Verification

I ran the full slice verification suite after the formatter/doc repairs, not just T04's narrow task checks, because this is the final task in the slice and all slice-level proof surfaces needed to be green before closing it.

All slice checks passed:
- formatter unit/regression tests
- formatter CLI e2e tests
- `meshc fmt --check reference-backend`
- `meshc test reference-backend`
- explicit unsupported `meshc test --coverage reference-backend`
- tooling e2e
- backend-shaped JSON-RPC LSP e2e
- `mesh-lsp` test suite
- targeted stale-string negative grep across the changed doc surfaces

I also persisted the machine-readable gate output to `.gsd/tmp/t04-verification-results.json` so the exact exit codes and durations are inspectable later.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p mesh-fmt -- --nocapture` | 0 | ✅ pass | 6.76s |
| 2 | `cargo test -p meshc --test e2e_fmt -- --nocapture` | 0 | ✅ pass | 8.30s |
| 3 | `cargo run -p meshc -- fmt --check reference-backend` | 0 | ✅ pass | 6.25s |
| 4 | `cargo run -p meshc -- test reference-backend` | 0 | ✅ pass | 8.65s |
| 5 | `! cargo run -p meshc -- test --coverage reference-backend` | 0 | ✅ pass | 6.08s |
| 6 | `cargo test -p meshc --test tooling_e2e -- --nocapture` | 0 | ✅ pass | 9.79s |
| 7 | `cargo test -p meshc --test e2e_lsp -- --nocapture` | 0 | ✅ pass | 11.10s |
| 8 | `cargo test -p mesh-lsp -- --nocapture` | 0 | ✅ pass | 10.09s |
| 9 | `! rg -n "meshc new|mesh fmt|meshc test \\.|mesh-lang-0\.1\.0\.vsix|Coverage reporting is available as a stub" README.md website/docs/docs/tooling/index.md website/docs/docs/testing/index.md website/docs/docs/cheatsheet/index.md tools/editors/vscode-mesh/README.md reference-backend/README.md` | 0 | ✅ pass | 0.06s |

## Diagnostics

Future agents can inspect this task from five stable surfaces:
- `compiler/mesh-fmt/src/walker.rs` for the exported type-alias spacing fix that prevents `pub type` → `pubtype` corruption.
- `compiler/mesh-fmt/src/lib.rs` for the new formatter regressions on `pub type` aliases and the real `reference-backend/types/job.mpl` file.
- `cargo run -p meshc -- fmt --check reference-backend` and `cargo run -p meshc -- test reference-backend` for the verified backend daily-driver path.
- `cargo test -p meshc --test e2e_lsp -- --nocapture` for the transport-level LSP proof surface named in the updated docs.
- `.gsd/tmp/t04-verification-results.json` for the exact command-by-command exit codes, durations, and captured stdout/stderr from the final gate run.

## Deviations

The written task plan was doc-focused, but local execution required a production formatter fix and a backend file repair before the docs could honestly describe the verified workflow. Specifically, `meshc fmt` was still capable of corrupting exported type aliases into `pubtype ...`, which re-broke `reference-backend` and the LSP/test verification bar. I fixed that runtime/tooling defect in this unit instead of documenting around it.

## Known Issues

None.

## Files Created/Modified

- `README.md` — updated the public tooling claims to `meshc init`, `meshc fmt`, and truthful test-runner wording.
- `website/docs/docs/tooling/index.md` — synced the tooling guide to the verified fmt/test/LSP/editor contract and current VSIX install flow.
- `website/docs/docs/testing/index.md` — replaced stale directory/coverage wording with the real project-root/directory/file test contract and honest unsupported coverage behavior.
- `website/docs/docs/cheatsheet/index.md` — removed the stale `meshc test .` example.
- `tools/editors/vscode-mesh/README.md` — updated the extension install instructions to `mesh-lang-0.3.0.vsix` and limited LSP claims to the proven feature set.
- `reference-backend/README.md` — added the canonical backend fmt/test/LSP daily-driver loop alongside build/run/migrate commands.
- `compiler/mesh-fmt/src/walker.rs` — fixed exported type-alias formatting so `pub type` keeps its required separator and added a focused walker regression.
- `compiler/mesh-fmt/src/lib.rs` — added formatter idempotency/canonical regressions for `pub type` aliases and the real backend job-types module.
- `reference-backend/types/job.mpl` — restored the exported `JobStatus` alias and re-canonicalized the backend type module with the fixed formatter.
- `.gsd/milestones/M028/slices/S03/S03-PLAN.md` — added the docs stale-string slice verification surface and marked T04 done.
- `.gsd/milestones/M028/slices/S03/tasks/T04-PLAN.md` — added the missing `## Observability Impact` section.
- `.gsd/KNOWLEDGE.md` — recorded the non-obvious `pub type` formatter gotcha for future agents.
- `.gsd/tmp/t04-verification-results.json` — saved the final verification gate evidence with exit codes, durations, and captured command output.
