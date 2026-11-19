# Example 74: Retrieval-Augmented Generation (RAG) Document Chunker

Recursive token-boundary document chunking, hybrid dense vector + sparse BM25 retrieval ranker, and cross-encoder re-ranking.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/74_rag_knowledge_pipeline/
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
kora check examples/74_rag_knowledge_pipeline/main.kora
kora test examples/74_rag_knowledge_pipeline/tests/
```
