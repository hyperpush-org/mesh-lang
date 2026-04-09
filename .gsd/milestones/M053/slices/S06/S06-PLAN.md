# S06: Hosted failover promotion truth and annotated tag reroll

**Goal:** Close the M053 hosted red honestly: restore the runtime-owned startup pending window the staged Postgres starter already exports, make the S02 failover proof fail closed on that window, and close the hosted verifier only after fresh mainline proof plus an annotated `v0.1.0` reroll on the same green SHA.
**Demo:** After this: Run `bash scripts/verify-m053-s03.sh` to green so `.tmp/m053-s03/verify/status.txt` is `ok`, `remote-runs.json` shows fresh successful `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs on the expected refs, and `refs/tags/v0.1.0^{}` resolves after the annotated reroll.

## Tasks
- [x] **T01: Restored the runtime-owned startup dispatch window env override and added fail-closed unit coverage.** — - Why: Hosted red currently depends on a runtime bug: the staged Postgres starter harness exports `MESH_STARTUP_WORK_DELAY_MS`, but `startup_dispatch_window_ms(...)` ignores it and closes the promotable window at 2500ms on hosted Ubuntu.
- Do: Restore the runtime-owned env override inside `compiler/mesh-rt/src/dist/node.rs`, keep 2500ms as the absent-env default, preserve zero-delay behavior for non-startup or replica-free requests, and extend the nearby runtime tests instead of pushing the fix back into starter source or wrapper scripts.
- Done when: `startup_dispatch_window_ms(...)` honors a positive `MESH_STARTUP_WORK_DELAY_MS` for runtime-owned clustered startup work, invalid or missing env falls back to the safe default, and targeted runtime tests cover override plus default behavior.

## Steps

1. Add a small helper next to `startup_dispatch_window_ms(...)` that reads/parses `MESH_STARTUP_WORK_DELAY_MS` once and returns either the configured positive delay or the existing `STARTUP_CLUSTERED_PENDING_WINDOW_MS` fallback.
2. Extend the nearby runtime tests in `compiler/mesh-rt/src/dist/node.rs` to prove override behavior, default behavior when the env is absent, and zero-delay behavior for non-startup requests or `required_replica_count == 0`.
3. Keep the change runtime-owned only: do not add app-owned `Timer.sleep(...)`, README knobs, or wrapper-script-only workarounds.

## Must-Haves

- [ ] A positive `MESH_STARTUP_WORK_DELAY_MS` changes the runtime-owned clustered startup window without affecting non-startup requests.
- [ ] Missing, zero, negative, or malformed env values fall back to the safe default instead of widening the contract or crashing startup work.
- [ ] The proof lives in runtime tests near `startup_dispatch_window_ms(...)`, so later agents can diagnose regressions without reopening starter code or hosted workflows.
  - Estimate: 1 context window
  - Files: compiler/mesh-rt/src/dist/node.rs
  - Verify: cargo test -p mesh-rt startup_work_dispatch_window_ -- --nocapture
- [x] **T02: Made the staged Postgres failover rail fail closed on pre-kill startup-window evidence.** — - Why: S05 already proved the hosted red is not a missing mirror transport seam; the retained bundle shows standby saw mirrored pending state, then primary completed startup before the kill. The local S02 rail must now fail closed on that timing invariant instead of passing by luck.
- Do: Tighten `compiler/meshc/tests/e2e_m053_s02.rs` around the startup diagnostics and retained bundle shape so the generated-starter failover proof records the configured `pending_window_ms`, rejects an owner-completed-before-kill run, and keeps `.tmp/m053-s02/verify/` as the starter-owned proof bundle that Task 3 will ship.
- Done when: the authoritative S02 e2e and assembled `bash scripts/verify-m053-s02.sh` both go green only when the configured startup window is visible in diagnostics and the owner-loss/promotion/recovery path remains truthful.

## Steps

1. Use the hosted red bundle under `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/` as comparison evidence for the exact failure shape: mirrored pending exists, `primary-run1.combined.log` later reaches `startup_completed`, and post-kill standby never promotes.
2. Update `compiler/meshc/tests/e2e_m053_s02.rs` (and a small helper in `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` if it simplifies the assertion) so the local failover rail explicitly checks startup diagnostics metadata for the configured `pending_window_ms` and fails if `startup_completed` beats the forced owner stop.
3. Re-run the targeted e2e and assembled `bash scripts/verify-m053-s02.sh` against a disposable Postgres URL so `.tmp/m053-s02/verify/` becomes the retained local proof bundle for hosted closeout.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e_m053_s02.rs` fails when runtime-owned startup diagnostics fall back to 2500ms instead of the configured delay.
- [ ] The destructive rail fails when the owner reaches `startup_completed` before the forced stop or when standby never records `automatic_promotion` / `automatic_recovery`.
- [ ] The green local replay still proves real starter HTTP CRUD/read behavior through `.tmp/m053-s02/verify/` without widening SQLite, packages, or docs scope.
  - Estimate: 1 context window
  - Files: compiler/meshc/tests/e2e_m053_s02.rs, compiler/meshc/tests/support/m053_todo_postgres_deploy.rs, .tmp/m053-s02/verify/status.txt, .tmp/m053-s02/verify/latest-proof-bundle.txt
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery -- --nocapture && DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m053-s02.sh
- [x] **T03: Ship the repair and close hosted/tag freshness on one SHA** — - Why: The slice does not close locally; it closes only when the repaired `main` SHA is green in both hosted starter proof and packages proof, and the binary tag is rerolled as annotated so `refs/tags/v0.1.0^{}` resolves for release freshness.
- Do: Refresh hosted evidence with `bash scripts/verify-m053-s03.sh`, keep `deploy-services.yml` green on the same shipped SHA, and then push/reroll only after explicit user confirmation for the outward-facing GitHub mutations. Do not mutate `main` or `v0.1.0` speculatively.
- Done when: after explicit approval for the remote mutations, fresh `authoritative-verification.yml`, `deploy-services.yml`, and `release.yml` runs are green on the expected refs, `bash scripts/verify-m053-s03.sh` turns green, and `git ls-remote` resolves `refs/tags/v0.1.0^{}`. Without approval, stop blocked with the retained mutation plan instead of pretending the slice is complete.

