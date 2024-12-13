use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub version: String,
    pub startup_delay: Duration,
    pub thread_count: usize,
    pub memory_limit: usize,
}

#[derive(Debug)]
pub struct AgentMetrics {
    pub uptime: Duration,
    pub responses_generated: u64,
    pub emotional_states_processed: u64,
    pub knowledge_integrations: u64,
}