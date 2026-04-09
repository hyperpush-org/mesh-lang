---
estimated_steps: 1
estimated_files: 6
skills_used: []
---

# T04: Execute clustered route handlers with synchronous HTTP reply delivery

Make the generated clustered route shims actually execute `Request -> Response` handlers and return their `Response` back to the ingress HTTP server on the same request. Reuse or extend the existing service-style reply transport instead of continuity polling, preserve route runtime-name/count/rejection truth in continuity diagnostics, and keep route failures diagnosable without leaking request payloads or cluster cookies.

## Inputs

- `T03 generated route shims and declared-handler registrations`
- `D277`
- `D278`

## Expected Output

- `Live clustered route execution returns normal `Response` values synchronously through the ingress HTTP request.`
- `Continuity/runtime diagnostics show clustered route runtime names, replication counts, and rejection/failure truth for wrapped routes.`

## Verification

cargo test -p mesh-rt m047_s03 -- --nocapture

## Observability Impact

- Signals added/changed: retained e2e artifacts should capture live HTTP replies, continuity JSON/human output, diagnostics, and runtime stderr for wrapped-route success and rejection paths.
- How a future agent inspects this: `cargo test -p meshc --test e2e_m047_s03 -- --nocapture` plus the retained `.tmp/m047-s03/...` bundle from the new rail.
- Failure state exposed: HTTP reply drift, missing runtime-name/count fields, or accidental route-closure support broadening is archived as concrete request/CLI/log evidence.
