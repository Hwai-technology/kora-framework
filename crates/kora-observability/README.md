# kora-observability

> **Kora Metrics, Structured Logging & Tracing**

Provides structured JSON logging, distributed OpenTelemetry trace propagation, and atomic performance counters.

---

## 🏛️ Architectural Role & Pipeline

* **Pipeline Placement:** Observability & Monitoring — Delivers telemetry for production workloads.
* **License:** GNU Affero General Public License v3 (AGPL-3.0)
* **File Support:** Fully supports `.kora` and `.kr` file extensions.

---

## 🧩 Key Types & Public API

- `MetricsCollector`
- `StructuredLogger`
- `SpanTracker`

---

## 🛡️ Security & Invariants

* **Memory Safety:** 100% safe Rust. Compiles under `#[forbid(unsafe_code)]`.
* **Security Guarantees:** Automatic redacting of sensitive credential patterns (passwords, tokens, keys) before writing to log streams.

---

## 🚀 How to Contribute & Improve

Add native Prometheus /metrics HTTP scrape endpoint exporter.

### Testing this Crate
```bash
cargo test -p kora-observability
cargo clippy -p kora-observability --all-targets -- -D warnings
```
