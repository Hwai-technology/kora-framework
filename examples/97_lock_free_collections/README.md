# Example 97: Lock-Free Michael-Scott Queue & Treiber Stack

Lock-free queue using Compare-And-Swap (CAS), Treiber concurrent stack, and ABA hazard pointer memory reclamation simulation.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/97_lock_free_collections/
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
kora check examples/97_lock_free_collections/main.kora
kora test examples/97_lock_free_collections/tests/
```
