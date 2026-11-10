# Mandatory Rule: Atomic Language Feature Evolution

No AI agent or human contributor may introduce or modify a Kora language feature without updating **every** affected layer in the same unit of work:

1. **Grammar (`docs/grammar.ebnf`)**: Formal EBNF production rules must match the parser 1:1.
2. **AST (`crates/kora-ast`)**: Typed syntax nodes with accurate `Span` tracking on every node.
3. **Lexer & Parser (`crates/kora-lexer`, `crates/kora-parser`)**: Tokenization, operator precedence, optional-semicolon newline rules, and parser recovery.
4. **HIR & Name Resolution (`crates/kora-hir`)**: Scope registration, shadowing/redeclaration rules, and symbol resolution.
5. **Type Checker (`crates/kora-ty`)**: Static type checking, bidirectional type inference, exhaustiveness or conformance checks, and structured diagnostics.
6. **Codegen (`crates/kora-codegen`, Phase 2+)**: Deterministic lowering to warning-free, safe Rust.
7. **Tests & Documentation**: Positive tests, negative diagnostic tests, and updated `docs/language-tour.md`.

Partial "syntax-only" additions that panic or silently skip downstream compiler stages are strictly forbidden.

