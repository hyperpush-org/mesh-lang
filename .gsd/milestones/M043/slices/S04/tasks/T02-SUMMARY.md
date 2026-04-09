---
id: T02
parent: S04
milestone: M043
provides: []
requires: []
affects: []
key_files: ["cluster-proof/README.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "README.md", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M043/slices/S04/tasks/T02-SUMMARY.md"]
key_decisions: ["Matched the proof-surface verifier's literal Fly command encoding, including doubled trailing backslashes in the continued live-mode command block, so the public docs and the rail stay mechanically aligned.", "Kept the repo README and generic distributed guide as routing surfaces that point operator claims at the proof page and runbook instead of duplicating failover wording."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the four task-contract checks after editing the docs. `bash scripts/verify-m043-s04-proof-surface.sh` passed and confirmed the proof page, runbook, distributed guide, repo README, and sidebar wiring all share the M043 wording and command list. `bash scripts/verify-m043-s04-fly.sh --help` passed and kept the public Fly help path aligned with the docs. `npm --prefix website run build` passed, confirming the VitePress docs still build after the wording changes. `bash scripts/verify-m043-s03.sh` passed, confirming the public docs still point at the real same-image failover authority that proves promotion, fenced stale-primary behavior, and the allowed post-rejoin `replication_health` truth."
completed_at: 2026-03-29T12:11:09.777Z
blocker_discovered: false
---

# T02: Reconciled the cluster-proof runbook and distributed docs to the shipped M043 failover/operator contract.

> Reconciled the cluster-proof runbook and distributed docs to the shipped M043 failover/operator contract.

## What Happened
---
id: T02
parent: S04
milestone: M043
key_files:
  - cluster-proof/README.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - README.md
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M043/slices/S04/tasks/T02-SUMMARY.md
key_decisions:
  - Matched the proof-surface verifier's literal Fly command encoding, including doubled trailing backslashes in the continued live-mode command block, so the public docs and the rail stay mechanically aligned.
  - Kept the repo README and generic distributed guide as routing surfaces that point operator claims at the proof page and runbook instead of duplicating failover wording.
duration: ""
verification_result: passed
completed_at: 2026-03-29T12:11:09.778Z
blocker_discovered: false
---

# T02: Reconciled the cluster-proof runbook and distributed docs to the shipped M043 failover/operator contract.

**Reconciled the cluster-proof runbook and distributed docs to the shipped M043 failover/operator contract.**

## What Happened

Rewrote `cluster-proof/README.md` around the M043 failover surface instead of the older continuity-only story: it now names `POST /promote` as the explicit authority boundary, describes runtime-owned `cluster_role` / `promotion_epoch` / `replication_health` truth, keeps the destructive same-image authority on `bash scripts/verify-m043-s03.sh`, and narrows Fly to a read-only evidence rail. Reworked `website/docs/docs/distributed-proof/index.md` to use the same M043 script names, headings, command list, supported topology, and non-goals that `scripts/verify-m043-s04-proof-surface.sh` enforces. Tightened `website/docs/docs/distributed/index.md` and the repo `README.md` into routing surfaces that point operator claims at the proof page and runbook instead of restating a weaker or stale contract. The only unexpected implementation detail was mechanical rather than architectural: the proof-surface verifier compares the continued Fly command block as exact markdown text, including doubled trailing backslashes. The first verification pass failed at `command-list`; after matching the docs to the verifier's literal escaped form, the proof-surface rail passed cleanly. I recorded that gotcha in `.gsd/KNOWLEDGE.md` so later agents do not spend time rediscovering it.

## Verification

Ran the four task-contract checks after editing the docs. `bash scripts/verify-m043-s04-proof-surface.sh` passed and confirmed the proof page, runbook, distributed guide, repo README, and sidebar wiring all share the M043 wording and command list. `bash scripts/verify-m043-s04-fly.sh --help` passed and kept the public Fly help path aligned with the docs. `npm --prefix website run build` passed, confirming the VitePress docs still build after the wording changes. `bash scripts/verify-m043-s03.sh` passed, confirming the public docs still point at the real same-image failover authority that proves promotion, fenced stale-primary behavior, and the allowed post-rejoin `replication_health` truth.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m043-s04-proof-surface.sh` | 0 | ✅ pass | 7551ms |
| 2 | `bash scripts/verify-m043-s04-fly.sh --help` | 0 | ✅ pass | 22ms |
| 3 | `npm --prefix website run build` | 0 | ✅ pass | 23621ms |
| 4 | `bash scripts/verify-m043-s03.sh` | 0 | ✅ pass | 311549ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M043/slices/S04/tasks/T02-SUMMARY.md`


## Deviations
None.

## Known Issues
None.
