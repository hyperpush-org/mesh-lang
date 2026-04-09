# S05: Supervision, Recovery, and Failure Visibility — Research

## Summary

S05 directly owns **R004** and should also produce real proof inputs for **R008** and **R009** later by making the reference backend’s failure story honest instead of implied.

The slice is riskier than it first looks.

The good news:

- `reference-backend/` already has the right **compiler-facing proof harness** in `compiler/meshc/tests/e2e_reference_backend.rs`.
- The Rust runtime’s supervisor core in `compiler/mesh-rt/src/actor/supervisor.rs` looks materially real, and `cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture` passed with **21 supervisor runtime tests**.
- The existing backend already exposes durable job state (`pending` / `processing` / `processed` / `failed`) and enough log/HTTP/DB seams to extend proof without inventing a new system.

The bad news is the important part:

1. **`reference-backend` is not supervised today.**
   - `reference-backend/main.mpl` starts the worker with a plain `spawn(...)` through `start_worker(...)`.
   - `reference-backend/jobs/worker.mpl` registers a separate state service and then spawns the actual worker actor, but it does not keep or expose the worker PID, restart count, or exit reason.

2. **A worker crash currently strands durable work.**
   - `reference-backend/storage/jobs.mpl` claims work by updating `status = 'processing'` and incrementing `attempts`.
   - If the worker crashes after claim and before `mark_job_processed(...)` / `mark_job_failed(...)`, that row stays `processing` forever because the app only claims `pending` jobs.
   - That is the concrete trust gap for R004.

3. **Failure visibility is currently too optimistic.**
   - `/health` always returns top-level `{"status":"ok"...}` and only reports the separate `JobWorkerState` service snapshot.
   - If the worker actor dies but the state service lives, `/health` can stay green with a stale `last_tick_at` / `last_status` and no explicit “worker is dead” signal.
   - Failures can therefore collapse into logs-only behavior or stale health data.

4. **The Mesh source-level supervisor path is not trustworthy enough yet for this slice without deeper proof.**
   - `compiler/meshc/tests/e2e_supervisors.rs` currently passes, but those tests only prove compile/run + banner strings. They do **not** prove that supervised children actually start, restart, or propagate failure correctly from Mesh source.
   - I ran a minimal temporary Mesh probe where a supervisor child actor prints `"worker boot"` on start. The compiled program printed only `"main done"`, which strongly suggests the source-level supervisor path is not actually starting the child in that scenario.
   - Code reading explains why this is plausible: `compiler/mesh-codegen/src/mir/lower.rs` reduces child `start:` clauses to “find the actor name being spawned”, while `compiler/mesh-codegen/src/codegen/expr.rs` serializes a child config shape that does not match the fuller `parse_supervisor_config(...)` contract in `compiler/mesh-rt/src/actor/mod.rs`.

This is a `debug-like-expert` situation: **verify, don’t assume**. S05 should not plan around “supervisors already work” just because shallow e2e tests are green.

## Recommendation

Treat S05 as **two linked hardening tracks**:

### Track A: make Mesh supervision trustworthy enough to use on the reference backend

First prove or repair the **source-level supervisor pipeline** before wiring it into `reference-backend/`.

Why first:

- R004 is specifically about concurrency and supervision being trustworthy.
- The Rust runtime unit tests are strong, but the Mesh-language path appears under-proved and may be partially broken for real child startup.
- `reference-backend` needs supervision with real app dependencies (`pool`, worker state, poll interval), so the language/compiler/runtime path must be honest before it becomes the slice’s foundation.

### Track B: add durable crash recovery + visible failure state to `reference-backend`

Once Track A is real enough, use it to harden the actual backend path:

- supervise the worker as a real child
- make claimed-but-not-finished jobs recoverable after crash/restart
- expose restart/failure state through `/health`
- prove the behavior through the existing compiler-facing backend harness

### Recommended app shape

Assuming the planner keeps supervisors as the canonical mechanism, the lowest-risk reference-backend shape is:

