# M043 — S04 Research

**Date:** 2026-03-28

## Summary

S04 is contract-reconciliation work, not new runtime work. The shipped M043 failover surface already exists in code and in the packaged local verifier, but the public/docs/verifier rail is still M042-shaped.

What already exists:

- `cluster-proof/main.mpl` exposes the real M043 HTTP surface: `GET /membership`, `POST /promote`, `GET /work`, `POST /work`, and `GET /work/:request_key`.
- `cluster-proof/cluster.mpl`’s membership payload already includes `cluster_role`, `promotion_epoch`, and `replication_health` alongside the older membership/discovery fields.
- `cluster-proof/work_continuity.mpl` already returns the same authority fields on keyed status payloads and on `/promote` responses (`ok`, `cluster_role`, `promotion_epoch`, `replication_health`, `error`).
- `cluster-proof/config.mpl` and `cluster-proof/docker-entrypoint.sh` already define and fail-close the same-image continuity env seam: `MESH_CONTINUITY_ROLE` and `MESH_CONTINUITY_PROMOTION_EPOCH`.
- `scripts/verify-m043-s03.sh` is already the authoritative packaged local failover rail. It proves mirrored standby truth, degraded standby truth after primary loss, explicit promotion, failover retry rollover, promoted completion, fenced old-primary rejoin, and retained same-image artifacts.

What is still stale:

- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `scripts/verify-m042-s04-proof-surface.sh`
- `scripts/verify-m042-s04-fly.sh`

Those surfaces still advertise the M042 story: keyed continuity, split probe surfaces, repo-root packaging, and read-only Fly sanity, but **not** explicit promotion, role/epoch truth, stale-primary fencing, same-image failover authority, or M043 non-goals. There is no `verify-m043-s04-*` rail yet.

This slice primarily owns **R053** and supports **R052** and **R051** indirectly:

- **R053:** public proof/docs/verifier truth is the core ownership of S04.
- **R052:** the operator story must remain “same image + small env surface,” not widen into bespoke orchestration.
- **R051:** public claims about primary→standby replication and failover remain honest only if the docs/verifiers point at the real M043 authority.

## Recommendation

Build S04 as two seams, with no runtime changes unless a doc statement is impossible to support from existing code:

1. **Add M043-specific proof-surface verifier rails.**
   - Create `scripts/verify-m043-s04-proof-surface.sh` by cloning the structure of `scripts/verify-m042-s04-proof-surface.sh`, but update the required strings, canonical commands, and rejected stale wording to the M043 failover contract.
   - Create `scripts/verify-m043-s04-fly.sh` by cloning `scripts/verify-m042-s04-fly.sh`, preserving the read-only model but upgrading the live JSON checks to require M043 authority fields.

2. **Update the public/operator text to the real M043 contract.**
   Update the runbook and public docs together so they all say the same thing:
   - authoritative local packaged rail: `bash scripts/verify-m043-s03.sh`
   - read-only Fly rail: `bash scripts/verify-m043-s04-fly.sh --help` and optional live read-only inspection
   - explicit promotion boundary: `POST /promote` is the authority change, not peer disappearance
   - stale-primary fencing: returning old primary is deposed/fenced and must not resume authority
   - supported topology: active primary cluster + live standby cluster, same image, small env surface
   - non-goals: no active-active intake, no automatic promotion from simple disappearance, no destructive Fly failover as a milestone blocker

Do **not** touch `mesh-rt` or Mesh app logic for this slice unless the docs uncover a missing observable field. The runtime authority/status seam is already present; the gap is public truth.

The `flyio-cli-public` skill reinforces the right operational stance here: prefer read-only Fly commands first and do not add state-changing Fly operations without approval. The `distributed-systems` fencing-token rule is the right lens for the M043 prose: promotion epoch is the authority fence, so a stale primary must never regain write authority merely because it came back.

## Implementation Landscape

### Public/runtime surfaces already shipping

- **`cluster-proof/main.mpl`**
  - Public HTTP surface is already M043-shaped.
  - `/membership` returns runtime authority truth through `membership_payload(...)`.
  - `/promote` is mounted and returns authority payloads via `handle_promote`.
  - `GET /work` remains the legacy routing probe.
  - `POST /work` and `GET /work/:request_key` are the keyed continuity surfaces.
  - Logs runtime authority readiness as:
    - `cluster_role`
    - `promotion_epoch`
    - `replication_health`

- **`cluster-proof/cluster.mpl`**
  - `membership_payload_json_from_membership(...)` emits:
    - `mode`
    - `self`
    - `peers`
    - `membership`
    - `http_port`
    - `cluster_port`
    - `discovery_provider`
    - `discovery_seed`
    - `cluster_role`
    - `promotion_epoch`
    - `replication_health`
  - This is the exact shape the new Fly verifier and docs should reference.

- **`cluster-proof/work.mpl`**
  - `WorkStatusPayload` already carries the authority fields alongside request/attempt/result/execution truth:
    - `cluster_role`
    - `promotion_epoch`
    - `replication_health`
  - `attempt_id` remains part of the request-level fence and should still be documented, but it is no longer the whole story.

