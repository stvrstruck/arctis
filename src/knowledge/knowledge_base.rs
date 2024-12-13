// Knowledge Management System
use crate::types::{Knowledge, Topic, Confidence};

pub struct KnowledgeBase {
    core_knowledge: HashMap<Topic, Knowledge>,
    personality_traits: Vec<PersonalityTrait>,
    expertise_areas: BTreeMap<Topic, Confidence>,
    learning_patterns: Vec<LearningPattern>,
}

impl KnowledgeBase {
    pub fn query_knowledge(&self, topic: &Topic) -> Option<Knowledge> {
        let base_knowledge = self.core_knowledge.get(topic)?;
        let expertise_level = self.expertise_areas.get(topic).unwrap_or(&Confidence::Low);
        
        Some(self.enhance_knowledge_with_personality(
            base_knowledge,
            expertise_level,
        ))
    }
    
    pub fn integrate_new_knowledge(&mut self, topic: Topic, knowledge: Knowledge) {
        if self.validate_knowledge(&knowledge) {
            self.core_knowledge.insert(topic, knowledge);
            self.update_expertise_levels(&topic);
            self.adjust_learning_patterns(&topic);
        }
    }
}