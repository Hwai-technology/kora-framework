# Example 12: Event-Driven Microservice Architecture

A high-throughput event store and messaging service built with Kora Clean Architecture:

- **Entity Model**: `@entity` [domain_event.entity.kora](models/domain_event.entity.kora) with zero-cost table auto-migration.
- **Repository Pattern**: [event_store.repository.kora](repositories/event_store.repository.kora) using parameterized queries.
- **Service Layer**: [event_publisher.service.kora](services/event_publisher.service.kora) handling asynchronous domain event emission.
- **HTTP Routes**: [order_events.routes.kora](routes/order_events.routes.kora) mapping `/events/publish` and `/events/stream`.
- **Database Seeder**: [event_store.seeder.kora](database/event_store.seeder.kora) initializing baseline event audit logs.
