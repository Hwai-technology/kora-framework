# Example 42: Mental Health Longitudinal Assessment Engine

Standardized PHQ-9 (depression) and GAD-7 (anxiety) questionnaire scoring, crisis intervention trigger, and longitudinal trend analytics.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/42_mental_health_assessment/
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
kora check examples/42_mental_health_assessment/main.kora
kora test examples/42_mental_health_assessment/tests/
```
