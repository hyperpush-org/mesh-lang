# M051: Mesher as the Living Reference App — Context Draft

**Gathered:** 2026-04-02
**Status:** Draft — needs dedicated discussion before planning

## Seed From Current Discussion

- Remove `reference-backend/` from the repo.
- Keep `mesher/` as the real product and the deeper maintained reference app.
- Use the now-updated docs and modern Mesh features to refit `mesher/` onto the current language/runtime surface honestly and efficiently.
- Ensure `mesher` still works in its current state while being prepared for further development.

## Technical Findings From Investigation

- `reference-backend/` still exists as a full top-level package and still anchors multiple public docs pages.
- `mesher/` is a much broader app with API, ingestion, services, storage, migrations, and a frontend, so it is a real reference-app candidate rather than a toy proof surface.
- Current docs, tests, and historical proof rails still mention `reference-backend/`, `tiny-cluster/`, and `cluster-proof/`, so retiring `reference-backend/` will have both product-path and docs/verifier fallout.
- `mesher` already pressure-tests Postgres, ORM expressions, JSON, realtime paths, and a frontend, which makes it a better long-term living reference than the narrow backend proof app.

## Likely Dependencies

- Depends on M049/M050 shrinking the public example/docs sprawl first.
- Likely feeds M052 because the public site should point at Mesher as the deeper real app only after the docs story is clean.

## Scope Seed

### Likely In Scope
- retire `reference-backend/` cleanly
- move any still-needed proof or helper value into `mesher/`, tests, or support code
- audit `mesher/` for modern Mesh patterns and current syntax/runtime usage
- keep Mesher running and useful as a future development base

### Likely Out of Scope
- landing-page rewrite
- packages-site CI/deploy work
- load-balancing follow-through beyond what Mesher needs directly

## Questions For Dedicated Discussion

- Which `reference-backend/` behaviors still need a separate proof seam before deletion, if any?
- How much Mesher modernization belongs in one milestone versus a follow-on cleanup wave?
- Which public docs should point at Mesher, and which should stay scaffold/examples first?
- What is the cleanest acceptance bar for “Mesher is ready for further development”?
