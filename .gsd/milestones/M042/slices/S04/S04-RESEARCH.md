# S04 Research — Thin cluster-proof consumer and truthful operator/docs rail

## Summary

S04 is mostly an **integration/truthfulness slice**, not another runtime-semantics slice.

The runtime-native continuity work is already landed:
- `mesh-rt` owns keyed continuity state, replica prepare/ack, owner-loss marking, retry rollover, and stale-completion fencing.
- the compiler already exposes a small `Continuity` module (`submit`, `status`, `mark_completed`, `acknowledge_replica`).
- `cluster-proof` no longer owns the continuity algorithm; it is already a consumer that does placement, HTTP translation, and work dispatch.

The actual gap is that the **operator-facing shell/docs rail still speaks in M039 terms**:
- the local/Fly verifiers are still hard-coded to `/membership` + legacy `GET /work` routing claims
- `cluster-proof/README.md` and `website/docs/docs/distributed-proof/index.md` omit `POST /work` and `GET /work/:request_key`
- the generic distributed guide documents `Node` and `Global`, but not `Continuity`
- the proof-surface verifier still enforces the old wording and old command set

So the planner should treat S04 as:
1. make `cluster-proof` visibly thin in code organization,
2. preserve the existing one-image Docker/Fly rail,
3. add/update verifiers and docs so the public claim matches the runtime-owned continuity truth.

## Requirements Focus

Primary slice ownership:
- **R052** — preserve the one-image Docker/Fly/operator rail while the implementation boundary lives in `mesh-rt`
- **R053** — keep the proof/docs surface honest now that runtime continuity replaced app-authored continuity machinery

Supported by this slice’s documentation/proof language:
- **R050** — must be described truthfully as owner-loss recovery via same-key retry rollover and attempt fencing, not as exactly-once or process-state migration

## Skills Discovered

Directly relevant installed skills:
- `rust-best-practices` — useful if any Rust-side helper/docs touch `mesh-rt` or compiler API comments
- `flyio-cli-public` — relevant because the Fly verifier must remain read-only and should not drift into deploy/mutate behavior
- `vitepress` — relevant because the public proof surface is under `website/docs/` and sidebar wiring lives in `.vitepress/config.mts`
- `distributed-systems` — newly installed and directly relevant to the doc contract around **idempotency keys** and **fencing-token semantics**

Skill-informed constraints that matter here:
- From `flyio-cli-public`: prefer read-only Fly operations first; do not turn the live proof rail into a mutating deploy/scale/secrets workflow.
- From `distributed-systems`: the public story must stay explicit that `request_key` is the idempotency key and `attempt_id` is the fencing token. Do **not** describe this as magical migration or exactly-once failover.
- From `vitepress`: proof-page work is file-routed Markdown plus explicit sidebar wiring in `website/docs/.vitepress/config.mts`; the doc verification script should check those concrete files, not inferred site behavior.

## What Exists Now

### Runtime-owned continuity substrate

These files already define the real capability:
- `compiler/mesh-rt/src/dist/continuity.rs`
  - owns keyed records, dedupe/conflict/rejection, replica mirroring, `owner_lost`, retry rollover, stale-completion fencing, snapshot/upsert merge precedence
- `compiler/mesh-rt/src/dist/node.rs`
  - owns continuity wire tags, replica prepare/ack transport, disconnect hooks that mark `owner_lost` / degraded records, and the owner-loss recovery-eligibility check
- `compiler/mesh-rt/src/lib.rs`
  - exports the continuity runtime symbols
- `compiler/mesh-typeck/src/infer.rs`
  - registers the `Continuity` stdlib module with four functions
- `compiler/mesh-codegen/src/mir/lower.rs`
  - lowers those module calls to the runtime intrinsics
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
  - declares the runtime externs

Important planner point: **there is no missing API plumbing**. S04 does not need a new language surface unless docs reveal a naming/ergonomics issue.

### `cluster-proof` is already a consumer, but not visibly so

Relevant files:
- `cluster-proof/work.mpl`
  - calls `Continuity.submit/status/mark_completed`
  - no longer owns replica prepare/ack choreography or node-loss recovery logic
  - still owns deterministic placement (`route_selection`), HTTP payload shaping, and legacy `GET /work` probe behavior
  - mixes two concerns in one file: the old routing probe and the keyed continuity submit/status rail
- `cluster-proof/main.mpl`
  - mounts `/membership`, `GET /work`, `POST /work`, `GET /work/:request_key`
- `cluster-proof/Cluster.mpl`
  - still owns deterministic canonical membership/placement used by the consumer app
- `cluster-proof/Config.mpl`
  - owns the small-env operator contract: standalone => `local-only`, cluster => `replica-backed`
- `cluster-proof/docker-entrypoint.sh`
  - fail-closed env gate for standalone vs cluster mode and durability policy defaults
