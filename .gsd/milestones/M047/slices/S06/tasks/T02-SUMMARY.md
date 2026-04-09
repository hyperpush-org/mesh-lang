---
id: T02
parent: S06
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m047_todo_scaffold.rs", "compiler/meshc/tests/e2e_m047_s05.rs", "compiler/mesh-pkg/src/scaffold.rs", ".gsd/KNOWLEDGE.md", ".gsd/DECISIONS.md"]
key_decisions: ["Keep the generated Todo runtime image as a prebuilt-`output` packager and emit `./output` on Linux first when proving Docker on non-Linux hosts.", "Retain container stdout/stderr, docker inspect JSON, and raw HTTP snapshots in the same `.tmp/m047-s05` bundle and fail closed on malformed or wrong-status container responses."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new fail-closed helper behavior with `cargo test -p meshc --test e2e_m047_s05 m047_s05_http_snapshot_helpers_fail_closed_on_bad_json_and_status -- --nocapture`, confirmed the scaffold README/Docker contract change with `cargo test -p mesh-pkg m047_s05 -- --nocapture`, reran the wrapper-dependent tooling filter with `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, and then ran the full `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` rail. That full rail proved native CRUD/rate-limit/restart truth plus containerized `/health` and real CRUD success while retaining `docker-output.file.txt`, container inspect JSON, container stdout/stderr, and negative-path timeout evidence. I then ran `bash scripts/verify-m047-s05.sh`; it finished with `.tmp/m047-s05/verify/status.txt = ok`, `.tmp/m047-s05/verify/current-phase.txt = complete`, and a phase report showing `m047-s05-pkg`, `m047-s05-tooling`, `m047-s05-e2e`, `m047-s05-docs-build`, and bundle-shape retention all passed."
completed_at: 2026-04-01T21:19:55.780Z
blocker_discovered: false
---

# T02: Extended the Todo scaffold proof to build a Linux `output`, run the generated image end to end, and retain container failure artifacts.

> Extended the Todo scaffold proof to build a Linux `output`, run the generated image end to end, and retain container failure artifacts.

## What Happened
---
id: T02
parent: S06
milestone: M047
key_files:
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - compiler/mesh-pkg/src/scaffold.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Keep the generated Todo runtime image as a prebuilt-`output` packager and emit `./output` on Linux first when proving Docker on non-Linux hosts.
  - Retain container stdout/stderr, docker inspect JSON, and raw HTTP snapshots in the same `.tmp/m047-s05` bundle and fail closed on malformed or wrong-status container responses.
duration: ""
verification_result: passed
completed_at: 2026-04-01T21:19:55.782Z
blocker_discovered: false
---

# T02: Extended the Todo scaffold proof to build a Linux `output`, run the generated image end to end, and retain container failure artifacts.

**Extended the Todo scaffold proof to build a Linux `output`, run the generated image end to end, and retain container failure artifacts.**

## What Happened

I extended the shared Todo scaffold harness with the missing container-proof surfaces: artifact-aware HTTP snapshot retention, fail-closed JSON/status parsing, published-port waiting with retained `docker inspect` JSON, attached container stdout/stderr capture, and container stop/remove helpers. I verified the local runtime constraint instead of assuming the old Docker story was already truthful: the retained native Todo binary on this host was a macOS Mach-O executable, so a generated image that simply copied host-built `./output` could still pass `docker build` while never being runnable inside a Linux container. To preserve the documented prebuilt-`output` model, I taught the harness to emit `./output` on Linux first when the host is non-Linux by reusing the repo’s `cluster-proof/Dockerfile` builder stage, then package that artifact with the generated Todo Dockerfile. I rewrote `e2e_m047_s05` so the generated project lives under the retained artifact bundle, still proves native CRUD/rate-limit/restart truth, then also builds the image, runs the generated Todo app in Docker, waits for the published host port, asserts `/health` plus real CRUD responses, and archives the mounted SQLite file. I also added the named negative checks from the task contract: malformed JSON and wrong-status snapshot helpers fail closed with retained `.http` / `.body.txt` evidence, a broken `TODO_DB_PATH` container fails before it can claim readiness while keeping inspect/log artifacts, and an unpublished-port container still boots but fails with a retained timeout artifact instead of being treated as success. Finally, I updated the generated Todo README to tell macOS/Windows operators to emit `./output` from Linux first, recorded the gotcha in `.gsd/KNOWLEDGE.md`, and saved the related decision to `.gsd/DECISIONS.md`.

## Verification

Verified the new fail-closed helper behavior with `cargo test -p meshc --test e2e_m047_s05 m047_s05_http_snapshot_helpers_fail_closed_on_bad_json_and_status -- --nocapture`, confirmed the scaffold README/Docker contract change with `cargo test -p mesh-pkg m047_s05 -- --nocapture`, reran the wrapper-dependent tooling filter with `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture`, and then ran the full `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` rail. That full rail proved native CRUD/rate-limit/restart truth plus containerized `/health` and real CRUD success while retaining `docker-output.file.txt`, container inspect JSON, container stdout/stderr, and negative-path timeout evidence. I then ran `bash scripts/verify-m047-s05.sh`; it finished with `.tmp/m047-s05/verify/status.txt = ok`, `.tmp/m047-s05/verify/current-phase.txt = complete`, and a phase report showing `m047-s05-pkg`, `m047-s05-tooling`, `m047-s05-e2e`, `m047-s05-docs-build`, and bundle-shape retention all passed.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_http_snapshot_helpers_fail_closed_on_bad_json_and_status -- --nocapture` | 0 | ✅ pass | 19400ms |
| 2 | `cargo test -p mesh-pkg m047_s05 -- --nocapture` | 0 | ✅ pass | 7400ms |
| 3 | `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture` | 0 | ✅ pass | 11100ms |
| 4 | `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` | 0 | ✅ pass | 560500ms |
| 5 | `bash scripts/verify-m047-s05.sh` | 0 | ✅ pass | 240600ms |


## Deviations

To keep the Docker runtime proof truthful on this non-Linux host, the harness now performs an internal Linux prebuild of `./output` before invoking the generated Todo Dockerfile. That preserves the required prebuilt-`output` runtime image contract instead of replacing it with an image that compiles Mesh inside Docker. I also moved the generated project workspace under the retained `.tmp/m047-s05/...` bundle instead of a disposable tempfile so the emitted output binary, mounted SQLite data, and generated sources survive in one proof surface.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`


## Deviations
To keep the Docker runtime proof truthful on this non-Linux host, the harness now performs an internal Linux prebuild of `./output` before invoking the generated Todo Dockerfile. That preserves the required prebuilt-`output` runtime image contract instead of replacing it with an image that compiles Mesh inside Docker. I also moved the generated project workspace under the retained `.tmp/m047-s05/...` bundle instead of a disposable tempfile so the emitted output binary, mounted SQLite data, and generated sources survive in one proof surface.

## Known Issues
None.
