# Example 69: Smart Building HVAC & Air Quality Coordinator

Occupancy PIR sensor aggregation, CO2 parts-per-million monitoring, PID thermal loop controller, and peak energy shaving.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/69_smart_building_hvac_control/
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
kora check examples/69_smart_building_hvac_control/main.kora
kora test examples/69_smart_building_hvac_control/tests/
```
