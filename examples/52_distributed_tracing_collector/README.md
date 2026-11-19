# Example 52: Distributed Tracing & OpenTelemetry Collector

W3C trace context propagation, OpenTelemetry span ingestion, trace tree dependency reconstruction, and adaptive sampling controller.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/52_distributed_tracing_collector/
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
kora check examples/52_distributed_tracing_collector/main.kora
kora test examples/52_distributed_tracing_collector/tests/
```