1. **Refactor the worker into a supervisor-friendly child with no captured startup args.**
   - Today `job_worker(pool, worker_state, poll_ms)` depends on runtime values passed via `spawn(...)`.
   - The current source-level supervisor lowering only appears to understand “which actor name is spawned”, not rich captured args.
   - So the safer pattern is to move runtime dependencies behind package-local named services / registry lookups.

2. **Add durable recovery for orphaned `processing` rows.**
   - A restarted worker must be able to detect and reclaim jobs that were left in `processing` by a prior crash.
   - The existing row shape already has enough data to support this (`status`, `attempts`, `updated_at`, `last_error`).

3. **Make crash/restart state explicit in health.**
   - At minimum, health should stop pretending everything is fine when the worker is dead or repeatedly restarting.
   - Planner should prefer explicit fields like restart count, last restart / exit reason, last recovered job, and a liveness classification over generic log strings.

4. **Use the existing Rust backend harness as the authoritative proof surface.**
   - This matches the `test` skill rule to extend existing test patterns instead of inventing a second harness.

### What not to do first

- **Do not build a logs-only proof story.** R004 wants visible failure state, not just banner lines.
- **Do not assume manual Mesh-level watchdog logic is the cheap fallback.** `Process.monitor` exists at the type level, but `trap_exit` / `exit` are not exposed as normal Mesh builtins, and there are no repo-local examples of a package-level actor consuming DOWN/EXIT messages. That is a research risk, not a safe primary plan.
- **Do not add only process restart tests without fixing orphaned `processing` rows.** That would prove the exact failure mode the slice is supposed to retire.

## What exists now

### `reference-backend/main.mpl`

Why it matters:
- the backend lifecycle root
- where supervision or startup wiring changes would land first

What it does now:
- validates env locally
- opens the Postgres pool
- starts the runtime registry
- starts the worker via `start_worker(pool, job_poll_ms)`
- calls `HTTP.serve(...)`

Key constraint:
- the worker path is a plain spawn path today, not a supervision tree
- the HTTP server is also not under any explicit reference-backend supervision structure

### `reference-backend/jobs/worker.mpl`

Why it matters:
- this is the center of the slice

What it does now:
- defines `JobWorkerState` as a separate service process
- records `last_tick_at`, `last_status`, `last_job_id`, `last_error`, `processed_jobs`, `failed_jobs`
- spawns `job_worker(pool, worker_state, poll_ms)` recursively
- handles DB-level soft failures by marking jobs failed or treating contention misses as idle

Critical gaps:
- no supervision
- no restart accounting
- no worker PID in state
- no durable recovery of orphaned `processing` rows
- if the worker actor crashes hard, the state service can outlive it and keep serving stale status

Natural seam:
- `start_worker(...)` is the clean place to switch from plain spawn to a supervised child model
- `JobWorkerState` is the clean place to add restart / recovery visibility fields

### `reference-backend/storage/jobs.mpl`

Why it matters:
- crash recovery semantics live here

What it does now:
- `claim_next_pending_job(...)` atomically moves one row to `processing`
- `mark_job_processed(...)` and `mark_job_failed(...)` finish the lifecycle

Critical gap:
- there is no reclaim path for `processing` rows left behind by worker or process death

Natural seam:
- add explicit recovery/requeue functions here
- if recovery needs performance help, this is also where a partial index or migration change would connect

### `reference-backend/api/health.mpl`

Why it matters:
- this is the required failure-visibility surface

What it does now:
- returns top-level `status: ok`
- returns raw worker snapshot values from `JobWorkerState`

Critical gap:
- health has no dead/stale/restarting concept
- health cannot distinguish “worker is healthy” from “worker state service still exists but the worker is gone”

Natural seam:
- extend health with explicit recovery/supervision/liveness fields rather than relying on operators to infer failure from timestamps

### `reference-backend/runtime/registry.mpl`

Why it matters:
- this is the easiest path to remove startup captures from supervised children

What it does now:
- stores only `pool`

Natural seam:
- if S05 wants supervisor children with simple `start: fn -> spawn(...) end`, this registry can carry the additional runtime dependencies the child needs to fetch for itself

### `compiler/meshc/tests/e2e_reference_backend.rs`

Why it matters:
- already the authoritative backend proof harness

