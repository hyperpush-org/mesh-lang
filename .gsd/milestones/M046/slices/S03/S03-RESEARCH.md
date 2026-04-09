# M046/S03 Research — `tiny-cluster/` local no-HTTP proof

## Requirements targeted

- **Primary:** R088 — create a real local `tiny-cluster/` package as the smallest route-free clustered proof surface.
- **Primary:** R092 — keep the proof route-free: no `/work`, no `/status`, no `/health`, and no app-owned submit/status contract.
- **Primary:** R093 — keep the workload visibly trivial (`1 + 1`) so any orchestration/failover complexity is obviously Mesh-owned.
- **Supports:** R091 — prove that `meshc cluster status|continuity|diagnostics` are sufficient to discover, inspect, and debug the local proof package.
- **Supports later:** R090 — landing a real third repo-owned example surface is a prerequisite for later scaffold/`cluster-proof`/`tiny-cluster` alignment, but S03 should not spend the public alignment work early.
- **Prereq already solved; consume, do not reopen:** R086 and R087. S02 already moved startup triggering and startup-state inspection onto runtime/tooling-owned surfaces. S03 should reuse that contract rather than inventing another trigger seam.
- **Not this slice unless the plan deliberately broadens scope:** R089 (`cluster-proof/` rebuild) and public docs-wide equal-surface enforcement.

## Skills Discovered

- Loaded installed skill: **`rust-best-practices`**.
  - Relevant rules here:
    - keep new Rust-side helper/verifier code explicit and boring; prefer `Result`-style failures and narrow helpers over clever abstractions;
    - comments should explain *why* a wait window, artifact copy, or allowed-status set exists, not restate the code;
    - tests should stay behavior-specific and fail legibly.
- Installed and loaded external skill: **`rust-testing`** (`affaan-m/everything-claude-code@rust-testing`).
  - Relevant rules here:
    - keep integration tests in `compiler/meshc/tests/` with shared helpers instead of duplicating process/CLI polling code inline;
    - prefer one behavioral contract per named test/filter so the verifier can fail closed on the exact rail that drifted;
    - keep package-level tests small and direct when they prove a source contract faster than a full e2e replay.
- Ran `npx skills find "mesh language runtime cli"`.
  - Results were generic and not more relevant than the existing Rust/testing guidance.
  - **No additional skill installed.**

## Summary

- **`tiny-cluster/` does not exist yet.** The slice is not about discovering hidden package code; it is about promoting an already-proven temporary route-free app shape into a real repo package.
- The strongest existing reference implementation is already in **`compiler/meshc/tests/e2e_m046_s02.rs`**:
  - `tiny_route_free_startup_main_source()` is exactly the desired main surface: only `Node.start_from_env()` plus a bootstrap log.
  - `tiny_route_free_startup_work_source()` is exactly the desired work surface: source-level `clustered(work)` and literal `1 + 1`.
  - `build_tiny_route_free_runtime_project(...)` already proves that a package manifest with **only `[package]`** is enough when the source carries `clustered(work)`.
  - `m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes()` already proves the local two-node startup/dedupe/status story entirely through `meshc cluster ...` with no HTTP routes and no explicit `Continuity.submit_declared_work(...)` call.
- That means S03 is **not primarily new runtime/compiler work**. The runtime-owned startup and CLI inspection contract already exists after S02. The missing deliverables are:
  1. a real repo-owned `tiny-cluster/` package,
  2. a route-free local failover/rejoin proof against that package,
  3. a slice-owned verifier that replays those rails and retains artifacts fail-closed.
- The existing destructive failover rail in **`compiler/meshc/tests/e2e_m045_s03.rs`** is still useful, but only in parts:
  - reusable: CLI polling, JSON parsing, artifact retention, `automatic_promotion` / `automatic_recovery` / `fenced_rejoin` assertions;
  - not reusable as-is: `/health` readiness, `/work/:request_key` submission, pre-kill request search through HTTP.
- The local proof package should be **source-first**, not manifest-first:
  - S02’s tiny fixture already proves source `clustered(work)` with a plain package manifest works.
  - Keeping `tiny-cluster/mesh.toml` free of `[cluster].declarations` makes the package visibly different from the older scaffold and keeps S03 aligned with D229/D230’s “lead with the decorator/source form” intent.
- Current public clustered surfaces are still routeful by design:
  - `compiler/mesh-pkg/src/scaffold.rs` still generates `/health` + `/work/:request_key` and explicit `Continuity.submit_declared_work(...)`.
  - `website/docs/docs/getting-started/clustered-example/index.md` still teaches that scaffold-first routeful submit flow.
  - `cluster-proof/` remains the deeper HTTP/operator proof package.
  - **S03 should not try to realign all of that yet.** A local `tiny-cluster/README.md` is reasonable; broad public docs alignment belongs to S05/S06.

