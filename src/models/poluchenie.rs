use std::fmt;
use time::UtcDateTime;

use crate::models::value_objects::{
    EntityId, EntityIdError, FormatOptions, Name, NameError, RecordLifetime, RecordLifetimeError,
};

#[derive(Debug)]
pub struct Poluchenie {
    pub(crate) rabotnik_id: EntityId,
    pub(crate) specodezhda_id: EntityId,
    pub(crate) data_polucheniya: UtcDateTime,
    pub(crate) podpis: Name,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug)]
pub enum PoluchenieError {
    RabotnikIdError(EntityIdError),
    SpecodezhdaIdError(EntityIdError),
    PodpisError(NameError),
    RecordLifetimeError(RecordLifetimeError),
}

impl fmt::Display for PoluchenieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoluchenieError::RabotnikIdError(e) => write!(f, "rabotnik_id error: {}", e),
            PoluchenieError::SpecodezhdaIdError(e) => write!(f, "specodezhda_id error: {}", e),
            PoluchenieError::PodpisError(e) => write!(f, "podpis error: {}", e),
            PoluchenieError::RecordLifetimeError(e) => write!(f, "record_lifetime error: {}", e),
        }
    }
}

impl std::error::Error for PoluchenieError {}

impl From<EntityIdError> for PoluchenieError {
    fn from(e: EntityIdError) -> Self { PoluchenieError::RabotnikIdError(e) }
}
impl From<NameError> for PoluchenieError {
    fn from(e: NameError) -> Self { PoluchenieError::PodpisError(e) }
}
impl From<RecordLifetimeError> for PoluchenieError {
    fn from(e: RecordLifetimeError) -> Self { PoluchenieError::RecordLifetimeError(e) }
}

impl Poluchenie {
    pub fn create_new(
        rabotnik_id: EntityId,
        specodezhda_id: EntityId,
        data_polucheniya: UtcDateTime,
        podpis: String,
    ) -> Result<Self, PoluchenieError> {
        let podpis = Name::new_with_format(podpis, vec![FormatOptions::Trim])?;
        let record_lifetime = RecordLifetime::create(time::UtcDateTime::now(), None, None)?;
        Ok(Self { rabotnik_id, specodezhda_id, data_polucheniya, podpis, record_lifetime })
    }
}