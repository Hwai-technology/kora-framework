# Example 06: Realtime WebSockets & Event Streaming Gateway

Production-grade WebSocket Gateway and federated Pub/Sub hub implementing **RFC 6455 Framing, Token Upgrade Authentication, Ping/Pong Heartbeats, Channel Isolation, and Watchdog Eviction**.

---

## 1. Real Engineering Architecture & RFC 6455 Invariants

### A. HTTP Upgrade Handshake & Bearer Authentication
- Upgrade requests to `/ws/connect` require a cryptographically valid token (`Authorization: Bearer kora_ws_token_<user>`).
- Unauthenticated upgrade attempts are rejected with `401 Unauthorized` before establishing TCP connection upgrades.

### B. RFC 6455 Framing & Protocol Defense
- **Masking Invariant (Section 5.1)**: Client-to-server frames **MUST** be masked. Any unmasked frame is immediately rejected with close frame `1002 Protocol Error`, and the socket is abruptly terminated.
- **OpCode Routing**:
  - `OpCode 1 (Text)`: Ingests payload and increments message telemetry.
  - `OpCode 8 (Close)`: Clean socket teardown, broadcast unsubscription, and normal closure response `1000`.
  - `OpCode 9 (Ping)`: Immediate turnaround responding with `OpCode 10 (Pong)` echoing the exact ping payload.

### C. Dead Connection Heartbeat Watchdog
- Sessions track monotonic `lastHeartbeatTs`.
- Background watchdog checks active connections against `WS_HEARTBEAT_TIMEOUT_MS`. Sessions without heartbeat responses are automatically evicted and reclaimed from room memberships.

### D. Federated Pub/Sub & Redis Federation
- Multi-room channel routing (`high_frequency_orders`, `market_data`, `general`).
- Topic broadcast delivers exclusively to subscribers of that specific room, recording zero dropped frames under high throughput.

---

## 2. Infrastructure Setup (Docker Compose)

```bash
# Start Redis 7 Pub/Sub & WebSocket Gateway
docker compose up -d

# Verify Redis health
docker compose exec redis redis-cli -a kora_secret_redis_pass ping
```

---

## 3. Clean Architecture Hierarchy

```
06_realtime_websockets/
├── .env.example                               # Environment & framing configuration
├── docker-compose.yml                         # Redis 7 container & gateway orchestration
├── enums/
│   ├── ws_opcode.enum.kora                    # RFC 6455 OpCode definitions (Text, Binary, Ping, Pong, Close)
│   └── ws_session_state.enum.kora             # Session state machine (Handshaking, Active, Terminated)
├── events/
│   └── websocket.event.kora                   # Domain events (Connected, Broadcast, HeartbeatTimeout)
├── models/
│   └── chat.models.kora                       # WsFrame, WsSession, RoomPresence, BroadcastEnvelope
├── interfaces/
│   └── chat_hub.interface.kora                # Gateway & hub contract
├── repositories/
│   └── chat.repository.kora                  # Vector-backed active session & room state manager
├── services/
│   └── chat_hub.service.kora                  # RFC 6455 framing, ping/pong, and broadcast engine
├── controllers/
│   └── chat.controller.kora                   # HTTP upgrade handshake & REST telemetry
├── routes/
│   └── chat.routes.kora                       # /ws/connect, /ws/broadcast, /ws/presence
├── main.kora                                  # Application container bootstrap
└── tests/
    ├── websockets.test.kora                   # End-to-end integration lifecycle test
    ├── websockets_unit.test.kora              # 6/6 RFC 6455 protocol & masking unit tests
    └── websockets_load.test.kora              # 250 Concurrent sessions & broadcast fanout stress test
```

---

## 4. Running & Validating

```bash
# Type-check the entire project
kora check main.kora

# Execute automated test suite
kora test tests/
```
