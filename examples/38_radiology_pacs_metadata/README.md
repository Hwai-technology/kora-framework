# Example 38: Radiology PACS DICOM Metadata Indexer

DICOM header parsing, imaging study series grouping (MRI, CT, X-Ray), radiologist priority worklist, and PACS archive retrieval.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/38_radiology_pacs_metadata/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Clinical domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/38_radiology_pacs_metadata/main.kora
kora test examples/38_radiology_pacs_metadata/tests/
```
