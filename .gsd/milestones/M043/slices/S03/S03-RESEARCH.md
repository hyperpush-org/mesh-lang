# M043 / S03 — Research

## Summary

S03 primarily owns **R052** and supports **R051** while preserving **R053**. I did not find a runtime/compiler gap here. S02 already shipped the hard part: runtime-owned role/epoch truth, `/promote`, recovery rollover, and stale-primary fencing. The missing piece is the **same-image packaged local operator rail** that exercises that contract through Docker and retains the evidence bundle.

The existing packaging stack is already close:

- `cluster-proof/Dockerfile` is already a real one-image, multi-stage build.
- `.dockerignore` already constrains the build context to Cargo/compiler/cluster-proof surfaces.
- `scripts/verify-m042-s04.sh` already manages a local Docker network, container lifecycle, readiness, DNS preflight, and retained artifacts for the older M042 rail.
- `scripts/lib/m043_cluster_proof.sh` already has the JSON/artifact assertions for the new M043 role/epoch/promotion contract.

The drift is in the **operator surface**, not the runtime:

- `cluster-proof/README.md` and `website/docs/docs/distributed-proof/index.md` are still M042-shaped.
- The current manual Docker runbook does **not** mention `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH` or `/promote`.
- `cluster-proof/docker-entrypoint.sh` validates cookie/discovery/identity/durability, but it does **not** validate the new continuity role/epoch envs; that validation currently lives in `cluster-proof/config.mpl` / `main.mpl`.

There is still a clean small-env path for S03: `docker-entrypoint.sh` auto-derives node identity from `HOSTNAME` when cluster mode is requested. That means the packaged verifier can keep the operator-visible env mostly to:

- shared cookie
- shared discovery seed / alias
- per-container `MESH_CONTINUITY_ROLE`
- per-container `MESH_CONTINUITY_PROMOTION_EPOCH`
- optional work delay for observability

No second image is needed, and no explicit `CLUSTER_PROOF_NODE_BASENAME` / `CLUSTER_PROOF_ADVERTISE_HOST` envs are required if the verifier uses `--hostname primary` / `--hostname standby`.

## Recommendation

Treat S03 as a **packaging/orchestration slice**, not a runtime slice.

Start from `scripts/verify-m042-s04.sh`, not from the compiler harness and not from docs.

Recommended implementation shape:

1. **Reuse the existing one-image Docker contract.**
   - Keep `cluster-proof/Dockerfile` as the single image.
   - Keep one local bridge network and two containers.
   - Do not introduce Compose, per-role images, or external control-plane scripts.

2. **Package the S02 failover contract, don’t reinterpret it.**
   - Replay `bash scripts/verify-m043-s02.sh` first.
   - Then run the Docker-packaged destructive verifier that proves the same role/epoch/fencing truth through containerized `cluster-proof`.

3. **Keep authority runtime-owned.**
   The distributed-systems skill is directly relevant here: use the existing runtime fence/idempotency surfaces (`request_key`, `attempt_id`, promotion epoch) as the truth boundary. Do **not** let the shell verifier become the real failover authority by carrying its own cluster state or “fixing” stale nodes.

4. **Preserve the stale-primary proof honestly.**
   When restarting the old primary, use its original stale env (`primary`, epoch `0`) and prove that the runtime demotes it after rejoin. Do not restart it as `standby`, epoch `1`; that would hide the real fence.

5. **Leave README/site/Fly proof-surface updates to S04.**
   S03 should produce the packaged verifier and retained artifact contract that S04 can later document mechanically.

## Implementation Landscape

### Key files

- `cluster-proof/Dockerfile`
  - Existing one-image multi-stage build.
  - Already aligned with the `multi-stage-dockerfile` skill: build in a builder stage, runtime copies only `/tmp/cluster-proof` and the entrypoint.

- `.dockerignore`
  - Keeps Docker build context narrow: repo root Cargo files, `compiler/**`, and `cluster-proof/**`.
  - Supports the current repo-root image contract; no obvious S03 change required.

- `cluster-proof/docker-entrypoint.sh`
  - Shell-level bootstrap and fail-closed validation for cookie/discovery/identity/durability.
  - Important S03 detail: if cluster mode is requested and no explicit identity env is set, it exports `CLUSTER_PROOF_NODE_BASENAME="$HOSTNAME"` and `CLUSTER_PROOF_ADVERTISE_HOST="$HOSTNAME"`.
  - Does **not** currently validate `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH`.

