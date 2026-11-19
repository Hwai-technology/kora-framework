# Example 89: Immutable Cryptographic Audit Trail (SOC 2 / ISO 27001)

Merkle hash chain linking audit logs, WORM (Write Once Read Many) storage simulator, tamper alarms, and compliance export.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/89_compliance_audit_ledger/
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
kora check examples/89_compliance_audit_ledger/main.kora
kora test examples/89_compliance_audit_ledger/tests/
```
