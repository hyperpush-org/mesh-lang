# S01: Limitation Truth Audit and Repro Matrix

**Goal:** Turn the audited `mesher/` limitation folklore into durable truth: supported patterns get permanent CLI-path proofs, still-real limits get reproducible failure-path checks, and the slice leaves a precise handoff for S02/S03 instead of one-off research notes.
**Demo:** `cargo test -p meshc --test e2e m032_ -- --nocapture`, `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture`, and `bash scripts/verify-m032-s01.sh` all pass; `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` classifies the audited mesher sites as stale vs real with concrete repros and likely owners.

## Must-Haves

- Permanent `meshc` e2e coverage proves the stale-folklore paths already supported today: query-string access, cross-module `from_json`, service-call `case` bodies, and cast-handler `if/else`.
- Permanent failure-path coverage proves the still-real limits that later slices must respect or fix: cross-module inferred polymorphic export, nested `&&` in nested `if`, timer-to-service-cast mismatch, and HTTP route closures failing on live requests.
- The slice publishes an audit matrix tying each mesher workaround family to exact source sites, proof commands/test names, status (`stale`, `real blocker`, `real keep`), and next-slice owner.
- `cargo run -q -p meshc -- fmt --check mesher` and `cargo run -q -p meshc -- build mesher` stay green so the audit is anchored to current dogfood reality.

## Proof Level

- This slice proves: contract
- Real runtime required: yes
- Human/UAT required: no

## Verification

