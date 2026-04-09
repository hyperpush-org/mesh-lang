# M046: Language-Owned Tiny Cluster Proofs

**Gathered:** 2026-03-31
**Status:** Ready for planning

## Project Description

M046 finishes the clustered-example cleanup the user expected to already be done. The current repo is close, but not honest enough yet: even after M045, clustered examples still expose too much app-owned triggering and too much proof-app-shaped surface. The user wants `cluster-proof/` completely nuked and rebuilt from zero, a brand-new local `tiny-cluster/` added beside it, and both examples reduced to almost nothing.

The user’s line is explicit and should be preserved literally: **the language/runtime decides this by itself**. App authors should only denote what work gets replicated and what does not. They should not have to own continuity submission, clustering behavior, failover logic, load balancing, routing choice, replica-count decisions, or status truth in app code. The proof workload itself should be trivial — literally `1 + 1` — so any remaining complexity is obviously Mesh complexity rather than example complexity.

## Why This Milestone

M045 proved that Mesh can move more clustered behavior into runtime/codegen, but it still left a visible honesty gap. The scaffold and `cluster-proof/` still rely on app-side submission and package-shaped proof surfaces that make clustering look more manual than the user wants. If the examples still look system-shaped, the platform is still over-explaining work the runtime should already own.

This milestone matters now because the user does not want another layered cleanup pass or a cosmetic docs rewrite. They want the clustered story to cross the last boundary: declaration in source or manifest, then runtime/tooling ownership for startup-triggered work, placement, replication, failover, recovery, and status truth. If Mesh cannot do that today, M046 should improve Mesh until it can.

## User-Visible Outcome

### When this milestone is complete, the user can:

- start a tiny local clustered proof with no HTTP routes, have the runtime/tooling trigger trivial clustered work automatically, and inspect cluster/work/failover truth entirely through built-in Mesh surfaces
- run a rebuilt packaged `cluster-proof/` that follows the same tiny route-free contract, while also having `meshc init --clustered` and `tiny-cluster/` express the same language-owned clustered story

### Entry point / environment

- Entry point: `meshc init --clustered`, local `tiny-cluster/`, rebuilt packaged `cluster-proof/`, and built-in `meshc cluster ...` tooling
- Environment: local dev, CI, package build/test, and packaged same-image / Fly-oriented proof surfaces where still relevant
- Live dependencies involved: Mesh runtime continuity/authority state, declared clustered work execution, compiler/codegen/typechecker/declaration plumbing, CLI cluster inspection, package build surfaces, and docs/verifier rails

## Completion Class

- Contract complete means: clustered work can be declared through both manifest and source decorator forms, and app code no longer owns clustered-work trigger or proof/status routes
- Integration complete means: `meshc init --clustered`, `tiny-cluster/`, and rebuilt `cluster-proof/` all exercise the same runtime-owned clustered-work story and stay in lockstep
- Operational complete means: the route-free proof apps auto-run their tiny clustered work on startup, survive the same honest failover bar Mesh already claims, and remain inspectable through runtime/tooling-only surfaces

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- one route-free local clustered proof (`tiny-cluster/`) starts two nodes, auto-runs trivial `1 + 1` clustered work, and exposes placement/work/failover truth through built-in tooling instead of app routes
- rebuilt `cluster-proof/` proves the same language-owned clustered contract in packaged form without carrying forward legacy HTTP/operator/status glue
- `meshc init --clustered`, `tiny-cluster/`, and `cluster-proof/` all tell the same clustered story, and the public docs/verifiers do not quietly treat one of them as the “real” path and the others as toy surfaces

## Risks and Unknowns

- The current runtime/tooling already owns inspection (`meshc cluster status|continuity|diagnostics`) but not an obviously route-free clustered-work trigger surface — M046 may need a new runtime/tooling seam before the proof apps can lose `Continuity.submit_declared_work(...)` in app code.
- Mesh does not appear to already have a general source decorator/annotation system, so supporting a source-side clustered-work decorator may require parser/typechecker/codegen work rather than a small syntax alias.
- `cluster-proof/` is deeply wired into docs, verifiers, package tests, and packaged proof rails; deleting and rebuilding it honestly may ripple across more of the repo than the app source alone suggests.
- Making `tiny-cluster/`, `cluster-proof/`, and `meshc init --clustered` equally canonical raises a drift risk unless M046 also adds explicit alignment checks and docs/verifier coverage.

## Existing Codebase / Prior Art

- `compiler/mesh-pkg/src/scaffold.rs` — current `meshc init --clustered` source; already smaller than `cluster-proof`, but still calls `Continuity.submit_declared_work(...)`, hardcodes the declared target, and decides replica count in app code.
- `compiler/meshc/src/cluster.rs` — current built-in runtime-owned inspection surface (`status`, `continuity`, `diagnostics`) that the new route-free proofs should rely on instead of app-owned status contracts.
- `compiler/mesh-pkg/src/manifest.rs` — current manifest-only clustered declaration path; M046 needs this to coexist with a source decorator form.
- `cluster-proof/main.mpl`, `cluster-proof/work.mpl`, `cluster-proof/work_continuity.mpl`, `cluster-proof/config.mpl` — current proof package shape that still carries package-owned trigger/status/config logic the user wants removed.
- `website/docs/docs/getting-started/clustered-example/index.md` — current scaffold-first docs story, which still shows app-side `Continuity.submit_declared_work(...)` and route-based triggering.
- `compiler/meshc/tests/e2e_m045_s02.rs`, `compiler/meshc/tests/e2e_m045_s03.rs`, `compiler/meshc/tests/e2e_m045_s04.rs`, `compiler/meshc/tests/e2e_m045_s05.rs` — current clustered-example proof rails whose assumptions will need to change when the proof apps become route-free and startup-triggered.

