# S01 Replan

**Milestone:** M033
**Slice:** S01
**Blocker Task:** T02
**Created:** 2026-03-25T07:02:17.465Z

## Blocker Description

T02 reproduced a clean-start Mesher ingest failure: the first /api/v1/events request returns HTTP 429 with the seeded default API key before any S01 mutation or upsert assertions run. That makes the original live-ingest acceptance work unexecutable as written. T02 also confirmed that meshc build mesher and the e2e harness race on shared mesher/mesher(.o) outputs, so verification must stay serialized while gathering proof.

## What Changed

Repurpose the remaining work around the real blocker. T03 now retires the upstream ingest/rate-limit failure on a fresh Mesher boot instead of assuming the live ingest path already works. Add T04 to re-prove the already-landed expression rewrites on real Mesher routes and rerun the slice acceptance/keep-list checks once ingest is truthful again. This reflects repo reality: the neutral write rewrites, issue upsert expression path, and verify script already exist, so the remaining work is blocker repair plus honest end-to-end verification rather than re-implementing those features.