- `cargo test -p meshc --test e2e m032_ -- --nocapture`
- `cargo test -p meshc --test e2e m032_limit -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib m032_ -- --nocapture`
- `cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`
- `bash scripts/verify-m032-s01.sh`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- build mesher`
- `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md`

## Observability / Diagnostics

- Runtime signals: LLVM verifier errors for `xmod_identity` / nested `&&`, route-server stderr and socket behavior for closure routes, timer-service stdout staying `0`
- Inspection surfaces: `compiler/meshc/tests/e2e.rs`, `compiler/meshc/tests/e2e_stdlib.rs`, `scripts/verify-m032-s01.sh`, `.tmp/m032-s01/*`, and the mesher comment inventory in `S01-SUMMARY.md`
- Failure visibility: each retained-limit proof must surface the exact failing fixture/test name plus the key stderr or runtime symptom that made the classification true
- Redaction constraints: none; keep proofs local and do not depend on repo secrets or external services

## Integration Closure

- Upstream surfaces consumed: `mesher/` workaround comments, compiler CLI e2e harnesses in `compiler/meshc/tests`, HTTP runtime behavior, existing `.tmp/m032-s01` repro fixtures
- New wiring introduced in this slice: a slice verification script that replays the audited matrix and ties mesher comment families to durable compiler/runtime proofs
- What remains before the milestone is truly usable end-to-end: S02 must fix the inferred-export blocker in Mesh and S03/S04 must retire the stale mesher workarounds the audit proves unnecessary

## Tasks

- [x] **T01: Encode stale-folklore paths as CLI e2e proofs** `est:1.5h`
  - Why: S01 cannot hand later slices a prose-only claim that the folklore is stale; the supported patterns need durable tests on the real `meshc` path.
  - Files: `compiler/meshc/tests/e2e.rs`, `.tmp/m032-s01/request_query/main.mpl`, `.tmp/m032-s01/xmod_from_json/main.mpl`, `.tmp/m032-s01/xmod_from_json/models.mpl`, `.tmp/m032-s01/service_call_case/main.mpl`, `.tmp/m032-s01/cast_if_else/main.mpl`
  - Do: Add `e2e_m032_supported_*` tests in `compiler/meshc/tests/e2e.rs` using the audited fixture shapes as the source of truth. Cover `Request.query(...)`, cross-module `User.from_json`, inline `case` inside a service call body, and inline `if/else` inside a cast handler. Assert real build/run behavior or exact stdout, not just parse success, and keep the test names/comments tied to the mesher folklore they retire.
  - Verify: `cargo test -p meshc --test e2e m032_supported -- --nocapture`
  - Done when: the stale-supported families are encoded as passing `meshc` e2e tests with names that later slices can reuse as proof.
- [x] **T02: Capture live blocker and retained-limit proofs in automation** `est:2h`
  - Why: S02/S03 need a trustworthy failing surface for the real limits, especially the cross-module inferred-export blocker hiding behind `mesher/storage/writer.mpl`.
  - Files: `compiler/meshc/tests/e2e.rs`, `compiler/meshc/tests/e2e_stdlib.rs`, `scripts/verify-m032-s01.sh`, `.tmp/m032-s01/xmod_identity/main.mpl`, `.tmp/m032-s01/xmod_identity/utils.mpl`, `.tmp/m032-s01/nested_and/main.mpl`, `.tmp/m032-s01/timer_service_cast/main.mpl`, `.tmp/m032-s01/route_closure_server/main.mpl`, `.tmp/m032-s01/route_bare_server/main.mpl`
  - Do: Add `e2e_m032_limit_*` tests for the real blocker/failure paths: `xmod_identity` must fail with the imported-polymorphic LLVM call-signature mismatch, nested `&&` must fail with the LLVM PHI mismatch, and the timer-to-service-cast repro must still print `0`. Add a stdlib runtime test that proves bare HTTP handlers succeed while closure handlers still crash or return an empty reply on a live request. Create `scripts/verify-m032-s01.sh` to replay the audited `.tmp/m032-s01` matrix from repo root and fail with the exact drifted command or symptom.
  - Verify: `cargo test -p meshc --test e2e m032_limit -- --nocapture`; `cargo test -p meshc --test e2e_stdlib m032_route -- --nocapture`; `bash scripts/verify-m032-s01.sh`
  - Done when: the live blocker and retained-limit families are reproducible by automation instead of only by research notes.
- [x] **T03: Publish the mesher limitation matrix and handoff** `est:1h`
  - Why: The slice demo is the classified audit itself, not just a pile of tests; later slices need a precise map of which mesher comments are stale, which are still truthful, and who owns the fix.
  - Files: `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md`, `compiler/meshc/tests/e2e.rs`, `compiler/meshc/tests/e2e_stdlib.rs`, `scripts/verify-m032-s01.sh`, `mesher/ingestion/routes.mpl`, `mesher/services/event_processor.mpl`, `mesher/services/stream_manager.mpl`, `mesher/storage/writer.mpl`, `mesher/storage/queries.mpl`, `mesher/services/writer.mpl`, `mesher/ingestion/pipeline.mpl`
  - Do: Write `S01-SUMMARY.md` as the authoritative audit matrix. For each audited family, record the mesher site(s), status (`stale`, `real blocker`, `real keep`), concrete proof command or test name, likely owning subsystem, and the next slice that should act on it. Call out the mixed-truth files (`storage/writer.mpl`, `storage/queries.mpl`), the route-closure runtime trap, and `xmod_identity` as the S02 root-cause target. Use `rg` over `mesher/` to pull in any remaining single-expression case-arm or timer keep-sites so the matrix is complete rather than illustrative.
  - Verify: `test -s .gsd/milestones/M032/slices/S01/S01-SUMMARY.md`; `bash scripts/verify-m032-s01.sh`
  - Done when: a fresh agent can read `S01-SUMMARY.md` and know exactly which mesher workaround families are stale vs real, what proves it, and which slice should touch them next.

## Files Likely Touched

- `compiler/meshc/tests/e2e.rs`
- `compiler/meshc/tests/e2e_stdlib.rs`
- `scripts/verify-m032-s01.sh`
- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md`
- `mesher/ingestion/routes.mpl`
- `mesher/services/event_processor.mpl`
- `mesher/services/stream_manager.mpl`
- `mesher/storage/writer.mpl`
- `mesher/storage/queries.mpl`
- `mesher/services/writer.mpl`
- `mesher/ingestion/pipeline.mpl`
