---
id: S07
parent: M034
milestone: M034
provides:
  - A single shared public-surface contract consumed by the canonical S05 verifier and the hosted deploy workflows.
  - Fresh hosted-rollout blocker evidence tying the S05 replay failure to stale remote workflow rollout rather than local script/YAML drift.
  - Direct live public-surface diagnostics proving the packages-site and registry are current while `meshlang.dev` installers/docs are still stale.
requires:
  - slice: S06
    provides: Stop-after remote-evidence archiving semantics, staged-rollout transport evidence, and the last truthful hosted baseline for `main` / candidate-tag rollout.
affects:
  []
key_files:
  - scripts/lib/m034_public_surface_contract.py
  - scripts/verify-m034-s05.sh
  - scripts/verify-m034-s05-workflows.sh
  - .github/workflows/deploy.yml
  - .github/workflows/deploy-services.yml
  - scripts/tests/verify-m034-s05-contract.test.mjs
  - scripts/tests/verify-m034-s07-public-contract.test.mjs
  - .tmp/m034-s05/verify/remote-runs.json
  - .tmp/m034-s07-public-http-check2/public-http.log
  - .gsd/KNOWLEDGE.md
  - .gsd/PROJECT.md
key_decisions:
  - D096: keep the public installer/docs/packages markers and bounded freshness wait in one shared helper consumed by S05 and the hosted deploy workflows.
  - D097: recover blocked remote rollout in bounded fast-forward prefixes and preserve reserved hosted-evidence labels until the truthful final state exists.
  - Reproduce closeout truth through the unmodified canonical `bash scripts/verify-m034-s05.sh` entrypoint rather than bypassing it with ad hoc hosted checks.
  - Use curl-backed live fetches in the shared public-surface helper so HTTPS verification exposes real public drift on this host instead of failing on local certificate validation.
patterns_established:
  - Keep one shared helper as the only owner of public installer/docs/packages markers, normalized HTML marker checks, and the bounded retry budget; both local wrappers and hosted workflows should call it verbatim.
  - Treat a green hosted run that is missing required jobs or steps as stale workflow rollout, not as deployment success.
  - Measure public freshness with exact installer byte diffs plus normalized docs markers and packages/registry parity; partial green surfaces are not enough to claim public truth.
  - On this host, prefer curl-backed live fetches for public verification to avoid local TLS false negatives masking the real public-state drift.
observability_surfaces:
  - `.tmp/m034-s05/verify/{status.txt,current-phase.txt,phase-report.txt,remote-evidence.log,remote-runs.json}` for the canonical replay boundary and hosted-workflow evidence.
  - `.tmp/m034-s07-public-http-check2/{public-http.log,public-install-sh.diff,public-install-ps1.diff,public-getting-started-check.log,public-tooling-check.log}` for direct live-surface mismatch diagnostics.
  - `.tmp/m034-s06/transport-recovery/attempts.log` for the staged-rollout recovery history leading to remote `main` = `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`.
  - `scripts/tests/verify-m034-s05-contract.test.mjs` and `scripts/tests/verify-m034-s07-public-contract.test.mjs` for mechanical contract coverage.
drill_down_paths:
  - .gsd/milestones/M034/slices/S07/tasks/T01-SUMMARY.md
  - .gsd/milestones/M034/slices/S07/tasks/T02-SUMMARY.md
  - .gsd/milestones/M034/slices/S07/tasks/T03-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-03-27T08:28:00.271Z
blocker_discovered: false
---

# S07: Public surface freshness and final assembly replay

**S07 unified the public-surface contract and refreshed blocker evidence, but the canonical S05 replay still fails at `remote-evidence` because remote rollout and `meshlang.dev` freshness are still stale.**

## What Happened

