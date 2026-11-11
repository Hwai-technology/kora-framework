# Kora CLI Command Reference

The `kora` command-line tool provides a unified interface for checking, running, building, testing, benchmarking, and scaffolding Kora applications.

---

## Global Flags

| Flag | Shorthand | Description |
|:---|:---|:---|
| `--help` | `-h`, `help` | Prints the complete usage manual. |
| `--version` | `-v`, `-V`, `version` | Displays the Kora compiler toolchain version. |

---

## Direct Script Execution

You can run any `.kora` or `.kr` file directly (Python-style):

```bash
kora main.kora
kora examples/hello.kora
```

---

## Commands

### `kora check [file.kora]`
Analyzes code syntax, performs full static type checking, validates single-null rules, and checks Clean Architecture boundaries with zero code generation.

```bash
kora check src/main.kora
```

---

### `kora run [file.kora]`
Compiles the application to a native release binary inside `.kora-build/` and executes it immediately. If omitted, it automatically resolves `src/main.kora` or the entry point declared in `kora.toml`.

```bash
kora run
kora run src/main.kora
```

---

### `kora watch [file.kora]` / `kora w`
Launches the development file watcher with sub-second hot reloading. Re-checks and re-runs the application instantaneously whenever source files change.

```bash
kora watch
```

---

### `kora build [file.kora]`
Compiles the project into an optimized standalone native executable binary in `.kora-build/target/release/kora_app`.

```bash
kora build src/main.kora
```

---

### `kora fmt [path] [--check]`
Canonical source code formatter.
- `kora fmt .`: Formats all `.kora` and `.kr` files in place according to 4-space indentation rules.
- `kora fmt --check`: Exits with code 1 if any unformatted files exist (used in CI/CD pipelines).

```bash
kora fmt .
kora fmt --check
```

---

### `kora init [dir]`
Initializes a new Clean Architecture project in the current working directory (`.`) or specified folder without overwriting existing files.

```bash
kora init
```

---

### `kora create <project-name>` / `kora new <name>`
Scaffolds a new Clean Architecture backend service inside a new `<project-name>/` directory.

```bash
kora create payment-service
```

---

### `kora g feature <name>` / `kora generate`
Generates a complete domain feature folder inside `src/features/<name>/`:
- `models/<name>_entity.kora`
- `models/<name>_dto.kora`
- `interfaces/<name>_service_interface.kora`
- `events/<name>_events.kora`
- `<name>.service.kora`
- `<name>.controller.kora`

```bash
kora g feature invoices
```

---

### `kora lsp`
Starts the JSON-RPC 2.0 Language Server Protocol (LSP) daemon over `stdio` for integration with VS Code, Neovim, Emacs, and other editors.

```bash
kora lsp
```

---

### `kora swagger [file.kora]` / `kora openapi`
Automatically discovers all routes, controllers, DTOs, `@entity` models, and enums from code AST and synthesizes an OpenAPI 3.1.0 specification and an interactive, standalone Swagger UI without writing any manual annotations or configs.
- Auto-generates `openapi.json` and `swagger-ui.html`.
- Accepts `--out <filename>` to customize the output path.

```bash
kora swagger
kora swagger src/main.kora --out dist/openapi.json
```

---

### `kora update`
Recompiles the compiler from source in release mode, updates the binary in `~/.local/bin/kora`, refreshes shell completions (Bash, Zsh, Fish), and syncs IDE extensions.

```bash
kora update
```

---

### `kora bench`
Runs reproducible quantitative benchmark suites comparing Kora to Rust (Axum), Go (Gin), Node.js (Fastify), and PHP (Laravel).

```bash
kora bench
```

---

### `kora pkg <action> [args]`
Package manager for Kora:
- `kora pkg install`: Resolves and installs dependencies from `kora.toml` and creates `kora.lock`.
- `kora pkg add <package>`: Adds a new dependency to `kora.toml`.
- `kora pkg remove <package>`: Removes a dependency.
- `kora pkg list`: Lists installed packages.

---

### `kora db <action> [file]`
Database migration and schema synchronization utility:
- `kora db migrate`: Executes pending SQL migrations.
- `kora db sync`: Auto-syncs database tables with `@entity` models.
- `kora db seed`: Populates database with initial seed data.
- `kora db truncate`: Truncates table rows safely.
- `kora db drop`: Drops database tables.
- `kora db refresh`: Resets and re-seeds database.

---

### `kora codegen [file.kora]`
Dumps the generated `#![forbid(unsafe_code)]` safe Rust source code to stdout without building.

```bash
kora codegen src/main.kora
```

---

### `kora ast [file.kora]`
Dumps the parsed Abstract Syntax Tree representation to stdout.

---

### `kora tokens [file.kora]`
Dumps the lexical token stream produced by `kora_lexer`.

