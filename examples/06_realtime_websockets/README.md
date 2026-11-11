# Example 06: Realtime WebSockets & Event Streaming

## Overview
This example demonstrates low-latency realtime communication in Kora:
- **WebSocket Broadcast Channels**: Pub/sub topic broadcasting across connected clients.
- **Presence Tracking**: Measuring active subscribers per channel/room.
- **Event Models**: Type-safe declarative message structures.
- **Clean Architecture Segregation**: Message models in `models/`, interface contracts in `interfaces/`, and business hub in `services/`.

## Directory Structure
```
06_realtime_websockets/
├── README.md
├── main.kora                   # Realtime routes and WebSocket broadcast triggers
├── interfaces/
│   └── chat_hub.kora           # Realtime broadcast hub interface contract
├── models/
│   └── chat_models.kora        # Declarative message, presence, and request schemas
└── services/
    └── chat_hub_service.kora   # Broadcast hub and presence implementation
```

## Running the Example
```bash
# Type check the realtime hub
kora check main.kora

# Start the realtime server
kora run main.kora

# Query room presence
curl http://localhost:7582/chat/presence/general

# Broadcast a message to the room
curl -X POST http://localhost:7582/chat/message -d "Hello world over WebSockets!"
```
