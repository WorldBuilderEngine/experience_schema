use crate::experience_schema::experience_schema::ExperienceSchema;

/// Implemented by authored schemas/template libraries that compile down into
/// the canonical published `ExperienceSchema` contract.
pub trait ExperienceSchemaTranspile {
    fn transpile(&self) -> anyhow::Result<ExperienceSchema>;
}
