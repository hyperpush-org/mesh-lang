# M053 / S06 Research — Hosted failover promotion truth and annotated tag reroll

**Date:** 2026-04-06  
**Status:** Ready for planning

## Summary

S06 is narrower than S05’s blocker note makes it sound. It owns the final hosted closeout of **R121** and **R122**, while supporting the upstream public-story constraints in **R115**, **R116**, **R117**, and **R120**.

The slice has two deliverables:

1. make the hosted starter failover proof go green on `main`
2. reroll `v0.1.0` as an **annotated** tag on that same green SHA so `refs/tags/v0.1.0^{}` resolves and `release.yml` freshness becomes satisfiable

The current retained state is still:

- `.tmp/m053-s03/verify/status.txt` = `failed`
- `.tmp/m053-s03/verify/current-phase.txt` = `remote-evidence`
- `deploy-services.yml` is green on shipped SHA `314bbac88b171388b04072a97f22be0bca4882aa`
- `authoritative-verification.yml` is red on that same SHA (`24014506220`)
- `release.yml` freshness is blocked because `refs/tags/v0.1.0^{}` is missing

The important correction from this research pass: **the hosted red does not currently look like a broken release workflow or missing mirror transport. It looks like a timing mismatch.**

Following the `debug-like-expert` playbook, the retained hosted bundle is the source of truth. That bundle shows:

- standby had a mirrored pending startup record before kill (`pre-kill-continuity-standby.json`)
- but the primary finished startup before the kill (`primary-run1.combined.log` contains `transition=completed request_key=startup::Work.sync_todos` and `transition=startup_completed`)
- then the standby truthfully rejected promotion with `automatic_promotion_rejected:no_mirrored_state`

So the likely issue is **not** “standby never had mirrored state.” It is that the **promotable pending window closed before the kill on hosted Ubuntu**.

There is a second critical finding: the harness and docs still refer to `MESH_STARTUP_WORK_DELAY_MS`, but the current runtime hot path in `compiler/mesh-rt/src/dist/node.rs` does **not** read that env var. `startup_dispatch_window_ms(...)` is hardcoded to `STARTUP_CLUSTERED_PENDING_WINDOW_MS = 2500`. The local retained green bundle also logged `pending_window_ms=2500`, which means the current local green is timing luck, not proof that the 20s harness knob is working.

## Requirements Focus

### Directly owned
- **R121** — packages site remains part of the normal hosted contract. S06 does not need new package work; it must preserve the already-green `deploy-services.yml` result while fixing the starter lane.
- **R122** — Postgres clustered deploy proof must be real. The hosted failover lane is the remaining open proof surface.

### Constraining upstream requirements
- **R115** — keep the SQLite/Postgres split honest; do not fix hosted failover by widening SQLite or softening Postgres claims.
- **R116** — keep generated starter proof primary; do not swap in retained fixtures as the public contract.
- **R117** — keep docs evaluator-facing; do not turn S06 into verifier-maze-first copy churn.
- **R120** — preserve one coherent public story across docs/landing/packages; S06 should not reopen already-green deploy-services/package surfaces.

## Skills Discovered

Relevant installed skills already present:

- `github-workflows` — directly relevant for the hosted run chain and tag-triggered `release.yml`
- `postgresql-database-engineering` — relevant only insofar as the hosted failover rail depends on a real Postgres-backed starter, but the DB itself is not the current blocker

Relevant bundled skill already guiding the approach:

- `debug-like-expert` — use retained evidence first, reproduce the exact failure condition, then change the smallest plausible seam instead of thrashing across workflows/docs

No new skill installs are needed for this slice.

## Implementation Landscape

### 1. Runtime promotion / startup timing seam

**File:** `compiler/mesh-rt/src/dist/node.rs`

This is the main implementation target.

Relevant functions:

- `handle_node_disconnect(...)` — loss handler; degrades continuity state and then calls `maybe_automatic_promote_and_resume(...)`
- `automatic_promotion_reason(...)` — node-level promotion precheck; only allows promotion when there is at least one pending standby-owned record whose disconnected owner matches and whose `replica_status` is `Preparing | Mirrored`
- `maybe_automatic_promote_and_resume(...)` — promotes standby authority and spawns automatic recovery submissions
- `prepare_continuity_replica(...)` — owner-side prepare request to the replica
- `startup_dispatch_window_ms(...)` / `maybe_hold_startup_work_dispatch(...)` — current startup hold path

