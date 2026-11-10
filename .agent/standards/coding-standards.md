# Coding Standards (Rust Implementation of Kora)

1. **Zero `unwrap()` / `expect()` on User Input**: Compiler stages (`lexer`, `parser`, `hir`, `ty`, `codegen`) must never panic on malformed `.kora` source files; always emit a `Diagnostic` and recover gracefully.
2. **Clippy & Formatting Cleanliness**: All crates must pass `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all -- --check`.
3. **Explicit Error Codes**: Every compiler diagnostic uses a stable code namespace:
   - `E1xxx`: Lexical errors
   - `E2xxx`: Syntax / Parser errors
   - `E3xxx`: Name resolution / HIR semantic errors
   - `E4xxx`: Static type, interface conformance, and DI errors
   - `E5xxx`: Ownership & async escape errors (Phase 2+)

