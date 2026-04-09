---
estimated_steps: 4
estimated_files: 9
skills_used:
  - lint
  - debug-like-expert
---

# T04: Canonicalize Mesher ingestion and storage modules with the fixed formatter

**Slice:** S03 — Multiline imports and final formatter compliance
**Milestone:** M029

## Description

Run the second bounded formatter wave across Mesher's ingestion and storage modules. This task carries the rewritten `mesher/ingestion/routes.mpl` import through the fixed formatter, accepts the known mechanical canonicalization in the surrounding files, and stops if the formatter starts reintroducing the S01 dotted-path or multiline-import bugs.

## Steps

1. Read the six `mesher/ingestion/*.mpl` files and three `mesher/storage/*.mpl` files so the canonicalization pass is grounded in the actual source being rewritten.
2. Run `cargo run -q -p meshc -- fmt mesher/ingestion` and `cargo run -q -p meshc -- fmt mesher/storage` to move this nine-file group onto canonical formatter output.
3. Inspect the resulting diffs, paying special attention to `mesher/ingestion/routes.mpl` and any dotted module paths inside ingestion/storage imports.
4. Verify the group is formatter-clean and that no long single-line or spaced dotted imports reappeared in these files.

## Must-Haves

- [ ] All six `mesher/ingestion/*.mpl` files and all three `mesher/storage/*.mpl` files are on canonical formatter output.
- [ ] `mesher/ingestion/routes.mpl` keeps its parenthesized multiline import after formatting.
- [ ] Any dotted-path or multiline-import regression is treated as a blocker rather than worked around in dogfood source.

## Verification

- `cargo run -q -p meshc -- fmt mesher/ingestion && cargo run -q -p meshc -- fmt mesher/storage && cargo run -q -p meshc -- fmt --check mesher/ingestion && cargo run -q -p meshc -- fmt --check mesher/storage`
- `! rg -n '^from .{121,}' mesher/ingestion/routes.mpl && ! rg -n '^from .*\. ' mesher/ingestion mesher/storage -g '*.mpl'`

## Inputs

- `mesher/ingestion/auth.mpl` — formatter-red ingestion file
- `mesher/ingestion/fingerprint.mpl` — formatter-red ingestion file
- `mesher/ingestion/pipeline.mpl` — formatter-red ingestion file
- `mesher/ingestion/routes.mpl` — T01 multiline import plus formatter-red ingestion file
- `mesher/ingestion/validation.mpl` — formatter-red ingestion file
- `mesher/ingestion/ws_handler.mpl` — formatter-red ingestion file
- `mesher/storage/queries.mpl` — formatter-red storage file
- `mesher/storage/schema.mpl` — formatter-red storage file
- `mesher/storage/writer.mpl` — formatter-red storage file

## Expected Output

- `mesher/ingestion/auth.mpl` — canonical formatter output for the ingestion module
- `mesher/ingestion/fingerprint.mpl` — canonical formatter output for the ingestion module
- `mesher/ingestion/pipeline.mpl` — canonical formatter output for the ingestion module
- `mesher/ingestion/routes.mpl` — canonical formatter output with multiline import preserved
- `mesher/ingestion/validation.mpl` — canonical formatter output for the ingestion module
- `mesher/ingestion/ws_handler.mpl` — canonical formatter output for the ingestion module
- `mesher/storage/queries.mpl` — canonical formatter output for the storage module
- `mesher/storage/schema.mpl` — canonical formatter output for the storage module
- `mesher/storage/writer.mpl` — canonical formatter output for the storage module

## Observability Impact

- Runtime signals changed: none. This task is limited to Mesher source-shape canonicalization.
- How to inspect later: rerun the scoped formatter round-trip on `mesher/ingestion` and `mesher/storage`, then use the targeted greps on `mesher/ingestion/routes.mpl` and the two directories to confirm the multiline import and dotted module paths stayed clean.
- Failure state made visible: `cargo run -q -p meshc -- fmt --check mesher/ingestion`, `cargo run -q -p meshc -- fmt --check mesher/storage`, `! rg -n '^from .{121,}' mesher/ingestion/routes.mpl`, and `! rg -n '^from .*\. ' mesher/ingestion mesher/storage -g '*.mpl'` distinguish a task-local formatter regression from downstream Mesher backlog outside this wave.
