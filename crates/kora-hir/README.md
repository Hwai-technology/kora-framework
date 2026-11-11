# kora-hir

> **Kora High-Level Intermediate Representation & Symbol Resolver**

Lowers AST into High-Level IR, constructs lexical symbol scopes, resolves variable bindings, and checks mutability rules (let vs var).

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 2 (Semantic Lowering) — Sits between syntax parsing and type checking.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `HirModule`
- `HirItem`
- `HirExpr`
- `HirStmt`
- `lower_and_resolve(ast, bag)`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Immutable-by-default enforcement: strictly rejects mutations on let-bound variables (E3002).

---

## 🚀 How to Contribute & Improve

Add dead code elimination and constant propagation passes at the HIR level before type checking.

### Testing this Crate
```bash
cargo test -p kora-hir
cargo clippy -p kora-hir --all-targets -- -D warnings
```
