use crate::types::dialogue::{Filter, FilterChain};

pub struct ArroganceFilter {
    confidence_boost: f32,
    dismissiveness: f32,
}

pub struct ComplexityFilter {
    min_complexity: u8,
    vocabulary_level: u8,
    abstraction_factor: f32,
}

pub struct ToneFilter {
    base_tone: ToneType,
    modifiers: Vec<ToneModifier>,
}