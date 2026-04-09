---
id: S05
parent: M053
milestone: M053
provides:
  - A refreshed hosted-verifier bundle pinned to shipped SHA `314bbac88b171388b04072a97f22be0bca4882aa`, with deploy-services green and authoritative/release blockers stated explicitly.
  - Hardened starter-proof wrapper scripts that use absolute cargo paths, prebuild `mesh-rt`, and preserve nested logs so future hosted failures remain actionable.
  - A precise remaining blocker record for downstream roadmap reassessment: standby auto-promotion still rejects with `no_mirrored_state`, and the release tag still needs an annotated reroll once main is green.
requires:
  - slice: S03
    provides: The hosted-verifier contract in `scripts/verify-m053-s03.sh` plus the authoritative/deploy/release workflow wiring that S05 refreshed against live GitHub state.
affects:
  - Remaining M053 hosted-contract closeout and any downstream roadmap slice that assumes the starter/packages evidence chain is fully green.
key_files:
  - scripts/verify-m053-s01.sh
  - scripts/verify-m053-s02.sh
  - scripts/verify-m053-s03.sh
  - compiler/meshc/tests/e2e_m053_s01.rs
  - compiler/meshc/tests/e2e_m053_s02.rs
  - .tmp/m053-s02/verify/status.txt
  - .tmp/m053-s02/verify/current-phase.txt
  - .tmp/m053-s03/verify/remote-runs.json
  - .tmp/m053-s05/rollout/main-shipped-sha.txt
  - .tmp/m053-s05/rollout/main-workflows.json
  - .tmp/m053-s05/rollout/release-workflow.json
  - .tmp/m053-s05/rollout/final-blocker.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Build the hosted rollout as an M053-only synthetic commit on top of current `origin/main` instead of pushing local `HEAD` with unrelated `/pitch` work.
  - Normalize `CARGO_HOME` / `CARGO_TARGET_DIR` to absolute repo-root paths inside the starter-proof wrappers and preserve nested S01 logs so hosted failures stay diagnosable.
  - Do not reroll `v0.1.0` until the same shipped `main` SHA is green in both `authoritative-verification.yml` and `deploy-services.yml`; otherwise the release lane would still be dishonest.
patterns_established:
  - Hosted closeout work in this repo needs three retained surfaces at once: fresh `.tmp/m053-s03/verify/` hosted-verifier output, rollout-specific `.tmp/m053-s05/rollout/` state, and downloaded remote failure bundles under `.tmp/m053-s05/remote-auth-<run-id>/`.
  - Nested shell verifiers must retain inner logs and distinguish non-zero exit from timeout; otherwise GitHub artifact bundles collapse real failures into misleading wrapper text.
  - For the current S02 failover rail, local green is not sufficient proof of hosted green: the remaining gap is a dual-stack standby promotion/mirrored-state seam that still reproduces on hosted Ubuntu runners.
observability_surfaces:
  - `.tmp/m053-s03/verify/status.txt`, `current-phase.txt`, and `remote-runs.json` now expose current hosted failure reasons against the latest shipped SHA.
  - `.tmp/m053-s05/starter-proof-repro/root-cause.md` and `ci-failure-classification.json` preserve the earlier cold-target `mesh-rt` prebuild diagnosis instead of relying on truncated workflow output.
  - `.tmp/m053-s05/rollout/main-workflows.json`, `release-workflow.json`, and `final-blocker.md` now capture the latest shipped SHA, hosted run IDs, and exact blocker text for the next recovery pass.
  - Downloaded hosted failure bundles under `.tmp/m053-s05/remote-auth-24014506220/` preserve the real S02 failover assertion and the archived `post-kill-status-standby.timeout.txt` state.
