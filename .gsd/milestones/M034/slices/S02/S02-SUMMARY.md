---
id: S02
parent: M034
milestone: M034
provides:
  - A reusable GitHub Actions workflow (`.github/workflows/authoritative-live-proof.yml`) that is the only workflow allowed to run the S01 live package-manager proof directly.
  - A trusted-event verification lane (`.github/workflows/authoritative-verification.yml`) for same-repo PRs, `main` pushes, manual dispatches, and weekly drift checks with explicit fork skip behavior.
  - A tag-release contract in `release.yml` that blocks `Create Release` on the same authoritative live proof and limits `contents: write` to the release job.
  - A repo-local workflow verifier (`bash scripts/verify-m034-s02-workflows.sh`) that mechanically rejects drift across the reusable, caller, and release workflow contract before CI runs.
requires:
  - slice: S01
    provides: The canonical live proof command (`bash scripts/verify-m034-s01.sh`) plus the real registry/package-manager truth surface that this slice promotes into CI and tag-release gating.
affects:
  - S03
  - S04
  - S05
key_files:
  - .github/workflows/authoritative-live-proof.yml
  - .github/workflows/authoritative-verification.yml
  - .github/workflows/release.yml
  - scripts/verify-m034-s02-workflows.sh
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - Keep `.github/workflows/authoritative-live-proof.yml` as the only workflow file allowed to run `bash scripts/verify-m034-s01.sh` directly; every other CI/release lane must call it instead of copying proof logic.
  - Keep the secret-bearing caller lane fail-closed by running on non-PR events and same-repo `pull_request` heads only, explicitly skipping fork PR live-proof runs and avoiding `pull_request_target`.
  - Keep `release.yml` workflow-wide permissions read-only, grant `contents: write` only to `Create Release`, and require a tag-only reusable authoritative-proof job before release publication.
patterns_established:
  - Use one reusable workflow as the single owner of a live proof script, and force downstream lanes to call it instead of duplicating publish/install assertions in YAML.
  - For secret-bearing PR verification, fail closed with `pull_request` plus an explicit same-repo guard; do not upgrade untrusted checked-out code to `pull_request_target` just to get secrets.
  - Back GitHub Actions policy with a repo-local contract verifier that parses workflow YAML mechanically, checks exact trigger/secret/permission/needs invariants, and leaves per-phase logs under a deterministic `.tmp/` artifact root.
  - Default workflows to read-only permissions and scope write access down to the one job that actually publishes or mutates release state.
observability_surfaces:
  - Repo-local verifier logs under `.tmp/m034-s02/verify/`: `reusable.log`, `caller.log`, `release.log`, and `full-contract.log`, which identify the first drifting phase and failing contract sweep.
  - Reusable-workflow failure artifact retention: `.github/workflows/authoritative-live-proof.yml` uploads `.tmp/m034-s01/verify/**` as `authoritative-live-proof-diagnostics` when the live proof step fails, preserving the same postmortem surface S01 established locally.
  - Rollout evidence ledger at `.tmp/m034-s02/verify/t03-evidence.json`, which records both the successful local verifier passes and the current remote-evidence gap (`gh run list` 404 plus the last historical pre-S02 release graph).
drill_down_paths:
  - .gsd/milestones/M034/slices/S02/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S02/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S02/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-26T22:48:47.638Z
blocker_discovered: false
---

# S02: Authoritative CI verification lane

**Codified the S01 live registry verifier as the single CI/release proof lane via reusable, trusted-event, and tag-gated GitHub Actions workflows plus a repo-local contract verifier that keeps secret boundaries, failure artifacts, and release permissions honest.**

## What Happened

S02 turns the S01 live registry proof from a repo-local command into the authoritative CI and release contract. T01 added `.github/workflows/authoritative-live-proof.yml` as the single reusable `workflow_call` owner of `bash scripts/verify-m034-s01.sh`: it stays on Ubuntu x86_64, bootstraps LLVM 21 and Rust explicitly, wires `MESH_PUBLISH_OWNER` / `MESH_PUBLISH_TOKEN` only through workflow-call secrets, and uploads `.tmp/m034-s01/verify/**` as failure diagnostics instead of re-implementing publish/install assertions in YAML.

T02 added `.github/workflows/authoritative-verification.yml` as the named trusted-event caller lane. That workflow now reruns the same reusable proof on same-repo pull requests, `main` pushes, `workflow_dispatch`, and one bounded weekly schedule; it keeps workflow permissions read-only, serializes same-ref runs with explicit concurrency, and fail-closes for fork PRs by skipping the secret-bearing proof instead of widening the trust boundary with `pull_request_target`.

T03 wired `release.yml` into the same proof surface. Tag runs now call the reusable authoritative proof before `Create Release`, workflow-wide permissions stay read-only, and `contents: write` is scoped down to the actual publish job. In parallel, `scripts/verify-m034-s02-workflows.sh` grew from a task-local check into the slice-wide contract verifier: `reusable`, `caller`, `release`, and `all` modes mechanically reject drift in triggers, secret mapping, toolchain bootstrap, failure-artifact retention, concurrency, tag gating, and permission hardening across all three workflow files.

