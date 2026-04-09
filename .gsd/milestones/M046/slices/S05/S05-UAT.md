# S05: Equal-surface scaffold alignment — UAT

**Milestone:** M046
**Written:** 2026-04-01T02:07:01.455Z

# S05: Equal-surface scaffold alignment — UAT

**Milestone:** M046  
**Written:** 2026-03-31

## UAT Type

- UAT mode: mixed scaffold-contract + runtime/CLI proof + docs/verifier closeout
- Why this mode is sufficient: S05 changed three kinds of truth surfaces at once — generated scaffold source/output, runtime-owned CLI proof rails, and public docs/readmes/verifier references. The acceptance surface must therefore exercise scaffold generation, the live equal-surface runtime rail, docs/readme drift guards, and the retained-bundle closeout scripts together.

## Preconditions

- Run from the repository root with Cargo, Node/npm, and the repo toolchain already available.
- `target/`, `.tmp/m046-s05/`, and temporary scaffold output directories must be writable.
- Loopback ports on `127.0.0.1` / `::1` must be free for the two-node scaffold runtime proof.
- No stale generated-scaffold node processes should be holding prior test ports.
- For the direct closeout rail, allow enough wall-clock time for delegated S03 and S04 replays plus the docs build.

## Smoke Test

Run:

```bash
cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture
cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
```

**Expected:** the clustered scaffold unit and CLI smoke both pass, proving `meshc init --clustered` emits the route-free equal-surface contract instead of the deleted routeful scaffold shape.

## Test Cases

### 1. `meshc init --clustered` emits the same route-free clustered-work contract as the proof packages

1. Run:
   ```bash
   cargo test -p mesh-pkg scaffold_clustered_project_writes_public_cluster_contract -- --nocapture
   cargo test -p meshc --test tooling_e2e test_init_clustered_creates_project -- --nocapture
   ```
2. **Expected:**
   - Generated `mesh.toml` is package-only and does not contain `[cluster]` or manifest declaration blocks.
   - Generated `main.mpl` has one `Node.start_from_env()` bootstrap path and only logs success/failure.
   - Generated `work.mpl` contains `declared_work_runtime_name()`, exactly one `clustered(work)` declaration, stable runtime name `Work.execute_declared_work`, and visible `1 + 1` work.
   - The generated scaffold README teaches source-owned clustered work and CLI inspection rather than `/health`, `/work`, `Continuity.submit_declared_work(...)`, or proof-only timing guidance.
   - Any surviving `[cluster]`, `HTTP.serve(...)`, `/health`, `/work`, `Continuity.submit_declared_work(...)`, or `Timer.sleep(...)` content would fail the tests.

