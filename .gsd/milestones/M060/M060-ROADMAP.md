# M060: Mesher Client Live Backend Wiring

## Vision
Connect `../hyperpush-mono/mesher/client` to the existing Mesher backend while changing as little UI as possible, fixing just enough backend breakage to work, and keeping still-mocked UI in place.

## Slice Overview
| ID | Slice | Risk | Depends | Done | After this |
|----|-------|------|---------|------|------------|
| S01 | S01 | high | — | ✅ | Against a seeded local Mesher backend, the dashboard boots in a truthful real project-org/API-key context and the Issues route loads live issue, detail, and event data through the existing shell. |
| S02 | S02 | high | — | ✅ | The dashboard summaries and issue actions are live: operators can inspect real issues, perform existing issue actions, and see backend-backed summary data instead of broad mock stats. |
| S03 | S03 | medium | — | ✅ | Alerts, settings/storage, team, and API-key areas use real backend reads and writes wherever the backend already has a route, while the broader shell stays visually intact. |
| S04 | S04 | medium | — | ✅ | In one seeded local environment, the full backend-backed shell walkthrough succeeds across every currently existing Mesher dashboard route, with only minimal backend seam repairs and no redesign drift. |
