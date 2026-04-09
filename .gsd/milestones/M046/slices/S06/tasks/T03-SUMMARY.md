---
id: T03
parent: S06
milestone: M046
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/distributed/index.md", "website/docs/docs/tooling/index.md", "website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/distributed-proof/index.md", "tiny-cluster/README.md", "cluster-proof/README.md", "compiler/meshc/tests/e2e_m046_s06.rs", ".gsd/milestones/M046/slices/S06/tasks/T03-SUMMARY.md"]
key_decisions: ["Expand the S06 Rust doc/content guard from README-plus-proof-page coverage to all task-owned clustered docs/runbook surfaces so closeout-rail wording drift fails the slice gate mechanically."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-plan acceptance checks directly: `npm --prefix website run build` passed after the wording updates, `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` passed with the widened clustered-surface coverage, and the negative grep sweep confirmed the touched docs/runbooks still omit stale routeful/operator drift markers like `[cluster]`, `Continuity.submit_declared_work`, `/health`, `/work/:request_key`, and `Timer.sleep(5000)`."
completed_at: 2026-04-01T03:17:27.335Z
blocker_discovered: false
---

# T03: Repointed clustered docs/runbooks to the S06 closeout rail and expanded the S06 Rust guard across the full clustered docs surface.

> Repointed clustered docs/runbooks to the S06 closeout rail and expanded the S06 Rust guard across the full clustered docs surface.

## What Happened
---
id: T03
parent: S06
milestone: M046
key_files:
  - website/docs/docs/distributed/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/distributed-proof/index.md
  - tiny-cluster/README.md
  - cluster-proof/README.md
  - compiler/meshc/tests/e2e_m046_s06.rs
  - .gsd/milestones/M046/slices/S06/tasks/T03-SUMMARY.md
key_decisions:
  - Expand the S06 Rust doc/content guard from README-plus-proof-page coverage to all task-owned clustered docs/runbook surfaces so closeout-rail wording drift fails the slice gate mechanically.
duration: ""
verification_result: passed
completed_at: 2026-04-01T03:17:27.336Z
blocker_discovered: false
---

# T03: Repointed clustered docs/runbooks to the S06 closeout rail and expanded the S06 Rust guard across the full clustered docs surface.

**Repointed clustered docs/runbooks to the S06 closeout rail and expanded the S06 Rust guard across the full clustered docs surface.**

## What Happened

Updated the remaining clustered docs surfaces so the public story now consistently names `bash scripts/verify-m046-s06.sh` as the authoritative assembled closeout rail, keeps `bash scripts/verify-m046-s05.sh` as the lower-level equal-surface subrail, and keeps `bash scripts/verify-m045-s05.sh` explicitly historical. That included the generic distributed guide, tooling guide, clustered-example walkthrough, the distributed-proof verifier map, and both package READMEs. I also added the missing repo-wide closeout pointer to `tiny-cluster/README.md` and repointed `cluster-proof/README.md` away from stale S05-only authority wording. To keep this from drifting again, I widened `compiler/meshc/tests/e2e_m046_s06.rs` so the S06 content guard now covers the full task-owned clustered docs/runbook set and also fails on routeful/app-owned operator strings reappearing in those surfaces.

## Verification

Ran the task-plan acceptance checks directly: `npm --prefix website run build` passed after the wording updates, `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` passed with the widened clustered-surface coverage, and the negative grep sweep confirmed the touched docs/runbooks still omit stale routeful/operator drift markers like `[cluster]`, `Continuity.submit_declared_work`, `/health`, `/work/:request_key`, and `Timer.sleep(5000)`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix website run build` | 0 | ✅ pass | 49830ms |
| 2 | `cargo test -p meshc --test e2e_m046_s06 m046_s06_ -- --nocapture` | 0 | ✅ pass | 2188ms |
| 3 | `! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" README.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md website/docs/docs/tooling/index.md website/docs/docs/getting-started/clustered-example/index.md tiny-cluster/README.md cluster-proof/README.md` | 0 | ✅ pass | 173ms |


## Deviations

The pre-task local S06 guard only covered `README.md` and `website/docs/docs/distributed-proof/index.md`, which was too narrow for the seven-surface rewrite in this task. I extended `compiler/meshc/tests/e2e_m046_s06.rs` to cover the rest of the task-owned clustered docs and package runbooks in the same change.

## Known Issues

None.

## Files Created/Modified

- `website/docs/docs/distributed/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`
- `compiler/meshc/tests/e2e_m046_s06.rs`
- `.gsd/milestones/M046/slices/S06/tasks/T03-SUMMARY.md`


## Deviations
The pre-task local S06 guard only covered `README.md` and `website/docs/docs/distributed-proof/index.md`, which was too narrow for the seven-surface rewrite in this task. I extended `compiler/meshc/tests/e2e_m046_s06.rs` to cover the rest of the task-owned clustered docs and package runbooks in the same change.

## Known Issues
None.
