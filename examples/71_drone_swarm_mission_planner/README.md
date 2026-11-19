# Example 71: Autonomous Drone Swarm Formation Coordinator

Multi-agent flocking algorithms (Reynolds separation, alignment, cohesion), airspace geo-corridor reservation, and battery relay handover.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/71_drone_swarm_mission_planner/
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
kora check examples/71_drone_swarm_mission_planner/main.kora
kora test examples/71_drone_swarm_mission_planner/tests/
```
