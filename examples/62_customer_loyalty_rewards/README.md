# Example 62: Customer Loyalty Rewards Ledger & Tier Upgrades

Reward points ledger, automated tier upgrade/downgrade calculations, promotional voucher redemption, and fraud abuse limits.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/62_customer_loyalty_rewards/
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
kora check examples/62_customer_loyalty_rewards/main.kora
kora test examples/62_customer_loyalty_rewards/tests/
```
