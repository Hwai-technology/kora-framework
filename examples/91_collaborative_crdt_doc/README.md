# Example 91: CRDT Collaborative Real-Time Document Editor

Conflict-free Replicated Data Type (YATA / Logoot sequence CRDT), multi-cursor position broadcast, and delta synchronization.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/91_collaborative_crdt_doc/
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
kora check examples/91_collaborative_crdt_doc/main.kora
kora test examples/91_collaborative_crdt_doc/tests/
```
