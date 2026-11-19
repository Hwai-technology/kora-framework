# Example 72: HNSW Vector Index & Approximate Nearest Neighbor

Hierarchical Navigable Small World (HNSW) multi-layer graph index, cosine distance metric kernel, and vector scalar quantization.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/72_vector_index_hnsw/
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
kora check examples/72_vector_index_hnsw/main.kora
kora test examples/72_vector_index_hnsw/tests/
```
