---
id: S01
parent: M032
milestone: M032
provides:
  - Durable stale-vs-real classification for the audited mesher limitation families, with exact proof surfaces and downstream slice owners.
affects:
  - S02
  - S03
  - S04
  - S05
key_files:
  - .gsd/milestones/M032/slices/S01/S01-SUMMARY.md
  - compiler/meshc/tests/e2e.rs
  - compiler/meshc/tests/e2e_stdlib.rs
  - scripts/verify-m032-s01.sh
  - mesher/ingestion/routes.mpl
  - mesher/services/event_processor.mpl
  - mesher/services/stream_manager.mpl
  - mesher/storage/writer.mpl
  - mesher/storage/queries.mpl
key_decisions:
  - S01 treats route-closure support as runtime-real only after a live request; compile-only success is not authoritative.
  - The priority Mesh fix handoff is the imported inferred-polymorphic export failure (`xmod_identity`), not the stale cross-module `from_json` family.
patterns_established:
  - Every M032 limitation family now needs a named test or concrete inspection command plus a next-slice owner before it can stay in `mesher/` comments.
observability_surfaces:
  - cargo test -p meshc --test e2e m032_ -- --nocapture
  - cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture
  - bash scripts/verify-m032-s01.sh
  - .tmp/m032-s01/verify/
  - .gsd/milestones/M032/slices/S01/S01-SUMMARY.md
drill_down_paths:
  - .gsd/milestones/M032/slices/S01/tasks/T01-SUMMARY.md
  - .gsd/milestones/M032/slices/S01/tasks/T02-SUMMARY.md
  - .gsd/milestones/M032/slices/S01/tasks/T03-SUMMARY.md
duration: 4h 00m
verification_result: passed
completed_at: 2026-03-24T16:26:47-0400
---

# S01: Limitation truth audit and repro matrix

**Published the mesher limitation matrix with durable proofs, mixed-truth notes, and explicit handoffs for S02/S03/S04/S05.**

## What Happened

S01 converted the `mesher/` limitation folklore into a durable proof set and a usable handoff.

T01 froze the stale-supported families into named CLI e2e tests: query-string access, cross-module `from_json`, inline `case` inside a service call body, and inline `if/else` inside a cast handler. T02 froze the still-real failure surfaces: the imported inferred-polymorphic export failure, the nested-`&&` codegen failure, the timer-to-service-cast no-op, and the route-closure runtime trap. This task turned that proof inventory into the authoritative matrix below so later slices can act without rereading research notes.

The important split is now explicit:

- `xmod_identity` is the real S02 blocker.
- request/handler/control-flow folklore is mostly stale and belongs to S03 cleanup.
- module-boundary `from_json` wording is stale, but some neighboring raw-SQL rationale is still truthful and belongs to S04 comment surgery.
- route closures, nested `&&`, timer-to-service-cast, and multi-statement `case` arms remain verified keep-sites until a later slice deliberately fixes them.

## Stale Folklore

| Family | Mesher site(s) | Status | Proof surface | Likely owning subsystem | Next slice | Action |
|---|---|---|---|---|---|---|
| Query-string parsing unavailable | `mesher/ingestion/routes.mpl:445` | stale | `cargo test -p meshc --test e2e e2e_m032_supported_request_query -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`build_request_query`, `request_query`) | mesher HTTP handler cleanup | S03 | Remove the stale comment and let `handle_list_issues(...)` use `Request.query(...)` directly when S03 cleans the handler. |
| Cross-module `from_json` unavailable | `mesher/services/event_processor.mpl:5,120`; `mesher/storage/queries.mpl:482`; `mesher/storage/writer.mpl:19-20` | stale wording | `cargo test -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`build_xmod_from_json`, `xmod_from_json`) | module-boundary cleanup in mesher storage/event flow | S04 | Rewrite the stale `from_json` rationale, but preserve the still-real ORM-boundary notes called out in the mixed-truth section. |
| Complex `case` in a service call body | `mesher/services/user.mpl:18-20` | stale | `cargo test -p meshc --test e2e e2e_m032_supported_service_call_case -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`build_service_call_case`, `service_call_case`) | mesher service cleanup | S03 | Inline or simplify the helper only if behavior stays identical; do not confuse this with the still-real case-arm keep-sites. |
| `if/else` inside cast handlers | `mesher/services/stream_manager.mpl:125` | stale | `cargo test -p meshc --test e2e e2e_m032_supported_cast_if_else -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`build_cast_if_else`, `cast_if_else`) | mesher service cleanup | S03 | Retire the stale parser-limitation comment; the real keep-site in this file is the nested-`&&` helper at line 63. |

