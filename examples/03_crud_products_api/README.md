# Example 03: RESTful CRUD Products API with Dynamic SEO & Geo Meta

## Overview
This example demonstrates a complete, production-grade CRUD REST API:
- **Full HTTP Method Suite**: `GET`, `POST`, `PUT`, and `DELETE` endpoints.
- **Dynamic SEO Metadata**: OpenGraph, Twitter card, canonical URLs, and structured meta generation.
- **Dynamic Geolocation**: Reverse-proxy client IP and location enrichment.
- **Clean Architecture Segregation**: Models in `models/`, interface contracts in `interfaces/`, and business logic in `services/`.
- **Result & Option Enums**: Compile-time safe error signaling with `Result<T, E>` and absence checks with `Option<T>`.

## Directory Structure
```
03_crud_products_api/
├── README.md
├── main.kora                   # REST routes for CRUD operations
├── interfaces/
│   └── product_repository.kora # Product repository interface contract
├── models/
│   └── product_models.kora     # Domain models, requests, responses, and SEO/Geo DTOs
└── services/
    └── product_service.kora    # Service implementation with CRUD operations & SEO logic
```

## Running the Example
```bash
# Type check the architecture
kora check main.kora

# Launch the server (runs natively on port 7582)
kora run main.kora

# Query the catalog with dynamic SEO and Geo metadata
curl http://localhost:7582/products

# Query a single product
curl http://localhost:7582/products/1

# Create a new product
curl -X POST http://localhost:7582/products -d "Kora Mechanical Keyboard"

# Update product
curl -X PUT http://localhost:7582/products/1 -d "Updated Title"

# Delete product
curl -X DELETE http://localhost:7582/products/1
```

