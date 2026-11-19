# Example 64: Smart Electrical Grid Substation Telemetry

High-voltage substation power factor telemetry, dynamic load shedding coordinator, and solar feed-in grid stabilization.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/64_smart_grid_energy_monitor/
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
kora check examples/64_smart_grid_energy_monitor/main.kora
kora test examples/64_smart_grid_energy_monitor/tests/
```
