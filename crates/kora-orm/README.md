# kora-orm

> **Kora Type-Safe ORM & Query Builder**

High-performance database abstraction layer atop SQLx, supporting PostgreSQL, SQLite in-memory testing, auto-migrations, and seeders.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Data Persistence — Bridges @entity models with relational databases.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `QueryBuilder`
- `Database`
- `SqlValue`
- `FilterClause`
- `FilterOp`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** 100% prepared parameterized queries (WHERE col = ?1). Eliminates SQL injection by design.

---

## 🚀 How to Contribute & Improve

Add compile-time SQL query syntax verification and automatic spatial (PostGIS) column mapping.

### Testing this Crate
```bash
cargo test -p kora-orm
cargo clippy -p kora-orm --all-targets -- -D warnings
```
