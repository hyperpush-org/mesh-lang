---
estimated_steps: 4
estimated_files: 4
skills_used:
  - create-skill
  - test
---

# T05: Retarget Mesh skill guidance and skill contract tests to the honest starter split

**Slice:** S02 — SQLite local starter contract
**Milestone:** M049

## Description

Finish the assistant-facing contract. The Mesh root/clustering/HTTP skills should keep the route-free clustered runtime story and bounded `HTTP.clustered(...)` guidance, but they must stop teaching the SQLite Todo starter as part of the clustered runtime path.

## Steps

1. Update `tools/skill/mesh/SKILL.md` and `tools/skill/mesh/skills/clustering/SKILL.md` so clustered-runtime questions route to `meshc init --clustered` / `meshc init --template todo-api --db postgres`, while the SQLite starter is explicitly local-only.
2. Update `tools/skill/mesh/skills/http/SKILL.md` so `HTTP.clustered(...)` remains documented, but no longer uses the SQLite starter as the proof that clustered reads are part of the local starter contract.
3. Rewrite `scripts/tests/verify-m048-s04-skill-contract.test.mjs` to pin the new split without regressing the M048 helper-name/editor/update truths.
4. Keep `scripts/tests/verify-m048-s05-contract.test.mjs` green as the broader retained M048 non-regression check.

## Must-Haves

- [ ] Mesh skill guidance clearly separates SQLite-local starter advice from clustered-runtime/bootstrap advice.
- [ ] The HTTP skill still documents `HTTP.clustered(...)`, but not as the SQLite starter’s public contract.
- [ ] The retained M048 skill-contract tests fail closed on stale clustered-SQLite guidance.

## Verification

- `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `node --test scripts/tests/verify-m048-s05-contract.test.mjs`

## Inputs

- `tools/skill/mesh/SKILL.md` — Mesh root skill that routes users into the clustering sub-skill.
- `tools/skill/mesh/skills/clustering/SKILL.md` — clustered runtime skill that currently still references the old SQLite starter story.
- `tools/skill/mesh/skills/http/SKILL.md` — HTTP skill whose `HTTP.clustered(...)` examples must stay bounded while the starter story changes.
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs` — retained skill-contract guardrail that must be updated to the new split.
- `scripts/tests/verify-m048-s05-contract.test.mjs` — broader retained M048 non-regression rail that must stay green.

## Expected Output

- `tools/skill/mesh/SKILL.md` — root skill routing updated to the honest starter split.
- `tools/skill/mesh/skills/clustering/SKILL.md` — clustering skill updated to keep SQLite local-only and Postgres/minimal scaffold clustered.
- `tools/skill/mesh/skills/http/SKILL.md` — HTTP skill still documenting `HTTP.clustered(...)` without using the SQLite starter as clustered proof.
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs` — skill-contract rail pinned to the new split.
