use crate::traits::{Cognitive, Responsive, SelfAware};
use crate::types::agent::{AgentConfig, AgentMetrics};
use crate::knowledge::KnowledgeBase;
use crate::emotion::EmotionalCore;
use crate::dialogue::DialogueManager;
use crate::personality::FrostyPersonality;
use crate::memory::MemorySystem;
use crate::perception::PerceptionEngine;

pub struct ArctisAgent {
    config: AgentConfig,
    state: AgentState,
    knowledge: KnowledgeBase,
    emotional_core: EmotionalCore,
    dialogue_system: DialogueManager,
    personality: FrostyPersonality,
    memory: MemorySystem,
    perception: PerceptionEngine,
    metrics: AgentMetrics,
    cognitive_threads: Vec<CognitiveThread>,
}