- `cluster-proof/tests/work.test.mpl`
  - covers placement, request parsing, response helpers, and policy distinctions
- `cluster-proof/tests/config.test.mpl`
  - covers env/durability/operator defaults

Planner implication: the likely code cleanup seam is **organizational**, not semantic. `work.mpl` is the file where S04 can make the app *look* like a thin runtime consumer.

### Existing proof surfaces

Current local/runtime proof surfaces already exist:
- `compiler/meshc/tests/e2e_m042_s01.rs`
- `compiler/meshc/tests/e2e_m042_s02.rs`
- `compiler/meshc/tests/e2e_m042_s03.rs`
- `scripts/verify-m042-s01.sh`
- `scripts/verify-m042-s02.sh`
- `scripts/verify-m042-s03.sh`

These are the authoritative local proofs for keyed continuity behavior.

Current operator/docs rail is still M039-shaped:
- `scripts/lib/m039_cluster_proof.sh`
  - shared shell assertions only know `/membership` and legacy `GET /work`
- `scripts/verify-m039-s04.sh`
  - local Docker proof checks remote `/work`, degraded local fallback, and restored remote routing
- `scripts/verify-m039-s04-fly.sh`
  - read-only Fly proof checks config, logs, `/membership`, `/work`
- `scripts/verify-m039-s04-proof-surface.sh`
  - enforces proof-page wording and command list for the M039 rail only
- `cluster-proof/README.md`
  - operator runbook still documents only `GET /membership` + `GET /work`
- `website/docs/docs/distributed-proof/index.md`
  - public proof page still describes only the routing proof
- `website/docs/docs/distributed/index.md`
  - generic distributed guide still documents `Node` / `Global`, but not `Continuity`
- `README.md`
  - top-level distributed-proof section still points to the older operator story

## Gaps That S04 Needs To Close

### 1. The public proof story is behind the code

The public/operator docs currently describe the M039 rail, not the actual M042 capability.

Specific drift:
- no public mention of `POST /work`
- no public mention of `GET /work/:request_key`
- no public mention that cluster mode defaults to `replica-backed`
- no public mention that owner loss becomes `owner_lost` status and same-key retry rolls a new `attempt_id`
- no public mention of the actual `Continuity` module API anywhere in docs

### 2. The proof-surface verifier is enforcing stale wording

`scripts/verify-m039-s04-proof-surface.sh` hard-codes phrases like:
- remote `/work` routing when both nodes are healthy
- local `/work` fallback during a degraded one-node window
- old canonical command list using only M039 verifier names

If docs are updated without replacing/extending this verifier, the slice will fail closed.

### 3. Live Fly verification must stay honest about what it can prove

Important constraint: the current Fly verifier is intentionally read-only.

That means it is a good fit for:
- deployed app/config truth
- membership truth
- repo-root Docker/Fly build contract truth
- live routing sanity through `GET /work`

It is **not** a good fit for destructive keyed continuity scenarios that require `POST /work`, retry rollover, or owner-loss mutation.

Planner implication: the slice should not pretend the live Fly rail proves the full M042 destructive continuity contract. The honest split is:
- **local destructive authority** for keyed continuity behavior
- **read-only Fly authority** for the one-image operator path and deployed-surface sanity

### 4. `cluster-proof/work.mpl` still hides the thin-consumer shape

The runtime owns continuity semantics already, but the file layout obscures that because `work.mpl` still mixes:
- legacy routing probe support
- keyed continuity HTTP adapter code
- placement helpers
- work execution helpers
- log formatting

A planner should expect at least one code-organization task here even if semantics stay unchanged.

## Recommendation

Use a **three-seam plan**.

### Seam A — make `cluster-proof` visibly thin

Target files:
- `cluster-proof/work.mpl`
- possibly new sibling Mesh modules under `cluster-proof/` if refactoring is worthwhile
- `cluster-proof/tests/work.test.mpl`

Recommended outcome:
- separate the legacy `GET /work` routing probe from the keyed continuity submit/status adapter
- keep placement and HTTP translation in Mesh code, but make it obvious that runtime continuity is the owner of state transitions and recovery truth
- do **not** widen the runtime API or move placement into Rust in this slice; that is beyond the operator/docs closeout scope

Good stopping point: a reader can open the consumer module and immediately see that the app is just:
1. pick owner/replica,
2. call `Continuity.*`,
3. dispatch work,
4. render HTTP/log output.

### Seam B — add or extend the operator verification rail without rewriting M039 history

Recommended approach:
- keep `scripts/verify-m039-s04.sh` and `scripts/verify-m039-s04-fly.sh` as the validated M039 baseline
- add an **M042 S04 wrapper** (or pair of wrappers) that reuses the same repo-root image / two-container / Fly rail but adds the runtime-native continuity truth checks
- keep the Fly lane read-only