drill_down_paths:
  - .gsd/milestones/M053/slices/S05/tasks/T01-SUMMARY.md
  - .gsd/milestones/M053/slices/S05/tasks/T02-SUMMARY.md
  - .gsd/milestones/M053/slices/S05/tasks/T03-SUMMARY.md
  - .gsd/milestones/M053/slices/S05/tasks/T04-SUMMARY.md
  - .gsd/milestones/M053/slices/S05/tasks/T05-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-06T01:16:54.171Z
blocker_discovered: false
---

# S05: Hosted workflow evidence closes the starter/packages contract

**S05 hardened the hosted closeout tooling, refreshed live GitHub evidence onto the latest shipped main SHA, and made the remaining blocker explicit: deploy-services is green, but authoritative starter failover proof still fails on main and the v0.1.0 tag still lacks the annotated peeled ref required for release freshness.**

## What Happened

This slice acted as the closer for the hosted starter/packages contract rather than as a pure code-delivery pass. T01 captured a fresh hosted-red baseline under `.tmp/m053-s03/verify/` and assembled an M053-only rollout commit from `origin/main` so the remote push target excluded unrelated M056 `/pitch` work. T02 pushed that retained rollout SHA to remote `main`, proved the packages/public-surface side of the contract on the shipped commit, and downloaded the first authoritative starter-proof failure bundle instead of guessing at the blocker. T03 reproduced that hosted failure locally in a cold target dir and showed the first red was environment/setup drift: nested `meshc test <project>` rails were missing `libmesh_rt.a` in an isolated `CARGO_TARGET_DIR`. T04 fixed the local diagnostic opacity and path normalization problems by absolutizing cargo paths inside `scripts/verify-m053-s01.sh` / `scripts/verify-m053-s02.sh`, prebuilding `mesh-rt`, and retaining nested S01 verifier logs so hosted failures no longer collapse into fake timeout wording. T05 then refreshed live hosted evidence again on newer shipped SHAs, ultimately reaching `main` SHA `314bbac88b171388b04072a97f22be0bca4882aa`, where `deploy-services.yml` is green and `scripts/verify-m053-s03.sh` now fails closed for the exact remaining reasons instead of for missing or stale evidence. The remaining blocker is now precise and durable: the hosted `authoritative-verification.yml` run on that SHA fails in `m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery`, where the standby never satisfies post-kill promotion truth and records `automatic_promotion_rejected:no_mirrored_state`; separately, `refs/tags/v0.1.0` is still a lightweight tag with no peeled `^{}` ref, so release freshness cannot close until an annotated reroll happens on a green main SHA.

## Verification

Passed local closeout rails: `bash scripts/verify-m034-s02-workflows.sh`, `node --test scripts/tests/verify-m053-s03-contract.test.mjs`, `bash -n scripts/verify-m053-s01.sh`, `bash -n scripts/verify-m053-s02.sh`, `cargo test -p meshc --test e2e_m053_s01 m053_s01_retained_verifier_avoids_env_sourcing_and_later_slice_scope -- --nocapture`, `cargo test -p meshc --test e2e_m053_s02 m053_s02_retained_verifier_keeps_nested_s01_logs_and_non_timeout_failure_reasoning -- --nocapture`, `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery -- --nocapture`, and `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' bash scripts/verify-m053-s02.sh` (retained `.tmp/m053-s02/verify/status.txt=ok`, `.tmp/m053-s02/verify/current-phase.txt=complete`). Refreshed hosted closeout state with `bash scripts/verify-m053-s03.sh`; it still exits non-zero and now truthfully records `.tmp/m053-s03/verify/status.txt=failed`, `.tmp/m053-s03/verify/current-phase.txt=remote-evidence`, `deploy-services.yml` status=`ok` on shipped SHA `314bbac88b171388b04072a97f22be0bca4882aa`, `authoritative-verification.yml` status=`failed` on that same SHA, and `release.yml` status=`failed` because `refs/tags/v0.1.0^{}` is still absent. Downloaded the hosted failure bundle for run `24014506220`; it proves `m053-s01-contract.log` is green (`verify-m053-s01: ok`) and that the real remaining red is the S02 failover assertion (`automatic_promotion_rejected:no_mirrored_state` / standby promotion never converged), not missing nested logs or a wrapper timeout misclassification.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice did not reach the original all-green demo end state. Instead of closing `bash scripts/verify-m053-s03.sh` to green and rerolling `v0.1.0`, the closer froze the lane after turning hidden hosted uncertainty into explicit retained blocker evidence. The summary therefore records a truthful partial close: local rails and package-side hosted proof are green, but the starter failover proof on main and the annotated-tag reroll remain open.