> See `.gsd/DECISIONS.md` for all architectural and pattern decisions — it is an append-only register; read it during planning, append to it during execution.

## Relevant Requirements

- R085 — add both manifest and source decorator clustered-work declaration
- R086 — move clustered-work triggering/control semantics fully into runtime/tooling ownership
- R087 — remove app-owned HTTP and explicit continuity submission from the proof flow
- R088 — create route-free local `tiny-cluster/` with literal `1 + 1` work
- R089 — rebuild `cluster-proof/` from zero on the same tiny route-free contract
- R090 — keep scaffold, `tiny-cluster/`, and `cluster-proof/` equally canonical and behaviorally aligned
- R091 — make runtime/tooling inspection sufficient for route-free proofs
- R092 — remove HTTP-route dependence from the public clustered proof story
- R093 — keep the proof workload intentionally trivial so platform complexity is unmistakable
- R052 / R082 — preserve room for packaged/deeper operator rails after the local story becomes tiny and language-owned

## Scope

### In Scope

- adding a source-side decorator form for clustered work while preserving manifest support
- moving the clustered-work trigger boundary out of app code and into runtime/tooling so proof apps can be route-free
- creating `tiny-cluster/` as a new local proof package
- completely deleting and rebuilding `cluster-proof/` around the new tiny route-free contract
- keeping `meshc init --clustered`, `tiny-cluster/`, and rebuilt `cluster-proof/` in sync as equally canonical clustered examples
- rewriting docs, tests, and verifiers so they prove the new language-owned clustered story honestly

### Out of Scope / Non-Goals

- claiming stronger distributed guarantees, active-active behavior, or consensus semantics just to make the new examples look more magical
- preserving legacy `cluster-proof` HTTP/operator/status routes for compatibility once the route-free runtime/tooling proof exists
- turning this milestone into a general-purpose annotation-system design wave beyond what clustered-work decorators require
- treating one of the three clustered-example surfaces as the “real” one and the others as second-class toys

## Technical Constraints

- the only thing app authors should need to do is denote what work gets replicated and what does not
- the new proof apps should have no HTTP routes at all; proof and status inspection should come from runtime/tooling surfaces only
- the proof work should stay literally trivial — `1 + 1` or equivalent — so remaining complexity is clearly Mesh-owned
- `tiny-cluster/` and rebuilt `cluster-proof/` should both auto-run their clustered work on startup
- M046 should improve Mesh when the existing runtime/tooling surface is insufficient instead of keeping app-owned seams around to hide the gap
- docs should lead with the decorator form while still supporting manifest-based declaration for existing/alternate workflows

## Integration Points

- `compiler/mesh-parser`, `compiler/mesh-lexer`, `compiler/mesh-typeck`, `compiler/mesh-codegen` — to add and lower a source decorator form for clustered work if Mesh does not already have one
- `compiler/mesh-pkg/src/manifest.rs` — to keep manifest and source declaration surfaces coherent and validated against the same clustered boundary
- `compiler/mesh-rt/src/dist/` and related codegen/runtime intrinsics — to move clustered-work triggering/control farther into runtime ownership
- `compiler/meshc/src/cluster.rs` — to remain or become the sole inspection surface for route-free proof apps
- `compiler/mesh-pkg/src/scaffold.rs` — to keep generated clustered apps aligned with the new route-free startup-triggered contract
- `cluster-proof/` and new `tiny-cluster/` — the two concrete proof packages M046 must leave behind
- docs and verifier scripts under `website/docs/`, `scripts/`, and `compiler/meshc/tests/` — to keep the three example surfaces equally canonical and mechanically aligned

## Open Questions

- What exact source syntax should the clustered-work decorator use? — current thinking: keep it narrow and purpose-built for clustered work rather than designing a broad annotation system in the same slice.
- What runtime/tooling surface should own startup-triggered clustered work for route-free proof apps? — current thinking: the proof apps should auto-run on startup, but the runtime/tooling seam still needs to become explicit enough that this does not hide app-owned submission logic under a different name.
- How much of the existing packaged Docker/Fly/verifier surface can survive the `cluster-proof/` rebuild without preserving the old package shape? — current thinking: keep the packaged proof value, but rebuild the app itself from zero and push proof complexity into runtime/tooling and verifiers instead of package code.
- How should equal-canonical-example drift be enforced? — current thinking: one of the later slices should add explicit alignment rails so scaffold, `tiny-cluster/`, and `cluster-proof/` cannot quietly diverge.