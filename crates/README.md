# Kora Compiler & Runtime Architecture (`crates/`)

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
