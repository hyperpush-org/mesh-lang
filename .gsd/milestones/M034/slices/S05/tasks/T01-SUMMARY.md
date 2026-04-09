---
id: T01
parent: S05
milestone: M034
provides: []
requires: []
affects: []
key_files: [".github/workflows/deploy.yml", ".github/workflows/deploy-services.yml", "scripts/verify-m034-s05-workflows.sh", ".gsd/KNOWLEDGE.md", ".gsd/milestones/M034/slices/S05/tasks/T01-SUMMARY.md"]
key_decisions: ["Verified rendered VitePress HTML via Python text extraction instead of raw grep because command text is split across markup spans.", "Kept deploy workflow verification local-first with parser-backed YAML guards and named `.tmp/m034-s05/workflows/*.log` artifacts before relying on hosted runs."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Task-level verification passed with `bash scripts/verify-m034-s05-workflows.sh`, YAML parse validation, explicit workflow-content grep checks, a real `npm --prefix website run build`, a local replay of the new `deploy.yml` docs-contract step, and confirmation that `.tmp/m034-s05/workflows/phase-report.txt` plus per-phase logs were emitted. Slice-level verification is partially complete as expected for T01: the workflow gate now passes, while the canonical assembled verifier and remote evidence surfaces are owned by later tasks."
completed_at: 2026-03-27T02:27:57.582Z
blocker_discovered: false
---

# T01: Added an S05 deploy-workflow verifier and exact Pages/Fly public-surface checks for docs, installers, and registry proof URLs.

> Added an S05 deploy-workflow verifier and exact Pages/Fly public-surface checks for docs, installers, and registry proof URLs.

## What Happened
---
id: T01
parent: S05
milestone: M034
key_files:
  - .github/workflows/deploy.yml
  - .github/workflows/deploy-services.yml
  - scripts/verify-m034-s05-workflows.sh
  - .gsd/KNOWLEDGE.md
  - .gsd/milestones/M034/slices/S05/tasks/T01-SUMMARY.md
key_decisions:
  - Verified rendered VitePress HTML via Python text extraction instead of raw grep because command text is split across markup spans.
  - Kept deploy workflow verification local-first with parser-backed YAML guards and named `.tmp/m034-s05/workflows/*.log` artifacts before relying on hosted runs.
duration: ""
verification_result: passed
completed_at: 2026-03-27T02:27:57.583Z
blocker_discovered: false
---

# T01: Added an S05 deploy-workflow verifier and exact Pages/Fly public-surface checks for docs, installers, and registry proof URLs.

**Added an S05 deploy-workflow verifier and exact Pages/Fly public-surface checks for docs, installers, and registry proof URLs.**

## What Happened

Added the S05-owned parser-backed deploy workflow verifier, tightened the Pages workflow so the built VitePress artifact must contain and preserve the exact public installer/docs surfaces, and tightened the Fly deploy workflow so post-deploy health checks prove the exact registry search, package detail, installer, and docs URLs instead of homepage-only reachability. During verification I found raw grep over rendered VitePress HTML was brittle because command text is split across markup spans, so I replaced that check with Python HTML text extraction and updated the workflow verifier accordingly. I also recorded that gotcha in `.gsd/KNOWLEDGE.md` for downstream tasks.

## Verification

Task-level verification passed with `bash scripts/verify-m034-s05-workflows.sh`, YAML parse validation, explicit workflow-content grep checks, a real `npm --prefix website run build`, a local replay of the new `deploy.yml` docs-contract step, and confirmation that `.tmp/m034-s05/workflows/phase-report.txt` plus per-phase logs were emitted. Slice-level verification is partially complete as expected for T01: the workflow gate now passes, while the canonical assembled verifier and remote evidence surfaces are owned by later tasks.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-m034-s05-workflows.sh` | 0 | ✅ pass | 1198ms |
| 2 | `ruby -e 'require "yaml"; [".github/workflows/deploy.yml", ".github/workflows/deploy-services.yml"].each { |path| YAML.load_file(path) }'` | 0 | ✅ pass | 380ms |
| 3 | `rg -n 'install\.sh|install\.ps1|packages/snowdamiz/mesh-registry-proof|api/v1/packages\?search=snowdamiz%2Fmesh-registry-proof' .github/workflows/deploy.yml .github/workflows/deploy-services.yml` | 0 | ✅ pass | 35ms |
| 4 | `npm --prefix website run build` | 0 | ✅ pass | 32440ms |
| 5 | `bash -c <deploy.yml verify public docs contract step>` | 0 | ✅ pass | 337ms |
| 6 | `test -f .tmp/m034-s05/workflows/phase-report.txt && find .tmp/m034-s05/workflows -maxdepth 1 -type f | sort` | 0 | ✅ pass | 26ms |


## Deviations

None.

## Known Issues

T02 still needs to reconcile the local public docs/installers/extension metadata with the final release claim, so this task intentionally does not close content drift such as stale public wording or repo-slug references. The assembled slice verifier `scripts/verify-m034-s05.sh` and remote-run evidence files are still future-task work.

## Files Created/Modified

- `.github/workflows/deploy.yml`
- `.github/workflows/deploy-services.yml`
- `scripts/verify-m034-s05-workflows.sh`
- `.gsd/KNOWLEDGE.md`
- `.gsd/milestones/M034/slices/S05/tasks/T01-SUMMARY.md`


## Deviations
None.

## Known Issues
T02 still needs to reconcile the local public docs/installers/extension metadata with the final release claim, so this task intentionally does not close content drift such as stale public wording or repo-slug references. The assembled slice verifier `scripts/verify-m034-s05.sh` and remote-run evidence files are still future-task work.