Critical current behavior:

- `startup_dispatch_window_ms(...)` ignores `MESH_STARTUP_WORK_DELAY_MS` and always returns the fixed `STARTUP_CLUSTERED_PENDING_WINDOW_MS` (2500 ms) for runtime-owned clustered startup work.
- The hosted failing bundle’s primary log shows exactly that fixed value:
  - `transition=startup_dispatch_window ... pending_window_ms=2500`
- The hosted primary log also shows startup completed before the owner kill:
  - `transition=completed request_key=startup::Work.sync_todos ...`
  - `transition=startup_completed runtime_name=Work.sync_todos ...`
- The hosted standby log then truthfully shows:
  - `transition=automatic_promotion_rejected ... reason=automatic_promotion_rejected:no_mirrored_state`

That sequence strongly suggests the failover rail is killing the primary after the startup request has already left the promotable pending state.

### 2. Continuity registry semantics

**File:** `compiler/mesh-rt/src/dist/continuity.rs`

This file does not appear to be the first fix target, but it constrains any runtime change.

Relevant functions / rules:

- `promote_authority()` — standby promotion at the registry layer; rejects only when the registry has no records at all
- `project_record_for_authority_change(...)` — when standby becomes primary, pending mirrored records become `OwnerLost` / `Unavailable`
- `mark_owner_loss_records_for_node_loss(...)`
- `degrade_replica_records_for_node_loss(...)`
- `degrade_replication_health_for_node_loss(...)`
- validation rule: standby records may not remain `ReplicaStatus::OwnerLost` (`STANDBY_OWNER_LOST_INVALID`)

Relevant existing tests already present:

- `continuity_promotion_rejects_standby_without_mirrored_state`
- `continuity_promotion_marks_mirrored_pending_record_owner_lost_and_reuses_retry_rollover`
- `automatic_promotion_promotes_mirrored_pending_record_and_reuses_retry_rollover`

These tests prove the registry-level model is fine when a mirrored pending record still exists. The current gap is higher up: a hosted real-runtime run can reach disconnect after the pending record is already completed.

### 3. Generated starter failover e2e rail

**File:** `compiler/meshc/tests/e2e_m053_s02.rs`

This is the authoritative local proof rail and the likely second code target.

What it does:

- initializes a fresh generated Postgres starter
- stages the deploy bundle outside the source tree
- applies deploy migrations
- boots a dual-stack primary/standby pair
- proves pre-kill continuity truth
- kills the primary
- requires standby promotion, automatic recovery, rejoin fencing, and post-failover CRUD truth

Important details:

- it currently calls `deploy::default_cluster_runtime_pair_for_primary_owned_startup(..., Some(20_000))`
- it waits for pre-kill startup continuity to be `preparing|mirrored` on primary and `mirrored` on standby
- it then does additional CRUD + route-continuity work before killing the owner
- the hosted failure happens at:
  - `route_free::wait_for_cluster_status_matching(... "post-kill standby promotion truth" ...)`

Critical contradiction resolved by this research:

- the test *intends* `Some(20_000)` to keep the startup request pending
- the runtime does not consume that env knob today
- so the hosted red can occur even while the pre-kill continuity probes still passed earlier in the run

This file likely needs either:

- no logic change once the runtime env override is restored, or
- an additional retained assertion that the configured startup dispatch window was actually applied, so this regression cannot hide again behind a green local race

### 4. Staged Postgres starter harness

**Files:**
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`

What matters:

- the support layer already exports `MESH_STARTUP_WORK_DELAY_MS` when `startup_work_delay_ms` is set
- it chooses a dual-stack port and brute-forces until the deterministic startup request hashes to the primary (`startup_request_owns_primary(...)`)
- so ownership nondeterminism is already handled by the harness

Important implication:

- the harness is already wired for a runtime-owned startup delay override
- if S06 restores that seam in `mesh-rt`, the existing test harness should benefit without a new public app-level timing knob

### 5. Retained wrapper / hosted verifier surfaces

**Files:**
- `scripts/verify-m053-s02.sh`
- `scripts/verify-m053-s03.sh`
- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/release.yml`

