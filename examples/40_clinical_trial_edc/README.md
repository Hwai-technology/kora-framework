# Example 40: Clinical Trial Electronic Data Capture (EDC)

CRF electronic data capture, double-entry verification, adverse event reporting (GCP compliance), protocol deviation tracker.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/40_clinical_trial_edc/
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
kora check examples/40_clinical_trial_edc/main.kora
kora test examples/40_clinical_trial_edc/tests/
```
