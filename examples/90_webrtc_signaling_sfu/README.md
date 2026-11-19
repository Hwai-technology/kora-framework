# Example 90: WebRTC SFU Media Router & SDP Signaling

SDP offer/answer exchange broker, ICE candidate gathering coordinator, Selective Forwarding Unit (SFU) track subscription.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/90_webrtc_signaling_sfu/
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
kora check examples/90_webrtc_signaling_sfu/main.kora
kora test examples/90_webrtc_signaling_sfu/tests/
```
