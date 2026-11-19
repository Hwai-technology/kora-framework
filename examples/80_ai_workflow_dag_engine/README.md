# Example 80: AI Workflow Directed Acyclic Graph (DAG) Engine

Topological sort dependency executor, state checkpoint persistence, failure retry branch policies, and human-in-the-loop review steps.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/80_ai_workflow_dag_engine/
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
kora check examples/80_ai_workflow_dag_engine/main.kora
kora test examples/80_ai_workflow_dag_engine/tests/
```
