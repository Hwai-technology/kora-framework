# Example 31: Inter-Bank Reconciliation & Settlement

End-of-day transaction matching, dispute resolution state machines, and clearing house reporting.

---

## 1. Clean Architecture File Layout (25 Files)

```
examples/31_reconciliation_settlement/
├── models/
│   ├── reconciliation_settlement_request.dto.kora
│   ├── reconciliation_settlement_response.dto.kora
│   ├── reconciliation_settlement_config.model.kora
│   └── reconciliation_settlement_event.model.kora
├── entities/
│   ├── reconciliation_settlement_primary.entity.kora
│   ├── reconciliation_settlement_secondary.entity.kora
│   ├── reconciliation_settlement_audit_log.entity.kora
│   └── reconciliation_settlement_snapshot.entity.kora
├── interfaces/
│   ├── reconciliation_settlement_repository.interface.kora
│   ├── reconciliation_settlement_service.interface.kora
│   └── reconciliation_settlement_notifier.interface.kora
├── repositories/
│   ├── reconciliation_settlement_primary.repository.kora
│   ├── reconciliation_settlement_audit.repository.kora
│   └── reconciliation_settlement_snapshot.repository.kora
├── services/
│   ├── reconciliation_settlement_core.service.kora
│   ├── reconciliation_settlement_validation.service.kora
│   ├── reconciliation_settlement_analytics.service.kora
│   └── reconciliation_settlement_notification.service.kora
├── controllers/
│   ├── reconciliation_settlement_command.controller.kora
│   ├── reconciliation_settlement_query.controller.kora
│   └── reconciliation_settlement_health.controller.kora
├── routes/
│   └── reconciliation_settlement.routes.kora
├── main.kora
└── tests/
    ├── reconciliation_settlement_unit.test.kora
    └── reconciliation_settlement_load.test.kora
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/31_reconciliation_settlement/main.kora
kora test examples/31_reconciliation_settlement/tests/
```
