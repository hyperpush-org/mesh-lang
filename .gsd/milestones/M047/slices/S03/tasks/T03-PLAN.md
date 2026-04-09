---
estimated_steps: 1
estimated_files: 5
skills_used: []
---

# T03: Synthesize clustered route registrations and generated bare route shims

Lower accepted `HTTP.clustered(...)` calls onto the existing declared-handler runtime-name and `replication_count` model used by ordinary `@cluster` functions. Generate deterministic clustered route runtime names plus bare route shim symbols, thread them through declared-handler planning/registration, and keep S03 scoped away from generic route-closure ABI widening unless the compiler proves that impossible.

## Inputs

- `T02 metadata handoff`
- `D271`
- `D273`
- `D278`

## Expected Output

- `Clustered route wrappers lower into generated bare route shims plus synthetic declared-handler registrations.`
- `Prepared build/runtime registration truth includes deterministic route runtime names and preserved replication counts.`

## Verification

cargo test -p mesh-codegen m047_s03 -- --nocapture

## Observability Impact

- Signals added/changed: clustered route execution should surface runtime name, count, phase/result, rejection reason, and reply-timeout/failure state through continuity/diagnostic output without logging raw request data.
- How a future agent inspects this: `cargo test -p mesh-rt m047_s03 -- --nocapture`, `meshc cluster continuity --json`, `meshc cluster diagnostics --json`, and retained stderr/http artifacts from the e2e rail.
- Failure state exposed: unsupported counts, owner/reply timeouts, or route-dispatch failures become explicit continuity/diagnostic truth instead of silent local fallback or hung requests.
