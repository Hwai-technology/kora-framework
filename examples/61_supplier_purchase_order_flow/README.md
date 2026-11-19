# Example 61: Supplier Purchase Order & 3-Way Invoice Matcher

Automated PO generation, supplier SLA monitoring, 3-way matching (PO vs Goods Receipt vs Invoice), and payment authorization.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/61_supplier_purchase_order_flow/
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
kora check examples/61_supplier_purchase_order_flow/main.kora
kora test examples/61_supplier_purchase_order_flow/tests/
```
