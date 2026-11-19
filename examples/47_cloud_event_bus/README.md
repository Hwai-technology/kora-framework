# Example 47: Cloud Event Bus & Partitioned Streaming

Partition-ordered event streaming, consumer group offset management, dead-letter DLQ, and schema registry enforcement.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/47_cloud_event_bus/
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
kora check examples/47_cloud_event_bus/main.kora
kora test examples/47_cloud_event_bus/tests/
```
