# S08 Replan

**Milestone:** M034
**Slice:** S08
**Blocker Task:** T02
**Created:** 2026-03-27T16:36:07.991Z

## Blocker Description

The candidate tags now exist on the approved rollout SHA, but the hosted tag workflows are genuinely red: `deploy-services.yml` fails because the `packages-website` runtime image reruns `npm install --omit=dev --ignore-scripts` and hits a Vite/Svelte peer-dependency `ERESOLVE`, while `release.yml` fails in `Verify release assets (...)` because the staged smoke path cannot truthfully satisfy the runtime-library/checksum contract on Unix, macOS, and Windows. `first-green` cannot be archived until those hosted regressions are fixed and the candidate tags are rerun on the repaired rollout commit.

## What Changed

Replaced the direct `first-green` capture plan with a blocker-first sequence. T03 now fixes the `packages-website` deploy image so `deploy-services.yml` can go green. T04 fixes the hosted `release.yml` release-asset verification path and updates the repo-owned workflow verifiers to encode the repaired contract. Because the existing `v0.1.0` / `ext-v0.3.0` runs were tied to the broken commit, T05 now retargets or recreates the candidate tags on the repaired rollout SHA only after explicit user approval and refreshes the durable hosted snapshots. T06 becomes the final one-shot `first-green` archive capture after T05 proves all required workflows green. Threat surface is unchanged except that the second remote tag mutation remains explicitly approval-gated; requirement coverage is unchanged and still ends at authoritative hosted evidence plus a preserved first-green bundle.
