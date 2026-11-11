# kora-ty

> **Kora Static Type System & Clean Architecture Verifier**

Performs static type checking, interface conformance verification, pattern matching exhaustiveness checks, and architectural boundary purity enforcement.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 3 (Type & Architectural Verification) — Validates type correctness before codegen.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `TypeCheckOutput`
- `Type`
- `check_module(hir, bag)`
- `ClassInfo`
- `ModelInfo`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Enforces single-null semantics: undefined is rejected with E2005. Enforces clean architectural separation (E5001-E5004).

---

## 🚀 How to Contribute & Improve

Add type inference for higher-kinded types and structural record typing for lightweight JSON payloads.

### Testing this Crate
```bash
cargo test -p kora-ty
cargo clippy -p kora-ty --all-targets -- -D warnings
```
