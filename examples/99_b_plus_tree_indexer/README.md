# Example 99: B+ Tree Disk Pager & Index Node Balancer

Disk-page backed B+ Tree index, node split and merge balancing heuristics, and cursor range query iterators.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/99_b_plus_tree_indexer/
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
kora check examples/99_b_plus_tree_indexer/main.kora
kora test examples/99_b_plus_tree_indexer/tests/
```
