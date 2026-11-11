# kora-realtime

> **Kora Realtime WebSocket & Event Engine**

High-throughput WebSocket pub/sub connection manager for broadcasting domain events to connected clients with channel isolation.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Realtime Streaming — Powers live web and mobile updates.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `RealtimeServer`
- `ChannelHub`
- `DomainEvent`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Enforces per-client connection limits, authentication handshakes, and subscription authorization.

---

## 🚀 How to Contribute & Improve

Implement Redis Pub/Sub backplane for horizontal multi-server WebSocket scale-out.

### Testing this Crate
```bash
cargo test -p kora-realtime
cargo clippy -p kora-realtime --all-targets -- -D warnings
```
