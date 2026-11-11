# Pattern Matching & Enums in Kora

Kora features expressive, compile-time exhaustive **Pattern Matching** with algebraic data types (`enum`).

---

## 1. Pure Enums

Standard enumerations represent a fixed set of states:

```kora
pub enum HttpStatus {
    Ok,
    Created,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalError,
}

pub enum UserRole {
    Admin,
    Member,
    Guest,
}
```

---

## 2. Enums with Payloads (Algebraic Data Types)

Variants can carry associated typed payloads:

```kora
enum PaymentStatus {
    Pending,
    Processing(Int),
    Completed(String),
    Failed(String),
}
```

---

## 3. The `match` Expression

Pattern matching in Kora is an expression that yields a value:

```kora
fn describePayment(status: PaymentStatus) -> String {
    return match status {
        PaymentStatus.Pending => "Waiting for customer",
        PaymentStatus.Processing(attempt) => "Processing attempt in progress",
        PaymentStatus.Completed(txId) => "Payment settled successfully",
        PaymentStatus.Failed(reason) => reason,
    }
}
```

### Matching Option Types:
`Option<T>` is natively unwrapped via `match`:

```kora
fn greetUser(optionalName: Option<String>) -> String {
    return match optionalName {
        Some(name) => name,
        None => "Guest",
    }
}
```

### Matching Result Types:
`Result<T, E>` provides functional error handling without exceptions:

```kora
fn handleResult(res: Result<Int, String>) -> Int {
    return match res {
        Ok(value) => value,
        Err(err) => {
            print(err)
            return 0
        },
    }
}
```

### Wildcard Matches (`_`):
Use the underscore `_` for fallback branches:

```kora
fn isPrivileged(role: UserRole) -> Bool {
    return match role {
        UserRole.Admin => true,
        _ => false,
    }
}
```

