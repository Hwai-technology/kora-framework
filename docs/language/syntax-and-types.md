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

## Models: Database Entities vs. Data Transfer DTOs
Kora strictly distinguishes between **Database Entity Models** (persisted tables) and **Data Transfer DTO Models** (API requests, responses, inputs):

### Database Entity Models
Decorated with `@entity` and `@table("table_name")` with primary keys and database column indexes:

```kora
@entity
@table("users")
model UserEntity {
    @primary
    @autoIncrement
    @unique
    @required
    id: Int

    @unique
    @index
    @required
    @email
    email: String

    @required
    fullName: String

    @default("customer")
    role: String

    @min(0)
    balance: Int

    @default(true)
    isActive: Bool
}
```

### Data Transfer DTO Models
Used for HTTP payloads, request validations, and response envelopes without database table bindings:

```kora
model UserRegisterInput {
    @required
    @email
    email: String

    @required
    fullName: String

    @min(0)
    initialBalance: Int
}

model UserProfileDto {
    id: Int
    email: String
    fullName: String
    role: String
}
```

When running `kora types <file.kora>`, the type checker explicitly categorizes and reports:
```text
  [Database Entity Model] UserEntity -> table: `users` (6 field(s))
  [Data Transfer DTO Model] UserRegisterInput (3 field(s))
  [Data Transfer DTO Model] UserProfileDto (4 field(s))
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

