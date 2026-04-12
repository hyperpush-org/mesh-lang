# Codebase Map

Generated: 2026-04-12T01:16:07Z | Files: 500 | Described: 0/500
<!-- gsd:codebase-meta {"generatedAt":"2026-04-12T01:16:07Z","fingerprint":"b245b9d28c0ec1744c62d9d56f39de0a920f0a10","fileCount":500,"truncated":true} -->
Note: Truncated to first 500 files. Run with higher --max-files to include all.

### (root)/
- `.dockerignore`
- `.gitignore`
- `AGENTS.md`
- `Cargo.toml`
- `CODE_OF_CONDUCT.md`
- `CONTRIBUTING.md`
- `LICENSE`
- `README.md`
- `SECURITY.md`
- `SUPPORT.md`
- `WORKSPACE.md`

### .cargo/
- `.cargo/config.toml`

### .githooks/
- `.githooks/pre-commit`
- `.githooks/pre-push`

### .github/
- `.github/CODEOWNERS`
- `.github/dependabot.yml`
- `.github/pull_request_template.md`

### .github/ISSUE_TEMPLATE/
- `.github/ISSUE_TEMPLATE/bug_report.yml`
- `.github/ISSUE_TEMPLATE/config.yml`
- `.github/ISSUE_TEMPLATE/documentation.yml`
- `.github/ISSUE_TEMPLATE/feature_request.yml`

### .github/workflows/
- `.github/workflows/authoritative-live-proof.yml`
- `.github/workflows/authoritative-starter-failover-proof.yml`
- `.github/workflows/authoritative-verification.yml`
- `.github/workflows/compatibility-matrix.yml`
- `.github/workflows/deploy-services.yml`
- `.github/workflows/deploy.yml`
- `.github/workflows/extension-release-proof.yml`
- `.github/workflows/publish-extension.yml`
- `.github/workflows/release.yml`

### articles/
- `articles/ARTICLE_1.md`
- `articles/ARTICLE_2.md`
- `articles/ARTICLE_3.md`

### benchmarks/
- `benchmarks/METHODOLOGY.md`
- `benchmarks/README.md`
- `benchmarks/RESULTS.md`
- `benchmarks/run_benchmarks.sh`

### benchmarks/elixir/
- `benchmarks/elixir/.formatter.exs`
- `benchmarks/elixir/mix.exs`

### benchmarks/elixir/lib/
- `benchmarks/elixir/lib/bench.ex`

### benchmarks/fly/
- `benchmarks/fly/Dockerfile.loadgen`
- `benchmarks/fly/Dockerfile.servers`
- `benchmarks/fly/README.md`
- `benchmarks/fly/run-benchmarks-isolated.sh`
- `benchmarks/fly/run-benchmarks.sh`
- `benchmarks/fly/start-server-isolated.sh`
- `benchmarks/fly/start-servers.sh`

### benchmarks/go/
- `benchmarks/go/go.mod`
- `benchmarks/go/main.go`

### benchmarks/mesh/
- `benchmarks/mesh/main.mpl`

### benchmarks/rust/
- `benchmarks/rust/Cargo.toml`
- `benchmarks/rust/main.rs`

### compiler/mesh-codegen/
- `compiler/mesh-codegen/Cargo.toml`

### compiler/mesh-codegen/src/
- `compiler/mesh-codegen/src/declared.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/mesh-codegen/src/link.rs`

### compiler/mesh-codegen/src/codegen/
- `compiler/mesh-codegen/src/codegen/expr.rs`
- `compiler/mesh-codegen/src/codegen/intrinsics.rs`
- `compiler/mesh-codegen/src/codegen/mod.rs`
- `compiler/mesh-codegen/src/codegen/pattern.rs`
- `compiler/mesh-codegen/src/codegen/types.rs`

### compiler/mesh-codegen/src/mir/
- `compiler/mesh-codegen/src/mir/lower.rs`
- `compiler/mesh-codegen/src/mir/mod.rs`
- `compiler/mesh-codegen/src/mir/mono.rs`
- `compiler/mesh-codegen/src/mir/types.rs`

### compiler/mesh-codegen/src/pattern/
- `compiler/mesh-codegen/src/pattern/compile.rs`
- `compiler/mesh-codegen/src/pattern/mod.rs`

### compiler/mesh-common/
- `compiler/mesh-common/Cargo.toml`

### compiler/mesh-common/src/
- `compiler/mesh-common/src/error.rs`
- `compiler/mesh-common/src/lib.rs`
- `compiler/mesh-common/src/module_graph.rs`
- `compiler/mesh-common/src/span.rs`
- `compiler/mesh-common/src/token.rs`

### compiler/mesh-fmt/
- `compiler/mesh-fmt/Cargo.toml`

### compiler/mesh-fmt/src/
- `compiler/mesh-fmt/src/ir.rs`
- `compiler/mesh-fmt/src/lib.rs`
- `compiler/mesh-fmt/src/printer.rs`
- `compiler/mesh-fmt/src/walker.rs`

