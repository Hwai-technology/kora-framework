# Example 33: Treasury Liquidity & Capital Allocation

Dynamic reserve ratio enforcement, yield curve projections, and automated liquidity sweeps.

---

## 1. Clean Architecture File Layout (25 Files)

```
examples/33_treasury_liquidity_pool/
├── models/
│   ├── treasury_liquidity_pool_request.dto.kora
│   ├── treasury_liquidity_pool_response.dto.kora
│   ├── treasury_liquidity_pool_config.model.kora
│   └── treasury_liquidity_pool_event.model.kora
├── entities/
│   ├── treasury_liquidity_pool_primary.entity.kora
│   ├── treasury_liquidity_pool_secondary.entity.kora
│   ├── treasury_liquidity_pool_audit_log.entity.kora
│   └── treasury_liquidity_pool_snapshot.entity.kora
├── interfaces/
│   ├── treasury_liquidity_pool_repository.interface.kora
│   ├── treasury_liquidity_pool_service.interface.kora
│   └── treasury_liquidity_pool_notifier.interface.kora
├── repositories/
│   ├── treasury_liquidity_pool_primary.repository.kora
│   ├── treasury_liquidity_pool_audit.repository.kora
│   └── treasury_liquidity_pool_snapshot.repository.kora
├── services/
│   ├── treasury_liquidity_pool_core.service.kora
│   ├── treasury_liquidity_pool_validation.service.kora
│   ├── treasury_liquidity_pool_analytics.service.kora
│   └── treasury_liquidity_pool_notification.service.kora
├── controllers/
│   ├── treasury_liquidity_pool_command.controller.kora
│   ├── treasury_liquidity_pool_query.controller.kora
│   └── treasury_liquidity_pool_health.controller.kora
├── routes/
│   └── treasury_liquidity_pool.routes.kora
├── main.kora
└── tests/
    ├── treasury_liquidity_pool_unit.test.kora
    └── treasury_liquidity_pool_load.test.kora
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/33_treasury_liquidity_pool/main.kora
kora test examples/33_treasury_liquidity_pool/tests/
```