## Real Blockers

| Family | Mesher site(s) | Status | Proof surface | Likely owning subsystem | Next slice | Action |
|---|---|---|---|---|---|---|
| Imported inferred-polymorphic export (`xmod_identity`) | `mesher/storage/writer.mpl:4-5` | real blocker | `cargo test -p meshc --test e2e e2e_m032_limit_xmod_identity -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`xmod_identity`) | import/export scheme handling and LLVM signature emission across `mesh-typeck` + codegen | S02 | Start here. Fix the real imported-polymorphic export bug before touching the surrounding workaround family. Do not spend S02 on the already-green `from_json` path. |

## Real Keep-Sites

| Family | Mesher site(s) | Status | Proof surface | Likely owning subsystem | Next slice | Action |
|---|---|---|---|---|---|---|
| HTTP route closures only fail at live request time | `mesher/ingestion/routes.mpl:2` | real keep | `cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture`; `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`route_bare_server`, `route_closure_server`) | runtime HTTP router / closure environment plumbing | S05 | Keep the bare-function guidance until route handlers can preserve closure env through the runtime path. Compile-only `meshc build` is not an authoritative proof here. |
| Nested `&&` inside nested `if` blocks | `mesher/services/stream_manager.mpl:63` | real keep | `cargo test -p meshc --test e2e e2e_m032_limit_nested_and -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`nested_and`) | codegen | S05 | Keep `both_match(...)` or an equivalent helper until the PHI mismatch is fixed in Mesh. |
| `Timer.send_after` does not satisfy service cast dispatch | `mesher/services/writer.mpl:173-175`; `mesher/ingestion/pipeline.mpl:81-82` | real keep | `cargo test -p meshc --test e2e e2e_m032_limit_timer_service_cast -- --nocapture`; `bash scripts/verify-m032-s01.sh` (`build_timer_service_cast`, `timer_service_cast`) | runtime/service dispatch | S05 | Keep the `Timer.sleep + recursive actor` pattern for service-triggered tickers until timer delivery can carry service-cast tags. |
| Multi-statement `case` arms still require `-> do ... end` | `mesher/services/event_processor.mpl:105`; `mesher/ingestion/fingerprint.mpl:53`; `mesher/services/retention.mpl:8`; `mesher/ingestion/pipeline.mpl:108,293`; `mesher/api/team.mpl:59,78` | real keep | `rg -n "parse_match_arm|single expression, or a do\.\.\.end block|expected `end` to close case arm `do` block" compiler/mesh-parser/src/parser/expressions.rs` | parser | S05 | Keep helper extraction comments that are genuinely about case-arm shape. If a future slice wants to retire them, it needs parser work or a new proof that arm bodies have changed. |

## Mixed-Truth Comments

| File / cluster | Why it is mixed | Stale side proof | Real side proof | Next slice | Action |
|---|---|---|---|---|---|
| `mesher/storage/writer.mpl:4-5` | The “services must live in `main.mpl`” implication is stale, but the inferred-polymorphic cross-module export failure is real. | `cargo test -q -p meshc --test e2e e2e_cross_module_service -- --nocapture` proves cross-module service imports already work. | `cargo test -p meshc --test e2e e2e_m032_limit_xmod_identity -- --nocapture` proves the imported inferred export still fails. | S02 | Rewrite the comment around the real blocker only. Do not preserve stale service-export wording. |
| `mesher/storage/writer.mpl:16-20` | The `from_json` reason is stale, but the `INSERT ... SELECT` JSONB extraction rationale is still real. | `cargo test -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture` | `rg -n "^# ORM boundary:|Intentional raw SQL" mesher/storage/writer.mpl` plus `cargo run -q -p meshc -- build mesher` | S04 | Remove the stale `from_json` explanation and keep the honest SQL-boundary rationale. |
| `mesher/storage/queries.mpl:482-489` | The `from_json` explanation is stale, but the fingerprint SQL comment describes a still-real ORM expressiveness boundary. | `cargo test -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture` | `rg -n "^# ORM boundary:|extract_event_fields|Intentional raw SQL" mesher/storage/queries.mpl` | S04 | Rewrite only the stale module-boundary wording. Preserve the CASE / `jsonb_array_elements` / `string_agg` boundary note unless M033 proves otherwise. |
| `mesher/services/event_processor.mpl:5,105,120` | The `from_json` motivation is stale, but the case-arm helper note is still truthful. | `cargo test -p meshc --test e2e e2e_m032_supported_cross_module_from_json -- --nocapture` | `rg -n "parse_match_arm|single expression, or a do\.\.\.end block" compiler/mesh-parser/src/parser/expressions.rs` | S04 for the stale wording, S05 for the keep-site ledger | Split the comment family instead of deleting it wholesale. The helper at line 105 is still justified even though the `from_json` explanation is not. |
| `mesher/types/event.mpl:55`; `mesher/types/issue.mpl:14` | These mention `from_json`, but they describe row-shape reality rather than a Mesh limitation claim. | n/a | `rg -n "from_json" mesher/types/event.mpl mesher/types/issue.mpl` | S05 | Leave these alone during stale-folklore cleanup. They are data-shape notes, not limitation folklore. |

## Next-Slice Handoff

### S02 — Cross-module and inferred-export blocker retirement
- Start from `compiler/meshc/tests/e2e.rs::e2e_m032_limit_xmod_identity` and `.tmp/m032-s01/xmod_identity/`.
- Treat `mesher/storage/writer.mpl:4-5` as a mixed-truth clue, not as a comment to preserve verbatim.
- Do not spend time on `from_json` in S02. That family is already green on the real CLI path.

### S03 — Request, handler, and control-flow dogfood cleanup
- Safe stale-cleanup targets are `mesher/ingestion/routes.mpl:445`, `mesher/services/user.mpl:18-20`, and `mesher/services/stream_manager.mpl:125`.
- Keep your hands off the real keep-sites in the same neighborhoods: `mesher/ingestion/routes.mpl:2` and `mesher/services/stream_manager.mpl:63`.
- The proof bundle for S03 is already in `e2e_m032_supported_*` plus `cargo run -q -p meshc -- build mesher` / `fmt --check mesher`.

### S04 — Module-boundary JSON and workaround convergence
- Rewrite the stale `from_json` wording in `mesher/services/event_processor.mpl`, `mesher/storage/queries.mpl`, and `mesher/storage/writer.mpl`.
- Preserve the honest raw-SQL boundary notes in `storage/queries.mpl` and `storage/writer.mpl`; S04 is comment surgery and usage cleanup, not a fake ORM rewrite.
- `e2e_m032_supported_cross_module_from_json` is the authoritative guardrail for this work.

### S05 — Integrated mesher proof and retained-limit ledger
- The real keep-list to re-check is: route closures, nested `&&`, timer-to-service-cast, single-expression case-arm helpers, and any still-honest ORM-boundary comments.
- Route closures require live-request proof. Reuse `e2e_m032_route_bare_handler_control`, `e2e_m032_route_closure_runtime_failure`, and `scripts/verify-m032-s01.sh`; do not rely on `meshc build` alone.
- `types/event.mpl` and `types/issue.mpl` should not be counted as stale limitation comments during the final grep-based reconciliation.

## Verification

Across the slice, the durable proof surfaces now exist and pass:

- `cargo test -p meshc --test e2e m032_ -- --nocapture`
- `cargo test -p meshc --test e2e m032_limit -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`
- `bash scripts/verify-m032-s01.sh`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`

