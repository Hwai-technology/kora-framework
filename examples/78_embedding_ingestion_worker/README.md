# Example 78: High-Throughput Vector Embedding Ingestion Pipeline

Async sliding-window text chunker actor, token-bucket rate-limited embedding API client, and vector upsert batcher.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/78_embedding_ingestion_worker/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/78_embedding_ingestion_worker/main.kora
kora test examples/78_embedding_ingestion_worker/tests/
```
