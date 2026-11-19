# Example 95: Multiplayer Game 60Hz Tick State Synchronizer

Server authoritative tick loop (60 FPS), client-side prediction reconciler, snapshot interpolation, and lag compensation rollback.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/95_gaming_state_synchronizer/
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
kora check examples/95_gaming_state_synchronizer/main.kora
kora test examples/95_gaming_state_synchronizer/tests/
```
