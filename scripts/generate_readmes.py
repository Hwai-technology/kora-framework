import os

crates = {
    'kora-span': {
        'title': 'Kora Span & Source Mapping Engine',
        'purpose': 'Manages source code spans, byte offsets, file IDs, line/column calculations, and cross-file source registries for high-precision error diagnostics.',
        'pipeline': 'Phase 0 (Foundation) — Referenced by every compiler phase to attach exact file coordinates to AST nodes and tokens.',
        'types': ['Span (start, end, file_id)', 'FileId', 'SourceMap (source code registry and line indexer)', 'Location (line, column)'],
        'security': 'Guarantees panic-free slice indexing through checked byte offsets and safe multi-byte UTF-8 boundary assertions.',
        'improve': 'Add memory-mapped file support for gigabyte-scale multi-file source trees and incremental source cache invalidation.'
    },
    'kora-diagnostics': {
        'title': 'Kora Compiler Diagnostic Reporting Engine',
        'purpose': 'Renders rich compiler diagnostics with ANSI color formatting, underlined source snippets, primary/secondary labels, error codes, and help notes.',
        'pipeline': 'Phase 0 (Foundation) — Diagnostic aggregation bus used across Lexer, Parser, HIR, Type Checker, and Ownership analyzer.',
        'types': ['Diagnostic (error, warning, note)', 'DiagnosticBag', 'Label', 'Severity'],
        'security': 'Zero panic on out-of-bounds spans; safely clips truncated source snippets and sanitizes terminal control characters.',
        'improve': 'Integrate machine-readable SARIF (Static Analysis Results Interchange Format) output for CI/CD integrations.'
    },
    'kora-ast': {
        'title': 'Kora Abstract Syntax Tree (AST)',
        'purpose': 'Defines the strongly typed Abstract Syntax Tree representation for Kora programs, including models, classes, interfaces, enums, decorators, and expressions.',
        'pipeline': 'Phase 1 (Syntax Representation) — The direct product of the recursive-descent parser.',
        'types': ['Module', 'Item (Model, Class, Interface, Enum, Fn, Import)', 'Expr (Binary, Unary, Call, Closure, Match)', 'Stmt', 'TypeExpr'],
        'security': 'Struct-free invariant enforcement: only Model and Class representations exist; struct keyword is structurally prohibited (E2003).',
        'improve': 'Add AST visitor and folder macros to simplify compiler pass transformations without manual tree walking.'
    },
    'kora-lexer': {
        'title': 'Kora Lexical Tokenizer Engine',
        'purpose': 'High-performance lexical scanner transforming raw UTF-8 Kora source text (.kora and .kr) into a strongly typed stream of tokens and trivia.',
        'pipeline': 'Phase 1 (Frontend Tokenization) — Precedes the recursive-descent parser.',
        'types': ['Token', 'TokenKind', 'tokenize(file_id, src, bag)'],
        'security': 'Bounded string and comment scanning with defense against unclosed delimiters and ReDoS token patterns.',
        'improve': 'Add SIMD-accelerated whitespace and identifier scanning utilizing AVX2/NEON intrinsics for multi-gigabyte parsing speed.'
    },
    'kora-parser': {
        'title': 'Kora Recursive-Descent & Pratt Expression Parser',
        'purpose': 'Constructs the full AST from tokens using Pratt operator precedence parsing, auto-constructor synthesis, parameter promotion, and error recovery.',
        'pipeline': 'Phase 1 (Syntactic Analysis) — Produces verified Module ASTs with error synchronization.',
        'types': ['parse_module(file_id, src, bag)', 'Parser'],
        'security': 'Recursion depth limits to prevent stack overflow from adversarial deeply nested parenthesis or bracket attacks.',
        'improve': 'Implement tree-sitter or incremental parsing support to achieve sub-millisecond editor re-parsing on keystroke.'
    },
    'kora-hir': {
        'title': 'Kora High-Level Intermediate Representation & Symbol Resolver',
        'purpose': 'Lowers AST into High-Level IR, constructs lexical symbol scopes, resolves variable bindings, and checks mutability rules (let vs var).',
        'pipeline': 'Phase 2 (Semantic Lowering) — Sits between syntax parsing and type checking.',
        'types': ['HirModule', 'HirItem', 'HirExpr', 'HirStmt', 'lower_and_resolve(ast, bag)'],
        'security': 'Immutable-by-default enforcement: strictly rejects mutations on let-bound variables (E3002).',
        'improve': 'Add dead code elimination and constant propagation passes at the HIR level before type checking.'
    },
    'kora-ty': {
        'title': 'Kora Static Type System & Clean Architecture Verifier',
        'purpose': 'Performs static type checking, interface conformance verification, pattern matching exhaustiveness checks, and architectural boundary purity enforcement.',
        'pipeline': 'Phase 3 (Type & Architectural Verification) — Validates type correctness before codegen.',
        'types': ['TypeCheckOutput', 'Type', 'check_module(hir, bag)', 'ClassInfo', 'ModelInfo'],
        'security': 'Enforces single-null semantics: undefined is rejected with E2005. Enforces clean architectural separation (E5001-E5004).',
        'improve': 'Add type inference for higher-kinded types and structural record typing for lightweight JSON payloads.'
    },
    'kora-ownership': {
        'title': 'Kora Compile-Time Ownership & DI Cycle Analyzer',
        'purpose': 'Validates lifecycle semantics, detects circular dependencies in Service/Controller Dependency Injection graphs, and ensures zero leaks.',
        'pipeline': 'Phase 4 (Safety Analysis) — Guarantees acyclic service instantiation before native code emission.',
        'types': ['analyze_ownership(hir, types, bag)', 'OwnershipReport', 'DiGraph'],
        'security': 'Guarantees absence of memory leaks and circular reference deadlock by rejecting DI graph cycles at compile time.',
        'improve': 'Add compile-time tracking for async task handle ownership and actor message buffer lifetimes.'
    },
    'kora-codegen': {
        'title': 'Kora Native Rust Code Generator & OpenAPI Synthesizer',
        'purpose': 'Emits mathematically safe, highly optimized native Rust source code from verified HIR, synthesizes automatic OpenAPI 3.1 & Swagger UI, and compiles via Cargo onto dedicated port 7582.',
        'pipeline': 'Phase 5 (Code Generation) — Bridges Kora frontend with Rust native backend and web ecosystem.',
        'types': ['generate_rust(hir, types, ownership, name)', 'compile_native(rust_code, target_dir)', 'has_http_routes', 'Database', 'Request', 'send'],
        'security': 'Generates 100% unsafe-free Rust code. Emits #[forbid(unsafe_code)] at the top of every generated Rust file.',
        'improve': 'Add direct LLVM IR or Cranelift emission backend to bypass cargo compilation for rapid development iteration.'
    },
    'kora-runtime': {
        'title': 'Kora Concurrency & Async Actor Runtime',
        'purpose': 'Provides Tokio-powered asynchronous concurrency: bounded MPSC channels, Pub/Sub broadcast channels, isolated stateful actors (tell/ask), and TaskGroups.',
        'pipeline': 'Runtime Subsystem — Powers execution of asynchronous Kora applications and services.',
        'types': ['Channel<T>', 'BroadcastChannel<T>', 'ActorRef<M, R>', 'spawn_actor', 'TaskGroup<T>', 'KoraRuntime'],
        'security': 'Bounded channel backpressure prevents memory exhaustion. Actor query timeouts protect against deadlocks.',
        'improve': 'Add distributed actor clustering across network nodes with automatic leader election and heartbeats.'
    },
    'kora-http': {
        'title': 'Kora High-Performance HTTP Engine',
        'purpose': 'Zero-overhead asynchronous HTTP web engine built on Axum and Hyper with automatic request routing, JSON serialization, middleware chaining, and OpenAPI 3.1 generation on port 7582.',
        'pipeline': 'Web Engine — Powers get, post, put, delete routing and server pipelines.',
        'types': ['KoraApp', 'RouteInfo', 'HttpServer', 'RouteHandler', 'Request', 'Validator'],
        'security': 'Built-in request body size limits, rate limiting, and header sanitization to protect against HTTP flood attacks.',
        'improve': 'Add native HTTP/3 (QUIC) protocol support and automated OpenAPI / Swagger 3.1 JSON schema generator.'
    },
    'kora-orm': {
        'title': 'Kora Type-Safe ORM & Query Builder',
        'purpose': 'High-performance database abstraction layer atop SQLx, supporting PostgreSQL, SQLite in-memory testing, auto-migrations, and seeders.',
        'pipeline': 'Data Persistence — Bridges @entity models with relational databases.',
        'types': ['QueryBuilder', 'Database', 'SqlValue', 'FilterClause', 'FilterOp'],
        'security': '100% prepared parameterized queries (WHERE col = ?1). Eliminates SQL injection by design.',
        'improve': 'Add compile-time SQL query syntax verification and automatic spatial (PostGIS) column mapping.'
    },
    'kora-auth': {
        'title': 'Kora Authentication, RBAC & Token Security',
        'purpose': 'Provides enterprise authentication: Argon2id password hashing with unique cryptographic salts, HMAC-SHA256 JWT tokens, and RBAC guards.',
        'pipeline': 'Security Subsystem — Protects endpoints and data access.',
        'types': ['PasswordAuth', 'JwtSigner', 'RbacGuard'],
        'security': 'Constant-time comparison prevents timing side-channels. Rejects tampered payloads, signature forgery, and key mismatches.',
        'improve': 'Add OAuth2 / OIDC provider integration and WebAuthn (Passkeys) hardware security key support.'
    },
    'kora-queue': {
        'title': 'Kora Distributed Background Job Queue',
        'purpose': 'Asynchronous job queue engine with retry policies, exponential backoff, dead-letter queues (DLQ), and in-memory or Redis backends.',
        'pipeline': 'Async Task Processing — Executes background tasks out-of-band.',
        'types': ['JobQueue', 'Job', 'JobStatus', 'RetryPolicy'],
        'security': 'Payload size limits and bounded queue depth prevent job flood denial-of-service.',
        'improve': 'Add persistent disk-backed WAL (Write-Ahead Log) for zero-loss message delivery across unexpected node restarts.'
    },
    'kora-realtime': {
        'title': 'Kora Realtime WebSocket & Event Engine',
        'purpose': 'High-throughput WebSocket pub/sub connection manager for broadcasting domain events to connected clients with channel isolation.',
        'pipeline': 'Realtime Streaming — Powers live web and mobile updates.',
        'types': ['RealtimeServer', 'ChannelHub', 'DomainEvent'],
        'security': 'Enforces per-client connection limits, authentication handshakes, and subscription authorization.',
        'improve': 'Implement Redis Pub/Sub backplane for horizontal multi-server WebSocket scale-out.'
    },
    'kora-observability': {
        'title': 'Kora Metrics, Structured Logging & Tracing',
        'purpose': 'Provides structured JSON logging, distributed OpenTelemetry trace propagation, and atomic performance counters.',
        'pipeline': 'Observability & Monitoring — Delivers telemetry for production workloads.',
        'types': ['MetricsCollector', 'StructuredLogger', 'SpanTracker'],
        'security': 'Automatic redacting of sensitive credential patterns (passwords, tokens, keys) before writing to log streams.',
        'improve': 'Add native Prometheus /metrics HTTP scrape endpoint exporter.'
    },
    'kora-pkg': {
        'title': 'Kora Distributed Package Manager (kora-pkg)',
        'purpose': 'Manages distributed dependencies, SemVer 2.0.0 constraint solving, cryptographic SHA-256 lockfile (kora.lock) generation, and publishing.',
        'pipeline': 'Package Tooling — Powers kora pkg <install|add|remove|list|publish>.',
        'types': ['PackageManager', 'SemVer', 'KoraManifest', 'KoraLockfile', 'RegistryIndexEntry'],
        'security': 'Strict path traversal validation (rejects ../, /, \\0). Enforces SHA-256 integrity checksums on all packages.',
        'improve': 'Add GPG package signature verification and decentralized IPFS/S3 package registry mirror fallbacks.'
    },
    'kora-fmt': {
        'title': 'Kora Code Formatter Engine',
        'purpose': 'Opinionated code formatter for Kora source files (.kora and .kr) guaranteeing consistent syntax style, indenting, and spacing.',
        'pipeline': 'Developer Tooling — Enforces unified formatting across teams.',
        'types': ['format_source(src, options)', 'FormatOptions'],
        'security': 'Idempotent formatting: running the formatter multiple times produces identical output without AST alterations.',
        'improve': 'Add parallel formatting using Rayon to format large repositories of thousands of files in milliseconds.'
    },
    'kora-lsp': {
        'title': 'Kora Language Server Protocol (LSP) Engine',
        'purpose': 'Implements the Language Server Protocol for IDEs: code completion, go-to-definition, hover documentation, and real-time diagnostics.',
        'pipeline': 'IDE Integration — Powers VS Code, Antigravity IDE, Neovim, and Helix editors.',
        'types': ['LspServer', 'CompletionHandler', 'DiagnosticsHandler'],
        'security': 'Non-blocking worker loop prevents editor UI freezing during heavy compiler analyses.',
        'improve': 'Add workspace-wide rename refactoring and semantic token highlighting.'
    },
    'kora-testing': {
        'title': 'Kora Built-in Unit & Integration Test Runner',
        'purpose': 'Lightweight test harness for Kora applications with assertion macros, test discovery, and isolated database sandboxing.',
        'pipeline': 'Testing Tooling — Provides native testing primitives.',
        'types': ['TestRunner', 'TestReport', 'TestCase'],
        'security': 'Isolates test environments using in-memory databases to eliminate destructive mutations on production databases.',
        'improve': 'Add property-based fuzz testing generation (QuickCheck style) for Kora functions.'
    },
    'kora-profiler': {
        'title': 'Kora CPU & Memory Allocation Profiler',
        'purpose': 'Instruments Kora applications to track execution time per function, allocations, and database query latencies.',
        'pipeline': 'Performance Tooling — Guides performance optimization.',
        'types': ['ProfilerHarness', 'ProfileReport', 'LatencyHistogram'],
        'security': 'Zero overhead when inactive; sampling profiler uses low-overhead thread signals to avoid degrading performance.',
        'improve': 'Generate interactive flamegraphs directly viewable in the browser or Antigravity IDE.'
    },
    'kora-bench': {
        'title': 'Kora Quantitative Benchmark Suite',
        'purpose': 'Reproducible quantitative benchmarking framework comparing Kora against Rust (Axum), Go (Gin), Node.js (Fastify), and PHP (Laravel).',
        'pipeline': 'Performance Verification — Powers kora bench CLI command.',
        'types': ['BenchmarkHarness', 'BenchmarkResult', 'simulate_run'],
        'security': 'Deterministic warmup cycles and statistical outlier rejection prevent thermal throttling bias.',
        'improve': 'Add real HTTP load generation against live running server processes using wrk or autocannon.'
    },
    'kora-cli': {
        'title': 'Kora Command-Line Interface (kora)',
        'purpose': 'The unified developer CLI providing check, types, ast, tokens, codegen, build, run, watch, fmt, lsp, swagger, bench, db, pkg, init, create, and update commands for .kora and .kr files.',
        'pipeline': 'Primary Entrypoint — User-facing executable for all developer operations and architectural scaffolding.',
        'types': ['main()', 'cmd_check', 'cmd_build', 'cmd_run', 'cmd_watch', 'cmd_fmt', 'cmd_lsp', 'cmd_swagger', 'cmd_bench', 'cmd_db', 'cmd_pkg', 'cmd_init', 'cmd_create', 'cmd_update'],
        'security': 'Validates all file paths, catches missing imports/files with actionable hints, enforces architectural purity rules, and builds binaries with #[forbid(unsafe_code)].',
        'improve': 'Add interactive project migration and deployment integrations for cloud providers directly from CLI.'
    }
}

