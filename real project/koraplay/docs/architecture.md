# KoraPlay — Architecture & Technical Design

## 1. Executive Overview

**KoraPlay** is a production-grade, server-authoritative, real-time interactive quiz and learning platform built natively on the **Kora Programming Language & Framework (v0.2.0)**.

The primary objective is twofold:
1. Provide a competitive, scalable alternative to platforms like Kahoot!.
2. Serve as the definitive real-world proof-of-concept for Kora, exercising its compiler, type system, ownership analysis, dependency injection, and native real-time actors.

---

## 2. Architectural Paradigm: Modular Clean Monolith

To maximize development velocity while enforcing strict separation of concerns, KoraPlay is organized as a **Modular Monolith** adhering to Kora's Clean Architecture standards:

```text
                               +-----------------------------+
                               |     Client Applications     |
                               |  (Web / Mobile / Screen)    |
                               +--------------+--------------+
                                              |
                          HTTP REST API (v1)  |  WebSocket (Realtime Hub)
                                              v
+---------------------------------------------------------------------------------------+
|                                    KORAPLAY ENGINE                                    |
|                                                                                       |
|   [Presentation Layer]                                                                |
|     - Controllers (REST endpoints)                                                    |
|     - Realtime Gateway (WebSocket connection management, heartbeat, room pub/sub)     |
|                                                                                       |
|   [Domain & Application Layer]                                                        |
|     +--------------------+   +-------------------+   +----------------------------+   |
|     |  Identity / Auth   |   |  Quiz Management  |   |  Live Game Engine          |   |
|     |  - Argon2id hash   |   |  - Questions      |   |  - Room State Machine      |   |
|     |  - JWT tokens      |   |  - Media/Categories|  |  - Authoritative Scoring   |   |
|     |  - RBAC            |   |  - Versioning     |   |  - Realtime Leaderboard    |   |
|     +--------------------+   +-------------------+   +----------------------------+   |
|                                                                                       |
|   [Infrastructure Layer]                                                              |
|     - Kora ORM Repositories (PostgreSQL persistence)                                  |
|     - Redis Session & Ephemeral State Hub                                            |
|     - Observability & Metrics Logger                                                  |
+---------------------------------------------------------------------------------------+
```

---

## 3. Core Modules & Boundaries

| Module | Boundary Responsibilities |
|---|---|
| **Identity & Access** | User accounts, teacher/student roles, Argon2id credentials, JWT token lifecycle. |
| **Quizzes & Questions** | Quiz authoring, question types (single/multi-choice, true/false, ordering), immutable published versions. |
| **Game Sessions** | PIN generation, room lifecycle (`CREATED` -> `WAITING` -> `QUESTION_ACTIVE` -> `FINISHED`), participant connections. |
| **Scoring & Engine** | Deterministic server calculation (Base points + time decay bonuses + streaks), anti-replay protection. |
| **Realtime Hub** | WebSocket envelope routing, low-latency broadcasts, reconnect tolerance. |

---

## 4. Enforcement via Kora Compiler

The Kora toolchain (`kora check`, `kora test`) enforces layer purity:
- **Enums** must reside strictly under `enums/`.
- **Models & DTOs** reside in `models/`.
- **Interfaces & Contracts** reside in `interfaces/`.
- **Services & Controllers** implement explicit contracts with compile-time dependency injection.
