# Example 92: Multi-Room WebSocket Chat & Presence Cluster

Distributed channel subscriptions, typing indicator ephemeral broadcasts, message delivery ACK tracking, and presence heartbeats.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/92_chat_multi_room_cluster/
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
kora check examples/92_chat_multi_room_cluster/main.kora
kora test examples/92_chat_multi_room_cluster/tests/
```
