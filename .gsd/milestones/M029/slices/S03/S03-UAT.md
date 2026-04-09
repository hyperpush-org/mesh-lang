# S03: Multiline imports and final formatter compliance — UAT

**Milestone:** M029
**Written:** 2026-03-24

## UAT Type

- UAT mode: artifact-driven
- Why this mode is sufficient: S03 changes source shape and formatter/tooling truth surfaces, not a live runtime contract. The honest acceptance surface is formatter regression coverage, formatter/build cleanliness on both dogfood apps, the import-shape greps, and the preserved multiline-import anchor in `reference-backend/api/health.mpl`.

## Preconditions

- Run from the repo root: `/Users/sn0w/Documents/dev/mesh-lang`
- Rust/Cargo toolchain is available
- `/tmp/m029-s03-fmt-mesher.log` can be created or overwritten
- `mesher/` and `reference-backend/` are not intentionally left in a partially formatted state

## Smoke Test

1. Execute `cargo run -q -p meshc -- fmt --check mesher && cargo run -q -p meshc -- fmt --check reference-backend`
2. **Expected:** both commands exit 0 and emit no formatter diagnostics.

## Test Cases

### 1. Formatter regression suites stay green

1. Execute `cargo test -q -p mesh-fmt --lib`
2. Execute `cargo test -q -p meshc --test e2e_fmt`
3. **Expected:** both test commands exit 0. Any failure indicates a formatter or CLI truth-surface regression rather than an ordinary dogfood-file cleanup miss.

### 2. Mesher has no remaining overlong single-line `from` imports

1. Execute `rg -n '^from .{121,}' mesher -g '*.mpl'`
2. **Expected:** no matches. Any match is a missed multiline-import rollout regression.

### 3. Mesher is formatter-clean and the captured formatter log stays empty

1. Execute `cargo run -q -p meshc -- fmt --check mesher`
2. Execute `cargo run -q -p meshc -- fmt --check mesher > /tmp/m029-s03-fmt-mesher.log 2>&1 && test ! -s /tmp/m029-s03-fmt-mesher.log`
3. **Expected:** both commands exit 0, and `/tmp/m029-s03-fmt-mesher.log` exists but is empty.

### 4. `reference-backend/` stays formatter-clean and preserves the multiline-import anchor

1. Execute `cargo run -q -p meshc -- fmt --check reference-backend`
2. Open `reference-backend/api/health.mpl`.
3. **Expected:** the formatter check exits 0, and the file still uses the parenthesized multiline `from Jobs.Worker import (...)` shape that anchored the Mesher import rewrites.

### 5. Both dogfood apps still build after the final formatter wave

1. Execute `cargo run -q -p meshc -- build mesher`
2. Execute `cargo run -q -p meshc -- build reference-backend`
3. **Expected:** both commands exit 0 and end with their respective `Compiled:` success lines.

### 6. No spaced dotted module paths were introduced across either app

1. Execute `rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'`
2. **Expected:** no matches. Any match is formatter corruption such as `Storage. Queries` or `Api. Router`.

## Edge Cases

### Canonical formatter output still preserves multiline imports after the last wave

1. Open `mesher/main.mpl`, `mesher/ingestion/routes.mpl`, `mesher/api/alerts.mpl`, `mesher/api/dashboard.mpl`, `mesher/api/team.mpl`, `mesher/services/project.mpl`, and `mesher/services/user.mpl`.
2. **Expected:** the parenthesized multiline imports introduced earlier in the slice are still multiline after the final formatter wave.

### Silent-success `fmt --check` is part of the acceptance contract

1. After running the formatter-log check above, inspect `/tmp/m029-s03-fmt-mesher.log`.
2. **Expected:** it is empty on success. If a future run fails, this file should contain the first formatter or parse diagnostic worth inspecting.

### Known accepted formatter aesthetics do not count as slice regressions

1. Open a few formatted Mesher files such as `mesher/api/alerts.mpl` and `mesher/services/project.mpl`.
2. **Expected:** spaces around some generic/result-type syntax and compact `do|state|` separators may still be present. Treat those as accepted canonical output for S03 unless the formatter/build/import-shape gates above fail.

## Failure Signals

- `cargo test -q -p mesh-fmt --lib` exits non-zero
- `cargo test -q -p meshc --test e2e_fmt` exits non-zero
- `cargo run -q -p meshc -- fmt --check mesher` exits non-zero
- `cargo run -q -p meshc -- fmt --check reference-backend` exits non-zero
- `cargo run -q -p meshc -- build mesher` or `cargo run -q -p meshc -- build reference-backend` exits non-zero
- `rg -n '^from .{121,}' mesher -g '*.mpl'` returns any match
- `rg -n '^from .*\. ' mesher reference-backend -g '*.mpl'` returns any match
- `/tmp/m029-s03-fmt-mesher.log` contains output on a supposedly green run
- `reference-backend/api/health.mpl` no longer shows the canonical multiline import anchor used by the slice

## Requirements Proved By This UAT

- R024 — proves Mesher’s remaining long imports were converted to the canonical multiline form and the final Mesher source passes formatter/build gates under canonical output
- R011 — proves late dogfood friction was fixed in Mesh itself rather than papered over in the slice artifacts
- R026 — proves formatter regressions are guarded at both the library and CLI layers during the final closeout path
- R027 — proves dotted-path corruption checks stay green while `reference-backend/` is formatter-clean under the repaired output

## Not Proven By This UAT

- Runtime behavior of Mesher or `reference-backend` under live traffic
- Postgres-backed integration flows, migrations, or worker recovery behavior
- The broader milestone-level `cargo test -p meshc --test e2e` acceptance gate outside S03’s formatter/build/import-shape scope

## Notes for Tester

If a future formatter run emits broken text such as `pubtype` or `table"..."`, restore the damaged source from a pre-format copy before rerunning the repaired formatter. Once the first broken pass has truncated the parsed declaration body, a second format pass cannot reconstruct it.
