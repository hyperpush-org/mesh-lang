# S05 Replan

**Milestone:** M047
**Slice:** S05
**Blocker Task:** T02
**Created:** 2026-04-01T16:15:50.115Z

## Blocker Description

T02 proved the planned route-free scaffold/example cutover cannot proceed yet because declared-work wrapper codegen still rejects zero-ceremony `@cluster` functions unless the public source signature exposes `request_key` and `attempt_id` arguments. A minimal temp package with `@cluster pub fn add() -> Int` still fails during `meshc build` with `declared work wrapper '__declared_work_work_add' expected request_key and attempt_id arguments`, so scaffold and example rebaselining must wait for the compiler/runtime seam to land first.

## What Changed

Reordered the remaining work around the real blocker. The old T03/T04 Todo tasks assumed the no-ceremony clustered-function contract already existed; they now become: (1) land the declared-work wrapper/codegen seam that hides continuity metadata from public `@cluster` signatures, (2) resume the route-free scaffold/example cutover on that corrected contract, (3) add the opt-in SQLite Todo starter, and (4) prove the generated Todo starter end to end. Threat surface and requirement coverage stay the same; the replan only changes execution order so the slice stops dogfooding a compiler contract that does not actually exist yet.