### compiler/mesh-lexer/
- `compiler/mesh-lexer/Cargo.toml`

### compiler/mesh-lexer/src/
- `compiler/mesh-lexer/src/cursor.rs`
- `compiler/mesh-lexer/src/lib.rs`

### compiler/mesh-lexer/tests/
- `compiler/mesh-lexer/tests/lexer_tests.rs`

### compiler/mesh-lexer/tests/snapshots/
- *(36 files: 36 .snap)*

### compiler/mesh-lsp/
- `compiler/mesh-lsp/Cargo.toml`

### compiler/mesh-lsp/src/
- `compiler/mesh-lsp/src/analysis.rs`
- `compiler/mesh-lsp/src/completion.rs`
- `compiler/mesh-lsp/src/definition.rs`
- `compiler/mesh-lsp/src/lib.rs`
- `compiler/mesh-lsp/src/server.rs`
- `compiler/mesh-lsp/src/signature_help.rs`

### compiler/mesh-parser/
- `compiler/mesh-parser/Cargo.toml`

### compiler/mesh-parser/src/
- `compiler/mesh-parser/src/cst.rs`
- `compiler/mesh-parser/src/error.rs`
- `compiler/mesh-parser/src/lib.rs`
- `compiler/mesh-parser/src/syntax_kind.rs`

### compiler/mesh-parser/src/ast/
- `compiler/mesh-parser/src/ast/expr.rs`
- `compiler/mesh-parser/src/ast/item.rs`
- `compiler/mesh-parser/src/ast/mod.rs`
- `compiler/mesh-parser/src/ast/pat.rs`

### compiler/mesh-parser/src/parser/
- `compiler/mesh-parser/src/parser/expressions.rs`
- `compiler/mesh-parser/src/parser/items.rs`
- `compiler/mesh-parser/src/parser/mod.rs`
- `compiler/mesh-parser/src/parser/patterns.rs`

### compiler/mesh-parser/tests/
- `compiler/mesh-parser/tests/parser_tests.rs`

### compiler/mesh-parser/tests/snapshots/
- *(176 files: 176 .snap)*

### compiler/mesh-pkg/
- `compiler/mesh-pkg/Cargo.toml`

### compiler/mesh-pkg/src/
- `compiler/mesh-pkg/src/lib.rs`
- `compiler/mesh-pkg/src/lockfile.rs`
- `compiler/mesh-pkg/src/manifest.rs`
- `compiler/mesh-pkg/src/resolver.rs`
- `compiler/mesh-pkg/src/scaffold.rs`
- `compiler/mesh-pkg/src/toolchain_update.rs`

### compiler/mesh-pkg/tests/
- `compiler/mesh-pkg/tests/toolchain_update.rs`

### compiler/mesh-repl/
- `compiler/mesh-repl/Cargo.toml`

### compiler/mesh-repl/src/
- `compiler/mesh-repl/src/jit.rs`
- `compiler/mesh-repl/src/lib.rs`
- `compiler/mesh-repl/src/session.rs`

### compiler/mesh-rt/
- `compiler/mesh-rt/Cargo.toml`

### compiler/mesh-rt/src/
- `compiler/mesh-rt/src/crypto.rs`
- `compiler/mesh-rt/src/datetime.rs`
- `compiler/mesh-rt/src/env.rs`
- `compiler/mesh-rt/src/file.rs`
- `compiler/mesh-rt/src/gc.rs`
- `compiler/mesh-rt/src/hash.rs`
- `compiler/mesh-rt/src/io.rs`
- `compiler/mesh-rt/src/iter.rs`
- `compiler/mesh-rt/src/json.rs`
- `compiler/mesh-rt/src/lib.rs`
- `compiler/mesh-rt/src/option.rs`
- `compiler/mesh-rt/src/panic.rs`
- `compiler/mesh-rt/src/regex.rs`
- `compiler/mesh-rt/src/string.rs`
- `compiler/mesh-rt/src/test.rs`

### compiler/mesh-rt/src/actor/
- `compiler/mesh-rt/src/actor/child_spec.rs`
- `compiler/mesh-rt/src/actor/heap.rs`
- `compiler/mesh-rt/src/actor/job.rs`
- `compiler/mesh-rt/src/actor/link.rs`
- `compiler/mesh-rt/src/actor/mailbox.rs`
- `compiler/mesh-rt/src/actor/mod.rs`
- `compiler/mesh-rt/src/actor/process.rs`
- `compiler/mesh-rt/src/actor/registry.rs`
- `compiler/mesh-rt/src/actor/scheduler.rs`
- `compiler/mesh-rt/src/actor/service.rs`
- `compiler/mesh-rt/src/actor/stack.rs`
- `compiler/mesh-rt/src/actor/supervisor.rs`

