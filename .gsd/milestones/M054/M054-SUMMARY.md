---
id: M054
title: "Load Balancing Truth & Follow-through — Context Draft"
status: complete
completed_at: 2026-04-06T17:02:57.047Z
key_decisions:
  - D418 — keep Fly as the proving environment and ingress evidence, but guard public load-balancing wording with repo-owned docs contract tests instead of making Fly the product contract
  - D419 — prove the one-public-URL story with a thin ingress harness plus existing `meshc cluster` continuity/diagnostics surfaces instead of frontend-aware routing or starter-owned placement logic
  - D420 — execute transient clustered HTTP route reply work inside an actor before building the response frame
  - D421 — materialize the committed Postgres example README from the scaffold template and republish one self-contained retained S01 proof bundle
  - D422 — expose clustered HTTP request correlation as a runtime-owned `X-Mesh-Continuity-Request-Key` response header and drive lookup from direct `meshc cluster continuity <node> <request-key> --json`
  - D423 — inject the same request-correlation header on both successful clustered responses and runtime-generated rejection responses while preserving app headers
  - D424 — copy the entire S01 verify tree unchanged into the S02 proof bundle and copy the fresh staged starter bundle so the retained proof stays self-contained
  - D425 — keep S03 scoped to VitePress homepage/distributed-proof/OG/verifier guardrails and defer `mesher/landing` cleanup
  - D426 — keep exact public-copy markers in the fast Node source-contract rail and let the shell wrapper assert built HTML fragments plus retained bundle shape
  - D427 — treat R123 as validated once the ingress proof, direct request correlation, and bounded public-contract rails all converge
key_files:
  - compiler/mesh-rt/src/http/server.rs
  - compiler/mesh-rt/src/dist/node.rs
  - compiler/meshc/tests/support/m054_public_ingress.rs
  - compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs
  - compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - compiler/meshc/tests/e2e_m047_s07.rs
  - compiler/meshc/tests/e2e_m054_s01.rs
  - compiler/meshc/tests/e2e_m054_s02.rs
  - compiler/meshc/tests/e2e_m054_s03.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - examples/todo-postgres/README.md
  - scripts/verify-m054-s01.sh
  - scripts/verify-m054-s02.sh
  - scripts/verify-m054-s03.sh
  - scripts/tests/verify-m054-s01-contract.test.mjs
  - scripts/tests/verify-m054-s02-contract.test.mjs
  - scripts/tests/verify-m054-s03-contract.test.mjs
  - website/docs/index.md
  - website/docs/.vitepress/config.mts
  - website/docs/docs/distributed-proof/index.md
  - website/scripts/generate-og-image.py
  - website/docs/public/og-image-v2.png
lessons_learned:
  - When local `main` already contains the milestone work, use the parent of the earliest milestone-tagged commit as the non-`.gsd` diff baseline; the literal `merge-base HEAD main` check can false-zero.
  - For clustered HTTP observability, a runtime-owned response header is the smallest truthful handoff; continuity-list diffing is a debugging workaround, not a durable operator contract.
  - Delegated proof chains stay debuggable when each later verifier copies earlier verify trees unchanged instead of re-deriving or mutating older `.tmp/.../verify` directories.
  - The visible requirements contract can outrun the GSD requirements DB; milestone closeout has to preserve the checked-in requirement file plus retained bundles as the visible truth until DB repair catches up.
---

# M054: Load Balancing Truth & Follow-through — Context Draft

**M054 proved the serious Postgres starter’s bounded one-public-URL load-balancing story end to end, added direct request-key follow-through for clustered HTTP, and aligned the public/docs/verifier contract to that runtime-owned model.**

## What Happened

