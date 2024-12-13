pub trait Validator {
    fn validate(&self, data: &impl Validatable) -> ValidationResult;
}

pub struct ValidationChain {
    validators: Vec<Box<dyn Validator>>,
}

impl ValidationChain {
    pub fn validate_all(&self, data: &impl Validatable) -> ValidationResult {
        self.validators.iter()
            .map(|validator| validator.validate(data))
            .fold(ValidationResult::default(), ValidationResult::combine)
    }
}