- **`cluster-proof/work_continuity.mpl`**
  - `ContinuityAuthorityStatus` is the operator-facing authority truth.
  - `authority_payload_json(...)` is the `/promote` response shape:
    - `ok`
    - `cluster_role`
    - `promotion_epoch`
    - `replication_health`
    - `error`
  - `promotion_response_status_code(...)` already distinguishes success vs rejection.
  - Keyed status and invalid-request responses already reuse the same authority fields.

### Operator/env contract already shipping

- **`cluster-proof/config.mpl`**
  - Defines the continuity env names:
    - `MESH_CONTINUITY_ROLE`
    - `MESH_CONTINUITY_PROMOTION_EPOCH`
  - Enforces cluster-mode explicit role and standby epoch-0-before-promotion.
  - Keeps the small operator surface intact.

- **`cluster-proof/docker-entrypoint.sh`**
  - Mirrors the config validation at container start.
  - Fails closed on contradictory role/epoch input before runtime startup.
  - Uses hostname fallback for same-image local/Docker identity.
  - This matters for R052: the image entrypoint remains the thin operator seam.

- **`cluster-proof/fly.toml`**
  - Still matches the small public env surface:
    - `PORT=8080`
    - `MESH_CLUSTER_PORT=4370`
    - `MESH_DISCOVERY_SEED=<app>.internal`
    - `CLUSTER_PROOF_DURABILITY=replica-backed`
    - `auto_stop_machines = 'off'`
  - It does **not** set continuity role/epoch today. Public docs should not imply that Fly is the destructive two-cluster failover authority.

### Existing verifier rails to reuse

- **`scripts/verify-m043-s03.sh`**
  - This is the packaged local authority for M043.
  - It already replays prerequisites, runs the same-image Docker failover target, checks the entrypoint misconfig probe, copies the retained bundle, and fail-closes on stale-primary drift.
  - It produces the authoritative retained state:
    - `.tmp/m043-s03/verify/status.txt`
    - `.tmp/m043-s03/verify/current-phase.txt`
    - `.tmp/m043-s03/verify/phase-report.txt`
    - `.tmp/m043-s03/verify/05-same-image-artifacts/...`
  - The copied artifact bundle verifies the concrete truth sequence:
    - pre-failover primary/standby membership
    - degraded standby after primary loss
    - promotion response and promoted membership
    - failover retry rollover (`attempt-0` -> `attempt-1`)
    - post-rejoin primary deposed as standby
    - stale-primary guard still returning promoted-standby truth

- **`compiler/meshc/tests/e2e_m043_s03.rs`**
  - Defines the retained artifact names and the exact failover story the docs should describe:
    - `promoted-membership-standby.json`
    - `post-rejoin-primary-status.json`
    - `post-rejoin-standby-status.json`
    - `stale-guard-primary.json`
    - `scenario-meta.json`
  - If public docs invent different state names than these artifacts prove, they will drift from the real authority.

- **`scripts/verify-m042-s04-proof-surface.sh`**
  - Good structural template for the new M043 proof-surface verifier.
  - It already knows how to fail-close on docs/README/sidebar/command drift.
  - But it hardcodes M042 commands and wording, so reusing it in place would keep stale truth around.

- **`scripts/verify-m042-s04-fly.sh`**
  - Good structural template for the new M043 read-only Fly verifier.
  - It already checks:
    - `fly status --json` for running machines
    - `fly config show` for the small env contract
    - `fly logs --no-tail`
    - live `GET /membership`
    - optional live `GET /work/:request_key`
  - But its JSON assertions are still M042-shaped:
    - `/membership` only verifies membership/discovery fields, not `cluster_role` / `promotion_epoch` / `replication_health`
    - `/work/:request_key` only verifies keyed status basics, not authority fields

### Public docs currently stale

- **`cluster-proof/README.md`**
  - Still describes the “current authoritative local proof” as `bash scripts/verify-m042-s03.sh`.
  - Still names `bash scripts/verify-m042-s04-fly.sh --help` and `bash scripts/verify-m042-s04-proof-surface.sh`.
  - Still frames the public story as M042 keyed continuity/read-only Fly only.
  - Does not mention `/promote`, explicit promotion boundary, stale-primary fencing, same-image two-cluster authority, or non-goals.

- **`website/docs/docs/distributed-proof/index.md`**
  - Still calls `scripts/verify-m042-s03.sh` the local authority.
  - Still centers the old continuity contract and old script names.
  - Does not mention M043 role/epoch/health truth or explicit promotion.

- **`website/docs/docs/distributed/index.md`** and **`README.md`**
  - Currently only route readers to the stale M042-shaped proof page/runbook summary.
  - These are routing surfaces, not the deepest contract, but they still need wording updates so they do not advertise the old shape.

- **`website/docs/.vitepress/config.mts`**
  - Sidebar already has a `Distributed Proof` entry.
  - No structural gap here unless the page title/path changes. Likely no code change needed.

