# Performance Standards

1. **Zero-Allocation Hot Paths Where Practical**: Use byte offsets (`Span { file_id, start, end }`) rather than copying source substrings on every AST token/node.
2. **Isolated & Reproducible Benchmarks**: HTTP routing, JSON serialization, and PostgreSQL ORM benchmarks must be measured independently against Rust+Axum, Go+Gin, Node.js+Fastify, and PHP+Laravel (`Req/s`, `P50`, `P95`, `P99`, RSS Memory, CPU, Startup, Binary Size).

