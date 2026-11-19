# Example 87: FIDO2 / WebAuthn Passkey Registration & Attestation

WebAuthn challenge-response cryptographic verification, authenticator attestation parsing, public key credential storage.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/87_hardware_security_token_fido2/
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
kora check examples/87_hardware_security_token_fido2/main.kora
kora test examples/87_hardware_security_token_fido2/tests/
```
