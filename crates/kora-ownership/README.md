# kora-ownership

> **Kora Compile-Time Ownership & DI Cycle Analyzer**

Validates lifecycle semantics, detects circular dependencies in Service/Controller Dependency Injection graphs, and ensures zero leaks.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 4 (Safety Analysis) — Guarantees acyclic service instantiation before native code emission.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `analyze_ownership(hir, types, bag)`
- `OwnershipReport`
- `DiGraph`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Guarantees absence of memory leaks and circular reference deadlock by rejecting DI graph cycles at compile time.

---

## 🚀 How to Contribute & Improve

Add compile-time tracking for async task handle ownership and actor message buffer lifetimes.

### Testing this Crate
```bash
cargo test -p kora-ownership
cargo clippy -p kora-ownership --all-targets -- -D warnings
```
