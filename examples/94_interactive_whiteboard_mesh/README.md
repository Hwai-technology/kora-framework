# Example 94: Interactive Vector Whiteboard & Delta Sync

Bézier curve vector stroke serialization, undo/redo delta stack history, binary WebSocket packing, and laser pointer sharing.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/94_interactive_whiteboard_mesh/
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
kora check examples/94_interactive_whiteboard_mesh/main.kora
kora test examples/94_interactive_whiteboard_mesh/tests/
```
