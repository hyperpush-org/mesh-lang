# S03 Research: Request, handler, and control-flow dogfood cleanup

## Summary

S03 is a narrow mesher cleanup slice with three named stale-folklore edits and two adjacent keep-sites that must remain untouched.

- `mesher/ingestion/routes.mpl:445-452` is stale. `handle_list_issues(...)` still hardcodes `"unresolved"` even though `Request.query(...)` already works both in mesher (`mesher/api/helpers.mpl:41-47`, `mesher/api/search.mpl:35-40`) and in the S01 CLI proof (`e2e_m032_supported_request_query`).
- `mesher/services/user.mpl:18-24,40-42` is stale. `login_user(...)` is a single-use wrapper extracted for a service-call-body limitation that the S01 proof already disproved.
- `mesher/services/stream_manager.mpl:125-131,240-241` is stale. `buffer_if_client(...)` is a single-use wrapper extracted for a cast-handler `if/else` limitation that the S01 proof already disproved.
- Do not touch the real keep-sites in the same neighborhoods:
  - `mesher/ingestion/routes.mpl:2` bare-function route guidance is still real because closure routes still fail at live request time.
  - `mesher/services/stream_manager.mpl:63-70` `both_match(...)` is still real because nested `&&` still fails in codegen.
  - `mesher/services/writer.mpl:153-162` and `mesher/ingestion/pipeline.mpl:81-82` timer comments remain real and belong to S05.
- Current baseline is green: the focused S01 supported/keep-site tests, `meshc fmt --check mesher`, and `meshc build mesher` all passed during this research pass.

## Recommendation

Use the `debug-like-expert` rules here: **VERIFY, DON'T ASSUME** and **NO DRIVE-BY FIXES**.

Plan S03 as direct dogfood cleanup, not compiler work.

Recommended edit order:

1. **`mesher/ingestion/routes.mpl`**
   - Replace the hardcoded `"unresolved"` with direct inline `Request.query(request, "status")` defaulting to `"unresolved"`.
   - Remove or rewrite the stale comment.
   - Keep the top-of-file route-closure keep-site comment intact.
   - Prefer direct `Request.query(...)` here instead of `Api.Helpers.query_or_default(...)`; S01 explicitly wanted this file to dogfood the supported request path directly.
2. **`mesher/services/user.mpl`**
   - Inline the `case authenticate_user(...)` expression directly inside `call Login(...)`.
   - Delete `login_user(...)` if no longer referenced.
   - Keep behavior identical: `Err(_) -> Err("authentication failed")`.
3. **`mesher/services/stream_manager.mpl`**
   - Inline the `if is_stream_client(...)` branch directly inside `cast BufferMessage(...)`.
   - Delete `buffer_if_client(...)` if no longer referenced.
   - Do not rewrite `matches_filter(...)` or `both_match(...)`.

Natural task seams:

- **Task A:** `mesher/ingestion/routes.mpl` request-query cleanup.
- **Task B:** `mesher/services/user.mpl` + `mesher/services/stream_manager.mpl` handler-body cleanup.

These are independent, but Task B needs the extra keep-site guard for nested `&&`.

Out of scope / avoid expansion:

- `mesher/services/writer.mpl:62-63` has a similar “service dispatch codegen” comment, but S01 did not freeze it as a named S03 target and the file also contains a real timer keep-site. Do not opportunistically refactor it in this slice without adding new proof first.
- S04 mixed-truth `from_json` wording in `mesher/services/event_processor.mpl`, `mesher/storage/queries.mpl`, and `mesher/storage/writer.mpl` is not S03 work.

## Requirements Targeted

- **R035** — primary S03 goal: stale limitation/workaround comments must become current.
- **R011** — the cleanup stays anchored to real mesher dogfood paths, not synthetic compiler examples.
- **Supports validated R013** — S02 already fixed the real blocker; S03 strengthens truthful dogfood by using currently supported Mesh patterns directly in mesher.

## Skills Discovered

- **Loaded:** `debug-like-expert`
  - Applied rules:
    - **VERIFY, DON'T ASSUME** — every stale cleanup target below has a named passing proof and every nearby keep-site has a named failing or behavioral proof.
    - **NO DRIVE-BY FIXES** — adjacent real keep-sites are called out explicitly so S03 does not “clean up” verified limitations.
