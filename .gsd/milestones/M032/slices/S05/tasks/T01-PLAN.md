---
estimated_steps: 4
estimated_files: 3
skills_used:
  - debug-like-expert
  - test
---

# T01: Freeze the last two supported paths and correct the remaining overbroad Mesher comments

**Slice:** S05 — Integrated mesher proof and retained-limit ledger
**Milestone:** M032

## Description

Add durable proof for the last two truth-surface drifts before touching the final ledger. `mesher/ingestion/routes.mpl` currently overstates the bulk-array limitation, and `mesher/services/writer.mpl` still blames service-dispatch codegen for a helper extraction that Mesh now handles fine. This task should add named regressions to `compiler/meshc/tests/e2e.rs`, then narrow those comments to the real current story without changing product behavior or erasing the adjacent real keep-sites.

## Steps

1. Extend `compiler/meshc/tests/e2e.rs` with a named regression proving a `deriving(Json)` wrapper type with a `List < ... >` field can decode nested JSON array payloads, so the bulk-ingest comment can say the real missing surface is bare top-level list decoding in this endpoint rather than array parsing in general.
2. Extend `compiler/meshc/tests/e2e.rs` with a named regression proving a writer-style `cast ... do|state|` body can inline the `List.append(...)`, `new_len`, capacity branch, and rebuilt state struct logic that `writer_store(...)` currently keeps in a helper.
3. Rewrite the `handle_bulk_authed(...)` comment in `mesher/ingestion/routes.mpl` and the helper rationale above `writer_store(...)` in `mesher/services/writer.mpl` to match those proofs; keep the top-level route-closure note and the `Timer.send_after` keep-sites untouched.
4. Run the two new tests plus the stale-phrase and retained-keep-site greps. If any proof fails, fix the truth surface before finishing rather than weakening the comment.

## Must-Haves

- [ ] `compiler/meshc/tests/e2e.rs` contains named tests `e2e_m032_supported_nested_wrapper_list_from_json` and `e2e_m032_supported_inline_writer_cast_body` with real assertions.
- [ ] `mesher/ingestion/routes.mpl` no longer claims individual JSON array parsing is unsupported at the Mesh language level; it states the narrower bare top-level bulk-array limitation for this endpoint.
- [ ] `mesher/services/writer.mpl` no longer blames “complex expressions inside service dispatch codegen”; any remaining helper comment is framed as readability or local reuse only.
- [ ] The route-closure and timer keep-site comments stay intact in `mesher/ingestion/routes.mpl`, `mesher/services/writer.mpl`, and `mesher/ingestion/pipeline.mpl`.

## Verification

- `cargo test -q -p meshc --test e2e e2e_m032_supported_nested_wrapper_list_from_json -- --nocapture`
- `cargo test -q -p meshc --test e2e e2e_m032_supported_inline_writer_cast_body -- --nocapture`
- `bash -lc '! rg -n "not supported at the Mesh language level|complex expressions inside service dispatch codegen" mesher/ingestion/routes.mpl mesher/services/writer.mpl'`
- `rg -n "HTTP routing does not support closures|Timer.send_after delivers raw bytes" mesher/ingestion/routes.mpl mesher/services/writer.mpl mesher/ingestion/pipeline.mpl`

## Observability Impact

- Signals added/changed: two named `e2e_m032_*` tests become the durable proof surface for the final two supported-path claims; runtime behavior and logs stay unchanged.
- How a future agent inspects this: run the two named tests, then compare the bulk-route and writer-helper comments against the retained route-closure and timer greps.
- Failure state exposed: the failing test name or grep match tells the next agent whether the truth drift is in nested JSON decode support, inline cast-body support, or an over-broad comment that resurfaced.

## Inputs

- `compiler/meshc/tests/e2e.rs` — existing M032 supported/retained proof cluster to extend.
- `mesher/ingestion/routes.mpl` — bulk-ingest overstatement that needs to be narrowed without touching the route-closure keep-site.
- `mesher/services/writer.mpl` — stale helper rationale to rewrite without touching the real timer keep-site.
- `mesher/ingestion/pipeline.mpl` — companion timer keep-site that must stay aligned with `mesher/services/writer.mpl`.

## Expected Output

- `compiler/meshc/tests/e2e.rs` — new named supported-path regressions for nested wrapper-list decode and inline writer-style cast logic.
- `mesher/ingestion/routes.mpl` — narrowed bulk-array limitation comment anchored to the real current surface.
- `mesher/services/writer.mpl` — helper rationale rewritten as readability/locality instead of stale codegen folklore.
