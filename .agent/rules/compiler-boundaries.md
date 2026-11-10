# Mandatory Rule: Compiler Crate Boundaries & Core Protection

1. **Single Source of Truth in `crates/`**: Do not duplicate compiler crates in a top-level `compiler/` source tree. Every library or binary belongs in `crates/kora-*` with its own `Cargo.toml` inheriting workspace metadata.
2. **Strict Unidirectional Dependency Graph**:
   - `kora-span` depends on nothing in the workspace.
   - `kora-diagnostics` depends only on `kora-span`.
   - `kora-ast` depends only on `kora-span`.
   - `kora-lexer` depends on `kora-span` and `kora-diagnostics`.
   - `kora-parser` depends on `kora-span`, `kora-diagnostics`, `kora-ast`, and `kora-lexer`.
   - `kora-hir` depends on `kora-span`, `kora-diagnostics`, and `kora-ast`.
   - `kora-ty` depends on `kora-span`, `kora-diagnostics`, `kora-ast`, and `kora-hir`.
   - Cyclic dependencies or upward imports are forbidden.
3. **Zero Raw `rustc` Diagnostic Leakage**: Any semantic, type, interface, DI, or ownership violation in `.kora` code must be caught before invoking `rustc` and reported with a `Kora` diagnostic code (`E0xxx`..`E4xxx`).
4. **Core Architectural Review**: Agents outside `compiler-architect` and `language-engineer` must not alter core AST, HIR, or Type System invariants without architectural compatibility verification.