- **Skill search performed:**
  - `npx skills find "compiler tooling"`
- **Result:** no new skill installed. Returned options were generic compiler or LLVM tooling skills, not directly useful for a small repo-local Mesh dogfood cleanup slice.

## Implementation Landscape

### A. Request-query cleanup is one-file, direct dogfood work

**Target file**

- `mesher/ingestion/routes.mpl` — ingestion HTTP handlers

**Relevant lines**

- `mesher/ingestion/routes.mpl:2`
  - real keep-site: `# Handlers are bare functions (HTTP routing does not support closures).`
- `mesher/ingestion/routes.mpl:445-452`
  - stale site: `handle_list_issues(...)` still hardcodes `"unresolved"` and says query parsing is unavailable
- `mesher/ingestion/routes.mpl:29`
  - currently imports only `require_param, get_registry` from `Api.Helpers`

**Reference patterns already in repo**

- `mesher/api/helpers.mpl:41-47`
  - `query_or_default(...)` shows the real `Request.query(...) -> Option<String>` contract
- `mesher/api/search.mpl:35-40`
  - `get_limit(request)` uses direct inline `Request.query(...)` with a `case` default

**Exact supported proof**

- `compiler/meshc/tests/e2e.rs:6830-6836` — `e2e_m032_supported_request_query`
- `.tmp/m032-s01/request_query/main.mpl`
  - inline `case Request.query(request, "status") do ... end` compiles on the real CLI path
- Re-run during research:

```bash
cargo test -q -p meshc --test e2e e2e_m032_supported_request_query -- --nocapture
```

Result: pass.

**Planning consequence**

This is the cleanest S03 edit. No helper extraction or compiler change is required. The direct pattern can live in `handle_list_issues(...)` without touching route registration or the real closure keep-site.

### B. Service-call-body cleanup in `user.mpl` is a safe single-use helper removal

**Target file**

- `mesher/services/user.mpl` — user auth/session service

**Relevant lines**

- `mesher/services/user.mpl:18-24`
  - stale helper rationale + single-use `login_user(...)`
- `mesher/services/user.mpl:40-42`
  - only call site inside `call Login(...)`

**Exact supported proof**

- `compiler/meshc/tests/e2e.rs:6858-6866` — `e2e_m032_supported_service_call_case`
- `.tmp/m032-s01/service_call_case/main.mpl`
  - proves a service `call` body can return `(state, case ... do ...)` directly
- Re-run during research:

```bash
cargo test -q -p meshc --test e2e e2e_m032_supported_service_call_case -- --nocapture
```

Result: pass.

**Behavior to preserve**

Current helper semantics are:

- `Ok(user)` → `create_session(pool, user.id)`
- `Err(_)` → `Err("authentication failed")`

**Planning consequence**

This file can likely be reduced to one edit:

- inline the `case authenticate_user(...)` into `call Login(...)`
- delete `login_user(...)`

No other handler in the file depends on this pattern.

### C. Cast-handler cleanup in `stream_manager.mpl` is safe only if the real `&&` keep-site stays untouched

**Target file**

- `mesher/services/stream_manager.mpl` — websocket streaming client state and buffering

**Relevant lines**

- `mesher/services/stream_manager.mpl:63-70`
  - real keep-site: `both_match(...)` exists because nested `&&` still fails in codegen
- `mesher/services/stream_manager.mpl:125-131`
  - stale helper rationale + single-use `buffer_if_client(...)`
- `mesher/services/stream_manager.mpl:240-241`
  - only call site inside `cast BufferMessage(...)`

**Exact supported proof**

- `compiler/meshc/tests/e2e.rs:6868-6876` — `e2e_m032_supported_cast_if_else`
- `.tmp/m032-s01/cast_if_else/main.mpl`
  - proves a service `cast` body can evaluate inline `if/else`
- Re-run during research:

```bash
cargo test -q -p meshc --test e2e e2e_m032_supported_cast_if_else -- --nocapture
```

Result: pass.

**Exact keep-site guard**

