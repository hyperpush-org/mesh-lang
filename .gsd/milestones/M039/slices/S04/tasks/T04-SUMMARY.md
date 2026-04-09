---
id: T04
parent: S04
milestone: M039
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "website/docs/.vitepress/config.mts", "README.md", "scripts/verify-m039-s04-proof-surface.sh", "cluster-proof/README.md", ".gsd/DECISIONS.md"]
key_decisions: ["Publish distributed operator claims on a dedicated `Distributed Proof` page backed by `cluster-proof/README.md` and `scripts/verify-m039-s04-proof-surface.sh`, while keeping `Distributed Actors` as the primitive/tutorial guide."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the task’s named verification commands: `npm --prefix website run build` and `bash scripts/verify-m039-s04-proof-surface.sh`. There is no separate `## Verification` block in `S04-PLAN.md`, so the task-level gates remained the slice-level verification surface for this closeout task."
completed_at: 2026-03-28T14:18:24.340Z
blocker_discovered: false
---

# T04: Published the Distributed Proof page, rerouted operator-facing docs to it, and added a fail-closed proof-surface verifier.

> Published the Distributed Proof page, rerouted operator-facing docs to it, and added a fail-closed proof-surface verifier.

## What Happened
---
id: T04
parent: S04
milestone: M039
key_files:
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - website/docs/.vitepress/config.mts
  - README.md
  - scripts/verify-m039-s04-proof-surface.sh
  - cluster-proof/README.md
  - .gsd/DECISIONS.md
key_decisions:
  - Publish distributed operator claims on a dedicated `Distributed Proof` page backed by `cluster-proof/README.md` and `scripts/verify-m039-s04-proof-surface.sh`, while keeping `Distributed Actors` as the primitive/tutorial guide.
duration: ""
verification_result: passed
completed_at: 2026-03-28T14:18:24.341Z
blocker_discovered: false
---

# T04: Published the Distributed Proof page, rerouted operator-facing docs to it, and added a fail-closed proof-surface verifier.

**Published the Distributed Proof page, rerouted operator-facing docs to it, and added a fail-closed proof-surface verifier.**

## What Happened

Added `website/docs/docs/distributed-proof/index.md` as the public operator-proof entrypoint for `cluster-proof/`, updated the generic distributed guide, README, and VitePress sidebar to route operator claims to that page and the linked runbook, and implemented `scripts/verify-m039-s04-proof-surface.sh` to fail closed on link drift, sidebar drift, and command-list drift. The verifier exposed two real inconsistencies during execution — the proof page did not yet use the exact repo-root Docker-build contract wording, and `cluster-proof/README.md` did not yet name the new docs-truth verifier — so both surfaces were aligned and the gate was rerun successfully.

## Verification

Passed the task’s named verification commands: `npm --prefix website run build` and `bash scripts/verify-m039-s04-proof-surface.sh`. There is no separate `## Verification` block in `S04-PLAN.md`, so the task-level gates remained the slice-level verification surface for this closeout task.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix website run build` | 0 | ✅ pass | 14231ms |
| 2 | `bash scripts/verify-m039-s04-proof-surface.sh` | 0 | ✅ pass | 672ms |


## Deviations

Added the docs-truth verifier command to `cluster-proof/README.md` even though the task plan listed that runbook as an input rather than an output. That change was necessary to keep the public proof page and deepest runbook on the same canonical command list.

## Known Issues

`npm --prefix website run build` still emits the pre-existing VitePress chunk-size warning about bundles larger than 500 kB after minification. The build remains green and this task did not change chunking behavior.

## Files Created/Modified

- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `website/docs/.vitepress/config.mts`
- `README.md`
- `scripts/verify-m039-s04-proof-surface.sh`
- `cluster-proof/README.md`
- `.gsd/DECISIONS.md`


## Deviations
Added the docs-truth verifier command to `cluster-proof/README.md` even though the task plan listed that runbook as an input rather than an output. That change was necessary to keep the public proof page and deepest runbook on the same canonical command list.

## Known Issues
`npm --prefix website run build` still emits the pre-existing VitePress chunk-size warning about bundles larger than 500 kB after minification. The build remains green and this task did not change chunking behavior.
