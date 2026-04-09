# M054: Load Balancing Truth & Follow-through — Context Draft

**Gathered:** 2026-04-02
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

- Do a deep dive into how load balancing actually works today.
- Answer the practical question: if a frontend points at one public server URL, where does balancing really happen?
- The preferred public model is server-side first, not frontend-held node awareness.
- The contract should stay platform-agnostic even if Fly is the first proving environment.
- Frontend-aware adapters stay deferred unless the deep dive proves the current server-side/runtime story is not enough.

## Technical Findings From Investigation

- Fly Proxy can distribute traffic across multiple Machines for a service.
- `Fly-Replay` can target a specific Machine or region after a request reaches an app, which is one possible server-side routing tool.
- Fly storage constraints make SQLite durability machine-local, so any balancing story must keep storage truth in mind.
- Existing public docs currently blur clustered apps, distributed primitives, and proof rails instead of explaining the balancing model directly.

## Likely Dependencies

- Depends on M053 because the serious starter deploy proof will show how routing behaves in a real proving environment.
- May feed back into M052 docs/site messaging if the current public balancing claims are too vague or too strong.

## Scope Seed

### Likely In Scope
- document the current balancing model honestly
- explain ingress versus runtime-owned routing clearly
- decide whether current server-side behavior is enough for the public Mesh story
- implement follow-through only if the current story is not sufficient

### Likely Out of Scope
- making browsers or frontend SDKs topology-aware by default
- Fly-only product positioning

## Questions For Dedicated Discussion

- What exact public claim should Mesh make about load balancing once this work is done?
- Is the missing piece explanation, runtime behavior, deploy guidance, or all three?
- When would frontend-aware adapters become justified instead of speculative?
- What proof bar demonstrates platform-agnostic truth rather than just one Fly-specific happy path?
