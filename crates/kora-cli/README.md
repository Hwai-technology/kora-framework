# kora-cli

> **Kora Command-Line Interface (`kora`)**

The unified developer toolchain providing `run`, `watch`, `fmt`, `lsp`, `check`, `types`, `ast`, `tokens`, `codegen`, `build`, `bench`, `db`, `create`, and `g feature` commands for `.kora` and `.kr` files.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Primary Entrypoint — User-facing executable for all compiler operations and developer workflows.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions symmetrically.

---

## 🧩 Key Commands & Public API

| Command | Usage | Description |
|:---|:---|:---|
| **Direct Run** | `kora app.kora` | Python-style instant execution without typing `run`. |
| **`run`** | `kora run [path]` | Auto-discovers entrypoint or scaffolds working starter `main.kora`. |
| **`watch` (`w`)** | `kora watch [path]` | Monitors project files and instantaneously re-compiles on change. |
| **`fmt`** | `kora fmt [path] [--check]` | Canonicalizes source formatting and indentation (CI `--check` flag). |
| **`lsp`** | `kora lsp` | Starts the JSON-RPC Language Server Protocol daemon for IDEs. |
| **`check`** | `kora check <file>` | Runs syntax, static type inference, and architectural purity checks. |
| **`build`** | `kora build <file>` | Compiles source into an optimized standalone native binary in `.kora-build/`. |
| **`create` / `init`** | `kora create <name>` | Scaffolds a production-grade Clean Architecture project. |
| **`g feature`** | `kora g feature <name>` | Generates a complete CRUD feature with models, DTOs, service, controller, and events. |
| **`bench`** | `kora bench` | Executes benchmark against Rust, Go, Node, and Laravel. |

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Validates all file paths, isolates build workspaces, prevents unsafe shell executions, and verifies architectural boundaries at compile time.

---

## 🚀 How to Contribute & Improve

- Add interactive terminal debugger integration (`kora debug`).
- Add parallel multi-core compilation passes.

### Testing this Crate
```bash
cargo test -p kora-cli
cargo clippy -p kora-cli --all-targets -- -D warnings
```
