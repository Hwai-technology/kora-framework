# Example 59: Reverse Logistics & RMA Inspection Engine

Return Merchandise Authorization (RMA) tracking, return shipping label generation, inspection disposition, and restocking.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/59_returns_rma_management/
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
kora check examples/59_returns_rma_management/main.kora
kora test examples/59_returns_rma_management/tests/
```
