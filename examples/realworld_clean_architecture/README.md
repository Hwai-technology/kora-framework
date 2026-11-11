# Kora Strict Modular Clean Architecture Enterprise Showcase
### The Definitive Clean Architecture Reference for the Kora Language Ecosystem

> **"Write Less. Build Faster. Run Smarter."**  
> A high-performance, fully featured, enterprise-grade application structured according to **Strict Modular Clean Architecture** principles using the **Kora** programming framework—compiled directly to native Rust with zero-cost abstractions and no garbage collection overhead.

---

## 📑 Table of Contents
1. [Strict Modular Directory Structure](#1-strict-modular-directory-structure)
2. [Domain Segregation & Layer Responsibilities](#2-domain-segregation--layer-responsibilities)
   - [`src/enums/`: Independent Enumerations](#21-srcenums-independent-enumerations)
   - [`src/constants/`: Application Constants & Configurations](#22-srcconstants-application-constants--configurations)
   - [`src/interfaces/`: Inversion of Control Contracts](#23-srcinterfaces-inversion-of-control-contracts)
   - [`src/models/`: Domain Entities & DTOs](#24-srcmodels-domain-entities--dtos)
   - [`src/database/`: Database Management & Auto-Migration](#25-srcdatabase-database-management--auto-migration)
   - [`src/middleware/`: Pipeline Middleware & Response Factory](#26-srcmiddleware-pipeline-middleware--response-factory)
   - [`src/shared/services/`: Enterprise Reusable Services](#27-srcsharedservices-enterprise-reusable-services)
   - [Access Modifiers & Auto-Constructor](#28-access-modifiers--auto-constructor)
   - [Schema Attributes & Decorators](#29-schema-attributes--decorators)
   - [Strict `kebab-case` Wire Formatting](#210-strict-kebab-case-wire-formatting)
   - [Struct-Free Architecture (`model` & `class`)](#211-struct-free-architecture-model--class)
   - [Named Import Syntax (`import { ... } from "..."`)](#212-named-import-syntax)
   - [First-Class Backend Types (`Request`, `Response`, `Headers`)](#213-first-class-backend-types)
   - [Exclusive `null` Semantics (`E2005`)](#214-exclusive-null-semantics-e2005)
   - [Dynamic Request-Bound Security Guards (`AuthGuard`)](#215-dynamic-request-bound-security-guards)
   - [Connection Pooling via `AmbientDatabasePool`](#216-connection-pooling-via-ambientdatabasepool)
3. [Compiler Architectural Purity Diagnostics](#3-compiler-architectural-purity-diagnostics)
4. [Feature Modules Architecture](#4-feature-modules-architecture)
   - [Authentication & Identity Module (`src/modules/auth/`)](#41-authentication--identity-module-srcmodulesauth)
   - [Orders & Commerce Module (`src/modules/orders/`)](#42-orders--commerce-module-srcmodulesorders)
5. [Quickstart & Verification Commands](#5-quickstart--verification-commands)

---

## 1. Strict Modular Directory Structure

The project serves as an authoritative enterprise template demonstrating full domain layer segregation:

```text
examples/realworld_clean_architecture/src/
├── constants/                            # 1. Security configurations & boundary constants
│   └── app_constants.kora                # Rate limits, headers, and crypto salt keys
│
├── enums/                                # 2. Isolated enumeration definitions
│   ├── http_status.kora                  # HTTP status codes (HttpStatus: Ok, Created, NotFound, ...)
│   ├── order_status.kora                 # Workflow states (OrderStatus, PaymentStatus)
│   └── user_role.kora                    # Authorization roles (UserRole: Admin, Customer)
│
├── interfaces/                           # 3. Contract definitions (IoC Interfaces)
│   ├── order_repository_interface.kora   # IOrderRepository
│   ├── user_repository_interface.kora    # IUserRepository
│   └── event_emitter_interface.kora      # IDomainEventEmitter
│
├── models/                               # 4. Domain entities & envelope models
│   ├── user_model.kora                   # User, UserRegisterInput
│   ├── product_model.kora                # Product model
│   ├── order_model.kora                  # Order, OrderItem, CreateOrderInput
│   └── response_envelope.kora            # SecureApiResponse, RequestContext, TelemetryTrace
│
├── database/                             # 5. Database engine & automatic schema migrator
│   ├── database_pool.kora                # Ambient DB connection pool with zero memory leaks
│   └── auto_migrator.kora                # Automatic table provisioner & baseline seeder
│
├── middleware/                           # 6. Pipeline middleware & response factories
│   ├── api_response_factory.kora         # Standardized envelope builder (ok, created, notFound, badRequest)
│   ├── cors_middleware.kora              # Strict CORS preflight policy enforcement
│   ├── rate_limit_middleware.kora        # Token bucket rate limiting (RateLimiterService)
│   ├── compression_middleware.kora       # Payload compression pipeline (CompressionService)
│   └── telemetry_middleware.kora         # Latency profiler & status inspector (TelemetryTracker)
│
├── shared/                               # 7. Shared enterprise utilities & infrastructure
│   └── services/                         # Production-grade shared services
│       ├── email_service.kora            # Transactional email dispatcher (SMTP & AWS SES)
│       ├── sms_service.kora              # OTP token generator & SMS transmitter (Twilio)
│       ├── jwt_service.kora              # Signed JWT encoder/decoder with claims verification
│       ├── crypto_service.kora           # Argon2id password hashing & AES-256-GCM encryption
│       ├── storage_service.kora          # Multi-backend file storage (Local Disk & Cloudflare R2 / S3)
│       ├── cache_service.kora            # Dual-tier cache engine (L1 In-Memory + L2 Redis)
│       ├── session_service.kora          # Session lifecycle manager with sliding expiration
│       ├── logger_service.kora           # Structured JSON logger with contextual trace IDs
│       └── validation_service.kora       # Field constraint validation engine
│
├── modules/                              # 8. Decoupled feature domain modules
│   ├── auth/                             # Complete authentication domain (6 specialized files)
│   │   ├── auth_controller.kora          # Ingestion routing & response formatting
│   │   ├── auth_service.kora             # Identity business logic & credential validation
│   │   ├── auth_types.kora               # LoginInput, RegisterInput, AuthSession DTOs
│   │   ├── auth_repository.kora          # Database queries & account persistence
│   │   ├── auth_cache.kora               # Session store & token revocation registry
│   │   └── auth_guard.kora               # Bearer token extractor & security evaluator
│   │
│   └── orders/                           # Order lifecycle & commerce domain
│       ├── order_controller.kora         # HTTP handlers for order placement
│       ├── order_service.kora            # Volume discount rules & checkout logic
│       ├── order_repository.kora         # Relational database persistence
│       ├── order_types.kora              # Input parameters & checkout DTOs
│       └── order_events.kora             # Domain events & async event queues
│
└── main.kora                             # Minimal application entrypoint & routing setup
```

---

## 2. Domain Segregation & Layer Responsibilities

### 2.1 `src/enums/`: Independent Enumerations
- All enumerations (`HttpStatus`, `OrderStatus`, `PaymentStatus`, `UserRole`) reside in their own dedicated files within `src/enums/`. This prevents circular dependency chains across controllers, services, and models.

### 2.2 `src/constants/`: Application Constants & Configurations
- [`app_constants.kora`](file:///home/ahmet/Desktop/Projects/Kora/examples/realworld_clean_architecture/src/constants/app_constants.kora) declares standard boundary limits, pagination thresholds, token lifetimes, and security header definitions.

### 2.3 `src/interfaces/`: Inversion of Control Contracts
- Declares decoupled interfaces (`IOrderRepository`, `IUserRepository`, `IDomainEventEmitter`) allowing business services to execute domain logic independent of storage engines or external drivers.

### 2.4 `src/models/`: Domain Entities & DTOs
- Contains clean domain entities (`User`, `Product`, `Order`, `OrderItem`) and envelope models (`SecureApiResponse`, `RequestContext`, `TelemetryTrace`).

### 2.5 `src/database/`: Database Management & Auto-Migration
- **Ambient Pool**: Provides thread-safe, RAII-governed connection pooling with zero memory leaks verified by Kora's compiler ownership rules.
- **Auto-Migration & Seeding**: Inspects defined entities and automatically aligns relational schemas while seeding default development data.

### 2.6 `src/middleware/`: Pipeline Middleware & Response Factory
- The [`ApiResponse`](file:///home/ahmet/Desktop/Projects/Kora/examples/realworld_clean_architecture/src/middleware/api_response_factory.kora) factory standardizes API outputs into unified JSON structures:
```kora
return this.response.ok("Order fetched successfully", 1)
return this.response.created("Order placed successfully", 1)
return this.response.notFound("Order not found")
return this.response.badRequest("Invalid input")
```
Every response automatically injects cryptographic nonces, HMAC signatures, and execution latency benchmarks (`execution-time-ms`).

---

### 2.7 `src/shared/services/`: Enterprise Reusable Services
Ten specialized enterprise services handle cross-cutting platform concerns:
1. **Email Service (`email_service.kora`)**: Transactional email dispatch over SMTP and AWS SES with automated retry strategies.
2. **SMS Service (`sms_service.kora`)**: 2FA token generation and SMS delivery integration.
3. **JWT Service (`jwt_service.kora`)**: High-speed HMAC-SHA256 token signing, claims verification, and expiration enforcement.
4. **Crypto Service (`crypto_service.kora`)**: Password hashing via Argon2id, and symmetric payload encryption using AES-256-GCM.
5. **Storage Service (`storage_service.kora`)**: Unified file persistence layer supporting local filesystem storage alongside S3 and Cloudflare R2 signed URLs.
6. **Multi-tier Cache Service (`cache_service.kora`)**: Two-layer caching architecture (L1 Local Memory + L2 Redis) with granular TTL expiration.
7. **Session Service (`session_service.kora`)**: Sliding session expiration, user agent fingerprinting, and instant revocation upon logout.
8. **Structured Logger Service (`logger_service.kora`)**: Structured JSON logging emitting standard log levels (`INFO`, `WARN`, `ERROR`) tagged with correlation `traceId`.
9. **Universal Validation Service (`validation_service.kora`)**: High-performance validation for email, UUID, range, phone, and custom regex patterns.

---

### 2.8 Access Modifiers & Auto-Constructor
Kora classes support strict member accessibility and boilerplate-free parameter properties:
- Explicit modifiers: `public`, `private`, `protected`, and `readonly`.
- **Auto-Constructor**: If an `init()` method is omitted, the compiler synthesizes a full constructor initializing each declared field.
- **Parameter Properties**: Modifiers declared inside constructor signatures (e.g., `init(public repo: Repo, private cache: Cache)`) automatically declare and bind the corresponding instance properties.

### 2.9 Schema Attributes & Decorators
Domain models support rich declarative validation and indexing attributes:
- `@unique`: Enforces unique index constraints on the database and DTO layers.
- `@index`: Configures database performance indexes on high-frequency query fields.
- `@required`: Enforces non-null, non-empty validation invariants.
- `@email`: Validates conforming RFC email address formats.
- `@min(n)`, `@max(n)`: Sets numeric range or string length bounds.
- `@default(val)`: Assigns fallback default expressions.

### 2.10 Strict `kebab-case` Wire Formatting
All JSON payloads, query parameters, and DTO serializations strictly follow **`kebab-case`** conventions via compile-time attribute transformation:
- Field naming: `status-code`, `security-token`, `execution-time-ms`, `order-number`, `total-amount`, `user-id`.
- HTTP headers: `content-type`, `authorization`, `x-request-id`, `x-kora-trace`.

### 2.11 Struct-Free Architecture (`model` & `class`)
The `struct` keyword has been removed in favor of intentional semantic primitives:
- **`model`**: Declares immutable data schemas, DTOs, and relational entities (`model User`, `model SecureApiResponse`).
- **`class`**: Declares stateful services, controllers, and dependency-injected containers featuring behavior and lifecycle hooks.

### 2.12 Named Import Syntax
Kora provides explicit symbol imports to maintain clean dependency trees and eliminate namespace pollution:
```kora
import { HttpStatus } from "enums/http_status";
import { User, UserProfileDto } from "models/user_model";
import { Order, CreateOrderInput } from "models/order_model";
import { SecureApiResponse, LogEntry } from "models/response_envelope";
import { Logger } from "shared/services/logger_service";
```

### 2.13 First-Class Backend Types
Standard backend types are natively available without requiring third-party library imports:
- **`Request`**: Exposes `path`, `method`, `body`, `ip`, `url`, `headers`, along with `header(name)`, `query(key)`, and `param(key)`.
- **`Headers`**: Offers `get(name)`, `set(name, value)`, and `has(name)`.
- **`Response`**: Provides fluent chainable methods: `status(code)`, `json(payload)`, `text(msg)`, and `header(k, v)`.

### 2.14 Exclusive `null` Semantics (`E2005`)
Kora eliminates JavaScript-style `undefined` in favor of a single unified `null` representation mapped directly to `Option<T>`:
- Attempting to use `undefined` results in compiler diagnostic `E2005`:
  ```text
  error[E2005]: `undefined` is not permitted in Kora; use `null` exclusively to represent absence of value.
  ```
- Pattern matching and equality checks operate seamlessly over `null`:
  ```kora
  if req.header("authorization") == null {
      return Err("Authorization header is missing")
  }

  match optUser {
      Some(u) => handleUser(u),
      null => handleNotFound(),
  }
  ```

### 2.15 Dynamic Request-Bound Security Guards
`AuthGuard` inspects inbound requests dynamically without hardcoded mock data:
- Dynamically extracts headers via `req.header("authorization")`.
- Evaluates token validity through `JwtService` and asserts expiration claims.
- Records all authorization passes and security rejections through structured `Logger` channels.

### 2.16 Connection Pooling via `AmbientDatabasePool`
- Eliminates hardcoded mock datasets across repositories and services.
- Acquires and releases pooled connections transparently via `AmbientDatabasePool`.
- Dispatches parameterized queries through `pool.executeQuery(...)` returning strongly typed `Result<T, E>` values.

---

## 3. Compiler Architectural Purity Diagnostics

The Kora compiler (`kora-cli`) enforces clean architecture boundaries directly during static analysis:

| Error Code | Architectural Rule | Prohibited Pattern |
|:---|:---|:---|
| `E5001` | **Purity of `enums/`** | Declaring functions, controllers, classes, or routes inside `enums/`. |
| `E5002` | **Purity of `models/` & `interfaces/`** | Embedding procedural execution logic, route handlers, or server startup calls inside models or interfaces. |
| `E5003` | **Segregation in Controllers & Services** | Inline declarations of `enum` or `model` definitions inside controllers or services (must use explicit `import`). |
| `E5004` | **Clean Root Entry (`main.kora`)** | Declaring inline `model`, `enum`, or `interface` types inside `main.kora`; the entrypoint must focus strictly on composition and startup. |

Violations immediately halt the compilation pipeline with detailed diagnostic pointers.

---

## 4. Feature Modules Architecture

### 4.1 Authentication & Identity Module (`src/modules/auth/`)
Structured into six specialized responsibility layers:
1. **`auth_controller.kora`**: Handles HTTP endpoints for registration and login, wrapping responses in standardized envelopes.
2. **`auth_service.kora`**: Validates credentials, dispatches cryptographic hashes, and coordinates cache writes.
3. **`auth_types.kora`**: DTO schemas (`LoginInput`, `RegisterInput`, `AuthSessionDto`).
4. **`auth_repository.kora`**: Executes relational user storage operations.
5. **`auth_cache.kora`**: Manages Redis session tokens and revoked token blacklists.
6. **`auth_guard.kora`**: Extracts `Authorization: Bearer <token>` and enforces role-based access control.

### 4.2 Orders & Commerce Module (`src/modules/orders/`)
Manages the commerce workflow, applies volume-based pricing policies (e.g., 15% discount for orders containing 5 or more line items), and dispatches asynchronous `OrderPlacedEvent` domain events.

---

## 5. Quickstart & Verification Commands

### 5.1 Static Verification Across All Files
```bash
for f in $(find src -name "*.kora"); do kora check "$f"; done
```
All system files pass with zero errors:
```text
✓ Checked `src/...` — 0 errors (frontend & ownership verified).
```

### 5.2 Generate Native Rust Code
```bash
kora codegen src/main.kora
```

### 5.3 Test System Endpoints via curl
```bash
# 1. Health check probe
curl http://localhost:7582/health

# 2. Trigger schema auto-migration
curl http://localhost:7582/api/v1/database/migrate

# 3. Authenticate user
curl -X POST http://localhost:7582/api/v1/auth/login

# 4. Place order, compute volume discounts, and broadcast event
curl -X POST http://localhost:7582/api/v1/orders
```
