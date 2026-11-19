# Example 26: Payment Gateway & Webhook Orchestrator

Production-grade Payment Orchestrator implementing **Multi-Acquirer Smart Routing (Stripe, Adyen, PayPal), PSD2 3D Secure 2.0 Challenge Authentication, Idempotency Gate (Double-Charge Defense), and HMAC-SHA256 Webhook Verification**.

---

## 1. Domain Engineering & Architectural Invariants

### A. Multi-Acquirer Smart Routing Heuristic
- **Adyen**: Automatically routed for high-ticket corporate transactions ($> \$10,000$) to minimize percentage interchange fees.
- **Stripe**: Domestic European cards (`EUR`, `FR`, `DE`, `ES`) routed for direct SEPA/card local acquiring.
- **PayPal**: Cross-border and digital wallet checkout default.

### B. PSD2 Strong Customer Authentication (3D Secure 2.0)
- Invariant: European domestic transactions over €50 (`amountCents > 5000`) trigger `CHALLENGE_REQUIRED`.
- Biometric or OTP challenge verification (`SCA-PASS-99`) is mandated prior to advancing state from `PENDING_3DS_CHALLENGE` to `AUTHORIZED`.

### C. Idempotency Gate (Double Charge Defense)
- Commands are signed with unique `idempotencyKey`. Replayed commands or network timeouts return the original authorization record with 0 duplicate charge side-effects.

### D. HMAC-SHA256 Webhook Defense
- Inbound acquirer webhooks (`PAYMENT_CAPTURED`, `PAYMENT_REFUNDED`) mandate valid cryptographic signature prefix (`sha256=...`).
- Tampered payloads are rejected with `ERR_INVALID_WEBHOOK_SIGNATURE`.

---

## 2. Infrastructure Setup (Docker Compose)

```bash
# Start PostgreSQL 16 & Redis
docker compose up -d

# Verify PostgreSQL connection
docker compose exec postgres pg_isready -U kora_pay_user -d payment_gateway_db
```

---

## 3. Clean Architecture Hierarchy

```
26_payment_gateway_orchestrator/
├── .env.example                                       # Provider keys & 3DS threshold configuration
├── docker-compose.yml                                 # PostgreSQL 16 & Redis 7 containers
├── enums/
│   ├── payment_provider.enum.kora                     # Stripe, Adyen, PayPal
│   ├── payment_status.enum.kora                       # Authorized, Captured, Refunded, ChallengeRequired
│   └── channel_type.enum.kora                         # Web, MobileSDK, POS
├── events/
│   ├── payment_authorized.event.kora                  # Authorization event
│   ├── payment_captured.event.kora                    # Settlement capture event
│   └── refund_processed.event.kora                    # Reversal refund event
├── models/
│   ├── payment_gateway_orchestrator_request.dto.kora  # Commands & Webhook DTOs
│   └── payment_gateway_orchestrator_response.dto.kora # API responses
├── entities/
│   └── payment_gateway_orchestrator_primary.entity.kora # Payment aggregate entity
├── interfaces/
│   ├── payment_gateway_orchestrator_repository.interface.kora
│   └── payment_gateway_orchestrator_service.interface.kora
├── repositories/
│   └── payment_gateway_orchestrator_primary.repository.kora # Vector-backed ledger store
├── services/
│   ├── payment_gateway_orchestrator_core.service.kora       # Routing, 3DS & Webhook engine
│   └── payment_gateway_orchestrator_validation.service.kora # Input BIN & idempotency validator
├── controllers/
│   ├── payment_gateway_orchestrator_command.controller.kora
│   └── payment_gateway_orchestrator_query.controller.kora
├── routes/
│   └── payment_gateway_orchestrator.routes.kora
├── main.kora                                          # Application bootstrap
└── tests/
    ├── payment_gateway_orchestrator_unit.test.kora    # 5/5 Smart routing & HMAC unit tests
    └── payment_gateway_orchestrator_load.test.kora    # 300-Transaction load stress test
```

---

## 4. Running & Validating

```bash
# Type-check the entire project
kora check main.kora

# Execute automated test suite
kora test tests/
```