What it already has:
- build helper
- spawn/stop helpers
- HTTP helpers for `/health` and `/jobs`
- direct Postgres query/execute helpers
- two-instance process orchestration
- strong DB + HTTP + log assertions

Natural seam:
- add crash/restart/failure-visibility proofs here instead of inventing a new test file
- the existing `spawn_reference_backend(...)`, `stop_reference_backend(...)`, and DB helpers make whole-process recovery proof relatively cheap

### `compiler/meshc/tests/e2e_supervisors.rs`

Why it matters:
- this is the current source-level supervision proof surface

What it proves now:
- compile success
- binary runs
- expected banner strings print
- one negative typecheck case fails

What it does **not** prove:
- child actually started
- child restarted on crash
- restart limit escalated correctly from Mesh source
- real shutdown behavior

This file needs to become materially stronger if S05 depends on supervisors.

### `compiler/mesh-rt/src/actor/supervisor.rs`

Why it matters:
- strong runtime donor implementation

What it appears to provide:
- one_for_one / one_for_all / rest_for_one / simple_one_for_one
- restart limit sliding window
- shutdown modes
- child lifecycle operations
- exit handling + strategy dispatch

Research verdict:
- the Rust runtime core looks substantially more mature than the current Mesh-language e2e coverage implies
- this makes the compiler/codegen bridge the likely weak link

### `compiler/mesh-codegen/src/mir/lower.rs` and `compiler/mesh-codegen/src/codegen/expr.rs`

Why they matter:
- they are the likely blocker for using real supervisors in `reference-backend`

What stands out:
- lowering comments explicitly say the child start handling is a “simple model” that finds the spawned actor name
- codegen serializes a child config layout that looks materially simpler than the runtime parser expects

Planner implication:
- a compiler/runtime repair task may be required before any honest reference-backend supervision work
- do not assume `start: fn -> spawn(job_worker, pool, worker_state, poll_ms) end` will work just because plain `spawn(...)` works outside supervisors

## Research evidence gathered

### 1. Existing reference-backend build proof is still healthy

Command run:

```bash
cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
```

Observed:
- passed
- `reference-backend` still builds cleanly as the canonical backend proof target

### 2. Current Mesh supervisor e2e suite is green but shallow

Command run:

```bash
cargo test -p meshc --test e2e_supervisors -- --nocapture
```

Observed:
- passed (4/4)
- assertions are banner-string based, not child-lifecycle based

### 3. Runtime supervisor unit tests are materially stronger

Command run:

```bash
cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture
```

Observed:
- passed (21/21)
- includes restart strategy, restart limit, shutdown, temporary/transient/permanent, and remote-child tests

Interpretation:
- the runtime core is much better proved than the Mesh source-level path

### 4. Source-level supervisor child startup appears broken or at least unproved

Manual probe run during research:

- created a temporary Mesh project in a temp dir
- actor `worker()` printed `"worker boot"` immediately on start
- supervisor child used `start: fn -> spawn(worker) end`
- `main()` spawned the supervisor, slept briefly, then printed `"main done"`

Observed output:

```text
main done
```

Not observed:
- `worker boot`

Interpretation:
- the current language-level supervisor path did not start the child in this simple probe
- this matches the code-reading concern around supervisor config lowering / serialization
- S05 should treat supervisor source support as a real risk item, not a solved dependency

### 5. The current durable job lifecycle cannot recover from crash-after-claim

This came from code inspection, not a DB-backed repro:

- `claim_next_pending_job(...)` moves a row to `processing`
- `process_claimed_job(...)` only transitions it onward if the worker keeps running
- there is no storage function that reclaims stale `processing` rows

Interpretation:
- a worker crash after claim currently creates a permanently marooned job row
- this is the concrete recovery hole S05 must close

## Natural seams for planning

### Seam 1: repair / strengthen source-level supervisor proof

