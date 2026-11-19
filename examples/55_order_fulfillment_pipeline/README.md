# Example 55: Order Fulfillment & Pick-Pack-Ship Workflow

WMS state machine, barcode scanner verification, shipping label broker, and tracking webhooks.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/55_order_fulfillment_pipeline/
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
kora check examples/55_order_fulfillment_pipeline/main.kora
kora test examples/55_order_fulfillment_pipeline/tests/
```
