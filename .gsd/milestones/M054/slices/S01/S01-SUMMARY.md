---
id: S01
parent: M054
milestone: M054
provides:
  - A green one-public-URL ingress proof for the serious Postgres starter: a thin public-ingress harness fronts the staged two-node runtime and the retained bundle proves the first standby-targeted `GET /todos` through runtime-owned ingress/owner/replica/execution fields.
  - A bounded public starter contract shared by the scaffold template, committed Postgres example, and retained verifier: one public app URL may front multiple nodes, `meshc cluster` remains the inspection path, SQLite stays local-only, and Fly/frontend-aware routing are not part of the starter promise.
  - One assembled slice-owned wrapper (`bash scripts/verify-m054-s01.sh`) that downstream slices can call to replay scaffold parity, starter-boundary proof, the one-public-URL e2e, retained artifact copy, staged-bundle copy, redaction drift, and bundle-shape checks without rebuilding that logic.
requires:
  []
affects:
  - S02
  - S03
key_files:
  - compiler/meshc/tests/support/m054_public_ingress.rs
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/support/mod.rs
  - compiler/meshc/tests/e2e_m054_s01.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - examples/todo-postgres/README.md
  - scripts/verify-m054-s01.sh
  - scripts/tests/verify-m054-s01-contract.test.mjs
key_decisions:
  - D419: prove the one-public-URL story with a thin ingress harness plus existing `meshc cluster` continuity/diagnostics surfaces instead of adding frontend-aware routing or starter-owned placement logic.
  - D420: execute transient clustered HTTP route reply work inside an actor so standby-first public ingress can succeed without owner-side service-call crashes.
  - D421: materialize the committed example README from the scaffold template and republish one self-contained retained proof bundle under `.tmp/m054-s01/proof-bundles/` so the public starter contract cannot drift from the runtime-owned evidence.
patterns_established:
  - When the starter already has a truthful staged deploy seam, prove one-public-URL behavior with a thin ingress harness on top of that seam instead of adding starter-owned placement endpoints or client-aware routing logic.
  - Derive the public request summary from runtime-owned continuity and diagnostics records, not from proxy-side guesses; the retained `selected-route.summary.json` is the authoritative ingress/owner/replica/execution boundary.
  - Keep committed public examples generator-truthful by re-materializing them from the scaffold template instead of hand-editing README copy in place.
  - For assembled proof rails, republish a self-contained retained bundle that copies both runtime artifacts and the staged deploy bundle, then fail closed on redaction drift and bundle-shape drift.
observability_surfaces:
  - .tmp/m054-s01/verify/status.txt
  - .tmp/m054-s01/verify/current-phase.txt
  - .tmp/m054-s01/verify/phase-report.txt
  - .tmp/m054-s01/verify/full-contract.log
  - .tmp/m054-s01/verify/latest-proof-bundle.txt
  - .tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/scenario-meta.json
  - .tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/public-ingress.requests.json
  - .tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/public-ingress.snapshot.json
  - .tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/public-selected-list.request-summary.json
  - .tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/selected-route.summary.json
  - .tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/cluster-diagnostics-primary.json
  - .tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000/retained-m054-s01-artifacts/staged-postgres-public-ingress-truth-1775456495651189000/cluster-diagnostics-standby.json
drill_down_paths:
  - .gsd/milestones/M054/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M054/slices/S01/tasks/T02-SUMMARY.md
duration: ""
verification_result: passed
completed_at: 2026-04-06T06:32:10.546Z
blocker_discovered: false
---

# S01: One-public-URL starter ingress truth

**The serious Postgres starter now has a green one-public-URL proof bundle that shows the same standby-first public request's ingress, owner, replica, and execution truth through runtime-owned continuity and diagnostics surfaces.**

## What Happened

