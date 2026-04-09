# S04: Public Proof Surface and Operator Contract Truth

**Goal:** Align the cluster-proof runbook, distributed-proof docs, and read-only Fly/public verifiers with the shipped M043 failover contract so operators see explicit promotion, runtime-owned role/epoch/health truth, stale-primary fencing, same-image local authority, and bounded Fly scope.
**Demo:** After this: The cluster-proof README, distributed-proof docs, proof-surface verifiers, and read-only Fly status checks all show the same failover contract: primary/standby roles, explicit promotion boundary, fenced old-primary behavior, supported topology, and non-goals.

## Tasks
- [x] **T01: Added M043 proof-surface and read-only Fly verifier rails with authority-field checks and retained failure artifacts.** — Create the fail-closed M043 verifier pair before touching public prose so the slice has an executable contract instead of ad hoc markdown edits.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `fly` CLI / read-only app inspection | Fail closed with the captured phase log and the last saved stdout/stderr artifact. | Mark the phase as failed, preserve partial artifacts, and report which read-only probe never converged. | Reject mismatched hosts, malformed JSON, or missing `cluster_role` / `promotion_epoch` / `replication_health` fields before claiming live truth. |
| M042 verifier templates | Stop and report the stale contract point instead of silently inheriting M042 wording into the new M043 rail. | N/A | Reject copied assertions that still look for M042-only command names or payload shapes. |

## Load Profile

- **Shared resources**: Local artifact directory, optional Fly API/logs, and live `cluster-proof` HTTP responses.
- **Per-operation cost**: One proof-surface string sweep plus read-only `fly status`, `fly config show`, `fly logs --no-tail`, `GET /membership`, and optional `GET /work/:request_key`.
- **10x breakpoint**: Fly API latency and log volume fail before the verifier logic changes.

## Negative Tests

- **Malformed inputs**: Missing `CLUSTER_PROOF_FLY_APP`, host/app mismatches in `CLUSTER_PROOF_BASE_URL`, and malformed optional request keys must fail in input validation.
- **Error paths**: Missing M043 script names, stale M042 wording, or live JSON without authority fields must stop the verifier with an artifact hint.
- **Boundary conditions**: Live Fly mode stays read-only even when `CLUSTER_PROOF_REQUEST_KEY` is set; the verifier must not submit work, restart machines, or mutate config.

## Steps

1. Clone the structure of `scripts/verify-m042-s04-proof-surface.sh` and `scripts/verify-m042-s04-fly.sh` into named M043 verifier scripts.
2. Upgrade the proof-surface contract to require M043 commands, explicit `/promote` authority wording, same-image local authority, stale-primary fencing, supported topology, and non-goals.
3. Upgrade the Fly verifier to keep the read-only command set but require `cluster_role`, `promotion_epoch`, and `replication_health` on `/membership` and optional keyed-status JSON.
4. Keep both scripts writing retained phase/state/full-log artifacts under `.tmp/m043-s04/` and failing closed on stale help text, config drift, or malformed live payloads.

## Must-Haves

- [ ] `scripts/verify-m043-s04-proof-surface.sh` exists and bans stale M042 command/wiring text from the public surfaces.
- [ ] `scripts/verify-m043-s04-fly.sh` help/live text names `bash scripts/verify-m043-s03.sh` as the destructive local authority and keeps Fly read-only.
- [ ] Live JSON checks require authority fields instead of the older M042 membership/status shape.
- [ ] Retained verifier artifacts point to the failing phase/log without leaking secrets.
  - Estimate: 2h
  - Files: scripts/verify-m043-s04-proof-surface.sh, scripts/verify-m043-s04-fly.sh, scripts/verify-m042-s04-proof-surface.sh, scripts/verify-m042-s04-fly.sh, scripts/verify-m043-s03.sh, scripts/lib/m043_cluster_proof.sh, cluster-proof/fly.toml
  - Verify: bash scripts/verify-m043-s04-fly.sh --help && bash -n scripts/verify-m043-s04-proof-surface.sh scripts/verify-m043-s04-fly.sh
- [x] **T02: Reconciled the cluster-proof runbook and distributed docs to the shipped M043 failover/operator contract.** — Update every public entrypoint to describe the same shipped M043 story the new verifier rails enforce: explicit promotion, runtime-owned authority fields, stale-primary fencing, same-image local authority, and bounded Fly scope.

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
  - Estimate: 2h
  - Files: cluster-proof/README.md, website/docs/docs/distributed-proof/index.md, website/docs/docs/distributed/index.md, README.md, scripts/verify-m043-s04-proof-surface.sh, scripts/verify-m043-s04-fly.sh, scripts/verify-m043-s03.sh
  - Verify: bash scripts/verify-m043-s04-proof-surface.sh && bash scripts/verify-m043-s04-fly.sh --help && npm --prefix website run build && bash scripts/verify-m043-s03.sh
