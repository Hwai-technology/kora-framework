# Models & Classes in Kora

Kora enforces a clear architectural dichotomy between **Models** (pure data schemas) and **Classes** (stateful behaviors with dependency injection).

> ⚠️ **Design Principle**: Kora is strictly **struct-free**. All data structures are defined as `model`, and all behavioral services are defined as `class`.

---

## 1. Models (`model`)

A `model` represents immutable, serializable data. Models cannot contain stateful logic or constructors. They are used for:
- Database entities (`@entity`)
- API Request and Response DTOs
- Domain Events
- Value Objects

### Data Transfer Object (DTO) Example:
```kora
model CreateUserDto {
    name: String,
    email: String,
}

model UserResponseDto {
    id: Int,
    name: String,
    email: String,
    active: Bool,
}
```

### Instantiating Models:
```kora
let user = UserResponseDto {
    id: 1,
    name: "Ahmet",
    email: "ahmet@kora.dev",
    active: true,
}
```

### Database Entity Models & Decorators:
Entities represent persistent database tables:

```kora
@entity
@table("customers")
model CustomerEntity {
    @primary
    @autoIncrement
    id: Int,

    @required
    @index
    name: String,

    @unique
    @email
    email: String,

    @default(true)
    active: Bool,
}
```

Available decorators:
- `@entity`: Marks a model as an ORM-managed database table.
- `@table("name")`: Specifies the underlying SQL table name.
- `@primary`: Marks the primary key field.
- `@autoIncrement`: Auto-incrementing numeric field.
- `@unique`: Enforces unique constraint in schema migrations.
- `@index`: Creates an index for query performance.
- `@required`: Mandatory non-null field.
- `@email`: Validates email syntax.
- `@default(val)`: Sets SQL default value.

---

## 2. Classes (`class`)

A `class` encapsulates state and methods. Classes participate in Kora's **Compile-Time Dependency Injection** system and represent:
- Controllers (`UsersController`)
- Domain Services (`UsersService`)
- Repositories (`UserRepository`)
- Cross-cutting utilities (`LoggerService`)

### Class Syntax & Constructor Injection:
```kora
import { UsersService } from "./users.service"
import { CreateUserDto, UserResponseDto } from "./models/user_dto"

class UsersController {
    service: UsersService,

    init(service: UsersService) {
        this.service = service
    }

    fn list() -> List<UserResponseDto> {
        return this.service.get_all_users()
    }

    fn create(dto: CreateUserDto) -> UserResponseDto {
        return this.service.create_user(dto)
    }
}
```

### Implementing Interfaces:
Classes can implement interfaces for contract enforcement and testability:

```kora
interface UserServiceInterface {
    fn create_user(dto: CreateUserDto) -> UserResponseDto
    fn delete_user(id: Int) -> Bool
}

class UsersService implements UserServiceInterface {
    fn create_user(dto: CreateUserDto) -> UserResponseDto {
        return UserResponseDto {
            id: 100,
            name: dto.name,
            email: dto.email,
            active: true,
        }
    }

    fn delete_user(id: Int) -> Bool {
        return id > 0
    }
}
```

---

## 3. Comparison: `model` vs `class`

| Feature | `model` | `class` |
|:---|:---|:---|
| Primary Purpose | Pure data & schema definition | Behavior, state & business logic |
| Mutable methods | ❌ No | ✅ Yes (`fn`) |
| Constructor (`init`) | ❌ No (literal instantiation) | ✅ Yes (`init(...)`) |
| Dependency Injection | ❌ No | ✅ Yes (Resolved at compile-time) |
| JSON Serialization | ✅ Automatic zero-cost | ❌ Not serialized directly |
| Interface implementation | ❌ No | ✅ Yes (`implements Trait`) |
| Decorators supported | ✅ `@entity`, `@table`, etc. | ❌ Architectural linting |

