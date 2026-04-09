---
estimated_steps: 3
estimated_files: 4
skills_used:
  - rust-best-practices
---

# T03: Expose startup-work identity on `meshc cluster continuity` surfaces

**Slice:** S02 — Runtime-owned startup trigger and route-free status contract
**Milestone:** M046

## Description

Expose enough continuity identity on `meshc cluster continuity` for a route-free proof to locate startup work by runtime name instead of relying on an app-owned status route or guessing an opaque internal key.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| `compiler/meshc/src/cluster.rs` JSON and human-readable rendering | Fail the CLI rail on missing identity fields instead of silently dropping startup-work metadata. | N/A | Reject malformed continuity JSON/rendering in tests instead of printing partial records. |
| `compiler/mesh-rt/src/dist/operator.rs` continuity query transport | Preserve existing `target_not_connected` and decode errors without adding the CLI as a visible cluster peer. | Return the existing operator timeout error. | Never strip the declared runtime name from list or single-record payloads. |

## Load Profile

- **Shared resources**: Operator query payload size and continuity list truncation.
- **Per-operation cost**: One additional string field per continuity record in list and single-record output.
- **10x breakpoint**: List truncation and payload size growth show up before transport or JSON serialization cost matters.

## Negative Tests

- **Malformed inputs**: Empty runtime names in continuity records and malformed query replies.
- **Error paths**: Disconnected-target and decode failures remain explicit on CLI output instead of encouraging app-owned fallbacks.
- **Boundary conditions**: List mode with multiple records, single-record mode by request key, and records whose runtime name is present even when owner/replica routing changes.

## Steps

1. Add `declared_handler_runtime_name` to `meshc cluster continuity` JSON output and human-readable record/list rendering.
2. Keep list and single-record paths aligned so a route-free proof can discover startup work in list mode and then inspect the exact record.
3. Add CLI-focused proof rails that assert runtime-name visibility without regressing the transient operator query contract.

## Must-Haves

- [ ] Route-free startup work is discoverable from `meshc cluster continuity` alone.
- [ ] JSON and human-readable continuity output stay aligned on runtime-name visibility.
- [ ] Operator/CLI failures remain explicit instead of pushing the proof back toward app-owned routes.

## Verification

- `cargo test -p meshc --test e2e_m046_s02 m046_s02_cli_ -- --nocapture`
- `cargo test -p meshc --test e2e_m044_s03 m044_s03_operator_ -- --nocapture`

## Observability Impact

- Signals added/changed: Continuity JSON/list output now carries declared runtime name for startup work discovery.
- How a future agent inspects this: `meshc cluster continuity <node> --json` or the human-readable list output.
- Failure state exposed: Missing or mismatched runtime-name surfacing fails the CLI rail before any app-owned probe is attempted.

## Inputs

- `compiler/meshc/src/cluster.rs` — current continuity rendering hides runtime-name identity.
- `compiler/mesh-rt/src/dist/continuity.rs` — record shape already carries declared runtime name and must stay the source of truth.
- `compiler/meshc/tests/e2e_m044_s03.rs` — existing operator/CLI proof rail that must stay green.
- `compiler/meshc/tests/e2e_m046_s02.rs` — new route-free proof rail that will depend on runtime-name visibility.

## Expected Output

- `compiler/meshc/src/cluster.rs` — continuity JSON and human-readable output include declared runtime name.
- `compiler/meshc/tests/e2e_m046_s02.rs` — CLI-focused assertions prove startup work is discoverable from tooling alone.
