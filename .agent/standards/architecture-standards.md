# Architecture Standards

1. **Hybrid Object Model**:
   - `struct` and `model` represent data layouts without hidden inheritance hierarchies.
   - `class` represents stateful or stateless service units eligible for compile-time Dependency Injection via `init(...)`.
   - `interface` defines behavioral polymorphism lowered to Rust traits.
2. **Single HTTP Stack**: Standardize on `Axum` + `Tower` + `Hyper` atop `Tokio` in Phase 3+. Never maintain competing custom HTTP parsers.
3. **Compile-Time Wiring**: Prefer compile-time symbol tables, DI graphs, route metadata extraction, and OpenAPI schema generation over runtime reflection.

