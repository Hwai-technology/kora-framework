# Example 12: Distributed Event-Driven Microservice & Transactional Outbox

Production-grade event-driven architecture implementing the **Transactional Outbox Pattern, At-Least-Once Delivery, Idempotency Deduplication, Exponential Backoff Retries, and Dead Letter Queue (DLQ)**.

---

## 1. Domain Engineering & Architectural Invariants

### A. Transactional Outbox Pattern
- When state transitions occur within an aggregate, events are not directly pushed over the network (which risks two-phase commit inconsistencies). Instead, an immutable `EventEnvelope` is committed atomically into the Outbox repository (`status = PendingOutbox`).
- Background worker processes forward pending events to Apache Kafka / RabbitMQ.

### B. Idempotency Gate (Replay Defense)
- Every event is indexed by a globally unique `idempotencyKey`.
- Replayed commands or network duplicate deliveries are suppressed immediately with `ERR_DUPLICATE_IDEMPOTENCY_KEY`, preventing double fulfillment or state corruption.

### C. Exponential Backoff & Dead Letter Queue (DLQ)
- For transient failures, delivery retries are scheduled with exponential backoff:
  - Attempt 1: 100ms
  - Attempt 2: 200ms
  - Attempt 3: 400ms
- If a poisonous payload continues to fail after 3 attempts, it is automatically routed to the **Dead Letter Queue (`DLQ`)**, alerting observability and avoiding message head-of-line blocking.

---

## 2. Infrastructure Setup (Docker Compose)

```bash
# Start Apache Kafka (KRaft), PostgreSQL 16 & Redis
docker compose up -d

# Verify Kafka cluster status
docker compose exec kafka kafka-topics --bootstrap-server localhost:9092 --list
```

---

## 3. Clean Architecture Hierarchy

```
12_microservice_event_driven/
├── .env.example                               # Broker & retry configuration
├── docker-compose.yml                         # Kafka, PostgreSQL & Redis containers
├── enums/
│   └── event_status.enum.kora                 # PendingOutbox, Published, Delivered, Retrying, DeadLettered
├── events/
│   └── order_event.event.kora                 # OrderCreated, OrderPaymentCaptured, OrderDispatched
├── models/
│   └── event_envelope.model.kora              # EventEnvelope, ConsumerAck, OutboxTelemetry
├── interfaces/
│   └── event_store.repository.interface.kora  # Outbox & DLQ persistence contract
├── repositories/
│   └── event_store.repository.kora           # Idempotency store, outbox queue & DLQ
├── services/
│   └── event_publisher.service.kora          # Sequencing, dispatch & exponential retry engine
├── controllers/
│   └── order_events.controller.kora          # HTTP command publishing & consumer simulation
├── routes/
│   └── order_events.routes.kora              # /events/orders/publish, /events/orders/consume
├── main.kora                                  # Application bootstrap
└── tests/
    ├── microservice.test.kora                 # End-to-end integration lifecycle test
    ├── microservice_unit.test.kora            # 4/4 Unit tests (outbox, idempotency, backoff, DLQ)
    └── microservice_load.test.kora            # 200-Event fault-tolerance & DLQ load stress test
```

---

## 4. Running & Validating

```bash
# Type-check the entire project
kora check main.kora

# Execute automated test suite
kora test tests/
```
