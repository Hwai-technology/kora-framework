# Example 28: Crypto Derivatives & Margin Engine

Perpetual futures settlement, funding rate calculations, and automated margin call liquidation.

---

## 1. Clean Architecture File Layout (25 Files)

```
examples/28_crypto_derivatives_exchange/
├── models/
│   ├── crypto_derivatives_exchange_request.dto.kora
│   ├── crypto_derivatives_exchange_response.dto.kora
│   ├── crypto_derivatives_exchange_config.model.kora
│   └── crypto_derivatives_exchange_event.model.kora
├── entities/
│   ├── crypto_derivatives_exchange_primary.entity.kora
│   ├── crypto_derivatives_exchange_secondary.entity.kora
│   ├── crypto_derivatives_exchange_audit_log.entity.kora
│   └── crypto_derivatives_exchange_snapshot.entity.kora
├── interfaces/
│   ├── crypto_derivatives_exchange_repository.interface.kora
│   ├── crypto_derivatives_exchange_service.interface.kora
│   └── crypto_derivatives_exchange_notifier.interface.kora
├── repositories/
│   ├── crypto_derivatives_exchange_primary.repository.kora
│   ├── crypto_derivatives_exchange_audit.repository.kora
│   └── crypto_derivatives_exchange_snapshot.repository.kora
├── services/
│   ├── crypto_derivatives_exchange_core.service.kora
│   ├── crypto_derivatives_exchange_validation.service.kora
│   ├── crypto_derivatives_exchange_analytics.service.kora
│   └── crypto_derivatives_exchange_notification.service.kora
├── controllers/
│   ├── crypto_derivatives_exchange_command.controller.kora
│   ├── crypto_derivatives_exchange_query.controller.kora
│   └── crypto_derivatives_exchange_health.controller.kora
├── routes/
│   └── crypto_derivatives_exchange.routes.kora
├── main.kora
└── tests/
    ├── crypto_derivatives_exchange_unit.test.kora
    └── crypto_derivatives_exchange_load.test.kora
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/28_crypto_derivatives_exchange/main.kora
kora test examples/28_crypto_derivatives_exchange/tests/
```