M054 closed the load-balancing truth gap in three connected slices instead of widening the product claim. S01 proved that the serious Postgres starter can sit behind one public app URL while Mesh runtime placement still stays truthful and inspectable: a thin public-ingress harness fronts the staged two-node starter, the first real `GET /todos` is forced through standby-first ingress, and the retained bundle records that same request’s ingress, owner, replica, and execution truth through runtime-owned continuity and diagnostics surfaces. S02 then replaced continuity-list diffing with the missing follow-through seam: `compiler/mesh-rt/src/http/server.rs` now injects a runtime-owned `X-Mesh-Continuity-Request-Key` response header, the low-level clustered-route rail uses it for direct continuity lookup on both nodes, and the serious starter proves the same direct response-header -> record/diagnostics path through standby-first public ingress. S03 closed the public contract around that shipped behavior: homepage metadata, Distributed Proof, starter guidance, and the generated OG asset now all tell the same bounded story — one public app URL may choose ingress, Mesh runtime placement begins after ingress, `meshc cluster` remains the operator truth surface, and sticky sessions/frontend-aware routing/Fly-specific overclaim stay out of the contract. The slice chain also stayed operationally honest: S02 republishes S01’s retained verify tree unchanged, S03 republishes S02’s retained verify tree unchanged, and the final docs bundle still points back to the runtime-owned evidence chain instead of replacing it with prose. The only closeout caveat is bookkeeping rather than delivery: the milestone evidence validates R123, but the GSD requirements DB still lacks an `R123` row, so DB-backed requirement projection remains out of sync with the checked-in visible artifacts.

## Decision Re-evaluation

| Decision | Original Rationale | Still Valid? | Action |
|----------|-------------------|-------------|--------|
| D418 | Keep Fly as evidence, not the product contract, and guard public load-balancing wording with repo-owned docs rails. | Yes | Keep |
| D419 | Prove one-public-URL behavior by composing a thin ingress harness over existing runtime/operator truth instead of adding frontend-aware routing. | Yes | Keep |
| D420 | Run transient clustered HTTP reply work inside an actor so owner-side service calls execute in a valid runtime context. | Yes | Keep |
| D421 | Keep scaffold/example/verifier surfaces generator-truthful by materializing the committed README from the template and publishing one retained S01 proof bundle. | Yes | Keep |
| D422 | Use a runtime-owned response header as the direct correlation seam from clustered HTTP to one continuity record. | Yes | Keep |
| D423 | Preserve application headers and emit the same request-correlation header on runtime-generated rejection responses. | Yes | Keep |
| D424 | Preserve delegated proof lineage by copying the entire S01 verify tree unchanged into S02 and retaining the fresh staged bundle beside it. | Yes | Keep |
| D425 | Keep S03 scoped to homepage/VitePress/distributed-proof/OG/verifier surfaces and avoid pulling `mesher/landing` into the closeout. | Yes | Keep |
| D426 | Split docs guarding into a fast source-contract rail plus a heavier built-HTML/retained-bundle wrapper rail. | Yes | Keep |
| D427 | Treat R123 as validated only once ingress proof, direct request correlation, and bounded public-contract rails all converged. | Yes, with DB caveat | Keep; repair DB projection separately |

## Success Criteria Results

- **One public app URL truthfully fronts the serious clustered PostgreSQL starter, and retained evidence shows ingress vs owner/replica/execution for the same real request — MET.** Evidence: S01 summary/UAT; `.tmp/m054-s01/verify/status.txt = ok`; `.tmp/m054-s01/verify/current-phase.txt = complete`; `.tmp/m054-s01/verify/latest-proof-bundle.txt` points at `.tmp/m054-s01/proof-bundles/retained-one-public-url-proof-1775494073795260000`; retained `selected-route.summary.json` records a standby-targeted `GET /todos` with `ingress_node=todo-postgres-standby@[::1]:55810`, `owner_node=todo-postgres-primary@127.0.0.1:55810`, `replica_node=todo-postgres-standby@[::1]:55810`, and `execution_node=todo-postgres-primary@127.0.0.1:55810`.
- **A single clustered HTTP request can be traced directly to one continuity record through runtime-owned correlation output instead of before/after continuity diffing — MET.** Evidence: S02 summary/UAT; `cargo test -p mesh-rt m054_s02_ -- --nocapture`; `cargo test -p meshc --test e2e_m047_s07 -- --nocapture`; `.tmp/m054-s02/verify/status.txt = ok`; `.tmp/m054-s02/verify/current-phase.txt = complete`; retained `public-selected-list.request-key.json` shows `header_name = X-Mesh-Continuity-Request-Key` and `request_key = http-route::Api.Todos.handle_list_todos::1`; retained `selected-route-direct-primary-record.json` and `selected-route-direct-standby-record.json` resolve that same key directly on both nodes.
- **Homepage, Distributed Proof docs, and serious starter guidance describe the same bounded load-balancing model, and contract tests fail if copy overclaims — MET.** Evidence: S03 summary/UAT; `node --test scripts/tests/verify-m054-s03-contract.test.mjs`; `cargo test -p meshc --test e2e_m054_s03 -- --nocapture`; `npm --prefix website run generate:og`; `npm --prefix website run build`; `.tmp/m054-s03/verify/status.txt = ok`; `.tmp/m054-s03/verify/current-phase.txt = complete`; `.tmp/m054-s03/verify/built-html-summary.json` reports `new_description=true`, `old_description_absent=true`, `boundary=true`, `header_lookup=true`, `list_first=true`, and `non_goals=true`.
- **The follow-through needed to make the story usable and auditable shipped with fail-closed proof surfaces — MET.** Evidence: S01, S02, and S03 each retain green assembled verifier trees and proof bundles; `.tmp/m054-s03/verify/phase-report.txt` shows the delegated `m054-s03-s02-replay` plus docs/OG/bundle-shape/redaction phases all passed; the retained S03 proof bundle contains `retained-m054-s02-verify/`, and the retained S02 proof bundle contains `retained-m054-s01-verify/`, so the final public-contract proof still resolves back to the original runtime-owned ingress evidence.