## Recommendation

### Recommended package shape

Promote the S02 temp fixture almost verbatim into a real repo package:

```text
tiny-cluster/
  mesh.toml
  main.mpl
  work.mpl
  README.md
  tests/
    work.test.mpl   # recommended, not strictly mandatory
```

Recommended content:

- `mesh.toml`: **package-only** manifest (`[package]`), no `[cluster]`, no declarations.
- `main.mpl`: only `Node.start_from_env()` and a small bootstrap log.
- `work.mpl`: source `clustered(work)` on `execute_declared_work(...)`, with default return `1 + 1`.
- `README.md`: local-only runbook for the two-node route-free proof and CLI inspection commands.
- `tests/work.test.mpl`: fast package rail asserting the declared work returns `2` and any optional delay default is inert.

### Recommended failover strategy

Do **not** invent a submit route or a synthetic app control path just to make failover easy.

The natural route-free proof is:

1. start primary and standby;
2. use `meshc cluster continuity <node> --json` **list mode** to discover the deterministic startup record by `declared_handler_runtime_name == "Work.execute_declared_work"`;
3. wait until that record reaches a stable pre-kill pending state (`phase=submitted`, `result=pending`, `replica_status in {preparing, mirrored}`);
4. kill the owner;
5. use only `meshc cluster status|continuity|diagnostics` to prove promotion, recovery, completion, and stale-primary fencing on rejoin.

That keeps the proof fully inside the runtime/tooling contract S02 already established.

### Recommended timing compromise

The one real design choice in S03 is **how to keep the startup work observable long enough for the destructive failover rail**.

Default `1 + 1` is too fast for a meaningful owner-loss window. The least dishonest option is the same pattern the old scaffold used:

- keep the default work semantically trivial (`1 + 1`),
- but allow a **package-local env-controlled delay** around it for the destructive failover harness.

That preserves R093’s “trivial workload” while still making pending/mirrored state observable. Keep the default no-delay path fast, and keep the delay out of the public runtime contract.

### Recommended scope discipline

S03 should build the real local proof surface, not spend the later alignment work early.

Concretely:

- yes: create `tiny-cluster/`, its package tests, real e2e rails, and a verifier script;
- yes: add a repo-local README/runbook for `tiny-cluster/`;
- no: rewrite scaffold generation now;
- no: rewrite public website docs now unless the package is otherwise undiscoverable;
- no: touch `cluster-proof/` yet beyond whatever a shared helper refactor would absolutely require.

## Implementation Landscape

### 1. The target app already exists as temp source in `e2e_m046_s02`

**File:** `compiler/meshc/tests/e2e_m046_s02.rs`

Most relevant functions:

- `package_manifest(name)` — plain `[package]` manifest; no `[cluster]`
- `tiny_route_free_startup_main_source()` — route-free `main.mpl`
- `tiny_route_free_startup_work_source()` — source `clustered(work)` + `1 + 1`
- `build_tiny_route_free_runtime_project(...)` — temp package builder
- `m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes()` — end-to-end route-free startup proof

What this gives S03:

- the desired Mesh source shape is already proven;
- source-declared clustered work already compiles and auto-registers without manifest declarations;
- the two-node startup/dedupe/inspection story is already green in Rust e2e form.

What is missing:

- a real repo package directory;
- any proof of owner-loss/rejoin on that route-free startup record;
- any package-level smoke tests or verifier wrapper.

### 2. CLI/runtime inspection surfaces are already sufficient

**Files:**
- `compiler/meshc/src/cluster.rs`
- `compiler/mesh-rt/src/dist/operator.rs`
- `compiler/mesh-rt/src/dist/continuity.rs`

Important surfaced fields already available to the harness:

- `meshc cluster status --json`
  - `membership.local_node`
  - `membership.peer_nodes`
  - `membership.nodes`
  - `authority.cluster_role`
  - `authority.promotion_epoch`
  - `authority.replication_health`
- `meshc cluster continuity --json`
  - list mode: `records[]`, `total_records`, `truncated`
  - single-record mode: `record.request_key`, `attempt_id`, `phase`, `result`, `owner_node`, `replica_node`, `execution_node`, `declared_handler_runtime_name`, `replica_status`, `cluster_role`, `promotion_epoch`, `replication_health`, `error`
- `meshc cluster diagnostics --json`
  - startup transitions already used in S02: `startup_trigger`, `startup_completed`, `startup_skipped`
  - failover transitions already used in M045 rails: `automatic_promotion`, `automatic_recovery`, `recovery_rollover`, `fenced_rejoin`

Implication: S03 should not need new runtime or CLI JSON fields unless the real repo package exposes a previously hidden gap.

### 3. The failover harness already exists, but it is HTTP-shaped

**File:** `compiler/meshc/tests/e2e_m045_s03.rs`

