---
estimated_steps: 4
estimated_files: 5
skills_used:
  - github-workflows
  - rust-best-practices
  - test
---

# T03: Harden registry truth edges and align the named-install/scoped-key contract

**Slice:** S01 — Real registry publish/install proof
**Milestone:** M034

## Description

The live verifier should not be able to report green while registry state is half-committed or the public docs lie about dependency declaration. Tighten publish/download ordering so blob truth exists before metadata/counters advance, correct `meshpkg install <name>` messaging to match its real fetch+lockfile behavior, and update the public tooling docs so scoped dependency examples use quoted TOML keys and the operator contract matches what the CLI actually does.

## Failure Modes

| Dependency | On error | On timeout | On malformed response |
|------------|----------|-----------|----------------------|
| Registry DB insert / R2 upload | Abort publish without leaving a metadata-only success path; surface 409 vs internal errors distinctly. | Fail the publish phase and record whether blob upload or DB persistence stalled. | Return an explicit failure instead of claiming success or leaving counters/metadata advanced. |
| Registry object fetch / counter update | Return 404/explicit failure without incrementing counters when the blob is unavailable. | Fail the download phase without mutating counters. | Treat missing blob/metadata state as a release-blocking proof failure. |
| CLI/docs consumers | Fail contract checks when output/docs still imply manifest mutation or omit quoted scoped dependency examples. | N/A | Treat stale contract text as drift to fix, not as a non-blocking note. |

## Load Profile

- **Shared resources**: versions table uniqueness, R2 object existence/lookups, registry download counters, and the live proof package namespace.
- **Per-operation cost**: one blob existence/upload decision plus one DB insert per publish, and one object fetch plus counter update per successful download.
- **10x breakpoint**: duplicate publish races and repeated object lookups would surface first, so the hardened path should stay idempotent and keep the verifier serialized to one version per run.

## Negative Tests

- **Malformed inputs**: duplicate name/version publishes, missing object state, and scoped dependency snippets written without quoted keys.
- **Error paths**: duplicate publish must stay a 409, missing blob fetch must fail without counter inflation, and named install must leave `mesh.toml` unchanged while still updating `mesh.lock`.
- **Boundary conditions**: existing blob reuse by SHA, exact latest-version metadata, and docs/CLI wording that distinguishes declared dependencies from one-off fetches.

## Steps

1. Reorder `registry/src/routes/publish.rs` so object truth is established before the version row becomes the source of public metadata truth, while keeping README extraction and immutable-version handling intact.
2. Reorder `registry/src/routes/download.rs` so counters move only after object fetch succeeds, and make the failure path visible instead of silently inflating download counts.
3. Fix `compiler/meshpkg/src/install.rs` messaging/comments so named install is explicitly a fetch+lockfile operation, not a hidden `mesh.toml` mutation.
4. Update `website/docs/docs/tooling/index.md` and extend `scripts/verify-m034-s01.sh` so the public/operator contract now covers quoted scoped dependency keys, named-install manifest stability, duplicate publish rejection, and the hardened failure expectations.

## Must-Haves

- [ ] Publish no longer leaves a version row as the public truth source before the blob exists.
- [ ] Download counters advance only after object fetch succeeds.
- [ ] CLI/docs explicitly state that named install fetches and locks a package but does not edit `mesh.toml`.
- [ ] The verifier reruns the duplicate-publish and named-install contract checks against the hardened flow.

## Verification

- `bash scripts/verify-m034-s01.sh`
- `rg -n '"your-login/your-package" = "1.0.0"' website/docs/docs/tooling/index.md`
- `rg -n 'does not edit mesh.toml|updates mesh.lock' website/docs/docs/tooling/index.md compiler/meshpkg/src/install.rs`

## Observability Impact

- Signals added/changed: publish/download failures remain visible as 409/404 or explicit verifier phase failures instead of false-green metadata/counter state.
- How a future agent inspects this: rerun `bash scripts/verify-m034-s01.sh` and inspect the publish/download phase logs under `.tmp/m034-s01/verify/`.
- Failure state exposed: whether the break came from blob upload ordering, download object lookup, or stale named-install/scoped-key contract text.

## Inputs

- `registry/src/routes/publish.rs` — current metadata-first publish ordering.
- `registry/src/routes/download.rs` — current counter-before-object-fetch download ordering.
- `compiler/meshpkg/src/install.rs` — current named-install messaging that overstates manifest mutation.
- `website/docs/docs/tooling/index.md` — current public package-manager contract text.
- `scripts/verify-m034-s01.sh` — canonical live proof surface from T02 that must absorb the hardened checks.

## Expected Output

- `registry/src/routes/publish.rs` — blob-before-metadata publish truth ordering.
- `registry/src/routes/download.rs` — object-before-counter download truth ordering.
- `compiler/meshpkg/src/install.rs` — honest named-install messaging/comments.
- `website/docs/docs/tooling/index.md` — quoted scoped dependency examples plus named-install contract text.
- `scripts/verify-m034-s01.sh` — extended duplicate-publish and named-install contract checks.
