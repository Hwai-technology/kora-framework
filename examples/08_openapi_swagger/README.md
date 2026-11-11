# Example 08: OpenAPI 3.1 & Interactive Swagger UI

## Overview
This example demonstrates Kora's zero-configuration OpenAPI specification and interactive Swagger UI:
- **Zero-Config Discovery**: Scans routes and `model` declarations to synthesize compliant OpenAPI 3.1 JSON.
- **Embedded Swagger UI**: Serves interactive API explorer and testing console natively at `/docs`.
- **Dynamic SEO & Metadata**: Injects dynamic OpenGraph, canonical tags, and Geo headers into the Swagger UI page head.
- **Clean Architecture Segregation**: Models in `models/`, interface contracts in `interfaces/`, and business logic in `services/`.

## Directory Structure
```
08_openapi_swagger/
├── README.md
├── main.kora                   # REST routes documented by OpenAPI
├── interfaces/
│   └── article_repository.kora # Article repository contract
├── models/
│   └── article_models.kora     # Domain models and response schemas
└── services/
    └── article_service.kora    # Service providing article business logic
```

## Running the Example
```bash
# Type check the API
kora check main.kora

# Generate OpenAPI 3.1 specification and Swagger documentation
kora swagger

# Launch the server
kora run main.kora

# Open Swagger UI in your browser:
# http://localhost:7582/docs

# Inspect the raw OpenAPI 3.1 JSON definition:
# http://localhost:7582/docs/openapi.json
```
