use crate::types::personality::{Modifier, ModifierStrength};

pub struct ArroganceModifier {
    strength: ModifierStrength,
    confidence_boost: f32,
    dismissiveness_factor: f32,
}

pub struct IntellectualModifier {
    vocabulary_enhancement: f32,
    complexity_factor: f32,
    abstraction_level: u8,
}