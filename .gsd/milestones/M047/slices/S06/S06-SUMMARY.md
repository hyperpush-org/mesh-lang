---
id: S06
parent: M047
milestone: M047
provides:
  - A dedicated built-package SQLite execute/query regression with retained `.tmp/m047-s06/sqlite-built-package-execute-*` artifacts.
  - The final `scripts/verify-m047-s06.sh` assembled closeout rail and `.tmp/m047-s06/verify/retained-proof-bundle` proof surface.
  - Public README/VitePress migration guidance that teaches canonical route-free `@cluster` surfaces first, then layers the Todo starter on top without claiming `HTTP.clustered(...)` exists.
requires:
  - slice: S04
    provides: The authoritative source-first cutover rail, migrated route-free `@cluster` contract, and legacy-surface removal that S06 reuses as a delegated authority.
  - slice: S05
    provides: The native+Docker Todo scaffold proof, prebuilt-`output` Docker contract, and ordinary `@cluster` public wording that S06 retains and wraps.
affects:
  []
key_files:
  - compiler/mesh-codegen/src/codegen/expr.rs
  - compiler/mesh-codegen/src/codegen/mod.rs
  - compiler/meshc/tests/e2e_sqlite_built_package.rs
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - README.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - compiler/meshc/tests/e2e_m047_s06.rs
  - scripts/verify-m047-s06.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Box non-pointer payloads before storing them in generic `{i8, ptr}` Result/Option slots so built-package helper rewraps stay ABI-safe.
  - Guard the SQLite AOT seam with a dedicated manifest-backed `e2e_sqlite_built_package` regression and retained `.tmp/m047-s06/sqlite-built-package-execute-*` artifacts instead of hiding it under broader Todo proof.
  - Keep the Todo runtime image as a prebuilt-`output` packager and emit a Linux `./output` first on non-Linux hosts so Docker proof stays truthful.
  - Make `scripts/verify-m047-s06.sh` own a fresh `.tmp/m047-s06/verify` bundle that copies and validates delegated S05 evidence instead of sharing or mutating `.tmp/m047-s05/verify`.
patterns_established:
  - Localize AOT/ABI regressions with narrow built-package rails that archive the emitted binary and run logs under `.tmp`, rather than waiting for broader end-to-end scaffolds to fail.
  - For generated app Docker truth on non-Linux hosts, prebuild the Linux `./output`, then retain container inspect/stdout/stderr plus raw HTTP snapshots in the same artifact bundle.
  - For closeout verifiers, replay lower-level rails, copy their verify trees into the current slice bundle, and validate status/phase/pointer files explicitly before claiming assembly success.
observability_surfaces:
  - cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture
  - cargo test -p meshc --test e2e_m047_s06 -- --nocapture
  - bash scripts/verify-m047-s06.sh
  - .tmp/m047-s06/sqlite-built-package-execute-*
  - .tmp/m047-s06/verify/status.txt
  - .tmp/m047-s06/verify/current-phase.txt
  - .tmp/m047-s06/verify/phase-report.txt
  - .tmp/m047-s06/verify/latest-proof-bundle.txt
  - .tmp/m047-s06/verify/retained-m047-s05-verify/
  - .tmp/m047-s05/todo-scaffold-runtime-truth-*
drill_down_paths:
  - .gsd/milestones/M047/slices/S06/tasks/T01-SUMMARY.md
  - .gsd/milestones/M047/slices/S06/tasks/T02-SUMMARY.md
  - .gsd/milestones/M047/slices/S06/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-01T22:30:48.066Z
blocker_discovered: false
---

# S06: Docs, migration, and assembled proof closeout

**S06 closed M047’s source-first clustered docs/proof layer by fixing the built-package SQLite AOT seam, retaining the native+Docker Todo proof under one assembled verifier, and making `scripts/verify-m047-s06.sh` the final closeout rail.**

## What Happened

S06 turned the M047 closeout into one truthful proof chain instead of three loosely related tasks. First, it fixed the AOT-only SQLite seam that was hiding under broader Todo work: helper-shaped Result/Option rewraps in built packages were storing raw scalar payloads in generic `{i8, ptr}` slots, so `meshc build` binaries could open SQLite yet fail on `Sqlite.execute(...)`. Boxing non-pointer payloads in codegen and pinning the path with `e2e_sqlite_built_package` restored built-package truth and left inspectable `.tmp/m047-s06/sqlite-built-package-execute-*` bundles.

