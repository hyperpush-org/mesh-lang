---
id: T02
parent: S02
milestone: M034
provides: []
requires: []
affects: []
key_files: [".github/workflows/authoritative-verification.yml", "scripts/verify-m034-s02-workflows.sh", ".gsd/milestones/M034/slices/S02/tasks/T02-SUMMARY.md"]
key_decisions: ["Keep the caller workflow secret-bearing only for non-PR events and same-repo pull_request heads by guarding the reusable workflow call with github.event.pull_request.head.repo.full_name == github.repository.", "Keep the full-slice `all` verifier mode intentionally red until T03 wires `release.yml` into the same reusable proof contract, so the slice does not claim completion early."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `bash -n scripts/verify-m034-s02-workflows.sh`, `bash scripts/verify-m034-s02-workflows.sh reusable`, `bash scripts/verify-m034-s02-workflows.sh caller`, and Ruby YAML parses for both `.github/workflows/authoritative-live-proof.yml` and `.github/workflows/authoritative-verification.yml`; all passed. Also ran `bash scripts/verify-m034-s02-workflows.sh all` to confirm the current full-slice preflight still fails only at the expected T03 release-gating boundary."
completed_at: 2026-03-26T22:31:35.192Z
blocker_discovered: false
---

# T02: Added the trusted-event authoritative verification workflow that reuses the live proof on same-repo PRs, main pushes, manual runs, and weekly drift checks.

> Added the trusted-event authoritative verification workflow that reuses the live proof on same-repo PRs, main pushes, manual runs, and weekly drift checks.

## What Happened
---
id: T02
parent: S02
milestone: M034
key_files:
  - .github/workflows/authoritative-verification.yml
  - scripts/verify-m034-s02-workflows.sh
  - .gsd/milestones/M034/slices/S02/tasks/T02-SUMMARY.md
key_decisions:
  - Keep the caller workflow secret-bearing only for non-PR events and same-repo pull_request heads by guarding the reusable workflow call with github.event.pull_request.head.repo.full_name == github.repository.
  - Keep the full-slice `all` verifier mode intentionally red until T03 wires `release.yml` into the same reusable proof contract, so the slice does not claim completion early.
duration: ""
verification_result: mixed
completed_at: 2026-03-26T22:31:35.192Z
blocker_discovered: false
---

# T02: Added the trusted-event authoritative verification workflow that reuses the live proof on same-repo PRs, main pushes, manual runs, and weekly drift checks.

**Added the trusted-event authoritative verification workflow that reuses the live proof on same-repo PRs, main pushes, manual runs, and weekly drift checks.**

## What Happened

Added `.github/workflows/authoritative-verification.yml` as the named trusted-event caller lane for the reusable authoritative live proof. The workflow now triggers on `pull_request`, `push` to `main`, `workflow_dispatch`, and one weekly `schedule`, keeps workflow permissions read-only, and serializes same-ref runs with a workflow-plus-ref concurrency group. The lone caller job reuses `./.github/workflows/authoritative-live-proof.yml`, maps `MESH_PUBLISH_OWNER` and `MESH_PUBLISH_TOKEN` explicitly, and fail-closes for fork PRs by only running when the event is not `pull_request` or the PR head repository matches `github.repository`. I also extended `scripts/verify-m034-s02-workflows.sh` with a `caller` mode that rejects drift in the trigger set, weekly schedule presence, read-only permissions, concurrency, reusable-workflow path, explicit secret mapping, fork trust guard, and the absence of `pull_request_target`, while keeping the full-slice `all` mode intentionally red until T03 wires `release.yml` into the same reusable proof contract.

## Verification

Ran `bash -n scripts/verify-m034-s02-workflows.sh`, `bash scripts/verify-m034-s02-workflows.sh reusable`, `bash scripts/verify-m034-s02-workflows.sh caller`, and Ruby YAML parses for both `.github/workflows/authoritative-live-proof.yml` and `.github/workflows/authoritative-verification.yml`; all passed. Also ran `bash scripts/verify-m034-s02-workflows.sh all` to confirm the current full-slice preflight still fails only at the expected T03 release-gating boundary.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m034-s02-workflows.sh` | 0 | ✅ pass | 12ms |
| 2 | `bash scripts/verify-m034-s02-workflows.sh reusable` | 0 | ✅ pass | 117ms |
| 3 | `bash scripts/verify-m034-s02-workflows.sh caller` | 0 | ✅ pass | 106ms |
| 4 | `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/authoritative-live-proof.yml")'` | 0 | ✅ pass | 97ms |
| 5 | `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/authoritative-verification.yml")'` | 0 | ✅ pass | 95ms |
| 6 | `bash scripts/verify-m034-s02-workflows.sh all` | 1 | ❌ fail | 213ms |


## Deviations

Kept `bash scripts/verify-m034-s02-workflows.sh all` intentionally red after landing the caller mode instead of broadening this task into T03. This preserves truthful slice-level signaling until the release workflow is actually gated on the shared proof.

## Known Issues

`bash scripts/verify-m034-s02-workflows.sh all` still fails intentionally because `.github/workflows/release.yml` has not been wired to the reusable proof or permission-hardening checks yet. That remaining full-slice contract is the explicit scope of T03.

## Files Created/Modified

- `.github/workflows/authoritative-verification.yml`
- `scripts/verify-m034-s02-workflows.sh`
- `.gsd/milestones/M034/slices/S02/tasks/T02-SUMMARY.md`


## Deviations
Kept `bash scripts/verify-m034-s02-workflows.sh all` intentionally red after landing the caller mode instead of broadening this task into T03. This preserves truthful slice-level signaling until the release workflow is actually gated on the shared proof.

## Known Issues
`bash scripts/verify-m034-s02-workflows.sh all` still fails intentionally because `.github/workflows/release.yml` has not been wired to the reusable proof or permission-hardening checks yet. That remaining full-slice contract is the explicit scope of T03.
