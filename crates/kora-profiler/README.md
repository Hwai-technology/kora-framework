# kora-profiler

> **Kora CPU & Memory Allocation Profiler**

Instruments Kora applications to track execution time per function, allocations, and database query latencies.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Performance Tooling — Guides performance optimization.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `ProfilerHarness`
- `ProfileReport`
- `LatencyHistogram`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Zero overhead when inactive; sampling profiler uses low-overhead thread signals to avoid degrading performance.

---

## 🚀 How to Contribute & Improve

Generate interactive flamegraphs directly viewable in the browser or Antigravity IDE.

### Testing this Crate
```bash
cargo test -p kora-profiler
cargo clippy -p kora-profiler --all-targets -- -D warnings
```
