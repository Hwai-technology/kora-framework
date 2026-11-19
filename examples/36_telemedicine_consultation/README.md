# Example 36: Telemedicine Virtual Clinic & E-Prescriptions

Secure WebRTC consultation sessions, electronic prescription digital signatures, clinical encounter notes, and billing integration.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/36_telemedicine_consultation/
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
kora check examples/36_telemedicine_consultation/main.kora
kora test examples/36_telemedicine_consultation/tests/
```
