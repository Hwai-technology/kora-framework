# Example 09: Enterprise Clean Architecture Real-World Application

## Overview
This showcase presents a complete, production-grade enterprise microservice written entirely in Kora. It demonstrates full modularization across 40+ source files adhering strictly to Uncle Bob's Clean Architecture and Domain-Driven Design (DDD) principles:

- **Enterprise Layering**:
  - `src/models/`: Declarative domain entities and DTOs (zero executable code).
  - `src/interfaces/`: Pure behavioral contracts and repository interfaces.
  - `src/enums/`: Algebraic data types and business state constants.
  - `src/modules/`: Domain feature modules (Users, Orders, Billing, Inventory) containing isolated services and controllers.
  - `src/middleware/`: Cross-cutting concerns (authentication, telemetry, rate limiting).
  - `src/database/`: Database connection management, migration runners, and transactions.
  - `src/shared/`: Shared utilities and cross-module helpers.
  - `src/main.kora`: Ultra-lean application orchestration and route registration.

- **Source Code Location**:
  The complete source code for this enterprise showcase is located in:
  [`examples/realworld_clean_architecture/`](../realworld_clean_architecture/)

## Key Architectural Principles Verified by the Compiler
1. **Model-Class Segregation (`E5003`)**: No file may co-locate a `model` and a `class`.
2. **Interface Purity (`E5002`)**: No executable server logic or routes allowed inside `interfaces/` or `models/`.
3. **Enum Purity (`E5001`)**: No functions or route handlers allowed inside `enums/`.
4. **Main Lean Orchestration (`E5004`)**: `main.kora` serves solely for boot orchestration and dependency wiring.

## Verifying All Enterprise Files
```bash
# Type check all 40 architectural files in the enterprise project:
for f in $(find examples/realworld_clean_architecture/src -name "*.kora"); do
  kora check "$f"
done
```

## Running the Enterprise Application
```bash
cd examples/realworld_clean_architecture
kora run
```
