# Example 37: ICU Real-Time High-Frequency Vitals Monitor

High-frequency patient vital signs stream (ECG, SpO2, blood pressure, respiration), anomaly detection actor, and emergency code alerts.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/37_icu_vitals_telemetry/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Clinical domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/37_icu_vitals_telemetry/main.kora
kora test examples/37_icu_vitals_telemetry/tests/
```
