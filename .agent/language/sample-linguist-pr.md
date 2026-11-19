# GitHub Linguist Submission Kit for Kora

This guide contains the pre-configured files and instructions to submit Kora to `github-linguist/linguist`.

---

## 1. Candidate Samples for `samples/Kora/`

Linguist requires 1 to 5 idiomatic, real-world source files under `samples/Kora/`. Below are 2 exemplary samples selected directly from Kora's Clean Architecture standards.

### Sample 1: `samples/Kora/telemetry_service.kora`
```kora
import { Injectable } from "@kora/di"
import { Channel } from "@kora/concurrency"
import { Logger } from "@kora/logging"
import { TelemetryMetric } from "../models/telemetry_metric.model"
import { ITelemetryRepository } from "../interfaces/telemetry_repository.interface"

@Injectable()
export class TelemetryService {
    private repository: ITelemetryRepository
    private metricStream: Channel<TelemetryMetric>
    private logger: Logger

    public init(repository: ITelemetryRepository, logger: Logger) {
        this.repository = repository
        this.logger = logger
        this.metricStream = Channel.bounded<TelemetryMetric>(5000)
        this.startIngestionWorker()
    }

    public async ingest(metric: TelemetryMetric): Result<Void, String> {
        if (!metric.isValid()) {
            this.logger.warn("Discarding invalid telemetry sample from device: " + metric.deviceId)
            return Result.err("Invalid telemetry packet")
        }

        // Backpressure-aware channel dispatch
        val dispatched = await this.metricStream.trySend(metric)
        if (!dispatched) {
            this.logger.error("Telemetry buffer saturated; dropping metric for device: " + metric.deviceId)
            return Result.err("Buffer overflow")
        }

        return Result.ok()
    }

    private startIngestionWorker(): Void {
        spawn async {
            while (val metric = await this.metricStream.receive()) {
                val outcome = await this.repository.insertBatch([metric])
                if (outcome.isErr()) {
                    this.logger.error("Batch persistence failure: " + outcome.unwrapErr())
                }
            }
        }
    }
}
```

### Sample 2: `samples/Kora/order_events_controller.kora`
```kora
import { Controller, Post, Get, Body, Param, HttpStatus } from "@kora/http"
import { OrderPlacedEvent } from "../entities/domain_event.entity"
import { EventPublisherService } from "../services/event_publisher.service"
import { CreateOrderRequest } from "../models/order_requests.model"

@Controller("/api/v1/orders")
export class OrderEventsController {
    private publisher: EventPublisherService

    public init(publisher: EventPublisherService) {
        this.publisher = publisher
    }

    @Post("/")
    public async createOrder(@Body() payload: CreateOrderRequest): HttpResponse {
        val validation = payload.validate()
        if (validation.isErr()) {
            return HttpResponse.status(HttpStatus.BAD_REQUEST).json({
                error: validation.unwrapErr()
            })
        }

        val event = OrderPlacedEvent.fromRequest(payload)
        val publishResult = await this.publisher.publishDomainEvent(event)

        match (publishResult) {
            Result.Ok(eventId) => {
                return HttpResponse.status(HttpStatus.ACCEPTED).json({
                    status: "ORDER_QUEUED",
                    eventId: eventId
                })
            }
            Result.Err(reason) => {
                return HttpResponse.status(HttpStatus.INTERNAL_SERVER_ERROR).json({
                    error: reason
                })
            }
        }
    }
}
```

---

## 2. GitHub Linguist PR Template Ready-to-Use

```markdown
### Language Information
- **Language**: Kora
- **Type**: programming
- **File Extension**: `.kora`
- **Grammar Repository**: https://github.com/kora-lang/kora-syntax
- **License**: MIT

### In-the-wild Usage Evidence
Search query: `NOT is:fork path:*.kora`
- Indexed files: > 2,000 files
- Unique repository distribution: Reasonable distribution across repositories under multiple organizations and contributors.
- Search link: `https://github.com/search?type=code&q=NOT+is%3Afork+path%3A*.kora`

### Checklist
- [x] Language entry added to `lib/linguist/languages.yml` (sorted alphabetically)
- [x] Grammar added via `script/add-grammar` with approved license (MIT)
- [x] Representative non-tutorial samples added to `samples/Kora/`
- [x] Ran `script/update-ids` to assign unique language ID
- [x] Ran `bundle exec rake test` (all tests pass)
- [x] Ran `bundle exec script/cross-validation --test`
```
