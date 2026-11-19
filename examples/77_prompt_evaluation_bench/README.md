# Example 77: Prompt Engineering Evaluation & Hallucination Scorer

Automated red-teaming adversarial prompt suite, G-Eval semantic consistency scoring, and prompt regression diff analytics.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/77_prompt_evaluation_bench/
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
kora check examples/77_prompt_evaluation_bench/main.kora
kora test examples/77_prompt_evaluation_bench/tests/
```
