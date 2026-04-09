---
id: T04
parent: S04
milestone: M042
provides: []
requires: []
affects: []
key_files: ["cluster-proof/README.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "README.md", "scripts/verify-m042-s04-proof-surface.sh", "scripts/verify-m042-s04-fly.sh", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Public distributed continuity docs now treat `bash scripts/verify-m042-s03.sh` as the local authority and keep the Fly lane scoped to read-only sanity only.", "Keep the distributed proof page and runbook on one mechanically checked command list, then reject stale M039 wording and stronger delivery/process-migration claims in the proof-surface verifier."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran `bash scripts/verify-m042-s04-proof-surface.sh`, `bash scripts/verify-m042-s04-fly.sh --help`, and `npm --prefix website run build`. All three checks passed, so the docs, help contract, and VitePress site build are aligned with the runtime-owned continuity story."
completed_at: 2026-03-29T02:35:48.670Z
blocker_discovered: false
---

# T04: Rewrote the distributed proof runbook and public docs around the runtime-owned continuity contract, added the M042 proof-surface verifier, and aligned the Fly help rail with the same local-authority story.

> Rewrote the distributed proof runbook and public docs around the runtime-owned continuity contract, added the M042 proof-surface verifier, and aligned the Fly help rail with the same local-authority story.

## What Happened
---
id: T04
parent: S04
milestone: M042
key_files:
  - cluster-proof/README.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - README.md
  - scripts/verify-m042-s04-proof-surface.sh
  - scripts/verify-m042-s04-fly.sh
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Public distributed continuity docs now treat `bash scripts/verify-m042-s03.sh` as the local authority and keep the Fly lane scoped to read-only sanity only.
  - Keep the distributed proof page and runbook on one mechanically checked command list, then reject stale M039 wording and stronger delivery/process-migration claims in the proof-surface verifier.
duration: ""
verification_result: passed
completed_at: 2026-03-29T02:35:48.671Z
blocker_discovered: false
---

# T04: Rewrote the distributed proof runbook and public docs around the runtime-owned continuity contract, added the M042 proof-surface verifier, and aligned the Fly help rail with the same local-authority story.

**Rewrote the distributed proof runbook and public docs around the runtime-owned continuity contract, added the M042 proof-surface verifier, and aligned the Fly help rail with the same local-authority story.**

## What Happened

Replaced the stale M039-facing distributed proof wording with the current M042 truth across `cluster-proof/README.md`, `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, and `README.md`. The runbook and proof page now describe `cluster-proof` as a thin consumer over `Continuity.submit`, `Continuity.status`, and `Continuity.mark_completed`, distinguish the legacy `GET /work` probe from the keyed `POST /work` / `GET /work/:request_key` surfaces, and document `request_key` as the idempotency key plus `attempt_id` as the runtime-issued retry fence/token. Added `scripts/verify-m042-s04-proof-surface.sh` to mechanically keep the proof page, runbook, guide, README, and sidebar entry aligned while rejecting stale M039 command names and stronger delivery/process-migration wording. Updated `scripts/verify-m042-s04-fly.sh --help` so the read-only Fly lane now points at `bash scripts/verify-m042-s03.sh` as the current local authority instead of the older packaged-wrapper wording.

## Verification

Ran `bash scripts/verify-m042-s04-proof-surface.sh`, `bash scripts/verify-m042-s04-fly.sh --help`, and `npm --prefix website run build`. All three checks passed, so the docs, help contract, and VitePress site build are aligned with the runtime-owned continuity story.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m042-s04-proof-surface.sh` | 0 | ✅ pass | 1682ms |
| 2 | `bash scripts/verify-m042-s04-fly.sh --help` | 0 | ✅ pass | 79ms |
| 3 | `npm --prefix website run build` | 0 | ✅ pass | 18327ms |


## Deviations

None.

## Known Issues

`npm --prefix website run build` still emits the existing VitePress chunk-size warning for bundles larger than 500 kB, but the build completes successfully.

## Files Created/Modified

- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `scripts/verify-m042-s04-proof-surface.sh`
- `scripts/verify-m042-s04-fly.sh`
- `.gsd/KNOWLEDGE.md`


## Deviations
None.

## Known Issues
`npm --prefix website run build` still emits the existing VitePress chunk-size warning for bundles larger than 500 kB, but the build completes successfully.
