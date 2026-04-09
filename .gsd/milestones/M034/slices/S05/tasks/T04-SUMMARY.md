---
id: T04
parent: S05
milestone: M034
provides: []
requires: []
affects: []
key_files: ["scripts/verify-m034-s05.sh", "README.md", "website/docs/docs/tooling/index.md", "scripts/tests/verify-m034-s05-contract.test.mjs", ".tmp/m034-s05/verify/candidate-tags.json", ".tmp/m034-s05/verify/remote-runs.json", ".gsd/milestones/M034/slices/S05/tasks/T04-SUMMARY.md"]
key_decisions: ["Query hosted workflow evidence by the real candidate identity (`v<Cargo version>` for binaries, `ext-v<extension version>` for the extension) and persist both exact `gh run list/view` logs and an aggregate `remote-runs.json` report so rollout gaps stay auditable.", "Run the new remote-evidence phase before the public-HTTP/S01 phases so `candidate-tags.json` and `remote-runs.json` are still produced even when hosted rollout drift keeps the assembled verifier red."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the updated verifier in layers. `bash -n scripts/verify-m034-s05.sh` passed. `node --test scripts/tests/verify-m034-s05-contract.test.mjs` passed. The task-contract grep over `README.md` and `website/docs/docs/tooling/index.md` passed and confirmed the exact command / workflow / candidate-tag strings are present. The full assembled verification command `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` ran through prereqs, candidate-tag derivation, the existing S05/S02/S03/S04 gates, then failed honestly at `remote-evidence`, which still produced both `.tmp/m034-s05/verify/candidate-tags.json` and `.tmp/m034-s05/verify/remote-runs.json` as required."
completed_at: 2026-03-27T03:16:11.706Z
blocker_discovered: false
---

# T04: Added candidate-tag derivation plus hosted-run evidence to the S05 release verifier and documented the canonical public release runbook.

> Added candidate-tag derivation plus hosted-run evidence to the S05 release verifier and documented the canonical public release runbook.

## What Happened
---
id: T04
parent: S05
milestone: M034
key_files:
  - scripts/verify-m034-s05.sh
  - README.md
  - website/docs/docs/tooling/index.md
  - scripts/tests/verify-m034-s05-contract.test.mjs
  - .tmp/m034-s05/verify/candidate-tags.json
  - .tmp/m034-s05/verify/remote-runs.json
  - .gsd/milestones/M034/slices/S05/tasks/T04-SUMMARY.md
key_decisions:
  - Query hosted workflow evidence by the real candidate identity (`v<Cargo version>` for binaries, `ext-v<extension version>` for the extension) and persist both exact `gh run list/view` logs and an aggregate `remote-runs.json` report so rollout gaps stay auditable.
  - Run the new remote-evidence phase before the public-HTTP/S01 phases so `candidate-tags.json` and `remote-runs.json` are still produced even when hosted rollout drift keeps the assembled verifier red.
duration: ""
verification_result: mixed
completed_at: 2026-03-27T03:16:11.708Z
blocker_discovered: false
---

# T04: Added candidate-tag derivation plus hosted-run evidence to the S05 release verifier and documented the canonical public release runbook.

**Added candidate-tag derivation plus hosted-run evidence to the S05 release verifier and documented the canonical public release runbook.**

## What Happened

Extended `scripts/verify-m034-s05.sh` with a `candidate-tags` phase that derives `v<Cargo version>` from `compiler/meshc/Cargo.toml` / `compiler/meshpkg/Cargo.toml` and `ext-v<extension version>` from `tools/editors/vscode-mesh/package.json`, then persists that result to `.tmp/m034-s05/verify/candidate-tags.json`. Added a `remote-evidence` phase that records exact `gh run list` / `gh run view` outputs per required workflow under `.tmp/m034-s05/verify/remote-*.{stdout,stderr,log}` and writes the aggregate hosted-rollout report to `.tmp/m034-s05/verify/remote-runs.json`. Updated `README.md` and `website/docs/docs/tooling/index.md` so they now publish the same assembled proof command, split binary-vs-extension candidate tag policy, required hosted workflow set, public URLs, and proof-artifact paths that the verifier checks mechanically. Added `scripts/tests/verify-m034-s05-contract.test.mjs` as a fast contract test for the new runbook wording and tag policy. The full assembled verifier now fails at the new remote-evidence boundary instead of hand-waving rollout state, and the current hosted failures are captured in the artifact bundle.

## Verification

Verified the updated verifier in layers. `bash -n scripts/verify-m034-s05.sh` passed. `node --test scripts/tests/verify-m034-s05-contract.test.mjs` passed. The task-contract grep over `README.md` and `website/docs/docs/tooling/index.md` passed and confirmed the exact command / workflow / candidate-tag strings are present. The full assembled verification command `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` ran through prereqs, candidate-tag derivation, the existing S05/S02/S03/S04 gates, then failed honestly at `remote-evidence`, which still produced both `.tmp/m034-s05/verify/candidate-tags.json` and `.tmp/m034-s05/verify/remote-runs.json` as required.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash -n scripts/verify-m034-s05.sh` | 0 | ✅ pass | 17ms |
| 2 | `node --test scripts/tests/verify-m034-s05-contract.test.mjs` | 0 | ✅ pass | 674ms |
| 3 | `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` | 1 | ❌ fail | 85900ms |
| 4 | `test -f .tmp/m034-s05/verify/remote-runs.json` | 0 | ✅ pass | 25ms |
| 5 | `test -f .tmp/m034-s05/verify/candidate-tags.json` | 0 | ✅ pass | 26ms |
| 6 | `rg -n 'verify-m034-s05|v<Cargo version>|ext-v<extension version>|deploy\.yml|deploy-services\.yml|authoritative-verification\.yml|extension-release-proof\.yml|publish-extension\.yml' README.md website/docs/docs/tooling/index.md` | 0 | ✅ pass | 58ms |


## Deviations

Added a fast repo-local node:test contract (`scripts/tests/verify-m034-s05-contract.test.mjs`) to pin the new runbook wording and candidate-tag policy. Also placed `remote-evidence` before `public-http` / `s01-live-proof` so the required `candidate-tags.json` and `remote-runs.json` artifacts are still emitted when the hosted rollout surface is red.

## Known Issues

The slice is still not fully green remotely. Hosted rollout evidence shows these unresolved gaps: `deploy.yml` on the remote default branch still lacks the `Verify public docs contract` step; `deploy-services.yml` has no hosted `push` run for binary candidate tag `v0.1.0`; `authoritative-verification.yml` is not present on the remote default branch queried by `gh run list`; `release.yml` has no hosted `push` run for binary candidate tag `v0.1.0`; `extension-release-proof.yml` is not present on the remote default branch queried by `gh run list`; and `publish-extension.yml` has no hosted `push` run for extension candidate tag `ext-v0.3.0`. Because `remote-evidence` fails first, the assembled S05 verifier does not currently reach `public-http` or the final S01 live-proof phase in this run.

## Files Created/Modified

- `scripts/verify-m034-s05.sh`
- `README.md`
- `website/docs/docs/tooling/index.md`
- `scripts/tests/verify-m034-s05-contract.test.mjs`
- `.tmp/m034-s05/verify/candidate-tags.json`
- `.tmp/m034-s05/verify/remote-runs.json`
- `.gsd/milestones/M034/slices/S05/tasks/T04-SUMMARY.md`


## Deviations
Added a fast repo-local node:test contract (`scripts/tests/verify-m034-s05-contract.test.mjs`) to pin the new runbook wording and candidate-tag policy. Also placed `remote-evidence` before `public-http` / `s01-live-proof` so the required `candidate-tags.json` and `remote-runs.json` artifacts are still emitted when the hosted rollout surface is red.

## Known Issues
The slice is still not fully green remotely. Hosted rollout evidence shows these unresolved gaps: `deploy.yml` on the remote default branch still lacks the `Verify public docs contract` step; `deploy-services.yml` has no hosted `push` run for binary candidate tag `v0.1.0`; `authoritative-verification.yml` is not present on the remote default branch queried by `gh run list`; `release.yml` has no hosted `push` run for binary candidate tag `v0.1.0`; `extension-release-proof.yml` is not present on the remote default branch queried by `gh run list`; and `publish-extension.yml` has no hosted `push` run for extension candidate tag `ext-v0.3.0`. Because `remote-evidence` fails first, the assembled S05 verifier does not currently reach `public-http` or the final S01 live-proof phase in this run.
