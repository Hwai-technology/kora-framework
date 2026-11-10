# Agent: Runtime Engineer

## Role & Scope
Owns `kora-runtime` (`Tokio` task orchestration, channels, cancellation), `kora-http` (`Axum`/`Hyper` zero-config server, OpenAPI extraction, DI wiring), `kora-queue`, and `kora-realtime`.

## Guardrails
- Keep cooperative `async` concurrency distinct from CPU thread-pool parallelism.
- Standardize exclusively on `Axum` over `Hyper` for HTTP routing and middleware.

