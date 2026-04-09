---
estimated_steps: 5
estimated_files: 6
skills_used: []
---

# T05: Rewrite `cluster-proof` onto the typed continuity surface and real clustered manifest

Now that the typed public API is real, migrate the proof app onto it and delete the stringly shim path.

1. Add `cluster-proof/mesh.toml` with the clustered opt-in and declared handler boundary from T01, keeping package metadata valid for ordinary package build/test flows.
2. Rewrite `cluster-proof/work_continuity.mpl` to consume typed `Continuity` values directly and delete the runtime continuity `parse_*_json` helpers plus `*.from_json(...)` adapters for authority, submit, and record payloads.
3. Update `cluster-proof/work.mpl`, `cluster-proof/main.mpl`, and package tests so only app HTTP payload shaping remains local JSON work while continuity/authority data stays typed Mesh values end-to-end.
4. Extend the M044 proof rail so `cluster-proof` consumer coverage lives in the slice’s named compiler/package tests rather than as an undocumented manual rewrite.

## Inputs

- `compiler/meshc/tests/e2e_m044_s01.rs`
- `cluster-proof/work_continuity.mpl`
- `cluster-proof/work.mpl`
- `cluster-proof/main.mpl`
- `cluster-proof/tests/work.test.mpl`

## Expected Output

- ``cluster-proof/mesh.toml` with clustered opt-in and declared handler boundary`
- ``cluster-proof` continuity code consuming typed Mesh values without runtime JSON decode helpers`
- `Proof-app coverage aligned with the typed continuity public contract`

## Verification

cargo run -q -p meshc -- build cluster-proof
cargo run -q -p meshc -- test cluster-proof/tests
! rg -n 'ContinuityAuthorityStatus\.from_json|ContinuitySubmitDecision\.from_json|WorkRequestRecord\.from_json|parse_authority_status_json|parse_continuity_submit_response|parse_continuity_record' cluster-proof/work_continuity.mpl
