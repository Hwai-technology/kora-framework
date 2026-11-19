# Example 85: Secret Management Vault & Dynamic Credentials

AES-256-GCM envelope encryption, dynamic ephemeral database credential generation with TTL, and automated credential rotation.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/85_secret_management_vault/
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
kora check examples/85_secret_management_vault/main.kora
kora test examples/85_secret_management_vault/tests/
```
