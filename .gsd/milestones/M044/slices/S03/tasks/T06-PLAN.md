---
estimated_steps: 6
estimated_files: 4
skills_used: []
---

# T06: Close S03 with an assembled verifier and public tooling docs

After the transient operator transport, public CLI, and clustered scaffold all exist, close the slice with one fail-closed acceptance rail and public docs that describe the actual shipped scope. The docs need to point at `meshc init --clustered` plus the read-only `meshc cluster` inspection commands without claiming automatic promotion or a finished `cluster-proof` rewrite.

Steps
1. Add `scripts/verify-m044-s03.sh` that replays `scripts/verify-m044-s02.sh`, runs the named `m044_s03_operator_` and `m044_s03_scaffold_` filters, fails closed on `running 0 tests`, checks scaffold output for the generic `MESH_*` contract and absence of `CLUSTER_PROOF_*`, and archives live operator/scaffold artifacts under `.tmp/m044-s03/verify/`.
2. Update `README.md` plus the tooling/getting-started VitePress pages so the public command surface shows both `meshc init` and `meshc init --clustered`, names the new `meshc cluster` inspection commands, and keeps low-level distributed docs separate from the new scaffolded story.
3. Keep the docs honest about scope: S03 is read-only operator inspection only; bounded automatic promotion and the full `cluster-proof` rewrite remain later slices.
4. Rebuild the docs site and rerun the assembled verifier so the public text and proof rail match.

## Inputs

- `scripts/verify-m044-s02.sh`
- `compiler/meshc/tests/e2e_m044_s03.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`

## Expected Output

- `scripts/verify-m044-s03.sh`
- `README.md`
- `website/docs/docs/getting-started/index.md`
- `website/docs/docs/tooling/index.md`

## Verification

`bash scripts/verify-m044-s03.sh`
`npm --prefix website run build`
