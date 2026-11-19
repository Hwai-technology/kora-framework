# Example 45: Service Mesh Sidecar Proxy & Traffic Director

mTLS identity verification, Envoy-style dynamic routing, exponential backoff retries, and token-bucket rate limiting.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/45_service_mesh_proxy/
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
kora check examples/45_service_mesh_proxy/main.kora
kora test examples/45_service_mesh_proxy/tests/
```
