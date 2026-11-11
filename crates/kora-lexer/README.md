# kora-lexer

> **Kora Lexical Tokenizer Engine**

High-performance lexical scanner transforming raw UTF-8 Kora source text (.kora and .kr) into a strongly typed stream of tokens and trivia.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Phase 1 (Frontend Tokenization) — Precedes the recursive-descent parser.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `Token`
- `TokenKind`
- `tokenize(file_id, src, bag)`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Bounded string and comment scanning with defense against unclosed delimiters and ReDoS token patterns.

---

## 🚀 How to Contribute & Improve

Add SIMD-accelerated whitespace and identifier scanning utilizing AVX2/NEON intrinsics for multi-gigabyte parsing speed.

### Testing this Crate
```bash
cargo test -p kora-lexer
cargo clippy -p kora-lexer --all-targets -- -D warnings
```
