# kora-diagnostics

> **Kora Compiler Diagnostic Reporting Engine**

Renders rich compiler diagnostics with ANSI color formatting, underlined source snippets, primary/secondary labels, error codes, and help notes.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 0 (Foundation) — Diagnostic aggregation bus used across Lexer, Parser, HIR, Type Checker, and Ownership analyzer.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `Diagnostic (error, warning, note)`
- `DiagnosticBag`
- `Label`
- `Severity`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Zero panic on out-of-bounds spans; safely clips truncated source snippets and sanitizes terminal control characters.

---

## 🚀 How to Contribute & Improve

Integrate machine-readable SARIF (Static Analysis Results Interchange Format) output for CI/CD integrations.

### Testing this Crate
```bash
cargo test -p kora-diagnostics
cargo clippy -p kora-diagnostics --all-targets -- -D warnings
```
