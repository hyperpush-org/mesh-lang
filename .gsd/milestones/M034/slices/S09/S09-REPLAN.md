# S09 Replan

**Milestone:** M034
**Slice:** S09
**Blocker Task:** T03
**Created:** 2026-03-27T18:43:31.296Z

## Blocker Description

The approved hosted reroll on head SHA c443270a8fe17419e9ca99b4755b90f3cb7af3a0 did not yield an all-green workflow set. `publish-extension.yml` failed because Open VSX rejected the already-published `mesh-lang` 0.3.0 extension as a duplicate, and `release.yml` later completed red: the Windows release-smoke lane crashed in `scripts/verify-m034-s03.ps1` when strict mode read an unset `$LASTEXITCODE`, while the authoritative live-proof job failed at the S01 metadata fetch with a curl SSL timeout. Because the hosted release/extension lanes are not reroll-safe yet, the old archive-and-replay-only T04 cannot succeed as written.

## What Changed

Replaced the old single archive/replay task with a three-step closeout. First, repair the deterministic hosted reroll blockers locally: make extension publication duplicate-safe on reruns, fix the Windows PowerShell verifier so an unset `$LASTEXITCODE` no longer crashes under strict mode, and harden the S01 metadata fetch so one transient transport timeout does not sink the release lane. Second, record a new repaired rollout SHA, get explicit approval for any outward GitHub mutation, move `main`, `v0.1.0`, and `ext-v0.3.0` onto that repaired SHA, and capture a fully green hosted evidence set with durable status and failed-job logs if anything is still red. Third, only after hosted evidence is green, claim `first-green` exactly once and rerun the canonical `bash scripts/verify-m034-s05.sh` replay. Threat surface is unchanged, and requirement coverage still targets the same hosted-freshness/public-surface closeout requirements; the replan just makes hosted reroll idempotency and release-lane truth explicit prerequisites instead of assumptions.
