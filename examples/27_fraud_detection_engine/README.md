# Example 27: Real-time Multi-Factor Fraud & Anomaly Evaluator

Production-grade Fraud Risk Scoring Engine implementing **Sliding-Window Velocity Bursts, Geolocation Impossible Travel Detection, Global Sanctions Enforcement (OFAC), and Multi-Factor Composite Risk Tiers**.

---

## 1. Domain Engineering & Architectural Invariants

### A. Geolocation Impossible Physical Travel
- Calculates velocity speed in km/h between sequential transactions on the same payment instrument:
  $$\text{Speed} = \frac{\Delta \text{Distance (km)}}{\Delta \text{Time (hours)}}$$
- If physical travel speed exceeds 800 km/h (e.g. London card used in New York 30 minutes later), an immediate $+50$ risk points is added, flagging an impossible physical transition.

### B. High-Frequency Sliding-Window Velocity
- Maintains sliding-window transaction counters per card/device token.
- If more than 3 transactions occur within 10 minutes, triggers a `HIGH_VELOCITY_BURST` penalty ($+35$ risk points).

### C. Merchant Category Code (MCC) Risk Weighting
- High-risk categories like Gambling (`MCC 7995`) and Crypto Exchanges (`MCC 6051`) carry an additional $+25$ baseline risk weight.

### D. Global Sanctions List Short-Circuiting (OFAC Defense)
- Transactions originating from or routed to sanctioned jurisdictions (`KP`, `IR`) immediately bypass scoring and trigger an irrevocable critical decline (`Score = 100`, `Outcome = DECLINED_SANCTIONS`).

### E. Risk Calibration & Decision Tiers
- **$\text{Score} < 30$**: `APPROVED`
- **$30 \le \text{Score} < 70$**: `STEP_UP_CHALLENGE` (Triggers SMS OTP, biometric, or 3DS verification)
- **$\text{Score} \ge 70$**: `DECLINED_FRAUD` (Blocks transaction and creates audit trail)

---

## 2. Infrastructure Setup (Docker Compose)

```bash
# Start Redis 7 & PostgreSQL 16
docker compose up -d

# Verify Redis connection
docker compose exec redis redis-cli -a kora_fraud_redis_pass ping
```

---

## 3. Clean Architecture Hierarchy

```
27_fraud_detection_engine/
├── .env.example                                       # Risk scoring thresholds & velocity windows
├── docker-compose.yml                                 # Redis & PostgreSQL containers
├── enums/
│   ├── risk_tier.enum.kora                            # Low, Medium, High, Critical
│   ├── decision_outcome.enum.kora                     # Approved, StepUpChallenge, DeclinedFraud, DeclinedSanctions
│   └── fraud_rule_type.enum.kora                      # Velocity, ImpossibleTravel, HighRiskMcc, Sanctions
├── events/
│   ├── anomaly_detected.event.kora                    # Anomaly trigger event
│   ├── rule_threshold_exceeded.event.kora             # Threshold exceeded event
│   └── compliance_alert_escalated.event.kora          # Escalation alert event
├── models/
│   ├── fraud_detection_engine_request.dto.kora        # Inbound transaction telemetry DTOs
│   └── fraud_detection_engine_response.dto.kora       # Risk score breakdown & outcome
├── entities/
│   └── fraud_detection_engine_primary.entity.kora     # Assessment record aggregate
├── interfaces/
│   ├── fraud_detection_engine_repository.interface.kora
│   └── fraud_detection_engine_service.interface.kora
├── repositories/
│   └── fraud_detection_engine_primary.repository.kora # Velocity & assessment ledger
├── services/
│   └── fraud_detection_engine_core.service.kora       # Multi-factor risk evaluation engine
├── controllers/
│   ├── fraud_detection_engine_command.controller.kora
│   └── fraud_detection_engine_query.controller.kora
├── routes/
│   └── fraud_detection_engine.routes.kora
├── main.kora                                          # Application bootstrap
└── tests/
    ├── fraud_detection_engine_unit.test.kora          # 4/4 Multi-factor risk scoring unit tests
    └── fraud_detection_engine_load.test.kora          # 500-Transaction anomaly interception load test
```

---

## 4. Running & Validating

```bash
# Type-check the entire project
kora check main.kora

# Execute automated test suite
kora test tests/
```