os.makedirs('crates', exist_ok=True)

# Generate crates/README.md
with open('crates/README.md', 'w') as f:
    f.write('''# Kora Compiler & Runtime Architecture (`crates/`)

Welcome to the internal modular architecture of the **Kora Programming Language & Backend Framework**.
Every component is segregated into focused, zero-unsafe Rust crates under this directory.

---

## 🗺️ Compilation Pipeline & Workspace Map

```
Source (.kora / .kr)
       │
       ▼
 [kora-lexer] ──► Token Stream
       │
       ▼
 [kora-parser] ──► Abstract Syntax Tree [kora-ast]
       │
       ▼
  [kora-hir] ──► Lowering & Lexical Scopes
       │
       ▼
   [kora-ty] ──► Type Inference & Clean Architecture Checks
       │
       ▼
[kora-ownership] ──► DI Graph Verification & Leak Safety
       │
       ▼
[kora-codegen] ──► Verified Safe Rust Code (.kora-build/)
       │
       ▼
 Native Executable (powered by [kora-runtime] + [kora-http])
```

---

## 📦 Directory of Crates

| Crate | Purpose | Stage |
|:---|:---|:---|
| [`kora-span`](./kora-span/) | Source maps, byte spans, and line/col indexing | Phase 0 |
| [`kora-diagnostics`](./kora-diagnostics/) | Rich terminal diagnostic rendering with ANSI styling | Phase 0 |
| [`kora-ast`](./kora-ast/) | Strongly-typed AST nodes (Models, Classes, Enums) | Phase 1 |
| [`kora-lexer`](./kora-lexer/) | High-speed tokenizer for `.kora` and `.kr` | Phase 1 |
| [`kora-parser`](./kora-parser/) | Recursive-descent & Pratt expression parser | Phase 1 |
| [`kora-hir`](./kora-hir/) | High-Level IR and symbol resolution | Phase 2 |
| [`kora-ty`](./kora-ty/) | Static type system and architectural purity rules | Phase 3 |
| [`kora-ownership`](./kora-ownership/) | DI cycle detection and safety invariants | Phase 4 |
| [`kora-codegen`](./kora-codegen/) | Emits verified native Rust source code | Phase 5 |
| [`kora-runtime`](./kora-runtime/) | Async channels, actors (`tell`/`ask`), task groups | Runtime |
| [`kora-http`](./kora-http/) | Ultra-low latency HTTP engine (Axum/Tokio) | Subsystem |
| [`kora-orm`](./kora-orm/) | 100% prepared parameterized query builder & SQLite/PG | Subsystem |
| [`kora-auth`](./kora-auth/) | Argon2id hashing, HMAC-SHA256 JWT, RBAC guards | Subsystem |
| [`kora-queue`](./kora-queue/) | Background job queue with retry and DLQ | Subsystem |
| [`kora-realtime`](./kora-realtime/) | WebSockets and live event pub/sub streaming | Subsystem |
| [`kora-observability`](./kora-observability/) | Metrics counters, tracing, and structured logging | Subsystem |
| [`kora-pkg`](./kora-pkg/) | Distributed package manager & SHA-256 lockfile | Tooling |
| [`kora-fmt`](./kora-fmt/) | Source code formatter for `.kora` and `.kr` | Tooling |
| [`kora-lsp`](./kora-lsp/) | Language Server Protocol for editor completions | Tooling |
| [`kora-testing`](./kora-testing/) | Isolated test runner and assertion framework | Tooling |
| [`kora-profiler`](./kora-profiler/) | Sampling CPU profiler & memory tracker | Tooling |
| [`kora-bench`](./kora-bench/) | Quantitative benchmark comparing Kora, Go, Rust, Node | Tooling |
| [`kora-cli`](./kora-cli/) | Unified command-line interface executable (`kora`) | Entrypoint |

---

## 🛡️ Workspace-Wide Security Standards

* **Zero Unsafe Code:** Enforced workspace-wide via `[workspace.lints.rust] unsafe_code = "forbid"`.
* **Single-Null Principle:** Enforced at type checker level (`undefined` forbidden, `E2005`).
* **Struct-Free Paradigm:** Pure `model` (immutable data) and `class` (stateful DI services) only.
* **Dual Extension Equivalence:** Seamless 100% feature parity between `.kora` and `.kr`.
''')

# Generate each crate README.md
for crate_name, info in crates.items():
    p = os.path.join('crates', crate_name, 'README.md')
    types_md = '\n'.join([f'- `{t}`' for t in info['types']])
    content = f'''# {crate_name}

> **{info['title']}**

{info['purpose']}

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** {info['pipeline']}
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

{types_md}

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** {info['security']}

---

## 🚀 How to Contribute & Improve

{info['improve']}

### Testing this Crate
```bash
cargo test -p {crate_name}
cargo clippy -p {crate_name} --all-targets -- -D warnings
```
'''
    with open(p, 'w') as f:
        f.write(content)

print(f'Successfully generated {len(crates) + 1} README.md files!')

