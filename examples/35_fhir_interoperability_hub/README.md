# Example 35: FHIR R4 & HL7 Interoperability Hub

HL7 v2 message parsing, FHIR R4 JSON bundle validation, Observation, Patient, and DiagnosticReport resource transformations.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/35_fhir_interoperability_hub/
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
kora check examples/35_fhir_interoperability_hub/main.kora
kora test examples/35_fhir_interoperability_hub/tests/
```
