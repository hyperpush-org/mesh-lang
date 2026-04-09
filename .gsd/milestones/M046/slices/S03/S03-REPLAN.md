# S03 Replan

**Milestone:** M046
**Slice:** S03
**Blocker Task:** T03
**Created:** 2026-03-31T21:04:31.468Z

## Blocker Description

The user observed `tiny-cluster/` still carrying explicit delay-normalization and sleep helper code. That makes failover observability look app-owned instead of language-owned, which breaks the M046 honesty bar even if the proof otherwise works.

## What Changed

S03 no longer accepts any package-owned or user-facing delay helper as part of the tiny-cluster proof contract. Add a follow-up task that removes remaining timing seams from the app/operator surface and proves failover observability remains Mesh-owned.
