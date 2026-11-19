# Example 70: Industrial SCADA Modbus Gateway & Water Treatment

Modbus TCP register mapper, automated chlorination chemical dosing, tank overflow float protection, and safety override interlocks.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/70_water_treatment_scada_bridge/
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
kora check examples/70_water_treatment_scada_bridge/main.kora
kora test examples/70_water_treatment_scada_bridge/tests/
```
