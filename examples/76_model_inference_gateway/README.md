# Example 76: Model Serving Inference Gateway & Dynamic Batcher

Microsecond dynamic request batching, GPU/NPU multi-worker round-robin distributor, and Server-Sent Events (SSE) token streaming.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/76_model_inference_gateway/
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
kora check examples/76_model_inference_gateway/main.kora
kora test examples/76_model_inference_gateway/tests/
```
