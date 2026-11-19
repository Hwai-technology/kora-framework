# Example 25: High-Frequency Trading Order Book Matching Engine

High-performance Continuous Double Auction Matching Engine implementing **Price-Time Priority (FIFO), Partial Fills, Order Cancellation, Volatility Circuit Breakers, and Crash Recovery via Snapshot & Log Replay**.

---

## 1. Domain Engineering & Market Dynamics

### A. Continuous Double Auction (L2/L3 Matching Engine)
- Bids sorted by price descending; Asks sorted by price ascending.
- At identical price levels, earlier timestamp maintains strict priority.

### B. Multi-Level Partial Order Fills
- Aggressive crossing orders consume resting liquidity across multiple price tiers.
- Residual unfulfilled volume seamlessly transitions into resting limit orders without book corruption.

### C. Market Volatility & Circuit Breakers
- Real-time volatility anchor tracking:
- Any incoming limit order or trade attempt deviating >25% from the reference anchor trips the engine's circuit breaker (`REJECTED_VOLATILITY_CIRCUIT_BREAKER`), immediately halting new trade generation until reset.

### D. Crash Recovery & Snapshotting
- Periodic snapshot capture: `lastTradePrice`, `totalVolumeTraded`, `activeOrderCount`.
- Engine simulates unexpected process crash, restores from snapshot, and recovers exact market depth and volume without data loss.

---

## 2. File Organization (Clean Architecture)

```
examples/25_hft_order_book/
├── models/
│   └── hft_order_book_request.dto.kora       # Placement, depth & snapshot DTOs
├── entities/
│   ├── hft_order_book_primary.entity.kora    # Active resting orders entity
│   └── hft_order_book_secondary.entity.kora  # Executed trade tape entity
├── services/
│   └── hft_order_book_core.service.kora      # Double auction matching & circuit engine
├── controllers/
│   └── hft_order_book_command.controller.kora# Order submission & depth query controller
├── main.kora                                 # Application container bootstrap
└── tests/
    ├── hft_order_book_unit.test.kora         # 5/5 Priority, partial fills & halt tests
    └── hft_order_book_load.test.kora         # 1,000 Microsecond orders stress suite
```

---

## 3. Verification Commands

```bash
kora check examples/25_hft_order_book/main.kora
kora test examples/25_hft_order_book/tests/hft_order_book_unit.test.kora
kora test examples/25_hft_order_book/tests/hft_order_book_load.test.kora
```
