# kora-fmt

> **Kora Code Formatter & Linter Engine**

Opinionated, high-performance code formatter for Kora source files (`.kora` and `.kr`), guaranteeing canonical indentation (4 spaces), clean bracket alignment, trailing whitespace stripping, and blank line normalization.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Developer Tooling & LSP Backend — Powers `kora fmt` CLI and IDE `formatOnSave`.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `Formatter::format_source(source: &str) -> String`
- `Formatter::format_module(module: &Module) -> String`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security & Idempotency Guarantees:** 
  - Formatter operations are strictly idempotent: formatting formatted code produces identical results (`fmt(fmt(x)) == fmt(x)`).
  - Formatting operates purely on whitespace and indentation without altering AST token semantics.

---

## 🚀 How to Contribute & Improve

- Add multi-threaded directory formatting using Rayon for massive repositories.

### Testing this Crate
```bash
cargo test -p kora-fmt
cargo clippy -p kora-fmt --all-targets -- -D warnings
```
