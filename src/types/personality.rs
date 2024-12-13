#[derive(Debug, Clone)]
pub enum PersonalityTrait {
    Arrogance(f32),
    Intellectualism(f32),
    Dismissiveness(f32),
    Sophistication(f32),
}

#[derive(Debug)]
pub struct PersonalityState {
    pub dominant_traits: Vec<PersonalityTrait>,
    pub trait_interactions: HashMap<(PersonalityTrait, PersonalityTrait), f32>,
    pub stability_index: f32,
}