## Steps

1. Re-run the local workflow/verifier preflights and refresh `.tmp/m053-s03/verify/` in read-only mode so Task 2’s green starter bundle and the current shipped SHA are captured before any remote mutation.
2. After explicit user confirmation, push the S06 repair commit to `main`, wait for fresh green `authoritative-verification.yml` and already-green `deploy-services.yml` runs on that exact SHA, and record the alignment in `.tmp/m053-s06/rollout/remote-mutation-plan.md` plus the refreshed hosted bundle.
3. After explicit user confirmation for the tag mutation, reroll `v0.1.0` as an annotated tag on the same green SHA, wait for `release.yml`, rerun `bash scripts/verify-m053-s03.sh`, and write the final closeout or exact remaining blocker to `.tmp/m053-s06/rollout/final-hosted-closeout.md`.

## Must-Haves

- [ ] The repaired S06 commit is pushed to `main` only after Task 2 is green locally and the user explicitly approves the remote mutation.
- [ ] Fresh `authoritative-verification.yml` and `deploy-services.yml` runs are green on the same shipped SHA before the annotated tag reroll begins.
- [ ] After explicit approval for the tag mutation, `v0.1.0` is rerolled as annotated on that SHA, `refs/tags/v0.1.0^{}` resolves, and `bash scripts/verify-m053-s03.sh` turns green; otherwise the task stops blocked with a retained mutation plan.
  - Estimate: 1 context window
  - Files: .tmp/m053-s03/verify/status.txt, .tmp/m053-s03/verify/current-phase.txt, .tmp/m053-s03/verify/remote-runs.json, .tmp/m053-s06/rollout/remote-mutation-plan.md, .tmp/m053-s06/rollout/release-workflow.json, .tmp/m053-s06/rollout/final-hosted-closeout.md
  - Verify: bash scripts/verify-m034-s02-workflows.sh && node --test scripts/tests/verify-m053-s03-contract.test.mjs && GH_TOKEN=${GH_TOKEN:?set GH_TOKEN} bash scripts/verify-m053-s03.sh && git ls-remote --quiet origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'