S07 did deliver the local hardening work promised by T01: one shared helper now owns the public installer/docs/packages marker set, normalized built/live docs checks, and the bounded freshness-wait contract that both `scripts/verify-m034-s05.sh` and the hosted deploy workflows consume. I also tightened that helper’s live transport to use `curl`, which matters on this host because the earlier `urllib` path could fail on local certificate validation before it ever measured real public-surface drift.

The slice did not deliver the planned final assembly replay. I reran the exact canonical entrypoint (`set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`) after the helper hardening, and every local phase through `s04-workflows` still passed before the wrapper failed closed at `remote-evidence`. The refreshed `.tmp/m034-s05/verify/remote-runs.json` shows the same blocker chain T02/T03 found: `main` is only at `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`, the green `deploy.yml` push run on that SHA (`23635781919`) still uses the older step graph and is missing `Verify public docs contract`, `authoritative-verification.yml` is still absent from the remote default branch, and the `v0.1.0` / `ext-v0.3.0` candidate tags still have no hosted push runs.

Because `remote-evidence` still blocks the canonical wrapper, I ran the shared helper directly against the live public surfaces with a fresh artifact root. That check now reaches the network successfully and shows the real public mismatch instead of a local TLS false negative: `packages.meshlang.dev` detail/search pages and the registry scoped-search API all pass, while `meshlang.dev/install.sh`, `meshlang.dev/install.ps1`, `/docs/getting-started/`, and `/docs/tooling/` all remain stale relative to the repo’s current truth. The installer diffs show the live site is still serving the older meshc-only installers and pre-S07 docs content.

So the actual output of S07 is a stronger, reusable public-surface contract plus a sharper blocker bundle, not the green final replay promised in the original slice goal. Downstream roadmap reassessment should treat S07 as proving two things: the remaining failure is not local verifier drift, and once remote rollout is finally fixed the next honest question is whether the live `meshlang.dev` deployment catches up enough for `public-http` to turn green.

## Verification

- `python3 -m py_compile scripts/lib/m034_public_surface_contract.py` ✅
- `bash -n scripts/verify-m034-s05.sh` ✅
- `node --test scripts/tests/verify-m034-s05-contract.test.mjs scripts/tests/verify-m034-s07-public-contract.test.mjs` ✅ (9 tests passed)
- `bash scripts/verify-m034-s05-workflows.sh` ✅
- `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh` ❌ fail-closed at `remote-evidence` after every local phase through `s04-workflows` passed
- `gh api repos/snowdamiz/mesh-lang/branches/main --jq '.commit.sha'` ✅ -> `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`
- `gh run list -R snowdamiz/mesh-lang --workflow deploy.yml --event push --branch main --limit 1 --json ...` ✅ -> green run `23635781919` on stale SHA `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`
- `gh run list -R snowdamiz/mesh-lang --workflow authoritative-verification.yml --event push --branch main --limit 1 --json ...` ✅ (truthful failure) -> workflow missing on remote default branch
- `gh run list -R snowdamiz/mesh-lang --workflow release.yml --event push --branch v0.1.0 --limit 1 --json ...` ✅ -> `[]`
- `gh run list -R snowdamiz/mesh-lang --workflow deploy-services.yml --event push --branch v0.1.0 --limit 1 --json ...` ✅ -> `[]`
- `gh run list -R snowdamiz/mesh-lang --workflow publish-extension.yml --event push --branch ext-v0.3.0 --limit 1 --json ...` ✅ -> `[]`
- `python3 scripts/lib/m034_public_surface_contract.py public-http --root . --artifact-dir .tmp/m034-s07-public-http-check2` ❌ -> reached the live HTTPS surfaces successfully and proved `meshlang.dev` installers/docs are still stale while package detail/search/registry search pass
- `grep -Fx 'ok' .tmp/m034-s05/verify/status.txt` / `grep -Fx 'complete' .tmp/m034-s05/verify/current-phase.txt` / `grep -Fx 'remote-evidence	passed' .tmp/m034-s05/verify/phase-report.txt` / `grep -Fx 'public-http	passed' .tmp/m034-s05/verify/phase-report.txt` / `grep -Fx 's01-live-proof	passed' .tmp/m034-s05/verify/phase-report.txt` / `test -s .tmp/m034-s05/verify/public-http.log` ❌ -> current canonical replay is not in the required final-green state

