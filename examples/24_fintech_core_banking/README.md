# Example 24: Core Banking Ledger & Distributed Sagas

Enterprise-grade core banking engine implementing **Distributed Sagas, Optimistic Concurrency Control (OCC), Event Sourcing, Idempotent Processing, and Mathematical Money Conservation Invariants**.

---

## 1. Domain Engineering & Architectural Invariants

### A. Distributed Saga Pattern with Compensation
- Multi-step asset transfer orchestration across account aggregates:
  1. **Idempotency Gate**: Deduplication store prevents duplicate command replay.
  2. **Fund Reservation & OCC**: Locks and reserves funds on source account (`balance -= amount`, `reservedBalance += amount`, `version += 1`).
  3. **Target Account Credit**: Attempts credit on destination account.
  4. **Compensating Rollback**: If target account is frozen, missing, or rejected by compliance, triggers automated compensation: releases reserved balance back to source and records a `SAGA_COMPENSATION_REVERSAL` event.
  5. **Finalization**: Commits reservation and produces immutable ledger audit entry.

### B. Optimistic Concurrency Control (OCC)
- Accounts maintain monotonic `version: Int`.
- Race condition mitigation: Stale read updates are detected and rejected (`FAILED_OPTIMISTIC_LOCK_CONFLICT`), preventing lost updates under concurrent access.

### C. Mathematical Conservation of Money Invariant
- At any point in time, under all successful or rolled-back sagas:
  $$\sum (\text{balance}_i + \text{reservedBalance}_i) = \text{Total System Assets}$$
- Discrepancy is asserted to be strictly $0 in all automated test suites.

---

## 2. File Organization (Clean Architecture)

```
examples/24_fintech_core_banking/
├── models/
│   └── banking.dto.kora                      # Saga commands, DTOs & audit summaries
├── entities/
│   ├── account.entity.kora                   # Account aggregate with versioning
│   └── ledger_entry.entity.kora              # Event sourcing journal log
├── interfaces/
│   └── banking.repository.interface.kora     # Persistence and idempotency contract
├── services/
│   └── banking.service.kora                  # Saga orchestrator & OCC engine
├── controllers/
│   └── banking.controller.kora               # Command & audit dispatcher
├── routes/
│   └── banking.routes.kora                   # Endpoint registry
├── main.kora                                 # Application container bootstrap
└── tests/
    ├── banking_unit.test.kora                # 5/5 Rigorous property & invariant tests
    └── banking_load.test.kora                # 1,000 Concurrent Sagas stress suite
```

---

## 3. Verification Commands

```bash
kora check examples/24_fintech_core_banking/main.kora
kora test examples/24_fintech_core_banking/tests/banking_unit.test.kora
kora test examples/24_fintech_core_banking/tests/banking_load.test.kora
```
