# kora-runtime

> **Kora Concurrency & Async Actor Runtime**

Provides Tokio-powered asynchronous concurrency: bounded MPSC channels, Pub/Sub broadcast channels, isolated stateful actors (tell/ask), and TaskGroups.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Runtime Subsystem — Powers execution of asynchronous Kora applications and services.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `Channel<T>`
- `BroadcastChannel<T>`
- `ActorRef<M, R>`
- `spawn_actor`
- `TaskGroup<T>`
- `KoraRuntime`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Bounded channel backpressure prevents memory exhaustion. Actor query timeouts protect against deadlocks.

---

## 🚀 How to Contribute & Improve

Add distributed actor clustering across network nodes with automatic leader election and heartbeats.

### Testing this Crate
```bash
cargo test -p kora-runtime
cargo clippy -p kora-runtime --all-targets -- -D warnings
```
