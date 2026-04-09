# Project Research Summary

**Project:** Mesh v14.0 — Ecosystem Expansion
**Domain:** Programming language ecosystem — stdlib crypto/datetime/encoding, HTTP client, testing framework, package registry
**Researched:** 2026-02-28
**Confidence:** HIGH

## Executive Summary

Mesh v14.0 is an ecosystem expansion milestone for an existing compiled programming language. The work spans six parallel domains: crypto/encoding stdlib, datetime stdlib, HTTP client improvements, a testing framework, a package manifest format, and a hosted package registry. The research finding that most shapes execution is that nearly all crypto and encoding work requires zero new Rust dependencies — `sha2`, `hmac`, `base64`, and `rand` are already compiled into `mesh-rt`. The only new runtime dependency is `chrono 0.4` for datetime, plus a `ureq 2 -> 3` upgrade for HTTP streaming. This dramatically de-risks the stdlib work: it is primarily wrapper code following an established three-file pattern (runtime impl + typechecker registration + LLVM extern declaration).

The recommended approach is to build in dependency order: encoding and crypto first (zero new deps, validates the three-file pattern), then datetime (one new dep, validates the i64 timestamp design), then HTTP client improvements (isolated to one file), then the test runner (requires assertion helpers in place), and finally the package registry (most complex, but independent of all compiler work). The registry backend (Axum 0.8 + PostgreSQL + sqlx) and the CLI (`meshpkg`) can be developed in parallel with compiler changes once the `mesh.toml` manifest format is finalized. The registry website extends the existing VitePress site — no new framework or deployment target.

The top risk is not dependency complexity but design decisions that cannot be retrofitted: timestamp representation (must be `i64` Unix milliseconds, not strings), constant-time HMAC comparison (must use `hmac::Mac::verify_slice`, not `==`), test actor isolation (must be architected before assertions), and registry immutability (publish-once, no overwrite). Each of these is inexpensive to get right and very expensive to fix after users depend on the wrong behavior. A secondary risk is LLVM coverage instrumentation, which is incompatible with Mesh's current codegen and should use source-level MIR counter injection instead.

## Key Findings

### Recommended Stack

The overwhelming majority of v14.0 work builds on the existing dependency graph. `sha2 0.10`, `hmac 0.12`, `base64 0.22`, and `rand 0.9` are already locked in `mesh-rt/Cargo.toml`. The only new additions to `mesh-rt` are `chrono 0.4` (datetime) and upgrading `ureq` from `"2"` to `"3"` (HTTP streaming and keep-alive). The registry backend is a new workspace member (`mesh-registry`) using Axum 0.8, sqlx 0.8, and tokio 1 — all of which align with existing workspace dependencies.

**Core technologies:**
- `chrono 0.4` (mesh-rt): DateTime parsing, formatting, and arithmetic — only new runtime dep; 392M downloads, multi-thread safe since 0.4.20
- `ureq 3.2` (mesh-rt upgrade): Streaming via `Body::into_reader()`, connection pooling via `Agent`, `Body: Send` guarantee needed for actor model
- `axum 0.8` (mesh-registry): Registry HTTP API — tokio-rs maintained, Tower middleware, same tokio dep already in workspace
- `sqlx 0.8` (mesh-registry): Async PostgreSQL for package metadata — compile-time checked queries, matches axum/tokio stack
- `uuid 1.21` (mesh-rt): UUID v4 using `rand 0.9` already present; only new crate added for crypto module
- `tar 0.4` + `flate2 1` (mesh-pkg, mesh-registry): Package tarball creation and extraction

**What not to add:** `hex` crate (3 lines inline), `chrono-tz` (~2MB bloat, not needed for v14.0), `reqwest` (async-only, conflicts with synchronous actor model), `diesel` (synchronous, incompatible with axum async handlers).

### Expected Features

**Must have (table stakes — P1, blocks v14.0):**
- `Crypto.sha256/sha512/hmac_sha256/hmac_sha512/secure_compare/uuid4` — API authentication, content addressing
- `Base64.encode/decode/encode_url/decode_url` and `Hex.encode/decode` — wire format, JWT tokens
- `DateTime.utc_now/from_iso8601/to_iso8601/from_unix/to_unix/add/diff/before?/after?` — timestamps for every web application
- `Http.build/header/body/timeout/send` builder API — composable HTTP client
- `meshc test` runner with `assert/assert_eq/assert_ne/assert_raises` — no testing framework means no confidence in code
- `mesh.toml` manifest and `mesh.lock` lockfile — reproducible builds
- `meshpkg publish/install/search` CLI and hosted registry site with browse/search/per-package pages

**Should have (competitive differentiators — P2):**
- `Http.stream` callback-based streaming and `Http.client()` keep-alive handle
- `describe "..." do ... end` grouping, `setup/teardown` blocks, `assert_receive`, `Test.mock_actor`
- `meshc test --jobs N` parallel test modules

**Defer to v14.1/v2+:**
- `meshc test --coverage` (HIGH implementation risk — LLVM incompatibility with current codegen)
- `DateTime.format` with strftime patterns, timezone-aware datetime
- `Crypto.pbkdf2`, Ed25519/RSA signing
- `meshpkg outdated`, private package namespaces

### Architecture Approach

Every new stdlib function follows the established three-file pattern: `mesh-rt/src/<module>.rs` (Rust `extern "C"` implementation), `mesh-typeck/src/builtins.rs` (type signature registration), `mesh-codegen/src/codegen/intrinsics.rs` (LLVM extern declaration). Stateful resources (HTTP keep-alive agent, streaming reader) use the opaque `u64` handle pattern established for DB connections and regex handles. The test runner is a new module within the `meshc` binary crate (following the `migrate.rs` precedent), not a separate library. The package registry is a separate `mesh-registry` workspace member — not part of the compiler.

**Major components:**
1. `mesh-rt/src/crypto.rs`, `date.rs`, `encoding.rs`, `test_support.rs` (NEW) — stdlib runtime implementations as `extern "C"` functions
2. `meshc/src/test_runner.rs` (NEW) — `*.test.mpl` discovery, compile, execute, aggregate pass/fail
3. `compiler/meshpkg/` (NEW binary crate) — publish, install, search, login CLI separate from `meshc`
4. `registry/` (NEW workspace member) — Axum + PostgreSQL HTTP API + tarball storage with SHA-256 content addressing
5. `website/docs/packages/` (MODIFIED) — Vue components fetching registry API at runtime; extends existing VitePress site

**Key architectural decisions from research:**
- DateTime is `i64` Unix milliseconds, not an opaque heap handle — avoids new type machinery in typeck/codegen
- HTTP streaming uses a dedicated OS thread per stream (WS reader pattern from v4.0), NOT blocking inside actor coroutines
- Each `*.test.mpl` is a complete Mesh program; the runner compiles and executes each independently (no function-level test injection)
- Registry package versions are immutable from day one; yank marks versions deprecated without deleting content
- Exact versions only in `mesh.toml` (`"1.2.0"` not `"^1.0"`) — SemVer range solving is deferred

### Critical Pitfalls

1. **Blocking HTTP I/O starving actor scheduler threads** — `ureq` streaming reads block OS threads; with 8 threads and 8 concurrent streaming actors the scheduler deadlocks. Prevention: spawn a dedicated OS thread per stream (WS reader pattern), deliver chunks to actor mailbox as messages.

2. **Variable-time HMAC comparison** — using `==` on HMAC outputs enables timing attacks on API tokens. Prevention: expose `Crypto.secure_compare` backed by `hmac::Mac::verify_slice` (constant-time via `subtle`); document that `==` must never be used for secret comparison in production.

3. **Test actor registry leaks between tests** — leftover named actors from test A cause "AlreadyRegistered" failures in test B. Prevention: each test function runs as a separate root actor; all linked mock actors die when the test actor exits via existing supervisor infrastructure.

4. **Registry version overwrite** — allowing `meshpkg publish` to overwrite an existing version breaks reproducible builds permanently and cannot be undone without trust damage. Prevention: content-address tarballs by SHA-256; reject re-upload of same version with different content; return HTTP 409 Conflict on duplicate publish.

5. **LLVM coverage incompatible with Mesh codegen** — Mesh emits LLVM IR without DWARF debug info; `llvm-profdata` produces empty reports or maps coverage to Rust compiler source. Prevention: implement coverage as source-level MIR counter injection dumped to JSON; defer LLVM-based coverage until codegen emits proper debug info.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Encoding and Crypto Stdlib
**Rationale:** Zero new Rust dependencies; validates the three-file pattern for all subsequent stdlib work; fastest to ship; crypto primitives (HMAC, UUID) are prerequisites for registry authentication.
**Delivers:** `Crypto.*` (sha256/sha512/hmac/uuid4/secure_compare), `Base64.*`, `Hex.*` — all as `extern "C"` wrappers over already-compiled crates.
**Addresses:** All P1 crypto and encoding features from FEATURES.md.
**Avoids:** Duplicate dep pitfall (audit `Cargo.toml` first); non-constant-time comparison pitfall (design `secure_compare` before any HMAC function); UUID from weak PRNG pitfall (use `ring::rand::SystemRandom`).

### Phase 2: DateTime Stdlib
**Rationale:** One new dependency (`chrono 0.4`); independent of all other v14.0 work; the `i64` Unix milliseconds representation decision must be locked before registry or test runner touches timestamps.
**Delivers:** `DateTime.*` — utc_now, from_iso8601, to_iso8601, from_unix, to_unix, add, diff, before?, after?.
**Uses:** `chrono 0.4` added to `mesh-rt/Cargo.toml`.
**Avoids:** String-based timestamp pitfall; silent UTC assumption pitfall (reject timezone-free strings with Err); integer overflow pitfall (use checked arithmetic, return Result from all arithmetic functions).

### Phase 3: HTTP Client Improvements
**Rationale:** Isolated to `mesh-rt/src/http/client.rs`; ureq 3 upgrade confined to one file; threading model for streaming must be decided before any streaming implementation begins.
**Delivers:** `Http.build/header/body/timeout/send` builder API; `Http.stream` (dedicated OS thread per stream); `Http.client()` keep-alive agent handle.
**Uses:** `ureq 3.2` upgrade; opaque `u64` handle pattern for Agent.
**Avoids:** Actor scheduler starvation pitfall (OS thread per stream, not blocking in coroutine); keep-alive pool on GC heap pitfall (`Box::into_raw` opaque handle); chunked parser edge cases (RFC 9112 strict: extensions, trailers, zero-chunk terminator).

### Phase 4: Testing Framework
**Rationale:** `meshc test` runner is the prerequisite for all testing features; test isolation architecture must be designed first; assertion helpers must exist in `mesh-rt` before the runner can compile and execute test files.
**Delivers:** `meshc test` discovery and runner; `assert/assert_eq/assert_ne/assert_raises`; `describe` blocks; `setup/teardown`; `assert_receive`; `Test.mock_actor`. Coverage treated as stretch goal.
**Addresses:** All P1 and P2 testing features.
**Avoids:** Test actor registry leak pitfall (actor-per-test isolation); mock actor orphan pitfall (link mocks to test actor for automatic cleanup on exit); LLVM coverage pitfall (use MIR counter injection if coverage is implemented at all in v14.0).

### Phase 5: Package Manifest and meshpkg CLI
**Rationale:** `mesh.toml` manifest format must be finalized before registry API contract can be defined; `meshpkg` CLI depends on mesh-pkg's Registry dep variant; exact-version-only policy avoids SemVer solver complexity.
**Delivers:** `mesh.toml` manifest with `Dependency::Registry { version }` variant; `mesh.lock` lockfile; `meshpkg publish/install/search/login` CLI binary as new `compiler/meshpkg/` crate.
**Uses:** `tar 0.4`, `flate2 1`, `sha2 0.10` added to mesh-pkg.
**Avoids:** SemVer range solver scope creep (exact versions only in v14.0).

### Phase 6: Package Registry Backend and Website
**Rationale:** Can be developed in parallel with Phase 5 once API contract is defined; registry server is independent of compiler changes; must ship with pre-published stdlib packages to avoid "ghost town" problem at launch.
**Delivers:** `mesh-registry` Axum server (publish/download/search/auth API); PostgreSQL schema with SHA-256 content addressing; tarball storage with `StorageBackend` trait; VitePress package browse/search/detail pages; at least 4 stdlib packages published at launch.
**Uses:** `axum 0.8`, `sqlx 0.8`, `tokio 1`, `uuid 1`, `chrono 0.4` in new `mesh-registry` workspace member.
**Avoids:** Registry version overwrite pitfall (immutable publish, HTTP 409 on duplicate); empty registry at launch (publish stdlib packages as first content); registry SQL full-table-scan (PostgreSQL FTS `tsvector` index from day one).

### Phase Ordering Rationale

- Phases 1-2 (stdlib) have zero external dependencies and validate the three-file pattern used by all later stdlib additions.
- Phase 3 (HTTP) is independent but benefits from the pattern being proven; ureq upgrade is confined to one file.
- Phase 4 (testing) requires assertion helpers in place but is otherwise independent of all other phases.
- Phase 5 (manifest + CLI) must precede Phase 6 (registry server) because the API contract flows from the manifest format.
- Phases 5 and 6 are separable: the registry server can be developed in parallel with the CLI once the API contract is defined on paper.
- The build order from ARCHITECTURE.md (encoding -> crypto -> date -> HTTP -> test assertions -> test runner -> manifest -> meshpkg -> registry -> website) validates this phase structure.
- All of Phases 1-4 are independent of Phases 5-6 and can run in parallel across teams.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3 (HTTP streaming):** The WS reader thread pattern is documented in PROJECT.md but the exact actor mailbox message format for HTTP chunks and backpressure model need a design spike before implementation begins.
- **Phase 4 (coverage):** MIR-level counter injection is the recommended approach but has no prior art in the Mesh codebase; needs a prototype before committing to the full feature in v14.0. Strong recommendation: defer coverage to v14.1.
- **Phase 6 (registry):** Tarball storage abstraction (`StorageBackend` trait for future S3/R2 migration), PostgreSQL full-text search configuration, and API auth token lifecycle all need design docs before coding starts.

Phases with well-documented patterns (skip research-phase):
- **Phase 1 (crypto/encoding):** Three-file pattern is fully established; existing `mesh_http_get` and `mesh_regex_compile` are direct implementation templates.
- **Phase 2 (datetime):** chrono API is mature and well-documented; i64 millisecond representation is a settled design decision from research.
- **Phase 5 (manifest):** `mesh-pkg` crate already has manifest parsing; adding `Dependency::Registry` variant is a small, well-understood change.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All critical crates verified against docs.rs and Cargo.lock; existing dep reuse confirmed by direct file inspection of mesh-rt/Cargo.toml |
| Features | HIGH | Based on ExUnit, Hex, Cargo, and Python stdlib conventions; all features mapped to concrete Mesh API signatures with complexity estimates |
| Architecture | HIGH | Based on direct codebase inspection of mesh-rt, mesh-typeck, mesh-codegen, mesh-pkg, meshc; three-file pattern and opaque handle pattern verified against multiple existing examples |
| Pitfalls | HIGH | Mix of direct source analysis (scheduler design, existing ureq usage) and verified external CVEs/RFCs (chunked transfer CVE-2025-66373, LLVM coverage format incompatibility) |

**Overall confidence:** HIGH

### Gaps to Address

- **HTTP streaming backpressure:** OS thread + mailbox model is the right pattern, but the message format for chunk delivery and EOF signaling needs a concrete design decision during Phase 3 planning. No gap in approach — gap in specifics.
- **Coverage deferral decision:** Research strongly suggests MIR counter injection over LLVM instrumentation, but the scope for v14.0 vs v14.1 should be confirmed at planning time. If test runner takes longer than expected, coverage is the correct cut.
- **Registry storage abstraction:** Starting with local filesystem is correct, but the `StorageBackend` trait design (interface for future S3/R2 migration) needs a concrete API before Phase 6 coding starts.
- **`meshpkg login` credential storage:** `~/.mesh/credentials` format and token rotation semantics are not fully specified in research. Low risk but needs a design decision during Phase 5 planning.

## Sources

