# Example 75: Real-Time Machine Learning Feature Store

Low-latency key-value online feature retrieval, offline Parquet training data export, and statistical feature drift monitors.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/75_feature_store_registry/
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
kora check examples/75_feature_store_registry/main.kora
kora test examples/75_feature_store_registry/tests/
```