The slice therefore delivers one coherent pattern for downstream release work: the real package-manager proof stays in `scripts/verify-m034-s01.sh`, every GitHub Actions lane reuses it instead of copying it, and repo-local verification fails fast before CI if any workflow drifts away from that contract. The only missing evidence is rollout-specific: GitHub cannot yet show a live `Authoritative verification` or tag-gated `Release` run because these workflow files are not on the remote default branch yet.

## Operational Readiness
- **Health signal:** `bash scripts/verify-m034-s02-workflows.sh` exits 0 and leaves green phase logs under `.tmp/m034-s02/verify/`; once pushed, trusted Actions runs should show an `Authoritative live proof` job that reaches `verify-m034-s01: ok` before any tag release publishes assets.
- **Failure signal:** the local verifier prints `verification drift`, `first failing phase`, and the failing command, while GitHub-side failures should surface either in the reusable proof logs or as a blocked `Create Release` job waiting on `Authoritative live proof`.
- **Recovery procedure:** inspect the relevant phase log in `.tmp/m034-s02/verify/`, repair the exact workflow/script drift, rerun `bash scripts/verify-m034-s02-workflows.sh` and the YAML parse sweep locally, then push and rerun `workflow_dispatch` / tag checks to capture remote evidence.
- **Monitoring gaps:** the workflows are still unshipped on GitHub's default branch, so the slice currently has structural local proof but not yet the first remote `verify-m034-s01: ok` / tag-gate evidence that the plan ultimately wants.

## Verification

Local slice-plan verification passed in full. `bash scripts/verify-m034-s02-workflows.sh` exited 0, and `ruby -e 'require "yaml"; %w[.github/workflows/authoritative-live-proof.yml .github/workflows/authoritative-verification.yml .github/workflows/release.yml].each { |f| YAML.load_file(f) }'` also exited 0. The verifier wrote the expected observability artifacts at `.tmp/m034-s02/verify/reusable.log`, `.tmp/m034-s02/verify/caller.log`, `.tmp/m034-s02/verify/release.log`, and `.tmp/m034-s02/verify/full-contract.log`, confirming that the reusable, caller, and release contracts all stay green together.

For the plan's remote GitHub Actions evidence, I reran the recorded rollout check: `gh run list --workflow authoritative-verification.yml --limit 1` still returns GitHub 404 because the new workflow file is not yet present on the remote default branch. The retained `.tmp/m034-s02/verify/t03-evidence.json` also shows the latest historical tag-side `Release` run still using the legacy graph without an authoritative proof job. That keeps the slice honest: local contract proof passed, but remote trusted-event/tag evidence remains pending the first push of these workflow files rather than being fabricated.

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

Could not satisfy the task-plan's live GitHub Actions acceptance from this local working tree alone because GitHub only resolves workflow files that already exist on the remote branch it is querying. Instead of inventing evidence, the slice preserved the real state by recording the 404 from `gh run list --workflow authoritative-verification.yml --limit 1` and by keeping the historical pre-S02 release graph in `.tmp/m034-s02/verify/t03-evidence.json`. No slice replan was needed.

## Known Limitations

The repo-local verifier proves the CI/release contract structurally, but the first live same-repo `pull_request`/`workflow_dispatch`/`main` run and the first tag-gated `Release` run are still pending the next push to GitHub. Fork PRs intentionally skip the live publish proof and remain on secret-free lanes by design. This slice also does not yet prove installer/release-asset truth or extension publish truth; those remain downstream in S03 and S04.

## Follow-ups

- Push the updated workflow files to the remote default branch and capture the first trusted-event run showing `Authoritative live proof` reaching `verify-m034-s01: ok`.
- Push a `v*` tag after rollout and confirm `Create Release` stays downstream of the authoritative proof job before any publication.
- Reuse this reusable-workflow plus repo-local-contract-verifier pattern in S03/S04/S05 when installer, release-asset, extension, and full public-release lanes are hardened.

## Files Created/Modified

- `.github/workflows/authoritative-live-proof.yml` — Added the reusable `workflow_call` proof lane that bootstraps Linux LLVM/Rust, wires publish secrets explicitly, shells out to `bash scripts/verify-m034-s01.sh`, and uploads failure diagnostics.
- `.github/workflows/authoritative-verification.yml` — Added the trusted-event caller workflow for same-repo PRs, `main` pushes, manual dispatch, and weekly drift checks with read-only permissions, concurrency, and fail-closed fork handling.
- `.github/workflows/release.yml` — Gated tag releases on the reusable authoritative proof and scoped `contents: write` down to the `Create Release` job.
- `scripts/verify-m034-s02-workflows.sh` — Implemented reusable/caller/release/full contract checks that parse Actions YAML and mechanically enforce the slice's CI/release policy.
- `.gsd/DECISIONS.md` — Recorded the slice-level CI/security decisions around reusable proof ownership and release permission hardening.
- `.gsd/KNOWLEDGE.md` — Captured the Ruby Actions-YAML `on:` parsing gotcha and the GitHub CLI default-branch workflow-discovery limitation encountered during rollout evidence checks.
- `.gsd/PROJECT.md` — Refreshed current-state project context so M034 now reflects the local authoritative CI/release workflow contract and the remaining remote-rollout gap.