Current status:

- `scripts/verify-m053-s02.sh` is already in the right shape; it preserves nested S01 logs and the retained S02 proof bundle. It is **not** the primary fix target.
- `scripts/verify-m053-s03.sh` already does the correct hosted read-only checks:
  - derives the binary tag from Cargo version
  - resolves exact remote refs with `git ls-remote`
  - requires `release.yml` freshness to resolve `refs/tags/v0.1.0^{}`
  - requires hosted runs to be green on the exact expected SHA
- `.github/workflows/authoritative-starter-failover-proof.yml` runs the failing S02 wrapper on Ubuntu 24.04 with runner-local Postgres
- `.github/workflows/release.yml` already depends on tag-only `Authoritative starter failover proof` and `Create Release`

Important implication:

- do **not** spend S06 on workflow edits unless the runtime fix reveals a real workflow drift afterward
- the tag issue is a Git ref truth issue, not a workflow-definition issue

### 6. Current retained evidence worth reusing

Primary files / dirs:

- `.tmp/m053-s03/verify/remote-runs.json`
- `.tmp/m053-s05/rollout/final-blocker.md`
- `.tmp/m053-s05/remote-auth-24014506220/artifacts/authoritative-starter-failover-proof-diagnostics/staged-postgres-failover-runtime-truth-1775437858534365102/`
- `.tmp/m053-s02/proof-bundles/retained-failover-proof-1775437263433228000/retained-m053-s02-artifacts/staged-postgres-failover-runtime-truth-1775437250020428000/`

Most useful exact artifacts:

**Hosted red bundle**
- `pre-kill-continuity-standby.json` — standby really did observe mirrored pending state earlier
- `primary-run1.combined.log` — startup later completed before kill; this is the decisive artifact
- `standby-run1.combined.log` — standby then rejected promotion because there was no promotable pending record
- `post-kill-status-standby.timeout.txt` / `.json` — standby stayed `cluster_role=standby`, `promotion_epoch=0`

**Local green bundle**
- `primary-run1.combined.log` — also shows `pending_window_ms=2500`
- `standby-run1.combined.log` — local pass promoted before startup completed, i.e. current green depends on faster timing, not a different runtime contract

## Key Findings

### A. The current hosted red is most likely a timing seam, not a missing mirror transport seam

Evidence chain:

1. Hosted standby pre-kill continuity artifact shows mirrored pending startup truth.
2. Hosted primary log later shows startup completed before owner kill.
3. Hosted standby then rejects promotion with `no_mirrored_state`.

That is consistent with a truthful promotion rejection after the owner already finished the startup work, not with a broken prepare/ack path.

### B. `MESH_STARTUP_WORK_DELAY_MS` is currently a dead knob in the runtime hot path

Evidence:

- harness exports it (`compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`)
- knowledge/docs expect it to own the delay seam
- current runtime does not read it
- both hosted red and local green bundles logged `pending_window_ms=2500`

This is the most actionable implementation seam in S06.

### C. The local green S02 bundle is timing-sensitive and should not be treated as robust proof

The local retained pass succeeded with the same fixed 2500 ms startup dispatch window. The difference is that the primary was killed before startup completed, while hosted Ubuntu reached `startup_completed` first. S06 should harden that proof so local green and hosted green mean the same thing.

### D. The tag problem is purely a ref-truth problem

`release.yml` freshness fails because `scripts/verify-m053-s03.sh` requires both:

- `refs/tags/v0.1.0`
- `refs/tags/v0.1.0^{}`

The current tag is still lightweight. No workflow change will fix that. An **annotated tag reroll** on the final green main SHA is required.

Also note the external-action constraint: rerolling the tag is a GitHub-facing mutation and will require explicit user confirmation before execution.

## Recommendation

Plan S06 as **two technical tasks plus one rollout task**, in this order.

### Task 1 — Restore the runtime-owned startup hold seam

Primary target:
- `compiler/mesh-rt/src/dist/node.rs`

Preferred change:
- make `startup_dispatch_window_ms(...)` honor `MESH_STARTUP_WORK_DELAY_MS` again for runtime-owned clustered startup work
- preserve the current fixed 2500 ms default when the env var is absent
- keep the delay runtime-owned; do not reintroduce app-owned `Timer.sleep` or work-source knobs

