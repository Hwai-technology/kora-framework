# kora-ast

> **Kora Abstract Syntax Tree (AST)**

Defines the strongly typed Abstract Syntax Tree representation for Kora programs, including models, classes, interfaces, enums, decorators, and expressions.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 1 (Syntax Representation) — The direct product of the recursive-descent parser.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `Module`
- `Item (Model, Class, Interface, Enum, Fn, Import)`
- `Expr (Binary, Unary, Call, Closure, Match)`
- `Stmt`
- `TypeExpr`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Struct-free invariant enforcement: only Model and Class representations exist; struct keyword is structurally prohibited (E2003).

---

## 🚀 How to Contribute & Improve

Add AST visitor and folder macros to simplify compiler pass transformations without manual tree walking.

### Testing this Crate
```bash
cargo test -p kora-ast
cargo clippy -p kora-ast --all-targets -- -D warnings
```