### Primary (HIGH confidence)
- `compiler/mesh-rt/Cargo.toml` — confirmed sha2/hmac/base64/rand/ureq already present; zero new crypto deps needed
- `compiler/mesh-rt/src/http/client.rs` — confirmed ureq 2.x blocking I/O, current get/post flat functions
- `compiler/mesh-pkg/src/manifest.rs` + `resolver.rs` — existing mesh.toml format, DFS resolver, Dependency enum
- `compiler/meshc/src/main.rs` + `migrate.rs` — subcommand-as-module pattern (test runner template)
- `compiler/mesh-codegen/src/codegen/intrinsics.rs` — LLVM extern declaration pattern
- `compiler/mesh-typeck/src/builtins.rs` — type registration pattern for stdlib functions
- [docs.rs/ureq/latest](https://docs.rs/ureq/latest/ureq/) — ureq 3.2 Agent pooling, Body streaming, Body: Send
- [docs.rs/chrono/latest](https://docs.rs/chrono/latest/chrono/) — DateTime<Utc>, parse_from_rfc3339, to_rfc3339, timestamp
- [tokio.rs/blog/2025-01-01-announcing-axum-0-8-0](https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0) — axum 0.8.8 stable
- [hexdocs.pm/ex_unit/ExUnit.html](https://hexdocs.pm/ex_unit/ExUnit.html) — file convention, runner, assert_receive API
- [RFC 9112 §7.1](https://www.rfc-editor.org/rfc/rfc9112#section-7.1) — chunked transfer coding spec (chunk extensions, trailers)
- [doc.rust-lang.org/cargo/reference/publishing.html](https://doc.rust-lang.org/cargo/reference/publishing.html) — immutability and yank design rationale

### Secondary (MEDIUM confidence)
- [github.com/rust-lang/crates.io](https://github.com/rust-lang/crates.io) — uses axum backend; permanent archive design philosophy
- [dalek-cryptography/subtle](https://github.com/dalek-cryptography/subtle) — LLVM branch re-introduction risk in constant-time code
- [CVE-2025-66373 Akamai](https://www.akamai.com/blog/security/cve-2025-66373-http-request-smuggling-chunked-body-size) — real-world chunked parser failure (2025)
- [LLVM source-based coverage docs](https://clang.llvm.org/docs/SourceBasedCodeCoverage.html) — format version incompatibility warning

---
*Research completed: 2026-02-28*
*Ready for roadmap: yes*

# Architecture Research

**Domain:** Programming language ecosystem — stdlib, HTTP client, test runner, package registry
**Researched:** 2026-02-28
**Confidence:** HIGH (based on direct codebase inspection + verified external sources)

## Standard Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          Mesh Compiler Pipeline                          │
│  mesh-lexer → mesh-parser → mesh-typeck → mesh-codegen(MIR) → LLVM IR  │
│                                                                          │
│  builtins.rs           intrinsics.rs          mesh-rt (libmesh_rt.a)    │
│  [type sigs]    →      [LLVM decls]    →      [extern "C" impls]        │
└─────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│                          mesh-rt Crate Layout                            │
│                                                                          │
│  actor/     string.rs    gc.rs     http/          db/         v14 NEW    │
│  ┌───────┐  ┌────────┐  ┌──────┐  ┌──────────┐  ┌───────┐  ┌────────┐  │
│  │sched  │  │string  │  │mark- │  │server.rs │  │pg.rs  │  │crypto  │  │
│  │pcb    │  │concat  │  │sweep │  │client.rs │  │pool.rs│  │date    │  │
│  │coros. │  │new     │  │alloc │  │router.rs │  │orm.rs │  │encode  │  │
│  └───────┘  └────────┘  └──────┘  └──────────┘  └───────┘  └────────┘  │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│                       meshc CLI (v14.0 additions)                        │
│                                                                          │
│  build   fmt   repl   lsp   migrate   deps   [NEW: test]                │
│                                                                          │
│  Dispatches to: mesh-codegen, mesh-fmt, mesh-repl, mesh-lsp,            │
│                 mesh-pkg, [NEW: test_runner.rs module in meshc]          │
└──────────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│              meshpkg Binary + Registry Backend (NEW)                     │
│                                                                          │
│  meshpkg CLI         Registry Server (Axum + Postgres)                  │
│  ┌──────────────┐    ┌──────────────────────────────────────┐           │
│  │ publish      │───▶│ POST /api/packages/{name}/{version}  │           │
│  │ install      │◀───│ GET  /api/packages/{name}            │           │
│  │ search       │◀───│ GET  /api/search?q=...               │           │
│  │ login        │    │ GET  /api/packages/{n}/{v}/download  │           │
│  └──────────────┘    └──────────────────────────────────────┘           │
│                                                                          │
│  mesh.toml + mesh.lock (already in mesh-pkg crate, extended)            │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Status |
|-----------|----------------|--------|
| `mesh-rt/src/crypto.rs` | SHA-256/512, HMAC-SHA256/SHA512, UUID v4 as `extern "C"` fns | NEW |
| `mesh-rt/src/date.rs` | Timestamps, parse, format, duration arithmetic as `extern "C"` fns | NEW |
| `mesh-rt/src/encoding.rs` | Base64 encode/decode, hex encode/decode as `extern "C"` fns | NEW |
| `mesh-rt/src/http/client.rs` | Streaming HTTP client, keep-alive agent, builder API | MODIFIED |
| `mesh-codegen/src/codegen/intrinsics.rs` | LLVM extern declarations for all new `mesh_*` symbols | MODIFIED |
| `mesh-typeck/src/builtins.rs` | Type signatures for `Crypto.*`, `Date.*`, `Encoding.*` | MODIFIED |
| `meshc/src/main.rs` | Add `Test` variant to `Commands` enum, dispatch to test runner | MODIFIED |
| `meshc/src/test_runner.rs` | `*.test.mpl` discovery, compile, execute, aggregate results | NEW |
| `mesh-rt/src/test_support.rs` | `mesh_test_assert`, `mesh_test_assert_eq`, `mesh_test_fail` fns | NEW |
| `mesh-pkg/src/manifest.rs` | Add `Dependency::Registry` variant with version field | MODIFIED |
| `mesh-pkg/src/resolver.rs` | Handle Registry variant: fetch tarball from registry URL | MODIFIED |
| `compiler/meshpkg/` | Standalone binary: publish, install, search, login subcommands | NEW CRATE |
| `registry/` | Axum + PostgreSQL HTTP API + tarball storage | NEW APP |
| `website/docs/packages/` | Package browse/search/detail pages in VitePress site | MODIFIED |

---

## Integration Points — New vs. Modified

### 1. Stdlib: Crypto / Date / Encoding

**Where it lives: `mesh-rt` as new modules, NOT new crates.**

All current stdlib (strings, collections, http, db, actor) lives in `mesh-rt/src/`. The crypto deps `sha2`, `hmac`, `md-5`, `base64`, `ring` are already compiled into `mesh-rt/Cargo.toml`. Adding new `extern "C"` functions to `mesh-rt` costs zero additional build time for existing dependencies and follows the established pattern exactly.

**New modules in `mesh-rt/src/`:**

```
mesh-rt/src/
├── crypto.rs     (NEW) — SHA-256, SHA-512, HMAC-SHA256/SHA512, UUID v4
├── date.rs       (NEW) — DateTime as i64 ms, parse/format/arithmetic
├── encoding.rs   (NEW) — base64 encode/decode, hex encode/decode
```

**Dependency additions to `mesh-rt/Cargo.toml`:**

| Dep | Version | For | Status |
|-----|---------|-----|--------|
| `sha2` | 0.10 | SHA-256/512 | Already present |
| `hmac` | 0.12 | HMAC | Already present |
| `base64` | 0.22 | base64 encode/decode | Already present (used as `base64ct` alias) |
| `ring` | 0.17 | UUID random bytes | Already present |
| `rand` | 0.9 | UUID v4 random bytes | Already present |
| `hex` | 0.4 | hex encode/decode | NEW — or implement inline (~10 lines) |
| `chrono` | 0.4 | date/time parsing and formatting | NEW |

**The three-file pattern — mandatory for every new stdlib function:**

Every new Mesh stdlib function requires exactly three files to change: the runtime implementation, the type checker registration, and the LLVM codegen declaration. This is the established pattern for all existing stdlib (verified by inspecting `mesh_http_get`, `mesh_regex_compile`, `mesh_iter_map`, etc.).

```
mesh-rt/src/crypto.rs          → implements  mesh_crypto_sha256(data: *const MeshString) -> *mut MeshString
mesh-typeck/src/builtins.rs    → registers   Crypto.sha256 :: (String) -> String
mesh-codegen/intrinsics.rs     → declares    LLVM extern fn mesh_crypto_sha256(ptr) -> ptr
```

**ABI design for crypto functions (HIGH confidence):**

```rust
// mesh-rt/src/crypto.rs

#[no_mangle]
pub extern "C" fn mesh_crypto_sha256(s: *const MeshString) -> *mut MeshString {
    use sha2::{Sha256, Digest};
    let input = unsafe { (*s).as_str() };
    let hash = format!("{:x}", Sha256::digest(input.as_bytes()));
    unsafe { mesh_string_new(hash.as_ptr(), hash.len() as u64) }
}

#[no_mangle]
pub extern "C" fn mesh_crypto_sha512(s: *const MeshString) -> *mut MeshString { ... }

#[no_mangle]
pub extern "C" fn mesh_crypto_hmac_sha256(
    key: *const MeshString,
    data: *const MeshString,
) -> *mut MeshString { ... }

#[no_mangle]
pub extern "C" fn mesh_crypto_uuid_v4() -> *mut MeshString {
    // Uses rand (already in Cargo.toml) to generate 16 random bytes
    // Formats as lowercase UUID string: "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx"
}
```

**ABI design for date functions:**

`DateTime` should NOT be an opaque heap handle. The simpler, more ergonomic design: represent all timestamps as `i64` milliseconds since Unix epoch. This avoids introducing a new opaque type (which would require new type machinery in mesh-typeck and mesh-codegen) and is consistent with how JavaScript, Java, and Python expose epoch-based timestamps.

```rust
// mesh-rt/src/date.rs

#[no_mangle]
pub extern "C" fn mesh_date_now() -> i64
    // returns Unix ms as Mesh Int

#[no_mangle]
pub extern "C" fn mesh_date_parse(s: *const MeshString) -> *mut u8
    // Returns Result<Int, String> — parses ISO 8601 string → Unix ms

#[no_mangle]
pub extern "C" fn mesh_date_format(ts_ms: i64, fmt: *const MeshString) -> *mut MeshString
    // Formats Unix ms as string using format specifier

#[no_mangle]
pub extern "C" fn mesh_date_add_ms(ts_ms: i64, delta_ms: i64) -> i64
    // Returns ts_ms + delta_ms (trivial arithmetic, but named for clarity)

#[no_mangle]
pub extern "C" fn mesh_date_diff_ms(a_ms: i64, b_ms: i64) -> i64
    // Returns a_ms - b_ms
```

Mesh API surface: `Date.now() -> Int`, `Date.parse(s) -> Int!String`, `Date.format(ts, fmt) -> String`, `Date.add(ts, delta) -> Int`, `Date.diff(a, b) -> Int`.

---

### 2. HTTP Client: Streaming / Keep-Alive / Builder API

**Where it lives: `mesh-rt/src/http/client.rs` — MODIFY existing file.**

**Current state (verified):** `mesh_http_get(url)` and `mesh_http_post(url, body)` use `ureq = "2"`. Both call `response.into_string()` which buffers the entire body. No streaming, no keep-alive, no custom headers.

**ureq v2 vs v3 decision:**

The project uses `ureq = "2"`. ureq v3 has a breaking API change: `Response` is replaced by `http::Response<Body>`, and `body_mut().as_reader()` replaces `into_reader()`. The key improvements in v3 relevant to v14.0:

- `Body: Send` — streaming body readable on another thread (important for actor model)
- `Body::with_config().limit(n)` — explicit size limits for safety
- Standard `http` crate types — consistent with broader Rust HTTP ecosystem

**Recommendation: upgrade to `ureq = "3"` when implementing streaming.** The API change is confined entirely to `client.rs` (one file). The upgrade provides the `Body: Send` guarantee needed for streaming in the actor model.

**Streaming architecture — opaque handle pattern:**

HTTP streams are stateful resources. They follow the same opaque `u64` handle pattern used by DB connections, pools, and regex (verified in codebase):

```rust
// mesh-rt/src/http/client.rs

use std::collections::HashMap;
use std::sync::Mutex;
use std::io::Read;

static STREAMS: Mutex<HashMap<u64, Box<dyn Read + Send>>> = Mutex::new(HashMap::new());
static NEXT_STREAM_ID: std::sync::atomic::AtomicU64 = ...;

#[no_mangle]
pub extern "C" fn mesh_http_stream_get(url: *const MeshString) -> *mut u8 {
    // ureq v3: response.into_parts().1.into_reader() — Body: Send
    // Stores Box<dyn Read + Send> in STREAMS map, returns Result<handle_u64, error_string>
}

#[no_mangle]
pub extern "C" fn mesh_http_read_chunk(handle: u64, max_bytes: i64) -> *mut u8 {
    // Reads up to max_bytes from stored stream
    // Returns Result<Option<String>, String>
    // Option::None signals EOF (0 bytes read)
}

#[no_mangle]
pub extern "C" fn mesh_http_close_stream(handle: u64) -> () {
    // Removes stream from STREAMS map, drops the reader (closes TCP conn)
}
```

**Actor model integration for streaming (HIGH confidence):**

Actor I/O is already blocking — PG queries, HTTP server I/O, and WebSocket sends all block inside actors. Blocking `mesh_http_read_chunk()` is the same model. The actor calls `HTTP.read_chunk(stream, 4096)` in a loop; each call blocks briefly until data arrives. This is Pattern A (simple, recommended).

Pattern B (spawn a reader actor to push chunks) is unnecessarily complex for pull-based streaming. The existing WebSocket reader thread exists because WS frames arrive asynchronously and must be pushed. HTTP streaming is pull-based.

**Keep-alive — ureq Agent:**

ureq v2/v3 both have an `Agent` type that maintains a connection pool with keep-alive. The agent is stored as a leaked `Box<ureq::Agent>` behind a `u64` handle, same pattern as DB pools:

```rust
#[no_mangle]
pub extern "C" fn mesh_http_build_client(/* config params */) -> u64 {
    // Creates ureq::AgentBuilder, stores Box<Agent> as opaque u64 handle
}

#[no_mangle]
pub extern "C" fn mesh_http_client_get(client: u64, url: *const MeshString) -> *mut u8 {
    // Uses stored Agent to make GET request with connection reuse
}

#[no_mangle]
pub extern "C" fn mesh_http_client_post(
    client: u64,
    url: *const MeshString,
    body: *const MeshString
) -> *mut u8 { ... }
```

**Fluent builder API:**

Rather than a mutable builder object (which would require another opaque handle), use function composition with a `RequestConfig` struct passed by value:

```
# Mesh API:
let response = HTTP.new_request(:post, url)
  |> HTTP.set_header("Authorization", "Bearer " <> token)
  |> HTTP.set_header("Content-Type", "application/json")
  |> HTTP.set_timeout(5000)
  |> HTTP.set_body(json_body)
  |> HTTP.send()
```

This maps to a `MeshRequestConfig` struct in the runtime (header list + timeout + body), passed by pointer. No opaque handle needed — the struct is stack-allocated in the Mesh caller and passed to `mesh_http_send`.

---

### 3. Testing Framework: `meshc test` Runner

**Where it lives: New `test_runner.rs` module in the `meshc` binary crate.**

The `migrate` subcommand is already implemented as `meshc/src/migrate.rs` — a module within the binary crate, not a separate library. The test runner follows the same pattern. It uses the existing `build()` function from `meshc/src/main.rs` and needs no new library crate.

**New Commands variant in `meshc/src/main.rs`:**

```rust
Test {
    /// Project directory (default: current directory)
    #[arg(default_value = ".")]
    dir: PathBuf,

    /// Run only tests whose filename matches this pattern
    #[arg(long)]
    filter: Option<String>,

    /// Keep compiled test binaries after running
    #[arg(long)]
    keep_artifacts: bool,
}
```

**File discovery:**

```rust
// meshc/src/test_runner.rs
fn discover_test_files(dir: &Path, filter: Option<&str>) -> Vec<PathBuf> {
    // Reuses collect_mesh_files_recursive from main.rs
    // Filters: file.extension() == "mpl" AND file.stem() ends with ".test"
    // i.e., matches *.test.mpl
    // Applies optional name filter substring match
}
```

**Execution model — test file = complete Mesh program:**

Each `*.test.mpl` is a complete Mesh program with a `main` function that calls test functions in sequence. If any assertion fails, `mesh_panic()` causes a non-zero exit. The test runner compiles and executes each file independently:

```
meshc test ./my-project
    ↓
discover all *.test.mpl files in project dir
    ↓
for each test_file:
    tmpdir = mktemp
    build(test_file_parent_dir, opt=0, output=Some(tmpdir/test_binary))
        (reuses existing build() pipeline — same parse/typecheck/codegen)
    ↓
    result = Command::new(tmpdir/test_binary).status()
    collect: (file_name, exit_code, elapsed)
    ↓
aggregate: count pass/fail
print summary: "5 passed, 1 failed"
exit(if any_failed { 1 } else { 0 })
```

**Why not function-level test discovery (like `cargo test`):**

The more sophisticated approach — parse test files, discover `test_` prefix functions, generate a runner `main` — requires the test runner to generate new Mesh source code and compile it. This duplicates parser/typechecker functionality and adds significant complexity. The "test file is a program" model is simpler, matches Go's original `_test.go` approach, and can be delivered in v14.0. Function-level discovery can be added in a future milestone.

**Assertion helpers — three-file pattern:**

```rust
// mesh-rt/src/test_support.rs

#[no_mangle]
pub extern "C" fn mesh_test_assert(condition: i8, msg: *const MeshString) {
    if condition == 0 {
        // Print "assertion failed: <msg>" then call mesh_panic()
    }
}

#[no_mangle]
pub extern "C" fn mesh_test_assert_eq(
    a: *const MeshString,
    b: *const MeshString,
    label: *const MeshString,
) {
    // String equality check; panic with "expected X, got Y" if not equal
}

#[no_mangle]
pub extern "C" fn mesh_test_fail(msg: *const MeshString) {
    // Unconditional panic with message
}
```

Mesh surface: `Test.assert(bool)`, `Test.assert_eq(a, b)`, `Test.fail(msg)`.

`assert_raises` (run closure, assert it panics) is more complex — defer to a later phase or implement via `catch_unwind` in the runtime (already used for actor crash isolation).

**Mock actors:**

In the actor model, "mocking" means replacing a named registered process with a test double. The pattern is documentable without new language features:

```mesh
# In test file: start a mock actor, register under expected name
let mock_pid = spawn do
  receive do
    {:get_user, id, reply_pid} -> send(reply_pid, {:ok, test_user})
  end
end
Process.register(mock_pid, :user_service)
# Run code under test — it calls named :user_service
# Assert on results
```

This requires no new framework features — it's a usage pattern. Document it in the testing guide.

**Coverage reporting:**

LLVM's coverage instrumentation requires adding `-fprofile-instr-generate -fcoverage-mapping` flags to the LLVM compilation step and running `llvm-profdata merge` + `llvm-cov report` on the resulting data. This is implementable but requires changes to `mesh-codegen/src/link.rs` and a post-execution step in the test runner. Mark as stretch goal for v14.0; defer if time is limited.

---

### 4. Package Registry: mesh.toml, meshpkg CLI, Hosted Backend

**mesh.toml current state (HIGH confidence from codebase inspection):**

The `mesh-pkg` crate already has:
- `Manifest` struct: `package` (name/version/description/authors) + `dependencies: BTreeMap<String, Dependency>`
- `Dependency` enum: `Git { git, rev, branch, tag }` and `Path { path }`
- `Lockfile` with `LockedPackage { name, source, revision }`
- DFS resolver with diamond detection and cycle detection
- git2-based clone/fetch for Git dependencies

v14.0 adds a `Registry` variant to `Dependency`:

```rust
// mesh-pkg/src/manifest.rs — MODIFIED

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Git { git: String, rev: Option<String>, branch: Option<String>, tag: Option<String> },
    Path { path: String },
    Registry { version: String },  // NEW — uses default registry or package-level override
}
```

Full `mesh.toml` format with registry dep:

```toml
[package]
name = "my-app"
version = "1.0.0"
description = "An example Mesh application"
authors = ["Alice <alice@example.com>"]

[dependencies]
# Registry dependency (NEW for v14.0)
crypto-utils = { version = "1.2.0" }

# Existing forms unchanged
local-lib    = { path = "../local-lib" }
github-lib   = { git = "https://github.com/example/lib.git", tag = "v2.0" }
```

**IMPORTANT: Exact versions only (no SemVer ranges).** Version ranges require a SemVer SAT solver — a multi-week project. Require exact versions (`"1.2.0"`) in v14.0. The lockfile already provides reproducibility. Ranges can be a future milestone feature.

**meshpkg binary — new crate at `compiler/meshpkg/`:**

```
compiler/
└── meshpkg/
    ├── Cargo.toml         — depends on mesh-pkg, reqwest/ureq for HTTP, tar, flate2
    └── src/
        └── main.rs        — publish, install, search, login, logout subcommands
```

This is a *separate binary from `meshc`*. Rationale: `meshc` is the compiler (build, test, format, migrate). `meshpkg` is the ecosystem tool (publish, install, search). Separation mirrors Go's `go` vs. package tools and keeps each binary focused.

**meshpkg commands:**

```
meshpkg publish             Pack mesh.toml + src/ into .tar.gz, upload to registry
meshpkg install [pkg@ver]   Download tarball, unpack to .mesh/deps/<pkg>/
meshpkg search <query>      Query registry search API, print results table
meshpkg login               Store API token to ~/.mesh/credentials
meshpkg logout              Remove stored token
meshpkg list                List installed packages from mesh.lock
```

**Package format:**

A Mesh package is a `.tar.gz` tarball containing:
- `mesh.toml` at root
- `*.mpl` source files (preserving directory structure)
- `README.md` (optional)
- No compiled artifacts, no `.mesh/` directory

Content-addressed by SHA-256 of the tarball. Registry identifies packages as `{name}@{version}`.

**Registry server stack (MEDIUM confidence — verified via web search):**

The registry is a separate HTTP service. It does NOT need to be a Mesh application (and using Mesh would be premature/risky for critical infrastructure). Recommended stack:

| Layer | Technology | Why |
|-------|------------|-----|
| HTTP framework | Axum 0.7 | Thin over hyper, Tower middleware, rustls-compatible, proven in 2025 ecosystem |
| Database | PostgreSQL | Already the production DB for Mesher; familiar wire protocol; already in use |
| ORM | sea-orm 1.x or sqlx | Standard Axum ecosystem; sea-orm 1.x stable; sqlx is simpler if schema is small |
| Auth | API key (HMAC-SHA256) | ring is already a dependency in mesh-rt; simple bearer token model |
| Tarball storage | Local filesystem to start | `/data/packages/{name}/{version}.tar.gz`; abstract behind `StorageBackend` trait for future S3/R2 |
| TLS | rustls (already in codebase) | Consistent with rest of Mesh infrastructure |

**Registry API surface (minimal v14.0):**

```
GET  /api/packages                       → paginated package list
GET  /api/packages/{name}                → package metadata + all versions
GET  /api/packages/{name}/{version}      → specific version: metadata + README
GET  /api/packages/{name}/{version}/download  → tarball download
POST /api/packages/{name}/{version}      → publish (requires Bearer token)
GET  /api/search?q={query}&page={n}      → search by name/description
POST /api/auth/token                     → exchange credentials for API token
```

**Registry database schema (minimal):**

```sql
CREATE TABLE packages (
    name       TEXT NOT NULL,
    version    TEXT NOT NULL,
    description TEXT,
    authors    TEXT[],
    published_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    checksum   TEXT NOT NULL,     -- SHA-256 of tarball
    tarball_path TEXT NOT NULL,   -- filesystem path
    readme     TEXT,
    PRIMARY KEY (name, version)
);

CREATE TABLE api_tokens (
    id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    token_hash TEXT NOT NULL,    -- HMAC-SHA256 of token
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

**Registry website integration:**

The existing VitePress website at `website/` gets new pages under `website/docs/packages/`:

```
/packages                  → browse all packages (client-side fetch from registry API)
/packages/{name}           → package detail: README, versions, install snippet
/search?q=...              → search results
```

These pages can be client-side rendered (Vue components fetching from the registry API) to avoid requiring the registry to be running at VitePress build time. This is simpler for v14.0.

---

## Recommended File Structure Changes

```
compiler/
├── mesh-rt/src/
│   ├── crypto.rs          (NEW) — SHA-256/512, HMAC, UUID, exported as mesh_crypto_*
│   ├── date.rs            (NEW) — Date/time operations, exported as mesh_date_*
│   ├── encoding.rs        (NEW) — base64/hex, exported as mesh_encoding_*
│   ├── test_support.rs    (NEW) — Test.assert/assert_eq/fail, exported as mesh_test_*
│   ├── http/
│   │   └── client.rs      (MODIFIED) — streaming, keep-alive, builder API; ureq → v3
│   └── lib.rs             (MODIFIED) — pub mod crypto; date; encoding; test_support; re-exports
├── mesh-typeck/src/
│   └── builtins.rs        (MODIFIED) — Crypto.*, Date.*, Encoding.*, Test.* type sigs
├── mesh-codegen/src/codegen/
│   └── intrinsics.rs      (MODIFIED) — LLVM extern decls for all new mesh_* symbols
├── mesh-pkg/src/
│   ├── manifest.rs        (MODIFIED) — add Dependency::Registry { version } variant
│   └── resolver.rs        (MODIFIED) — handle Registry variant: fetch tarball from registry URL
│   └── publish.rs         (NEW) — pack tarball, upload to registry
├── meshc/src/
│   ├── main.rs            (MODIFIED) — add Commands::Test variant
│   └── test_runner.rs     (NEW) — *.test.mpl discovery, compile, execute, report
└── meshpkg/               (NEW binary crate)
    ├── Cargo.toml
    └── src/
        └── main.rs        — publish, install, search, login, logout

registry/                  (NEW — top-level, outside compiler/)
├── Cargo.toml             — axum, sea-orm or sqlx, tokio, serde, ring, tar, flate2
└── src/
    ├── main.rs
    ├── routes/
    │   ├── packages.rs
    │   ├── search.rs
    │   └── auth.rs
    ├── storage.rs         — StorageBackend trait + FilesystemStorage impl
    └── db.rs

website/docs/
└── packages/              (NEW) — browse, search, detail pages (Vue + client-side fetch)
```

---

## Architectural Patterns

### Pattern 1: Three-File Stdlib Expansion

**What:** Every new Mesh stdlib function requires changes to exactly three files: `mesh-rt/src/<module>.rs` (implementation), `mesh-typeck/src/builtins.rs` (type signature registration), `mesh-codegen/src/codegen/intrinsics.rs` (LLVM extern declaration).

**When to use:** All new `Crypto.*`, `Date.*`, `Encoding.*`, `Test.*`, HTTP client functions. Non-negotiable — this is how every existing stdlib function works.

**Trade-offs:** Repetitive but enforces consistency. Missing any one of the three causes either a compile-time type error (typeck missing) or a linker error (rt missing). The pattern makes additions predictable and reviewable.

**Example (adding `Crypto.sha256`):**

```rust
// File 1: mesh-rt/src/crypto.rs
#[no_mangle]
pub extern "C" fn mesh_crypto_sha256(s: *const MeshString) -> *mut MeshString {
    use sha2::{Sha256, Digest};
    let input = unsafe { (*s).as_str() };
    let hash = format!("{:x}", Sha256::digest(input.as_bytes()));
    unsafe { mesh_string_new(hash.as_ptr(), hash.len() as u64) }
}
```

```rust
// File 2: mesh-typeck/src/builtins.rs (inside register_builtins)
env.define("crypto_sha256", Scheme::simple(
    Ty::fun(vec![Ty::string()], Ty::string())
));
```

```rust
// File 3: mesh-codegen/src/codegen/intrinsics.rs (inside declare_intrinsics)
module.add_function(
    "mesh_crypto_sha256",
    ptr_type.fn_type(&[ptr_type.into()], false),
    Some(inkwell::module::Linkage::External),
);
```

### Pattern 2: Opaque Handle for Stateful Resources

**What:** Stateful resources (DB connections, pools, streams, regexes) are stored behind a `Box<T>` leaked into a `u64` handle. The handle is opaque from Mesh's perspective (`u64` → `Int` in the type system). Operations take the handle as first argument.

**When to use:** HTTP streaming (stream handle), HTTP keep-alive client (agent handle). NOT for date/time (use `i64` directly).

**Trade-offs:** Simple to implement, GC-safe. Downside: no type safety at Mesh level. All DB connections and pools use this pattern — it is battle-tested in the codebase.

### Pattern 3: Subcommand as Module in meshc

**What:** New `meshc` subcommands are implemented as `src/subcommand_name.rs` modules within the `meshc` binary crate. Not as separate library crates.

**When to use:** `meshc test`, and any future subcommands.

**Trade-offs:** Simple. The test runner is ~200-400 lines, does not need its own crate. The `migrate.rs` module in `meshc` is the established model.

---

## Data Flow

### `meshc test` Execution Flow

```
meshc test ./my-project
    ↓
test_runner::discover_test_files(dir, filter)
    walks dir recursively, collects *.test.mpl paths
    ↓
for each test_file in discovered:
    tmpdir = tempfile::TempDir::new()
    build(test_file_parent_dir, opt=0, output=Some(tmpdir.path().join("test_bin")))
        → same build() function used by meshc build
        → parse → typecheck → MIR → LLVM → native binary
    ↓
    let output = Command::new(tmpdir/test_bin).output()
    result = TestResult { file, exit_code, stdout, stderr, duration }
    ↓
print_test_summary(results):
    "test/user.test.mpl ... ok"
    "test/auth.test.mpl ... FAILED (exit 1)"
    stdout/stderr of failed tests
    "5 passed, 1 failed" / exit(1) if any failed
```

### Crypto/Encoding Call Flow

```
Mesh source:  Crypto.sha256(my_string)
    ↓
mesh-typeck:  resolves "crypto_sha256" → Ty::Fun([String], String)
    ↓
mesh-codegen MIR:  lower to Call { callee: "mesh_crypto_sha256", args: [string_val] }
    ↓
mesh-codegen LLVM: call ptr @mesh_crypto_sha256(ptr %str)
    ↓
runtime:  mesh_crypto_sha256() in mesh-rt/src/crypto.rs
    sha2::Sha256::digest(input_bytes)
    hex-encode → mesh_string_new(hex_bytes, len) → GC-managed MeshString ptr
    ↓
return value to Mesh caller
```

### Package Publish Flow

```
meshpkg publish (in project root with mesh.toml)
    ↓
read + validate mesh.toml
    ↓
pack: tar::Builder → gzip → .tar.gz
    include: mesh.toml, **/*.mpl, README.md
    exclude: .mesh/, compiled binaries
    ↓
sha256 = sha2::Sha256::digest(tarball_bytes)
    ↓
read ~/.mesh/credentials → API token
    ↓
POST /api/packages/{name}/{version}
    Content-Type: multipart/form-data
    Authorization: Bearer {token}
    body: { tarball, checksum, metadata }
    ↓
registry server:
    verify token
    check version not already published (immutable)
    parse + validate mesh.toml from tarball
    write tarball to /data/packages/{name}/{version}.tar.gz
    insert packages row
    ↓
200 OK: "Published {name}@{version}"
```

---

## Suggested Build Order

Dependencies drive the order. Items with no dependencies on each other can be built in parallel.

| Step | Feature | Depends On | Rationale |
|------|---------|------------|-----------|
| 1 | Encoding stdlib (base64, hex) | Nothing | Simplest, validates three-file pattern |
| 2 | Crypto stdlib (SHA/HMAC/UUID) | Step 1 pattern | Deps already present |
| 3 | Date stdlib (timestamps, format) | Nothing | Needs chrono dep added |
| 4 | HTTP streaming (ureq v3 upgrade) | Nothing | Isolated to client.rs |
| 5 | HTTP keep-alive + builder API | Step 4 | Builds on streaming infrastructure |
| 6 | Test assertion helpers (Test.assert etc) | Nothing | Three-file pattern |
| 7 | `meshc test` runner | Step 6 | Needs assertion helpers in rt |
| 8 | mesh.toml Registry dep variant | Nothing | Extends manifest.rs/resolver.rs |
| 9 | meshpkg CLI binary | Step 8 | Needs mesh-pkg registry support |
| 10 | Registry server (Axum + PG) | Step 8 API contract | Independent of compiler changes |
| 11 | Registry website pages | Step 10 | Needs registry API |

**Parallelization:** Steps 1-5 (stdlib + HTTP) are independent of steps 6-7 (test runner). Steps 8-11 (registry) are independent of all compiler/runtime work and can be built in parallel by a separate effort.

---

## Anti-Patterns

### Anti-Pattern 1: New Crates for Stdlib Modules

**What people do:** Create `mesh-crypto`, `mesh-date`, `mesh-encoding` as separate Rust crates.

**Why it's wrong:** All stdlib compiles into `libmesh_rt.a` which is statically linked into every Mesh binary. Separate crates either (a) get merged into `libmesh_rt.a` via re-exports anyway — adding build complexity with no benefit — or (b) become separate static libs, requiring linker changes in `mesh-codegen/src/link.rs`. Neither is justified for the scale of these additions.

**Do this instead:** Add modules directly to `mesh-rt/src/` and `pub mod` them from `lib.rs`. This is identical to how `regex.rs`, `json.rs`, `iter.rs` were added in prior milestones.

### Anti-Pattern 2: Streaming via Dedicated OS Thread per Request

**What people do:** Spawn an OS thread for each HTTP stream (analogous to WebSocket's reader thread).

**Why it's wrong:** WebSocket's reader thread exists because WS frames arrive asynchronously and must be pushed into the actor mailbox. HTTP streaming is pull-based — the actor requests chunks when ready. Spawning OS threads for pull-based streaming adds unnecessary complexity and overhead.

**Do this instead:** Blocking `mesh_http_read_chunk()` called directly inside the actor. Accepted pattern since DB queries, HTTP server I/O, and WS sends already block inside actors.

### Anti-Pattern 3: Test Files Without a `main` Function

**What people do:** Design test files as collections of annotated functions without an entry point, requiring the runner to generate a main.

**Why it's wrong:** Generating a main requires the test runner to parse Mesh syntax, extract function names, synthesize new Mesh code, and compile the generated code. This is significant complexity duplicating parser/typechecker work.

**Do this instead:** Each `*.test.mpl` is a complete Mesh program with a `main` that calls test functions. The runner compiles and executes — nothing more. This is the established model for Go test files before `testing.T` was mature.

### Anti-Pattern 4: SemVer Range Resolution in registry deps

**What people do:** Allow `"^1.0"` or `">=2.0"` in mesh.toml registry dependencies.

**Why it's wrong:** SemVer constraint solving (similar to Cargo's resolver) is a multi-week project. Cargo's own resolver took years to stabilize. For v14.0, this is out of scope.

**Do this instead:** Require exact versions: `version = "1.2.0"`. The lockfile already guarantees reproducibility. Add ranges in a future milestone.

### Anti-Pattern 5: DateTime as Opaque Handle

**What people do:** Represent `DateTime` as an opaque heap-allocated struct behind a `u64` handle.

**Why it's wrong:** Requires new type machinery in mesh-typeck (new opaque `DateTime` type), new codegen handling, and forces users to interact with a resource that has no GC collection (requires explicit `Date.free(dt)` or leaks). Timestamps are naturally numeric values.

**Do this instead:** Represent timestamps as `i64` Unix milliseconds. Arithmetic is trivial (`Date.add(ts, delta) -> Int`), formatting is a string operation, and no new type machinery is needed. JavaScript, Go, and Python's standard libraries all use this pattern.

---

## Integration Points Summary

| Feature | Files Modified | Files Added | New Rust Deps |
|---------|---------------|-------------|---------------|
| Encoding stdlib | `mesh-rt/lib.rs`, `builtins.rs`, `intrinsics.rs` | `mesh-rt/src/encoding.rs` | `hex = "0.4"` (optional) |
| Crypto stdlib | `mesh-rt/lib.rs`, `builtins.rs`, `intrinsics.rs` | `mesh-rt/src/crypto.rs` | none (sha2/hmac/ring/rand already present) |
| Date stdlib | `mesh-rt/lib.rs`, `builtins.rs`, `intrinsics.rs`, `mesh-rt/Cargo.toml` | `mesh-rt/src/date.rs` | `chrono = "0.4"` |
| HTTP streaming | `mesh-rt/http/client.rs`, `mesh-rt/Cargo.toml`, `builtins.rs`, `intrinsics.rs` | — | ureq upgrade `"2"` → `"3"` |
| HTTP keep-alive + builder | `mesh-rt/http/client.rs`, `builtins.rs`, `intrinsics.rs` | — | (same ureq upgrade) |
| Test assertions | `mesh-rt/lib.rs`, `builtins.rs`, `intrinsics.rs` | `mesh-rt/src/test_support.rs` | — |
| `meshc test` runner | `meshc/src/main.rs` | `meshc/src/test_runner.rs` | — |
| mesh.toml Registry variant | `mesh-pkg/src/manifest.rs`, `mesh-pkg/src/resolver.rs` | `mesh-pkg/src/publish.rs` | reqwest or ureq (for HTTP downloads) |
| meshpkg CLI | — | `compiler/meshpkg/` (new binary crate) | (inherits from mesh-pkg) |
| Registry server | — | `registry/` (new workspace member) | axum 0.7, sea-orm 1.x, tokio, serde_json, ring, tar, flate2 |
| Registry website | — | `website/docs/packages/` (Vue pages) | — |

---

## Sources

- Codebase: `compiler/mesh-rt/src/lib.rs` — all stdlib modules and re-exports (direct inspection)
- Codebase: `compiler/mesh-rt/Cargo.toml` — confirmed sha2/hmac/md-5/base64/ring already present
- Codebase: `compiler/mesh-rt/src/http/client.rs` — current mesh_http_get/post implementation using ureq 2
- Codebase: `compiler/mesh-codegen/src/codegen/intrinsics.rs` — LLVM extern declaration pattern
- Codebase: `compiler/mesh-typeck/src/builtins.rs` — type registration pattern for stdlib functions
- Codebase: `compiler/meshc/src/main.rs` + `migrate.rs` — subcommand-as-module pattern
- Codebase: `compiler/mesh-pkg/src/manifest.rs` + `resolver.rs` — existing mesh.toml format and resolver
- [ureq 2.x Response docs](https://docs.rs/ureq/2.3.0/ureq/struct.Response.html) — `into_reader()` for streaming
- [ureq 3.x Body docs](https://docs.rs/ureq/latest/ureq/struct.Body.html) — `body_mut().as_reader()`, `Body: Send`
- [ureq CHANGELOG](https://docs.rs/crate/ureq/latest/source/CHANGELOG.md) — v2→v3 API changes confirmed
- [axum crates.io](https://crates.io/crates/axum) — registry server web framework
- [sea-orm GitHub](https://github.com/SeaQL/sea-orm) — async ORM for registry server (1.x stable)

---

*Architecture research for: Mesh v14.0 Ecosystem (stdlib, HTTP client, testing, package registry)*
*Researched: 2026-02-28*

# Stack Research

**Domain:** Programming language ecosystem — stdlib expansion, HTTP client, testing framework, package registry
**Researched:** 2026-02-28
**Confidence:** HIGH (all critical crates verified against docs.rs and Cargo.lock)

---

## Context: Existing Stack (Do Not Re-Research)

The Mesh compiler already depends on these crates that are **directly reusable** for v14.0 features:

| Already Present | Version (locked) | Reusable For |
|-----------------|-----------------|--------------|
| `sha2` | 0.10.9 | SHA-256/512 — expose via FFI, zero new dep |
| `hmac` | 0.12.1 | HMAC-SHA256/512 — expose via FFI, zero new dep |
| `base64` | 0.22.1 | Base64 encode/decode — expose via FFI, zero new dep |
| `rand` | 0.9 | UUID v4 random source |
| `rustls` / `ring` | 0.23 / 0.17 | Crypto provider already installed at runtime init |
| `ureq` | 2.12.1 | HTTP client (upgrade path to 3.x for streaming/pooling) |
| `tokio` | 1.x | Async runtime for axum-based packages registry backend |
| `serde` / `serde_json` | 1.x | JSON for registry API |
| `toml` | 0.8 | mesh.toml manifest parsing |
| `semver` | 1.x | Package version constraint resolution |
| `git2` | 0.19 | Git operations in mesh-pkg |
| `clap` | 4.5 | CLI arg parsing for meshpkg |

All crypto stdlib work (SHA-256/512, HMAC, Base64) has **zero new Rust dependencies** — the crates are already compiled as transitive deps.

---

## New Dependencies Required

### 1. Crypto Stdlib (SHA-256/512, HMAC, UUID)

| Library | Version | Add To | Purpose | Why |
|---------|---------|--------|---------|-----|
| `uuid` | 1.21 | `mesh-rt` | UUID v4 generation | Standard crate (374M+ downloads), `v4` feature uses `rand` already present; 1.21 is latest stable |
| `sha2` | 0.10 | `mesh-rt` | SHA-256/512 | **Already present** — just add `mesh_sha256` / `mesh_sha512` extern "C" fns, zero new dep |
| `hmac` | 0.12 | `mesh-rt` | HMAC-SHA256/512 | **Already present** — already used for PostgreSQL SCRAM auth, zero new dep |
| `base64` | 0.22 | `mesh-rt` | Base64 encode/decode | **Already present** — used for PostgreSQL auth, zero new dep |

**uuid Cargo.toml addition to mesh-rt:**
```toml
uuid = { version = "1", features = ["v4"] }
```

For hex encoding, do NOT add the `hex` crate. Hex is trivial inline Rust:
```rust
bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
```
Decode is a simple loop over char pairs. Zero dependency for 3 lines of code.

### 2. Date/Time Stdlib

| Library | Version | Add To | Purpose | Why |
|---------|---------|--------|---------|-----|
| `chrono` | 0.4.42 | `mesh-rt` | Parse, format, arithmetic, timestamps | 392M+ downloads, UTC-first, strftime-style formatting, serde integration, no soundness caveats in multi-threaded programs (fixed in 0.4.20). Preferred over `jiff` because v14.0 only needs UTC timestamps + duration arithmetic — not DST-aware calendar arithmetic. Preferred over `time 0.3` because time crate has soundness caveats with `UtcOffset::current_local_offset` in multi-threaded programs (and mesh-rt is multi-threaded). |

**Cargo.toml addition to mesh-rt:**
```toml
chrono = { version = "0.4", features = ["serde"] }
```

Do NOT add `chrono-tz`. Mesh doesn't need timezone-aware calendar arithmetic in v14.0. UTC timestamps + duration arithmetic cover the full feature set and chrono-tz adds ~2MB binary bloat.

### 3. Base64/Hex Encoding

| Library | Version | Add To | Purpose | Why |
|---------|---------|--------|---------|-----|
| `base64` | 0.22 | `mesh-rt` | Base64 encode/decode | **Already present** — `general_purpose::STANDARD` engine for standard, `URL_SAFE` engine for URL-safe variant |
| hex (inline) | n/a | `mesh-rt` | Hex encode/decode | Implement directly in 3-5 lines of Rust — zero dep for trivial functionality |

The base64 Engine API (0.22.x) uses `general_purpose::STANDARD.encode(bytes)` and `general_purpose::STANDARD.decode(b64str)`. This is the stable API post-0.21 migration away from the deprecated top-level `encode`/`decode` functions.

### 4. HTTP Client Improvements (Streaming, Keep-Alive, Builder API)

| Library | Version | Add To | Purpose | Why |
|---------|---------|--------|---------|-----|
| `ureq` | **3.2** | `mesh-rt` | Streaming, connection pooling, builder API | Upgrade from locked 2.12.1. ureq 3.x adds proper `Agent` connection pooling, `Body::into_reader()` for streaming, `RequestBuilder` fluent API with `.header()` / `.timeout_global()`, semver-stable re-exports. Already the HTTP client dep — upgrade not replace. |

**Cargo.toml change in mesh-rt:**
```toml
# Before:
ureq = "2"
# After:
ureq = "3"
```

**ureq 3.x API mapping to Mesh stdlib additions:**

| Mesh Feature | ureq 3.x API |
|---|---|
| Connection keep-alive | `Agent::config_builder().build()` — Agent holds a connection pool, shared via `Arc`, reused across requests |
| Streaming response | `response.body_mut().into_reader()` → `impl Read + 'static` — owned reader that can be sent across actor boundaries |
| Chunked response | Automatic — ureq transparently decodes `Transfer-Encoding: chunked`; `content_length()` returns `None` for chunked |
| Builder API | `agent.get(url).header("Authorization", "Bearer x").timeout_global(Duration::from_secs(30)).call()` |
| Limited reads | `.body_mut().with_config().limit(N).read_to_vec()` — safe downloads without memory exhaustion |

**Breaking change from 2.x:** `response.into_string()` → `response.body_mut().read_to_string()`. Both existing `mesh_http_get` and `mesh_http_post` in `http/client.rs` need updating.

Do NOT switch to `reqwest`. reqwest requires async/await (Tokio) in the calling code. mesh-rt uses synchronous blocking I/O throughout — actor coroutines, not async/await. Switching would require redesigning all HTTP client callsites and adding Tokio runtime management inside the actor scheduler.

### 5. Testing Framework (meshc test, coverage)

The testing framework is entirely implemented **within the compiler and runtime** — no new Rust crates needed for the test runner or assertion framework. Coverage reporting uses an external tool invoked by `meshc test --coverage`.

| Component | Approach | New Dep? |
|-----------|----------|----------|
| `*.test.mpl` discovery | Filesystem walk already in compiler pipeline | No |
| Test runner execution | Compile test files with special harness entry point; capture pass/fail/panic | No |
| Assertion helpers | `assert`, `assert_eq`, `assert_raises` — Mesh stdlib functions returning `Result<(), String>` | No |
| Mock actors | Stub actor registrations in mesh-rt using existing spawn/mailbox infrastructure | No |
| Function stubs | Compiler-level concept: register alternative function bindings for test scope | No |
| Coverage instrumentation | Add `-C instrument-coverage` LLVM flag during test builds | No |
| Coverage report | Invoke `llvm-profdata` + `llvm-cov` from `llvm-tools-preview` rustup component | No new dep |

For running `meshc`'s own Rust test suite with coverage (CI), use `cargo-llvm-cov`. This is a dev tool for the compiler developers, not part of the Mesh language distribution.

**Why cargo-llvm-cov over cargo-tarpaulin:**
- cargo-tarpaulin uses ptrace on Linux (x86_64 only by default) and LLVM on macOS. Mesh targets both.
- cargo-llvm-cov is cross-platform LLVM instrumentation — same backend on all platforms.
- cargo-llvm-cov supports proc-macros and doc tests.
- cargo-llvm-cov is faster because it instruments only necessary crates.

For Mesh program coverage specifically (coverage of .mpl programs compiled by meshc): the codegen phase adds LLVM instrumentation (`-C instrument-coverage` equivalent on IR), and `llvm-profdata`/`llvm-cov` process the raw profile data. These come from the `llvm-tools-preview` rustup component, not a new crate.

### 6. Package Registry (meshpkg CLI + Hosted Site)

**CLI additions to existing `mesh-pkg` crate:**

| Library | Version | Already Present | Purpose | Why |
|---------|---------|----------------|---------|-----|
| `ureq` | 3.2 | No (mesh-pkg) | HTTP calls to registry API from CLI | Upgrade ureq in mesh-rt covers mesh-rt; mesh-pkg needs its own dep since it doesn't depend on mesh-rt |
| `sha2` | 0.10 | No (mesh-pkg) | Package checksum verification | mesh-pkg doesn't currently depend on mesh-rt where sha2 is present |
| `tar` | 0.4 | No | Package tarball creation/extraction | Standard crate for .tar.gz archives; pairs with flate2 |
| `flate2` | 1 | No | gzip compression for tarballs | Standard gzip; pairs with tar for .tar.gz format |

**mesh-pkg Cargo.toml additions:**
```toml
ureq = "3"
sha2 = "0.10"
tar = "0.4"
flate2 = "1"
```

**New `mesh-registry` binary (separate crate in workspace):**

The hosted packages site backend is a new Rust binary. It is NOT part of meshc or mesh-pkg — it's a separate server deployed independently.

| Library | Version | Purpose | Why |
|---------|---------|---------|-----|
| `axum` | 0.8.8 | Registry API web framework | tokio-rs maintained, same tokio dep already in workspace, Tower middleware ecosystem, 0.8 released Jan 2025 with stable API. crates.io itself uses axum. |
| `tokio` | 1 | Async runtime | Already in workspace |
| `tower` | 0.5 | Rate limiting, auth middleware | axum uses Tower natively — get rate limiting, timeouts, auth for free |
| `serde` / `serde_json` | 1 | JSON request/response | Already in workspace |
| `sqlx` | 0.8 | PostgreSQL for package metadata | Async-native, compile-time checked queries, matches axum/tokio stack |
| `sha2` | 0.10 | Package integrity verification | Registry validates checksums on publish |
| `tar` | 0.4 | Tarball handling | Inspect/store uploaded packages |
| `flate2` | 1 | gzip compression | Paired with tar |
| `uuid` | 1 | Package upload tokens | Random tokens for publish authentication |
| `chrono` | 0.4 | Timestamps on package versions | Consistent with mesh-rt chrono usage |

**New workspace member Cargo.toml:**
```toml
[package]
name = "mesh-registry"

[dependencies]
axum = "0.8"
tokio = { workspace = true }
tower = "0.5"
serde = { workspace = true }
serde_json = { workspace = true }
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "macros"] }
sha2 = "0.10"
tar = "0.4"
flate2 = "1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
```

**Workspace Cargo.toml changes:**
```toml
# Add to [workspace.members]:
"compiler/mesh-registry"

# Add to [workspace.dependencies]:
axum = "0.8"
```

**Hosted Registry Frontend — extend existing website:**

The packages site is NOT a separate SPA. It extends the existing `website/` VitePress site with new pages.

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| VitePress | 1.6.4 | Static pages (browse, search, per-package) | Already deployed for docs site — zero new tooling, zero new deployment |
| Vue 3 | 3.5.28 | Dynamic package browsing components fetching registry API | Already present |
| Tailwind CSS v4 | 4.1.18 | Styling | Already present |

The packages site does not need a separate framework. VitePress pages with Vue components fetching the registry API at runtime cover browse/search/detail views. Static pre-rendered pages for SEO, dynamic fetch for real-time version data.

---

## Recommended Stack Summary

### Additions to mesh-rt/Cargo.toml

```toml
# New additions (everything else already present):
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
# Upgrade (not addition):
ureq = "3"   # was "2"
```

### Additions to mesh-pkg/Cargo.toml

```toml
ureq = "3"
sha2 = "0.10"
tar = "0.4"
flate2 = "1"
```

### New mesh-registry/Cargo.toml

```toml
[package]
name = "mesh-registry"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { workspace = true }
tower = "0.5"
serde = { workspace = true }
serde_json = { workspace = true }
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "macros"] }
sha2 = "0.10"
tar = "0.4"
flate2 = "1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
```

---

## Alternatives Considered

| Recommended | Alternative | Why Not |
|-------------|-------------|---------|
| `chrono 0.4` | `jiff` (BurntSushi, 2024) | jiff is more correct for DST-aware calendar math but v14.0 only needs UTC timestamps + duration arithmetic. chrono's 392M downloads, mature serde integration, and smaller API surface make it the pragmatic choice. Revisit if timezone features are requested. |
| `chrono 0.4` | `time 0.3` | time crate has soundness caveats with `UtcOffset::current_local_offset` in multi-threaded programs. mesh-rt is multi-threaded (actor scheduler). |
| hex inline impl | `hex 0.4` crate | Hex is 3 lines of Rust. `hex::encode` is `bytes.iter().map(|b| format!("{:02x}", b)).collect()`. Adding a crate dependency for this is wasteful. Add the crate only if performance profiling shows hex is a bottleneck. |
| ureq 3.x upgrade | reqwest 0.12 | reqwest requires async/await (Tokio) in calling code. mesh-rt is synchronous blocking I/O — actor coroutines, not async/await. Switching would require redesigning all HTTP callsites and adding Tokio runtime management in the actor scheduler. |
| ureq 3.x upgrade | hyper 1.x direct | hyper is too low-level — manual chunked decoding, header parsing, redirect handling, keep-alive management. ureq handles all of this. |
| axum 0.8 for registry backend | actix-web 4.x | actix-web is ~10-15% faster under extreme load but the registry backend is not performance-critical. axum shares the tokio workspace dep already present, uses Tower middleware, and is maintained by the tokio-rs team. |
| axum 0.8 for registry backend | Mesh's own HTTP server | Dogfooding is tempting but premature — Mesh's HTTP server is synchronous, lacks axum's middleware ecosystem, and has no async/await integration. Use Mesh's server for user examples only. |
| VitePress for packages site | Next.js / Nuxt | Project already has VitePress for docs site. Reusing the same toolchain eliminates a new framework, new build pipeline, and new deployment target. |
| sqlx 0.8 for registry DB | Mesh's own ORM | Mesh ORM uses synchronous PostgreSQL wire protocol. The axum registry backend is async. sqlx is async-native with compile-time checked queries. Migrate to Mesh ORM if/when Mesh gains async support. |
| local fs for tarball storage (v14.0) | S3 / object_store | Start simple. Design the storage layer against `object_store 0.11`'s trait from day one but use local filesystem for the initial deployment. |

---

## What NOT to Add

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `hex = "0.4"` crate | Trivial functionality (3 lines of Rust) — no new dep needed | Inline: `bytes.iter().map(\|b\| format!("{:02x}", b)).collect()` |
| `chrono-tz` | Adds ~2MB binary bloat for IANA timezone DB not needed in v14.0 | Plain `chrono` (UTC timestamps only) |
| `openssl` as new direct dep | openssl-sys is already present for musl targets (vendored). Adding it directly risks conflicts with ring/rustls TLS stack. | ring + rustls already provide all needed crypto primitives |
| `reqwest` | async-only, conflicts with synchronous actor model in mesh-rt | ureq 3.x with Agent pooling |
| `rocket` or `warp` for registry | Not in existing dep tree, worse Tower/middleware integration than axum | axum 0.8 |
| `diesel` for registry DB | Diesel is synchronous, incompatible with axum async handlers without spawn_blocking wrappers | sqlx (async-native) |
| Separate npm project for packages site | Creates parallel toolchain to existing website/ stack | Extend existing VitePress site |
| `cargo-tarpaulin` | Linux-only ptrace backend (x86_64), unreliable on macOS; Mesh CI targets both | cargo-llvm-cov (cross-platform LLVM instrumentation) |

---

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| `ureq 3.2` | `rustls 0.23`, `ring 0.17` | ureq 3.x defaults to rustls with ring provider — exact match with mesh-rt's existing TLS stack. Zero conflict. |
| `uuid 1.21` | `rand 0.9` | uuid v4 feature uses rand as random source. rand 0.9 already locked in mesh-rt. |
| `chrono 0.4.42` | `serde 1` | serde feature aligns with workspace serde 1.x. |
| `axum 0.8.8` | `tokio 1`, `tower 0.5`, `hyper 1` | axum 0.8 requires tokio 1 (workspace dep) and upgraded to hyper 1.x internally. No conflict with mesh-rt since mesh-rt uses ureq (blocking), not hyper directly. |
| `sqlx 0.8` | `tokio 1` | Async runtime match with workspace tokio dep. |
| `tar 0.4` | `flate2 1` | Standard pairing for .tar.gz archives. |

---

## Sources

- [docs.rs/uuid/latest](https://docs.rs/uuid/latest/uuid/) — uuid 1.21.0, v4 feature, rand 0.9 backend confirmed — HIGH confidence
- [docs.rs/ureq/latest](https://docs.rs/ureq/latest/ureq/) — ureq 3.2.0, Agent pooling, Body streaming API confirmed — HIGH confidence
- [docs.rs/ureq/latest/ureq/struct.Body.html](https://docs.rs/ureq/latest/ureq/struct.Body.html) — `into_reader()`, `as_reader()`, `with_config().limit()` API — HIGH confidence
- [crates.io/crates/chrono](https://crates.io/crates/chrono) — chrono 0.4.42, 392M downloads, multi-thread soundness fix in 0.4.20+ — HIGH confidence
- [tokio.rs/blog/2025-01-01-announcing-axum-0-8-0](https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0) — axum 0.8.8 latest, Tower integration, tokio-rs maintained — HIGH confidence
- [github.com/RustCrypto/MACs/tree/master/hmac](https://github.com/RustCrypto/MACs/tree/master/hmac) — hmac 0.12 current stable — HIGH confidence
- [github.com/taiki-e/cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) — cross-platform LLVM coverage, supports cargo test, proc-macros, doc tests — HIGH confidence
- [github.com/rust-lang/crates.io](https://github.com/rust-lang/crates.io) — crates.io uses axum backend — MEDIUM confidence (secondary source)
- Cargo.lock direct inspection — sha2 0.10.9, hmac 0.12.1, base64 0.22.1, ureq 2.12.1 already locked — HIGH confidence

---

*Stack research for: Mesh v14.0 — stdlib crypto/date/encoding, HTTP client, testing framework, package registry*
*Researched: 2026-02-28*

# Feature Research

**Domain:** Programming language ecosystem expansion (stdlib crypto/datetime/encoding + HTTP client + testing + package registry)
**Researched:** 2026-02-28
**Confidence:** HIGH

---

## Context: Mesh v14.0 Feature Scope

This research covers six feature areas being added in v14.0. Each section maps
"expected behavior" from comparable ecosystems to concrete decisions for Mesh.

**Key existing-dep fact:** `sha2 = "0.10"`, `hmac = "0.12"`, `base64 = "0.22"`,
`rand = "0.9"`, `ureq = "2"` are already present in `compiler/mesh-rt/Cargo.toml`.
Crypto and encoding work is primarily Rust `extern "C"` wrapper code + Mesh-side
API design, not new dependency acquisition. DateTime is the one area requiring a
new dep (`chrono = "0.4"`).

---

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `Crypto.sha256(s)` -> hex String | Standard primitive in every lang stdlib; required for HMAC verification, content addressing | LOW | `sha2` crate already in mesh-rt; need `extern "C"` wrapper + Mesh stdlib binding |
| `Crypto.sha512(s)` -> hex String | Stronger variant; some APIs require it | LOW | Same crate, different digest size |
| `Crypto.hmac_sha256(key, msg)` -> hex String | API webhook verification, JWT signing, distributed node auth (already used internally) | LOW | `hmac` crate already in mesh-rt; expose as user-callable stdlib |
| `Crypto.hmac_sha512(key, msg)` -> hex String | Stronger HMAC variant | LOW | Same pattern as hmac_sha256 |
| `Crypto.secure_compare(a, b)` -> Bool | Timing-safe string comparison; prevents timing attacks on API tokens | LOW | Use `hmac` crate's `verify_slice` which is constant-time; critical for security correctness |
| `Crypto.uuid4()` -> String | Row IDs, idempotency keys, session tokens; Mesher already uses UUIDs via PG's `gen_random_uuid()` | LOW | `rand` crate already present; format 128-bit random as canonical UUID string |
| `Base64.encode(s)` -> String | API tokens, file uploads, binary data in JSON payloads | LOW | `base64` crate already present; standard alphabet |
| `Base64.decode(s)` -> Result<String, String> | Decode JWT headers, HTTP Basic auth, binary blobs | LOW | Returns Result because input may be malformed |
| `Base64.encode_url(s)` -> String | JWT tokens require URL-safe alphabet; common in web APIs | LOW | URL-safe alphabet replaces `+` with `-` and `/` with `_` |
| `Base64.decode_url(s)` -> Result<String, String> | Parse JWT tokens | LOW | URL-safe decode |
| `Hex.encode(s)` -> String | Display hash digests, binary data inspection | LOW | Thin wrapper; lowercase hex string of bytes |
| `Hex.decode(s)` -> Result<String, String> | Parse hex-encoded keys, digests from external sources | LOW | Validates hex character set |
| `DateTime.utc_now()` -> DateTime | Current timestamp for created_at, updated_at fields; every web application needs this | MEDIUM | New `chrono = "0.4"` dep; DateTime is an opaque GC-heap handle (same pattern as Regex) |
| `DateTime.from_iso8601(s)` -> Result<DateTime, String> | Parse timestamps from JSON API bodies, database strings | MEDIUM | Parses RFC 3339 / ISO 8601 extended format |
| `DateTime.to_iso8601(dt)` -> String | Serialize timestamps to JSON responses | MEDIUM | Produces `"2024-01-15T14:30:00Z"` format |
| `DateTime.from_unix(Int)` -> DateTime | Convert stored Unix timestamps (database integer columns) to DateTime | LOW | Wrap chrono's `from_timestamp` |
| `DateTime.to_unix(dt)` -> Int | Store DateTime as integer in database | LOW | Wrap chrono's `timestamp()` |
| `DateTime.add(dt, Int, unit)` -> DateTime | Compute expiry times, scheduling offsets, TTL calculations | MEDIUM | Units: `:second`, `:minute`, `:hour`, `:day` (avoid month/year — calendar complexity) |
| `DateTime.diff(dt1, dt2, unit)` -> Int | Compute age, elapsed time, rate limiting windows | MEDIUM | Signed difference; dt1 - dt2 |
| `DateTime.before?(dt1, dt2)` / `DateTime.after?` -> Bool | Expiry checks, sorting | LOW | Boolean comparisons; complements `compare` |
| `Http.build(:get/:post/:put/:delete, url)` -> Request | Fluent builder entry point; current `Http.get(url)` / `Http.post(url, body)` are not composable | MEDIUM | Returns a Request value (opaque handle); pipe-compatible |
| `Http.header(req, k, v)` -> Request | Set custom headers (Authorization, Content-Type, etc.) | LOW | Chainable; returns modified Request |
| `Http.body(req, s)` -> Request | Set request body for POST/PUT | LOW | Chainable |
| `Http.timeout(req, ms)` -> Request | Per-request timeout control | LOW | Chainable; maps to ureq timeout |
| `Http.send(req)` -> Result<Response, String> | Execute the built request | MEDIUM | Response struct: `{ status :: Int, body :: String, headers :: Map<String, String> }` |
| `meshc test` runner with `*.test.mpl` discovery | Tests must run via compiler CLI; no external test runner dependency | MEDIUM | New subcommand; discovers test files recursively, compiles + runs each, aggregates results |
| `assert expr` | Fundamental test assertion; halts test with useful message on failure | LOW | ExUnit-style; show expression source, value at failure |
| `assert_eq a, b` | Equality assertion with diff output | LOW | Show expected vs actual |
| `assert_ne a, b` | Inequality assertion | LOW | Inversion of assert_eq |
| `assert_raises fn` | Verify that a function panics or propagates an error | LOW | Catch panic from test function |
| Test pass/fail output with file + line info | Failure messages must be actionable | MEDIUM | Show `test_file.mpl:42: assertion failed` |
| `mesh.toml` manifest format | Declare package metadata and dependencies | MEDIUM | `mesh-pkg` crate already exists; extend with standard fields |
| `meshpkg publish` | Upload package to hosted registry | HIGH | Requires auth token, tarball creation, registry HTTP API |
| `meshpkg install <name>` | Fetch and install a package | HIGH | Download tarball from registry, extract to project deps directory |
| `meshpkg search <query>` | Find packages by name/keyword | MEDIUM | Query registry search endpoint, display results |
| Registry hosted site: browse + search | Public discoverability; without this the registry is unusable | HIGH | List packages by popularity/recency, full-text search |
| Registry per-package page | Rendered README, version history, install command | HIGH | hex.pm / crates.io standard: README in Markdown, all published versions listed |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| HTTP streaming via callback | Process large responses (AI streaming, file downloads) without buffering entire body in memory | MEDIUM | `Http.stream(req, fn chunk -> ... end)`; each chunk delivered to callback; actor-compatible |
| HTTP client handle (keep-alive reuse) | Avoid per-request TCP+TLS handshake; significant latency improvement for high-frequency API calls | MEDIUM | `Http.client()` -> Client handle; `Http.send_with(client, req)` reuses connections; ureq 2.x has built-in pooling |
| `describe "..." do ... end` grouping | Organizes large test files by feature; ExUnit and RSpec both have this; improves failure output | LOW | Syntactic grouping only; no new test semantics required |
| Actor mock via `Test.mock_actor` | In actor model languages, mocking actors (not just functions) is the core concurrency testing need | HIGH | Spawn an actor with a custom message handler; return its PID for use in tests; `Test.mock_actor(fn msg -> ... end)` |
| `setup do ... end` / `teardown` blocks | Per-test setup/cleanup without manual repetition | LOW | ExUnit `setup` pattern; run before each test in a describe block |
| `assert_receive pattern, timeout` | Test actor message delivery; critical for verifying actor behavior | MEDIUM | Check the test actor's mailbox for a pattern within timeout ms; key for actor model testing |
| Test module parallelism (`meshc test --jobs N`) | Faster test suite execution on multi-core; run N test files concurrently | MEDIUM | Fork N compiler processes; aggregate results; tests within a file remain sequential |
| Package `mesh.lock` lockfile | Reproducible builds; same dependency versions across environments | MEDIUM | Generated automatically by `meshpkg install`; committed to source control |
| `meshpkg outdated` | List packages with newer versions available | LOW | Query registry API, compare against mesh.lock versions |
| Package categories (validated list) | Structured discoverability; search by category like crates.io | LOW | Registry defines valid category slugs; max 5 per package |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Timezone-aware datetime (full tz database) | "Complete" datetime support | IANA tz database is 50+ KB and requires updates; DST ambiguity makes month/year arithmetic lossy; bloats CLI tools | Ship UTC + Unix timestamps. Defer timezone-aware operations to an optional future `DateTime.Tz` module with explicit opt-in |
| `Crypto.md5()` / `Crypto.sha1()` | Familiar from legacy codebases | MD5 and SHA-1 are cryptographically broken; stdlib inclusion normalizes insecure use; causes false sense of security | Explicitly absent from stdlib. Developers who need MD5 for non-security checksums can note this is intentional. Document why in stdlib docs |
| Async/Future-based HTTP streaming | "Modern" async API patterns | Mesh explicitly rejects colored functions (async/await); introducing Future types contradicts the actor model philosophy | Callback-based streaming within an actor; `Http.stream` delivers chunks synchronously to a callback; the actor is the unit of concurrency |
| Test mocking via global function replacement | "Easy" mocking without dependency injection | Global mutation breaks test isolation; parallel test modules would interfere with each other | Behavior-based mocking: define a trait, pass mock implementation as parameter. "Mocks as noun" pattern (Elixir Mox philosophy) |
| Test parallelism at the individual test level | "Maximum speed" | Tests within a Mesh actor context share the scheduler; parallel tests touching shared state cause flaky failures | Parallelize at module level (`meshc test --jobs N`); tests within a file run sequentially and reliably |
| Mutable published package versions | "Hotfix a bad release without bumping version" | Breaks reproducible builds; users with lockfiles get different code silently | Use `meshpkg yank <name>@<version>` to mark a version as deprecated (still downloadable for existing lockfiles, blocked for new installs) |
| Automatic dependency updates | "Stay current automatically" | Pulls in breaking changes without review; security theater without audit | Provide `meshpkg outdated` to surface available updates; updates are always manual and intentional |
| strftime format strings for DateTime | "Flexible" custom date formatting | strftime format strings are notoriously cryptic (`%Y-%m-%dT%H:%M:%SZ`); hard to read and write; ISO 8601 covers 95% of use cases | Ship `to_iso8601` for standard format. Add `DateTime.format(dt, pattern)` as a v1.x feature once demand is established |
| SHA-256 of raw bytes input | "Binary hashing" | Mesh has no binary/bytes type; accepting raw bytes would require a new type or unsafe FFI casting | Accept String input; treat as UTF-8 bytes internally; callers who need binary hashing work with hex-encoded strings |

---

## Feature Dependencies

```
[Crypto.sha256 / sha512]
    depends on: sha2 crate (already in mesh-rt/Cargo.toml) — no new deps
    no Mesh feature dependencies

[Crypto.hmac_sha256 / hmac_sha512]
    depends on: hmac crate (already present)
    no Mesh feature dependencies

[Crypto.uuid4]
    depends on: rand crate (already present)
    no Mesh feature dependencies

[Crypto.secure_compare]
    depends on: hmac crate (already present)
    no Mesh feature dependencies

[Base64.encode / decode]
    depends on: base64 crate (already present)
    no Mesh feature dependencies

[Hex.encode / decode]
    depends on: stdlib string formatting
    no Mesh feature dependencies

[DateTime.* (all)]
    depends on: chrono = "0.4" (NEW — not yet in Cargo.toml)
    DateTime is an opaque heap pointer (same pattern as Regex, DB connections)
    no other Mesh feature depends on DateTime (independent)

[Http.build / header / body / timeout / send]
    depends on: ureq = "2" (already present)
    enhances: existing Http.get/Http.post (those remain as convenience wrappers)

[Http.stream]
    requires: Http.build (need a Request struct to attach streaming to)
    depends on: ureq 2.x streaming reader (into_reader() API)

[Http.client / send_with (keep-alive)]
    requires: Http.build (client handle concept flows from builder pattern)
    depends on: ureq 2.x connection reuse (Client::new().agent() pattern)

[meshc test runner]
    requires: new meshc subcommand (compiler CLI change)
    requires: test runtime primitives in mesh-rt (assert panic, result reporting)
    is prerequisite for: ALL other testing features

[assert / assert_eq / assert_ne / assert_raises]
    requires: meshc test runner (these are runtime primitives for tests)
    no dependencies on each other

[describe "..." do ... end]
    requires: meshc test runner
    optional: syntactic grouping only; does not require setup blocks

[setup do ... end / teardown]
    requires: describe blocks (scoped to describe context)

[assert_receive pattern, timeout]
    requires: meshc test runner
    requires: actor mailbox access in test context (existing actor API)

[Test.mock_actor]
    requires: meshc test runner
    requires: actor spawn API (already exists in mesh-rt)
    requires: test isolation semantics (each test gets clean actor context)

[meshc test --coverage]
    requires: meshc test runner
    requires: LLVM instrumentation pass (significant new compiler work)
    CAUTION: High implementation risk; consider deferring within v14.0

[mesh.toml manifest]
    depends on: toml crate (already in mesh-pkg/Cargo.toml)
    is prerequisite for: meshpkg CLI, hosted registry

[meshpkg publish]
    requires: mesh.toml manifest (metadata to send)
    requires: hosted registry HTTP API (publish endpoint)
    requires: auth token management (meshpkg login)

[meshpkg install <name>]
    requires: hosted registry HTTP API (download endpoint)
    requires: mesh.toml manifest (dependency section to update)
    requires: mesh.lock (lockfile to write)

[meshpkg search <query>]
    requires: hosted registry HTTP API (search endpoint)
    can be implemented before install (only needs read API)

[Package registry hosted site]
    requires: mesh.toml manifest (defines metadata structure)
    is a separate Mesh web application (not part of compiler)
    can be built in parallel with CLI after manifest format is decided
```

### Dependency Notes

- **Crypto requires zero new Rust deps.** `sha2`, `hmac`, `base64`, `rand` are already compiled into `mesh-rt`. Adding user-facing Mesh APIs is purely wrapper code: add `extern "C"` functions, register in typechecker as stdlib functions, no cargo changes needed.
- **DateTime requires one new dep.** `chrono = "0.4"` must be added to `compiler/mesh-rt/Cargo.toml`. Chrono is the canonical Rust datetime crate (48M+ downloads/month). DateTime values are opaque u64 pointers to chrono structs on the GC heap, following the established pattern for Regex and DB connection handles.
- **Http builder is a refactor, not a rewrite.** The existing `mesh_http_get` and `mesh_http_post` functions in `http/client.rs` use ureq 2.x already. The builder wraps `ureq::RequestBuilder`. Existing `Http.get` and `Http.post` become thin wrappers over the builder for backward compatibility.
- **meshc test runner is the single biggest prerequisite.** All 5 other testing features require it. Build the runner first, then layer assertions, describe blocks, actor mocking, and coverage on top.
- **Package registry hosted site is independent of the compiler.** It is a separate web application (likely written in Mesh itself, similar to how Mesher was built). It does not block CLI or manifest development. Design the manifest format and API contract first.
- **Coverage reporting has high implementation risk.** LLVM coverage instrumentation (`-fprofile-instr-generate`, `llvm-profdata`, `llvm-cov`) requires accessing LLVM APIs through Inkwell. This is feasible but non-trivial. If it blocks the milestone, defer to v14.1.

---

## MVP Definition

### Launch With (v1 — all committed v14.0 requirements)

The following are all required per PROJECT.md v14.0 target features.

**Crypto stdlib:**
- [x] `Crypto.sha256(s)` -> String (hex) — required for API signature verification
- [x] `Crypto.sha512(s)` -> String (hex) — stronger hashing
- [x] `Crypto.hmac_sha256(key, msg)` -> String (hex) — API authentication
- [x] `Crypto.hmac_sha512(key, msg)` -> String (hex) — stronger HMAC
- [x] `Crypto.secure_compare(a, b)` -> Bool — constant-time comparison (security requirement)
- [x] `Crypto.uuid4()` -> String — UUID v4 generation

**Encoding:**
- [x] `Base64.encode(s)` -> String — standard base64
- [x] `Base64.decode(s)` -> Result<String, String> — standard decode
- [x] `Base64.encode_url(s)` -> String — URL-safe base64
- [x] `Base64.decode_url(s)` -> Result<String, String> — URL-safe decode
- [x] `Hex.encode(s)` -> String — hex encoding
- [x] `Hex.decode(s)` -> Result<String, String> — hex decoding

**DateTime:**
- [x] `DateTime.utc_now()` -> DateTime — current UTC timestamp
- [x] `DateTime.from_iso8601(s)` -> Result<DateTime, String> — parse ISO 8601
- [x] `DateTime.to_iso8601(dt)` -> String — format ISO 8601
- [x] `DateTime.from_unix(Int)` -> DateTime — from Unix timestamp
- [x] `DateTime.to_unix(dt)` -> Int — to Unix timestamp
- [x] `DateTime.add(dt, Int, unit)` -> DateTime — arithmetic (second/minute/hour/day)
- [x] `DateTime.diff(dt1, dt2, unit)` -> Int — time difference
- [x] `DateTime.before?(dt1, dt2)` / `DateTime.after?(dt1, dt2)` -> Bool — comparisons

**HTTP client improvements:**
- [x] `Http.build(method, url)` -> Request — builder entry point
- [x] `Http.header(req, k, v)` -> Request — add header
- [x] `Http.body(req, s)` -> Request — set body
- [x] `Http.timeout(req, ms)` -> Request — set timeout
- [x] `Http.send(req)` -> Result<Response, String> — execute request
- [x] `Http.stream(req, fn chunk -> ... end)` -> Result<Unit, String> — streaming
- [x] `Http.client()` -> Client; `Http.send_with(client, req)` — connection keep-alive/reuse

**Testing framework:**
- [x] `meshc test` — discovers `*.test.mpl`, compiles, runs, reports pass/fail
- [x] `assert expr` — basic boolean assertion
- [x] `assert_eq a, b` — equality with diff output
- [x] `assert_ne a, b` — inequality assertion
- [x] `assert_raises fn` — exception/panic assertion
- [x] `describe "..." do ... end` — test grouping
- [x] `Test.mock_actor(fn msg -> ... end)` -> Pid — mock actor for concurrency testing
- [x] `meshc test --coverage` — coverage reporting

**Package registry:**
- [x] `mesh.toml` manifest format — name, version, description, license, dependencies
- [x] `mesh.lock` lockfile — auto-generated, reproducible builds
- [x] `meshpkg publish` — publish to hosted registry
- [x] `meshpkg install <name>` — install from registry
- [x] `meshpkg search <query>` — search registry
- [x] Hosted packages site — browse, search, per-package page with README + versions

### Add After Validation (v1.x — post v14.0)

- [ ] `DateTime.format(dt, pattern)` with strftime-style format strings — trigger: custom display needs emerge from user feedback
- [ ] `Crypto.pbkdf2(password, salt, iterations)` — trigger: password hashing use cases appear in user apps
- [ ] Timezone-aware datetime (`DateTime.shift_zone`) — trigger: user volume requesting it; requires tz database dep
- [ ] `meshpkg update` / `meshpkg outdated` — trigger: package ecosystem matures enough to have outdated packages
- [ ] Private/org package namespaces on registry — trigger: enterprise adoption signals
- [ ] Coverage delta reporting (compare against baseline) — trigger: CI integration demand

### Future Consideration (v2+)

- [ ] Full IANA timezone database embedding — defer: 50+ KB binary size cost; UTC satisfies most use cases
- [ ] Ed25519 / RSA signing and verification — defer: specialized crypto beyond stdlib scope
- [ ] Property-based testing / fuzzing integration — defer: significant framework work
- [ ] Package registry billing / private packages — defer: not a v1 goal
- [ ] `meshpkg audit` (vulnerability scanning) — defer: requires CVE database integration

---

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Crypto stdlib (sha256/hmac/uuid) | HIGH | LOW | P1 |
| Encoding (base64/hex) | HIGH | LOW | P1 |
| DateTime (now/parse/format/add/diff) | HIGH | MEDIUM | P1 |
| `mesh.toml` manifest format | HIGH | LOW | P1 |
| `meshc test` runner + assertions | HIGH | MEDIUM | P1 |
| Http builder API | MEDIUM | MEDIUM | P1 |
| `meshpkg` CLI (publish/install/search) | HIGH | HIGH | P1 |
| Package hosted site | HIGH | HIGH | P1 |
| Http streaming | MEDIUM | MEDIUM | P2 |
| Http keep-alive client handle | LOW | LOW | P2 |
| `describe` blocks | MEDIUM | LOW | P2 |
| `assert_receive` for actor testing | HIGH | MEDIUM | P2 |
| `setup do ... end` | MEDIUM | LOW | P2 |
| `Test.mock_actor` | MEDIUM | HIGH | P2 |
| `meshc test --coverage` | MEDIUM | HIGH | P3 |

**Priority key:**
- P1: Must have for v14.0; blocks the milestone if missing
- P2: Should have; included in v14.0 once P1 features are done
- P3: Nice to have; target v14.0 if time permits; defer if coverage implementation is too risky

---

## Comparable Ecosystem Analysis

### Crypto API Conventions

| Language | SHA-256 | HMAC | UUID |
|----------|---------|------|------|
| Elixir | `:crypto.hash(:sha256, data)` -> binary; `Base.encode16(h)` for hex | `:crypto.mac(:hmac, :sha256, key, msg)` | `:crypto.strong_rand_bytes(16)` + format |
| Python | `hashlib.sha256(data.encode()).hexdigest()` | `hmac.new(key, msg, sha256).hexdigest()` | `str(uuid.uuid4())` |
| Go | `sha256.Sum256([]byte(s))` + `hex.EncodeToString(h[:])` | `hmac.New(sha256.New, key)` | third-party `github.com/google/uuid` |
| Node.js | `crypto.createHash('sha256').update(s).digest('hex')` | `crypto.createHmac('sha256', key).update(msg).digest('hex')` | `crypto.randomUUID()` |
| **Mesh** | `Crypto.sha256(s)` -> String | `Crypto.hmac_sha256(key, msg)` -> String | `Crypto.uuid4()` -> String |

**Decision**: Return hex strings directly (not raw bytes). Mesh has no binary/bytes type; hex strings compose naturally with string interpolation and are the format users actually need. Functions like `sha256` are total (cannot fail on String input). `hmac` functions are also total given valid key and message strings.

### DateTime API Conventions

| API | Elixir (DateTime) | Rust (chrono) | Python (datetime) | Mesh (proposed) |
|-----|-------------------|---------------|-------------------|-----------------|
| Current UTC | `DateTime.utc_now()` | `Utc::now()` | `datetime.utcnow()` | `DateTime.utc_now()` |
| ISO 8601 parse | `DateTime.from_iso8601("...")` | `DateTime::parse_from_rfc3339` | `datetime.fromisoformat` | `DateTime.from_iso8601(s)` -> Result<DateTime, String> |
| ISO 8601 format | `DateTime.to_iso8601(dt)` | `dt.to_rfc3339()` | `dt.isoformat()` | `DateTime.to_iso8601(dt)` -> String |
| Add time | `DateTime.add(dt, 3600, :second)` | `dt + Duration::hours(1)` | `dt + timedelta(hours=1)` | `DateTime.add(dt, 3600, :second)` |
| Unix timestamp | `DateTime.to_unix(dt)` | `dt.timestamp()` | `dt.timestamp()` | `DateTime.to_unix(dt)` -> Int |
| Difference | `DateTime.diff(dt1, dt2, :second)` | `(dt1 - dt2).num_seconds()` | `(dt1 - dt2).total_seconds()` | `DateTime.diff(dt1, dt2, :second)` -> Int |
| Compare | `DateTime.compare(dt1, dt2)` -> :lt/:eq/:gt | `dt1.cmp(&dt2)` | `dt1 < dt2` | `DateTime.before?(dt1, dt2)` -> Bool |

**Decision**: Follow Elixir DateTime API naming conventions exactly. Mesh already has Elixir-style idioms throughout. `from_iso8601` returns `Result<DateTime, String>` for parse failures — consistent with Mesh error handling patterns. `add` units use atoms (`:second`, `:minute`, `:hour`, `:day`) matching Elixir. Avoid `:month` / `:year` units — calendar arithmetic (DST, variable month lengths, leap years) requires full tz database.

### Base64 / Hex Encoding Conventions

| API | Elixir (Base) | Python (base64) | Node.js | Mesh (proposed) |
|-----|---------------|-----------------|---------|-----------------|
| Base64 encode | `Base.encode64(s)` -> String | `base64.b64encode(b).decode()` | `Buffer.from(s).toString('base64')` | `Base64.encode(s)` -> String |
| Base64 decode | `Base.decode64(s)` -> {:ok, s} or :error | `base64.b64decode(s)` | `Buffer.from(s, 'base64')` | `Base64.decode(s)` -> Result<String, String> |
| URL-safe encode | `Base.url_encode64(s)` | `base64.urlsafe_b64encode(b).decode()` | custom | `Base64.encode_url(s)` -> String |
| Hex encode | `Base.encode16(s, case: :lower)` | `s.encode().hex()` | `Buffer.from(s).toString('hex')` | `Hex.encode(s)` -> String |
| Hex decode | `Base.decode16(s, case: :mixed)` | `bytes.fromhex(s)` | `Buffer.from(s, 'hex')` | `Hex.decode(s)` -> Result<String, String> |

**Decision**: `Base64` and `Hex` as separate modules (not a combined `Encoding` module). Decode always returns `Result<String, String>` because malformed input must be handled. Encode is always total. `Hex.encode` produces lowercase hex (the overwhelming convention for hash digests and cryptographic output).

### HTTP Client Builder Conventions

| API | reqwest (Rust) | Finch (Elixir) | Python requests | Mesh (proposed) |
|-----|----------------|----------------|-----------------|-----------------|
| Create request | `client.get(url)` | `Finch.build(:get, url, headers, body)` | `requests.get(url)` | `Http.build(:get, url)` |
| Add header | `.header(k, v)` | headers in build | `headers={k: v}` | `|> Http.header(k, v)` |
| Set body | `.body(s)` | body in build | `data=s` | `|> Http.body(s)` |
| Set timeout | `.timeout(dur)` | `receive_timeout: ms` | `timeout=s` | `|> Http.timeout(ms)` |
| Execute | `.send().await` | `Finch.request(req, Finch)` | (immediate in requests.get) | `|> Http.send()` |
| Stream | `.bytes_stream()` | streaming accumulator | `stream=True` iterator | `Http.stream(req, fn chunk -> ... end)` |
| Keep-alive | `Client` reuse | Finch connection pool | `Session` reuse | `Http.client()` handle; `Http.send_with(client, req)` |

**Decision**: `Http.build(:method, url)` matches Mesh pipe idioms. The `|>` pipe operator makes builder APIs feel native. Return a `Response` struct (not a raw string) from `Http.send()`: `{ status :: Int, body :: String, headers :: Map<String, String> }`. Keep existing `Http.get(url)` and `Http.post(url, body)` as backward-compatible convenience functions — they delegate to the builder internally.

### Testing Framework Conventions

| Concept | ExUnit (Elixir) | EUnit (Erlang) | pytest (Python) | Mesh (proposed) |
|---------|-----------------|----------------|-----------------|-----------------|
| File convention | `*_test.exs` in `test/` | `_test()` functions in module | `test_*.py` in any dir | `*.test.mpl` anywhere in project |
| Runner | `mix test` | `eunit:test(Module)` | `pytest` | `meshc test` |
| Basic assert | `assert expr` | `?assert(Expr)` | `assert expr` | `assert expr` |
| Equality | `assert a == b` (or `assert_eq`) | `?assertEqual(A, B)` | `assert a == b` | `assert_eq a, b` |
| Exception | `assert_raise(Error, fn)` | `?assertException(class, term, expr)` | `pytest.raises(Error)` | `assert_raises fn` |
| Grouping | `describe "..." do` | test generators | `class TestFoo:` | `describe "..." do ... end` |
| Setup | `setup do ... end` | `{setup, Setup, Tests}` | `@pytest.fixture` | `setup do ... end` |
| Actor msg | `assert_receive pattern, timeout` | manual mailbox | n/a | `assert_receive pattern, timeout` |
| Mock | Mox (behaviour + expect/stub) | `:meck` | `unittest.mock.patch` | `Test.mock_actor(fn msg -> end)` |
| Module concurrency | `async: true` per module | manual | `pytest-xdist` | `meshc test --jobs N` |

**Key decisions for Mesh testing:**

1. **File discovery**: `*.test.mpl` anywhere in the project directory tree (recursive). This is more flexible than ExUnit's fixed `test/` directory — Mesh projects may colocate tests with source.

2. **Test function identification**: Functions whose names start with `test_` (e.g., `fn test_login_success do ... end`). Tests inside `describe "..." do ... end` blocks inherit the describe name in failure output.

3. **`assert_receive` is critical for actors**: A test actor can receive messages. `assert_receive pattern, 5000` waits up to 5 seconds for a matching message in the test's mailbox. Essential for verifying that other actors sent the expected messages.

4. **Mock actors via spawn**: `Test.mock_actor(fn msg -> ... end)` spawns a new actor with the given message handler and returns its PID. Tests pass this PID to the system under test. No global function replacement needed — the actor model makes mocking compositional.

5. **Coverage reporting** (`--coverage`): Requires LLVM instrumentation. High implementation risk. If this is blocking, defer to a v14.1 phase.

### Package Registry Conventions

| Concept | Hex (Elixir) | Cargo (Rust) | npm (Node.js) | Mesh (proposed) |
|---------|--------------|--------------|---------------|-----------------|
| Manifest file | `mix.exs` | `Cargo.toml` | `package.json` | `mesh.toml` |
| Lockfile | `mix.lock` | `Cargo.lock` | `package-lock.json` | `mesh.lock` |
| Versioning | SemVer | SemVer (3-part required) | SemVer | SemVer (major.minor.patch required) |
| Publish | `mix hex.publish` | `cargo publish` | `npm publish` | `meshpkg publish` |
| Install | `mix deps.get` | `cargo add` + `cargo build` | `npm install` | `meshpkg install <name>` |
| Search | `mix hex.search` | `cargo search` | `npm search` | `meshpkg search <query>` |
| Auth | `mix hex.user auth` | `cargo login` | `npm login` | `meshpkg login` |
| Yank | `mix hex.retire` | `cargo yank` | `npm deprecate` | `meshpkg yank <name>@<ver>` |
| Registry site | hex.pm | crates.io | npmjs.com | packages.meshlang.dev |
| Per-package page | README, versions, downloads | README, versions, deps, MSRV | README, weekly downloads, dependents | README, versions, install command, license |

**Decision for `mesh.toml` format** (follows Cargo.toml conventions, which are clean and TOML-native):

```toml
[package]
name = "my-package"
version = "1.0.0"
description = "One-line summary"
license = "MIT"
authors = ["Name <email>"]
keywords = ["http", "web"]  # max 5, ASCII, alphanumeric
categories = ["web-programming"]  # validated list, max 5

[dependencies]
json-utils = "1.2"
http-client = "~> 2.0"
```

Version requirements use Cargo-style `"1.2"` (compatible with 1.x.x >= 1.2.0) and Hex-style `"~> 2.0"` (compatible with 2.x.x, not 3.x.x). `mesh.lock` is auto-generated on install/publish and should be committed to source control for applications (but not libraries, matching Cargo convention).

---

## Implementation Complexity Notes

### LOW complexity (thin wrappers over existing Rust deps)

These are primarily `extern "C"` function additions in `mesh-rt` + typechecker
registration in `mesh-typeck`. No new Rust dependencies. No compiler architecture
changes. Each represents roughly 50-150 LOC Rust + 20-50 LOC compiler registration.

- `Crypto.sha256 / sha512` — wrap `sha2::Sha256::digest` / `sha2::Sha512::digest`; hex-encode with stdlib format
- `Crypto.hmac_sha256 / hmac_sha512` — wrap `hmac::Hmac<Sha256>::new` + `finalize`; hex-encode
- `Crypto.secure_compare` — wrap `hmac::Mac::verify_slice` (constant-time)
- `Crypto.uuid4` — `rand::random::<u128>()` formatted as UUID v4 string
- `Base64.encode / encode_url` — wrap `base64::engine::general_purpose::STANDARD.encode`
- `Base64.decode / decode_url` — wrap decode; return Result
- `Hex.encode` — `format!("{:02x}", byte)` per byte
- `Hex.decode` — parse hex pairs; return Result
- `DateTime.from_unix / to_unix` — wrap `chrono::DateTime::from_timestamp` / `.timestamp()`
- `DateTime.before? / after?` — boolean comparisons on DateTime handles

### MEDIUM complexity (new dep, or non-trivial API design)

Approximately 300-600 LOC Rust each, plus compiler registration.

- **DateTime full API** — Add `chrono = "0.4"` to `mesh-rt/Cargo.toml`. DateTime is an opaque u64 pointer (GC heap-allocated `DateTime<Utc>` struct). Wrap `utc_now()`, `from_iso8601` (parse_from_rfc3339), `to_iso8601` (to_rfc3339), `add` (with unit dispatch), `diff`. The DateTime opaque pointer pattern follows the Regex and DB connection handle precedents in the codebase.
- **Http builder API** — Refactor `compiler/mesh-rt/src/http/client.rs`. Add a `MeshRequest` struct (URL, method, headers HashMap, body Option, timeout_ms). Add `extern "C"` functions for build, header, body, timeout. `Http.send` converts MeshRequest to ureq request, executes, returns MeshResponse. ~400 LOC Rust.
- **Http streaming** — ureq 2.x provides `response.into_reader()` returning an `impl Read`. Read in chunks (e.g., 8KB), call the Mesh callback function pointer with each chunk (following existing callback ABI used in iterators and query results). ~200 LOC Rust.
- **`meshc test` runner** — New subcommand in `compiler/meshc/src/main.rs`. File discovery (walkdir), compile each `*.test.mpl` with special `--test` flag that injects a `__run_tests()` entry point, execute compiled binary, capture output (pass/fail counts, failure details), aggregate. ~600 LOC Rust across meshc + mesh-rt test runtime.
- **`mesh.toml` manifest** — Extend `compiler/mesh-pkg/src/lib.rs`. `mesh-pkg` crate already has TOML parsing. Add `[package]` and `[dependencies]` deserialization. ~200 LOC Rust.

### HIGH complexity (significant new systems, cross-cutting concerns)

These require careful design and are the main risk areas for the milestone.

- **`Test.mock_actor`** (~400 LOC Rust + design): Spawning a mock actor is straightforward (existing `mesh_actor_spawn` API). The complexity is test isolation: each test needs a clean actor context, and the mock actor must be cleaned up after the test. Requires a test supervisor structure and careful mailbox semantics. `assert_receive` needs access to the test actor's mailbox.

- **`meshc test --coverage`** (HIGH risk): LLVM coverage instrumentation requires `-fprofile-instr-generate` flags, `llvm-profdata merge`, and `llvm-cov show`. Integrating this through Inkwell and the mesh-codegen pipeline is feasible but non-trivial (~1000 LOC Rust + LLVM API work). Risk: if blocked, defer to v14.1 phase.

- **`meshpkg` CLI** (~1000 LOC Rust): auth token management (`meshpkg login` stores token in `~/.mesh/credentials`), tarball creation from project directory (zip/tar + metadata), HTTP upload to registry API, HTTP download for install, local extraction to `~/.mesh/packages/` or project-local deps directory, version resolution from `mesh.toml` + `mesh.lock`.

- **Package registry hosted site** (open-ended): A separate Mesh web application. Not part of the compiler. Estimate ~1500-2000 LOC Mesh for the site backend + frontend. The site serves package metadata from a database (PostgreSQL via Mesh ORM), renders README from markdown, shows version history. Can be built in parallel with CLI once the API contract is decided. Time-box this: the site must be functional (browse, search, per-package page) but does not need to be feature-complete.

---

## Sources

- [ExUnit v1.19.5 documentation](https://hexdocs.pm/ex_unit/ExUnit.html) — file convention, runner, async:true, assert_receive
- [ExUnit.Assertions v1.19.5](https://hexdocs.pm/ex_unit/ExUnit.Assertions.html) — assert, refute, assert_raise, assert_receive signatures
- [Elixir DateTime v1.19.5](https://hexdocs.pm/elixir/DateTime.html) — from_iso8601, to_unix, add, diff, compare, before?, after? API — HIGH confidence
- [chrono Rust crate docs](https://docs.rs/chrono/latest/chrono/) — NaiveDateTime, DateTime<Utc>, parse_from_rfc3339, to_rfc3339, timestamp, num_seconds — HIGH confidence
- [reqwest ClientBuilder](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html) — builder pattern, keep-alive (90s idle default), streaming via bytes_stream — HIGH confidence
- [Hex package manager docs](https://hex.pm/docs/publish) — mix.exs fields, publish flow, hexdocs integration — HIGH confidence
- [Cargo manifest format](https://doc.rust-lang.org/cargo/reference/manifest.html) — Cargo.toml field design, keywords (max 5), categories (validated slugs) — HIGH confidence
- [RFC 4648 - Base64/Base16 Data Encodings](https://datatracker.ietf.org/doc/html/rfc4648) — canonical encoding standard, URL-safe variant, test vectors — HIGH confidence
- [Python hashlib docs](https://docs.python.org/3/library/hashlib.html) — algorithm-named constructor pattern, update/digest/hexdigest API — HIGH confidence
- [Go crypto/hmac package](https://pkg.go.dev/crypto/hmac) — constant-time comparison via hmac.Equal — HIGH confidence
- [EUnit documentation](https://www.erlang.org/doc/apps/eunit/chapter.html) — Erlang unit testing conventions, assert macros — HIGH confidence
- [Elixir School: Mox](https://elixirschool.com/en/lessons/testing/mox) — mocks-as-noun philosophy, behaviour-based mocking pattern — MEDIUM confidence
- [mesh-rt/Cargo.toml](compiler/mesh-rt/Cargo.toml) — confirmed existing deps (sha2, hmac, base64, rand, ureq); validated zero new deps needed for crypto/encoding — HIGH confidence
- [mesh-rt/src/http/client.rs](compiler/mesh-rt/src/http/client.rs) — confirmed existing ureq 2.x usage, current `get`/`post` flat functions — HIGH confidence

---

*Feature research for: Mesh v14.0 Ecosystem Expansion*
*Researched: 2026-02-28*

# Pitfalls Research

**Domain:** Ecosystem expansion for an existing compiled programming language — crypto stdlib, date/time, HTTP client improvements, test framework, package registry
**Researched:** 2026-02-28
**Confidence:** HIGH (direct Mesh source analysis + ecosystem research across all five domains)

---

## Critical Pitfalls

Mistakes that cause rewrites, multi-day debugging sessions, or fundamental design lock-in.

---

### Pitfall 1: Duplicating Crypto Dependencies Already in mesh-rt

**What goes wrong:**

The developer adds new crates for crypto operations — e.g., pulling in `sha3`, `aes-gcm`, or `blake3` — without noticing that `sha2`, `hmac`, `ring`, `base64`, and `pbkdf2` are already compiled into `mesh-rt`. The result is two separate crypto ecosystems living inside the same binary: one used by the PG auth/TLS subsystem and one used by the new stdlib module. Version conflicts arise when the ecosystem crates (`sha2 0.10` vs a hypothetical `sha2 0.11`) do not agree, and the binary bloats by 200-400 KB with duplicate crypto codegen.

**Why it happens:**

The new stdlib developer looks at what is "needed" for the feature (SHA-256, HMAC-SHA256, UUID v4), finds crates, and adds them to `Cargo.toml` without auditing what is already there. `mesh-rt/Cargo.toml` already lists `sha2 = "0.10"`, `hmac = "0.12"`, `ring = "0.17"`, `base64 = "0.22"`, and `rand = "0.9"`. Every v14.0 crypto operation (SHA-256, SHA-512, HMAC, UUID v4, hex encoding) can be implemented using exactly these existing dependencies.

**How to avoid:**

Before adding any crate for crypto stdlib, read `compiler/mesh-rt/Cargo.toml` top-to-bottom and map each needed operation to an existing dep:
- SHA-256 / SHA-512 → `sha2` (already present)
- HMAC-SHA256 / HMAC-SHA512 → `hmac` + `sha2` (already present)
- UUID v4 → `rand` (already present) — generate 16 random bytes, set version/variant bits per RFC 4122
- Base64 encode/decode → `base64 = "0.22"` (already present; replaces `base64ct` used for PG auth)
- Hex encode → implement inline (8 lines) or use `ring`'s utilities (already present)
- Constant-time compare → `ring::constant_time` or `hmac::Mac::verify_slice` (already present and already security-reviewed for the cluster auth use case)

Zero new crates needed for the entire crypto stdlib module.

**Warning signs:**

Any PR that adds a new crate from the `RustCrypto` org for v14.0 should be challenged — the required functionality is already available. The exception is if a truly new capability is needed (e.g., AES encryption, which is not in any existing dep).

**Phase to address:** Crypto stdlib phase — the very first task is auditing `Cargo.toml` before writing a single line of code.

---

### Pitfall 2: Exposing Non-Constant-Time Comparison for Secrets

**What goes wrong:**

The crypto stdlib module exposes a `Crypto.compare(a, b)` function that internally uses `==` (which compiles to a short-circuit byte comparison). A user calls it to compare HMAC values or session tokens. The function leaks timing information: it exits earlier when bytes differ near the start, allowing an attacker to reconstruct a valid token one byte at a time via timing measurements.

This is not theoretical. A published security advisory for `curve25519-dalek` showed LLVM re-introducing branches into what was written as bitwise constant-time code. Using `==` directly is worse — it is explicitly a short-circuit comparison with no mitigations at all.

**Why it happens:**

The developer implements `compare` as a convenience function and does not flag it as a security primitive. String equality in Mesh compiles to `snow_string_eq` which is not constant-time. The function looks correct from a functional perspective — it returns the right answer — but leaks timing.

**How to avoid:**

Do NOT expose a generic `Crypto.compare` function. Instead:

1. For HMAC verification: use `hmac::Mac::verify_slice` from the existing `hmac` dep. This calls `subtle::ConstantTimeEq` internally and is the same code already used for the cluster HMAC-SHA256 cookie auth (v5.0 decision).
2. For raw digest comparison: call `ring::constant_time::verify_slices_are_equal` (already present).
3. Document in the Mesh stdlib that `Crypto.hmac_verify` should be used for all secret comparisons. Never suggest using `==` or `assert_eq` for HMAC or token comparison.

The extern C signature for secret comparison should accept two byte-slice representations and return a Bool via the constant-time path, with no way to call the variable-time path by mistake.

**Warning signs:**

Any implementation that calls Rust's `==` on two `&[u8]` slices containing cryptographic output is wrong. Any test that passes by comparing HMAC output with a string literal using `assert_eq` is exposing a variable-time comparison path.

**Phase to address:** Crypto stdlib phase — design the API surface before implementing anything. The API must make constant-time the only path for secret comparison.

---

### Pitfall 3: Representing Timestamps as Formatted Strings Instead of Unix Epoch Integers

**What goes wrong:**

The date/time stdlib stores timestamps as formatted strings (e.g., `"2026-02-28T12:00:00Z"`) rather than as integer Unix timestamps (seconds or milliseconds since epoch). Arithmetic operations (`add_seconds`, `diff`, duration comparison) then require parsing the string on every call. Sorting dates requires parsing. Comparing timestamps requires parsing. The result is a date/time module that is ergonomic for display but broken for computation.

An alternative failure mode: storing timestamps as `Float` (fractional seconds). This loses precision for sub-second durations at the nanosecond level and creates floating-point comparison problems (`1740744000.0 == 1740744000.0` is `true` until it is not due to float rounding).

**Why it happens:**

ISO 8601 strings are "obviously" what dates look like. PostgreSQL returns timestamps as strings over the text protocol. Mesh has no native integer-typed timestamp. The path of least resistance is to wrap the string representation and call it a "DateTime."

**How to avoid:**

Use two representations, clearly distinguished:
- `Int` (Unix timestamp in milliseconds) as the canonical internal representation for arithmetic, sorting, and storage
- `String` (ISO 8601) as the display/serialization representation

The stdlib functions should be:
- `Date.now() -> Int` (milliseconds since epoch)
- `Date.parse(str) -> Result<Int, String>` (string to epoch ms)
- `Date.format(ts_ms) -> String` (epoch ms to ISO 8601)
- `Date.add_seconds(ts_ms, seconds) -> Int`
- `Date.diff_seconds(ts_ms_a, ts_ms_b) -> Int`

This matches Erlang's `:erlang.system_time(:millisecond)` pattern and avoids the "naive datetime" trap (Python's most infamous date/time mistake). Storing epoch milliseconds means comparisons and arithmetic are integer operations — no parsing, no floating-point, no timezone confusion during storage.

**Warning signs:**

If the `DateTime` type is a String alias, arithmetic requires parsing. If it is a `Float`, sub-millisecond comparisons will drift. If the internal representation is opaque and hides its unit (seconds vs. milliseconds), bugs appear when mixing values.

**Phase to address:** Date/time stdlib phase — the representation decision must be made first; every other function depends on it.

---

### Pitfall 4: Silently Assuming UTC When Timezone Information is Missing

**What goes wrong:**

The date/time stdlib accepts strings like `"2026-02-28 12:00:00"` (no timezone) and treats them as UTC. A user in Tokyo passes their local time. The server records it as UTC. The stored value is 9 hours wrong. There is no error, no warning, and no indication the conversion was applied. This class of bug hides for weeks until a date-sensitive feature fails in production in a non-UTC timezone.

This is the most common date/time bug in production systems. Python's `datetime.datetime.utcnow()` was deprecated precisely because it silently dropped timezone context. JavaScript's `new Date()` parsing is notorious for this.

**Why it happens:**

Parsing `"2026-02-28 12:00:00"` succeeds — it is a valid date. The developer does not add a check for missing timezone info because the happy path works. The bug only manifests when a user provides input without explicit UTC marker.

**How to avoid:**

`Date.parse` must return `Err` for any string that does not include explicit timezone offset. Accept only:
- `"2026-02-28T12:00:00Z"` (UTC)
- `"2026-02-28T12:00:00+09:00"` (explicit offset)

Reject `"2026-02-28 12:00:00"` with an error: "timestamp must include timezone offset (use Z for UTC)". This is strict but correct. Users who need to parse local times must explicitly provide their offset.

Do not attempt to load the system timezone database for DST handling in v14.0. The scope for v14.0 is: parse UTC/fixed-offset timestamps, format timestamps, do arithmetic. Full IANA timezone database (handling DST transitions, historical offsets, "America/New_York" strings) is a separate, substantial feature that should be deferred.

**Warning signs:**

Any `Date.parse` function that accepts strings without a `+HH:MM` or `Z` suffix without returning an error is broken. Any function that calls `chrono::NaiveDateTime` (or equivalent) without attaching a timezone is accumulating timezone debt.

**Phase to address:** Date/time stdlib phase — must be in the initial API design. Cannot be retrofitted after users start passing timezone-free strings.

---

### Pitfall 5: HTTP Client Blocking I/O Starving Actor Scheduler Threads

**What goes wrong:**

The HTTP client is extended to support connection keep-alive and streaming. A Mesh actor calls `Http.get_streaming(url, callback)`. Under the hood, `ureq` (already in `mesh-rt`) makes a blocking read call that waits for the server to send data. This blocking call runs on one of the M:N scheduler's OS worker threads. That thread is now blocked — it cannot resume other actors. If 8 actors simultaneously make streaming HTTP requests and the scheduler has 8 threads, all 8 threads block and the entire actor system deadlocks: no actor can make progress.

**Why it happens:**

The Mesh scheduler is designed for CPU-bound coroutines with cooperative preemption via reduction checks. Blocking I/O calls bypass the reduction-check yield mechanism entirely. `ureq` is explicitly a blocking I/O library (`ureq` README: "uses blocking I/O... requires an OS thread per concurrent request"). The WS reader thread bridge (v4.0 decision in PROJECT.md) already handled this for WebSockets by using a dedicated OS thread per connection that delivers messages via mailbox. The same architectural pattern must apply to streaming HTTP.

**How to avoid:**

For streaming HTTP reads, follow the WS reader thread pattern exactly:

1. Spawn a dedicated OS thread (not a Mesh actor) for the blocking `ureq` read loop.
2. The OS thread reads chunks from the response body and sends them to the actor's mailbox as messages.
3. The actor receives chunks via `receive` with a timeout, processing them cooperatively.
4. When the stream ends (EOF or error), the OS thread sends a sentinel message and exits.

This isolates the blocking I/O from the scheduler threads. The cost is one OS thread per active streaming request — acceptable for the use cases this serves (batch file downloads, long-running API streaming responses).

For connection keep-alive (non-streaming), the issue is less severe because `ureq` requests complete quickly. However, the connection pool state (the `Agent` struct) must be stored outside the GC heap to survive between requests without being collected. Use an opaque `u64` handle (same pattern as DB connection handles) to refer to a `Box<ureq::Agent>` stored in a global registry or per-actor context.

**Warning signs:**

Any benchmark showing request throughput drops to near-zero when actors make concurrent streaming requests. Timer-based tests flaking under load (actors cannot receive timer messages because scheduler threads are blocked).

**Phase to address:** HTTP client improvements phase — before implementing any streaming or keep-alive feature, the threading model must be decided and documented.

---

### Pitfall 6: Chunked Transfer Encoding Parser Missing Zero-Chunk Terminator and Extension Handling

**What goes wrong:**

The hand-rolled HTTP/1.1 parser in `mesh-rt/src/http/server.rs` correctly handles standard request bodies but does not handle chunked transfer encoding for client responses. When the developer adds chunked response reading to the HTTP client, the parser:

1. Reads the hex chunk size and data, but fails to handle the empty terminator chunk (`0\r\n\r\n`) — it either reads past the end of the stream or returns truncated data.
2. Does not skip chunk extensions (the `;name=value` part after the chunk size, specified in RFC 9112 §7.1.1). Real servers send chunk extensions. Encountering a `;` after the chunk size causes the hex parser to fail or produce an incorrect size value.
3. Does not handle trailers (headers after the final `0\r\n` chunk). Trailers are rare but not parsing them leaves junk bytes in the socket buffer, corrupting the next keep-alive request on the same connection.

**Why it happens:**

The happy path (server sends data chunks, no extensions, clean terminator) works in testing. Edge cases only appear against real-world servers. A CVE (2025-66373) was issued against Akamai's edge servers in 2025 for exactly this: "logic error when an edge server received a request whose chunked body was invalid — the edge did not always terminate or sanitize the request." Duplicate `Transfer-Encoding: chunked` headers (rejected by aiohttp in March 2025) are another real-world edge case.

**How to avoid:**

Follow RFC 9112 §7.1 strictly. The chunk-reading loop must:

```
1. Read chunk-size line: everything before optional ';' is hex size; skip any extensions after ';' up to CRLF
2. If chunk-size == 0: read and discard optional trailers until empty CRLF line; break
3. Read exactly chunk-size bytes as chunk-data
4. Read and discard trailing CRLF after chunk-data
5. Append chunk-data to body buffer; go to 1
```

Reject (return Err) on:
- Invalid hex in chunk size
- Duplicate `Transfer-Encoding: chunked` headers
- Chunk size exceeding a configurable limit (prevent memory exhaustion)
- Missing terminator chunk after N bytes (detect hung server)

Write unit tests for: zero-length chunk body, single chunk, multiple chunks, chunk extensions, trailer headers, oversized chunk size, and truncated stream.

**Warning signs:**

Response bodies that are randomly truncated by a few bytes. Keep-alive connections producing garbage on the second request. Tests against `httpbin.org` or `nghttp2.org/httpbin` passing but production servers occasionally returning corrupt data.

**Phase to address:** HTTP client improvements phase — chunk parsing correctness must be verified before keep-alive reuse, because a bad chunk read corrupts the connection state.

---

### Pitfall 7: Test Runner Sharing Actor Scheduler State Across Tests

**What goes wrong:**

The `meshc test` runner spawns a single Mesh process and runs all `*.test.mpl` tests sequentially in it. If test A spawns an actor that handles messages and does not shut it down cleanly, that actor is still running (and registered by name) when test B starts. Test B spawns what it thinks is a fresh actor with the same name, hits the `AlreadyRegistered` error, and the test fails with a confusing error unrelated to the actual assertion being tested. Alternatively, test A's leftover actor receives a message intended for test B's actor, producing a spurious assertion failure in test A several milliseconds after it "passed."

This is the Mesh-specific form of the general "shared state between tests" pitfall documented across Jest, ExUnit, and any other concurrent test framework. The actor registry is global mutable state; tests that register named actors without deregistering them are not isolated.

**Why it happens:**

Mesh actors are designed to be long-lived. Test authors write actors for their test setup and forget that the process registry persists for the lifetime of the runtime. There is no automatic cleanup between tests because the runtime has no concept of "test boundaries."

**How to avoid:**

The test runner must enforce actor isolation per test. Two approaches — choose one:

**Option A (recommended for v14.0 simplicity):** Each test function runs as a separate root actor, spawned fresh with a clean mailbox. Actor names registered during the test are automatically deregistered when the test actor exits (reuse the existing link/exit-signal infrastructure). The scheduler runs all test actors and collects their results via a dedicated result-channel.

**Option B:** Each test function runs in the same process but the test framework tracks all actors spawned during a test (via a thread-local spawn hook) and force-kills them after the test completes, deregistering names.

Option A is cleaner and reuses existing crash-isolation infrastructure (`catch_unwind` per actor already works). Option B requires hooking the spawn path, which is more invasive.

**Warning signs:**

Test suite passes locally when run in isolation but fails intermittently when all tests run together. Test failures that mention "AlreadyRegistered" or "process not found" when no registration errors should exist.

**Phase to address:** Test framework phase — this must be the first design decision, before implementing any assertion helpers or test discovery.

---

### Pitfall 8: Mock Actor Cleanup Leaving Orphan Processes After Test Failure

**What goes wrong:**

The developer creates mock actors for tests: `Mock.spawn_echo_actor()` creates an actor that records received messages. The test calls `Mock.assert_received(pid, expected_msg)` at the end. If the assertion fails (throwing a test failure), the mock actor is never shut down because the cleanup code is after the failing assertion. The mock actor runs indefinitely, consuming a slot in the process table. Over a large test suite with many failures, the leaked mock actors accumulate, eventually exhausting process IDs or memory.

This is the Mesh-specific form of the "neglecting cleanup" pitfall from Jest and other frameworks.

**Why it happens:**

Imperative test cleanup (calling `Process.exit(mock_pid)` at the end of the test) is skipped when an assertion panics or returns early. There is no equivalent of Go's `defer` or Python's `with` statement in Mesh to guarantee cleanup.

**How to avoid:**

Design the mock API so cleanup is automatic, not manual:

1. Mock actors are always linked to the test actor (via `Process.link`). When the test actor exits (whether passing or failing via `catch_unwind`), all linked mock actors receive the exit signal and terminate automatically.
2. Provide a `Mock.with_echo_actor(fn(pid) do ... end)` API where the mock's lifetime is scoped to the closure. The mock is spawned, the closure executes (possibly failing), and the mock is killed when the closure returns — whether normally or via an error.

The supervisor infrastructure already handles this: if a test actor crashes, its linked children (mock actors) also crash. The test runner catches the test actor's exit and records the failure.

**Warning signs:**

Test suite memory consumption grows linearly with the number of test failures. `meshc test` hangs after a failing test because a mock actor is still blocking on `receive`.

**Phase to address:** Test framework phase — design the mock lifecycle to be crash-safe before implementing any mock functionality.

---

### Pitfall 9: LLVM Coverage Instrumentation Incompatible with Mesh's Custom Codegen

**What goes wrong:**

The developer attempts to add coverage reporting to `meshc test` using LLVM's source-based coverage instrumentation (`-fprofile-instr-generate -fcoverage-mapping` in Clang, or `instrument-coverage` in Rust). The coverage data is intended to map back to `.mpl` source files. But Mesh's codegen emits LLVM IR directly (via Inkwell) with no connection to original source positions — there are no `DILocation` debug info metadata nodes attached to most instructions. The coverage tool produces either empty reports or maps coverage to incorrect line numbers in the Rust compiler source, not the `.mpl` user code.

A secondary failure: coverage instrumentation adds counters to every basic block. Mesh's GC uses conservative stack scanning. The counter variables on the stack look like valid pointers and may prevent GC collection of objects whose addresses happen to match counter values (false live roots). This is the same hazard as the "conservative stack scanning may retain some garbage" limitation documented in PROJECT.md, but amplified.

**Why it happens:**

LLVM coverage uses the binary profiling format (`default.profraw`) which requires: (1) instrumented binary run, (2) `llvm-profdata merge`, (3) `llvm-cov report`. This pipeline assumes Clang-compiled code with debug info. Mesh compiles via Inkwell without emitting DWARF debug info or source location metadata by default.

**How to avoid:**

For v14.0, implement coverage at the Mesh source level rather than LLVM IR level:

1. Instrument coverage in the MIR lowering pass: before each statement, emit a call to a coverage counter increment function: `mesh_coverage_record(file_id, line_no, counter_id)`.
2. The coverage counters are stored in a global array (not on the stack, avoiding the conservative GC issue).
3. At test exit, dump the counter array to a JSON file: `coverage.json` with `{file: line: count:}` entries.
4. A post-processing script (or `meshc coverage` command) reads the JSON and generates an HTML report showing which lines were executed.

This approach is simpler than LLVM source-based coverage, works with Mesh's existing codegen, and avoids version mismatch issues (LLVM coverage format is not forwards-compatible between versions — the official docs warn: "newer binaries cannot always be analyzed by older tools").

The LLVM profiling approach can be revisited in a future milestone when Mesh's codegen emits proper DWARF debug info.

**Warning signs:**

Empty coverage reports with `0 functions covered`. `llvm-profdata merge` failing with "Unsupported instrumentation profile format version." Coverage line numbers pointing to `lower.rs` in the Mesh compiler source.

**Phase to address:** Test framework phase — coverage should be the last sub-feature, after the test runner and assertions are working. Start with source-level MIR instrumentation, not LLVM IR instrumentation.

---

### Pitfall 10: Package Registry Allowing Version Overwrites

**What goes wrong:**

The `meshpkg publish` command allows a package author to publish `mylib 1.0.0`, then immediately re-publish `mylib 1.0.0` with different content (a "hotfix" or a "mistake correction"). Any project that previously installed `mylib 1.0.0` now gets different code the next time it runs `meshpkg install` — even though the version number is the same. Builds stop being reproducible. This is the npm "left-pad" failure mode: a maintainer can alter or remove a version that other packages depend on.

**Why it happens:**

The simplest registry API allows PUT/overwrite. It requires deliberate design effort to make publish immutable. New registry implementors often add overwrite as a convenience for "fixing mistakes" without understanding the reproducibility implications.

**How to avoid:**

Make publish-once immutable from day one — this is crates.io's explicit design philosophy: "one of the major goals of crates.io is to act as a permanent archive of crates that does not change over time." The mechanisms:

1. Content-address each version: store packages as `{name}/{version}/{sha256-of-tarball}.tar.gz`. Reject uploads where the SHA-256 differs from a previously stored version.
2. Yank mechanism (not delete): `meshpkg yank mylib 1.0.0` marks the version as "do not use for new installs" but existing lock files can still resolve it. The package content is never deleted.
3. No delete endpoint in the registry API. If a package contains a security vulnerability, yank it and publish `1.0.1`.
4. The `mesh.toml` lock file records exact SHA-256 digests of resolved packages. Install checks the digest against the registry. A tampered registry cannot serve different content to a project with a valid lock file.

**Warning signs:**

A registry API design that has a `PUT /packages/{name}/{version}` endpoint (update semantics). Any "admin override" path that allows content replacement without version bump.

**Phase to address:** Package registry phase — immutability must be in the initial API design document, not retrofitted after the registry is deployed with mutable semantics.

---

## Moderate Pitfalls

---

### Pitfall 11: UUID v4 Using Non-CSPRNG Randomness

**What goes wrong:**

UUID v4 requires 122 bits of cryptographically secure randomness. If the developer generates it using `rand::thread_rng()` without checking that the underlying PRNG is seeded from a secure source, or (worse) uses `rand::rngs::SmallRng` for "performance," the generated UUIDs are predictable. An attacker who can observe a few UUIDs can reconstruct the PRNG state and predict future UUIDs — enabling ID enumeration, SSRF via predictable resource IDs, or session token forgery.

**Why it happens:**

The `rand` crate (already in `mesh-rt`) has multiple RNG backends. `rand::random::<u128>()` uses `ThreadRng` which IS cryptographically secure. But `rand::rngs::SmallRng` is explicitly documented as "not for security." The performance difference is negligible (a few nanoseconds), but developers sometimes reach for the "fast" option without reading the security implications.

**How to avoid:**

Use `ring::rand::SystemRandom` (already in `mesh-rt`) to generate the 16 random bytes for UUID v4. `ring::rand::SecureRandom::fill` uses the OS CSPRNG (`/dev/urandom` on Linux, `BCryptGenRandom` on Windows). Apply UUID v4 bit-masking per RFC 4122 §4.4 to the result. This is the approach already used in the distributed node clustering code for ephemeral key generation.

Document in the stdlib: "UUIDs generated by `Crypto.uuid_v4()` are cryptographically random and suitable for use as unique identifiers in security-sensitive contexts."

**Warning signs:**

Any UUID implementation that uses `rand::rngs::SmallRng` or any non-OS-seeded PRNG. Any implementation that seeds from `SystemTime` instead of OS randomness.

**Phase to address:** Crypto stdlib phase — UUID is one of the simpler functions but must use the correct PRNG.

---

### Pitfall 12: HTTP Keep-Alive Pool Stored on GC Heap

**What goes wrong:**

The developer stores the `ureq::Agent` (which manages the keep-alive connection pool) as an opaque value in the Mesh runtime. If it is allocated on the GC heap with `mesh_gc_alloc_actor`, the GC's conservative stack scanner may decide the Agent is unreachable during a GC cycle and free it mid-use. The next request using the freed Agent causes a use-after-free, typically presenting as a SIGSEGV or a corrupt HTTP response.

**Why it happens:**

The existing pattern for opaque handles (DB connections, pool handles, regex handles, WebSocket rooms) uses `Box::into_raw` and stores the resulting raw pointer as a `u64` in a global registry or returns it directly as a Mesh `Int`. The GC cannot collect objects referenced only by a `u64` because it does not know the value is a pointer. This is the documented pattern ("Opaque u64 handles are GC-safe" in PROJECT.md decisions), but a developer unfamiliar with this pattern may try to allocate the Agent struct on the GC heap directly, which breaks the contract.

**How to avoid:**

Follow the exact pattern established for DB connections (v2.0) and regex handles (v12.0):

```rust
// Correct: Box the Agent, leak it, return raw pointer as u64
let agent = ureq::AgentBuilder::new().build();
let ptr = Box::into_raw(Box::new(agent)) as u64;
// Return ptr as a Mesh Int (opaque handle)

// Wrong: allocate on GC heap
let ptr = mesh_gc_alloc_actor(...) as *mut ureq::Agent; // GC can collect this
```

The `u64` handle is passed back to Mesh code and stored in a `let` binding or struct field as `Int`. The GC sees an integer, not a pointer, and does not attempt to collect it. Cleanup requires an explicit `Http.close_client(handle)` call that runs `Box::from_raw` and drops the Agent.

**Warning signs:**

Occasional SIGSEGV on keep-alive requests. Requests that succeed on first call but fail randomly on subsequent calls (GC ran between calls and freed the Agent).

**Phase to address:** HTTP client improvements phase — establish the handle pattern first, before any keep-alive implementation.

---

### Pitfall 13: Date/Time Arithmetic Overflowing Integer Range

**What goes wrong:**

The date/time stdlib uses `Int` (i64 in Mesh's LLVM representation) for Unix timestamps in milliseconds. `Date.add_seconds(ts_ms, seconds)` computes `ts_ms + seconds * 1000`. If `seconds` is user-supplied and large (e.g., a typo: `Date.add_seconds(now, years_to_seconds(10000))`), the multiplication overflows `i64`. In debug builds Rust detects integer overflow, but in release builds (how Mesh compiles) the overflow wraps silently, producing a timestamp in 1970 or far future.

**Why it happens:**

Unchecked integer arithmetic is the default in Rust release builds. LLVM optimizes away overflow checks when compiling without `-C overflow-checks=on`.

**How to avoid:**

Use checked arithmetic for all date/time operations. The extern C function for `Date.add_seconds` should validate inputs before computing:

```rust
pub extern "C" fn mesh_date_add_ms(ts_ms: i64, delta_ms: i64) -> *mut u8 {
    match ts_ms.checked_add(delta_ms) {
        Some(result) => alloc_ok_int(result),
        None => alloc_err_string("timestamp arithmetic overflow"),
    }
}
```

Return `Result<Int, String>` from all arithmetic functions. This forces the Mesh caller to handle the overflow case via `?` operator, making the error visible.

Validate that all returned timestamps are in a reasonable range (e.g., year 1970 to year 2262 for ms timestamps, which fit comfortably in i64).

**Warning signs:**

Date arithmetic that returns the year 1970, or dates in the distant future (year 30000+). Any date/time function that returns a raw `Int` rather than `Result<Int, String>` for operations that can overflow.

**Phase to address:** Date/time stdlib phase — arithmetic overflow checks should be the default, not an afterthought.

---

### Pitfall 14: Test Assertions Using Variable-Time String Comparison for Crypto Output

**What goes wrong:**

The test framework includes `assert_eq(a, b)` which calls `snow_string_eq` (variable-time string comparison) on its arguments. A test for the HMAC module writes `assert_eq(Crypto.hmac_sha256(key, msg), expected_mac)`. This works correctly for testing — the assertion passes or fails — but it also means the test itself leaks timing information about the HMAC output. This is a minor issue in a test suite but a major issue if `assert_eq` is ever used in production code to compare tokens.

More immediately: the test framework's `assert_eq` will be used as the example pattern in documentation. If developers see `assert_eq(computed_mac, expected_mac)` in test code, they will replicate it in production code, creating a timing vulnerability.

**Why it happens:**

General-purpose equality assertions cannot be constant-time — they are for testing, not production auth. The problem is that the API surface is the same (`==` in production vs `assert_eq` in tests), so developers do not see a clear distinction between "comparison for testing" and "comparison for security."

**How to avoid:**

Add a note in the stdlib documentation for every crypto output function: "Do not compare outputs using `==` or `assert_eq` in production code. Use `Crypto.hmac_verify` or `Crypto.secure_compare`." In test code, `assert_eq` is fine because correctness checking is the only goal.

In the test framework internals, assert functions should NOT be used for constant-time comparisons — they should explicitly use value equality. The distinction must be documented, not enforced at the API level (since constant-time assertions would be slower than necessary and confusing).

**Warning signs:**

Documentation examples that show `assert_eq(Crypto.hmac_sha256(...), ...)` without a note that this is test-only.

**Phase to address:** Crypto stdlib phase (documentation) and test framework phase (documentation).

---

### Pitfall 15: Package Registry Search Unavailable Before Content Exists

**What goes wrong:**

The developer builds the full package registry — publish, install, search API, hosted website — before any real packages exist. The search feature returns empty results for all queries. The "browse packages" page shows nothing. The hosted site looks like a ghost town. When the internal team tries to demo it or write documentation, there is nothing to show. Momentum dies.

**Why it happens:**

The registry is built in "correct order" (infrastructure before content), but the usability dependency is inverted: the registry only feels useful when packages exist, and packages only get published when there is a registry to publish to.

**How to avoid:**

Publish Mesh's own standard library modules as packages on the registry immediately after the registry is functional:

- `mesh/crypto` (the new v14.0 crypto stdlib)
- `mesh/datetime` (the new date/time module)
- `mesh/testing` (the test framework helpers)
- `mesh/http-client` (the improved HTTP client)

These are real packages with real code, maintained by the Mesh team, that immediately demonstrate the registry working end-to-end. When users visit the packages site, they see known-good packages with actual documentation.

This mirrors crates.io's launch: the Rust standard library's component crates (`serde`, `tokio`) were available from early on, demonstrating the registry's value immediately.

**Warning signs:**

Registry launch date is set before any packages are planned for initial publication. The hosted site's "Browse Packages" page is designed before at least 4-5 packages are ready to appear there.

**Phase to address:** Package registry phase — include "publish stdlib packages" as an explicit deliverable in the same phase as the registry launch.

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Variable-time comparison for secrets | Simpler implementation | Timing attack vulnerability | Never for production auth code |
| String-based timestamp representation | Easy to debug, PostgreSQL-compatible | Slow arithmetic, silent timezone loss | Only for display/serialization layer |
| Skipping chunked trailer parsing | Simpler parser | Keep-alive connection corruption on real servers | Never — trailers are in RFC 9112 |
| Global ureq::Agent (not per-actor) | One connection pool, simpler bookkeeping | All actors share pool limits; one slow actor can block others | Acceptable for v14.0 as a starting point |
| Test runner in same process as tested code | No IPC overhead | Actor registry leaks between tests, no true isolation | Only if tests explicitly clean up named actors |
| Allow publish overwrite in registry v1 | Simpler implementation | Breaks reproducibility, cannot be taken back without disruption | Never — immutability must be designed in from the start |

---

## Integration Gotchas

Common mistakes when connecting to external services or internal subsystems.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| `sha2` crate for Mesh SHA-256 | Adding a new `sha2` dep when one already exists in Cargo.toml | Audit `mesh-rt/Cargo.toml` before adding crypto deps; reuse existing |
| `ring::rand` for UUID | Using `rand::rngs::SmallRng` for speed | Always use `ring::rand::SystemRandom` for security-sensitive randomness |
| `ureq::Agent` keep-alive pool | Storing Agent in GC heap via `mesh_gc_alloc_actor` | Use `Box::into_raw` opaque u64 handle pattern (same as DB connections) |
| Actor scheduler + blocking ureq reads | Calling blocking read inside coroutine | Spawn dedicated OS thread for blocking I/O (WS reader pattern from v4.0) |
| Chunked response bodies | Stopping at last data chunk | Always consume the zero-length terminator chunk (`0\r\n\r\n`) |
| Test actor registry | Registering named actors without cleanup | Link mock actors to test actor so they die when test exits |
| Registry publish | No content-addressing | SHA-256 hash every tarball; reject re-upload of same version with different content |
| Date parsing | Accepting timezone-free strings silently | Return `Err` for any input without explicit `Z` or `+HH:MM` offset |

---

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| One OS thread per streaming HTTP request | 100% CPU on 8-core machine with 8 streams | Use async-friendly HTTP client (reqwest) for high-concurrency streaming in future | At ~8-16 concurrent streaming requests |
| No connection keep-alive for HTTP client | Each request pays TCP + TLS handshake overhead | Implement ureq::Agent-based connection pooling with opaque u64 handle | At ~50 req/s to the same host |
| All test files compiled into one binary | `meshc test` build time grows linearly with test count | Parallel compilation per file; cache unchanged test artifacts | At ~200 test files |
| Registry full-text search via SQL LIKE | `SELECT * FROM packages WHERE name LIKE '%query%'` is a full table scan | Add `tsvector` index for description, use PostgreSQL FTS from the start | At ~1,000 packages |
| Test runner starts fresh runtime per test file | `meshc test` takes 10 seconds for 50 test files due to 50 runtime initializations | Run all tests in a single runtime, isolating via actor-per-test | At ~50 test files |

---

## Security Mistakes

Domain-specific security issues beyond general web security.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Variable-time HMAC comparison in `Crypto.hmac_verify` | Timing attack allows secret recovery | Use `hmac::Mac::verify_slice` from existing dep (constant-time via `subtle`) |
| UUID v4 from `SmallRng` or seeded PRNG | Predictable IDs enable enumeration/forgery | Use `ring::rand::SystemRandom` (OS CSPRNG) |
| Package registry publish without authentication | Anyone can publish to any namespace | Require API token; associate tokens with package namespace ownership at creation time |
| Registry tarball served without integrity check | Man-in-the-middle can substitute malicious package | SHA-256 content-address storage; `mesh.toml` lock file records digests |
| Date/time silent UTC assumption | Business logic bugs from timezone confusion | Reject timezone-free timestamp strings in `Date.parse` |
| Base64 decoding without padding validation | Malformed input causes panic in some decoders | Use `base64::engine::general_purpose::STANDARD.decode` with explicit error handling |

---

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Crypto.sha256:** Appears done — but verify that the output is hex-encoded consistently (lowercase hex, no `0x` prefix, exactly 64 characters). The underlying `sha2` crate returns bytes; encoding to hex string is a separate step that must be tested.
- [ ] **Crypto.hmac_sha256:** Appears done — but verify that the `hmac_verify` companion function uses constant-time comparison, not `==` on the String output.
- [ ] **Date.parse:** Appears done — but verify that strings without timezone offset return `Err`, not a silently-wrong UTC value.
- [ ] **Date.format:** Appears done — but verify that millisecond timestamps are serialized in ISO 8601 with `Z` suffix, not as bare Unix numbers.
- [ ] **HTTP chunked reading:** Appears done — but verify with a real chunked response that includes chunk extensions (`;name=value` after the size) and trailers. Happy-path tests with clean chunks will pass even with a broken parser.
- [ ] **HTTP keep-alive:** Appears done — but verify that the `ureq::Agent` survives a GC cycle between requests (store as opaque u64, never on GC heap).
- [ ] **Test runner isolation:** Appears done — but run two tests that both register the same actor name and verify the second test doesn't fail with "AlreadyRegistered."
- [ ] **Mock cleanup:** Appears done — but write a test that fails mid-execution and verify no orphan actors remain after the suite completes.
- [ ] **Coverage reporting:** Appears done — but verify that coverage line numbers map to `.mpl` source files, not to Rust compiler source.
- [ ] **Package publish once:** Appears done — but attempt to publish the same `name@version` twice with different content and verify the second publish is rejected with an error, not silently accepted or silently ignored.
- [ ] **Package install with lock file:** Appears done — but verify that `meshpkg install` with an existing lock file uses pinned versions (does not upgrade), and that the installed content matches the recorded SHA-256 digest.

---

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Duplicate crypto deps | LOW | Remove new dep, rewire calls to existing dep; no API change if the extern C signatures are compatible |
| Variable-time comparison already shipped | MEDIUM | Add `Crypto.secure_compare` function; deprecate usage of `==` on crypto output; publish security advisory |
| Timestamp as String already in use | HIGH | Add conversion functions `Date.from_string_to_ms` and `Date.from_ms_to_string`; deprecate old String-based functions; cannot change existing data without migration |
| Test registry leaks causing flaky tests | MEDIUM | Add `after_each` cleanup hook to deregister known actor names; or switch to actor-per-test isolation model |
| Package registry allows overwrites | HIGH | Cannot take back: once a version is overwritten, trust is broken. Must announce "registry reset" with v2.0 API, explain why immutability matters, re-publish all packages. |
| Chunked parser corruption of keep-alive socket | MEDIUM | Disable keep-alive for affected endpoints as workaround; fix parser per RFC 9112; add regression test for each edge case |
| LLVM coverage version mismatch | LOW | Fall back to source-level MIR instrumentation (the recommended approach); LLVM-based coverage can be added later when debug info is complete |

---

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Duplicate crypto deps | Crypto stdlib (first task: `Cargo.toml` audit) | `cargo tree | grep sha2` shows one version |
| Non-constant-time HMAC comparison | Crypto stdlib (API design) | Security review of `hmac_verify` implementation |
| Timestamp as String | Date/time stdlib (representation decision) | `Date.now()` returns `Int`, not `String` |
| Silent UTC assumption | Date/time stdlib (parser) | `Date.parse("2026-02-28 12:00:00")` returns `Err` |
| Integer overflow in date arithmetic | Date/time stdlib (arithmetic functions) | `Date.add_seconds(max_i64, 1)` returns `Err` |
| Blocking I/O starvation | HTTP client improvements (threading design) | 16 concurrent streaming actors do not deadlock 8-thread scheduler |
| Chunked parser edge cases | HTTP client improvements (chunk parser) | Unit tests for: extensions, trailers, zero-length body, oversized chunk |
| Keep-alive pool on GC heap | HTTP client improvements (handle design) | Agent survives GC cycle; no use-after-free under load |
| Test actor registry leaks | Test framework (isolation design, first task) | Two tests with same actor name both pass when run sequentially |
| Mock actor orphans | Test framework (mock API design) | Suite with 10 failing tests leaves 0 orphan actors |
| LLVM coverage mismatch | Test framework (coverage design) | Coverage report shows `.mpl` line numbers, not Rust line numbers |
| Registry version overwrite | Package registry (API design) | Second publish of same version returns HTTP 409 Conflict |
| Empty registry at launch | Package registry (content plan) | Registry launches with at least 4 stdlib packages already published |
| UUID from weak PRNG | Crypto stdlib (UUID implementation) | UUIDs generated using `ring::rand::SystemRandom` |

---

## Sources

- `/Users/sn0w/Documents/dev/mesh/compiler/mesh-rt/Cargo.toml` — existing crypto deps (sha2, hmac, ring, base64, rand) — HIGH confidence, direct source
- `/Users/sn0w/Documents/dev/mesh/compiler/mesh-rt/src/http/client.rs` — current ureq 2.x blocking I/O pattern — HIGH confidence, direct source
- `/Users/sn0w/Documents/dev/mesh/compiler/mesh-rt/src/actor/scheduler.rs` — M:N scheduler design, coroutines `!Send`, thread-pinned — HIGH confidence, direct source
- `/Users/sn0w/Documents/dev/mesh/.planning/PROJECT.md` — v4.0 WS reader thread decision, opaque u64 handle pattern, conservative GC scanning — HIGH confidence, direct source
- [dalek-cryptography/subtle — constant-time Rust utilities](https://github.com/dalek-cryptography/subtle) — LLVM branch re-introduction risk, best-effort constant-time — MEDIUM confidence
- [ureq blocking I/O model](https://docs.rs/ureq) — "blocking I/O... one OS thread per concurrent request" — HIGH confidence, official docs
- [ureq connection pool issue: chunked + compressed response inhibits reuse](https://github.com/algesten/ureq/issues/549) — known keep-alive edge case — MEDIUM confidence
- [CVE-2025-66373 Akamai chunked body size](https://www.akamai.com/blog/security/cve-2025-66373-http-request-smuggling-chunked-body-size) — real-world chunked parser failure, 2025 — HIGH confidence
- [aiohttp duplicate Transfer-Encoding bug 2025](https://github.com/aio-libs/aiohttp/issues/10611) — duplicate chunked header edge case — MEDIUM confidence
- [RFC 9112 §7.1 chunked transfer coding](https://www.rfc-editor.org/rfc/rfc9112#section-7.1) — authoritative spec for chunk extensions and trailers — HIGH confidence
- [Falsehoods programmers believe about time](https://gist.github.com/timvisee/fcda9bbdff88d45cc9061606b4b923ca) — timezone and timestamp pitfall catalog — MEDIUM confidence
- [crates.io publishing semantics](https://doc.rust-lang.org/cargo/reference/publishing.html) — immutability and yank design rationale — HIGH confidence
- [LLVM source-based code coverage](https://clang.llvm.org/docs/SourceBasedCodeCoverage.html) — format version incompatibility, profdata merge requirement — HIGH confidence
- [Jest mock cleanup pitfalls 2025](https://www.mindfulchase.com/explore/troubleshooting-tips/testing-frameworks/advanced-troubleshooting-in-jest-flaky-tests,-mocks,-and-performance-at-scale.html) — stale mock state, shared global objects — MEDIUM confidence
- [Dependency hell and SemVer limitations 2025](https://prahladyeri.github.io/blog/2024/11/dependency-hell-revisited.html) — transitive dep drift, publish semantics — MEDIUM confidence

---

*Pitfalls research for: Mesh v14.0 Ecosystem & Standard Library*
*Researched: 2026-02-28*