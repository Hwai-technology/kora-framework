# Example 51: Distributed Lock Manager & Fencing Tokens

Redlock distributed quorum algorithm, monotonic fencing tokens preventing race conditions, and automatic heartbeat lease extensions.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/51_distributed_lock_manager/
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
kora check examples/51_distributed_lock_manager/main.kora
kora test examples/51_distributed_lock_manager/tests/
```
