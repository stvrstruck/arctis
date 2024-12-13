// Cognitive Trait Implementation
pub trait Cognitive {
    fn process_input(&mut self) -> CognitiveResult;
    fn analyze_context(&self) -> ContextAnalysis;
    fn generate_thoughts(&self) -> Vec<Thought>;
    fn evaluate_options(&self) -> DecisionMatrix;
}