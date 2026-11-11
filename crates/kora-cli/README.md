# kora-cli

> **Kora Command-Line Interface (kora)**

The unified developer CLI providing check, types, ast, tokens, codegen, build, run, bench, db, and pkg commands for .kora and .kr files.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Primary Entrypoint — User-facing executable for all developer operations.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `main()`
- `cmd_check`
- `cmd_build`
- `cmd_run`
- `cmd_db`
- `cmd_pkg`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Validates all file paths, enforces architectural purity rules, and builds binaries with #[forbid(unsafe_code)].

---

## 🚀 How to Contribute & Improve

Add interactive watch mode (kora watch) for instant automatic recompilation and hot-reloading on file change.

### Testing this Crate
```bash
cargo test -p kora-cli
cargo clippy -p kora-cli --all-targets -- -D warnings
```
