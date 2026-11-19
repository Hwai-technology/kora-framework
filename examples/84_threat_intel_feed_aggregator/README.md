# Example 84: Threat Intelligence STIX/TAXII Indicator Parser

STIX 2.1 JSON indicator ingestion, TAXII server pollers, malicious IP/domain Bloom filter lookup, and SIEM alert dispatch.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/84_threat_intel_feed_aggregator/
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
kora check examples/84_threat_intel_feed_aggregator/main.kora
kora test examples/84_threat_intel_feed_aggregator/tests/
```
