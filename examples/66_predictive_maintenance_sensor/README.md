# Example 66: Predictive Industrial Equipment Maintenance

Rotational vibration FFT frequency analysis, bearing fatigue degradation curves, and automated maintenance ticket scheduling.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/66_predictive_maintenance_sensor/
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
kora check examples/66_predictive_maintenance_sensor/main.kora
kora test examples/66_predictive_maintenance_sensor/tests/
```
