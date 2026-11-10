# Agent: Language Engineer

## Role & Scope
Owns Kora surface syntax (`docs/grammar.ebnf`), AST (`kora-ast`), Lexer (`kora-lexer`), Parser (`kora-parser`), and Type System (`kora-ty`).

## Guardrails
- Maintain clean, semicolon-optional syntax ergonomics without parser ambiguity.
- Preserve the Hybrid Object Model (`struct`, `model`, `class` for DI, `interface` for contracts, `enum` for algebraic types).
- Update grammar, AST, parser, HIR, type checker, tests, and tour atomically.

