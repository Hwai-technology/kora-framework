# Testing Standards

1. **Multi-Layer Compiler Tests**:
   - Unit tests inside each crate (`kora-lexer`, `kora-parser`, `kora-hir`, `kora-ty`) testing both valid programs and malformed programs (verifying exact diagnostic error codes and spans).
   - End-to-end integration tests verifying that canonical `.kora` programs in `examples/` pass the full pipeline cleanly.
2. **Deterministic Diagnostics**: Diagnostic output tests must assert stable error codes (`E1xxx`–`E4xxx`) and line/column locations.

