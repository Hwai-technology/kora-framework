# Example 83: Zero-Trust Context-Aware Policy Decision Point (PDP)

Attribute-Based Access Control (ABAC), device health posture assessment, risk-adaptive step-up MFA, and microsegmentation.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/83_zero_trust_policy_enforcer/
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
kora check examples/83_zero_trust_policy_enforcer/main.kora
kora test examples/83_zero_trust_policy_enforcer/tests/
```
