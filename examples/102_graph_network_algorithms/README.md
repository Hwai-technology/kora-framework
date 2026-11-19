# Example 102: Graph Algorithms & Tarjan SCC / PageRank

Adjacency-list directed graph, Tarjan's Strongly Connected Components (SCC) algorithm, and PageRank power iteration convergence.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/102_graph_network_algorithms/
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
kora check examples/102_graph_network_algorithms/main.kora
kora test examples/102_graph_network_algorithms/tests/
```
