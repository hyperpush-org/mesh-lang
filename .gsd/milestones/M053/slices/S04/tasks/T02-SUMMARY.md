---
id: T02
parent: S04
milestone: M053
provides: []
requires: []
affects: []
key_files: ["website/docs/docs/distributed/index.md", "website/docs/docs/distributed-proof/index.md", "scripts/fixtures/clustered/cluster-proof/README.md", "scripts/verify-m043-s04-fly.sh", "scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl", ".gsd/DECISIONS.md", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Anchor Distributed Proof on `scripts/verify-m053-s01.sh`, `scripts/verify-m053-s02.sh`, and `scripts/verify-m053-s03.sh`, while treating Fly/`cluster-proof` as retained read-only reference proof instead of a coequal public starter surface."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed `bash scripts/verify-production-proof-surface.sh`, a rerun of `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` after fixing exact README phrase drift, and the exact combined task contract `bash scripts/verify-production-proof-surface.sh && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`."
completed_at: 2026-04-05T21:44:55.295Z
blocker_discovered: false
---

# T02: Reframed Distributed Proof around the M053 Postgres starter chain and demoted Fly/`cluster-proof` to retained reference surfaces.

> Reframed Distributed Proof around the M053 Postgres starter chain and demoted Fly/`cluster-proof` to retained reference surfaces.

## What Happened
---
id: T02
parent: S04
milestone: M053
key_files:
  - website/docs/docs/distributed/index.md
  - website/docs/docs/distributed-proof/index.md
  - scripts/fixtures/clustered/cluster-proof/README.md
  - scripts/verify-m043-s04-fly.sh
  - scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl
  - .gsd/DECISIONS.md
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Anchor Distributed Proof on `scripts/verify-m053-s01.sh`, `scripts/verify-m053-s02.sh`, and `scripts/verify-m053-s03.sh`, while treating Fly/`cluster-proof` as retained read-only reference proof instead of a coequal public starter surface.
duration: ""
verification_result: mixed
completed_at: 2026-04-05T21:44:55.296Z
blocker_discovered: false
---

# T02: Reframed Distributed Proof around the M053 Postgres starter chain and demoted Fly/`cluster-proof` to retained reference surfaces.

**Reframed Distributed Proof around the M053 Postgres starter chain and demoted Fly/`cluster-proof` to retained reference surfaces.**

## What Happened

Rewrote `website/docs/docs/distributed-proof/index.md` so the public clustered proof map now names the generated Postgres starter's M053 chain directly: `scripts/verify-m053-s01.sh` for staged deploy proof, `scripts/verify-m053-s02.sh` for failover proof, and `scripts/verify-m053-s03.sh` for the hosted packages/public-surface contract. Kept the existing public-secondary page markers and the Production Backend Proof → Mesher → retained backend verifier handoff intact so the established proof-surface contract still passes. Updated `website/docs/docs/distributed/index.md` to hand readers to that M053 story instead of emphasizing older retained rails, while keeping SQLite explicitly local-only and PostgreSQL explicitly shared/deployable. Reframed `scripts/fixtures/clustered/cluster-proof/README.md` and the `scripts/verify-m043-s04-fly.sh` help surface so Fly and `cluster-proof` are described as bounded retained reference/read-only proof assets rather than equal canonical starter surfaces. Tightened `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl` to pin the new retained-reference wording in both the fixture README and Fly verifier help text. The first fixture-test rerun failed because the README rewrite had preserved the meaning but dropped two literal pinned phrases (`route-free` and the exact-cased `It is not a public starter surface` sentence). I restored those exact markers, reran the fixture suite, and then reran the exact combined task verification command. Recorded the public-contract choice as decision D410 and added a knowledge note about these literal fixture-copy pins so T03 and later docs work do not have to rediscover the same gotcha.

## Verification

Passed `bash scripts/verify-production-proof-surface.sh`, a rerun of `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` after fixing exact README phrase drift, and the exact combined task contract `bash scripts/verify-production-proof-surface.sh && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `bash scripts/verify-production-proof-surface.sh` | 0 | ✅ pass | 4590ms |
| 2 | `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` | 1 | ❌ fail | 7960ms |
| 3 | `cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` | 0 | ✅ pass | 6500ms |
| 4 | `bash scripts/verify-production-proof-surface.sh && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests` | 0 | ✅ pass | 10140ms |


## Deviations

Added decision D410 in `.gsd/DECISIONS.md` and a matching note in `.gsd/KNOWLEDGE.md` because the task clarified a downstream public-contract choice and exposed a non-obvious literal-copy pin in the retained fixture rail.

## Known Issues

None.

## Files Created/Modified

- `website/docs/docs/distributed/index.md`
- `website/docs/docs/distributed-proof/index.md`
- `scripts/fixtures/clustered/cluster-proof/README.md`
- `scripts/verify-m043-s04-fly.sh`
- `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl`
- `.gsd/DECISIONS.md`
- `.gsd/KNOWLEDGE.md`


## Deviations
Added decision D410 in `.gsd/DECISIONS.md` and a matching note in `.gsd/KNOWLEDGE.md` because the task clarified a downstream public-contract choice and exposed a non-obvious literal-copy pin in the retained fixture rail.

## Known Issues
None.