This summary itself is the matrix surface the later slices consume.

## Requirements Advanced

- R011 — S01 turned real `mesher/` friction into a named proof inventory instead of speculative compiler work.
- R013 — S01 identified the concrete Mesh blocker (`xmod_identity`) that S02 must fix instead of perpetuating workarounds indefinitely.
- R035 — S01 replaced folklore with a current classification, proof surface, and owner for each audited limitation family.

## Requirements Validated

- none — S01 proved and classified the workaround families, but the actual comment cleanup and blocker retirement land in S02-S05.

## New Requirements Surfaced

- none

## Requirements Invalidated or Re-scoped

- none

## Deviations

None.

## Known Limitations

- `mesher/` still contains the stale comments and workaround structure that later slices must retire; S01 classified them but did not rewrite product code.
- The real keep-sites remain real today: route closures, nested `&&`, timer-to-service-cast, and multi-statement `case` arms.
- The `storage/queries.mpl` / `storage/writer.mpl` raw-SQL boundaries remain comment-level truth only in M032; any honest ORM expansion is later M033 work.

## Follow-ups

- S02 should add or keep a control proving cross-module service imports still pass while inferred polymorphic exports fail, so the fix does not regress the already-supported path.
- S03 should remove only the stale request/handler folklore and leave the real keep-sites intact.
- S04 should rewrite mixed-truth `from_json` comments surgically instead of deleting raw-SQL rationale.
- S05 should treat this file plus `scripts/verify-m032-s01.sh` as the authoritative retained-limit ledger.

