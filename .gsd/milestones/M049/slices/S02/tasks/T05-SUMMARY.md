---
id: T05
parent: S02
milestone: M049
provides: []
requires: []
affects: []
key_files: ["tools/skill/mesh/SKILL.md", "tools/skill/mesh/skills/clustering/SKILL.md", "tools/skill/mesh/skills/http/SKILL.md", "scripts/tests/verify-m048-s04-skill-contract.test.mjs"]
key_decisions: ["Keep clustered starter guidance centered on `meshc init --clustered` and `meshc init --template todo-api --db postgres`, while treating `meshc init --template todo-api --db sqlite` as the honest local single-node starter.", "Make the retained M048 skill contract fail closed on generic `meshc init --template todo-api` wording and stale clustered-SQLite claims, not just legacy helper names."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "`node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs` passed after the fail-closed rewrite, proving the new SQLite-local/Postgres-clustered skill contract and the retained stale-wording guards. `node --test scripts/tests/verify-m048-s05-contract.test.mjs` also passed, proving the broader retained M048 public-contract rail stayed green."
completed_at: 2026-04-03T00:18:16.890Z
blocker_discovered: false
---

# T05: Retargeted the Mesh skills and retained M048 contract rail to enforce the SQLite-local/Postgres-clustered starter split.

> Retargeted the Mesh skills and retained M048 contract rail to enforce the SQLite-local/Postgres-clustered starter split.

## What Happened
---
id: T05
parent: S02
milestone: M049
key_files:
  - tools/skill/mesh/SKILL.md
  - tools/skill/mesh/skills/clustering/SKILL.md
  - tools/skill/mesh/skills/http/SKILL.md
  - scripts/tests/verify-m048-s04-skill-contract.test.mjs
key_decisions:
  - Keep clustered starter guidance centered on `meshc init --clustered` and `meshc init --template todo-api --db postgres`, while treating `meshc init --template todo-api --db sqlite` as the honest local single-node starter.
  - Make the retained M048 skill contract fail closed on generic `meshc init --template todo-api` wording and stale clustered-SQLite claims, not just legacy helper names.
duration: ""
verification_result: passed
completed_at: 2026-04-03T00:18:16.891Z
blocker_discovered: false
---

# T05: Retargeted the Mesh skills and retained M048 contract rail to enforce the SQLite-local/Postgres-clustered starter split.

**Retargeted the Mesh skills and retained M048 contract rail to enforce the SQLite-local/Postgres-clustered starter split.**

## What Happened

Updated the Mesh root skill and the clustering sub-skill so clustered-runtime questions now point at `meshc init --clustered` for the minimal scaffold and `meshc init --template todo-api --db postgres` for the fuller shared/deployable starter, while `meshc init --template todo-api --db sqlite` is explicitly described as the honest local single-node starter. I also tightened the HTTP sub-skill so `HTTP.clustered(...)` stays documented as a bounded routed-read wrapper, but no longer implies that the SQLite starter participates in the clustered runtime story.

Then I rewrote `scripts/tests/verify-m048-s04-skill-contract.test.mjs` into a fail-closed validator with temp-copy mutation cases. The retained rail now proves the new starter split across the root, clustering, syntax, and HTTP skills, and it also rejects the old unsplit `meshc init --template todo-api` wording plus stale clustered-SQLite copy. The first pass exposed a bug in the new negative test itself because the root skill mentions each starter twice; I fixed that by mutating both copies with `replaceAll(...)` and reran the rail green.

## Verification

`node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs` passed after the fail-closed rewrite, proving the new SQLite-local/Postgres-clustered skill contract and the retained stale-wording guards. `node --test scripts/tests/verify-m048-s05-contract.test.mjs` also passed, proving the broader retained M048 public-contract rail stayed green.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs` | 0 | ✅ pass | 1066ms |
| 2 | `node --test scripts/tests/verify-m048-s05-contract.test.mjs` | 0 | ✅ pass | 947ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `tools/skill/mesh/SKILL.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`
- `tools/skill/mesh/skills/http/SKILL.md`
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs`


## Deviations
None.

## Known Issues
None.
