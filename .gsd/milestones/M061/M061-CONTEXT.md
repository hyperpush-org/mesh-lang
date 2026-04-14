# M061: Mesher Client Mock Truth & Backend Gap Map

**Gathered:** 2026-04-11
**Status:** Ready for planning

## Project Description

This is a documentation milestone for the canonical dashboard package in `../hyperpush-mono/mesher/client/`. Its job is to say exactly what is still mocked after M060, what is already live, what is mixed live/mock, and where the current client side promises more than the backend currently supports.

The output is not a public product page and not a stealth backend implementation wave. It is a maintainer-facing source of truth that backend work can use later to expand Mesher until it fully supports what the current client shell implies.

## Why This Milestone

M060 proved the seeded backend-backed shell that already exists, but the current client still spans three realities at once: fully live surfaces, mixed live/mock surfaces, and fully mock-only screens. Right now that truth is spread across route files, adapters, test assertions, and local shell copy.

That is enough to operate the app, but not enough to plan the next backend wave cleanly. Before adding new backend support, the repo needs one place that says exactly what in `@mesher/client` is still mocked and what backend seam would be needed to make the shell fully truthful.

## User-Visible Outcome

### When this milestone is complete, the user can:

- open one maintainer-facing document next to `mesher/client` and answer, route by route and subsection by subsection, what is live, mixed, or mock-only now
- use that same document to plan backend expansion from the current client side promises instead of re-auditing the shell from scratch

### Entry point / environment

- Entry point: maintainer docs beside `../hyperpush-mono/mesher/client/` plus repo-owned proof/verification surfaces
- Environment: local repo workspace with the sibling product repo available at `../hyperpush-mono/`
- Live dependencies involved: existing `mesher/client` route files, adapters, Playwright proof rails, and the Mesher `/api/v1` seams they already exercise

## Completion Class

- Contract complete means: the mock/live classification and backend gap map are documented in a canonical maintainer surface with evidence pointers and no hand-wavy categories
- Integration complete means: the documentation, route map, adapters, and current Playwright proof rails agree on what is live, mixed, and mock-only
- Operational complete means: a repeatable verifier or drift-check can be rerun later to catch inventory drift when the shell changes

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- every top-level dashboard route in `mesher/client` is classified truthfully as live, mixed, or mock-only with evidence
- mixed routes like Issues, Alerts, and especially Settings are broken down far enough that maintainers can see which specific panels and controls are real versus shell-only
- the final backend gap map is actionable enough that a later backend milestone can sequence work from it without re-auditing the client first

## Architectural Decisions

### Canonical inventory lives with the client maintainers

**Decision:** Put the canonical mock/live inventory beside `mesher/client` in the product-maintainer surface instead of leaving it only in `.gsd/` or moving it into public docs.

**Rationale:** The point is to help frontend and backend maintainers work from one current-state map. A milestone-only artifact is too hidden for ongoing use, and a public docs page would frame the work as outward-facing product documentation instead of a backend planning aid.

**Alternatives Considered:**
- Keep the truth only in milestone artifacts — rejected because the next backend wave would still have to rediscover it
- Publish the inventory as public product docs — rejected because this is a maintainer/backend planning surface, not an evaluator-facing story

### Fine-grained audit beats route-level labels

**Decision:** Classify at route, panel, subsection, and control level wherever the page mixes real backend behavior with shell-only behavior.

**Rationale:** Route-level labels would hide the real work. `settings-page.tsx`, `dashboard-issues-state.tsx`, `alerts-live-state.tsx`, and the current Playwright rails already show that some pages are truthful only at subsection level.

**Alternatives Considered:**
- Route-level inventory only — rejected because mixed pages would still overcompress the truth
- Control-level audit for every page regardless of risk — rejected because top-level mock-only routes do not need unnecessary decomposition

### Backend gap map should follow current client promises, not aspirational redesign

**Decision:** Build the gap map from what the current shell actually presents and implies today, then map that to current backend seams and missing support.

**Rationale:** The user asked for something that can be used to expand the backend to fully support what the client side promises. That means starting from the shell as-shipped after M060, not from a cleaner hypothetical redesign.

**Alternatives Considered:**
- Redesign the shell first and document that cleaner model — rejected because it changes the evidence instead of documenting it
- Document only existing backend routes without reference to client promise — rejected because that would not answer what still feels mocked from the client side

### Evidence beats prose-only classification

**Decision:** Every live/mixed/mock-only classification should point at code and proof surfaces, and the milestone should end with a repeatable drift check.

**Rationale:** The inventory will become stale unless it is tied to existing route files, adapters, and proof rails. The doc needs to be maintainable, not just accurate on the day it is written.

**Alternatives Considered:**
- Narrative documentation only — rejected because it will drift too easily
- Full automatic extraction as part of this milestone — rejected because the immediate need is truthful documentation and a useful backend gap map, not a new inventory framework

## Error Handling Strategy

This milestone should fail closed on uncertainty. If a surface cannot be proven live, it should not be described as live. If a page is mixed, the document should say mixed and identify the exact boundaries instead of compressing it into a cleaner label.

Where the code, tests, and shell copy disagree, the milestone should surface the disagreement explicitly as a gap or ambiguity rather than choosing the nicest interpretation. The final verifier should check for drift in the documented inventory markers, route coverage, or evidence anchors so future maintainers can re-run it after client changes.

## Risks and Unknowns

