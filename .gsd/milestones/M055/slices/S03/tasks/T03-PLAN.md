---
estimated_steps: 3
estimated_files: 8
skills_used:
  - create-skill
  - vitepress
  - test
---

# T03: Realign generic guide callouts, the Mesh clustering skill, and retained docs wrappers to the new public boundary

The public contract is still incomplete if the generic guide callouts, the auto-loaded clustering skill, or the retained docs wrappers keep teaching the old local-product path. This task should align those secondary surfaces and historical rails to the same repo-boundary handoff so future agents and retained milestone verifiers do not drag the old story back in.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `scripts/tests/verify-m048-s04-skill-contract.test.mjs` | Fail closed on the first stale skill marker so executor guidance cannot silently drift. | N/A for local source checks. | Treat a mixed old/new handoff in the skill as contract drift. |
| `scripts/verify-m051-s04.sh` and `scripts/verify-m047-s06.sh` | Stop on the first retained-wrapper mismatch and preserve the failing phase under their existing `.tmp/` roots. | Use each wrapper’s bounded timeout budget and stop before stale bundle reuse. | Treat missing expected markers or malformed retained bundle pointers as wrapper drift. |

## Load Profile

Shared resources are `website/docs/.vitepress/dist`, `.tmp/m051-s04/verify/`, and `.tmp/m047-s06/verify/`; per-operation cost is one docs build, one skill mutation test, and two retained wrapper replays; the first 10x breakpoint is wrapper churn and stale docs markers across historical rails.

## Negative Tests

- **Malformed inputs**: generic guides or the clustering skill still naming local `mesher/...` paths as the public follow-on step.
- **Error paths**: the public docs are corrected, but a retained wrapper or skill still demands the old wording and makes the closeout rails false-red.
- **Boundary conditions**: generic guides stay subsystem-focused, the clustering skill stays examples-first, and historical wrappers still keep their retained proof surfaces intact.

## Steps

1. Rewrite `website/docs/docs/web/index.md`, `website/docs/docs/databases/index.md`, `website/docs/docs/testing/index.md`, `website/docs/docs/concurrency/index.md`, and `tools/skill/mesh/skills/clustering/SKILL.md` so they match the repo-boundary product handoff from T01/T02 without reintroducing local product-source-path teaching.
2. Update `scripts/tests/verify-m048-s04-skill-contract.test.mjs` plus the retained wrapper expectations in `scripts/verify-m047-s06.sh` and `scripts/verify-m051-s04.sh` so the new boundary does not make older assembled docs rails false-red.
3. Rebuild the docs site and replay the skill/historical wrapper rails.

## Must-Haves

- [ ] Generic guide callouts and the clustering skill match the same public boundary as the first-contact and proof pages.
- [ ] Retained M047/M051 docs wrappers stay green without reintroducing local product-source-path teaching.
- [ ] Skill and wrapper drift remain fail-closed.

## Inputs

- `website/docs/docs/web/index.md`
- `website/docs/docs/databases/index.md`
- `website/docs/docs/testing/index.md`
- `website/docs/docs/concurrency/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `scripts/verify-m047-s06.sh`
- `scripts/verify-m051-s04.sh`
- `website/docs/docs/distributed-proof/index.md`

## Expected Output

- `website/docs/docs/web/index.md`
- `website/docs/docs/databases/index.md`
- `website/docs/docs/testing/index.md`
- `website/docs/docs/concurrency/index.md`
- `tools/skill/mesh/skills/clustering/SKILL.md`
- `scripts/tests/verify-m048-s04-skill-contract.test.mjs`
- `scripts/verify-m047-s06.sh`
- `scripts/verify-m051-s04.sh`

## Verification

node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs
bash scripts/verify-m051-s04.sh
bash scripts/verify-m047-s06.sh
npm --prefix website run build

## Observability Impact

- Signals added/changed: retained wrapper phase failures under `.tmp/m047-s06/verify/` and `.tmp/m051-s04/verify/`, plus skill mutation-test marker errors.
- How a future agent inspects this: start with `node --test scripts/tests/verify-m048-s04-skill-contract.test.mjs`, then open the retained wrapper `phase-report.txt` for whichever wrapper turns red.
- Failure state exposed: the exact stale marker in the skill/generic guides or the exact retained wrapper phase that drifted.
