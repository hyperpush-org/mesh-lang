---
depends_on: [M034]
---

# M035: Test Framework Hardening — Context Draft

**Gathered:** 2026-03-26
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

The user wants Mesh’s test implementation reviewed and made production ready. They explicitly said the goal is for it to be ready to test the `mesher` dogfooding app thoroughly during development, and they do **not** want obvious maturity gaps that would make the framework feel behind other language testing frameworks.

They did **not** want to micromanage the exact feature list. They delegated that judgment, with the direction being: decide what “best” means, but make the result serious enough for daily app development.

## What This Milestone Likely Covers

- review `meshc test`, `.test.mpl`, assertion/runtime behavior, grouping, setup/teardown, actor helpers, and failure visibility as a coherent testing product surface
- identify the biggest gaps between the current Mesh testing story and what `mesher` would need for thorough day-to-day development
- harden semantics, isolation, async/test-actor behavior, fixture ergonomics, selection/filtering, and failure surfaces where the current framework is clearly behind the bar
- leave line-coverage, debugger/profiler depth, and broader observability work separate unless they become necessary for an honest test-framework claim

## Why This Needs Its Own Discussion

The current conversation established the bar, but not the exact feature priorities. There is still no explicit user decision on questions like:
- which missing capabilities matter most first
- whether the first proof target should be integration-heavy `mesher` flows, low-level framework semantics, or both
- how much parity with other language testing frameworks is actually necessary for the user’s intended daily-driver bar

A dedicated M035 discussion should turn that into a sharper capability contract and milestone-specific success criteria.

## Existing Codebase / Prior Art To Revisit

- `compiler/meshc/src/test_runner.rs`
- `compiler/mesh-rt/src/test.rs`
- `website/docs/docs/testing/index.md`
- existing `*.test.mpl` examples under `tests/e2e/`, `reference-backend/tests/`, and `mesher/tests/`

## Likely Risks / Unknowns

- the current framework may be good enough for small examples but still weak for real app-level async/integration work
- some missing capabilities may belong in the runtime or compiler lowering, not just the CLI runner
- “on par with other frameworks” is still too broad and needs a narrowed, concrete proof bar

## Likely Outcome When Done

Mesh has a testing story that `mesher` can rely on during development without obvious missing primitives, weak isolation, or embarrassing failure ergonomics.

## Open Questions For The Dedicated Discussion

- What is the minimum set of testing capabilities `mesher` must have to stop papering over framework gaps?
- Which current limitations are acceptable tradeoffs, and which are reputation-damaging?
- What should the final proof surface be: named `mesher` tests, dedicated framework regressions, or both?
