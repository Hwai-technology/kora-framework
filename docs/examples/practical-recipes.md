# Practical Recipes & Production Examples

This guide contains complete, ready-to-use production recipes for Kora applications.

---

## 1. REST API with Clean Architecture

```kora
// src/features/products/models/product_dto.kora
model CreateProductDto {
    name: String,
    priceCents: Int,
}

model ProductResponseDto {
    id: Int,
    name: String,
    priceCents: Int,
    inStock: Bool,
}

// src/features/products/products.service.kora
import { CreateProductDto, ProductResponseDto } from "./models/product_dto"

class ProductsService {
    fn create(dto: CreateProductDto) -> ProductResponseDto {
        return ProductResponseDto {
            id: 1,
            name: dto.name,
            priceCents: dto.priceCents,
            inStock: true,
        }
    }

    fn list() -> List<ProductResponseDto> {
        return [
            ProductResponseDto { id: 1, name: "Mechanical Keyboard", priceCents: 12000, inStock: true },
            ProductResponseDto { id: 2, name: "4K Monitor", priceCents: 45000, inStock: true },
        ]
    }
}

// src/features/products/products.controller.kora
import { ProductsService } from "./products.service"
import { CreateProductDto, ProductResponseDto } from "./models/product_dto"

class ProductsController {
    service: ProductsService,

    init(service: ProductsService) {
        this.service = service
    }

    fn listProducts() -> List<ProductResponseDto> {
        return this.service.list()
    }

    fn createProduct(dto: CreateProductDto) -> ProductResponseDto {
        return this.service.create(dto)
    }
}
```

---

## 2. Main Entrypoint & Route Dispatch

```kora
// src/main.kora
import { ProductsController } from "./features/products/products.controller"
import { ProductsService } from "./features/products/products.service"
import { CreateProductDto } from "./features/products/models/product_dto"
import { LoggerService } from "./shared/services/logger_service"

let logger = LoggerService {}
logger.info("Initializing Products Microservice...")

let service = ProductsService {}
let controller = ProductsController { service: service }

let catalog = controller.listProducts()
logger.info("Current product catalog loaded:")
for item in catalog {
    print(item.name)
}
```

---

## 3. Password Hashing & Authentication with Argon2id

Kora provides built-in enterprise authentication utilities:

```kora
import { hashPassword, verifyPassword } from "kora_auth"

let password = "SuperSecretPassword123!"
let hash = hashPassword(password)

let isMatch = verifyPassword(password, hash)
if isMatch {
    print("Authentication successful!")
}
```

---

## 4. Structured JSON Logging

```kora
import { LoggerService } from "./shared/services/logger_service"

let logger = LoggerService {}
logger.info("Server started on port 8080")
logger.error("Database connection retry attempt 1")
```

