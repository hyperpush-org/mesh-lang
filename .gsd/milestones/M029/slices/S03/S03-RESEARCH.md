# S03: Multiline imports and final formatter compliance — Research

**Depth:** Targeted. This is established Mesher application cleanup, not new compiler work, but the slice still has one real execution wrinkle: `meshc fmt --check mesher` is red across 35 files, so targeted import edits alone will not close the slice. I followed the `debug-like-expert` skill’s **VERIFY, DON'T ASSUME** rule and measured the live formatter/build surface instead of trusting the roadmap counts.

## Requirements Targeted

### Direct
- **R024** — finish Mesher’s remaining multiline-import cleanup and bring the repo to final `meshc fmt --check` compliance.

### Supported / regression-kept-green
- **R011** — this is still dogfood-driven DX cleanup from Mesher readability/tooling friction, not speculative language expansion.
- **R026 / R027** — both are already validated by S01, but S03 must keep their proof surfaces green (`fmt --check reference-backend`, no dotted-path regression) while closing Mesher.

## Summary

1. **Both dogfood apps already build on current HEAD.**
   - Verified with:
     - `cargo run -q -p meshc -- build mesher`
     - `cargo run -q -p meshc -- build reference-backend`
   - S03 is formatter/readability work, not a compile-fix slice.

2. **`reference-backend/` is already clean and should be treated as a regression gate, not an edit target.**
   - `cargo run -q -p meshc -- fmt --check reference-backend` passes.
   - `rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'` returns no matches, so the old `Api. Router` / `Jobs. Worker` corruption is not live on current HEAD.
   - No backend source changes are currently justified for S03.

3. **`mesher/` still fails final formatter compliance broadly.**
   - `cargo run -q -p meshc -- fmt --check mesher` reports **35 files** needing rewrite.
   - Directory spread:
     - `mesher/api`: 7
     - `mesher/ingestion`: 6
     - `mesher/main.mpl`: 1
     - `mesher/migrations`: 2
     - `mesher/services`: 8
     - `mesher/storage`: 3
     - `mesher/tests`: 2
     - `mesher/types`: 6
   - This means the slice naturally ends with a deliberate repo-wide `meshc fmt mesher` pass, not only hand-edited imports.

4. **The manual multiline-import surface is much smaller than the roadmap text now implies.**
   - Current live count: **10** over-120-char `from ... import ...` lines across **8 files**.
   - Exact targets:
     - `mesher/api/alerts.mpl:6`
     - `mesher/api/dashboard.mpl:7`
     - `mesher/api/team.mpl:8`
     - `mesher/ingestion/routes.mpl:13`
     - `mesher/main.mpl:13`
     - `mesher/main.mpl:15`
     - `mesher/main.mpl:17`
     - `mesher/main.mpl:18`
     - `mesher/services/project.mpl:5`
     - `mesher/services/user.mpl:5`
   - So the roadmap’s “20+ mesher files” assumption is stale after S02. The real import conversion work is compact.

5. **Mesher currently has no local multiline-import prior art.**
   - `rg -n '^from .*\($|^\)$' mesher reference-backend -g '*.mpl'` finds only `reference-backend/api/health.mpl`.
   - That file is the canonical style anchor for S03: one imported name per line, 2-space indentation, closing `)` on its own line.

6. **The slice should stay entirely out of compiler code unless verification unexpectedly regresses.**
   - S01 already closed the formatter bug and its exact-output tests.
   - Current evidence says S03 is pure Mesher source cleanup plus final proof, with `reference-backend/api/health.mpl` reused only as a formatting model/smoke target.

## Implementation Landscape

### Canonical multiline-import style anchor

Use `reference-backend/api/health.mpl` as the source-of-truth shape:

```mpl
from Jobs.Worker import (
  get_worker_boot_id,
  ...
  get_worker_started_at
)
```

Important details for S03:
- opening `(` stays on the import line
- each imported name gets its own indented line
- closing `)` is alone on its own line
- this is already proven safe under the fixed formatter from S01

### Manual import rewrite surface

These are the only files that need hand conversion before the repo-wide formatter run:

- `mesher/main.mpl`
  - four long imports:
    - `Ingestion.Routes`
    - `Api.Dashboard`
    - `Api.Team`
    - `Api.Alerts`
- `mesher/ingestion/routes.mpl`
  - one very long `from Storage.Queries import ...` line (282 chars)
- `mesher/api/alerts.mpl`
  - one long `from Storage.Queries import ...` line
- `mesher/api/dashboard.mpl`
  - one long `from Storage.Queries import ...` line
- `mesher/api/team.mpl`
  - one long `from Storage.Queries import ...` line
- `mesher/services/project.mpl`
  - one long `from Storage.Queries import ...` line
- `mesher/services/user.mpl`
  - one long `from Storage.Queries import ...` line

This is the only place where S03 needs manual source editing for readability. Everything else in the 35-file `fmt --check` set can be treated as formatter canonicalization.

### Mechanical formatter-rewrite surface

After the import rewrites, `meshc fmt mesher` still needs to touch the full current 35-file set:

- `mesher/api`
  - `alerts.mpl`
  - `dashboard.mpl`
  - `detail.mpl`
  - `helpers.mpl`
  - `search.mpl`
  - `settings.mpl`
  - `team.mpl`
- `mesher/ingestion`
  - `auth.mpl`
  - `fingerprint.mpl`
  - `pipeline.mpl`
  - `routes.mpl`
  - `validation.mpl`
  - `ws_handler.mpl`