- `cluster-proof/config.mpl`
  - Authoritative continuity topology validator.
  - Cluster mode requires `MESH_CONTINUITY_ROLE`.
  - `standby` requires promotion epoch `0` before promotion.
  - Standalone rejects `MESH_CONTINUITY_ROLE` / `MESH_CONTINUITY_PROMOTION_EPOCH` entirely.
  - Primary may start at a nonzero epoch.

- `cluster-proof/main.mpl`
  - Wires the live operator surfaces:
    - `GET /membership`
    - `POST /promote`
    - `GET /work`
    - `POST /work`
    - `GET /work/:request_key`
  - Uses `runtime_authority_status()` for operator-visible role/epoch/health truth.

- `cluster-proof/work_continuity.mpl`
  - Promotion and keyed-status payload/log translation.
  - Important log surfaces already shipped:
    - `[cluster-proof] continuity promote ...`
    - `[cluster-proof] keyed status ... cluster_role=... promotion_epoch=... replication_health=...`
    - `[cluster-proof] work executed request_key=... attempt_id=... execution=...`

- `cluster-proof/tests/config.test.mpl`
  - Strong config contract coverage for role/epoch/topology validation.
  - Good regression proof if S03 touches config/entrypoint semantics.

- `cluster-proof/tests/work.test.mpl`
  - Covers M043 JSON payload shapes and promotion/fenced-old-primary reporting.
  - Good fast regression proof for any payload/log contract changes.

- `scripts/verify-m042-s04.sh`
  - Existing same-image local Docker rail.
  - Useful skeleton for:
    - image build
    - docker network creation
    - attached container logs
    - readiness checks
    - DNS preflight
    - retained manifests / inspect snapshots
  - But its assertions are M042-shaped (`/work` legacy probe + older keyed continuity contract).

- `scripts/lib/m042_cluster_proof.sh`
  - Has reusable HTTP helpers and M042 keyed JSON assertions.
  - Also has `m042_find_remote_submit()`, but that helper is tuned for **remote-owner** placement, not the M043 primary-owner / standby-replica shape.

- `scripts/lib/m043_cluster_proof.sh`
  - Already has the right M043 primitives:
    - `m043_http_json_request`
    - `m043_copy_artifact_dir`
    - `m043_assert_membership_payload_json`
    - `m043_assert_keyed_payload_json`
  - Natural place to add a generalized deterministic placement-search helper for packaged M043 verification.

- `scripts/verify-m043-s01.sh`
  - Assembles the pre-promotion primary→standby mirrored truth contract.
  - Reusable prerequisite and artifact-contract reference.

- `scripts/verify-m043-s02.sh`
  - Current authoritative local failover contract.
  - Replays prerequisites, runs the destructive compiler harness, copies `07-failover-artifacts/`, and asserts the full role/epoch/rejoin contract.
  - S03 should replay this first, then prove the same story via the Docker image.

- `compiler/meshc/tests/e2e_m043_s01.rs`
  - Contains the clean deterministic placement helper:
    - `placement_score(...)`
    - `placement_tie_breaker(...)`
    - `request_key_matches_placement(...)`
    - `find_submit_matching_placement(...)`
  - This is the cleanest reference for selecting a request key whose owner is the primary and replica is the standby **before** submitting.

- `compiler/meshc/tests/e2e_m043_s02.rs`
  - Canonical phase order for the failover story.
  - Important packaged-proof truth from this file:
    - run1 primary starts as `primary`, epoch `0`
    - standby starts as `standby`, epoch `0`
    - old primary run2 is restarted with the **same stale config as run1**
    - the runtime, not startup env, is what demotes it after rejoin

- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `scripts/verify-m042-s04-proof-surface.sh`
  - Still M042-shaped and therefore S04 material, not S03 execution.

### Natural seams

1. **Docker lifecycle vs proof assertions**
   - `scripts/verify-m042-s04.sh` has a reusable container/network pattern but keeps most helpers local.
   - M043 assertions belong in `scripts/lib/m043_cluster_proof.sh` or a new sibling lib, not inlined everywhere.