## Natural Seams / Build Order

1. **Create the M043 proof-surface verifier first.**
   - This gives a fail-closed target for the docs/runbook edits.
   - Reuse the structure of `scripts/verify-m042-s04-proof-surface.sh`, but swap in the M043 command list and wording contract.
   - This is the fastest way to stop the slice from devolving into ad hoc markdown edits.

2. **Create the M043 read-only Fly verifier second.**
   - Fork `scripts/verify-m042-s04-fly.sh`.
   - Keep the read-only command set.
   - Upgrade live assertions so `/membership` and optional `/work/:request_key` must expose the authority fields.
   - If helpful, also assert the retained Fly log contains the runtime-authority log line from `cluster-proof/main.mpl`, but keep live expectations generic enough to accept primary or standby states.

3. **Update the runbook and public docs together.**
   - `cluster-proof/README.md`
   - `website/docs/docs/distributed-proof/index.md`
   - `website/docs/docs/distributed/index.md`
   - `README.md`
   - Keep the canonical command list mirrored between the proof page and runbook, just like the old M042 verifier already enforced.

4. **Only touch VitePress/sidebar if the page title/path changes.**
   - Right now the sidebar wiring already exists.

## Constraints

- **R052 small-env rule is active.**
  The public operator story must stay “same image + small env surface.” The M043 docs should explain `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH` for the same-image local rail, but should not widen the operator contract into a new orchestration layer.

- **R053 is the primary slice contract.**
  Update verifier, docs, runbook, and README together. Partial updates are exactly the failure mode S04 exists to prevent.

- **Fly remains read-only evidence, not destructive authority.**
  The `flyio-cli-public` skill matches the repo’s intended stance here: prefer read-only Fly commands first, and do not add deploy/restart/scale/secret mutations to the verifier.

- **Do not overclaim automatic failover.**
  Current code exposes explicit `/promote`. Public surfaces should say promotion is an operator-triggered boundary, not something that happens automatically because a peer disappeared.

- **Post-rejoin `replication_health` is timing-sensitive.**
  S03 already established that the promoted standby may report either `local_only` or `healthy` after rejoin. Public/verifier wording should stay strict on role/epoch/fencing truth, not claim one fixed post-rejoin health string as the only honest state.

- **No runtime change should be required.**
  If work starts drifting into `mesh-rt` or continuity merge logic, the slice probably lost scope.

## Common Pitfalls

- **Updating docs but keeping M042 verifier names.**
  The current proof page and runbook hardcode `verify-m042-*`. If S04 only edits prose, the next verifier run will keep the repo publicly stale.

- **Reusing the M042 Fly verifier unchanged.**
  `scripts/verify-m042-s04-fly.sh` checks the old JSON shape only. That is not enough for S04 because it ignores `cluster_role`, `promotion_epoch`, and `replication_health`.

- **Describing Fly as the destructive failover proof.**
  The milestone explicitly keeps live destructive Fly failover out of scope. Public docs should route destructive authority to `bash scripts/verify-m043-s03.sh`, not to Fly.

- **Explaining M043 only in request-fence terms.**
  `attempt_id` still matters, but it is not the full authority story anymore. The public contract must also name the authority fence:
  - cluster role
  - promotion epoch
  - stale-primary fencing after promotion

- **Forgetting `/promote` in the public surface.**
  `cluster-proof/main.mpl` already mounts it, and `work_continuity.mpl` already returns a concrete authority payload. Leaving it out of docs would keep the promotion boundary implicit, which is exactly what the milestone wants to avoid.

- **Broadening the topology claims.**
  Docs should stay on the narrow first-wave contract:
  - one active primary cluster
  - one live standby cluster
  - same image
  - explicit promotion
  - no active-active intake
  - no arbitrary app-state replication beyond runtime-owned continuity records

## Verification Approach

Primary S04 gates should be:

```bash
bash scripts/verify-m043-s04-proof-surface.sh
bash scripts/verify-m043-s04-fly.sh --help
```

If a real deployed Fly app exists and live read-only inspection is desired, the contract should remain:

```bash
CLUSTER_PROOF_FLY_APP=<fly-app> \
CLUSTER_PROOF_BASE_URL=https://<fly-app>.fly.dev \
[CLUSTER_PROOF_REQUEST_KEY=<existing-request-key>] \
  bash scripts/verify-m043-s04-fly.sh
```

The authoritative destructive local rail should stay named in docs and should be rerunnable when needed:

```bash
bash scripts/verify-m043-s03.sh
```

If S04 edits website docs, an additional docs smoke is reasonable after the proof-surface script passes:

```bash
npm --prefix website run build
```

Run website install/build **serially**, not in parallel, per existing repo knowledge.

## Skills Discovered

| Technology | Skill | Status |
|---|---|---|
| Fly.io operator/status checks | `flyio-cli-public` | available |
| Distributed failover/fencing semantics | `distributed-systems` | installed during research (`yonatangross/orchestkit@distributed-systems`) |
| VitePress docs site | `vitepress` | available |