Why this is the right first move:
- it matches the existing harness/config surface
- it matches the M046 contract and knowledge note
- it directly addresses the hosted/local timing split visible in retained bundles

Verification to add here:
- a runtime unit test proving the env override changes the effective pending window
- a unit test preserving the default 2500 ms path when the env var is absent

### Task 2 — Tighten the S02 failover rail so it proves the intended pending window, not accidental timing

Primary targets:
- `compiler/meshc/tests/e2e_m053_s02.rs`
- possibly `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs`

Recommended shape:
- keep the current generated-starter-first failover proof
- add an assertion against retained logs/artifacts that the startup dispatch window actually used the configured override
- optionally add a direct assertion that primary `startup_completed` did **not** occur before the forced owner stop in the failover scenario

Goal:
- after the runtime fix, both local and hosted rails should fail closed if the startup window falls back to 2500 ms again

### Task 3 — Refresh hosted evidence, then reroll the annotated tag

Read-only verification first:
- rerun local rails until green
- ship to `main`
- confirm fresh green `authoritative-verification.yml` and `deploy-services.yml` on the same SHA via `bash scripts/verify-m053-s03.sh`

Only then:
- reroll `v0.1.0` as an **annotated** tag on that same SHA
- wait for tag-triggered `release.yml`
- rerun `bash scripts/verify-m053-s03.sh` so `remote-runs.json` shows all three workflows green on expected refs

Constraint:
- the annotated tag reroll is an external GitHub mutation and must not be executed without explicit user confirmation

## Verification Plan

### Local runtime / unit proof

Run targeted runtime tests after the env-override change:

- `cargo test -p mesh-rt startup_work_dispatch_window_only_applies_to_runtime_owned_clustered_startup_requests -- --nocapture`
- add and run a new targeted test for the `MESH_STARTUP_WORK_DELAY_MS` override path
- keep the existing continuity promotion tests green in `compiler/mesh-rt/src/dist/continuity.rs`

### Local generated-starter failover proof

Authoritative local rail:

- `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' cargo test -p meshc --test e2e_m053_s02 m053_s02_staged_postgres_failover_proves_clustered_http_and_runtime_recovery -- --nocapture`

Then the assembled wrapper:

- `DATABASE_URL='postgres://postgres:postgres@127.0.0.1:5432/postgres' bash scripts/verify-m053-s02.sh`

Success criteria after S06 fix:

- retained local failover bundle logs show the configured pending window, not `2500`
- primary does not complete startup before kill in the failover scenario
- standby reaches `automatic_promotion`, `automatic_recovery`, and completed continuity truth

### Hosted read-only closeout

After shipping the fix to `main`:

- `bash scripts/verify-m034-s02-workflows.sh`
- `node --test scripts/tests/verify-m053-s03-contract.test.mjs`
- `bash scripts/verify-m053-s03.sh`

Expected hosted outcomes before tag reroll:

- `authoritative-verification.yml` green on shipped `main` SHA
- `deploy-services.yml` still green on the same SHA
- `release.yml` still blocked only on missing peeled tag data

### Tag freshness proof

After approved annotated reroll:

- `git ls-remote origin refs/tags/v0.1.0 'refs/tags/v0.1.0^{}'`
- `bash scripts/verify-m053-s03.sh`

Success criteria:

- `refs/tags/v0.1.0^{}` resolves
- `remote-runs.json` shows green `release.yml` on the tag ref
- `.tmp/m053-s03/verify/status.txt` becomes `ok`

## Planner Notes

- `S06-PLAN.md` exists but has no tasks yet.
- Do not plan wrapper-script churn first. `scripts/verify-m053-s02.sh` and `scripts/verify-m053-s03.sh` already retain the right evidence.
- Do not plan package/deploy-services work unless the runtime fix reveals new drift. That surface is already green.
- The most honest first task is restoring the runtime-owned delay seam and proving it in the logs.
- The second task is making the S02 failover rail fail closed on the real timing invariant.
- The final task is rollout evidence refresh plus annotated tag reroll preparation; the reroll itself requires explicit user approval because it mutates GitHub state.
