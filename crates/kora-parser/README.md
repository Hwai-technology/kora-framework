# kora-parser

> **Kora Recursive-Descent & Pratt Expression Parser**

Constructs the full AST from tokens using Pratt operator precedence parsing, auto-constructor synthesis, parameter promotion, and error recovery.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 1 (Syntactic Analysis) — Produces verified Module ASTs with error synchronization.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `parse_module(file_id, src, bag)`
- `Parser`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Recursion depth limits to prevent stack overflow from adversarial deeply nested parenthesis or bracket attacks.

---

## 🚀 How to Contribute & Improve

Implement tree-sitter or incremental parsing support to achieve sub-millisecond editor re-parsing on keystroke.

### Testing this Crate
```bash
cargo test -p kora-parser
cargo clippy -p kora-parser --all-targets -- -D warnings
```
