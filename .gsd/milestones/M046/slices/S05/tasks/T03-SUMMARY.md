---
id: T03
parent: S05
milestone: M046
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/getting-started/clustered-example/index.md", "website/docs/docs/tooling/index.md", "website/docs/docs/distributed-proof/index.md", "website/docs/docs/distributed/index.md", "README.md", "tiny-cluster/README.md", "cluster-proof/README.md", ".gsd/milestones/M046/slices/S05/tasks/T03-SUMMARY.md"]
key_decisions: ["Used the generated scaffold README contract as the wording baseline so public docs and package READMEs teach the same source-owned clustered(work) and CLI-only inspection flow.", "Repointed docs to scripts/verify-m046-s05.sh as the authoritative equal-surface rail while labeling scripts/verify-m045-s05.sh as historical to remove the older wrapper-first story."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Ran the task-plan verification command exactly as written: npm --prefix website run build && ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md. The VitePress build completed successfully and the guarded docs surfaces no longer contain the deleted routeful strings. Added targeted content guards to prove the docs now mention scripts/verify-m046-s05.sh plus historical scripts/verify-m045-s05.sh, name the equal canonical surfaces, and keep the package READMEs aligned on status → continuity list → continuity record → diagnostics while staying route-free."
completed_at: 2026-04-01T01:08:24.741Z
blocker_discovered: false
---

# T03: Aligned the scaffold docs, tiny-cluster, and cluster-proof around one route-free clustered-work story and repointed public verifier references to the S05 equal-surface rail.

> Aligned the scaffold docs, tiny-cluster, and cluster-proof around one route-free clustered-work story and repointed public verifier references to the S05 equal-surface rail.

## What Happened
---
id: T03
parent: S05
milestone: M046
key_files:
  - website/docs/docs/getting-started/clustered-example/index.md
  - website/docs/docs/tooling/index.md
  - website/docs/docs/distributed-proof/index.md
  - website/docs/docs/distributed/index.md
  - README.md
  - tiny-cluster/README.md
  - cluster-proof/README.md
  - .gsd/milestones/M046/slices/S05/tasks/T03-SUMMARY.md
key_decisions:
  - Used the generated scaffold README contract as the wording baseline so public docs and package READMEs teach the same source-owned clustered(work) and CLI-only inspection flow.
  - Repointed docs to scripts/verify-m046-s05.sh as the authoritative equal-surface rail while labeling scripts/verify-m045-s05.sh as historical to remove the older wrapper-first story.
duration: ""
verification_result: passed
completed_at: 2026-04-01T01:08:24.743Z
blocker_discovered: false
---

# T03: Aligned the scaffold docs, tiny-cluster, and cluster-proof around one route-free clustered-work story and repointed public verifier references to the S05 equal-surface rail.

**Aligned the scaffold docs, tiny-cluster, and cluster-proof around one route-free clustered-work story and repointed public verifier references to the S05 equal-surface rail.**

## What Happened

Rewrote the clustered getting-started and distributed-proof pages so they no longer teach manifest-owned cluster declarations, app-owned proof/status routes, or delay-edit walkthroughs. The public story now treats the generated scaffold, tiny-cluster, and cluster-proof as three equal canonical surfaces that all share the same contract: package-only mesh.toml, source-owned clustered(work), startup through Node.start_from_env(), automatic startup work, and CLI-only inspection. Updated the tooling page, distributed guide intro, and repo README.md to carry the same verifier map and explicitly frame scripts/verify-m046-s05.sh as the authoritative closeout rail while keeping scripts/verify-m045-s05.sh clearly historical. Aligned tiny-cluster/README.md and cluster-proof/README.md on the same operator runbook sequence: cluster status, continuity list, continuity record, then diagnostics.

## Verification

Ran the task-plan verification command exactly as written: npm --prefix website run build && ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md. The VitePress build completed successfully and the guarded docs surfaces no longer contain the deleted routeful strings. Added targeted content guards to prove the docs now mention scripts/verify-m046-s05.sh plus historical scripts/verify-m045-s05.sh, name the equal canonical surfaces, and keep the package READMEs aligned on status → continuity list → continuity record → diagnostics while staying route-free.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `npm --prefix website run build && ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md` | 0 | ✅ pass | 38400ms |
| 2 | `rg -q 'scripts/verify-m046-s05\.sh' website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md && rg -q 'scripts/verify-m045-s05\.sh' website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md && rg -q 'three equal canonical|equal canonical' website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/distributed-proof/index.md README.md tiny-cluster/README.md cluster-proof/README.md && rg -q 'meshc cluster status <node-name@host:port> --json' tiny-cluster/README.md cluster-proof/README.md && rg -q 'meshc cluster continuity <node-name@host:port> --json' tiny-cluster/README.md cluster-proof/README.md && rg -q 'meshc cluster continuity <node-name@host:port> <request-key> --json' tiny-cluster/README.md cluster-proof/README.md && rg -q 'meshc cluster diagnostics <node-name@host:port> --json' tiny-cluster/README.md cluster-proof/README.md` | 0 | ✅ pass | 190ms |
| 3 | `! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work|Timer\.sleep|request-key-only" tiny-cluster/README.md cluster-proof/README.md` | 0 | ✅ pass | 30ms |


## Deviations

None.

## Known Issues

The docs now identify scripts/verify-m046-s05.sh as the authoritative equal-surface closeout rail, but that script still lands in T04. Until then, scripts/verify-m045-s05.sh remains the only runnable historical wrapper present on disk.

## Files Created/Modified

- `website/docs/docs/getting-started/clustered-example/index.md`
- `website/docs/docs/tooling/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `tiny-cluster/README.md`
- `cluster-proof/README.md`
- `.gsd/milestones/M046/slices/S05/tasks/T03-SUMMARY.md`


## Deviations
None.

## Known Issues
The docs now identify scripts/verify-m046-s05.sh as the authoritative equal-surface closeout rail, but that script still lands in T04. Until then, scripts/verify-m045-s05.sh remains the only runnable historical wrapper present on disk.
