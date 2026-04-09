# S01: One-public-URL starter ingress truth

**Goal:** Prove the serious Postgres starter behind one public ingress URL, retain auditable evidence of proxy/public ingress versus Mesh runtime placement for the same clustered request, and align the generated starter surfaces with that bounded server-side story.
**Demo:** After this: After this: the serious PostgreSQL starter can be exercised through one public app URL, and retained evidence shows ingress-node versus owner, replica, and execution truth for the same real request.

## Tasks
- [x] **T01: Added a first-pass public-ingress harness and M054 staged Postgres proof rail; compile passes, full runtime proof still needs rerun.** — - Why: M053 proved the serious starter on direct node ports, but S01 needs a real one-public-URL replay that shows where proxy/platform ingress ends and where Mesh runtime placement begins.
- Do: Add a thin local public-ingress harness on top of the staged two-node Postgres starter, reuse the existing staged deploy helper and `deploy-smoke.sh` against that single base URL, diff `meshc cluster continuity` before/after a real clustered `GET /todos`, and retain redacted artifacts that show the same request's `ingress_node`, `owner_node`, `replica_node`, and `execution_node` truth.
- Done when: the new `meshc` e2e target leaves a redacted `.tmp/m054-s01/...` bundle proving CRUD through one public URL plus same-request ingress/owner/replica/execution evidence without changing the starter runtime contract.
  - Estimate: 1 context window
  - Files: compiler/meshc/tests/e2e_m054_s01.rs, compiler/meshc/tests/support/mod.rs, compiler/meshc/tests/support/m054_public_ingress.rs, compiler/meshc/tests/support/m053_todo_postgres_deploy.rs
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} cargo test -p meshc --test e2e_m054_s01 -- --nocapture
- [x] **T02: Aligned the Postgres starter README and added the M054 contract/verifier rails, but the standby-first public-ingress e2e still fails on the clustered route transport.** — - Why: the new ingress proof only helps if the generated starter, committed example, and retained verifier all describe the same bounded server-side story instead of drifting back to direct-node or Fly-specific claims.
- Do: Update the Postgres scaffold/example README wording so one public app URL and `meshc cluster` inspection are the explicit contract, add a cheap contract test plus `scripts/verify-m054-s01.sh`, and make the wrapper replay the new cargo rail, scaffold/example parity, and retained-bundle checks fail-closed.
- Done when: the generated starter and committed example render the same one-public-URL wording, the wrapper republishes a retained proof bundle instead of relying on ad-hoc local state, and older starter-boundary rails remain truthful.
  - Estimate: 1 context window
  - Files: compiler/mesh-pkg/src/scaffold.rs, examples/todo-postgres/README.md, compiler/meshc/tests/e2e_m054_s01.rs, scripts/verify-m054-s01.sh, scripts/tests/verify-m054-s01-contract.test.mjs
  - Verify: DATABASE_URL=${DATABASE_URL:?set DATABASE_URL} bash scripts/verify-m054-s01.sh
