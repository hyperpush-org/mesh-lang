# S04: Thin cluster-proof consumer and truthful operator/docs rail — UAT

**Milestone:** M042
**Written:** 2026-03-29T05:32:30.418Z

# S04 UAT — Thin cluster-proof consumer and truthful operator/docs rail

## Preconditions
- Run from the repo root.
- Docker is available locally for the one-image operator replays.
- The Rust workspace builds in the current checkout.
- For the optional live Fly inspection path, an existing Fly deployment and matching `CLUSTER_PROOF_FLY_APP` / `CLUSTER_PROOF_BASE_URL` are available.

## Test Case 1 — Thin-consumer split stays green
1. Run `cargo run -q -p meshc -- test cluster-proof/tests`.
   - Expected: the `cluster-proof` config and keyed work contract suites pass, including legacy probe coverage and keyed submit/status parsing/response checks.
2. Run `cargo run -q -p meshc -- build cluster-proof`.
   - Expected: `cluster-proof/cluster-proof` builds successfully.
3. Inspect the source layout in `cluster-proof/`.
   - Expected: `work.mpl` is shared glue only, `work_legacy.mpl` owns legacy `GET /work`, and `work_continuity.mpl` owns keyed `POST /work` / `GET /work/:request_key` adaptation over `Continuity.*`.

## Test Case 2 — Historical M039 one-image operator rail still holds
1. Run `bash scripts/verify-m039-s04.sh`.
   - Expected: the wrapper replays the historical M039 local Docker operator path and exits with `verify-m039-s04: ok`.
2. Inspect `.tmp/m039-s04/`.
   - Expected: the artifact root contains the one-image baseline evidence and fail-closed phase outputs instead of a partial success story.

## Test Case 3 — Runtime-owned local continuity authority is green
1. Run `bash scripts/verify-m042-s03.sh`.
   - Expected: the authoritative local owner-loss/rejoin replay exits with `verify-m042-s03: ok`.
2. Inspect `.tmp/m042-s03/verify/`.
   - Expected: owner-loss and rejoin artifact bundles are present and show the runtime-owned recovery story without app-authored repair logic.

## Test Case 4 — Packaged one-image keyed continuity rail is green
1. Run `bash scripts/verify-m042-s04.sh`.
   - Expected: the packaged wrapper builds the repo-root `cluster-proof` image, stands up the two-node local rail, and exits with `verify-m042-s04: ok`.
2. Watch the wrapper output.
   - Expected: it prints all four packaged keyed success signals:
     - `packaged keyed submit response: keyed payload ok`
     - `packaged pending keyed status on node-a: keyed payload ok`
     - `packaged completed keyed status on node-a: keyed payload ok`
     - `packaged completed keyed status on node-b: keyed payload ok`
3. Inspect `.tmp/m042-s04/`.
   - Expected: the wrapper archived membership, legacy work, keyed submit, and keyed status artifacts so the first failing phase would be obvious from JSON/logs alone.

## Test Case 5 — Docs/help surfaces tell the same truth
1. Run `bash scripts/verify-m042-s04-proof-surface.sh`.
   - Expected: the proof-surface verifier passes and reports the distributed proof surface verified.
2. Run `bash scripts/verify-m042-s04-fly.sh --help`.
   - Expected: help text states that the Fly rail is read-only, requires an existing app/base URL, does not do deploys or `POST /work`, and points to the local continuity authority instead of claiming destructive recovery proof.
3. Run `npm --prefix website run build`.
   - Expected: the VitePress docs build completes successfully after the proof-surface verifier passes.

## Edge Cases
- **Malformed keyed submit / blank payload / invalid request key:** covered by `cargo run -q -p meshc -- test cluster-proof/tests`; expected result is fail-closed parsing/validation without legacy fallback.
- **Legacy probe drift after the refactor:** covered by the same test suite plus `bash scripts/verify-m039-s04.sh`; expected result is legacy `GET /work` still works and remains clearly isolated from keyed continuity code.
- **Fly contract safety when no live deployment is available:** `bash scripts/verify-m042-s04-fly.sh --help` must still succeed locally and must not imply deploys, restarts, or destructive continuity proof.
