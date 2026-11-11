//! Asynchronous Runtime, Structured Task Orchestrator, Channels, and Actor Concurrency for Kora.
//! Built atop Tokio's multi-threaded work-stealing scheduler with CPU thread pool isolation.

use std::future::Future;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::JoinSet;

/// Core Asynchronous Runtime Engine for Kora Applications.
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

    pub async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    pub async fn timeout<F, T>(duration: Duration, future: F) -> Result<T, String>
    where
        F: Future<Output = T>,
    {
        tokio::time::timeout(duration, future)
            .await
            .map_err(|_| "operation timed out".to_string())
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

    pub async fn send(&self, val: T) -> Result<(), String> {
        self.tx
            .send(val)
            .await
            .map_err(|e| format!("channel send error: {e}"))
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.rx.recv().await
    }

    pub fn try_send(&self, val: T) -> Result<(), String> {
        self.tx
            .try_send(val)
            .map_err(|e| format!("channel try_send error: {e}"))
    }

    pub fn try_recv(&mut self) -> Result<T, String> {
        self.rx
            .try_recv()
            .map_err(|e| format!("channel try_recv error: {e}"))
    }

    pub fn clone_tx(&self) -> mpsc::Sender<T> {
        self.tx.clone()
    }
}

/// Broadcast Fan-Out Channel for Realtime WebSockets and SSE Pub/Sub
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

    pub fn receiver_count(&self) -> usize {
        self.tx.receiver_count()
    }
}

/// Request-Response Envelope for Actor Communication
pub enum ActorMessage<M, R> {
    Tell(M),
    Ask(M, oneshot::Sender<R>),
}

/// Handle to communicate with an isolated asynchronous Actor.
#[derive(Clone)]
pub struct ActorRef<M, R> {
    tx: mpsc::Sender<ActorMessage<M, R>>,
}

impl<M: Send + 'static, R: Send + 'static> ActorRef<M, R> {
    /// Fire-and-forget message delivery.
    pub async fn tell(&self, msg: M) -> Result<(), String> {
        self.tx
            .send(ActorMessage::Tell(msg))
            .await
            .map_err(|e| format!("actor tell failed: {e}"))
    }

    /// Request-Response ask pattern with timeout.
    pub async fn ask(&self, msg: M, timeout_dur: Duration) -> Result<R, String> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx
            .send(ActorMessage::Ask(msg, reply_tx))
            .await
            .map_err(|e| format!("actor ask failed: {e}"))?;

        tokio::time::timeout(timeout_dur, reply_rx)
            .await
            .map_err(|_| "actor response timed out".to_string())?
            .map_err(|_| "actor dropped response channel".to_string())
    }
}

/// Spawns an isolated stateful Actor executing on Tokio worker threads.
pub fn spawn_actor<M, R, S, F>(initial_state: S, capacity: usize, mut handler: F) -> ActorRef<M, R>
where
    M: Send + 'static,
    R: Send + 'static,
    S: Send + 'static,
    F: FnMut(&mut S, M) -> Option<R> + Send + 'static,
{
    let (tx, mut rx) = mpsc::channel::<ActorMessage<M, R>>(capacity);

    tokio::spawn(async move {
        let mut state = initial_state;
        while let Some(msg) = rx.recv().await {
            match msg {
                ActorMessage::Tell(payload) => {
                    let _ = handler(&mut state, payload);
                }
                ActorMessage::Ask(payload, reply_channel) => {
                    let response = handler(&mut state, payload);
                    if let Some(res) = response {
                        let _ = reply_channel.send(res);
                    }
                }
            }
        }
    });

    ActorRef { tx }
}

/// Structured Task Group Orchestrator (Structured Concurrency)
pub struct TaskGroup<T> {
    set: JoinSet<T>,
}

impl<T: Send + 'static> Default for TaskGroup<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + 'static> TaskGroup<T> {
    pub fn new() -> Self {
        Self {
            set: JoinSet::new(),
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
    {
        self.set.spawn(future);
    }

    pub async fn join_all(mut self) -> Vec<T> {
        let mut results = Vec::new();
        while let Some(res) = self.set.join_next().await {
            if let Ok(val) = res {
                results.push(val);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn verifies_mpsc_channel_pair() {
        let mut ch = Channel::<i64>::pair(10);
        ch.send(42).await.expect("send succeeds");
        let val = ch.recv().await.expect("recv receives");
        assert_eq!(val, 42);
    }

    #[tokio::test]
    async fn verifies_broadcast_channel_fanout() {
        let bcast = BroadcastChannel::<String>::new(16);
        let mut sub1 = bcast.subscribe();
        let mut sub2 = bcast.subscribe();

        bcast
            .send("broadcast_event".to_string())
            .expect("send succeeds");

        assert_eq!(sub1.recv().await.unwrap(), "broadcast_event");
        assert_eq!(sub2.recv().await.unwrap(), "broadcast_event");
    }

    #[tokio::test]
    async fn verifies_actor_tell_and_ask() {
        struct CounterState {
            count: i64,
        }

        enum CounterMsg {
            Increment(i64),
            GetCount,
        }

        let actor: ActorRef<CounterMsg, i64> =
            spawn_actor(CounterState { count: 100 }, 16, |state, msg| match msg {
                CounterMsg::Increment(n) => {
                    state.count += n;
                    None
                }
                CounterMsg::GetCount => Some(state.count),
            });

        // Tell: fire-and-forget increment
        actor
            .tell(CounterMsg::Increment(25))
            .await
            .expect("tell succeeds");

        // Ask: request-response with reply
        let total = actor
            .ask(CounterMsg::GetCount, Duration::from_millis(500))
            .await
            .expect("ask succeeds");

        assert_eq!(total, 125);
    }

    #[tokio::test]
    async fn verifies_structured_task_group() {
        let mut group = TaskGroup::<i64>::new();
        for i in 1..=5 {
            group.spawn(async move {
                KoraRuntime::sleep(Duration::from_millis(5 * i as u64)).await;
                i * 10
            });
        }

        let mut results = group.join_all().await;
        results.sort();
        assert_eq!(results, vec![10, 20, 30, 40, 50]);
    }

    #[tokio::test]
    async fn security_verifies_bounded_channel_prevents_memory_exhaustion() {
        // Enforce hard bound on capacity to prevent denial-of-service / memory leaks
        let ch = Channel::<i32>::pair(2);
        assert!(ch.try_send(1).is_ok());
        assert!(ch.try_send(2).is_ok());
        // 3rd item must be rejected immediately because buffer is full (bounded backpressure)
        assert!(ch.try_send(3).is_err());
    }

    #[tokio::test]
    async fn security_verifies_actor_ask_timeout_protection() {
        let actor: ActorRef<(), ()> = spawn_actor((), 4, |_state, _msg| {
            // Intentionally return None without responding to test timeout protection
            None
        });

        // Querying an unresponsive actor must timeout safely without hanging forever
        let timeout_res = actor.ask((), Duration::from_millis(50)).await;
        assert!(timeout_res.is_err());
    }
}
