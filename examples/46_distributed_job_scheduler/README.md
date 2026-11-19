# Example 46: Distributed Task Scheduler & Leader Lease

Distributed cron scheduling, leader lease heartbeat locks, worker failure detection, and dead-letter retry queues.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/46_distributed_job_scheduler/
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
kora check examples/46_distributed_job_scheduler/main.kora
kora test examples/46_distributed_job_scheduler/tests/
```
