// Main entry point for Arctis AI Agent
use crate::agent::ArctisAgent;
use crate::personality::FrostyPersonality;
use crate::knowledge::KnowledgeBase;
use crate::emotion::EmotionalCore;
use crate::dialogue::DialogueManager;
use rig::prelude::*;

#[tokio::main]
async fn main() {
    // Initialize the Arctis AI agent with frosty personality
    let knowledge_base = KnowledgeBase::new()
        .load_core_knowledge()
        .load_personality_traits();
        
    let emotional_core = EmotionalCore::new()
        .with_base_temperament(Temperament::Cold)
        .with_emotional_range(0.3..0.8); // Limited emotional range for arrogance
        
    let dialogue_system = DialogueManager::new()
        .with_language_model("arctic-gpt-v2")
        .with_response_style(ResponseStyle::Sophisticated);
        
    let personality = FrostyPersonality::new()
        .with_arrogance_factor(0.85)
        .with_confidence_threshold(0.92);
        
    let mut arctis = ArctisAgent::new()
        .with_knowledge(knowledge_base)
        .with_emotional_core(emotional_core)
        .with_dialogue_system(dialogue_system)
        .with_personality(personality);
        
    // Start the agent's cognitive loop
    arctis.initialize_cognitive_processes().await;
    arctis.run().await;
}