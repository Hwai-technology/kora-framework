# Kora Official Examples & Architecture Catalog

Welcome to the official collection of Kora examples. Each example is housed in its own isolated directory to prevent collisions, demonstrate clean code patterns, and illustrate specific capabilities of the Kora language and framework.

---

## 📚 Examples Directory Index

| # | Example Directory | Topic / Paradigm | Key Features Demonstrated | Command |
|:---:|:---|:---|:---|:---|
| **01** | [`01_hello_world`](./01_hello_world/) | **Language Fundamentals** | Models, algebraic enums, pattern matching, functions, static typing | `kora run examples/01_hello_world/main.kora` |
| **02** | [`02_clean_architecture_users`](./02_clean_architecture_users/) | **Clean Architecture REST** | Model/service segregation, interface contracts, dependency injection | `kora run examples/02_clean_architecture_users/main.kora` |
| **03** | [`03_crud_products_api`](./03_crud_products_api/) | **Full CRUD & Dynamic SEO/Geo** | `GET`/`POST`/`PUT`/`DELETE`, OpenGraph/Twitter SEO, client Geo headers | `kora run examples/03_crud_products_api/main.kora` |
| **04** | [`04_mysql_database_orm`](./04_mysql_database_orm/) | **Relational Database & ORM** | MySQL pool, prepared statements, transactions, Docker Compose | `kora run examples/04_mysql_database_orm/main.kora` |
| **05** | [`05_jwt_auth_argon2`](./05_jwt_auth_argon2/) | **Enterprise Security** | Argon2id password hashing, HMAC-SHA256 JWT tokens, auth DTOs | `kora run examples/05_jwt_auth_argon2/main.kora` |
| **06** | [`06_realtime_websockets`](./06_realtime_websockets/) | **Realtime & WebSockets** | Pub/sub topic broadcasting, channel presence, realtime streaming | `kora run examples/06_realtime_websockets/main.kora` |
| **07** | [`07_async_actors_concurrency`](./07_async_actors_concurrency/) | **Actor Model Concurrency** | Isolated actor state, MPSC channels, non-blocking background workers | `kora run examples/07_async_actors_concurrency/main.kora` |
| **08** | [`08_openapi_swagger`](./08_openapi_swagger/) | **OpenAPI 3.1 & Swagger UI** | Zero-config spec discovery, interactive `/docs` UI, dynamic head meta | `kora run examples/08_openapi_swagger/main.kora` |
| **09** | [`09_enterprise_clean_architecture`](./09_enterprise_clean_architecture/) | **Full Enterprise Showcase** | 40+ source files, DDD modules, middleware, database layer | `cd examples/realworld_clean_architecture && kora run` |

---

## 🏛️ Strict Clean Architecture Standards

All examples in this repository strictly satisfy Kora's compiler-enforced architectural guard rules:
1. **Model-Class Segregation (`E5003`)**: Data entities defined via `model` and business classes defined via `class` are never co-located in the same file.
2. **Interface Purity (`E5002`)**: Files inside `interfaces/` and `models/` contain zero executable server logic or route handlers.
3. **Enum Purity (`E5001`)**: Files inside `enums/` contain pure variant definitions without methods or routes.
4. **Lean Orchestration (`E5004`)**: `main.kora` is reserved for dependency wiring and route declarations.

---

## 🧪 Verifying All Examples

You can verify the entire catalog in one command:
```bash
# Type check all example entrypoints
for dir in examples/0*; do
  if [ -f "$dir/main.kora" ]; then
    echo "Verifying $dir..."
    kora check "$dir/main.kora"
  fi
done

# Type check all 40 files in the enterprise clean architecture showcase
for f in $(find examples/realworld_clean_architecture/src -name "*.kora"); do
  kora check "$f"
done
```
