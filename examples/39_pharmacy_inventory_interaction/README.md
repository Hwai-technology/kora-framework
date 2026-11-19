# Example 39: Pharmacy Dispensing & Drug Interaction Checker

Automated drug-to-drug interaction matrix, contraindication checker, controlled substance batch tracking, and inventory replenishment.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/39_pharmacy_inventory_interaction/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Clinical domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/39_pharmacy_inventory_interaction/main.kora
kora test examples/39_pharmacy_inventory_interaction/tests/
```
