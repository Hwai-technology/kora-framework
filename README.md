# Kora

> **Write Less. Build Faster. Run Smarter.**
> A modern compiled backend programming language and integrated framework, powered by Rust.

---

## Philosophy

Kora bridges developer productivity (like Laravel and TypeScript) with native compiled performance and safety (like Rust and Go), without exposing manual memory lifetime annotations or boilerplate framework wiring.

- **Clean Modern Syntax**: Optional semicolons, clean blocks, algebraic types, and intuitive declarations.
- **Strong Static Typing + Type Inference**: Zero dynamic guesswork; types are inferred locally and verified strictly at compile time.
- **3-Tier Ownership Inference (Zero-GC)**: Automatic local borrows (`&T` / `&mut T`), deterministic moves, and automatic thread-safe shared handles (`Arc<T>`) across async/DI boundaries—never exposing raw Rust borrow-checker errors to the Kora developer.
- **Hybrid Object & Data Model**:
  - `struct` and `model` for pure value/entity data structures.
  - `class` for stateful services with built-in compile-time Dependency Injection (`init(...)`).
  - `interface` for polymorphism without fragile deep class inheritance.
- **Integrated Backend Ecosystem**: Zero-config HTTP (`Axum`/`Hyper` + `Tokio`), compile-time validated ORM (`SQLx`/PostgreSQL), security, queues, realtime, and OpenTelemetry observability.

---

## Workspace Architecture

All compiler, runtime, framework, and tooling components live in a unified Cargo Workspace under `crates/`:

```text
kora/
├── crates/
│   ├── kora-span/          # SourceMap, FileId, BytePos, Span, Line/Column lookup
│   ├── kora-diagnostics/   # Compiler error codes, labeled spans, snippet renderer
│   ├── kora-ast/           # Complete Abstract Syntax Tree
│   ├── kora-lexer/         # Lexical analyzer with optional-semicolon newline awareness
│   ├── kora-parser/        # Recursive-descent + Pratt expression parser
│   ├── kora-hir/           # High-Level IR & multi-pass lexical symbol resolution
│   ├── kora-ty/            # Strong static type system, inference & DI/interface verifier
│   └── kora-cli/           # Official `kora` command-line tool
├── docs/
│   ├── architecture.md     # Complete system architecture & 8-phase roadmap
│   ├── grammar.ebnf        # Formal EBNF grammar specification
│   └── language-tour.md    # Kora syntax & type system tour
├── examples/
│   ├── hello.kora          # Core language, structs, enums, pattern matching
│   └── users_api.kora      # Models, DI service classes, interfaces, HTTP routes
└── .agent/                 # Multi-agent engineering rules, standards & specialist roles
```

---

## Quick Start

```bash
# Type-check a Kora source file
cargo run -p kora-cli -- check examples/hello.kora
cargo run -p kora-cli -- check examples/users_api.kora

# Inspect inferred symbol types and verified declarations
cargo run -p kora-cli -- types examples/users_api.kora

# Inspect the parsed AST or lexical token stream
cargo run -p kora-cli -- ast examples/hello.kora
cargo run -p kora-cli -- tokens examples/hello.kora

# Initialize a new Kora project scaffold
cargo run -p kora-cli -- init my-api
```

---

## Development Roadmap

| Phase | Milestone | Status |
| :--- | :--- | :--- |
| **Phase 1** | **Language Foundation** (`span`, `diagnostics`, `lexer`, `ast`, `parser`, `hir`, `ty`, `cli`) | **Active / Implemented (`v0.1-alpha.1`)** |
| **Phase 2** | **Compiler Backend & Runtime** (`ownership`, `codegen` -> Rust/Cargo, `tokio` runtime) | Planned |
| **Phase 3** | **Backend Core** (`kora-http` on Axum, routing, middleware, JSON validation, OpenAPI) | Planned |
| **Phase 4** | **Database & ORM** (`kora-orm` on SQLx/PostgreSQL, query builder, transactions, migrations) | Planned |
| **Phase 5** | **Advanced Language** (Generics on user types, extension methods, modules, async/channels) | Planned |
| **Phase 6** | **Production Features** (Auth/RBAC, queues, cache, WebSockets/SSE, OpenTelemetry) | Planned |
| **Phase 7** | **Developer Ecosystem** (Package manager, `kora-lsp`, formatter, debugger, profiler) | Planned |
| **Phase 8** | **Stabilization & Release** (Reproducible benchmarks vs Rust/Go/Fastify/Laravel, audit) | Planned |

