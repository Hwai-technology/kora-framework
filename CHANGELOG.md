# Changelog

All notable changes to the **Kora** programming framework ecosystem will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-11

### Added
- **Automated OpenAPI 3.1 & Interactive Swagger UI Generation**:
  - Direct AST inspection synthesizes OpenAPI 3.1.0 specifications (`openapi.json`) without requiring manual Swagger annotations.
  - Automatically correlates routes (`GET`, `POST`, `PUT`, `DELETE`), URL path parameters (`:id` -> `{id}`), request body DTOs, and response envelope schemas.
  - Bundles standalone offline Swagger UI (`swagger-ui.html`) served automatically during `kora run` and outputted during `kora build`.
  - Added dedicated CLI command: `kora swagger [entrypoint] [--out <file>]` and `kora openapi`.
- **Dedicated Application Port `7582`**:
  - Migrated server binding away from colliding ports (`8080`, `3000`, `4200`) to dedicated port `7582` (representing ASCII `K`=75, `R`=82 for Kora).
  - Configurable at runtime via `PORT` or `KORA_PORT` environment variables and project-level `kora.toml`.
- **Automatic Health & Root Endpoints**:
  - Out-of-the-box `GET /` (Welcome banner, version, docs link) and `GET /health` (status, uptime, system metrics) generated for all scaffolded and run applications.
- **Enterprise Project Scaffolding Suite (`kora create` & `kora init`)**:
  - Automatically generates complete production configuration files: `.editorconfig`, `Dockerfile`, `docker-compose.yml`, `.env.example`, `.env`, `kora.toml`, `.gitignore`, and `README.md`.
  - Emits clean modular architecture directory structure: `src/enums/`, `src/models/`, `src/interfaces/`, `src/database/`, `src/middleware/`, `src/shared/`, and `src/modules/`.
- **Interactive Multi-Choice Installer & Uninstaller**:
  - `install.sh` / `scripts/setup.sh` now detects existing installations and presents interactive options: `[1] Update / Refresh`, `[2] Clean Reinstall`, `[3] Uninstall`, `[4] Exit`.
  - Non-interactive automation flags supported: `--clean`, `--reinstall`, `--uninstall`, `--yes`.
  - Standalone uninstall (`scripts/uninstall.sh`) and fast update (`scripts/update.sh`) scripts.
- **Strict Clean Architecture Compiler Diagnostics (`E5001`-`E5004`)**:
  - `E5001`: Purity of `enums/` folder (no functions, routes, or classes permitted).
  - `E5002`: Purity of `models/` & `interfaces/` (no runtime business logic or server execution permitted).
  - `E5003`: Segregation in controllers & services (inline `enum` or `model` definitions forbidden; must import).
  - `E5004`: Clean root entry (`main.kora` cannot define inline models/enums/interfaces; composition only).
- **Struct-Free Architecture**:
  - Deprecation and removal of the `struct` keyword in favor of intentional primitives: `model` (pure immutable data schemas / DTOs) and `class` (stateful services with Dependency Injection).
- **Exclusive `null` Semantics (`E2005`)**:
  - JavaScript-style `undefined` is strictly forbidden and triggers compiler error `E2005`.
  - Single, unambiguous `null` primitive natively mapped to Rust's zero-cost `Option<T>`.
- **Comprehensive Documentation Suite**:
  - 10+ detailed guides added in `docs/` covering Quickstart, Models & Classes, Pattern Matching, Dependency Injection, CLI Reference, Compiler Diagnostics, OpenAPI/Swagger, Realtime Events, IDE Integration, and Practical Recipes.
  - Standardized English documentation across all 24 workspace crates and showcase applications.

### Changed
- Refactored `kora-cli` run loop to avoid raw rustc compiler errors leaking to end-users; errors are transformed into actionable Kora diagnostics with ASCII diagrams and helpful remediation suggestions.
- Standardized wire formatting across JSON serializers to strict `kebab-case`.

---

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
  - `kora-ast`: Full Abstract Syntax Tree covering `model`, `class` (DI services), `interface`, `enum`, `fn`, `let`/`var`, `match`, closures, and expressions.
  - `kora-parser`: Recursive-descent + Pratt expression parser with error recovery.
  - `kora-hir`: High-Level Intermediate Representation and multi-pass lexical scope / symbol resolution.
  - `kora-ty`: Strong static type checker with bidirectional type inference, `Option<T>` / `Result<T, E>` algebraic verification, interface conformance verification, and compile-time Dependency Injection constructor checks.
  - `kora-cli`: Developer CLI supporting `kora check`, `kora tokens`, `kora ast`, `kora types`, `kora run`, `kora build`, `kora create`, and `kora init`.
