# Example 57: Marketplace Vendor Settlement & Escrow Payouts

Multi-vendor order splitting, commission deduction, withholding tax calculations, and dispute escrow hold.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/57_marketplace_vendor_settlement/
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
kora check examples/57_marketplace_vendor_settlement/main.kora
kora test examples/57_marketplace_vendor_settlement/tests/
```
