# Example 32: Anti-Money Laundering (AML) SIEM Pipeline

Sanctions screening fuzzy lookup, structuring detection, and regulatory compliance reporting.

---

## 1. Clean Architecture File Layout (25 Files)

```
examples/32_anti_money_laundering_siem/
├── models/
│   ├── anti_money_laundering_siem_request.dto.kora
│   ├── anti_money_laundering_siem_response.dto.kora
│   ├── anti_money_laundering_siem_config.model.kora
│   └── anti_money_laundering_siem_event.model.kora
├── entities/
│   ├── anti_money_laundering_siem_primary.entity.kora
│   ├── anti_money_laundering_siem_secondary.entity.kora
│   ├── anti_money_laundering_siem_audit_log.entity.kora
│   └── anti_money_laundering_siem_snapshot.entity.kora
├── interfaces/
│   ├── anti_money_laundering_siem_repository.interface.kora
│   ├── anti_money_laundering_siem_service.interface.kora
│   └── anti_money_laundering_siem_notifier.interface.kora
├── repositories/
│   ├── anti_money_laundering_siem_primary.repository.kora
│   ├── anti_money_laundering_siem_audit.repository.kora
│   └── anti_money_laundering_siem_snapshot.repository.kora
├── services/
│   ├── anti_money_laundering_siem_core.service.kora
│   ├── anti_money_laundering_siem_validation.service.kora
│   ├── anti_money_laundering_siem_analytics.service.kora
│   └── anti_money_laundering_siem_notification.service.kora
├── controllers/
│   ├── anti_money_laundering_siem_command.controller.kora
│   ├── anti_money_laundering_siem_query.controller.kora
│   └── anti_money_laundering_siem_health.controller.kora
├── routes/
│   └── anti_money_laundering_siem.routes.kora
├── main.kora
└── tests/
    ├── anti_money_laundering_siem_unit.test.kora
    └── anti_money_laundering_siem_load.test.kora
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/32_anti_money_laundering_siem/main.kora
kora test examples/32_anti_money_laundering_siem/tests/
```
