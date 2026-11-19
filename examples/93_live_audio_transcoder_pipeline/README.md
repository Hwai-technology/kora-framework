# Example 93: Live Audio Transcoding & Packet Chunking Pipeline

PCM to Opus audio frame chunking, adaptive jitter buffer manager, packet loss concealment (PLC), and audio level metering.

---

## 1. Clean Architecture File Layout (25 .kora Files)

```
examples/93_live_audio_transcoder_pipeline/
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
kora check examples/93_live_audio_transcoder_pipeline/main.kora
kora test examples/93_live_audio_transcoder_pipeline/tests/
```
