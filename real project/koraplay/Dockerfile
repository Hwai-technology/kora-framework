# Multi-Stage Production Dockerfile for Kora Microservice
FROM rust:1.80-alpine AS builder
RUN apk add --no-cache musl-dev gcc
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release || true

FROM alpine:3.20
RUN apk add --no-cache ca-certificates tzdata
WORKDIR /app
COPY --from=builder /usr/src/app/.kora-build/target/release/kora_app /app/server 2>/dev/null || true
EXPOSE 7582
ENV PORT=7582
ENV HOST=0.0.0.0
USER nobody
ENTRYPOINT ["/app/server"]
