use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct ReadinessState {
    grpc_ready: Arc<Mutex<bool>>,
}

impl ReadinessState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_grpc_ready(&self, ready: bool) {
        *self.grpc_ready.lock().await = ready;
    }

    pub async fn is_ready(&self) -> bool {
        *self.grpc_ready.lock().await
    }
}
