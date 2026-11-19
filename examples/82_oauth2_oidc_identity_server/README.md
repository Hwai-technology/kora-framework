# Example 82: OAuth 2.1 & OpenID Connect Authorization Server

Authorization Code flow with mandatory PKCE (RFC 7636), RS256 JWT minting, JWKS key rotation, and refresh token rotation.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/82_oauth2_oidc_identity_server/
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
kora check examples/82_oauth2_oidc_identity_server/main.kora
kora test examples/82_oauth2_oidc_identity_server/tests/
```
