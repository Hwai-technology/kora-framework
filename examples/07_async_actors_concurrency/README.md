# Example 07: Async Actors & Channel Concurrency

## Overview
This example demonstrates Kora's Go-style concurrency and actor-based message passing:
- **Actor State Isolation**: Dedicated mailboxes avoiding mutex locks or race conditions.
- **MPSC Channels**: Multi-producer, single-consumer typed queues for background task processing.
- **Task Scheduling**: Non-blocking asynchronous task execution with throughput tracking.
- **Clean Architecture Segregation**: Task models in `models/`, worker interface in `interfaces/`, and actor service in `services/`.

## Directory Structure
```
07_async_actors_concurrency/
├── README.md
├── main.kora                       # REST endpoints dispatching messages to actors
├── interfaces/
│   └── task_worker_interface.kora  # Task worker interface contract
├── models/
│   └── task_models.kora            # Declarative job and actor stats models
└── services/
    └── counter_actor_service.kora  # Actor service managing task processing & metrics
```

## Running the Example
```bash
# Type check the concurrency engine
kora check main.kora

# Launch the actor runtime
kora run main.kora

# Check actor metrics
curl http://localhost:7582/actors/stats

# Submit an asynchronous job to the actor queue
curl -X POST http://localhost:7582/actors/jobs -d "GenerateAnalyticsReport"
```
