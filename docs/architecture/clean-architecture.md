# Kora Clean Architecture & Modular Purity

Kora enforces Clean Architecture principles through compiler-level diagnostics:

## Standard Directory Structure
```text
src/
├── constants/       # Global constants, limits, security parameters
├── enums/           # Pure enumeration types (HttpStatus, OrderStatus)
├── interfaces/      # Contract interfaces for IoC and decoupling
├── models/          # Declarative data schemas and DTOs
├── database/        # Database pools, migrations, seeders
├── middleware/      # Interceptors, response factories, rate limiters
├── shared/services/ # Cross-cutting enterprise services
└── modules/         # High-cohesion domain feature modules
    └── <feature>/
        ├── <feature>_controller.kora
        ├── <feature>_service.kora
        ├── <feature>_repository.kora
        ├── <feature>_types.kora
        ├── <feature>_guard.kora
        └── <feature>_cache.kora
```

## Compiler Diagnostics
- **`E5001`**: Files in `enums/` cannot define functions, classes, or routes.
- **`E5002`**: Files in `models/` or `interfaces/` cannot define server runners or executable logic.
- **`E5003`**: Services and controllers cannot declare inline models or enums; they must import them explicitly.
- **`E5004`**: `main.kora` cannot declare inline models or enums; it must import them from dedicated architectural modules.

