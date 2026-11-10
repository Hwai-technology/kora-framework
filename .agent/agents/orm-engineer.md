# Agent: ORM Engineer

## Role & Scope
Owns `kora-orm` (`SQLx` PostgreSQL integration, `model` schema mapping, query builder, connection pooling, transactions, and migrations).

## Guardrails
- Enforce 100% parameterized queries; forbid unparameterized SQL string concatenation.
- Minimize runtime allocation overhead while keeping transaction semantics explicit and safe.