Reusable helpers:

- lifecycle/process helpers:
  - `ensure_mesh_rt_staticlib()`
  - `dual_stack_cluster_port()`
  - `artifact_dir(...)`
  - `write_artifact(...)`
  - `run_meshc_cluster_json(...)`
  - `wait_for_status(...)`
  - `wait_for_continuity(...)`
  - `wait_for_diagnostics(...)`
- assertion helpers:
  - `status_matches(...)`
  - `prekill_pending_matches(...)`
  - `recovered_pending_matches(...)`
  - `completed_matches(...)`
  - `has_automatic_promotion(...)`
  - `automatic_recovery_attempt_id(...)`
  - `has_automatic_recovery(...)`
  - `has_fenced_rejoin(...)`

Helpers to **leave behind** for S03:

- `wait_for_health(...)`
- `post_work(...)`
- `find_pending_primary_owned_request(...)`
- any `/health` or `/work` HTTP parsing

The main adaptation is selection: S03 does not need a brute-force request search anymore because the runtime-owned startup record is deterministic and discoverable by runtime name.

### 4. Existing package-test patterns are simple and reusable

**Example file:** `cluster-proof/tests/work.test.mpl`

The repo already uses package-level Mesh tests to lock small source contracts separately from Rust e2e rails. For S03, that suggests a clean split:

- Mesh package tests prove the source contract (`clustered(work)` target exists, work returns `2`, optional delay default is harmless);
- Rust e2e proves the real clustered startup/failover/rejoin behavior;
- shell verifier replays the named rails and retains timestamped artifacts.

### 5. Verifier patterns exist, but nested wrapper replays are a trap

**Files:**
- `scripts/verify-m045-s02.sh`
- `scripts/verify-m045-s03.sh`
- project knowledge notes

Useful verifier patterns:

- `run_expect_success(...)`
- `assert_test_filter_ran(...)`
- copy-new-artifacts snapshot/diff pattern
- retained bundle shape checks
- `status.txt` / `current-phase.txt` / `phase-report.txt` fail-closed bookkeeping

Important repo knowledge to carry forward:

- nested verifier wrappers that transitively replay older rails can hang after leaf commands have already passed;
- assembled verifiers in this repo should prefer **direct prerequisite commands** over calling older wrappers when the older wrappers shell into more long-running package/test steps.

Implication: `scripts/verify-m046-s03.sh` should probably replay the direct prerequisites it needs instead of shelling through older M045 or M046 wrappers.

## Natural Task Seams

### Seam A — Real package creation

**Likely files:**
- `tiny-cluster/mesh.toml`
- `tiny-cluster/main.mpl`
- `tiny-cluster/work.mpl`
- `tiny-cluster/README.md`
- optionally `tiny-cluster/tests/work.test.mpl`

**Deliverable:**
- real route-free package in the repo;
- source-declared clustered work via `clustered(work)`;
- default workload remains `1 + 1`;
- no HTTP routes, no explicit continuity submit/status logic, no cluster-proof env names.

### Seam B — Rust e2e startup/package contract

**Likely file:**
- new `compiler/meshc/tests/e2e_m046_s03.rs`

**Deliverable:**
- prove the real repo package has the same contract S02’s temp fixture had:
  - package manifest has no `[cluster]` declarations,
  - `main.mpl` only uses `Node.start_from_env()`,
  - `work.mpl` uses source `clustered(work)` and trivial arithmetic,
  - `meshc build tiny-cluster` succeeds,
  - `meshc cluster continuity` discovers the startup work by runtime name on both nodes,
  - completion is visible through runtime-owned surfaces only.

This can reuse S02 helper logic almost directly.

### Seam C — Rust e2e failover/rejoin rail

**Likely file:**
- same `compiler/meshc/tests/e2e_m046_s03.rs`

**Deliverable:**
- route-free owner-loss/recovery/rejoin proof against `tiny-cluster/`:
  - pending startup record discovered by CLI list mode,
  - primary kill during pending window,
  - standby promotion + automatic recovery,
  - completion on promoted standby,
  - stale-primary rejoin fenced,
  - retained `.tmp/m046-s03/...` artifacts for debugging.

This should reuse M045’s CLI wait/assert helpers, not its HTTP helpers.

### Seam D — Verifier wrapper

**Likely file:**
- `scripts/verify-m046-s03.sh`

**Deliverable:**
- fail-closed replay of the real local proof surface;
- direct prerequisite commands (not nested wrappers);
- named test filters must prove `running N test` with `N > 0`;
- fresh `.tmp/m046-s03/...` e2e artifacts copied into verifier-owned retained bundle.

## Risks / Unknowns

### 1. Pending-window observability

The biggest real unknown is not placement or CLI semantics; it is **timing**.

