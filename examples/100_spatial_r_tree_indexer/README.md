# Example 100: Spatial 2D R-Tree & k-Nearest Neighbors (k-NN)

Hierarchical minimum bounding box (MBR) spatial indexing, quadratic node split algorithm, and k-NN nearest point search.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/100_spatial_r_tree_indexer/
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
kora check examples/100_spatial_r_tree_indexer/main.kora
kora test examples/100_spatial_r_tree_indexer/tests/
```
