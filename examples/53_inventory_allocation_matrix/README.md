# Example 53: Inventory Allocation Matrix & Multi-Warehouse Stock

Multi-tier warehouse inventory reservation, safety stock replenishment triggers, and backorder queuing.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/53_inventory_allocation_matrix/
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
kora check examples/53_inventory_allocation_matrix/main.kora
kora test examples/53_inventory_allocation_matrix/tests/
```
