//! Asynchronous Runtime, Structured Task Orchestrator, and Channels for Kora.
//! Built atop Tokio's multi-threaded work-stealing scheduler with CPU thread pool isolation.

use std::future::Future;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::{broadcast, mpsc};

pub struct KoraRuntime {
    inner: Runtime,
}

impl KoraRuntime {
    pub fn new() -> Result<Self, String> {
        let inner = Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("failed to initialize tokio runtime: {e}"))?;
        Ok(Self { inner })
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.inner.block_on(future)
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.inner.spawn(future);
    }

    pub async fn spawn_blocking<F, R>(f: F) -> Result<R, String>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        tokio::task::spawn_blocking(f)
            .await
            .map_err(|e| format!("blocking task panicked: {e}"))
    }
}

/// MPSC Bounded Typed Message Channel for Kora
pub struct Channel<T> {
    pub tx: mpsc::Sender<T>,
    pub rx: mpsc::Receiver<T>,
}

impl<T> Channel<T> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(capacity: usize) -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
        mpsc::channel(capacity)
    }

    pub fn pair(capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel(capacity);
        Self { tx, rx }
    }
}

/// Broadcast Fan-Out Channel for Realtime WebSockets and SSE
pub struct BroadcastChannel<T> {
    tx: broadcast::Sender<T>,
}

impl<T: Clone> BroadcastChannel<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.tx.subscribe()
    }

    pub fn send(&self, msg: T) -> Result<usize, String> {
        self.tx
            .send(msg)
            .map_err(|e| format!("broadcast error: {e}"))
    }
}
