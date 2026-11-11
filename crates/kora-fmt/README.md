# kora-fmt

> **Kora Code Formatter Engine**

Opinionated code formatter for Kora source files (.kora and .kr) guaranteeing consistent syntax style, indenting, and spacing.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Developer Tooling — Enforces unified formatting across teams.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `format_source(src, options)`
- `FormatOptions`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Idempotent formatting: running the formatter multiple times produces identical output without AST alterations.

---

## 🚀 How to Contribute & Improve

Add parallel formatting using Rayon to format large repositories of thousands of files in milliseconds.

### Testing this Crate
```bash
cargo test -p kora-fmt
cargo clippy -p kora-fmt --all-targets -- -D warnings
```
