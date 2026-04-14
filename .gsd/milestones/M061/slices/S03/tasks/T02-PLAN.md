---
estimated_steps: 4
estimated_files: 7
skills_used:
  - test
---

# T02: Document the missing backend route families behind mock-only dashboard routes

**Slice:** S03 — Backend gap map
**Milestone:** M061

## Description

Finish the maintainer-facing backend gap map by covering the pages that are still wholly mock-backed at the route level: `performance`, `solana-programs`, `releases`, `bounties`, and `treasury`. Keep these rows route-scoped or major-subsection-scoped rather than exploding every visible button into its own gap row. The goal is to tell backend maintainers which route families are missing — performance summaries/transactions, Solana program or log inspection, release list/detail/actions, bounty review/payout flows, treasury balances/allocations/transactions — without turning the canonical inventory into an unmaintainable control dump.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `../hyperpush-mono/mesher/main.mpl` | If no route family exists, classify the row as `no-route-family` instead of speculating about hidden support. | N/A — local file reads are synchronous and cheap. | Do not invent backend families from route names alone; tie the absence to actual missing registrations. |
| Mock-only route components under `../hyperpush-mono/mesher/client/components/dashboard/` | Keep the gap at route or major-subsection granularity and name the visible promise truthfully. | N/A — local file reads are synchronous and cheap. | If a component only exposes mock collections or placeholder actions, record the visible promise as shell-only scope and avoid implying implemented writes. |

## Load Profile

- **Shared resources**: local reads across the canonical doc, the route registry, and five mock-only route components.
- **Per-operation cost**: one bounded audit of the five remaining mock-backed route families.
- **10x breakpoint**: the risk is maintainability, not runtime; if every visible button becomes its own row, the document will rot faster than the backend evolves.

## Negative Tests

- **Malformed inputs**: route-level UI chrome such as Rollback, Process Payout, Connect, or transaction drill-down must not be described as backed by hidden APIs when `main.mpl` registers nothing for that family.
- **Error paths**: if a mock route imports only `MOCK_*` data, the row must explicitly say `no-route-family` rather than hand-waving future support.
- **Boundary conditions**: keep wholly mock-backed pages at route or major-subsection level; do not create per-control rows for every CTA on Releases, Bounties, Treasury, Performance, or Solana Programs.

## Steps

1. Confirm in `main.mpl` that no dedicated `/api/v1` route families exist today for Performance, Solana Programs, Releases, Bounties, or Treasury.
2. Audit each mock-only route component to capture the maintainer-visible promise at the smallest actionable scope that remains maintainable.
3. Add backend-gap rows for those mock-only route families to `ROUTE-INVENTORY.md`, using `no-route-family` and explicit missing-support notes tied to the visible route promise.
4. Add or tighten gap-map notes/invariants so future slices preserve the rule that mock-only routes stay route-scoped unless a real mixed seam appears.

## Must-Haves

- [ ] The backend gap map covers `performance`, `solana-programs`, `releases`, `bounties`, and `treasury` without inventing backend routes that do not exist.
- [ ] Each mock-only row names the missing backend family the page would need before its visible promise can become truthful.
- [ ] The doc stays maintainable by grouping wholly mock routes at route or major-subsection granularity instead of per-button granularity.

## Inputs

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`
- `../hyperpush-mono/mesher/main.mpl`
- `../hyperpush-mono/mesher/client/components/dashboard/performance-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/solana-programs-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/releases-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/bounties-page.tsx`
- `../hyperpush-mono/mesher/client/components/dashboard/treasury-page.tsx`

## Expected Output

- `../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md`

## Verification

`python3 - <<'PY'
from pathlib import Path
text = Path('../hyperpush-mono/mesher/client/ROUTE-INVENTORY.md').read_text()
for needle in (
    '`performance`',
    '`solana-programs`',
    '`releases`',
    '`bounties`',
    '`treasury`',
    '`no-route-family`',
):
    assert needle in text, needle
PY`
