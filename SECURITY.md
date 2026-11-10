# Security Policy

## Security Philosophy

Kora enforces **secure-by-default** design rather than marketing claims of "automatic invulnerability":

1. **Memory & Type Safety**: Enforced at compile time via `kora-ty` (strong static typing) and `kora-ownership` (3-tier ownership inference lowered to verified Rust). `unsafe` Rust emission is strictly prohibited in user-code generation.
2. **Parameterized Queries Only**: `kora-orm` compiles queries into parameterized bindings (`SQLx`) and forbids raw string interpolation in SQL builders.
3. **Contextual Security Controls**: Authentication, RBAC, CORS, CSRF, Rate Limiting, and Output Encoding are explicitly typed and context-aware per transport (JSON API vs HTML response vs WebSocket channel).

## Reporting a Vulnerability

Please **do not** open public issues for suspected security vulnerabilities in the compiler, runtime, HTTP engine, or ORM.
Report security findings privately with:
- Affected crate and version (`crates/kora-*`)
- Minimal `.kora` reproduction program
- Expected vs actual behavior and impact assessment

