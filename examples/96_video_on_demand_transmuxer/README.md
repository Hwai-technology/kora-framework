# Example 96: Video on Demand (VOD) HLS/DASH Transmuxer

MPEG-TS to fMP4 chunker, HLS m3u8 master playlist generation, multi-bitrate ladder (1080p, 720p, 480p), and DRM token verification.

---

## 1. Clean Architecture File Layout (26 .kora Files)

```
examples/96_video_on_demand_transmuxer/
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
kora check examples/96_video_on_demand_transmuxer/main.kora
kora test examples/96_video_on_demand_transmuxer/tests/
```
