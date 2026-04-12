# M061: Mesher Client Mock Truth & Backend Gap Map

## Vision
Document exactly what is still mocked in `@mesher/client` after the server wiring milestone so backend expansion can fully support what the current client side promises.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | S01 | high | — | ✅ | Maintainers can see every top-level `mesher/client` route classified as live, mixed, or mock-only with code/test evidence. |
| S02 | S02 | medium | — | ⬜ | Maintainers can answer exactly which Issues, Alerts, and Settings panels and controls are real versus shell-only. |
| S03 | Backend gap map | medium | S01, S02 | ⬜ | Backend maintainers can trace each client-side promise to an existing backend seam or a documented missing seam. |
| S04 | Canonical maintainer handoff | low | S01, S02, S03 | ⬜ | The canonical inventory and backend gap map live beside `mesher/client` with a rerunnable drift-proof rail. |
