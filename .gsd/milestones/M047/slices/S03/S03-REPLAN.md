# S03 Replan

**Milestone:** M047
**Slice:** S03
**Blocker Task:** T01
**Created:** 2026-04-01T08:04:11.409Z

## Blocker Description

T01 never implemented the compiler-known `HTTP.clustered(...)` typing/inference seam. The remaining tasks assumed wrapper metadata already existed, but the blocker summary shows the real dependency is a new typeck-to-lowering metadata map plus wrapper-specific diagnostics/LSP updates before any route lowering or runtime reply work can proceed honestly.

## What Changed

Reinsert the missing compiler task at the front of the remaining plan, then shift the original lowering/runtime/e2e work down one slot. T02 now lands the blocked typeck/LSP metadata and misuse diagnostics; T03 handles generated route shims and declared-handler registration once that metadata exists; T04 focuses only on synchronous clustered route execution and reply transport; new T05 closes the slice with live HTTP proof plus S02/M032 regression controls. Security posture and requirement coverage stay the same, but the execution order now matches the real dependency chain discovered in T01.
