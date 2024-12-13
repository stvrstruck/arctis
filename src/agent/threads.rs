use crate::types::cognitive::{ThreadState, ThreadPriority};
use crate::utils::thread::{ThreadManager, ThreadScheduler};

pub struct CognitiveThread {
    id: ThreadId,
    state: ThreadState,
    priority: ThreadPriority,
    processor: Box<dyn CognitiveProcessor>,
    scheduler: ThreadScheduler,
}

impl CognitiveThread {
    pub async fn process(&mut self) {
        self.scheduler.schedule(self.priority).await;
        self.processor.execute().await;
        self.update_metrics();
    }
}