# Kora Technical Roadmap

## Completed Milestones
- [x] Recursive-descent parser and lexer with error recovery.
- [x] Static type system with local inference and interface contract validation.
- [x] Removal of `struct` keyword in favor of pure `model` and `class` (`E2003`).
- [x] Single-null discipline: `null` literal support and complete rejection of `undefined` (`E2005`).
- [x] Visibility modifiers (`public`, `private`, `protected`, `readonly`) and constructor parameter promotion.
- [x] Strict `kebab-case` JSON API serialization.
- [x] Architectural purity compiler diagnostics (`E5001`-`E5004`).
- [x] Ambient database pool with live SQL query execution.
- [x] Production enterprise services: `JwtService`, `CryptoService`, `StorageService`, `ValidationService`, `Logger`, `SessionService`, `CacheService`, `EmailService`, `SmsService`.
- [x] Modern CI/CD automation, pull request auto-merge, contributor recognition, and AI code review agent.

## In Progress
- [ ] Language Server Protocol (LSP) auto-completion and hover documentation.
- [ ] Integrated package manager (`kora pkg`) and dependency resolution.
- [ ] Database schema reflection and auto-migration CLI command.

## Future Horizons
- [ ] Direct LLVM native backend codegen.
- [ ] WebAssembly compilation target for edge runtimes (Cloudflare Workers, Fastly).

