# Realtime & WebSocket Events in Kora

Modern cloud backends demand instant, bi-directional event distribution. Kora provides native **Realtime WebSocket Event Emission** with transactional guarantees.

---

## 1. Domain Event Definition

Every domain mutation (Create, Update, Delete, Status change) must define explicit event payloads inside `src/features/<feature>/events/`:

```kora
// src/features/users/events/user_events.kora

model UserCreatedEvent {
    user_id: Int,
    email: String,
    timestamp: Int,
}

model UserStatusChangedEvent {
    user_id: Int,
    old_status: String,
    new_status: String,
    timestamp: Int,
}

model UserDeletedEvent {
    user_id: Int,
    timestamp: Int,
}
```

---

## 2. Emitting Realtime Events

Events are published over WebSocket channels **only after** the underlying database transaction has successfully committed:

```kora
import { UserCreatedEvent } from "./events/user_events"

class UsersService {
    fn create_user(dto: CreateUserDto) -> UserResponseDto {
        // 1. Commit database transaction
        let user = UserResponseDto {
            id: 101,
            name: dto.name,
            email: dto.email,
            active: true,
        }

        // 2. Publish Realtime WebSocket event
        emit("users:created", UserCreatedEvent {
            user_id: user.id,
            email: user.email,
            timestamp: 1718000000,
        })

        return user
    }
}
```

---

## 3. Frontend & Client Synchronization

Clients (Angular, React, Vue, Mobile) subscribe to topics to reflect state changes without polling:

```typescript
// Frontend client subscriber (e.g. Angular service)
realtimeService.on<UserCreatedEvent>('users:created', (event) => {
    console.log(`New user registered: ${event.email}`);
    this.refreshUserList();
});
```

---

## 4. Key Guarantees

1. **Transactional Delivery**: Events are never fired before the database write succeeds.
2. **Type Safety**: Event payloads are strictly typed `model` definitions.
3. **Low Latency**: Emitted via native non-blocking asynchronous WebSocket channels powered by Tokio and Hyper.

