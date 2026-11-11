# kora-span

> **Kora Span & Source Mapping Engine**

Manages source code spans, byte offsets, file IDs, line/column calculations, and cross-file source registries for high-precision error diagnostics.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 0 (Foundation) — Referenced by every compiler phase to attach exact file coordinates to AST nodes and tokens.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `Span (start, end, file_id)`
- `FileId`
- `SourceMap (source code registry and line indexer)`
- `Location (line, column)`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Guarantees panic-free slice indexing through checked byte offsets and safe multi-byte UTF-8 boundary assertions.

---

## 🚀 How to Contribute & Improve

Add memory-mapped file support for gigabyte-scale multi-file source trees and incremental source cache invalidation.

### Testing this Crate
```bash
cargo test -p kora-span
cargo clippy -p kora-span --all-targets -- -D warnings
```