A startup work item that just returns `1 + 1` will often complete before a destructive failover harness can kill the owner. Without an observable pending window, S03 can prove route-free startup, but not route-free failover.

Most likely honest fix: package-local env-controlled delay around the `1 + 1` work path, defaulting to zero.

### 2. Health assertions need to stay runtime-owned

The old failover rail relies on `/health` to decide when scaffold nodes are “up.” `tiny-cluster/` will have no HTTP surface, so readiness must come from process liveness plus `meshc cluster status` convergence. Do not accidentally smuggle a health route back into the app just to simplify the test.

### 3. Allowed replication-health states are broader than a single happy value

Repo knowledge already says:

- before a continuity record exists, two-node status can still truthfully report `local_only`;
- after rejoin, the promoted standby can settle as either `local_only` or `healthy` while the real ownership/fencing proof is unchanged.

S03 assertions should therefore be strict on node ownership/epoch/fencing fields, but tolerant on those specific health values where the runtime already behaves that way.

### 4. Real package drift vs temp-fixture drift

Because S02’s temp fixture is already green, the most likely S03 failures are not runtime failures but **package drift**:

- accidental reintroduction of `[cluster]` manifest declarations,
- accidental `HTTP` imports or `/work` strings,
- accidental explicit `Continuity.submit_declared_work(...)` or `Continuity.mark_completed(...)`,
- README/public contract drift back toward scaffold/cluster-proof language.

Add file-content contract assertions early so these fail fast.

### 5. Verifier nesting/hanging

Do not make `verify-m046-s03.sh` a thin shell around older M045/M046 wrappers. Project knowledge already shows nested wrapper replays can hang after leaf phases are green. Direct commands plus retained copied artifacts are safer.

## Don’t Hand-Roll

- Do **not** add `/health`, `/work`, `/status`, or any other HTTP surface to `tiny-cluster/`.
- Do **not** call `Continuity.submit_declared_work(...)` or `Continuity.mark_completed(...)` from app code.
- Do **not** reintroduce `[cluster].declarations` into `tiny-cluster/mesh.toml` if the point of the package is to prove source `clustered(work)` is enough.
- Do **not** re-derive placement or request-key ownership in the test harness. Discover the runtime-owned startup record through `meshc cluster continuity` list mode and use the returned record as truth.
- Do **not** copy the scaffold failover harness wholesale; port only the CLI/process helpers and drop the HTTP-only pieces.
- Do **not** spend the public docs/scaffold alignment work here unless the plan explicitly broadens. S05 already owns that surface.

## Verification Plan

Recommended acceptance stack:

1. **Protect the consumed S02 contract first**
   - `cargo build -q -p mesh-rt`
   - `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_tiny_route_free_startup_dedupes_on_two_nodes -- --nocapture`

2. **Package-local source/build smoke**
   - `cargo run -q -p meshc -- build tiny-cluster`
   - `cargo run -q -p meshc -- test tiny-cluster/tests` *(recommended if the package tests are added)*

3. **New S03 e2e rail**
   - `cargo test -p meshc --test e2e_m046_s03 m046_s03_ -- --nocapture`

4. **Verifier wrapper**
   - `bash scripts/verify-m046-s03.sh`

What the new e2e/verifier should explicitly prove:

- `tiny-cluster/` contains no HTTP routes or explicit continuity submit/status code;
- the startup record is discoverable from CLI list mode by `declared_handler_runtime_name`;
- the two-node route-free startup run completes on both nodes with one logical record;
- destructive owner-loss promotes the standby and recovers the same startup record without a second submit;
- stale-primary rejoin is fenced and does not re-execute locally;
- named test filters run **> 0 tests** and artifacts are retained under `.tmp/m046-s03/...`.

## Resume Notes

- There is currently **no `tiny-cluster/` directory** in the repo.
- The exact app surface the package should start from is already in `compiler/meshc/tests/e2e_m046_s02.rs`:
  - `tiny_route_free_startup_main_source()`
  - `tiny_route_free_startup_work_source()`
- The most reusable helper surface for S03 is split across two files:
  - startup/package contract helpers from `compiler/meshc/tests/e2e_m046_s02.rs`
  - failover/rejoin CLI wait/assert helpers from `compiler/meshc/tests/e2e_m045_s03.rs`
- Use the established local dual-stack node pattern for honest two-node local proof:
  - primary advertised on `127.0.0.1`
  - standby advertised on `::1`
  - same cluster port, shared cookie/discovery seed
- Keep assertions strict on `cluster_role`, `promotion_epoch`, request ownership, `execution_node`, and `fenced_rejoin`; allow `replication_health` to be `local_only` or `healthy` where the current runtime already does that.
- If failover cannot catch a pending startup record without artificial help, add the smallest package-local env-controlled delay around `1 + 1` instead of reopening the runtime startup contract.
