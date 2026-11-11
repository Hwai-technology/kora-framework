# kora-lsp

> **Kora Language Server Protocol (LSP) Engine**

Implements the Language Server Protocol for IDEs: code completion, go-to-definition, hover documentation, and real-time diagnostics.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** IDE Integration — Powers VS Code, Antigravity IDE, Neovim, and Helix editors.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `LspServer`
- `CompletionHandler`
- `DiagnosticsHandler`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Non-blocking worker loop prevents editor UI freezing during heavy compiler analyses.

---

## 🚀 How to Contribute & Improve

Add workspace-wide rename refactoring and semantic token highlighting.

### Testing this Crate
```bash
cargo test -p kora-lsp
cargo clippy -p kora-lsp --all-targets -- -D warnings
```
