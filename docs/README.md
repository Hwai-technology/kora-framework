# 📚 Kora Language & Framework Documentation Portal

Welcome to the official documentation for **Kora** — the high-performance, statically typed, garbage-collector-free backend programming language and Clean Architecture framework designed for modern cloud services.

```
Write Less. Build Faster. Run Smarter.
⚡ Sub-millisecond execution · 🏛️ Clean Architecture · 🛡️ Memory & Type Safety
```

---

## 🧭 Documentation Navigation

### 1. 🚀 Getting Started
- [**Installation & Setup Guide**](getting-started/installation.md): One-command installer, system PATH, updating, uninstallation.
- [**5-Minute Quickstart Tutorial**](getting-started/quickstart.md): Create your first Clean Architecture microservice from scratch.

### 2. 🔤 Language Fundamentals
- [**Syntax & Type System**](language/syntax-and-types.md): Primitives, Single-Null philosophy, Collections, Functions.
- [**Models & Classes**](language/models-and-classes.md): Data Entities vs DTOs vs Stateful Classes, Decorators.
- [**Pattern Matching & Enums**](language/pattern-matching.md): Algebraic data types, Match expressions, Safe unwrapping.
- [**Grammar Specification (EBNF)**](grammar.ebnf): Formal language grammar specification.

### 3. 🏛️ Architecture & Best Practices
- [**Clean Architecture Guide**](architecture/clean-architecture.md): Modular directory structure, architectural linting rules.
- [**Compile-Time Dependency Injection**](architecture/dependency-injection.md): Inversion of Control, cycle detection, zero-reflection.

### 4. 🛠️ Command-Line Interface (CLI)
- [**CLI Command Reference**](cli/cli-reference.md): Full command-by-command manual (`run`, `watch`, `fmt`, `check`, `pkg`, `update`, etc.).
- [**Compiler Error Codes Catalog**](cli/error-codes.md): Exhaustive guide to all diagnostics (`E1001` to `E5004`) with remedies.

### 5. 🌐 Ecosystem & Integrations
- [**Zero-Config Swagger & OpenAPI 3.1**](ecosystem/swagger-and-openapi.md): Automatic documentation discovery from models and routes.
- [**Realtime & WebSocket Events**](ecosystem/realtime-and-events.md): Live domain state broadcast, transactional guarantees.
- [**IDE & Tooling Ecosystem**](ecosystem/ide-and-tooling.md): VS Code extension, Language Server Protocol (LSP), Neovim, Docker, CI/CD.

### 6. 💡 Practical Recipes
- [**Practical Recipes & Examples**](examples/practical-recipes.md): Production-ready code templates (CRUD APIs, JWT Auth, Database pool).

---

## ⚡ Quick Cheat Sheet

```bash
# Verify file and architecture
kora check src/main.kora

# Run file in JIT-like sub-second execution
kora run src/main.kora

# Live-reload watch mode during development
kora watch src/main.kora

# Canonical code formatting
kora fmt .

# Initialize Clean Architecture in current folder
kora init

# Create a new project in a new folder
kora create my-service

# Generate a new CRUD domain feature
kora g feature products

# Refresh & update Kora to the latest version
kora update
```