## Files Created/Modified

- `compiler/meshc/tests/e2e.rs` — added `e2e_m032_supported_*` and `e2e_m032_limit_*` CLI-path proofs that anchor the stale-vs-real split.
- `compiler/meshc/tests/e2e_stdlib.rs` — added the live bare-route control and closure-route runtime failure proof.
- `scripts/verify-m032-s01.sh` — added the repo-root replay script for the full M032/S01 matrix, including failure-path artifacts.
- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` — published the authoritative limitation matrix and downstream handoff.

## Forward Intelligence

### What the next slice should know
- `xmod_identity` is the real priority blocker. The stale `from_json` family is adjacent noise, not the fix target.
- `mesher/storage/writer.mpl` and `mesher/storage/queries.mpl` are mixed-truth files. Rewrite them surgically.
- The case-arm keep-sites extend beyond the original research focus: `api/team.mpl`, `ingestion/fingerprint.mpl`, `services/retention.mpl`, and `ingestion/pipeline.mpl` all still carry real extraction rationale.

### What's fragile
- Route-closure classification is fragile if anyone stops at `meshc build`; the compile path passes while the live request path fails.
- The inferred-export comment cluster is fragile because one stale sentence sits next to the best clue for the real blocker.

### Authoritative diagnostics
- `bash scripts/verify-m032-s01.sh` — fastest end-to-end replay of the supported paths, retained failures, live route behavior, and mesher baseline.
- `compiler/meshc/tests/e2e.rs` / `compiler/meshc/tests/e2e_stdlib.rs` — stable named proofs when a future agent needs a single failing family instead of the whole matrix.
- `.tmp/m032-s01/verify/` — failure artifacts when the replay script catches drift.

### What assumptions changed
- “Cross-module `from_json` is the important module-boundary blocker.” — Actually false; it already works on the real CLI path.
- “If closure routes build, HTTP routing supports closures.” — Also false; live request proof is required.
- “Only the originally-audited case-arm helper files matter.” — Also false; there are additional real keep-sites under `api/team.mpl`, `services/retention.mpl`, and `ingestion/pipeline.mpl`.