### 2. Historical scaffold rails now fail closed on route-free contract drift instead of reviving deleted HTTP flows

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m044_s03 m044_s03_scaffold_ -- --nocapture
   cargo test -p meshc --test e2e_m045_s01 m045_s01_ -- --nocapture
   cargo test -p meshc --test e2e_m045_s02 m045_s02_ -- --nocapture
   cargo test -p meshc --test e2e_m045_s03 m045_s03_ -- --nocapture
   ```
2. **Expected:**
   - All four targets pass.
   - The M044/M045 rails assert the route-free scaffold source/build/delegation contract only.
   - None of these rails require `/health`, `/work`, `[cluster]`, request-key-only continuity assumptions, or app-owned `Continuity.submit_declared_work(...)` helpers.
   - `m045_s03` acts as a helper/delegation guard around the shared S05 proof seam rather than a second bespoke live scaffold harness.

### 3. The generated scaffold proves runtime truth through the same CLI-only surfaces as `tiny-cluster/` and `cluster-proof`

1. Run:
   ```bash
   cargo test -p meshc --test e2e_m046_s05 m046_s05_ -- --nocapture
   ```
2. **Expected:**
   - The target reports `running 4 tests` and all four pass.
   - The generated scaffold is compared on disk against the route-free package contract, built to a temp output path, and launched on two nodes.
   - The proof discovers startup truth through `meshc cluster status --json`, `meshc cluster continuity --json`, `meshc cluster continuity <node> <request-key> --json`, and `meshc cluster diagnostics --json` only.
   - The rail retains generated-project source, build metadata, cluster status/continuity/diagnostics JSON, and per-node logs under `.tmp/m046-s05/retained-m046-s05-artifacts` (via the direct verifier) or the scenario-specific `.tmp/m046-s05/...` directories.

### 4. Public docs, repo README, and package READMEs all teach the same route-free equal-surface story

1. Run:
   ```bash
   npm --prefix website run build
   ! rg -n "\[cluster\]|Continuity\.submit_declared_work|/health|/work/:request_key|Timer\.sleep\(5000\)" website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md
   rg -q 'scripts/verify-m046-s05\.sh' website/docs/docs/getting-started/clustered-example/index.md website/docs/docs/tooling/index.md website/docs/docs/distributed-proof/index.md website/docs/docs/distributed/index.md README.md
   rg -q 'meshc cluster status <node-name@host:port> --json' tiny-cluster/README.md cluster-proof/README.md
   rg -q 'meshc cluster continuity <node-name@host:port> --json' tiny-cluster/README.md cluster-proof/README.md
   rg -q 'meshc cluster continuity <node-name@host:port> <request-key> --json' tiny-cluster/README.md cluster-proof/README.md
   rg -q 'meshc cluster diagnostics <node-name@host:port> --json' tiny-cluster/README.md cluster-proof/README.md
   ```
2. **Expected:**
   - The VitePress docs build passes.
   - The guarded docs and repo README no longer contain routeful clustered-example strings.
   - The docs point at `scripts/verify-m046-s05.sh` as the authoritative equal-surface closeout rail and keep `scripts/verify-m045-s05.sh` clearly historical.
   - `tiny-cluster/README.md` and `cluster-proof/README.md` both use the same operator sequence: status, continuity list, continuity record, then diagnostics.
   - The three clustered-example surfaces are presented as equal canonical examples rather than “real” versus “toy” paths.

### 5. The direct S05 verifier and historical alias retain one truthful assembled proof chain

1. Run:
   ```bash
   bash scripts/verify-m046-s05.sh
   bash scripts/verify-m045-s05.sh
   ```
2. **Expected:**
   - Both scripts print `ok` and exit successfully.
   - `.tmp/m046-s05/verify/status.txt` contains `ok` and `.tmp/m046-s05/verify/current-phase.txt` contains `complete`.
   - `.tmp/m046-s05/verify/phase-report.txt` shows the full phase set passing: `contract-guards`, delegated `m046-s03` replay/retain, delegated `m046-s04` replay/retain, scaffold unit, scaffold smoke, `m046-s05-e2e`, docs build, artifact checks, bundle-chain, and bundle-shape.
   - `.tmp/m046-s05/verify/latest-proof-bundle.txt` points at `.tmp/m046-s05/verify/retained-proof-bundle`.
   - That retained bundle contains copied `retained-m046-s03-artifacts`, `retained-m046-s04-artifacts`, and `retained-m046-s05-artifacts` subtrees.
   - The historical wrapper does not invent extra proof steps; it only delegates to the direct S05 verifier and checks the retained artifact/status shape.

## Edge Cases

### Older scaffold rails must stay narrow once the authoritative S05 runtime rail exists

1. Inspect the passing `m044_s03_scaffold_`, `m045_s01_`, `m045_s02_`, and `m045_s03_` results after the commands above.
2. **Expected:** these tests protect route-free source/build/delegation truth, but they do not create a second runtime proof stack alongside `e2e_m046_s05`.

### The docs may mention that routeful proof/status seams are absent without reviving real route guidance

1. Inspect the updated README/docs surfaces after the build passes.
2. **Expected:** wording may explain that app-owned submit/status routes were removed, but the owned surfaces must not present real `/health`, `/work`, or request-key-only guidance as current truth.

### The retained proof-bundle pointer is the first debugging stop for wrapper failures

1. If `bash scripts/verify-m045-s05.sh` fails, inspect:
   ```bash
   cat .tmp/m046-s05/verify/phase-report.txt
   cat .tmp/m046-s05/verify/latest-proof-bundle.txt
   ```
2. **Expected:** debugging starts from the delegated S05 phase report and the nested retained proof bundle, not by adding new assertions back into the historical wrapper.

## Failure Signals

- `meshc init --clustered` emits `[cluster]`, HTTP routes, explicit continuity submission calls, or proof-only timing seams again.
- Historical scaffold rails start requiring deleted routeful behavior or fork a second runtime harness.
- `e2e_m046_s05` stops proving startup truth through `meshc cluster status|continuity|diagnostics` only.
- Docs/README surfaces drift apart on the operator sequence or stop naming the scaffold, `tiny-cluster`, and `cluster-proof` as equal canonical clustered examples.
- `scripts/verify-m046-s05.sh` stops producing a green `status.txt`, complete phase report, or the nested retained proof-bundle chain.
- `scripts/verify-m045-s05.sh` grows independent proof logic instead of staying a retained alias.

## Requirements Proved By This UAT

- R090 — `meshc init --clustered`, `tiny-cluster/`, and rebuilt `cluster-proof/` remain equally canonical clustered examples with one shared route-free clustered-work contract.
- R092 — The public clustered story no longer depends on HTTP submit/status routes for proof or operator truth.
- R085, R086, R088, R089, and R091 are advanced because the source-owned clustered-work story, route-free operator flow, and shared CLI/runtime proof surfaces now cover the generated scaffold as well as the local and packaged proof apps.

## Notes for Tester

If the assembled equal-surface rail goes red, debug in this order: scaffold contract drift, historical contract/delegation guards, `e2e_m046_s05` runtime JSON/log evidence, docs/readme routeful-string drift, then the retained proof-bundle chain under `.tmp/m046-s05/verify/latest-proof-bundle.txt`. Do not reintroduce app-owned routes, proof-only timing code, or a second bespoke scaffold harness as a shortcut; those are exactly the regressions S05 is supposed to catch.
