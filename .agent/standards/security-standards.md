# Security Standards

1. **No `unsafe` Code Generation**: Generated Rust code from Kora user sources must compile with `#![forbid(unsafe_code)]`.
2. **SQL Injection Prevention**: `kora-orm` must bind parameters exclusively via typed `SQLx` prepared statements.
3. **Context-Aware Web Protections**: Security middleware (`CORS`, `CSRF`, `RateLimit`, `JWT`, `RBAC`, `SecureHeaders`) must apply explicit, inspectable policies rather than opaque global side effects.