- `mesher/main.mpl`
- `mesher/migrations`
  - `20260216120000_create_initial_schema.mpl`
  - `20260226000000_seed_default_org.mpl`
- `mesher/services`
  - `event_processor.mpl`
  - `org.mpl`
  - `project.mpl`
  - `rate_limiter.mpl`
  - `retention.mpl`
  - `stream_manager.mpl`
  - `user.mpl`
  - `writer.mpl`
- `mesher/storage`
  - `queries.mpl`
  - `schema.mpl`
  - `writer.mpl`
- `mesher/tests`
  - `fingerprint.test.mpl`
  - `validation.test.mpl`
- `mesher/types`
  - `alert.mpl`
  - `event.mpl`
  - `issue.mpl`
  - `project.mpl`
  - `retention.mpl`
  - `user.mpl`

I read representative non-import files from that set (`api/helpers.mpl`, `api/settings.mpl`, `ingestion/pipeline.mpl`, `storage/queries.mpl`, `types/project.mpl`). Nothing there suggests new architecture work or compiler-side changes. Their presence in the `fmt --check` output is formatter canonicalization churn, not hidden feature work.

### Regression-only backend surface

`reference-backend/` is already in the desired state:
- `cargo run -q -p meshc -- fmt --check reference-backend` passes
- `cargo run -q -p meshc -- build reference-backend` passes
- dotted-path grep is clean

Use it only for:
- final regression proof
- the canonical multiline-import example (`api/health.mpl`)
- a quick smoke signal if the formatter behavior ever looks suspicious again

## Recommendation

Three tasks are enough.

### T01: Convert the 10 overlong imports to parenthesized multiline form in the 8 Mesher target files

**Goal**
- rewrite only the import statements listed above
- use `reference-backend/api/health.mpl` as the exact formatting model
- do not touch compiler code or reference-backend source

**Why first**
- this is the only true manual-edit surface in S03
- it directly closes the remaining R024 readability requirement
- it makes the subsequent repo-wide formatter pass truthful instead of letting `meshc fmt` reflow long single-line imports on its own terms

### T02: Run `cargo run -q -p meshc -- fmt mesher` and accept the full 35-file canonicalization rewrite

**Goal**
- move the entire Mesher tree onto the fixed formatter’s canonical output
- let formatter-only files (`types/`, `tests/`, `migrations/`, etc.) settle in one mechanical pass

**Why separate**
- keeps the human-authored import surgery distinct from the broad mechanical rewrite
- makes review/debugging easier if a follow-up grep or build fails
- reflects the real structure of the slice: targeted readability edits first, bulk canonicalization second

### T03: Run the final proof surface on both dogfood apps

**Goal**
- prove Mesher is formatter-clean
- prove reference-backend stayed formatter-clean and buildable
- prove no dotted-path regression or lingering long single-line imports remain

**Required checks**
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- fmt --check reference-backend`
- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- build reference-backend`
- `! rg -n '^from .{121,}$' mesher -g '*.mpl'`
- `! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`

## Verification

### Current baseline (already green)
- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- build reference-backend`
- `cargo run -q -p meshc -- fmt --check reference-backend`
- `! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`

### Slice closeout gate
- `cargo run -q -p meshc -- fmt mesher`
- `cargo run -q -p meshc -- fmt --check mesher`
- `cargo run -q -p meshc -- fmt --check reference-backend`
- `cargo run -q -p meshc -- build mesher`
- `cargo run -q -p meshc -- build reference-backend`
- `! rg -n '^from .{121,}$' mesher -g '*.mpl'`
- `! rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`

## Risks and Watchouts

1. **Do not trust the roadmap file count.**
   - Live surface is 10 long imports across 8 files, not “20+ files.”
   - Skip broad rediscovery; the worklist is already known.

2. **Do not reopen formatter/compiler work unless a regression is reproduced.**
   - S01 already fixed the risky part.
   - S03 should stay inside Mesher source plus closeout verification.

3. **`fmt --check mesher` is not a useful mid-slice proof after only import edits.**
   - It will stay red until the executor runs the full `meshc fmt mesher` rewrite.
   - Plan for that rewrite explicitly.

4. **Mesher has no internal multiline-import example.**
   - Copy `reference-backend/api/health.mpl` exactly.
   - Avoid inventing a variant style in `main.mpl` or `ingestion/routes.mpl`.

5. **S02’s exact-location `<>` proof is line-number fragile.**
   - `mesher/storage/queries.mpl` and `mesher/storage/schema.mpl` are both in the 35-file formatter set.
   - If later closeout work reuses S02’s `file:line` diff, expect those line numbers to shift after formatter canonicalization even when the accepted `<>` keep sites remain semantically unchanged.

## Skills Discovered

### Loaded
- `debug-like-expert` — used its **VERIFY, DON'T ASSUME** rule to measure the live formatter/build state instead of trusting stale roadmap counts.

### Searched
- `npx skills find "Rust"` surfaced the relevant missing-skill candidates for Rust-based formatter/compiler work, including:
  - `apollographql/skills@rust-best-practices` (4.4K installs)
  - `jeffallan/claude-skills@rust-engineer` (1.5K installs)

### Installed
- `apollographql/skills@rust-best-practices` — installed globally so later Rust-side units inherit it automatically if this milestone unexpectedly has to reopen formatter/compiler code. For S03 as currently scoped, the slice remains Mesher-source work only.