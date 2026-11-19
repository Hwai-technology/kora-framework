# Example 49: API Gateway & Web Application Firewall

JWT cryptographic validation, IP threat reputation scoring, SQL injection sanitizer, and distributed token quota limiting.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/49_api_gateway_waf/
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
kora check examples/49_api_gateway_waf/main.kora
kora test examples/49_api_gateway_waf/tests/
```
