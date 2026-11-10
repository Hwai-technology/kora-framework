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

## Database Lifecycle Management
Kora provides built-in commands and REST endpoints for managing the full database lifecycle:

```bash
# Run schema migrations
kora db migrate [src/main.kora]

# Synchronize schema differences automatically with @entity models
kora db sync [src/main.kora]

# Seed initial database fixtures
kora db seed [src/main.kora]

# Truncate all table data cleanly (without dropping schema)
kora db truncate [src/main.kora]

# Drop all database tables
kora db drop [src/main.kora]

# Reset and refresh database (drop -> migrate -> seed)
kora db refresh [src/main.kora]
```