- Mixed surfaces may overpromise through shell copy or still-visible controls — that matters because backend follow-up work needs the promise boundary, not just the fetch boundary
- The current proof rails are strongest for Issues, Alerts, and Settings — mock-only routes may need more code-reading than test-reading to classify confidently
- Some backend gaps may be implicit rather than explicit missing endpoints — that matters because later milestones need to know whether they are adding routes, enriching payloads, or just wiring dormant seams
- The maintainer document can drift quickly if it is not anchored to stable route names, `data-*` markers, and test surfaces — that matters because a stale gap map misleads the next backend milestone

## Existing Codebase / Prior Art

- `../hyperpush-mono/mesher/client/README.md` — current maintainer summary of the mixed live/mock shell after M060
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` — canonical top-level route inventory used by the assembled walkthrough
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx` — mixed page with explicit live and mock-only subsection markers
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-live-state.tsx` — subsection-scoped same-origin reads/writes and failure visibility for the live Settings surfaces
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` — provider-owned Issues live read/mutation seam
- `../hyperpush-mono/mesher/client/components/dashboard/alerts-live-state.tsx` — provider-owned Alerts live read/mutation seam
- `../hyperpush-mono/mesher/client/lib/mock-data.ts` — the shell contract still supplying fully mock or fallback-backed surfaces
- `../hyperpush-mono/mesher/client/lib/solana-mock-data.ts` — explicit mock-only Solana route data
- `../hyperpush-mono/mesher/client/lib/mesher-api.ts` — current same-origin Mesher API client boundary
- `../hyperpush-mono/mesher/client/tests/e2e/seeded-walkthrough.spec.ts` — canonical assembled route-to-route shell proof
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-read.spec.ts` — Issues live read and fallback truth surface
- `../hyperpush-mono/mesher/client/tests/e2e/issues-live-actions.spec.ts` — Issues action and shell-only action boundary proof
- `../hyperpush-mono/mesher/client/tests/e2e/admin-ops-live.spec.ts` — Alerts and Settings live/admin proof, including explicit mock-only subsections

## Relevant Requirements

- R167 — create the canonical maintainer-facing mock/live inventory
- R168 — make the audit fine-grained enough for mixed routes and controls
- R169 — produce the backend gap map from client promise to missing support
- R170 — back the classifications with repo evidence and a repeatable proof rail
- R171 — leave a handoff that later backend planning can consume directly
- R173 — do not implement the missing backend surfaces in this milestone
- R174 — do not redesign `mesher/client` to simplify the docs
- R175 — do not turn this into a public-facing docs wave

## Scope

### In Scope

- route-by-route inventory of the current `mesher/client` dashboard shell
- fine-grained breakdown of mixed routes, especially Issues, Alerts, and Settings
- evidence mapping from shell surface to current backend seam or explicit absence of backend support
- a backend gap map that says what would have to change for the current client side promises to become fully true
- a canonical maintainer document beside `mesher/client`
- a repeatable verifier or drift-proof rail for the new inventory

### Out of Scope / Non-Goals

- implementing missing backend routes, payloads, or actions
- redesigning the shell or removing mock-only UI to make the inventory look cleaner
- rewriting public Mesh docs around this internal client/backend audit
- turning this into a broad product or backend cleanup milestone

## Technical Constraints

- Preserve the current shell as evidence; document it rather than simplifying it away
- Use the canonical product repo path `../hyperpush-mono/mesher/client/` for source truth
- Reuse existing route maps, adapters, test IDs, and Playwright proof rails where possible instead of inventing a second classification vocabulary
- Keep the final artifact maintainable by tying it to stable code/test anchors
- When a route is wholly mock-only, do not over-engineer the audit — reserve the fine-grained breakdown for the mixed zones where maintainers could otherwise be misled

## Integration Points

- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-route-map.ts` — authoritative top-level route inventory
- `../hyperpush-mono/mesher/client/components/dashboard/settings/settings-page.tsx` — subsection and control-level mixed-shell truth
- `../hyperpush-mono/mesher/client/components/dashboard/dashboard-issues-state.tsx` — provider-owned issue live seam
- `../hyperpush-mono/mesher/client/components/dashboard/alerts-live-state.tsx` — provider-owned alert live seam
- `../hyperpush-mono/mesher/client/lib/mock-data.ts` and `lib/solana-mock-data.ts` — explicit mock/fallback data sources
- `../hyperpush-mono/mesher/client/lib/mesher-api.ts` — existing same-origin backend client surface
- `../hyperpush-mono/mesher/client/tests/e2e/*.spec.ts` — route-level and assembled proof that define what is already live today
- `../hyperpush-mono/mesher/api/*.mpl` — backend seams the gap map will point at or mark as missing

## Testing Requirements

Contract verification should include code-backed classification of every top-level route, fine-grained mixed-route evidence for Issues/Alerts/Settings, and a repeatable proof/drift rail that checks the canonical document against route inventory and existing evidence anchors. Integration verification should reuse the current Playwright proof surfaces as evidence for what is already truly live and should avoid inventing a second runtime story. There is no new operational runtime to prove, but the final verifier must be rerunnable by maintainers after future shell changes.

## Acceptance Criteria

- S01 produces an evidence-backed top-level route inventory for `mesher/client`
- S02 decomposes mixed routes into truthful subsection/control-level live versus shell-only maps
- S03 produces a backend gap map tied to current shell promises and current backend seams
- S04 lands the canonical maintainer document beside `mesher/client` and a repeatable drift-proof rail that keeps it honest

## Open Questions

- How much of the current shell promise should be judged by visible copy versus by actionable controls and live data seams — current thinking: treat both as evidence, but mark visible overclaim separately from implemented action gaps
- Whether the final gap map should include a likely backend implementation order or only the missing seams — current thinking: include a pragmatic implementation order because the user wants the document to be used for backend expansion planning
