# Changelog

All notable changes to the **Kora** ecosystem will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-alpha.1] - 2026-11-10

### Added
- **Architecture & Governance**:
  - Unified Cargo Workspace layout under `crates/`.
  - `.agent/` multi-agent governance rules, engineering standards, and specialized agent specifications.
  - Formal language specification (`docs/architecture.md`), EBNF grammar (`docs/grammar.ebnf`), and language tour (`docs/language-tour.md`).
- **Phase 1 — Language Foundation**:
  - `kora-span`: Source mapping, byte spans, and line/column resolution.
  - `kora-diagnostics`: Structured compiler diagnostics with source snippets, primary/secondary labels, error codes, and hints.
  - `kora-lexer`: Complete lexical analyzer with newline-aware optional semicolon support, literals, operators, and lexical diagnostics.
  - `kora-ast`: Full Abstract Syntax Tree covering `struct`, `model`, `class` (DI services), `interface`, `enum`, `fn`, `let`/`var`, `match`, closures, and expressions.
  - `kora-parser`: Recursive-descent + Pratt expression parser with error recovery.
  - `kora-hir`: High-Level Intermediate Representation and multi-pass lexical scope / symbol resolution.
  - `kora-ty`: Strong static type checker with bidirectional type inference, `Option<T>` / `Result<T, E>` algebraic verification, interface conformance verification, and compile-time Dependency Injection constructor checks.
  - `kora-cli`: Developer CLI supporting `kora check`, `kora tokens`, `kora ast`, `kora types`, and `kora init`.

