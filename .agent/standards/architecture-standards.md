# Architecture Standards

1. **Hybrid Object Model (Struct-Free Paradigm)**:
   - `model` represents pure declarative data schemas (Entities & DTOs) with schema attributes (`@entity`, `@table`, `@required`, `@email`, `@min`, `@default`).
   - `class` represents stateful service units eligible for compile-time Dependency Injection via `init(...)`.
   - `interface` defines behavioral polymorphism lowered to Rust traits.
2. **Single HTTP Stack & Dedicated Port**:
   - Standardize on `Axum` + `Tower` + `Hyper` atop `Tokio`.
   - Default server port is dedicated to **`7582`** (avoiding common collisions with 8080/3000/4200), dynamically overridable via `PORT` or `KORA_PORT`.
   - Native runners automatically expose root `/` and `/health` endpoints.
3. **Compile-Time Wiring & Zero-Config OpenAPI**:
   - Prefer compile-time symbol tables, DI graphs, route metadata extraction, and automatic OpenAPI 3.1 & Swagger UI schema synthesis (`kora swagger`) over runtime reflection.
4. **Mandatory Production Scaffolding**:
   - Projects scaffolded via `kora init` or `kora create` must generate complete production suites (`.editorconfig`, `Dockerfile`, `docker-compose.yml`, `.env.example`, `.env`, and Clean Architecture feature directories).
5. **Compiler-Grade Error Diagnostics**:
   - All developer error states (missing files `E1001`, directory paths `E1002`, read errors `E1003`, unresolved imports `E1004`, missing toolchains `E1005`) must yield actionable copy-paste solutions.