## Known Limitations

`authoritative-verification.yml` is still red on shipped SHA `314bbac88b171388b04072a97f22be0bca4882aa` because the hosted S02 failover proof stalls after primary kill with `automatic_promotion_rejected:no_mirrored_state`; the archived `post-kill-status-standby.timeout.txt` still shows the standby at `cluster_role=standby`, `promotion_epoch=0`, `replication_health=healthy`. `refs/tags/v0.1.0` still points at old lightweight ref `74f2d8558b9fe7cd4cf03548e93a101308244db6` and still lacks a peeled `refs/tags/v0.1.0^{}` entry, so `release.yml` freshness remains blocked even before rerunning the tag lane. Because of those two live blockers, the slice goal (‘hosted workflow evidence closes the starter/packages contract’) is not yet fully satisfied even though the remaining gap is now tightly localized.

## Follow-ups

1. Fix the dual-stack standby promotion / mirrored-state seam behind the hosted S02 failover proof (`compiler/mesh-rt/src/dist/node.rs` plus the S02 failover harness), not the wrapper scripts.
2. After that runtime/harness repair, rerun `bash scripts/verify-m034-s02-workflows.sh`, `node --test scripts/tests/verify-m053-s03-contract.test.mjs`, and `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' bash scripts/verify-m053-s02.sh` locally.
3. Ship the next repair to `main`, require fresh green `authoritative-verification.yml` and `deploy-services.yml` runs on the same SHA, reroll `v0.1.0` as an annotated tag on that green SHA, and only then rerun `bash scripts/verify-m053-s03.sh` to close the hosted contract.

## Files Created/Modified

- `scripts/verify-m053-s01.sh` — Normalized cargo path handling and switched wrapper failures from generic success-budget text to real exit-vs-timeout reasons.
- `scripts/verify-m053-s02.sh` — Retained nested S01 verifier logs inside the S02 bundle and preserved real failure reasons for hosted starter failover runs.
- `compiler/meshc/tests/e2e_m053_s01.rs` — Locked in the tightened S01 verifier contract so hidden wrapper wording regressions fail locally.
- `compiler/meshc/tests/e2e_m053_s02.rs` — Tightened the verifier assertions and increased the local failover startup delay used by the targeted S02 proof to keep the mirrored-state window observable.
- `scripts/verify-m053-s03.sh` — Remains the authoritative hosted-verifier surface; rerunning it now refreshes the exact remote blocker against the latest shipped SHA instead of stale evidence.
- `.tmp/m053-s05/rollout/main-workflows.json` — Updated to the latest shipped SHA and current mainline workflow state, including the authoritative blocking job and green deploy-services run.
- `.tmp/m053-s05/rollout/release-workflow.json` — Updated to the latest tag-state snapshot, preserving the missing peeled-tag blocker for release freshness.
- `.tmp/m053-s05/rollout/final-blocker.md` — Rewrote the durable blocker note to point at the current shipped SHA, hosted failover failure, and pending annotated tag reroll.
- `.gsd/KNOWLEDGE.md` — Added a resume note that the remaining hosted blocker is the S02 standby promotion/mirrored-state seam, not wrapper opacity.
- `.gsd/PROJECT.md` — Refreshed current-state text so M053 includes the S05 closer outcome and its still-open hosted blocker.