S01 turned the serious Postgres starter into a truthful one-public-URL proof surface instead of a direct-node-only story. The slice added a thin public-ingress harness in front of the staged two-node starter, forced the first real `GET /todos` through standby-first ingress, then used runtime-owned continuity and diagnostics surfaces to isolate that exact clustered route request and retain a summary of where ingress ended and Mesh runtime placement began. The retained proof now shows a real standby-targeted public request with `public_target_label=standby`, `ingress_node=todo-postgres-standby@[::1]:...`, `owner_node=todo-postgres-primary@127.0.0.1:...`, `replica_node=todo-postgres-standby@[::1]:...`, and `execution_node=todo-postgres-primary@127.0.0.1:...`, which is the core server-side-first balancing truth the milestone needed before any broader docs or observability work.

The slice also aligned the starter surfaces around that bounded contract. `compiler/mesh-pkg/src/scaffold.rs` and the committed `examples/todo-postgres/README.md` now say that one public app URL may front multiple starter nodes, `meshc cluster status|continuity|diagnostics` remain the inspection path for ingress/owner/replica/execution truth, SQLite stays the explicitly local branch, and the starter does not promise frontend-aware node selection or a Fly-specific product contract. On the verifier side, `scripts/verify-m054-s01.sh` now assembles the full S01 acceptance rail into one retained bundle: scaffold parity, meshc build preflight, example parity, starter-boundary replay, the one-public-URL e2e, copied ingress/runtime artifacts, copied staged deploy bundle, redaction checks, and bundle-shape checks all fail closed under `.tmp/m054-s01/verify/` and `.tmp/m054-s01/proof-bundles/`.

Closing the slice required one small but durable artifact-contract repair on top of the earlier runtime follow-through: the startup proof needed both `cluster-diagnostics-primary` and `cluster-diagnostics-standby` archived, not just the primary-side startup diagnostics. With that fixed, the direct e2e and the assembled verifier both go green, so downstream slices can now start from a real retained proof bundle instead of from task-level red bundles and partial docs wording.

## Verification

Plan-level verification passed on a disposable local Docker PostgreSQL admin URL.

- `DATABASE_URL=<redacted> cargo test -p meshc --test e2e_m054_s01 -- --nocapture`
- `DATABASE_URL=<redacted> bash scripts/verify-m054-s01.sh`

The assembled wrapper replay passed every slice-owned phase:

- `m054-s01-db-env-preflight`
- `m054-s01-scaffold-rail`
- `m054-s01-meshc-build-preflight`
- `m054-s01-example-parity`
- `m054-s01-starter-boundary`
- `m054-s01-public-ingress-e2e`
- `m054-s01-retain-artifacts`
- `m054-s01-retain-staged-bundle`
- `m054-s01-redaction-drift`
- `m054-s01-bundle-shape`

Retained verifier state after the green replay:

- `.tmp/m054-s01/verify/status.txt` = `ok`
- `.tmp/m054-s01/verify/current-phase.txt` = `complete`
- `.tmp/m054-s01/verify/latest-proof-bundle.txt` -> `.tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775456513099725000`

The retained selected-request proof is explicit:

- `public-selected-list.request-summary.json` records request `1` as `GET /todos` through the standby target with HTTP `200`
- `selected-route.summary.json` records that same request with `ingress_node=todo-postgres-standby@[::1]:52907`, `owner_node=todo-postgres-primary@127.0.0.1:52907`, `replica_node=todo-postgres-standby@[::1]:52907`, `execution_node=todo-postgres-primary@127.0.0.1:52907`, `runtime_name=Api.Todos.handle_list_todos`, `phase=completed`, and `result=succeeded`

### Operational Readiness

