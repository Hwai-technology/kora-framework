# Example 48: Distributed Cache Cluster & Write-Behind

LRU/LFU cache evictions, cache-aside data retrieval, write-behind batch coalescing, and hot-key thundering herd protection.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/48_distributed_cache_cluster/
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
kora check examples/48_distributed_cache_cluster/main.kora
kora test examples/48_distributed_cache_cluster/tests/
```
