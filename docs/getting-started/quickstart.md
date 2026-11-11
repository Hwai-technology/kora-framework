# 🚀 5-Minute Quickstart Tutorial

In this tutorial, you will create, verify, and run a production-ready Clean Architecture microservice using Kora.

---

## Step 1: Initialize a New Clean Architecture Project

You can either create a new directory or initialize in your current directory:

```bash
# Option A: In a new folder
kora create bookstore-service
cd bookstore-service

# Option B: In an existing folder
mkdir bookstore-service && cd bookstore-service
kora init
```

### Generated Project Layout:
```text
bookstore-service/
├── kora.toml               # Package metadata and HTTP server config
├── README.md               # Project documentation
├── .gitignore              # Git ignore for artifacts (.kora-build/, etc.)
└── src/
    ├── main.kora           # Application entrypoint & HTTP route dispatch
    ├── enums/              # Pure domain enumerations
    │   ├── http_status.kora
    │   └── user_role.kora
    ├── models/             # Global DTO envelopes
    │   └── response_envelope.kora
    ├── shared/services/    # Cross-cutting infrastructure services
    │   └── logger_service.kora
    └── features/users/     # Default modular domain feature
        ├── models/         # Entities and DTOs
        ├── interfaces/     # Service contracts
        ├── events/         # Realtime WebSocket event schemas
        ├── users.service.kora
        └── users.controller.kora
```

---

## Step 2: Generate a New Domain Feature

Use the `kora g feature` generator to scaffold an entire CRUD module with zero boilerplate:

```bash
kora g feature books
```

This immediately creates `src/features/books/`:
- `models/book_entity.kora` (Database schema model with decorators)
- `models/book_dto.kora` (`CreateBookDto`, `UpdateBookDto`, `BookResponseDto`)
- `interfaces/book_service_interface.kora` (Interface contract)
- `events/book_events.kora` (Realtime events: `BookCreatedEvent`, `BookDeletedEvent`)
- `books.service.kora` (Domain business logic)
- `books.controller.kora` (HTTP controller layer)

---

## Step 3: Check & Verify Code

Before building, verify that your code adheres to static type checking and Clean Architecture rules:

```bash
kora check src/main.kora
```

Output:
```text
✓ Checked `src/main.kora` — 0 errors (frontend & ownership verified).
```

---

## Step 4: Run the Application

Compile and execute your backend service natively:

```bash
kora run src/main.kora
```

Output:
```text
Compiling and running `src/main.kora` ...
    Finished `release` profile [optimized] target(s) in 0.18s
Starting Kora server on http://127.0.0.1:7582
⚡ HTTP Route registered: GET    /api/users
⚡ HTTP Route registered: GET    /api/users/:id
⚡ HTTP Route registered: POST   /api/users
```

---

## Step 5: Live Hot-Reloading with `kora watch`

During development, use `kora watch` to automatically re-verify and re-launch your service the moment you save any `.kora` file:

```bash
kora watch
```

Any changes to models, routes, or services will trigger sub-second reloads without manual restarts.

