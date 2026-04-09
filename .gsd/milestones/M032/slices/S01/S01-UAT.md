# S01: Limitation truth audit and repro matrix — UAT

`bash scripts/verify-m032-s01.sh`

**Milestone:** M032
**Written:** 2026-03-24

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S01 closed with a proof matrix, a replay script, named Cargo filters, and durable verification artifacts. This UAT should replay those current surfaces, not invent a historical snapshot of pre-S02 failures.

## Preconditions

- Run from repo root: `/Users/sn0w/Documents/dev/mesh-lang`
- Rust/Cargo and `rg` are available
- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` exists and remains the authoritative stale-vs-real matrix
- `scripts/verify-m032-s01.sh` and `.tmp/m032-s01/verify/` are present
- Treat the repo as post-S02/S03/S04/S05 when interpreting results: this UAT replays current proof surfaces, not old broken states

## Smoke Test

1. Run `bash scripts/verify-m032-s01.sh`
2. **Expected:** exit code 0 and `verify-m032-s01: ok`
3. If the command fails, inspect `.tmp/m032-s01/verify/` first before changing the UAT wording or reopening compiler/Mesher work

## Test Cases

### 1. Replay the integrated S01 proof bundle

1. Run `bash scripts/verify-m032-s01.sh`
2. **Expected:** the replay stays green across the supported-now probes (`request_query`, `xmod_from_json`, `service_call_case`, `cast_if_else`, `xmod_identity`), the retained-limit probes (`nested_and`, `Timer.send_after` service-cast behavior), the live route controls, and the Mesher `fmt --check` / `build` baseline.
3. **Expected:** failures leave named logs in `.tmp/m032-s01/verify/` instead of forcing a future agent to rediscover the proof surface.

### 2. The S01 matrix still names the real classification and handoff

1. Run `rg -n "Stale Folklore|Real Blockers|Real Keep-Sites|Mixed-Truth Comments|Next-Slice Handoff|xmod_identity|route closures|Timer.send_after" .gsd/milestones/M032/slices/S01/S01-SUMMARY.md`
2. **Expected:** the summary still contains the section landmarks and the named families that matter for S01 truthfulness: `xmod_identity` as the real blocker/handoff family, route closures as a retained live-request keep-site, and `Timer.send_after` as a retained runtime keep-site.
3. **Expected:** this check stays tied to the summary matrix instead of inventing alternate acceptance criteria inside the UAT.

### 3. The broad `m032_` proof filters still replay non-empty named tests

1. Run `cargo test -q -p meshc --test e2e m032_ -- --nocapture`
2. Confirm the output says `running [1-9][0-9]* tests`
3. Run `cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture`
4. Confirm the output says `running [1-9][0-9]* tests`
5. **Expected:** both commands pass, and neither command is trusted unless the test count is non-zero.
6. **Expected:** the `compiler/meshc/tests/e2e.rs` filter still covers the named S01 proof family, including the `.tmp/m032-s01/xmod_identity/` fixture via `m032_inferred_cross_module_identity`, while `compiler/meshc/tests/e2e_stdlib.rs` still covers the route bare-function control and the route-closure runtime failure.

### 4. The acceptance artifact itself stays current and non-placeholder

1. Run the slice-plan placeholder-absence grep against `.gsd/milestones/M032/slices/S01/S01-UAT.md` and expect no matches.
2. Run `rg -n "verify-m032-s01|cargo test -q -p meshc --test e2e m032_|cargo test -q -p meshc --test e2e_stdlib m032_|xmod_identity|route closures|Timer.send_after|Zero-test false positives|\.tmp/m032-s01/verify" .gsd/milestones/M032/slices/S01/S01-UAT.md`
3. **Expected:** the UAT remains anchored to the current replay script, the broad filters, the named `xmod_identity` family, the route closures warning, the `Timer.send_after` keep-site, the `Zero-test false positives` guard, and `.tmp/m032-s01/verify/` as the first debug surface.

## Edge Cases

### Zero-test false positives are unacceptable

1. Inspect the output from `cargo test -q -p meshc --test e2e m032_ -- --nocapture` and `cargo test -q -p meshc --test e2e_stdlib m032_ -- --nocapture`
2. **Expected:** neither command says `running 0 tests`. Zero-test false positives are acceptance failures even if Cargo exits 0.

### Route closures require live-request proof

1. Treat the live request exercised by `bash scripts/verify-m032-s01.sh` and `e2e_m032_route_closure_runtime_failure` as the authoritative route-closure check
2. **Expected:** route closures remain classified from runtime/live-request evidence, not from compile-only success. A bare `meshc build` of the closure fixture is not sufficient proof.

### `xmod_identity` is current-proof context, not a required failure

1. Treat `xmod_identity` as a named S01 handoff/current-proof family that must stay visible in the replay bundle and summary matrix
2. **Expected:** this UAT does not demand the old pre-S02 failure to still reproduce. Current repo truth is that the family remains part of the S01 story and current proof surfaces, not that S06 re-breaks it for nostalgia.

## Failure Signals

- `bash scripts/verify-m032-s01.sh` fails or its logs under `.tmp/m032-s01/verify/` drift from the established proof bundle
- either broad Cargo filter exits 0 while reporting `running 0 tests`
- `.gsd/milestones/M032/slices/S01/S01-UAT.md` reintroduces placeholder wording or drops the current replay commands
- `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` loses `Stale Folklore`, `Real Blockers`, `Real Keep-Sites`, `Mixed-Truth Comments`, `Next-Slice Handoff`, `xmod_identity`, route closures, or `Timer.send_after`
- the UAT starts treating route closures as compile-only proof or `xmod_identity` as a failure that must still reproduce today

## Requirements Proved By This UAT

- R035 — the accepted S01 matrix now has a real replayable acceptance artifact instead of a doctor placeholder
- Supports R011 — the acceptance story stays grounded in named Mesher-facing proof surfaces rather than prose-only claims
- Supports R013 — the real blocker/handoff family remains visible through `xmod_identity` and the current proof bundle without rewriting repo history

## Not Proven By This UAT

- That route closures, nested `&&`, `Timer.send_after`, or parser-bound case-arm keep-sites are fixed; this UAT only proves they remain truthfully classified
- That historical pre-S02 failures still reproduce; current acceptance is based on current repo truth
- That later M033 data-layer work (`ORM boundary`, `PARTITION BY`) is implemented

## Notes for Tester

- Start from `bash scripts/verify-m032-s01.sh`; it is the fastest end-to-end truth surface for this slice
- When the replay script fails, inspect `.tmp/m032-s01/verify/` before editing docs or assumptions
- Keep the route closures warning explicit: live request behavior is authoritative, compile-only success is not
- If the `m032_` filters drift, read `compiler/meshc/tests/e2e.rs` and `compiler/meshc/tests/e2e_stdlib.rs` before rewriting the UAT
- Keep `.gsd/milestones/M032/slices/S01/S01-SUMMARY.md` as the matrix source of truth; this UAT should replay and reference it, not replace it
