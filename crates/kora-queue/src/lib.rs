//! Asynchronous Background Job Queues, Workers, and Dead Letter Queues (DLQ) for Kora.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub payload: String,
    pub attempts: usize,
    pub max_attempts: usize,
}

#[derive(Clone, Default)]
pub struct QueueEngine {
    pending: Arc<Mutex<VecDeque<Job>>>,
    dead_letter: Arc<Mutex<Vec<Job>>>,
}

impl QueueEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn push(&self, name: &str, payload: &str, max_attempts: usize) -> String {
        let id = format!(
            "job_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );
        let job = Job {
            id: id.clone(),
            name: name.to_string(),
            payload: payload.to_string(),
            attempts: 0,
            max_attempts,
        };
        self.pending.lock().await.push_back(job);
        id
    }

    pub async fn pop(&self) -> Option<Job> {
        self.pending.lock().await.pop_front()
    }

    pub async fn fail_job(&self, mut job: Job) {
        job.attempts += 1;
        if job.attempts >= job.max_attempts {
            self.dead_letter.lock().await.push(job);
        } else {
            self.pending.lock().await.push_back(job);
        }
    }

    pub async fn dead_letter_count(&self) -> usize {
        self.dead_letter.lock().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn pushes_retries_and_moves_to_dlq() {
        let q = QueueEngine::new();
        q.push("send_email", r#"{"to":"user@kora.dev"}"#, 2).await;

        let job = q.pop().await.expect("job exists");
        assert_eq!(job.name, "send_email");

        // Fail once -> retry
        q.fail_job(job).await;
        assert_eq!(q.dead_letter_count().await, 0);

        // Fail second time -> DLQ
        let job2 = q.pop().await.expect("job retried");
        q.fail_job(job2).await;
        assert_eq!(q.dead_letter_count().await, 1);
    }
}
