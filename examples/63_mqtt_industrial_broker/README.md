# Example 63: MQTT Industrial Telemetry Broker & Topic Tree

QoS 0/1/2 message delivery guarantees, hierarchical topic tree wildcards (+ and #), and client keepalive heartbeats.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/63_mqtt_industrial_broker/
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
kora check examples/63_mqtt_industrial_broker/main.kora
kora test examples/63_mqtt_industrial_broker/tests/
```
