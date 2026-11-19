# Example 29: Algorithmic Market Maker & Quoting Engine

Avellaneda-Stoikov inventory skewing, automated bid/ask spread adjustment, and cancel-replace logic.

---

## 1. Clean Architecture File Layout (25 Files)

```
examples/29_algorithmic_market_maker/
├── models/
│   ├── algorithmic_market_maker_request.dto.kora
│   ├── algorithmic_market_maker_response.dto.kora
│   ├── algorithmic_market_maker_config.model.kora
│   └── algorithmic_market_maker_event.model.kora
├── entities/
│   ├── algorithmic_market_maker_primary.entity.kora
│   ├── algorithmic_market_maker_secondary.entity.kora
│   ├── algorithmic_market_maker_audit_log.entity.kora
│   └── algorithmic_market_maker_snapshot.entity.kora
├── interfaces/
│   ├── algorithmic_market_maker_repository.interface.kora
│   ├── algorithmic_market_maker_service.interface.kora
│   └── algorithmic_market_maker_notifier.interface.kora
├── repositories/
│   ├── algorithmic_market_maker_primary.repository.kora
│   ├── algorithmic_market_maker_audit.repository.kora
│   └── algorithmic_market_maker_snapshot.repository.kora
├── services/
│   ├── algorithmic_market_maker_core.service.kora
│   ├── algorithmic_market_maker_validation.service.kora
│   ├── algorithmic_market_maker_analytics.service.kora
│   └── algorithmic_market_maker_notification.service.kora
├── controllers/
│   ├── algorithmic_market_maker_command.controller.kora
│   ├── algorithmic_market_maker_query.controller.kora
│   └── algorithmic_market_maker_health.controller.kora
├── routes/
│   └── algorithmic_market_maker.routes.kora
├── main.kora
└── tests/
    ├── algorithmic_market_maker_unit.test.kora
    └── algorithmic_market_maker_load.test.kora
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/29_algorithmic_market_maker/main.kora
kora test examples/29_algorithmic_market_maker/tests/
```
