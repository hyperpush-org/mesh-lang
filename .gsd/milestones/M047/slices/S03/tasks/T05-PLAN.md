---
estimated_steps: 1
estimated_files: 3
skills_used: []
---

# T05: Prove clustered HTTP routes live and preserve the retained route-limit contract

Lock the slice with a real HTTP e2e rail after the compiler/runtime path exists. Build a temp clustered app that uses `HTTP.clustered(handle)` and `HTTP.clustered(3, handle)`, send live HTTP requests through the wrapped routes, query continuity/diagnostic output for runtime-name and count truth, and replay the existing S02 plus M032 control rails so the new wrapper does not regress ordinary `@cluster` semantics or accidentally broaden generic route-closure support.

## Inputs

- `T02-T04 implementation`
- `S02 replication-count truth rails`
- `M032 retained route-limit controls`

## Expected Output

- `A named e2e rail proves `HTTP.clustered(...)` routes execute through the clustered handler boundary and return real HTTP responses.`
- `Regression controls confirm ordinary `@cluster` behavior stays intact and generic route-closure limits remain unchanged.`

## Verification

cargo test -p meshc --test e2e_m047_s03 -- --nocapture && cargo test -p meshc --test e2e_m047_s02 -- --nocapture && cargo test -p meshc --test e2e_stdlib e2e_m032_route_bare_handler_control -- --nocapture && cargo test -p meshc --test e2e_stdlib e2e_m032_route_closure_runtime_failure -- --nocapture
