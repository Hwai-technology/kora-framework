# kora-codegen

> **Kora Native Rust Code Generator & OpenAPI Synthesizer**

Emits mathematically safe, highly optimized native Rust source code from verified HIR, synthesizes automatic OpenAPI 3.1 & Swagger UI, and compiles via Cargo onto dedicated port 7582.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 5 (Code Generation) — Bridges Kora frontend with Rust native backend and web ecosystem.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `generate_rust(hir, types, ownership, name)`
- `compile_native(rust_code, target_dir)`
- `has_http_routes`
- `Database`
- `Request`
- `send`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Generates 100% unsafe-free Rust code. Emits #[forbid(unsafe_code)] at the top of every generated Rust file.

---

## 🚀 How to Contribute & Improve

Add direct LLVM IR or Cranelift emission backend to bypass cargo compilation for rapid development iteration.

### Testing this Crate
```bash
cargo test -p kora-codegen
cargo clippy -p kora-codegen --all-targets -- -D warnings
```