2. **Placement search vs HTTP assertion**
   - Packaged proof needs a deterministic request-key chooser for “owner primary, replica standby”.
   - That is a distinct seam from the HTTP status/assertion helpers.

3. **Packaged verifier vs public docs**
   - S03 should build the verifier and artifact contract.
   - S04 should update README/site/Fly proof wording to match it.

4. **Shell fail-fast vs Mesh config validation**
   - `docker-entrypoint.sh` currently handles only part of the config contract.
   - `cluster-proof/config.mpl` / `main.mpl` still own continuity role/epoch validation.
   - Only tighten the shell layer if S03 exposes a concrete operator-UX problem.

### What to build or prove first

1. **Lock the packaged topology and env contract.**
   The smallest honest same-image packaged topology is:
   - one bridge network
   - one shared discovery alias / seed
   - one `primary` container (`MESH_CONTINUITY_ROLE=primary`, epoch `0`)
   - one `standby` container (`MESH_CONTINUITY_ROLE=standby`, epoch `0`)
   - old-primary rejoin uses the original stale primary env again

2. **Add or extract a deterministic placement-search helper.**
   The packaged verifier should not hardcode a request key and should not brute-force blindly via submits when the deterministic placement function already exists in the compiler e2e harness.

3. **Implement `scripts/verify-m043-s03.sh`.**
   It should follow the M042 packaged-rail structure but assert the M043 failover contract.

4. **Only then decide whether `docker-entrypoint.sh` needs earlier continuity-env validation.**
   That is optional and should be driven by observed packaged-verifier pain, not by theory.

5. **Do not spend time on docs/Fly/public proof wording in this slice.**
   Those remain S04.

### Proposed packaged verifier shape

`bash scripts/verify-m043-s03.sh` should likely do this, in order:

1. replay `bash scripts/verify-m043-s02.sh`
2. build image from `cluster-proof/Dockerfile`
3. create isolated bridge network + shared seed alias
4. create/start `primary` and `standby` containers with attached stdout/stderr capture
5. wait for `/membership` on both
6. assert pre-failover membership truth
   - primary role `primary`, epoch `0`, health `local_only`
   - standby role `standby`, epoch `0`, health `local_only`
7. choose a deterministic request key that places owner on primary and replica on standby
8. submit keyed work on primary and assert mirrored pending truth on both nodes
9. kill primary run1 and preserve its logs
10. assert degraded standby truth after primary loss
11. `POST /promote` to standby and assert response + promoted membership/status truth
12. re-submit same request key and assert recovery rollover to a new attempt on promoted standby
13. wait for completion on promoted standby
14. restart old primary with its original stale env and preserve run2 logs
15. assert fenced rejoin truth on both nodes
16. assert stale-primary same-key guard still resolves to promoted standby truth
17. fail if logs show the old primary completed/executed the promoted attempt
18. copy all retained HTTP/log artifacts and manifests into `.tmp/m043-s03/verify/`

### Expected retained artifact surface

S03 should probably mirror the existing verifier pattern:

- `.tmp/m043-s03/verify/phase-report.txt`
- `.tmp/m043-s03/verify/status.txt`
- `.tmp/m043-s03/verify/current-phase.txt`
- `.tmp/m043-s03/verify/full-contract.log`
- copied inspect/manifest/log artifacts per phase

And the retained live failover bundle should preserve these proof points explicitly:

- pre-failover membership on primary and standby
- submit response and pending status on primary and standby
- degraded standby membership/status after primary loss
- promotion response from `/promote`
- promoted standby membership/status
- failover retry pending/completed status
- post-rejoin membership/status on both nodes
- stale-primary same-key guard response
- primary run1 / primary run2 / standby stdout+stderr logs

The stable truth fields to assert are already clear from `scripts/verify-m043-s02.sh`:

- `cluster_role`
- `promotion_epoch`
- `replication_health`
- `attempt_id`
- `owner_node`
- `replica_node`
- `replica_status`

## Constraints

- **R052 small-env same-image contract remains active.**
  Do not turn S03 into multiple images, bespoke role-specific startup wrappers, or a Compose-based orchestration story unless forced by hard evidence.

