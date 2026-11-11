# kora-testing

> **Kora Built-in Unit & Integration Test Runner**

Lightweight test harness for Kora applications with assertion macros, test discovery, and isolated database sandboxing.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Testing Tooling — Provides native testing primitives.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `TestRunner`
- `TestReport`
- `TestCase`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Isolates test environments using in-memory databases to eliminate destructive mutations on production databases.

---

## 🚀 How to Contribute & Improve

Add property-based fuzz testing generation (QuickCheck style) for Kora functions.

### Testing this Crate
```bash
cargo test -p kora-testing
cargo clippy -p kora-testing --all-targets -- -D warnings
```
