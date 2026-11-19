# Example 86: SIEM High-Throughput Log Ingestion & CEF Normalizer

Syslog RFC 5424 parser, Common Event Format (CEF) normalizer, high-throughput ingest actor, and correlation alert engine.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/86_siem_log_ingestion_pipeline/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/86_siem_log_ingestion_pipeline/main.kora
kora test examples/86_siem_log_ingestion_pipeline/tests/
```
