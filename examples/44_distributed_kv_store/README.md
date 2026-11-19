# Example 44: Distributed Dynamo-Style Key-Value Store

Consistent hashing ring with virtual nodes, vector clocks conflict resolution, hinted handoff, and read-repair quorum (R+W>N).

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/44_distributed_kv_store/
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
kora check examples/44_distributed_kv_store/main.kora
kora test examples/44_distributed_kv_store/tests/
```
