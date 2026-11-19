# Example 54: Dynamic Pricing Engine & Flash Sale Rate-Limiter

Real-time elasticity pricing, bundle discount evaluator, and flash sale checkout concurrency controls.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/54_dynamic_pricing_promotions/
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
kora check examples/54_dynamic_pricing_promotions/main.kora
kora test examples/54_dynamic_pricing_promotions/tests/
```
