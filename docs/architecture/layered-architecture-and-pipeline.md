# Backend Architecture & Request Lifecycle

## Philosophy

Kora is designed to support highly structured, maintainable, and scalable backend applications.

The framework enforces a clear separation of concerns through a **Layered Clean Architecture**. HTTP routing, request interception, presentation handling, business rules, and data persistence remain strictly decoupled responsibilities.

Kora discourages placing business logic directly inside route handlers or application entry points.

---

## Standard Request Lifecycle

Every HTTP request follows a deterministic, unidirectional execution pipeline:

```text
Application
    │
    ▼
Middleware Pipeline (Global & Route-specific)
    │
    ▼
Routes (Endpoint declaration & Controller binding)
    │
    ▼
Controllers (HTTP transport, params/body parsing, Response generation)
    │
    ▼
Services (Business logic, domain validation, transaction rules)
    │
    ▼
Repositories (Data access abstraction, entity mapping)
    │
    ▼
Database (Persistent storage & query execution)
```

### Execution Steps
1. **Application Initialization**: The DI container constructs and injects services, repositories, and controllers.
2. **Global Middleware Execution**: Interceptors handle authentication, CORS, rate-limiting, and telemetry.
3. **Route Matching**: The HTTP engine matches the verb and URI pattern.
4. **Route-Specific Middleware Execution**: Route guards and validation middleware execute before the controller.
5. **Controller Execution**: Extracts path parameters (`request.params["id"]`), query strings, and payloads (`request.body`).
6. **Service Execution**: Executes business workflows and orchestrates domain operations.
7. **Repository Execution**: Queries database pools and entity collections.
8. **Database Operation**: Transactions commit or roll back.
9. **Response Generation**: The controller transforms domain results into structured HTTP responses (`Response.json(...)`).

---

## Architectural Layers

| Layer | Responsibility | Allowed Dependencies |
|---|---|---|
| **Application (`main.kora`)** | Application initialization, module registration via `app.use(...)`, port listening | Middleware, Route Modules |
| **Middleware** | Authentication, authorization, logging, telemetry, CORS, header mutations | Services, Interfaces |
| **Routes** | HTTP method and endpoint declaration, binding to controller actions | Controllers |
| **Controllers** | HTTP request unwrapping, parameter extraction, response formatting | Services, DTOs |
| **Services** | Domain rules, workflow coordination, business logic | Repositories, Validators, Events |
| **Repositories** | SQL/NoSQL query execution, database schema interactions | `Database`, Entity Models |
| **Models & DTOs** | Pure schema contracts, transfer objects, database entity mappings | None (Pure Data Schemas) |

---

## Standard Implementation Example

### 1. Application Entry Point (`src/main.kora`)
The main file initializes the server container and registers modules without defining inline business logic:

```kora
import { App } from "kora/http"
import { ChatRoutes } from "./modules/chat/chat.routes"
import { AuthMiddleware } from "./middleware/auth.middleware"
import { LoggerMiddleware } from "./middleware/logger.middleware"

let app = App()

// Global Middleware Pipeline
app.use(LoggerMiddleware)
app.use(AuthMiddleware)

// Register Domain Route Modules
app.use(ChatRoutes)

// Start Server Listener
app.listen(3000)
```

### 2. Routes Layer (`src/modules/chat/chat.routes.kora`)
Routes only define HTTP endpoints and connect them to their corresponding controller actions:

```kora
import { ChatController } from "./chat.controller"

class ChatRoutes {
    controller: ChatController

    init(controller: ChatController) {
        this.controller = controller
    }

    fn register(app: App) -> App {
        app.get(
            "/chat/presence/{room}",
            this.controller.presence
        )

        app.post(
            "/chat/message",
            this.controller.sendMessage
        )

        return app
    }
}
```

### 3. Controller Layer (`src/modules/chat/chat.controller.kora`)
The controller handles transport-level details, extracts request parameters, calls the service, and serializes the response:

```kora
import { ChatService } from "./chat.service"

class ChatController {
    service: ChatService

    init(service: ChatService) {
        this.service = service
    }

    fn presence(request: Request) -> Response {
        let room = request.params["room"]
        let result = this.service.getPresence(room)
        return Response.json(result)
    }

    fn sendMessage(request: Request) -> Response {
        let result = this.service.sendMessage(request.body)
        return Response.json(result)
    }
}
```

### 4. Service Layer (`src/modules/chat/chat.service.kora`)
The service encapsulates application business rules and coordinates data persistence:

```kora
import { ChatMessage, RoomPresence } from "./models/chat_models"
import { ChatRepository } from "./chat.repository"

class ChatService {
    repository: ChatRepository

    init(repository: ChatRepository) {
        this.repository = repository
    }

    fn sendMessage(body: String) -> ChatMessage {
        let message = ChatMessage {
            id: 1,
            sender: "Ahmet",
            room: "general",
            content: body,
            timestamp: 1710000000,
        }
        return this.repository.create(message)
    }

    fn getPresence(room: String) -> RoomPresence {
        return this.repository.getPresence(room)
    }
}
```

### 5. Repository Layer (`src/modules/chat/chat.repository.kora`)
The repository isolates persistent storage access:

```kora
import { ChatMessage, RoomPresence } from "./models/chat_models"

class ChatRepository {
    database: Database

    init(database: Database) {
        this.database = database
    }

    fn create(message: ChatMessage) -> ChatMessage {
        let res = this.database.chatMessages.create(message)
        return message
    }

    fn getPresence(room: String) -> RoomPresence {
        return RoomPresence {
            room: room,
            activeMembers: 42,
        }
    }
}
```

---

## Middleware Architecture

Kora supports middleware composition at three distinct levels:

### 1. Global Middleware
Executed for every incoming HTTP request:
```kora
app.use(LoggerMiddleware)
app.use(CorsMiddleware)
```

### 2. Route-Specific Middleware
Attached to individual endpoints as an execution guard:
```kora
app.post(
    "/chat/message",
    [AuthMiddleware, ValidationMiddleware],
    ChatController.sendMessage
)
```

### 3. Group / Router-Level Middleware
Scoped to a specific URL sub-tree:
```kora
let chat = Router("/chat")

chat.use(AuthMiddleware)
chat.get("/presence/{room}", ChatController.presence)
chat.post("/message", ChatController.sendMessage)

app.use(chat)
```

---

## Native Compile-Time Dependency Injection

Kora features built-in compile-time Dependency Injection:
- Constructors declared with `init(...)` explicitly declare incoming dependencies.
- Every dependency parameter must be a `class`, `interface`, `Database`, or `App`.
- Primitive types (`Int`, `String`) cannot be injected into service constructors; they must be passed to methods.
- The Kora compiler checks the entire dependency graph, verifies interface conformance, and detects circular dependencies at build time.