Second, the slice kept the new Todo starter honest all the way through Docker. The shared Todo harness now retains health/CRUD snapshots, `docker inspect` output, container stdout/stderr, and timeout artifacts, and on non-Linux hosts it emits a Linux `./output` before building the runtime image so a host-native binary cannot produce a false-green container proof. That preserved the documented prebuilt-`output` packaging model while keeping the native and container rails on one generated project.

Finally, S06 finished the public closeout contract. `README.md` and the clustered docs now teach one two-layer story: three canonical route-free `@cluster` surfaces first, then `meshc init --template todo-api` as the fuller starter built on the same contract. Migration language explicitly covers `clustered(work)`, `[cluster]`, `execute_declared_work(...)`, and `Work.execute_declared_work`, while `HTTP.clustered(...)` remains an explicit non-goal. `compiler/meshc/tests/e2e_m047_s06.rs` and `scripts/verify-m047-s06.sh` now fail closed on doc authority drift, malformed delegated handoff, or overclaims, and S06 owns the assembled `.tmp/m047-s06/verify` bundle by copying S05 evidence instead of mutating the delegated verifier tree.

### Operational Readiness (Q8)
- **Health signal:** `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` runs 3 passing contract tests; `bash scripts/verify-m047-s06.sh` finishes with `.tmp/m047-s06/verify/status.txt = ok`, `.tmp/m047-s06/verify/current-phase.txt = complete`, pass markers for `contract-guards`, `m047-s05-replay`, `retain-m047-s05-verify`, `m047-s06-e2e`, `m047-s06-docs-build`, `m047-s06-artifacts`, and `m047-s06-bundle-shape`; and the retained built-package regression bundle logs `schema=ok`, `insert=1`, `count=1`, `mismatch_err=column index out of range`, `done`.
- **Failure signal:** any missing pass marker, non-`ok`/non-`complete` status files, empty or broken `latest-proof-bundle.txt`, missing retained S05 verifier files, docs that drop the migration wording or claim `HTTP.clustered(...)` shipped, or a built-package bundle that no longer reaches `schema=ok` / `done`.
- **Recovery procedure:** rerun `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture` to localize AOT/ABI drift; rerun `cargo test -p meshc --test e2e_m047_s06 -- --nocapture` for docs/verifier authority drift; rerun `bash scripts/verify-m047-s06.sh`, then inspect `.tmp/m047-s06/verify/phase-report.txt` and `latest-proof-bundle.txt`, open `retained-m047-s05-verify/` next, and follow the retained S05 bundle into native/container runtime artifacts if the lower-level Todo proof is suspect.
- **Monitoring gaps:** closeout health is still file-based rather than emitted as structured metrics, the docs build emits chunk-size warnings that do not currently fail the rail, and delegated phase reports are validated by required pass markers rather than a stricter monotonic phase ledger.

## Verification

Ran the slice-level contract rail `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`; it passed with all 3 contract tests green. Ran the assembled verifier `bash scripts/verify-m047-s06.sh`; it replayed `scripts/verify-m047-s05.sh`, reran the S06 contract rail, built the docs site, and finished successfully after 681.9s with `.tmp/m047-s06/verify/status.txt = ok`, `.tmp/m047-s06/verify/current-phase.txt = complete`, and `.tmp/m047-s06/verify/latest-proof-bundle.txt -> .tmp/m047-s06/verify/retained-proof-bundle`. Confirmed the delegated S05 verifier copied under `.tmp/m047-s06/verify/retained-m047-s05-verify/` also ended with `status.txt = ok`, `current-phase.txt = complete`, and the expected pass markers. Reran `cargo test -p meshc --test e2e_sqlite_built_package -- --nocapture`; it passed and the latest `.tmp/m047-s06/sqlite-built-package-execute-*` bundle logged `schema=ok`, `insert=1`, `count=1`, `mismatch_err=column index out of range`, and `done`. Confirmed the S06 artifact manifest retained one docs-authority contract bundle, one rail-layering contract bundle, and one verifier-contract bundle, and the wrapper docs-build log ends with `build complete in 41.08s`.

## Requirements Advanced

- R099 — S06 docs and verifier guards keep clustering framed as a general route-free `@cluster` function capability by making the canonical public surfaces route-free and explicitly marking `HTTP.clustered(...)` as unshipped.
- R102 — S06 turns `clustered(work)` / `[cluster]` into migration-only text and fails closed if the old surfaces reappear as present-tense public authority.
- R103 — S06 preserves the repo-owned route-free surfaces by reusing S04 as the delegated cutover authority and teaching `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` as the canonical public story.
- R104 — S06 replays and retains the native+Docker Todo starter proof under the final closeout bundle instead of dropping the scaffold from milestone-closeout evidence.
- R105 — S06 docs present the Todo scaffold as the fuller layered starter rather than a proof-app replacement, preserving the low-boilerplate starting-point story.