### compiler/mesh-rt/src/collections/
- `compiler/mesh-rt/src/collections/list.rs`
- `compiler/mesh-rt/src/collections/map.rs`
- `compiler/mesh-rt/src/collections/mod.rs`
- `compiler/mesh-rt/src/collections/queue.rs`
- `compiler/mesh-rt/src/collections/range.rs`
- `compiler/mesh-rt/src/collections/set.rs`
- `compiler/mesh-rt/src/collections/tuple.rs`

### compiler/mesh-rt/src/db/
- `compiler/mesh-rt/src/db/changeset.rs`
- `compiler/mesh-rt/src/db/expr.rs`
- `compiler/mesh-rt/src/db/json.rs`
- `compiler/mesh-rt/src/db/migration.rs`
- `compiler/mesh-rt/src/db/mod.rs`
- `compiler/mesh-rt/src/db/orm.rs`
- `compiler/mesh-rt/src/db/pg_schema.rs`
- `compiler/mesh-rt/src/db/pg.rs`
- `compiler/mesh-rt/src/db/pool.rs`
- `compiler/mesh-rt/src/db/query.rs`
- `compiler/mesh-rt/src/db/repo.rs`
- `compiler/mesh-rt/src/db/row.rs`
- `compiler/mesh-rt/src/db/sqlite.rs`

### compiler/mesh-rt/src/http/
- `compiler/mesh-rt/src/http/client.rs`
- `compiler/mesh-rt/src/http/mod.rs`
- `compiler/mesh-rt/src/http/router.rs`
- `compiler/mesh-rt/src/http/server.rs`

### compiler/mesh-rt/src/ws/
- `compiler/mesh-rt/src/ws/close.rs`
- `compiler/mesh-rt/src/ws/frame.rs`
- `compiler/mesh-rt/src/ws/handshake.rs`
- `compiler/mesh-rt/src/ws/mod.rs`
- `compiler/mesh-rt/src/ws/rooms.rs`
- `compiler/mesh-rt/src/ws/server.rs`

### compiler/mesh-typeck/
- `compiler/mesh-typeck/Cargo.toml`

### compiler/mesh-typeck/src/
- `compiler/mesh-typeck/src/builtins.rs`
- `compiler/mesh-typeck/src/diagnostics.rs`
- `compiler/mesh-typeck/src/env.rs`
- `compiler/mesh-typeck/src/error.rs`
- `compiler/mesh-typeck/src/exhaustiveness.rs`
- `compiler/mesh-typeck/src/infer.rs`
- `compiler/mesh-typeck/src/lib.rs`
- `compiler/mesh-typeck/src/traits.rs`
- `compiler/mesh-typeck/src/ty.rs`
- `compiler/mesh-typeck/src/unify.rs`

### compiler/mesh-typeck/tests/
- `compiler/mesh-typeck/tests/actors.rs`
- `compiler/mesh-typeck/tests/assoc_types.rs`
- `compiler/mesh-typeck/tests/diagnostics.rs`
- `compiler/mesh-typeck/tests/exhaustiveness_integration.rs`
- `compiler/mesh-typeck/tests/http_clustered_routes.rs`
- `compiler/mesh-typeck/tests/inference.rs`
- `compiler/mesh-typeck/tests/integration.rs`
- `compiler/mesh-typeck/tests/structs.rs`
- `compiler/mesh-typeck/tests/sum_types.rs`
- `compiler/mesh-typeck/tests/supervisors.rs`
- `compiler/mesh-typeck/tests/traits.rs`

### compiler/mesh-typeck/tests/snapshots/
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_ambiguous_method_deterministic_order.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_ambiguous_method_help_text.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_arity_mismatch.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_if_branch_mismatch.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_invalid_guard_expression.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_missing_field.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_non_exhaustive_match.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_not_a_function.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_receive_outside_actor.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_redundant_arm.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_self_outside_actor.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_send_type_mismatch.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_spawn_non_function.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_trait_not_satisfied.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_type_mismatch.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_unbound_variable.snap`
- `compiler/mesh-typeck/tests/snapshots/diagnostics__diag_unknown_field.snap`

### compiler/meshc/
- `compiler/meshc/Cargo.toml`

### compiler/meshc/src/
- `compiler/meshc/src/cluster.rs`
- `compiler/meshc/src/discovery.rs`
- `compiler/meshc/src/main.rs`
- `compiler/meshc/src/migrate.rs`
- `compiler/meshc/src/test_runner.rs`

### compiler/meshc/tests/
- *(66 files: 66 .rs)*

### compiler/meshc/tests/support/
- `compiler/meshc/tests/support/m046_route_free.rs`
- `compiler/meshc/tests/support/m047_todo_scaffold.rs`
- `compiler/meshc/tests/support/m049_todo_examples.rs`
- `compiler/meshc/tests/support/m049_todo_postgres_scaffold.rs`