## Definition of Done Results

- [x] **All planned slices are complete.** The roadmap slice overview marks S01, S02, and S03 complete, and the validation delivery audit records every slice as delivered.
- [x] **All slice summaries and UAT artifacts exist.** Files exist for `.gsd/milestones/M054/slices/S01/S01-SUMMARY.md`, `S01-UAT.md`, `.gsd/milestones/M054/slices/S02/S02-SUMMARY.md`, `S02-UAT.md`, `.gsd/milestones/M054/slices/S03/S03-SUMMARY.md`, and `S03-UAT.md`.
- [x] **The milestone produced real code and contract changes, not only planning artifacts.** The literal `git diff --stat HEAD $(git merge-base HEAD main) -- ':!.gsd/'` false-zeroed because local `main` already contains the milestone work; using the equivalent pre-M054 baseline (`git diff --stat db4bf8ad HEAD -- ':!.gsd/'`) returned a non-empty diff across 37 non-`.gsd/` files including runtime, scaffold, tests, docs, OG generation, and verifier surfaces.
- [x] **Cross-slice integration is closed.** S02 consumes and republishes the S01 verify tree unchanged, S03 consumes and republishes the S02 verify tree unchanged, and the retained artifacts prove the runtime -> starter -> docs chain stayed aligned without re-deriving the earlier proof surfaces.
- [x] **Operational / integration / UAT classes are met inside the milestone’s bounded scope.** S01 proves live public-ingress runtime truth, S02 proves direct request-key follow-through on both low-level and serious-starter rails, and S03 proves the public/docs/OG contract plus fail-closed drift guards.
- [x] **Horizontal checklist review is complete.** `M054-ROADMAP.md` does not contain a `Horizontal Checklist` section, so there were no additional checklist items beyond the milestone contract itself.
- [x] **No unchecked slice/task completion checkboxes remain.** `rg -n -- "^- \[ \] \*\*S[0-9]+:" .gsd/milestones/M054` and `rg -n -- "^- \[ \] \*\*T[0-9]+:" .gsd/milestones/M054` returned no unchecked roadmap or task checklist items.

## Requirement Outcomes

- **R123: active -> validated.** Evidence: the visible requirement text in `.gsd/REQUIREMENTS.md` describes the exact load-balancing honesty/follow-through problem M054 was created to close; S01 proved one-public-URL ingress truth on a real staged two-node starter; S02 added the runtime-owned `X-Mesh-Continuity-Request-Key` handoff plus direct continuity/diagnostics lookup on both nodes; S03 aligned homepage, Distributed Proof, starter guidance, OG output, and fail-closed verifier rails to that bounded model; D427 records the validation decision; and the assembled evidence chain is `bash scripts/verify-m054-s01.sh` -> `bash scripts/verify-m054-s02.sh` -> `node --test scripts/tests/verify-m054-s03-contract.test.mjs` + `cargo test -p meshc --test e2e_m054_s03 -- --nocapture` + `npm --prefix website run generate:og` + `npm --prefix website run build` + `bash scripts/verify-m054-s03.sh`.
- **No requirement was deferred, blocked, or moved out of scope during milestone closeout.**
- **Bookkeeping caveat:** the checked-in `.gsd/REQUIREMENTS.md` still showed `R123` as `active` before closeout because the GSD requirements DB does not contain an `R123` row. The validation above is still supported by shipped milestone evidence, so closeout must preserve that visible truth even if the DB update path remains degraded.

## Deviations