## Requirements Validated

- R106 — `cargo test -p meshc --test e2e_m047_s06 -- --nocapture`, `bash scripts/verify-m047-s06.sh`, the retained `.tmp/m047-s06/verify/` bundle, and the wrapper docs-build log now prove README/VitePress docs, migration guidance, and the final closeout verifier consistently teach one source-first clustered model while explicitly marking `HTTP.clustered(...)` as not shipped.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The T01 fix landed lower than the original task text predicted: the root cause was generic sum-variant construction in `compiler/mesh-codegen/src/codegen/expr.rs`, not MIR lowering or the SQLite runtime wrapper, so the repair shipped there alongside a focused IR/codegen regression.

To keep the Docker proof honest on this non-Linux host, T02 performs an internal Linux `./output` prebuild before packaging the runtime image instead of compiling Mesh inside the generated Dockerfile.

T03 also made small assertion updates in the delegated S04/S05 rails so the final S06 wrapper could replay the lower-level proof stack without reintroducing stale helper-shaped wording.

## Known Limitations

`HTTP.clustered(...)` remains unshipped. S06 closes on the current source-first model and the docs/verifier rails now fail closed if they imply route-local clustered HTTP support already exists.

The Todo Docker runtime model still packages a prebuilt `./output` binary. On non-Linux hosts that means emitting a Linux `./output` first rather than trusting a host-native binary or a public installer-in-container flow.

## Follow-ups

- When route-local clustering work resumes, land the real `HTTP.clustered(...)` compiler/runtime path plus an `e2e_m047_s03` proof target, then replace the current docs non-goal markers with live proof.
- If closeout automation needs stronger machine-readable health, promote the `.tmp/m047-s06/verify` status/phase/pointer files into a stricter structured summary and decide whether docs-build warnings should gate.

## Files Created/Modified

- `compiler/mesh-codegen/src/codegen/expr.rs` — Boxes scalar payloads when constructing generic Result/Option variants so built-package helper rewraps stop storing raw integers in pointer slots.
- `compiler/mesh-codegen/src/codegen/mod.rs` — Adds the focused IR/codegen regression for scalar boxing in generic Result construction.
- `compiler/meshc/tests/e2e_sqlite_built_package.rs` — Proves built-package SQLite CREATE/INSERT/query/mismatch behavior through a manifest-backed rail with retained artifacts.
- `compiler/meshc/tests/support/m047_todo_scaffold.rs` — Extends the Todo harness with retained native/container HTTP snapshots, Docker inspect/log capture, and Linux prebuild support for non-Linux hosts.
- `compiler/meshc/tests/e2e_m047_s05.rs` — Extends the Todo end-to-end rail to cover native + container proof, negative Docker paths, and delegated assertion updates needed for S06 replay.
- `compiler/mesh-pkg/src/scaffold.rs` — Keeps the Todo scaffold source, README, and Docker contract aligned with source-first `@cluster` and prebuilt-output packaging.
- `README.md` — Documents the canonical route-free `@cluster` surfaces, migration path, Todo layering, and S04/S05/S06 verification rails.
- `website/docs/docs/tooling/index.md` — Rewrites tooling docs so the Todo starter is layered on top of the same route-free `@cluster` contract.
- `website/docs/docs/getting-started/clustered-example/index.md` — Keeps the canonical clustered walkthrough route-free while pointing users to the Todo template for the fuller starter.
- `website/docs/docs/distributed-proof/index.md` — Layers the S04, S05, and S06 proof rails and keeps `HTTP.clustered(...)` explicitly out of scope.
- `website/docs/docs/distributed/index.md` — Teaches the three route-free clustered surfaces first, then layers the Todo starter on top without changing the contract.
- `compiler/meshc/tests/e2e_m047_s06.rs` — Adds the closeout contract tests for docs authority, rail layering, and verifier bundle ownership.
- `scripts/verify-m047-s06.sh` — Adds the final wrapper that replays S05, retains delegated verify state, copies fresh S06 artifacts, and publishes the final bundle pointer.
- `.gsd/KNOWLEDGE.md` — Captures the built-package SQLite, non-Linux Docker, and S06 bundle-debugging gotchas future agents are likely to hit.
- `.gsd/PROJECT.md` — Refreshes current-state project context to include the S06 closeout rail and retained proof surfaces.
