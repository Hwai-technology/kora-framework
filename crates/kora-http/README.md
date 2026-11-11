# kora-http

> **Kora High-Performance HTTP Engine**

Zero-overhead asynchronous HTTP web engine built on Axum and Hyper with automatic request routing, JSON serialization, middleware chaining, and OpenAPI 3.1 generation on port 7582.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Web Engine — Powers get, post, put, delete routing and server pipelines.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `KoraApp`
- `RouteInfo`
- `HttpServer`
- `RouteHandler`
- `Request`
- `Validator`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Built-in request body size limits, rate limiting, and header sanitization to protect against HTTP flood attacks.

---

## 🚀 How to Contribute & Improve

Add native HTTP/3 (QUIC) protocol support and automated OpenAPI / Swagger 3.1 JSON schema generator.

### Testing this Crate
```bash
cargo test -p kora-http
cargo clippy -p kora-http --all-targets -- -D warnings
```
