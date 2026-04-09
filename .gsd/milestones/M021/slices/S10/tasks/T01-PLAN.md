# T01: 114-compile-run-and-end-to-end-verification 01

**Slice:** S10 — **Milestone:** M021

## Description

Verify zero-error compilation of Mesher with the fully rewritten ORM query layer, confirm successful startup with PostgreSQL, run migrations, and confirm the MirType::Tuple SIGSEGV fix is active.

Purpose: Phase 113 completed the ORM rewrite. Phase 114 must prove the full build pipeline works end-to-end before HTTP/WS endpoint testing begins.
Output: A freshly compiled `mesher/mesher` binary, confirmed startup log, and documented SIGSEGV resolution status.

## Must-Haves

- [ ] "`/Users/sn0w/Documents/dev/snow/target/debug/meshc build mesher` exits with code 0 and produces the `mesher/mesher` binary"
- [ ] "Mesher starts, connects to PostgreSQL at postgres://mesh:mesh@localhost:5432/mesher, and prints `[Mesher] Foundation ready`"
- [ ] "`meshc migrate up` completes without error (schema is up to date)"
- [ ] "The MirType::Tuple SIGSEGV fix is confirmed present in types.rs and the live binary does not crash on the first authenticated event POST"

## Files

- `mesher/mesher`
- `SERVICE_CALL_SEGFAULT.md`
