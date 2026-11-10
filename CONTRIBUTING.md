# Contributing to Kora

Thank you for contributing to **Kora** — *Write Less. Build Faster. Run Smarter.*

## Golden Rule of Language Evolution

No language construct or syntax change may be merged unless **all** layers are updated atomically in the same change:

1. **Formal Grammar**: `docs/grammar.ebnf`
2. **AST & Span Coverage**: `crates/kora-ast`
3. **Lexer & Parser**: `crates/kora-lexer` and `crates/kora-parser`
4. **HIR & Name Resolution**: `crates/kora-hir`
5. **Type System & Inference**: `crates/kora-ty`
6. **Code Generation (Phase 2+)**: `crates/kora-codegen`
7. **Automated Tests**: Positive, negative (diagnostic), and edge-case tests
8. **Documentation**: `docs/language-tour.md` and `CHANGELOG.md`

## Pre-Commit Verification Checklist

Every pull request must pass cleanly with zero warnings:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

