# Example 43: Raft Distributed Consensus Engine

Leader election, term increments, log replication state machine, quorum heartbeats, and cluster snapshotting.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/43_raft_distributed_consensus/
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
kora check examples/43_raft_distributed_consensus/main.kora
kora test examples/43_raft_distributed_consensus/tests/
```