- **Health signal:** the retained proof shows `GET /health` succeeding through the same public URL with `status=ok`, `db_backend=postgres`, `migration_strategy=meshc migrate`, and `clustered_handler=Work.sync_todos`, while the copied diagnostics/continuity JSON provide the runtime-owned operator view.
- **Failure signal:** the slice now has explicit fail-closed rails for empty ingress targets, truncated backend responses, non-JSON backend responses, malformed route-summary inputs, missing/empty retained-bundle pointers, redaction drift, and retained bundle-shape drift.
- **Recovery procedure:** rerun `bash scripts/verify-m054-s01.sh` against a disposable PostgreSQL container, inspect `.tmp/m054-s01/verify/full-contract.log`, then follow `.tmp/m054-s01/verify/latest-proof-bundle.txt` into the copied truth artifacts. If the runtime rail is red, start with `selected-route.summary.json`, `public-ingress.requests.json`, and the paired `cluster-diagnostics-{primary,standby}.json` files before changing starter docs or wrapper wording.
- **Monitoring gaps:** S01 still needs S02's direct correlation surface so operators do not have to isolate the selected route by diffing continuity snapshots, and S03 still needs the same bounded contract propagated to the broader public docs surfaces.

## Requirements Advanced

- R123 — Proved the serious starter's current one-public-URL, server-side-first story on real two-node runtime surfaces, and aligned the starter README/example/verifier around that bounded load-balancing contract without introducing client-aware routing or Fly-specific overclaim.

## Requirements Validated

None.

## New Requirements Surfaced

None.

## Requirements Invalidated or Re-scoped

None.

## Deviations

No scope deviation from the slice goal. Local closeout used a disposable Docker PostgreSQL admin URL because this environment does not ship a repo-local `DATABASE_URL`. The only late repair was mechanical but important: the retained artifact contract expected both startup diagnostics nodes, so the final green closeout archived `cluster-diagnostics-primary` and `cluster-diagnostics-standby` instead of weakening the bundle-shape gate.

## Known Limitations

S01 still identifies the selected clustered `GET /todos` request by diffing continuity snapshots before and after the first public request; it does not yet ship a direct runtime-owned correlation field that maps a public request to a continuity record in one step. The bounded one-public-URL contract is now explicit on the serious starter surfaces, but broader homepage/distributed-proof/public-claim cleanup is still S03 work. This slice does not introduce frontend-aware node selection, client-visible node topology, or any stronger product claim than server-side replay plus runtime-owned inspection.

## Follow-ups

S02 should replace the before/after continuity diff seam with a direct runtime-owned request-correlation surface so operators can jump from one public request to one continuity record without artifact comparison. S03 should propagate the same bounded contract to homepage/distributed-proof/other public docs and keep it guarded with repo-owned docs tests instead of prose-only promises. If the route metadata shape changes later, keep `selected-route.summary.json` and the `scripts/verify-m054-s01.sh` retained bundle-shape gate aligned in the same change.

## Files Created/Modified

- `compiler/meshc/tests/support/m054_public_ingress.rs` — Added the thin public-ingress harness that fronts multiple starter nodes through one public URL and retains request/response metadata for the selected request.
- `compiler/meshc/tests/support/m053_todo_postgres_deploy.rs` — Extended the staged Postgres starter support seam so the ingress proof can reuse the M053 deploy/runtime helpers and dual-node continuity/diagnostics checks.
- `compiler/meshc/tests/support/mod.rs` — Registered the new M054 ingress support module with the meshc integration-test support surface.
- `compiler/meshc/tests/e2e_m054_s01.rs` — Added the authoritative M054 runtime rail and fail-closed negative tests for invalid ingress config, truncated backend replies, and malformed route-summary inputs.
- `compiler/mesh-pkg/src/scaffold.rs` — Updated the generated Postgres starter README contract so one public app URL, `meshc cluster` inspection, and the SQLite-local boundary are explicit.
- `examples/todo-postgres/README.md` — Re-materialized the committed Postgres example README from the scaffold template so the checked-in example stays generator-truthful.
- `scripts/verify-m054-s01.sh` — Added the assembled slice verifier that replays scaffold parity, starter-boundary proof, the one-public-URL e2e, redaction checks, retained artifact copy, and staged-bundle copy.
- `scripts/tests/verify-m054-s01-contract.test.mjs` — Added the cheap contract rail that fail-closes when the scaffold/example/verifier wording drifts away from the bounded one-public-URL story.
