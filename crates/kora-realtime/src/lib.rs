//! Realtime WebSockets, Server-Sent Events (SSE), and Pub/Sub Hub for Kora.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

#[derive(Clone, Default)]
pub struct RealtimeHub {
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
}

impl RealtimeHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn subscribe(&self, topic: &str) -> broadcast::Receiver<String> {
        let mut channels = self.channels.write().await;
        let sender = channels
            .entry(topic.to_string())
            .or_insert_with(|| broadcast::channel(128).0);
        sender.subscribe()
    }

    pub async fn publish(&self, topic: &str, message: &str) -> Result<usize, String> {
        let channels = self.channels.read().await;
        if let Some(sender) = channels.get(topic) {
            sender
                .send(message.to_string())
                .map_err(|e| format!("publish failed: {e}"))
        } else {
            Ok(0)
        }
    }
}

pub struct SseFrame {
    pub event: Option<String>,
    pub data: String,
}

impl SseFrame {
    pub fn format(&self) -> String {
        let mut out = String::new();
        if let Some(ev) = &self.event {
            out.push_str(&format!("event: {ev}\n"));
        }
        out.push_str(&format!("data: {}\n\n", self.data));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn publishes_and_receives_realtime_events() {
        let hub = RealtimeHub::new();
        let mut rx = hub.subscribe("orders:new").await;

        hub.publish("orders:new", "{\"orderId\": 42}")
            .await
            .unwrap();
        let msg = rx.recv().await.unwrap();
        assert_eq!(msg, "{\"orderId\": 42}");
    }
}
