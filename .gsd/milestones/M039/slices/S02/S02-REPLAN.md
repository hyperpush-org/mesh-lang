# S02 Replan

**Milestone:** M039
**Slice:** S02
**Blocker Task:** T01
**Created:** 2026-03-28T10:39:01.446Z

## Blocker Description

The current /work design is blocked by two runtime constraints proven during T01: Mesh HTTP handlers are not actor contexts, so the handler cannot use self()/receive to wait for a worker reply, and distributed Node.spawn/send only safely carry raw scalar values, so the original string/struct-heavy request-reply protocol is not a valid cross-node return path.

## What Changed

Replaced the remaining single implementation/proof task with a staged plan that first moves /work onto a runtime-supported coordinator boundary, then adds direct-port e2e proof, then wraps that proof in the canonical verifier. The slice goal and read-only route surface stay the same, /membership remains unchanged, and the security/requirement posture does not expand beyond the original S02 scope; the material change is that cross-node completion must now flow through a registered actor/service seam with scalar-only transport instead of a handler mailbox.
