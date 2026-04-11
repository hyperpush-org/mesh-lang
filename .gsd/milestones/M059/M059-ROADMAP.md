# M059: Frontend Framework Migration to TanStack Start

## Vision
Replace the product dashboard’s Next.js runtime with TanStack Start, land the app at `../hyperpush-mono/mesher/client/`, and keep the design and user-visible behavior effectively identical so the only meaningful change is the underlying framework.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | S01 | high | — | ✅ | After this: the existing dashboard codebase runs on TanStack Start migration plumbing in place, with the current shell still visible and the normal command contract preserved while the app remains at `frontend-exp`. |
| S02 | S02 | high | — | ✅ | After this: the dashboard uses real TanStack routes for the current sections while preserving the same visible shell, URLs, panels, filters, and mock-data interactions. |
| S03 | S03 | medium | — | ⬜ | After this: the migrated dashboard runs from `../hyperpush-mono/mesher/client/` with `dev`, `build`, and `start`, and Next.js is no longer on the critical runtime path. |
| S04 | Equivalence proof and direct operational cleanup | medium | S03 | ⬜ | After this: maintainers can run the migrated app, exercise the key dashboard flows, and rely on updated direct references without stale `frontend-exp` / Next.js operational guidance. |
