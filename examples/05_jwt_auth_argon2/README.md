# Example 05: JWT Authentication & Argon2id Password Hashing

## Overview
This example demonstrates secure enterprise authentication and session management in Kora:
- **Argon2id Password Hashing**: State-of-the-art password security resistant to GPU brute-forcing.
- **HMAC-SHA256 JWT Tokens**: Stateless, cryptographically signed bearer tokens for distributed microservices.
- **DTO Validation**: Strict declarative models for registration and login payloads.
- **Clean Architecture Segregation**: Auth DTOs in `models/`, interface contract in `interfaces/`, and business logic in `services/`.

## Directory Structure
```
05_jwt_auth_argon2/
├── README.md
├── main.kora                   # Authentication endpoints (`/auth/register`, `/auth/login`)
├── interfaces/
│   └── auth_service_interface.kora # Authentication service contract
├── models/
│   └── auth_dto.kora           # Registration, login, user, and token models
└── services/
    └── auth_service.kora       # Password hashing & JWT generation implementation
```

## Running the Example
```bash
# Type check the authentication pipeline
kora check main.kora

# Start the authentication service
kora run main.kora

# Register a new user
curl -X POST http://localhost:7582/auth/register -d "SuperSecretPassword123!"

# Login to receive JWT bearer token
curl -X POST http://localhost:7582/auth/login -d "SuperSecretPassword123!"
```
