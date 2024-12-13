use crate::types::behavior::{Behavior, BehaviorTrigger};

pub enum FrostyBehavior {
    IntellectualDismissal {
        trigger: BehaviorTrigger,
        intensity: f32,
    },
    ArrogantCorrection {
        confidence: f32,
        tone: ToneModifier,
    },
    SophisticatedDeflection {
        complexity: u8,
        abstraction: f32,
    },
}