---
id: T03
parent: S06
milestone: M034
provides: []
requires: []
affects: []
key_files: [".tmp/m034-s06/transport-recovery/attempts.log", ".tmp/m034-s06/transport-recovery/02-http11-repro.stderr", ".tmp/m034-s06/transport-recovery/03-http11-postbuffer-1g.stderr", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M034/slices/S06/tasks/T03-SUMMARY.md"]
key_decisions: ["Treat a 1 GiB buffered HTTPS push that still ends in HTTP 408 as the same transport blocker, not as proof that `origin/main` advanced or that hosted `main` evidence can be archived.", "Keep the deterministic `main` archive label unspent because remote `main` never reached the rollout SHA and no fresh `push` runs exist for it."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Re-ran `bash scripts/verify-m034-s05-workflows.sh`, `bash scripts/verify-m034-s02-workflows.sh`, `bash scripts/verify-m034-s04-workflows.sh`, and `bash -n scripts/verify-m034-s05.sh` on the current branch. Exercised two bounded HTTPS push attempts against `bddedd52aeb67f7b9e4c165e684063e2812c3953`: the plain HTTP/1.1 repro and a buffered retry with `http.postBuffer=1073741824`. Re-checked the remote state with `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'`, `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url`, and `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url` to confirm the rollout did not land."
completed_at: 2026-03-27T05:28:25.642Z
blocker_discovered: true
---

# T03: Captured repeated HTTPS rollout push failures and left durable transport-recovery evidence for `main`.

> Captured repeated HTTPS rollout push failures and left durable transport-recovery evidence for `main`.

## What Happened
---
id: T03
parent: S06
milestone: M034
key_files:
  - .tmp/m034-s06/transport-recovery/attempts.log
  - .tmp/m034-s06/transport-recovery/02-http11-repro.stderr
  - .tmp/m034-s06/transport-recovery/03-http11-postbuffer-1g.stderr
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M034/slices/S06/tasks/T03-SUMMARY.md
key_decisions:
  - Treat a 1 GiB buffered HTTPS push that still ends in HTTP 408 as the same transport blocker, not as proof that `origin/main` advanced or that hosted `main` evidence can be archived.
  - Keep the deterministic `main` archive label unspent because remote `main` never reached the rollout SHA and no fresh `push` runs exist for it.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T05:28:25.643Z
blocker_discovered: true
---

# T03: Captured repeated HTTPS rollout push failures and left durable transport-recovery evidence for `main`.

**Captured repeated HTTPS rollout push failures and left durable transport-recovery evidence for `main`.**

## What Happened

Started from the T02 rollout target and current local `main` state, then verified that `HEAD` (`bddedd52aeb67f7b9e4c165e684063e2812c3953`) is a descendant of the previously validated rollout commit (`f4eb54b7f2e51af871d5afca4dfa6a017ff81275`) with no `.github/workflows/` or `scripts/` drift, so advancing `origin/main` to `HEAD` would preserve the already-validated workflow graph. Re-ran the local workflow contract gates on the current branch, confirmed both the T02 rollout commit and current `HEAD` are still absent from GitHub’s object database, and confirmed SSH push is unavailable on this host (`Permission denied (publickey)`). Recorded bounded transport-recovery attempts under `.tmp/m034-s06/transport-recovery/`. The plain HTTPS/HTTP 1.1 repro failed after 717629 ms with the same `POST git-receive-pack (chunked)` → `HTTP 408` / `unexpected disconnect while reading sideband packet` signature captured in T02. Measured the ahead-range object set at roughly 856837246 bytes, then retried with `http.postBuffer=1073741824`; that removed the chunked marker and produced `POST git-receive-pack (564496785 bytes)`, but it still failed with the same HTTP 408 after 759760 ms. After both attempts, `origin/main` remained at `5ddf3b2dce17abe08e1188d9b46e575d83525b50`, the latest `deploy.yml` `push` run on `main` was still the old success on that stale SHA, and `authoritative-verification.yml` was still not present on the remote default branch. Because the remote default branch never advanced to the rollout SHA, there was no truthful fresh hosted `push` evidence to archive under `.tmp/m034-s06/evidence/main/`, so the remote-evidence helper was intentionally not run.

## Verification

Re-ran `bash scripts/verify-m034-s05-workflows.sh`, `bash scripts/verify-m034-s02-workflows.sh`, `bash scripts/verify-m034-s04-workflows.sh`, and `bash -n scripts/verify-m034-s05.sh` on the current branch. Exercised two bounded HTTPS push attempts against `bddedd52aeb67f7b9e4c165e684063e2812c3953`: the plain HTTP/1.1 repro and a buffered retry with `http.postBuffer=1073741824`. Re-checked the remote state with `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'`, `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url`, and `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url` to confirm the rollout did not land.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m034-s05-workflows.sh` | 0 | ✅ pass | 1100ms |
| 2 | `bash scripts/verify-m034-s02-workflows.sh` | 0 | ✅ pass | 1500ms |
| 3 | `bash scripts/verify-m034-s04-workflows.sh` | 0 | ✅ pass | 880ms |
| 4 | `bash -n scripts/verify-m034-s05.sh` | 0 | ✅ pass | 10ms |
| 5 | `env GIT_TERMINAL_PROMPT=0 git -c http.version=HTTP/1.1 push --verbose origin bddedd52aeb67f7b9e4c165e684063e2812c3953:refs/heads/main` | 1 | ❌ fail | 717629ms |
| 6 | `env GIT_TERMINAL_PROMPT=0 git -c http.version=HTTP/1.1 -c http.postBuffer=1073741824 push --verbose origin bddedd52aeb67f7b9e4c165e684063e2812c3953:refs/heads/main` | 1 | ❌ fail | 759760ms |
| 7 | `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'` | 0 | ❌ fail | 530ms |
| 8 | `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url` | 0 | ❌ fail | 780ms |
| 9 | `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json databaseId,status,conclusion,headSha,url` | 1 | ❌ fail | 480ms |


## Deviations

Used current `HEAD` (`bddedd52aeb67f7b9e4c165e684063e2812c3953`) as the push target instead of reusing T02’s exact `f4eb54b7f2e51af871d5afca4dfa6a017ff81275` SHA, because `HEAD` is a descendant with no `.github/workflows/` or `scripts/` drift and advancing to `HEAD` would keep remote `main` aligned with the current rollout branch if transport succeeded. Also recorded and discarded an initial harness mistake (`timeout` is not installed on this host) before the real bounded repro attempts. Did not run `bash scripts/verify-m034-s06-remote-evidence.sh main || true` and did not create `.tmp/m034-s06/evidence/main/`, because the remote branch never advanced and any `main` archive would have been untruthful.

## Known Issues

This host still cannot land the rollout over HTTPS: both the default chunked push and a 1 GiB buffered non-chunked push fail with `HTTP 408` after roughly twelve minutes, leaving `origin/main` unchanged at `5ddf3b2dce17abe08e1188d9b46e575d83525b50`. SSH push is unavailable, neither the T02 rollout commit nor current `HEAD` exists remotely yet, `authoritative-verification.yml` is still absent from the remote default branch, and downstream hosted-evidence tasks T04/T05 remain blocked until a transport path that can upload the ~565 MB receive-pack payload is available.

## Files Created/Modified

- `.tmp/m034-s06/transport-recovery/attempts.log`
- `.tmp/m034-s06/transport-recovery/02-http11-repro.stderr`
- `.tmp/m034-s06/transport-recovery/03-http11-postbuffer-1g.stderr`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M034/slices/S06/tasks/T03-SUMMARY.md`


## Deviations
Used current `HEAD` (`bddedd52aeb67f7b9e4c165e684063e2812c3953`) as the push target instead of reusing T02’s exact `f4eb54b7f2e51af871d5afca4dfa6a017ff81275` SHA, because `HEAD` is a descendant with no `.github/workflows/` or `scripts/` drift and advancing to `HEAD` would keep remote `main` aligned with the current rollout branch if transport succeeded. Also recorded and discarded an initial harness mistake (`timeout` is not installed on this host) before the real bounded repro attempts. Did not run `bash scripts/verify-m034-s06-remote-evidence.sh main || true` and did not create `.tmp/m034-s06/evidence/main/`, because the remote branch never advanced and any `main` archive would have been untruthful.

## Known Issues
This host still cannot land the rollout over HTTPS: both the default chunked push and a 1 GiB buffered non-chunked push fail with `HTTP 408` after roughly twelve minutes, leaving `origin/main` unchanged at `5ddf3b2dce17abe08e1188d9b46e575d83525b50`. SSH push is unavailable, neither the T02 rollout commit nor current `HEAD` exists remotely yet, `authoritative-verification.yml` is still absent from the remote default branch, and downstream hosted-evidence tasks T04/T05 remain blocked until a transport path that can upload the ~565 MB receive-pack payload is available.
