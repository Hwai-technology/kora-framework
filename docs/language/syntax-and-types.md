# Kora Language Syntax & Type System

## Core Primitives
- `Int`: 64-bit signed integer.
- `Float`: 64-bit IEEE floating-point number.
- `String`: UTF-8 encoded string with `.len()`, `.contains()`, `.startsWith()`, `.endsWith()`, and `.trim()`.
- `Bool`: `true` or `false`.
- `Unit`: Equivalent to `()`.

## Variables & Mutability
- `let`: Declares an immutable binding.
- `var`: Declares a mutable binding.

```kora
let maxConnections = 100
var activeSessions = 0
activeSessions = activeSessions + 1
```

## Models (Data Entities & DTOs)
Models represent pure data structures and API schemas. They support declarative field attributes:

```kora
model Account {
    @unique
    @required
    id: Int

    @unique
    @index
    @required
    @email
    email: String

    @min(0)
    balance: Int

    @default(true)
    isActive: Bool
}
```

## Classes & Auto-Constructors
Classes represent services and controllers with logic and state. Parameters declared in `init(...)` are promoted to fields automatically:

```kora
class AccountService {
    repo: AccountRepository

    init(repo: AccountRepository) {
        this.repo = repo
    }

    fn getAccount(email: String) -> Option<Account> {
        if email == "" {
            return null
        }
        return this.repo.findByEmail(email)
    }
}
```

## Null Safety
- Only `null` is allowed to represent missing values (`Option<T>::None`).
- The keyword `undefined` is strictly forbidden and produces error `E2005`.

