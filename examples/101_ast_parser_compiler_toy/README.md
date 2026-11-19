# Example 101: Pratt Parsing & Bytecode Virtual Machine

Top-Down Operator Precedence (Pratt Parser), Abstract Syntax Tree (AST) node generation, and register-based stack VM bytecode emulator.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/101_ast_parser_compiler_toy/
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
kora check examples/101_ast_parser_compiler_toy/main.kora
kora test examples/101_ast_parser_compiler_toy/tests/
```
