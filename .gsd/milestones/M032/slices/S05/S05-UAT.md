# S05: Integrated mesher proof and retained-limit ledger — UAT

**Milestone:** M032
**Written:** 2026-03-24

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S05 closes a proof-and-truthfulness milestone. The acceptance bar is the integrated replay script, the named compiler/runtime proofs, Mesher fmt/build, and the final ledger text that maps supported-now paths, retained Mesh keep-sites, and M033 follow-on families.

## Preconditions

- Run from repo root: `/Users/sn0w/Documents/dev/mesh-lang`
- Rust/Cargo and `rg` are available
- The working tree contains `S05-SUMMARY.md`, `S05-UAT.md`, and the completed M032 roadmap entry
- Treat `.tmp/m032-s01/verify/*.log` as the first debug surface if the replay script fails

## Smoke Test

Run the integrated replay first:

1. `bash scripts/verify-m032-s01.sh`
2. **Expected:** exit code 0 and `verify-m032-s01: ok`. This proves the named Mesher proof bundle still replays cleanly before checking the closeout docs.

## Test Cases

### 1. Supported-now proofs still hold, including `xmod_identity`

1. Run `cargo test -q -p meshc --test e2e e2e_m032_supported_nested_wrapper_list_from_json -- --nocapture`
2. Run `cargo test -q -p meshc --test e2e e2e_m032_supported_inline_writer_cast_body -- --nocapture`
3. Run `cargo test -q -p meshc --test e2e m032_inferred -- --nocapture`
4. Confirm Cargo reports `running 1 test`, `running 1 test`, and `running 2 tests` respectively.
5. **Expected:** all three commands pass. The final command keeps `.tmp/m032-s01/xmod_identity` visible as a supported cross-module inferred-export path, not a lingering blocker.

### 2. The retained Mesh keep-sites are still real and still named by the right Mesher files

1. Run `cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture`
2. Run `rg -n "HTTP routing does not support closures|avoids && codegen issue inside nested if blocks|Timer.send_after delivers raw bytes|single-expression case arm constraint|single-expression case arms|case arm extraction|^# ORM boundary:|Migration DSL does not support PARTITION BY|from_json" mesher/ingestion/routes.mpl mesher/services/stream_manager.mpl mesher/services/writer.mpl mesher/ingestion/pipeline.mpl mesher/services/event_processor.mpl mesher/ingestion/fingerprint.mpl mesher/services/retention.mpl mesher/api/team.mpl mesher/storage/queries.mpl mesher/storage/writer.mpl mesher/migrations/20260216120000_create_initial_schema.mpl mesher/types/event.mpl mesher/types/issue.mpl`
3. **Expected:** the runtime failure control passes, and the grep output only shows the honest retained keep-sites: `HTTP routing does not support closures`, nested `&&`, `Timer.send_after`, parser-bound case-arm extraction, `ORM boundary`, `PARTITION BY`, and the truthful row-shape `from_json` notes.

### 3. Mesher still formats and builds on the cleaned codebase

1. Run `cargo run -q -p meshc -- fmt --check mesher`
2. Run `cargo run -q -p meshc -- build mesher`
3. **Expected:** both commands exit 0. The retained-limit ledger is only credible if the cleaned Mesher codebase still formats and builds.

### 4. The closeout docs carry the required supported-versus-retained split and the M033 handoff

1. Run `bash -lc 'test -s .gsd/milestones/M032/slices/S05/S05-SUMMARY.md && test -s .gsd/milestones/M032/slices/S05/S05-UAT.md'`
2. Run `rg -n "xmod_identity|HTTP routing does not support closures|Timer.send_after|ORM boundary|PARTITION BY|M033" .gsd/milestones/M032/slices/S05/S05-SUMMARY.md .gsd/milestones/M032/slices/S05/S05-UAT.md`
3. Run `rg -n "\[x\] \*\*S05: Integrated mesher proof and retained-limit ledger\*\*" .gsd/milestones/M032/M032-ROADMAP.md`
4. **Expected:** both artifacts exist, both mention `xmod_identity`, the retained keep-site strings, and the M033 handoff, and the roadmap shows S05 as completed.

## Edge Cases

### Zero-test false positives do not count as proof

1. Inspect Cargo output for the filtered test commands above.
2. **Expected:** none of the filtered commands say `running 0 tests`. A zero-test exit code is slice-incomplete coverage, not a passing replay.

### Negative stale-folklore sweep stays clean

1. Run `bash -lc '! rg -n "not supported at the Mesh language level|complex expressions inside service dispatch codegen|query string parsing not available in Mesh|complex case expressions|parser limitation with if/else in cast handlers|cross-module from_json limitation|from_json limitation per decision \[88-02\]|Validation is done by the caller|caller is responsible for JSON parsing and field validation|services and inferred/polymorphic functions cannot be exported across modules|must stay in main\.mpl" mesher'`
2. **Expected:** exit code 0 with no matches. This proves the disproven folklore has not crept back into Mesher while the honest retained keep-list remains present.

## Failure Signals

- `bash scripts/verify-m032-s01.sh` fails or its logs in `.tmp/m032-s01/verify/` drift from the replayed truth
- any filtered Cargo command reports `running 0 tests`
- `e2e_m032_route_closure_runtime_failure` stops failing at live request time or the retained keep-site grep loses `HTTP routing does not support closures`, nested `&&`, `Timer.send_after`, case-arm extraction, `ORM boundary`, or `PARTITION BY`
- `S05-SUMMARY.md` or `S05-UAT.md` loses `xmod_identity` or stops naming the M033 handoff families
- `meshc fmt --check mesher` or `meshc build mesher` regresses on the cleaned codebase

## Requirements Proved By This UAT

- R010 — the project now has a current evidence bundle for Mesh’s backend-development differentiators instead of vague comparison language
- R011 — the final M032 changes are still visibly grounded in real Mesher friction and replayable proof
- R013 — the repaired inferred-export path remains live through `xmod_identity`, `m032_inferred`, and Mesher build/fmt
- R035 — the remaining Mesher limitation comments match current verified reality

## Not Proven By This UAT

- That the remaining Mesh keep-sites are solved; route closures, nested `&&`, `Timer.send_after`, and single-expression case-arm extraction remain honest retained limits
- That the M033 `ORM boundary` and `PARTITION BY` families are implemented; this UAT only proves they are the truthful next pressure map
- Any future SQLite-specific extras beyond the explicit M033 design constraint

## Notes for Tester

- Start from `bash scripts/verify-m032-s01.sh`; do not jump straight into comment inspection unless the replay script points at a specific family.
- Treat `mesher/types/event.mpl` and `mesher/types/issue.mpl` `from_json` notes as row-shape documentation, not as retained limitation claims.
- If the ledger drifts, update proof first and wording second. The docs should follow the tests and keep-sites, not lead them.