No product-scope deviation was required. The only closeout caveat is bookkeeping: M054 materially validates R123, but the current GSD requirements DB still lacks that row, so DB-backed requirement projection may lag the shipped visible artifacts.

## Follow-ups

Repair the GSD requirements DB so `R123` can be projected as validated without manual visible-file sync or failed `gsd_requirement_update` calls. If future milestones touch load-balancing, request correlation, or docs claims, keep the S01/S02/S03 retained-bundle contract aligned in the same change and preserve the bounded model: one public URL may choose ingress, runtime placement begins after ingress, and the response-header/request-key seam is for operator truth rather than client-side routing.

## Forward Intelligence

### What the next milestone should know
- The authoritative M054 proof chain is nested, not flat: start at `.tmp/m054-s03/verify/latest-proof-bundle.txt`, then follow `retained-m054-s02-verify/`, then the retained S01 bundle if the issue is runtime-owned ingress truth rather than docs copy.
- The bounded public contract is now explicit and verified: one public app URL may choose ingress, runtime placement begins after ingress, and request-key lookup is an operator/debug seam rather than a sticky-session or client-routing promise.

### What's fragile
- `R123` is now validated in the visible artifacts, but the GSD requirements DB still has no `R123` row. Future closeout work can still fail on `gsd_requirement_update` until that storage layer is repaired.
- The source-copy docs rails are intentionally literal. If homepage/distributed-proof/starter/OG wording changes, update the fast Node contract tests and the built-HTML wrapper assertions in the same change or the retained docs rail will go red for honest copy changes.

### Authoritative diagnostics
- `.tmp/m054-s02/.../public-selected-list.request-key.json` and `selected-route-direct-{primary,standby}-{record,diagnostics}.json` are the fastest trustworthy request-scoped debug seam when clustered HTTP correlation regresses.
- `.tmp/m054-s01/.../selected-route.summary.json` is still the authoritative ingress/owner/replica/execution boundary record for the public one-URL story.
- `.tmp/m054-s03/verify/built-html-summary.json` is the fastest truth surface for public docs/OG drift before opening the full retained site bundle.

### What assumptions changed
- The original public one-URL story was not enough on its own; the real missing follow-through was a runtime-owned response-to-record handoff, not a second starter-owned control route or frontend-aware adapter.
- Public/docs verification needed a layered rail: exact source-copy markers in a fast test plus built-site/bundle-shape proof in the shell wrapper, instead of one oversized verifier trying to own both.

## Files Created/Modified

- `compiler/mesh-rt/src/http/server.rs` — Injects the runtime-owned `X-Mesh-Continuity-Request-Key` correlation header on clustered HTTP success and rejection responses.
- `compiler/mesh-rt/src/dist/node.rs` — Repairs the runtime-owned transient clustered HTTP reply path so standby-first ingress can execute owner-side route work safely.
- `compiler/meshc/tests/support/m054_public_ingress.rs` — Implements the thin one-public-URL ingress harness plus retained public-ingress/request-correlation artifact helpers.
- `compiler/meshc/tests/e2e_m054_s01.rs` — Proves one-public-URL ingress truth on the staged serious Postgres starter.
- `compiler/meshc/tests/e2e_m054_s02.rs` — Proves direct response-header-to-continuity correlation on the staged serious Postgres starter.
- `compiler/meshc/tests/e2e_m054_s03.rs` — Pins the public-docs/verifier layering and retained-bundle contract under Cargo.
- `compiler/mesh-pkg/src/scaffold.rs` and `examples/todo-postgres/README.md` — Align generated and committed starter guidance to the bounded one-public-URL/request-key contract.
- `scripts/verify-m054-s01.sh`, `scripts/verify-m054-s02.sh`, and `scripts/verify-m054-s03.sh` — Assemble the retained runtime, correlation, and public-docs proof rails into one delegated closeout chain.
- `scripts/tests/verify-m054-s01-contract.test.mjs`, `scripts/tests/verify-m054-s02-contract.test.mjs`, and `scripts/tests/verify-m054-s03-contract.test.mjs` — Fail closed on starter/docs/verifier wording drift and retained-bundle contract drift.
- `website/docs/index.md`, `website/docs/.vitepress/config.mts`, `website/docs/docs/distributed-proof/index.md`, `website/scripts/generate-og-image.py`, and `website/docs/public/og-image-v2.png` — Align the public site and OG surface to the shipped bounded load-balancing contract.
