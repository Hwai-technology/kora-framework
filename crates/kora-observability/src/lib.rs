//! Structured JSON Logging, Metrics Registry, and OpenTelemetry Health Checks for Kora.

use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Serialize)]
pub struct LogRecord<'a> {
    pub level: &'static str,
    pub message: &'a str,
    pub target: &'a str,
    pub timestamp_ms: u128,
}

pub struct Logger;

impl Logger {
    pub fn info(target: &str, message: &str) -> String {
        let rec = LogRecord {
            level: "INFO",
            message,
            target,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        };
        serde_json::to_string(&rec).unwrap()
    }

    pub fn error(target: &str, message: &str) -> String {
        let rec = LogRecord {
            level: "ERROR",
            message,
            target,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        };
        serde_json::to_string(&rec).unwrap()
    }
}

/// Metrics Registry for HTTP requests & Database queries
#[derive(Default)]
pub struct MetricsRegistry {
    counters: Arc<parking_lot_sim::SimMutex<HashMap<String, AtomicU64>>>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&self, name: &str) {
        let mut map = self.counters.lock();
        let counter = map
            .entry(name.to_string())
            .or_insert_with(|| AtomicU64::new(0));
        counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get(&self, name: &str) -> u64 {
        let map = self.counters.lock();
        map.get(name)
            .map(|c| c.load(Ordering::Relaxed))
            .unwrap_or(0)
    }
}

mod parking_lot_sim {
    use std::sync::{Mutex, MutexGuard};
    #[derive(Default)]
    pub struct SimMutex<T>(Mutex<T>);
    impl<T> SimMutex<T> {
        pub fn lock(&self) -> MutexGuard<T> {
            self.0.lock().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_structured_json_record() {
        let log = Logger::info("http", "request received");
        assert!(log.contains("\"level\":\"INFO\""));
        assert!(log.contains("\"message\":\"request received\""));
    }

    #[test]
    fn tracks_atomic_metrics() {
        let metrics = MetricsRegistry::new();
        metrics.inc("http_requests_total");
        metrics.inc("http_requests_total");
        assert_eq!(metrics.get("http_requests_total"), 2);
    }
}