## Requirements Advanced

None.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

The slice plan targeted a green final replay and fresh hosted evidence on `main`, `v0.1.0`, and `ext-v0.3.0`. That did not happen. The truthful delivered state is: (1) one shared public-surface contract and stronger local/hosted workflow guards, (2) refreshed blocker evidence showing the remote rollout graph is still stale, and (3) direct live-surface diagnostics proving `meshlang.dev` installers/docs are still stale even though the packages-site and registry surfaces are current. I also had to switch the shared helper’s live fetch transport from `urllib` to `curl` so `public-http` could measure real drift on this host instead of failing locally on certificate validation.

## Known Limitations

The slice still does not achieve its original demo. `origin/main` remains at `8d3e76a65986e7bd5e00d107f9e11c1923ecd0ab`; the latest green `deploy.yml` run on that SHA is still missing `build: Verify public docs contract`; `authoritative-verification.yml` is still absent from the remote default branch; there are still no truthful `push` runs for `release.yml`, `deploy-services.yml`, or `publish-extension.yml` on `v0.1.0` / `ext-v0.3.0`; no `.tmp/m034-s06/evidence/first-green/remote-runs.json` bundle exists; and the canonical S05 replay still stops at `remote-evidence` before `public-http` or `s01-live-proof`. Direct live checks now also confirm that `meshlang.dev` is still serving stale installers/docs, even though the packages-site and registry surfaces are current.

## Follow-ups

Land the current rollout graph on the remote default branch through a transport or remote-update path that can actually publish the missing history, wait for fresh `main` and candidate-tag push runs, then rerun the canonical `set -a && source .env && set +a && bash scripts/verify-m034-s05.sh`. If `remote-evidence` finally clears, use the curl-backed `public-http` artifacts to reconcile any remaining `meshlang.dev` installer/docs drift before trusting `s01-live-proof`.

## Files Created/Modified

- `scripts/lib/m034_public_surface_contract.py` — Centralized the public installer/docs/packages contract and switched live HTTPS fetches to curl-backed transport so `public-http` exposes real drift on this host.
- `scripts/verify-m034-s05.sh` — Kept the canonical S05 wrapper on one shared contract for local docs, built docs, hosted evidence, and live public-surface checks.
- `scripts/verify-m034-s05-workflows.sh` — Pinned the hosted deploy workflows to the shared helper call-sites and rejected the older shallow curl/grep proof bodies.
- `.github/workflows/deploy.yml` — Strengthened the GitHub Pages build job to require the shared built-docs contract before upload.
- `.github/workflows/deploy-services.yml` — Strengthened the Fly post-deploy health check to call the shared live public-surface contract instead of weaker inline checks.
- `scripts/tests/verify-m034-s05-contract.test.mjs` — Pinned helper ownership, workflow wiring, and retry-budget semantics for the assembled S05/S07 contract.
- `scripts/tests/verify-m034-s07-public-contract.test.mjs` — Added fail-closed coverage for local-docs, built-docs, and live `public-http` retry/exhaustion behavior.
- `.tmp/m034-s05/verify/remote-runs.json` — Captured the refreshed remote-evidence blocker state for the canonical replay closeout rerun.
- `.tmp/m034-s07-public-http-check2/public-http.log` — Captured direct live public-surface diffs and missing-marker logs showing `meshlang.dev` is still stale while packages surfaces pass.
- `.gsd/KNOWLEDGE.md` — Recorded the rollout/public-surface gotchas for future agents, including curl-backed live checks and stale S01 artifact handling.
- `.gsd/PROJECT.md` — Refreshed the project current-state narrative to reflect the S07 helper hardening and the still-open hosted rollout/public freshness gap.
