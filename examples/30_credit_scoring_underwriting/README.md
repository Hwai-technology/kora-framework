# Example 30: Credit Underwriting & Risk Assessment

Debt-to-income ratio calculations, credit bureau payload normalization, and risk tier assignments.

---

## 1. Clean Architecture File Layout (25 Files)

```
examples/30_credit_scoring_underwriting/
├── models/
│   ├── credit_scoring_underwriting_request.dto.kora
│   ├── credit_scoring_underwriting_response.dto.kora
│   ├── credit_scoring_underwriting_config.model.kora
│   └── credit_scoring_underwriting_event.model.kora
├── entities/
│   ├── credit_scoring_underwriting_primary.entity.kora
│   ├── credit_scoring_underwriting_secondary.entity.kora
│   ├── credit_scoring_underwriting_audit_log.entity.kora
│   └── credit_scoring_underwriting_snapshot.entity.kora
├── interfaces/
│   ├── credit_scoring_underwriting_repository.interface.kora
│   ├── credit_scoring_underwriting_service.interface.kora
│   └── credit_scoring_underwriting_notifier.interface.kora
├── repositories/
│   ├── credit_scoring_underwriting_primary.repository.kora
│   ├── credit_scoring_underwriting_audit.repository.kora
│   └── credit_scoring_underwriting_snapshot.repository.kora
├── services/
│   ├── credit_scoring_underwriting_core.service.kora
│   ├── credit_scoring_underwriting_validation.service.kora
│   ├── credit_scoring_underwriting_analytics.service.kora
│   └── credit_scoring_underwriting_notification.service.kora
├── controllers/
│   ├── credit_scoring_underwriting_command.controller.kora
│   ├── credit_scoring_underwriting_query.controller.kora
│   └── credit_scoring_underwriting_health.controller.kora
├── routes/
│   └── credit_scoring_underwriting.routes.kora
├── main.kora
└── tests/
    ├── credit_scoring_underwriting_unit.test.kora
    └── credit_scoring_underwriting_load.test.kora
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/30_credit_scoring_underwriting/main.kora
kora test examples/30_credit_scoring_underwriting/tests/
```
