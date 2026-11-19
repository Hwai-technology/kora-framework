# Example 65: Fleet Telematics & Polygonal Geofence Monitor

Streaming GPS ingestion, ray-casting point-in-polygon geofence breach detection, and harsh braking accelerometer analytics.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/65_fleet_telematics_geofencing/
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
kora check examples/65_fleet_telematics_geofencing/main.kora
kora test examples/65_fleet_telematics_geofencing/tests/
```
