// Type Definitions
pub enum AgentState {
    Initializing,
    Active,
    Processing,
    Responding,
    Learning,
}

pub enum BehavioralPattern {
    Dismissive,
    Intellectual,
    Sophisticated,
    Analytical,
}

pub enum InteractionType {
    Challenge,
    Query,
    Casual,
    Intellectual,
}

pub enum SocialStrategy {
    Dominate,
    DisplayExpertise,
    MaintainDistance,
    Default,
}

pub struct EmotionalResponse {
    primary: Emotion,
    intensity: f32,
    expression: ExpressionPattern,
}