Why this is safer:
- M039 stays a readable baseline
- the new slice can fail closed on the updated proof contract without mutating the earlier acceptance story out from under it
- docs can describe the relationship directly: M039 route proof preserved, M042 continuity proof added

Natural implementation options:
- new `scripts/lib/m042_cluster_proof.sh` with JSON assertions for keyed submit/status payloads
- or extend `scripts/lib/m039_cluster_proof.sh` carefully with generic helpers and keep callers separate

I would avoid replacing the M039 wrapper wholesale unless the planner has a very strong reason.

### Seam C — make the docs truthful in the right places

Primary doc files:
- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `website/docs/.vitepress/config.mts` only if navigation changes
- proof-surface verifier script(s)

Recommended doc shape:
- `cluster-proof/README.md`: deepest operator runbook, now with both legacy `GET /work` probe and keyed continuity endpoints/behavior
- `website/docs/docs/distributed-proof/index.md`: public proof map, explicitly split into
  - one-image operator rail,
  - local destructive keyed continuity authority,
  - read-only Fly sanity authority
- `website/docs/docs/distributed/index.md`: add a `Continuity` section and API-table rows so the runtime-facing API is actually documented where `Node` and `Global` already live
- `README.md`: short pointer text only; do not over-explain here

I would **not** put the main Continuity docs in `website/docs/docs/stdlib/index.md`. That page is currently a narrow utility-module page, while `Continuity` belongs conceptually with distributed runtime primitives.

## Constraints / Truths To Preserve

- Do not describe M042 as exactly-once execution.
- Do not describe owner loss as arbitrary process-state migration.
- Keep the stable wording that recovery is **same-key retry** with a newer `attempt_id` fence.
- Keep the Fly verifier read-only unless the user explicitly approves a different external-state policy.
- Preserve the repo-root Docker/Fly build context contract (`cluster-proof/Dockerfile` still depends on repo root).
- Preserve the small-env operator contract from `Config.mpl` / `docker-entrypoint.sh`: standalone defaults to `local-only`, cluster defaults to `replica-backed`.

## Verification Plan

Existing prerequisite commands that should stay green:
- `cargo test -p mesh-rt continuity -- --nocapture`
- `cargo run -q -p meshc -- test cluster-proof/tests`
- `cargo test -p meshc --test e2e_m042_s03 -- --nocapture`
- `bash scripts/verify-m042-s02.sh`
- `bash scripts/verify-m042-s03.sh`

Slice-specific verification should likely be layered like this:

1. **Consumer/code organization**
   - `cargo run -q -p meshc -- test cluster-proof/tests`
   - `cargo run -q -p meshc -- build cluster-proof`

2. **Local packaged operator rail (destructive authority)**
   - repo-root Docker image build
   - two-container run
   - `/membership` still converges truthfully
   - `POST /work` returns keyed continuity JSON with runtime-owned fields
   - `GET /work/:request_key` returns truthful keyed status
   - if the planner can afford it, one degraded-status or retry-rollover packaged proof; otherwise explicitly leave destructive recovery authority with the existing Rust e2e + `verify-m042-s03.sh`

3. **Read-only Fly operator rail**
   - preserve existing config/membership/routing sanity checks
   - do not claim that this lane alone proves destructive keyed continuity

4. **Docs truth**
   - proof-surface verifier script for the updated page/runbook wording
   - `npm --prefix website run build` serially if docs text or nav changes

## Natural Task Boundaries

### Task 1 — Thin-consumer code shape
Files:
- `cluster-proof/work.mpl`
- maybe new `cluster-proof/*.mpl` helpers
- `cluster-proof/tests/work.test.mpl`

Goal:
- refactor for visibility, not new semantics

### Task 2 — Updated operator/verifier rail
Files:
- new/updated `scripts/lib/*cluster_proof*.sh`
- new `scripts/verify-m042-s04*.sh` wrappers or carefully extended existing scripts
- maybe `cluster-proof/README.md` command references

Goal:
- keep one-image/operator rail intact while adding truthful continuity checks

### Task 3 — Public docs and proof-surface checks
Files:
- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- proof-surface verifier script
- maybe `website/docs/.vitepress/config.mts` if navigation changes

Goal:
- operator page, runbook, and distributed guide all say the same honest thing

## Recommendation to Planner

Do **not** spend context rediscovering runtime semantics. Treat S04 as a closeout slice over already-landed runtime continuity.

The highest-value order is:
1. make `cluster-proof` visibly thin,
2. design the updated operator proof split (local destructive vs Fly read-only),
3. then rewrite docs and proof-surface verifiers to match that exact split.

If you need to cut scope, cut cosmetic refactors before you cut proof-truth work. The real acceptance bar for S04 is that the repo stops telling the old M039-only story when the code is already on the M042 runtime-owned continuity rail.
