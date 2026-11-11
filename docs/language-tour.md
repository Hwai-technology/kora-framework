# Kora Language Tour (`v0.1.0`)

Kora combines clean, expressive syntax with **strong static typing** and **automatic type inference**.

## 1. Bindings & Type Inference

Use `let` for immutable bindings and `var` for mutable bindings. Types are inferred automatically or annotated explicitly:

```kora
let name = "Ahmet"          // Inferred: String
let age = 25                // Inferred: Int
let active = true           // Inferred: Bool
var requestsCount: Int = 0
requestsCount = requestsCount + 1
```

## 2. Value Structs & Domain Models

Use `struct` for value objects and `model` for database-backed entities:

```kora
struct Coordinates {
    lat: Float
    lng: Float
}

model User {
    id: Int
    name: String
    email: String
    active: Bool
}

let admin = User {
    id: 1,
    name: "Ahmet",
    email: "ahmet@kora.dev",
    active: true,
}
```

## 3. Algebraic Enums, `Option<T>`, and `Result<T, E>`

Kora eliminates `null` exceptions using `Option<T>` (`Some(T)`, `None`) and handles recoverable errors explicitly via `Result<T, E>` (`Ok(T)`, `Err(E)`):

```kora
enum PaymentState {
    Pending
    Authorized(Int)
    Failed(String)
}

fn describePayment(state: PaymentState) -> String {
    return match state {
        PaymentState.Pending => "Pending confirmation",
        PaymentState.Authorized(amount) => "Authorized payment",
        PaymentState.Failed(reason) => reason,
    }
}
```

## 4. Interfaces & Compile-Time Dependency Injection (`class`)

Service classes declare dependencies in `init(...)` and can implement one or more `interface` contracts without fragile class inheritance:

```kora
interface UserStore {
    fn find(id: Int) -> Option<User>
}

class UserService implements UserStore {
    db: Database

    init(db: Database) {
        this.db = db
    }

    fn find(id: Int) -> Option<User> {
        if id <= 0 {
            return None
        }
        return Some(User {
            id: id,
            name: "Ahmet",
            email: "ahmet@kora.dev",
            active: true,
        })
    }
}
```

## 5. Zero-Config Backend Routes

Top-level route declarations automatically type-check handler closures and injected service classes:

```kora
let app = run()

get("/users/{id}", (id: Int, users: UserService) => {
    return users.find(id)
})
```

