# kora-queue

> **Kora Distributed Background Job Queue**

Asynchronous job queue engine with retry policies, exponential backoff, dead-letter queues (DLQ), and in-memory or Redis backends.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Async Task Processing — Executes background tasks out-of-band.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `JobQueue`
- `Job`
- `JobStatus`
- `RetryPolicy`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Payload size limits and bounded queue depth prevent job flood denial-of-service.

---

## 🚀 How to Contribute & Improve

Add persistent disk-backed WAL (Write-Ahead Log) for zero-loss message delivery across unexpected node restarts.

### Testing this Crate
```bash
cargo test -p kora-queue
cargo clippy -p kora-queue --all-targets -- -D warnings
```
