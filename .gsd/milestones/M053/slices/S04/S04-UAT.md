# S04: Public docs and Fly reference assets match the shipped contract — UAT

**Milestone:** M053
**Written:** 2026-04-05T22:13:22.428Z

# S04 UAT — Public docs and Fly reference assets match the shipped contract

## Preconditions

- The worktree contains the S04 docs and verifier changes.
- `node`, `npm`, `cargo`, `python3`, and `bash` are installed.
- Website dependencies are available so `npm --prefix website run build` can run.
- No Fly credentials are required; the retained Fly rail for this slice is read-only/help-surface only.

## Test Case 1 — First-contact docs stay starter-first and keep the SQLite/Postgres split honest

1. Run `bash scripts/verify-m050-s02.sh`.
   - **Expected:** Exit code `0`.
   - **Expected:** `.tmp/m050-s02/verify/status.txt` is `ok` and `.tmp/m050-s02/verify/current-phase.txt` is `complete`.
2. Inspect `.tmp/m050-s02/verify/built-html/summary.json`.
   - **Expected:** The retained built-doc summary includes `getting-started`, `clustered-example`, and `tooling` entries.
   - **Expected:** Those entries retain markers for the honest local-only SQLite starter, the serious shared/deployable PostgreSQL starter, and the proof-page handoff for staged deploy + failover plus the hosted packages/public-surface contract.
3. Spot-check the public source docs (`README.md`, `website/docs/docs/getting-started/index.md`, `website/docs/docs/getting-started/clustered-example/index.md`, and `website/docs/docs/tooling/index.md`).
   - **Expected:** None of them present Fly or retained proof fixtures as first-contact starter surfaces.
   - **Expected:** The public ladder stays clustered scaffold -> SQLite local-only starter -> PostgreSQL shared/deployable starter -> proof pages.

## Test Case 2 — Distributed Proof and retained Fly assets present one coherent M053 proof map

1. Run `bash scripts/verify-production-proof-surface.sh && cargo run -q -p meshc -- test scripts/fixtures/clustered/cluster-proof/tests`.
   - **Expected:** Exit code `0`.
2. Inspect `website/docs/docs/distributed-proof/index.md` and `website/docs/docs/distributed/index.md`.
   - **Expected:** `Distributed Proof` names `bash scripts/verify-m053-s01.sh`, `bash scripts/verify-m053-s02.sh`, and `bash scripts/verify-m053-s03.sh` as the generated Postgres starter's staged deploy, failover, and hosted-contract chain.
   - **Expected:** SQLite is explicitly local-only / not a clustered proof surface.
   - **Expected:** Fly is described as a retained read-only/reference lane rather than a required or canonical public starter lane.
3. Inspect `scripts/fixtures/clustered/cluster-proof/README.md` and `scripts/verify-m043-s04-fly.sh --help`.
   - **Expected:** Both surfaces describe `cluster-proof` / Fly as retained reference proof only.
   - **Expected:** The README still preserves the literal `route-free` and `It is not a public starter surface` markers required by the fixture tests.
4. Review the `meshc test` output from `scripts/fixtures/clustered/cluster-proof/tests/work.test.mpl`.
   - **Expected:** The retained fixture package reports the README/reference wording tests as passing.

## Test Case 3 — The assembled S04 verifier fails closed and retains the right evidence bundle

1. Run `node --test scripts/tests/verify-m053-s04-contract.test.mjs`.
   - **Expected:** Exit code `0` with all contract tests passing.
   - **Expected:** The suite covers negative cases for Fly-first wording, proof-maze-first regressions, retained reference drift, and corrupted/duplicated doc tails.
2. Run `bash scripts/verify-m053-s04.sh`.
   - **Expected:** Exit code `0`.
3. Inspect `.tmp/m053-s04/verify/status.txt`, `.tmp/m053-s04/verify/current-phase.txt`, and `.tmp/m053-s04/verify/phase-report.txt`.
   - **Expected:** `status.txt=ok`.
   - **Expected:** `current-phase.txt=complete`.
   - **Expected:** `phase-report.txt` shows all phases passing: `docs-build`, `retain-built-html`, `built-html`, `first-contact-rail`, `retain-first-contact-bundle`, `proof-surface-rail`, `proof-surface-output`, `cluster-proof-fixture`, `m053-s04-contract`, and `verifier-bundle`.
4. Inspect `.tmp/m053-s04/verify/built-html/summary.json`, `.tmp/m053-s04/verify/log-paths.txt`, and `.tmp/m053-s04/verify/retained-m050-s02-verify/status.txt`.
   - **Expected:** The built-doc summary retains the rendered markers for first-contact and distributed-proof surfaces.
   - **Expected:** `log-paths.txt` points at the per-phase logs.
   - **Expected:** The copied upstream first-contact bundle exists and reports `status.txt=ok`.

## Edge Cases

- If `website/docs/docs/tooling/index.md` or another public doc reintroduces a duplicated/corrupted tail, `node --test scripts/tests/verify-m053-s04-contract.test.mjs` or the built-HTML assertions in `bash scripts/verify-m053-s04.sh` must fail closed.
- If Fly or `cluster-proof` is reworded as a public starter surface, both the Node contract suite and the retained fixture tests must fail.
- If the docs build succeeds but the verifier bundle is missing copied HTML, `docs-build.log`, or the retained M050 bundle pointer, `bash scripts/verify-m053-s04.sh` must fail instead of accepting partial evidence.
