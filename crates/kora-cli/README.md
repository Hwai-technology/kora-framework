# kora-cli

> **Kora Command-Line Interface (kora)**

The unified developer CLI providing check, types, ast, tokens, codegen, build, run, watch, fmt, lsp, swagger, bench, db, pkg, init, create, and update commands for .kora and .kr files.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Primary Entrypoint — User-facing executable for all developer operations and architectural scaffolding.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `main()`
- `cmd_check`
- `cmd_build`
- `cmd_run`
- `cmd_watch`
- `cmd_fmt`
- `cmd_lsp`
- `cmd_swagger`
- `cmd_bench`
- `cmd_db`
- `cmd_pkg`
- `cmd_init`
- `cmd_create`
- `cmd_update`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Validates all file paths, catches missing imports/files with actionable hints, enforces architectural purity rules, and builds binaries with #[forbid(unsafe_code)].

---

## 🚀 How to Contribute & Improve

Add interactive project migration and deployment integrations for cloud providers directly from CLI.

### Testing this Crate
```bash
cargo test -p kora-cli
cargo clippy -p kora-cli --all-targets -- -D warnings
```
