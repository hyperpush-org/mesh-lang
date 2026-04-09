---
title: Production Backend Proof
description: Compact public-secondary handoff for Mesh's starter/examples-first backend story, the Hyperpush product repo, and retained backend-only compatibility rails
prev: false
next: false
---

# Production Backend Proof

This is the compact public-secondary handoff for Mesh's backend proof story.

Public readers should still stay scaffold/examples first: start with [Clustered Example](/docs/getting-started/clustered-example/), [`examples/todo-sqlite/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md), or [`examples/todo-postgres/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md) before using this page as a deeper handoff.

This page is the repo-boundary handoff from mesh-lang into the maintained backend/app contract. The maintained app runbook and primary verifier live in the [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono); mesh-lang keeps only the public proof-page contract and retained compatibility wrappers on this side of the boundary.

## Canonical surfaces

- [Clustered Example](/docs/getting-started/clustered-example/) — public scaffold-first clustered walkthrough
- [`examples/todo-sqlite/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-sqlite/README.md) — honest local single-node starter
- [`examples/todo-postgres/README.md`](https://github.com/hyperpush-org/mesh-lang/blob/main/examples/todo-postgres/README.md) — shared/deployable PostgreSQL starter
- [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) — repo-boundary maintained app/backend handoff
- [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md) — deeper maintained app runbook in the Hyperpush repo
- `bash mesher/scripts/verify-maintainer-surface.sh` — product-owned verifier run from the Hyperpush repo after the handoff
- `bash scripts/verify-m051-s01.sh` — mesh-lang compatibility wrapper that confirms the public handoff still points at the product-owned verifier
- `bash scripts/verify-m051-s02.sh` — retained backend-only verifier replay
- `bash scripts/verify-production-proof-surface.sh` — compatibility verifier that keeps this public proof page truthful

## Named maintainer verifiers

These are the named commands behind the current deeper backend-maintainer story:

```bash
# after the repo-boundary handoff in the Hyperpush product repo
bash mesher/scripts/verify-maintainer-surface.sh

# retained mesh-lang compatibility rails
bash scripts/verify-m051-s01.sh
bash scripts/verify-m051-s02.sh
bash scripts/verify-production-proof-surface.sh
```

Use `bash mesher/scripts/verify-maintainer-surface.sh` when you are verifying the maintained deeper app contract that now lives with Hyperpush. Use `bash scripts/verify-m051-s01.sh` only when you need to confirm the mesh-lang compatibility wrapper still delegates to that product-owned verifier. Use `bash scripts/verify-m051-s02.sh` when you need the retained backend-only proof replay without turning its internal fixture layout into public teaching.

## Retained backend-only recovery signals

When `bash scripts/verify-m051-s02.sh` fails on the retained backend-only proof, inspect the recovery markers it preserves:

- `restart_count`
- `last_exit_reason`
- `recovered_jobs`
- `last_recovery_at`
- `last_recovery_job_id`
- `last_recovery_count`
- `recovery_active`

Those signals stay maintainer-facing on purpose: they prove the retained backend-only recovery story without turning the compatibility fixture tree into a public tutorial.

## When to use this page vs the generic guides

Use [Web](/docs/web/), [Databases](/docs/databases/), [Testing](/docs/testing/), [Concurrency](/docs/concurrency/), or [Developer Tools](/docs/tooling/) when you want a subsystem API in isolation.

Use this page when you need the handoff from the public starter/examples-first route into the maintained backend/app contract. From here, continue to the [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) and its [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md) maintainer runbook. Treat `bash scripts/verify-m051-s01.sh` as the mesh-lang compatibility wrapper for that product-owned contract, or use `bash scripts/verify-m051-s02.sh` for the retained backend-only proof.

## Failure inspection map

If a maintainer proof fails, rerun the smallest named surface that matches the drift:

- **Public proof-page contract:** `bash scripts/verify-production-proof-surface.sh`
- **Repo-boundary maintained-app handoff:** [Hyperpush product repo](https://github.com/hyperpush-org/hyperpush-mono) and [`mesher/README.md`](https://github.com/hyperpush-org/hyperpush-mono/blob/main/mesher/README.md)
- **Product-owned Mesher contract:** `bash mesher/scripts/verify-maintainer-surface.sh`
- **mesh-lang compatibility wrapper:** `bash scripts/verify-m051-s01.sh`
- **Retained backend-only proof:** `bash scripts/verify-m051-s02.sh`

If a public docs page starts teaching the old mesh-lang-local product path again, treat that as contract drift instead of a docs cleanup detail.
