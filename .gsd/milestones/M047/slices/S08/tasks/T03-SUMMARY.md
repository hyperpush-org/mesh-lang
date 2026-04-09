---
id: T03
parent: S08
milestone: M047
provides: []
requires: []
affects: []
key_files: ["compiler/meshc/tests/support/m047_todo_scaffold.rs", "compiler/meshc/tests/e2e_m047_s05.rs", ".gsd/KNOWLEDGE.md", ".gsd/DECISIONS.md"]
key_decisions: ["Query Docker clustered-route operator surfaces from a helper container sharing `--network container:<target>` instead of the host-published cluster port because `MESH_NODE_NAME=@127.0.0.1:port` binds the listener to container loopback.", "Keep published cluster-port assertions and retained Docker inspect/port artifacts as publication evidence, but treat the same-netns helper query as the authoritative status/continuity seam."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Passed the previously failing `m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container` rail after moving Docker operator queries onto the helper-container seam, then passed the full `e2e_m047_s05`, `e2e_m047_s07`, and `tooling_e2e` rails plus `bash scripts/verify-m047-s05.sh`. The assembled verifier replay also re-ran the S04 prerequisite, `mesh-pkg` S05 contract rail, the Todo tooling rail, the full S05 e2e rail, and the website docs build successfully. After formatting, `cargo fmt --all --check` passed and the targeted clustered-route proof stayed green."
completed_at: 2026-04-02T03:05:59.944Z
blocker_discovered: false
---

# T03: Switched the Todo Docker clustered-route proof to a same-netns helper-container operator seam and restored green S05 verification.

> Switched the Todo Docker clustered-route proof to a same-netns helper-container operator seam and restored green S05 verification.

## What Happened
---
id: T03
parent: S08
milestone: M047
key_files:
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - .gsd/KNOWLEDGE.md
  - .gsd/DECISIONS.md
key_decisions:
  - Query Docker clustered-route operator surfaces from a helper container sharing `--network container:<target>` instead of the host-published cluster port because `MESH_NODE_NAME=@127.0.0.1:port` binds the listener to container loopback.
  - Keep published cluster-port assertions and retained Docker inspect/port artifacts as publication evidence, but treat the same-netns helper query as the authoritative status/continuity seam.
duration: ""
verification_result: passed
completed_at: 2026-04-02T03:05:59.946Z
blocker_discovered: false
---

# T03: Switched the Todo Docker clustered-route proof to a same-netns helper-container operator seam and restored green S05 verification.

**Switched the Todo Docker clustered-route proof to a same-netns helper-container operator seam and restored green S05 verification.**

## What Happened

Used the retained T02 Docker artifacts plus the runtime operator transport code to confirm that the host-side `meshc cluster status` EOF was a Docker loopback mismatch, not a Todo runtime failure: the container advertised `name@127.0.0.1:port`, the Mesh node listener bound to container loopback, and host-published port handshakes closed during the transient operator send_name step. I then updated the Todo scaffold support harness to reuse the cached `cluster-proof/Dockerfile` builder image as a Linux `meshc` helper and run Docker clustered status/continuity queries from a helper container sharing `--network container:<target>`. Native proofs still use direct host-side `meshc`, while Docker proofs explicitly select the helper seam and keep the existing request-key diff, continuity record, published-port, and fail-closed negative-path assertions. After wiring that seam through `e2e_m047_s05`, the previously blocked Docker clustered-route proof passed natively and in Docker, and the broader S05/S07/tooling/assembled verification rails stayed green.

## Verification

Passed the previously failing `m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container` rail after moving Docker operator queries onto the helper-container seam, then passed the full `e2e_m047_s05`, `e2e_m047_s07`, and `tooling_e2e` rails plus `bash scripts/verify-m047-s05.sh`. The assembled verifier replay also re-ran the S04 prerequisite, `mesh-pkg` S05 contract rail, the Todo tooling rail, the full S05 e2e rail, and the website docs build successfully. After formatting, `cargo fmt --all --check` passed and the targeted clustered-route proof stayed green.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_scaffold_clustered_list_route_truth_is_real_natively_and_in_container -- --nocapture` | 0 | ✅ pass | 54099ms |
| 2 | `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` | 0 | ✅ pass | 65414ms |
| 3 | `cargo test -p meshc --test e2e_m047_s07 -- --nocapture` | 0 | ✅ pass | 18040ms |
| 4 | `cargo test -p meshc --test tooling_e2e test_init_clustered_todo_ -- --nocapture` | 0 | ✅ pass | 7404ms |
| 5 | `bash scripts/verify-m047-s05.sh` | 0 | ✅ pass | 231014ms |
| 6 | `cargo fmt --all --check` | 0 | ✅ pass | 8101ms |


## Deviations

None.

## Known Issues

None.

## Files Created/Modified

- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `.gsd/KNOWLEDGE.md`
- `.gsd/DECISIONS.md`


## Deviations
None.

## Known Issues
None.
