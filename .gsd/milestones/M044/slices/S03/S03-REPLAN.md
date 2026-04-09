# S03 Replan

**Milestone:** M044
**Slice:** S03
**Blocker Task:** T02
**Created:** 2026-03-30T00:27:59.500Z

## Blocker Description

The current mesh-rt operator query helpers require a live node session and connect through the ordinary cluster peer path, so a straightforward `meshc cluster` implementation would join the inspected cluster, exchange peer/continuity sync, and pollute the membership snapshot it is supposed to report. S03 needs a transient authenticated operator query path that never registers a cluster session before the public CLI can ship honestly.

## What Changed

Inserted a new runtime-first task to add a transient authenticated non-registering operator transport, then moved the public `meshc cluster` work behind that seam. The clustered scaffold and slice closeout move back one slot so they build on the truthful CLI surface instead of the session-based query path. The slice remains read-only on the operator side, but the threat surface now explicitly includes an authenticated transient operator endpoint that must stay non-registering, bounded, and cookie-safe.
