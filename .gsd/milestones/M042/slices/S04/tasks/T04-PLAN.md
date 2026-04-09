---
estimated_steps: 4
estimated_files: 6
skills_used: []
---

# T04: Rewrite the runbook and public docs to the verified runtime-owned continuity truth.

1. Update `cluster-proof/README.md` to document the thin-consumer shape, the legacy `GET /work` probe versus keyed `POST /work` / `GET /work/:request_key`, the small env contract, and the local-authority versus read-only-Fly proof split.
2. Update `website/docs/docs/distributed-proof/index.md`, `website/docs/docs/distributed/index.md`, and repo-level entry points so the public story matches the verified M042 rail, including the runtime `Continuity` API semantics and the authoritative command set from T03.
3. Add or update the proof-surface verifier and VitePress wiring so the runbook, proof page, distributed guide, and README mechanically agree on links, commands, and wording, with no exactly-once or process-state-migration claims.
4. Build the docs serially after the proof-surface verifier passes.

## Inputs

- `cluster-proof/README.md`
- `website/docs/docs/distributed-proof/index.md`
- `website/docs/docs/distributed/index.md`
- `README.md`
- `scripts/verify-m042-s04.sh`
- `scripts/verify-m042-s04-fly.sh`

## Expected Output

- `Docs and runbooks that describe request_key as the idempotency key and attempt_id as the retry fence/token`
- `Public wording that stays at-least-once/idempotent, distinguishes the legacy probe from keyed continuity, and scopes Fly verification as read-only sanity only`
- `A passing proof-surface verifier and serial VitePress build`

## Verification

bash scripts/verify-m042-s04-proof-surface.sh && bash scripts/verify-m042-s04-fly.sh --help && npm --prefix website run build
