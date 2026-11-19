# Example 56: Last-Mile Delivery Dispatch & Fleet Geo-Routing

Vehicle Routing Problem (VRP) heuristics, courier real-time GPS tracking, and dynamic ETA estimation.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/56_last_mile_delivery_dispatch/
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
kora check examples/56_last_mile_delivery_dispatch/main.kora
kora test examples/56_last_mile_delivery_dispatch/tests/
```
