---
id: T01
parent: S02
milestone: M049
provides: []
requires: []
affects: []
key_files: ["scripts/fixtures/m047-s05-clustered-todo/mesh.toml", "scripts/fixtures/m047-s05-clustered-todo/main.mpl", "scripts/fixtures/m047-s05-clustered-todo/api/router.mpl", "compiler/meshc/tests/support/m047_todo_scaffold.rs", "compiler/meshc/tests/e2e_m047_s05.rs", "scripts/verify-m047-s05.sh", "tiny-cluster/work.mpl", ".gsd/KNOWLEDGE.md"]
key_decisions: ["Preserved the historical M047 clustered SQLite Todo contract as a committed fixture instead of depending on the evolving public todo-api scaffold.", "Required retained M047 artifacts to prove fixture-copy provenance explicitly via init.log and retained generated-project markers."]
patterns_established: []
drill_down_paths: []
observability_surfaces: []
duration: ""
verification_result: "Verified the new helper seam first with cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_fixture_ -- --nocapture, then reran the full cargo test -p meshc --test e2e_m047_s05 -- --nocapture target successfully against the fixture-backed path. Repaired the upstream tiny-cluster/work.mpl drift and confirmed the prerequisite rail with cargo run -q -p meshc -- test tiny-cluster/tests. Finally ran bash scripts/verify-m047-s05.sh successfully; .tmp/m047-s05/verify/status.txt=ok, current-phase.txt=complete, phase-report.txt includes m047-s05-fixture-provenance=passed, and latest-proof-bundle.txt points at .tmp/m047-s05/verify/retained-proof-bundle."
completed_at: 2026-04-02T22:51:36.403Z
blocker_discovered: false
---

# T01: Pinned the historical clustered Todo proof to a committed fixture and made the M047 wrapper verify fixture-copy provenance.

> Pinned the historical clustered Todo proof to a committed fixture and made the M047 wrapper verify fixture-copy provenance.

## What Happened
---
id: T01
parent: S02
milestone: M049
key_files:
  - scripts/fixtures/m047-s05-clustered-todo/mesh.toml
  - scripts/fixtures/m047-s05-clustered-todo/main.mpl
  - scripts/fixtures/m047-s05-clustered-todo/api/router.mpl
  - compiler/meshc/tests/support/m047_todo_scaffold.rs
  - compiler/meshc/tests/e2e_m047_s05.rs
  - scripts/verify-m047-s05.sh
  - tiny-cluster/work.mpl
  - .gsd/KNOWLEDGE.md
key_decisions:
  - Preserved the historical M047 clustered SQLite Todo contract as a committed fixture instead of depending on the evolving public todo-api scaffold.
  - Required retained M047 artifacts to prove fixture-copy provenance explicitly via init.log and retained generated-project markers.
duration: ""
verification_result: passed
completed_at: 2026-04-02T22:51:36.404Z
blocker_discovered: false
---

# T01: Pinned the historical clustered Todo proof to a committed fixture and made the M047 wrapper verify fixture-copy provenance.

**Pinned the historical clustered Todo proof to a committed fixture and made the M047 wrapper verify fixture-copy provenance.**

## What Happened

Committed the old clustered SQLite Todo starter under scripts/fixtures/m047-s05-clustered-todo as a literal snapshot of the M047 public contract, then rewrote compiler/meshc/tests/support/m047_todo_scaffold.rs so the historical rails copy that fixture instead of invoking meshc init --template todo-api. The helper now validates the required fixture files up front, fails closed with init.error.txt, records init.log provenance for retained artifacts, and verifies the copied tree shape before runtime proof starts. Updated compiler/meshc/tests/e2e_m047_s05.rs to assert fixture provenance, validate the committed fixture directly, and cover fail-closed paths for missing fixture files and unsupported hidden project renames. Updated scripts/verify-m047-s05.sh so the retained bundle must contain fixture-copy provenance markers and copied generated-project markers. During verification I also had to restore tiny-cluster/work.mpl to the documented route-free Work.add contract because the retained verify-m047-s05.sh wrapper delegates through verify-m047-s04.sh, and that nested replay was failing at tiny-cluster/tests before it could reach the Todo rails.

## Verification

Verified the new helper seam first with cargo test -p meshc --test e2e_m047_s05 m047_s05_todo_fixture_ -- --nocapture, then reran the full cargo test -p meshc --test e2e_m047_s05 -- --nocapture target successfully against the fixture-backed path. Repaired the upstream tiny-cluster/work.mpl drift and confirmed the prerequisite rail with cargo run -q -p meshc -- test tiny-cluster/tests. Finally ran bash scripts/verify-m047-s05.sh successfully; .tmp/m047-s05/verify/status.txt=ok, current-phase.txt=complete, phase-report.txt includes m047-s05-fixture-provenance=passed, and latest-proof-bundle.txt points at .tmp/m047-s05/verify/retained-proof-bundle.

## Verification Evidence

| # | Command | Exit Code | Verdict | Duration |
|---|---------|-----------|---------|----------|
| 1 | `cargo run -q -p meshc -- test tiny-cluster/tests` | 0 | ✅ pass | 1870ms |
| 2 | `cargo test -p meshc --test e2e_m047_s05 -- --nocapture` | 0 | ✅ pass | 61670ms |
| 3 | `bash scripts/verify-m047-s05.sh` | 0 | ✅ pass | 289587ms |


## Deviations

Fixed tiny-cluster/work.mpl during closeout because verify-m047-s05.sh delegates through verify-m047-s04.sh and the nested tiny-cluster package-test rail had drifted red before the Todo fixture-backed proof could run. This was outside the written T01 file list but required to produce truthful retained-wrapper evidence.

## Known Issues

None.

## Files Created/Modified

- `scripts/fixtures/m047-s05-clustered-todo/mesh.toml`
- `scripts/fixtures/m047-s05-clustered-todo/main.mpl`
- `scripts/fixtures/m047-s05-clustered-todo/api/router.mpl`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/e2e_m047_s05.rs`
- `scripts/verify-m047-s05.sh`
- `tiny-cluster/work.mpl`
- `.gsd/KNOWLEDGE.md`


## Deviations
Fixed tiny-cluster/work.mpl during closeout because verify-m047-s05.sh delegates through verify-m047-s04.sh and the nested tiny-cluster package-test rail had drifted red before the Todo fixture-backed proof could run. This was outside the written T01 file list but required to produce truthful retained-wrapper evidence.

## Known Issues
None.
