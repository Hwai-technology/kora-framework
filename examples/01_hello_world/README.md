# Example 01: Hello World & Language Fundamentals

## Overview
This example demonstrates the core syntax and language primitives of Kora:
- **Declarative Data Models**: Pure data schemas defined via `model` without Rust struct complexity.
- **Algebraic Data Types (`enum`)**: Enums with unit, tuple, or structured variants in dedicated `enums/` modules.
- **Pattern Matching (`match`)**: Exhaustive pattern matching over enum variants.
- **Functions & Type Inference**: Strong static typing with type inference for local variables.
- **Architectural Purity**: Clean separation between data models, enums, and executable logic.

## Directory Structure
```
01_hello_world/
├── README.md
├── main.kora                   # Entrypoint with functions and match logic
├── enums/
│   └── order_status.kora       # Dedicated algebraic enum definition
└── models/
    └── coordinates.kora        # Dedicated declarative data model
```

## Running the Example
```bash
# Type check the source files
kora check main.kora

# Execute the entrypoint
kora run main.kora
```

## Expected Output
```
Ahmet
Order payment confirmed
```
