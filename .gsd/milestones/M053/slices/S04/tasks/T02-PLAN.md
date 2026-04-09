---
estimated_steps: 4
estimated_files: 5
skills_used:
  - vitepress
  - bash-scripting
---

# T02: Reframe the distributed proof map and retained Fly assets as secondary reference surfaces

Update the proof-map docs and retained Fly reference asset copy so they match the shipped M053 contract: the generated Postgres starter owns staged deploy + failover truth, SQLite remains outside clustered proof, and Fly is a bounded read-only/reference environment rather than a coequal public starter surface.

## Steps

1. Rewrite `website/docs/docs/distributed-proof/index.md` around the M053 proof chain: `scripts/verify-m053-s01.sh`, `scripts/verify-m053-s02.sh`, and `scripts/verify-m053-s03.sh`, plus the SQLite local-only boundary and the hosted packages/public-surface contract.
2. Update `website/docs/docs/distributed/index.md` so its proof-page handoff points at the new M053 story instead of older proof-rail emphasis.
3. Reframe `scripts/fixtures/clustered/cluster-proof/README.md` and `scripts/verify-m043-s04-fly.sh` help text so the Fly rail is explicitly a retained reference/proof asset, not one of the equal canonical starter surfaces.
4. Adjust `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl` to pin the new retained-reference wording without widening Fly or `cluster-proof` into the public starter contract.

## Must-Haves

- [ ] `Distributed Proof` names the M053 starter-owned staged deploy, failover, and hosted-contract verifiers.
- [ ] Retained Fly reference assets describe read-only/reference proof only and stop claiming equal public-starter status.
- [ ] Old proof-app-first wording is removed or demoted behind retained/reference language.

## Inputs

- `website/docs/docs/distributed/index.md` — current public proof-page handoff from the generic distributed guide
- `website/docs/docs/distributed-proof/index.md` — current public proof map, including stale M043/M047 emphasis
- `scripts/fixtures/clustered/cluster-proof/README.md` — retained Fly reference README that currently overstates its public/canonical role
- `scripts/verify-m043-s04-fly.sh` — read-only Fly verifier help surface
- `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl` — retained fixture assertions that pin README/Fly copy
- `scripts/verify-m053-s01.sh` — starter-owned staged deploy proof contract to surface publicly
- `scripts/verify-m053-s02.sh` — starter-owned failover proof contract to surface publicly
- `scripts/verify-m053-s03.sh` — hosted starter/packages evidence-chain contract to surface publicly

## Expected Output

- `website/docs/docs/distributed/index.md` — updated proof-page routing language for the M053 story
- `website/docs/docs/distributed-proof/index.md` — updated M053 proof map with Fly explicitly secondary/reference-only
- `scripts/fixtures/clustered/cluster-proof/README.md` — retained-reference wording instead of equal-canonical/public-starter wording
- `scripts/verify-m043-s04-fly.sh` — help text aligned to read-only/reference proof language
- `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl` — updated assertions for the retained-reference copy

## Verification

- `bash scripts/verify-production-proof-surface.sh && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`
