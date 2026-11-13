# Example 14: Realtime IoT Telemetry & Metrics Processing Engine

A high-performance IoT time-series and sensor telemetry ingestion backend:

- **Entity Auto-Migration**: `@entity` [iot_metric.entity.kora](models/iot_metric.entity.kora) -> `iot_metrics` table.
- **Repository**: [telemetry.repository.kora](repositories/telemetry.repository.kora) executing indexed metrics inserts and windowed queries.
- **Service**: [telemetry.service.kora](services/telemetry.service.kora) sensor ingestion and sanity validation.
- **Routes**: [telemetry.routes.kora](routes/telemetry.routes.kora) `/api/telemetry/ingest` and `/api/telemetry/metrics`.
- **Database Seeder**: [telemetry.seeder.kora](database/telemetry.seeder.kora) for baseline hardware simulation.
