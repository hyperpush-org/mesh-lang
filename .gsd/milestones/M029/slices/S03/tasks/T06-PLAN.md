---
estimated_steps: 5
estimated_files: 11
skills_used:
  - test
  - lint
---

# T06: Canonicalize remaining Mesher files and record final slice proof

**Slice:** S03 — Multiline imports and final formatter compliance
**Milestone:** M029

## Description

Finish the last formatter-red Mesher files, then run the full slice closeout gate across both dogfood apps and capture the green proof in `S03-UAT.md`. This task is the truthful end of S03: Mesher must be formatter-clean, `reference-backend/` must stay green as a regression target, both builds must pass, and the repo-level grep checks must still show no long single-line or spaced dotted imports.

## Steps

1. Read the two migration files, two test files, and six type files that remain in Mesher's `fmt --check` red set so the final formatter wave is grounded in the actual source.
2. Run `cargo run -q -p meshc -- fmt mesher/types`, `cargo run -q -p meshc -- fmt mesher/tests`, and `cargo run -q -p meshc -- fmt mesher/migrations` to move the remaining Mesher files onto canonical formatter output.
3. Run the full closeout gate: `meshc fmt --check` for `mesher` and `reference-backend`, `meshc build` for both apps, the repo-wide long-import grep, and the repo-wide spaced-dotted-import grep.
4. Write `.gsd/milestones/M029/slices/S03/S03-UAT.md` with the passed command set, what the slice proved, and the fact that `reference-backend/api/health.mpl` remained the canonical multiline smoke target.
5. Re-run the file-existence check for `S03-UAT.md` as part of the closeout gate so the proof artifact is guaranteed to exist when the task finishes.

## Must-Haves

- [ ] The remaining Mesher `types`, `tests`, and `migrations` files are on canonical formatter output.
- [ ] `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- fmt --check reference-backend`, `cargo run -q -p meshc -- build mesher`, and `cargo run -q -p meshc -- build reference-backend` all pass.
- [ ] `.gsd/milestones/M029/slices/S03/S03-UAT.md` exists and records the green closeout evidence for the slice.

## Verification

- `cargo run -q -p meshc -- fmt mesher/types && cargo run -q -p meshc -- fmt mesher/tests && cargo run -q -p meshc -- fmt mesher/migrations && cargo run -q -p meshc -- fmt --check mesher && cargo run -q -p meshc -- fmt --check reference-backend && cargo run -q -p meshc -- build mesher && cargo run -q -p meshc -- build reference-backend && ! rg -n '^from .{121,}' mesher -g '*.mpl' && ! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl' && test -f .gsd/milestones/M029/slices/S03/S03-UAT.md`
- `! rg -n '^from .{121,}' mesher -g '*.mpl' && ! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`

## Inputs

- `mesher/migrations/20260216120000_create_initial_schema.mpl` — formatter-red migration file
- `mesher/migrations/20260226000000_seed_default_org.mpl` — formatter-red migration file
- `mesher/tests/fingerprint.test.mpl` — formatter-red test file
- `mesher/tests/validation.test.mpl` — formatter-red test file
- `mesher/types/alert.mpl` — formatter-red type file
- `mesher/types/event.mpl` — formatter-red type file
- `mesher/types/issue.mpl` — formatter-red type file
- `mesher/types/project.mpl` — formatter-red type file
- `mesher/types/retention.mpl` — formatter-red type file
- `mesher/types/user.mpl` — formatter-red type file
- `reference-backend/api/health.mpl` — canonical multiline import smoke target that stays green through final proof

## Expected Output

- `mesher/migrations/20260216120000_create_initial_schema.mpl` — canonical formatter output for the migration file
- `mesher/migrations/20260226000000_seed_default_org.mpl` — canonical formatter output for the migration file
- `mesher/tests/fingerprint.test.mpl` — canonical formatter output for the test file
- `mesher/tests/validation.test.mpl` — canonical formatter output for the test file
- `mesher/types/alert.mpl` — canonical formatter output for the type file
- `mesher/types/event.mpl` — canonical formatter output for the type file
- `mesher/types/issue.mpl` — canonical formatter output for the type file
- `mesher/types/project.mpl` — canonical formatter output for the type file
- `mesher/types/retention.mpl` — canonical formatter output for the type file
- `mesher/types/user.mpl` — canonical formatter output for the type file
- `.gsd/milestones/M029/slices/S03/S03-UAT.md` — final closeout proof artifact for S03

## Observability Impact

- Runtime signals: none added; this task is formatter/build compliance work plus the slice UAT artifact.
- Inspection surfaces: `cargo run -q -p meshc -- fmt mesher/types`, `cargo run -q -p meshc -- fmt mesher/tests`, `cargo run -q -p meshc -- fmt mesher/migrations`, `cargo run -q -p meshc -- fmt --check mesher`, `cargo run -q -p meshc -- fmt --check reference-backend`, `cargo run -q -p meshc -- build mesher`, `cargo run -q -p meshc -- build reference-backend`, `! rg -n '^from .{121,}' mesher -g '*.mpl'`, `! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`, `/tmp/m029-s03-fmt-mesher.log`, and `.gsd/milestones/M029/slices/S03/S03-UAT.md`.
- Failure visibility: the formatter commands expose any remaining non-canonical Mesher or `reference-backend` files, the build commands expose syntax/type drift introduced by the cleanup wave, the two `rg` checks expose long single-line or spaced dotted imports, `/tmp/m029-s03-fmt-mesher.log` preserves the first Mesher formatter failure for post-mortem inspection, and `S03-UAT.md` records the exact green closeout proof a future agent should compare against.
