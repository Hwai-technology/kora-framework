# kora-lsp

> **Kora Language Server Protocol (LSP) Engine**

Full JSON-RPC 2.0 Language Server Protocol daemon powering IDE integration for Visual Studio Code, Antigravity IDE, Neovim, Helix, and Emacs.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Editor & IDE Integration — Provides intelligent completion, hover docs, formatting, and diagnostics over standard I/O.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Protocol Capabilities & Methods Handled

- **`initialize`**: Declares hover, completion (`.`, `@`, `:` triggers), and document formatting capabilities.
- **`textDocument/completion`**: Provides semantic completion items for language keywords (`model`, `class`, `interface`, `fn`, `actor`), primitive types, and decorators.
- **`textDocument/hover`**: Delivers rich markdown documentation and performance characteristics for hovered symbols.
- **`textDocument/formatting`**: Formats source text on save or shortcut via `kora-fmt`.
- **`shutdown`**: Clean daemon teardown.

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Strict JSON-RPC schema parsing ensures resilience against malformed editor requests with zero memory leaks.

---

## 🚀 How to Contribute & Improve

- Add multi-file semantic rename refactoring.
- Add signature help for method parameter completions.

### Testing this Crate
```bash
cargo test -p kora-lsp
cargo clippy -p kora-lsp --all-targets -- -D warnings
```
