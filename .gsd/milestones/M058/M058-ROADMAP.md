# M058: Frontend Framework Migration to TanStack Start

## Vision
Move the product dashboard from Next.js to TanStack Start at `mesher/client` while keeping the current URLs, visuals, interactions, mock-data behavior, and maintainer command contract effectively identical.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | TanStack Start shell cutover | high | — | ✅ | `../hyperpush-mono/mesher/client/` boots under TanStack Start and shows the existing dashboard shell with current mock-data behavior intact. |
| S02 | Route and interaction parity | medium | S01 | ✅ | The migrated app serves the same dashboard URLs and preserves sidebar, panels, filters, and section behavior against the existing mock data. |
| S03 | Package and command contract parity | medium | S02 | ✅ | Maintainers run `npm run dev`, `npm run build`, and `npm run start` from `mesher/client`, and direct product-repo references now point to `client` plus TanStack Start. |
| S04 | Behavioral equivalence proof | low | S02, S03 | ⬜ | A final before/after verification pass shows no meaningful visual or interaction drift on the dashboard surface now served from `mesher/client`. |
