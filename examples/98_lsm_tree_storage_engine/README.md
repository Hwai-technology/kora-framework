# Example 98: Log-Structured Merge-Tree (LSM-Tree) Engine

In-memory skip list MemTable, immutable on-disk SSTable block writer with Bloom filters, and multi-tier compaction merger.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/98_lsm_tree_storage_engine/
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
kora check examples/98_lsm_tree_storage_engine/main.kora
kora test examples/98_lsm_tree_storage_engine/tests/
```
