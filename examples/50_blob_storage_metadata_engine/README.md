# Example 50: Distributed Blob Storage Metadata Engine

Multipart upload chunk assembler, content deduplication hashing (SHA-256), multi-region replication, and lifecycle tiering.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/50_blob_storage_metadata_engine/
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
kora check examples/50_blob_storage_metadata_engine/main.kora
kora test examples/50_blob_storage_metadata_engine/tests/
```
