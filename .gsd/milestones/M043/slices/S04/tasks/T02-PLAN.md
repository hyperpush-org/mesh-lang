---
estimated_steps: 4
estimated_files: 7
skills_used:
  - vitepress
  - flyio-cli-public
---

# T02: Reconcile the runbook and public docs to the M043 failover contract

**Slice:** S04 — Public Proof Surface and Operator Contract Truth
**Milestone:** M043

## Description

Update every public entrypoint to describe the same shipped M043 story the new verifier rails enforce: explicit promotion, runtime-owned authority fields, stale-primary fencing, same-image local authority, and bounded Fly scope.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Public proof-surface verifier | Stop on the first stale command, link, or contract sentence and use the emitted artifact/log path to localize the drift. | N/A | Treat mismatched command lists or missing required wording as contract drift, not as a docs-style warning. |
| VitePress build | Stop and keep the build log in place; do not mark the public rail complete with broken website docs. | Abort the build and preserve the log. | Reject broken markdown/frontmatter or bad links that surface as build errors. |
| Local packaged verifier | Stop and treat it as proof that the docs now describe something the shipped M043 rail no longer proves. | Abort and preserve the verifier bundle. | N/A |

## Load Profile

- **Shared resources**: VitePress build cache, repo markdown files, and the local same-image verifier bundle.
- **Per-operation cost**: One proof-surface verifier run, one website build, one Fly help replay, and one local failover replay.
- **10x breakpoint**: Website build time and local failover replay time fail before markdown editing becomes logically different.

## Negative Tests

- **Malformed inputs**: Stale `verify-m042-*` script names, missing `/promote` references, or claims of automatic promotion / active-active intake must fail the proof-surface sweep.
- **Error paths**: Broken links, bad frontmatter, or docs that disagree on the canonical commands must fail either the proof-surface verifier or the website build.
- **Boundary conditions**: The docs must allow post-rejoin `replication_health` truth to vary between `local_only` and `healthy` on the promoted standby instead of freezing one post-rejoin value as the only honest state.

## Steps

1. Rewrite `cluster-proof/README.md` so the environment contract, canonical commands, and failure-inspection guidance point at the new M043 verifier pair and the shipped same-image failover rail.
2. Update `website/docs/docs/distributed-proof/index.md` with the M043 contract: `bash scripts/verify-m043-s03.sh` local authority, `/promote`, authority fields, stale-primary fencing, supported topology, and non-goals.
3. Keep `website/docs/docs/distributed/index.md` and repo `README.md` as routing surfaces that send operator claims to the proof page/runbook instead of duplicating stale or weaker wording.
4. Verify the reconciled text with the new proof-surface script, the website build, the Fly help path, and the local same-image verifier.

## Must-Haves

- [ ] `cluster-proof/README.md`, `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, and `README.md` all reference the same M043 script names and contract language.
- [ ] Public docs name `/promote` as the explicit authority boundary and old-primary rejoin as fenced/deposed behavior.
- [ ] Public docs keep the operator seam narrow: same image, small env surface, no active-active intake, no automatic promotion, and no destructive Fly failover requirement.
- [ ] `npm --prefix website run build` stays green after the wording changes.

## Verification

- `bash scripts/verify-m043-s04-proof-surface.sh`
- `bash scripts/verify-m043-s04-fly.sh --help`
- `npm --prefix website run build`
- `bash scripts/verify-m043-s03.sh`

## Inputs

- `scripts/verify-m043-s04-proof-surface.sh` — new public-contract gate that the docs must satisfy.
- `scripts/verify-m043-s04-fly.sh` — new read-only Fly verifier whose help text must match the docs.
- `scripts/verify-m043-s03.sh` — destructive local authority the public docs must point at.
- `cluster-proof/README.md` — deepest runbook to reconcile to the M043 contract.
- `website/docs/docs/distributed-proof/index.md` — public proof page to update.
- `website/docs/docs/distributed/index.md` — routing guide that should point readers to the proof page instead of duplicating stale operator claims.
- `README.md` — repo-root entrypoint that must route distributed-proof claims to the reconciled surfaces.
- `cluster-proof/fly.toml` — reference for the bounded Fly operator contract the docs should describe.

## Expected Output

- `cluster-proof/README.md` — reconciled M043 runbook.
- `website/docs/docs/distributed-proof/index.md` — reconciled public proof page.
- `website/docs/docs/distributed/index.md` — updated routing copy.
- `README.md` — updated repo-root distributed-proof routing copy.
