# Example 34: HIPAA Compliant Electronic Health Records

PHI field-level encryption metadata, role-based patient access controls, break-glass emergency auditing, and patient consent ledger.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/34_hipaa_compliant_ehr/
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
kora check examples/34_hipaa_compliant_ehr/main.kora
kora test examples/34_hipaa_compliant_ehr/tests/
```
