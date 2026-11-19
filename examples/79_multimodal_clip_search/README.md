# Example 79: Multimodal CLIP Cross-Search Engine

Joint text-image 512D embedding projection, zero-shot visual classification, thumbnail cache, and cross-modal retrieval.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/79_multimodal_clip_search/
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
kora check examples/79_multimodal_clip_search/main.kora
kora test examples/79_multimodal_clip_search/tests/
```
