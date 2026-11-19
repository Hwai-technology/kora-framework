# Example 73: Autonomous LLM Agent Orchestrator & Tool Calling

ReAct prompt reasoning loops, structured JSON tool invocation dispatcher, short-term conversational context buffers, and guardrails.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/73_llm_agent_orchestrator/
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
kora check examples/73_llm_agent_orchestrator/main.kora
kora test examples/73_llm_agent_orchestrator/tests/
```