Files:
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-rt/src/actor/mod.rs`
- `compiler/meshc/tests/e2e_supervisors.rs`
- likely new or updated fixtures under `tests/e2e/`

Goal:
- make a Mesh supervisor actually start children, restart them, and surface restart-limit failure in a real e2e path

Why separate:
- this is compiler/runtime infrastructure work, not reference-backend app work
- it unblocks everything else

### Seam 2: make `reference-backend` worker startup supervisor-friendly

Files:
- `reference-backend/main.mpl`
- `reference-backend/runtime/registry.mpl`
- `reference-backend/jobs/worker.mpl`
- possibly a new package-local supervision module

Goal:
- remove reliance on captured startup args so the worker can be started by a simple child-spec function

Why separate:
- this is app refactoring around an already-identified seam
- it can proceed once supervisor source support is trustworthy enough

### Seam 3: durable recovery for orphaned `processing` jobs

Files:
- `reference-backend/storage/jobs.mpl`
- possibly `reference-backend/migrations/20260323010000_create_jobs.mpl` or a new migration if an extra recovery index/column is chosen
- `reference-backend/jobs/worker.mpl`

Goal:
- on worker/process restart, requeue or otherwise recover work abandoned mid-flight

Why separate:
- this is the core data-correctness part of the slice
- it should be proved independently from the health/reporting surface

### Seam 4: health and failure visibility

Files:
- `reference-backend/api/health.mpl`
- `reference-backend/jobs/worker.mpl`
- maybe `reference-backend/api/jobs.mpl` if job-level error payloads need clearer surfacing

Goal:
- make failures visible without log tailing
- expose enough state to distinguish healthy, restarting, stale, recovered, and hard-failed behavior

Why separate:
- once the recovery semantics are decided, the health surface becomes straightforward

### Seam 5: compiler-facing backend crash/restart verification

Files:
- `compiler/meshc/tests/e2e_reference_backend.rs`
- optionally `reference-backend/scripts/smoke.sh` or a new sibling script if a package-local operator drill is wanted

Goal:
- add authoritative end-to-end proof for worker crash, restart, recovery, and visible failure state

Why separate:
- harness work can be built after the runtime/app semantics are clear
- it should remain the slice gate

## What to build or prove first

1. **First prove whether the Mesh-language supervisor path is actually usable for this slice.**
   - This is the highest-risk unknown.
   - The temporary child-start probe says “probably not yet”.
   - A stronger `e2e_supervisors` test should be the first concrete gate.

2. **If supervisors need repair, fix that before touching reference-backend health semantics.**
   - Otherwise the app risks building a fake supervision story on top of a broken primitive.

3. **After that, refactor the worker startup shape so it can be supervised without captured args.**
   - `runtime/registry.mpl` is the obvious donor seam.

4. **Then add crash recovery for orphaned `processing` rows.**
   - This is the durable-state heart of S05.
   - It is what turns “restart happened” into “work actually recovered”.

5. **Then add explicit failure/restart visibility to `/health`.**
   - Visibility should describe the actual recovery semantics, not guess at them.

6. **Finally, extend `e2e_reference_backend.rs` with crash/restart gates.**
   - Keep one authoritative backend harness.

## Verification plan

### Source-level supervisor gate

Strengthen and run:

```bash
cargo test -p meshc --test e2e_supervisors -- --nocapture
```

The stronger suite should explicitly prove at least:
- a child starts and does visible work after `spawn(Supervisor)`
- a crashing child is restarted under `one_for_one`
- restart-limit exhaustion is visible from the compiled Mesh path, not just Rust unit tests

### Runtime supervisor donor gate

Keep rerunning:

```bash
cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture
```

This should stay green as compiler/app work lands.

### Reference-backend crash/restart gates

Extend `compiler/meshc/tests/e2e_reference_backend.rs` with ignored DB-backed tests such as:

```bash
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_crash_recovers_job -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_worker_restart_is_visible_in_health -- --ignored --nocapture
DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_process_restart_recovers_inflight_job -- --ignored --nocapture
```

Whatever names the planner chooses, the slice should prove at least:
- a crash after claim does **not** leave the job permanently stuck in `processing`
- the restarted worker eventually processes or explicitly fails the durable row
- `/health` exposes restart/failure state without requiring log inspection
- DB truth, `/jobs/:id`, and `/health` agree on the final outcome

### DB truth checks

Use direct Postgres assertions from the existing harness to confirm:
- no stranded `processing` row remains after the recovery window
- `attempts` reflects the retry / recovery contract honestly
- `last_error` / recovery metadata match the visible failure story

### Optional package-local operator drill

If the slice adds a package-local drill, it should be a sibling to the existing smoke path and prove:
- crash the backend or worker deliberately
- restart it
- observe recovered work and explicit health state

But this should be secondary to the compiler-facing Rust harness.

## Risks / gotchas

- **Biggest risk: the Mesh-language supervisor pipeline may not currently work for real children.**
  - The runtime unit tests are not enough.
  - The temporary child-start probe is the clearest warning sign.

- **Do not wrap the current worker in a supervisor naively.**
  - It depends on `pool`, `worker_state`, and `poll_ms` arguments.
  - The current lowering path does not look capture-friendly.

- **Do not equate stale health with safe health.**
  - Today `/health` can be stale-but-green if the worker actor dies and the state service survives.

- **Do not add crash injection without recovery semantics.**
  - A deliberate worker crash on the current code will strand a row in `processing`.

- **Do not depend on low-level manual supervision from Mesh source unless you first prove it.**
  - `Process.monitor` exists, but the surrounding source-level ergonomics are not clearly exercised anywhere in repo code.

- **Prefer extending `e2e_reference_backend.rs` over creating a second backend failure harness.**
  - The current file already has the right process/HTTP/DB helpers.

## Skill discovery

Relevant missing skills I checked but did **not** install:

- **Rust:** `npx skills add apollographql/skills@rust-best-practices`
  - most directly relevant missing skill from search results for compiler/runtime work

- **PostgreSQL:** `npx skills add github/awesome-copilot@postgresql-optimization`
  - most relevant if S05 adds reclaim queries, retry indexes, or stale-processing scans

- **Systemd:** `npx skills add chaterm/terminal-skills@systemd`
  - only useful if the planner chooses to add an external service-manager recovery proof alongside in-process supervision

Relevant already-loaded skills that should shape implementation:
- `debug-like-expert`: **VERIFY, DON’T ASSUME** — this is why S05 should start by strengthening supervision proof instead of trusting shallow green tests
- `test`: extend existing harnesses and match repo patterns rather than inventing new verification surfaces

## Sources

Repo files inspected:

- `reference-backend/main.mpl`
- `reference-backend/jobs/worker.mpl`
- `reference-backend/api/health.mpl`
- `reference-backend/api/jobs.mpl`
- `reference-backend/api/router.mpl`
- `reference-backend/storage/jobs.mpl`
- `reference-backend/runtime/registry.mpl`
- `reference-backend/migrations/20260323010000_create_jobs.mpl`
- `reference-backend/scripts/smoke.sh`
- `reference-backend/scripts/deploy-smoke.sh`
- `reference-backend/README.md`
- `compiler/meshc/tests/e2e_reference_backend.rs`
- `compiler/meshc/tests/e2e_supervisors.rs`
- `compiler/meshc/tests/e2e_stdlib.rs`
- `tests/e2e/supervisor_basic.mpl`
- `tests/e2e/supervisor_one_for_all.mpl`
- `tests/e2e/supervisor_restart_limit.mpl`
- `tests/e2e/stdlib_http_crash_isolation.mpl`
- `compiler/mesh-rt/src/actor/child_spec.rs`
- `compiler/mesh-rt/src/actor/supervisor.rs`
- `compiler/mesh-rt/src/actor/mod.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `website/docs/docs/concurrency/index.md`
- `.gsd/KNOWLEDGE.md`

Commands run during research:

```bash
cargo test -p meshc --test e2e_supervisors -- --nocapture
cargo test -p meshc --test e2e_reference_backend e2e_reference_backend_builds -- --nocapture
cargo test -p mesh-rt supervisor::tests:: --lib -- --nocapture
npx skills find "Rust"
npx skills find "PostgreSQL"
npx skills find "systemd"
```

Additional manual proof performed:
- compiled and ran a temporary Mesh supervisor probe where the child actor should print on startup
- observed only `main done`, with no child-start output
- used that probe together with code inspection to treat source-level supervisor support as a real S05 risk instead of a solved dependency
