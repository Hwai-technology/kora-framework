# Example 58: Distributed Shopping Cart Session Actor

Stateful session actors, persistent shopping cart synchronization across devices, and soft inventory reservation.

---

## 1. Clean Architecture File Layout (24 .kora Files)

```
examples/58_shopping_cart_session_actor/
├── enums/                                # Domain enums
├── events/                               # Domain events
├── models/                               # DTOs & payloads
├── entities/                             # Database tables
├── interfaces/                           # Contracts
├── repositories/                         # Data access
├── services/                             # Domain logic
├── controllers/                          # Presentation endpoints
├── routes/                               # Route dispatcher
├── main.kora                             # Bootstrap
└── tests/                                # Unit & load test suites
```

---

## 2. CLI Execution & Verification

```bash
kora check examples/58_shopping_cart_session_actor/main.kora
kora test examples/58_shopping_cart_session_actor/tests/
```