- **R051’s honest bar is the full failover story, not just startup.**
  The packaged rail must prove mirrored standby truth before failover, explicit promotion, runtime-owned retry rollover, completion on promoted standby, and fenced stale-primary rejoin.

- **R053 is still a guardrail even though docs move to S04.**
  S03 cannot invent a packaged contract that differs from the shipped runtime surfaces. The later docs/verifier updates need to describe the exact same packaged truth.

- **There are no existing compose files.**
  Current operator packaging is shell + docker CLI. Reusing that pattern is lower risk than creating a new orchestration contract for one slice.

- **Docker proof uses hostname-based node identities.**
  The compiler harness uses loopback IP / IPv6 names (`primary@127.0.0.1:4370`, `standby@[::1]:4370`). The Docker rail will instead surface `primary@primary:4370` / `standby@standby:4370`-style names via `HOSTNAME` fallback. Assertions must account for that.

- **Continuity topology validation currently lives in Mesh config/runtime, not the entrypoint shell.**
  A bad `MESH_CONTINUITY_ROLE` / epoch can still fail closed, but the failure surface is later than the shell-level identity/durability checks.

## Common Pitfalls

- **Hardcoding a request key.**
  Placement is deterministic but request-key dependent. The packaged verifier needs the placement search helper, not a guessed key.

- **Restarting the old primary with “fixed” env.**
  That would weaken the fence proof. Restart it stale and prove runtime demotion.

- **Porting the compiler-harness node names directly into Docker assertions.**
  Docker proof should expect hostname identities, not loopback IP identities.

- **Reusing `m042_find_remote_submit()` unchanged.**
  That helper is tuned for remote-owner M042 flow, not the primary-owner / standby-replica M043 flow.

- **Letting the verifier script become the real authority.**
  Per the distributed-systems skill, keep fencing/idempotency/runtime authority in the runtime. The shell script should observe `request_key`, `attempt_id`, role, epoch, and health truth — not invent new coordination state.

- **Reopening the image architecture.**
  Per the multi-stage-dockerfile skill, the current builder/runtime split is already the right pattern. S03 should preserve the single compiled artifact runtime image unless a concrete packaging problem appears.

- **Pulling docs into S03.**
  The docs are stale, but that is S04’s job. S03 should produce the packaged verifier and artifact contract that S04 can wire into the proof surface later.

## Verification Approach

Fast regression surfaces already in place:

- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo run -q -p meshc -- build cluster-proof`

Prerequisite authority that S03 should replay:

- `bash scripts/verify-m043-s02.sh`

New authoritative packaged proof surface:

- `bash scripts/verify-m043-s03.sh`

`verify-m043-s03.sh` should fail closed on at least these conditions:

- prerequisite replay is not green
- Docker image build fails
- either container never serves `/membership`
- deterministic placement search does not find a primary-owner / standby-replica key
- pre-failover mirrored truth is absent
- degraded standby truth after primary loss is absent
- `/promote` does not return `ok=true`, `cluster_role=primary`, `promotion_epoch=1`, `replication_health=unavailable`
- same-key retry after promotion does not roll to a new `attempt_id`
- post-rejoin old primary is not `standby`, epoch `1`
- logs do not contain:
  - `transition=promote`
  - `transition=recovery_rollover`
  - `transition=fenced_rejoin`
- logs show the old primary executed or completed the promoted attempt after rejoin
- retained artifacts / manifests are missing or empty

If S03 changes entrypoint/config validation, keep `cargo run -q -p meshc -- test cluster-proof/tests` as the config contract gate.

## Skills Discovered

- `multi-stage-dockerfile` — available and directly relevant.
  - Relevant rule: keep a builder stage for compilation and a minimal runtime stage that copies only necessary artifacts. Current `cluster-proof/Dockerfile` already matches this; S03 should preserve it.

- `flyio-cli-public` — available but only indirectly relevant here.
  - This becomes more useful in S04 when the public/Fly proof surface is updated. It is not needed for S03’s local destructive authority.

- `distributed-systems` (`yonatangross/orchestkit@distributed-systems`) — installed during this research.
  - Relevant rule: preserve fencing/idempotency boundaries as the real authority surface. For S03 that means the operator rail must observe runtime-owned `request_key`, `attempt_id`, role, epoch, and fence truth instead of re-implementing failover control in shell logic.
