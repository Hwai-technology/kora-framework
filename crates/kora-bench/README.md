# kora-bench

> **Kora Quantitative Benchmark Suite**

Reproducible quantitative benchmarking framework comparing Kora against Rust (Axum), Go (Gin), Node.js (Fastify), and PHP (Laravel).

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Performance Verification — Powers kora bench CLI command.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `BenchmarkHarness`
- `BenchmarkResult`
- `simulate_run`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Deterministic warmup cycles and statistical outlier rejection prevent thermal throttling bias.

---

## 🚀 How to Contribute & Improve

Add real HTTP load generation against live running server processes using wrk or autocannon.

### Testing this Crate
```bash
cargo test -p kora-bench
cargo clippy -p kora-bench --all-targets -- -D warnings
```