- `compiler/meshc/tests/e2e.rs:6914-6924` — `e2e_m032_limit_nested_and`
- Re-run during research:

```bash
cargo test -q -p meshc --test e2e e2e_m032_limit_nested_and -- --nocapture
```

Result: pass by observing the expected LLVM PHI failure.

**Planning consequence**

Inline only the `BufferMessage` branch. Do not fold `both_match(...)` away or rewrite `matches_filter(...)` to use `&&`; that would turn a comment cleanup into an S05 compiler regression.

### D. Neighboring work to explicitly avoid in S03

**Real keep-sites outside the three edit targets**

- `mesher/ingestion/routes.mpl:2`
  - still real: closure route support fails at live request time
  - guard: `compiler/meshc/tests/e2e_stdlib.rs:1992-2043`
    - `e2e_m032_route_bare_handler_control`
    - `e2e_m032_route_closure_runtime_failure`
  - re-run during research:

```bash
cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture
```

Result: pass.

- `mesher/services/writer.mpl:153-162`
- `mesher/ingestion/pipeline.mpl:81-82`
  - still real: `Timer.send_after` does not satisfy service cast dispatch
  - guard: `compiler/meshc/tests/e2e.rs:6927-6935` — `e2e_m032_limit_timer_service_cast`

**Mixed-truth / later-slice files**

- `mesher/services/event_processor.mpl`
- `mesher/storage/queries.mpl`
- `mesher/storage/writer.mpl`
  - S04 owns the `from_json` wording surgery, not S03

**Unevaluated nearby comment**

- `mesher/services/writer.mpl:62-63`
  - similar “service dispatch codegen” wording, but not part of the S01 named proof inventory
  - treat as out of scope unless execution adds a new proof and explicitly expands the slice

## Verification Plan

### Focused task-level checks

```bash
cargo test -q -p meshc --test e2e e2e_m032_supported_request_query -- --nocapture
cargo test -q -p meshc --test e2e e2e_m032_supported_service_call_case -- --nocapture
cargo test -q -p meshc --test e2e e2e_m032_supported_cast_if_else -- --nocapture
```

### Guardrail keep-site checks

```bash
cargo test -q -p meshc --test e2e e2e_m032_limit_nested_and -- --nocapture
cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture
```

### Mesher closeout checks

```bash
cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
```

### Optional broad replay

```bash
bash scripts/verify-m032-s01.sh
```

Use this as final slice-level confirmation, not as the first feedback loop.

### Comment-truth greps

Stale comment strings should be gone from the three S03 targets:

```bash
rg -n "query string parsing not available in Mesh|complex case expressions|parser limitation with if/else in cast handlers" mesher/ingestion/routes.mpl mesher/services/user.mpl mesher/services/stream_manager.mpl
```

Real keep-site wording should still exist where S01 says it is real:

```bash
rg -n "HTTP routing does not support closures|avoids && codegen issue inside nested if blocks|Timer.send_after delivers raw bytes" mesher/ingestion/routes.mpl mesher/services/stream_manager.mpl mesher/services/writer.mpl mesher/ingestion/pipeline.mpl
```

## Current Baseline Observed During Research

The following all passed from repo root during this scout pass:

```bash
cargo test -q -p meshc --test e2e e2e_m032_supported_request_query -- --nocapture
cargo test -q -p meshc --test e2e e2e_m032_supported_service_call_case -- --nocapture
cargo test -q -p meshc --test e2e e2e_m032_supported_cast_if_else -- --nocapture
cargo test -q -p meshc --test e2e e2e_m032_limit_nested_and -- --nocapture
cargo test -q -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture
cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
```

Observed result:

- all focused tests passed
- `meshc fmt --check mesher` was silent-success
- `meshc build mesher` produced `Compiled: mesher/mesher`

## Planner Notes

- S03 does not need compiler changes, library docs, or web research.
- The highest-risk mistake is accidental over-cleanup in `routes.mpl` or `stream_manager.mpl`, where stale and real limitation notes are close together.
- If the planner wants the smallest reversible execution order, do `routes.mpl` first, then `user.mpl`, then `stream_manager.mpl`, with a focused check after each file or pair.
