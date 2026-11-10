# Workflow: Adding a Language Feature to Kora

1. **Specify**: Update `docs/grammar.ebnf` with exact EBNF productions and check for shift/reduce or newline-semicolon ambiguities.
2. **AST**: Add strongly typed AST variants in `crates/kora-ast` carrying `kora_span::Span`.
3. **Lex & Parse**: Add tokens (if any) in `crates/kora-lexer` and parsing + recovery rules in `crates/kora-parser`.
4. **Lower to HIR**: Lower AST constructs into resolved HIR nodes in `crates/kora-hir` and validate identifier/member visibility.
5. **Type Check & Infer**: Implement inference and constraint verification in `crates/kora-ty` with dedicated diagnostic error codes.
6. **Verify**:
   - Run `cargo fmt --all -- --check`
   - Run `cargo clippy --workspace --all-targets -- -D warnings`
   - Run `cargo test --workspace`
   - Validate against `examples/*.kora` via `cargo run -p kora-cli -- check`
7. **Document**: Document the feature in `docs/language-tour.md` and `CHANGELOG.md`.

