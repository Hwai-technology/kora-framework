# Example 02: Clean Architecture Users API

## Overview
This example demonstrates Kora's Clean Architecture patterns for REST services:
- **Model / Class Separation**: Pure declarative DTOs in `models/` and business logic in `services/`.
- **Interface Contracts**: Repository interfaces in `interfaces/` defining typed data access boundaries.
- **Dependency Injection**: Automatic route parameter injection of service instances (`service: UserService`).
- **Input Validation & Results**: Strongly-typed `Result<T, E>` and `Option<T>` for error and presence handling.

## Directory Structure
```
02_clean_architecture_users/
├── README.md
├── main.kora                   # REST endpoints and route orchestration
├── interfaces/
│   └── user_repository.kora    # Repository contract interface
├── models/
│   └── user_models.kora        # Declarative user entity model
└── services/
    └── user_service.kora       # Domain business logic & validation
```

## Running the Example
```bash
# Type check the architecture
kora check main.kora

# Run the REST server
kora run main.kora

# Test the endpoints
curl http://localhost:7582/users
curl http://localhost:7582/users/1
curl -X POST http://localhost:7582/users -d "Alice"
```

