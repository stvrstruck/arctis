// Responsive Trait Implementation
pub trait Responsive {
    fn generate_response(&self, stimulus: Stimulus) -> Response;
    fn adapt_to_feedback(&mut self, feedback: Feedback);
    fn calibrate_response_patterns(&mut self);
}