# Example 81: Synthetic Tabular Data & Differential Privacy Generator

Markovian correlation matrix estimation, differential privacy Laplacian noise injection, and synthetic test database populator.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/81_synthetic_data_generator/
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
kora check examples/81_synthetic_data_generator/main.kora
kora test examples/81_synthetic_data_generator/tests/
```
