# Example 67: Autonomous Rover Navigation & Obstacle Avoidance

Waypoint path tracking, 2D LiDAR ray distance map, dynamic window approach (DWA) obstacle avoidance, and battery reserve safety.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/67_autonomous_rover_command/
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
kora check examples/67_autonomous_rover_command/main.kora
kora test examples/67_autonomous_rover_command/tests/
```
