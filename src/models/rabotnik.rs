use std::fmt;

use crate::models::value_objects::{
    EntityId, EntityIdError, FormatOptions, Name, NameError, RecordLifetime,
    RecordLifetimeError, Skidka, SkidkaError,
};

#[derive(Debug)]
pub struct Rabotnik {
    pub(crate) id: EntityId,
    pub(crate) fio: Name,
    pub(crate) dolzhnost: Name,
    pub(crate) skidka: Skidka,
    pub(crate) ceh_id: EntityId,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug)]
pub enum RabotnikError {
    IdError(EntityIdError),
    FioError(NameError),
    DolzhnostError(NameError),
    SkidkaError(SkidkaError),
    CehIdError(EntityIdError),
    RecordLifetimeError(RecordLifetimeError),
}

impl fmt::Display for RabotnikError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RabotnikError::IdError(e) => write!(f, "id error: {}", e),
            RabotnikError::FioError(e) => write!(f, "fio error: {}", e),
            RabotnikError::DolzhnostError(e) => write!(f, "dolzhnost error: {}", e),
            RabotnikError::SkidkaError(e) => write!(f, "skidka error: {}", e),
            RabotnikError::CehIdError(e) => write!(f, "ceh_id error: {}", e),
            RabotnikError::RecordLifetimeError(e) => write!(f, "record_lifetime error: {}", e),
        }
    }
}

impl std::error::Error for RabotnikError {}

impl From<EntityIdError> for RabotnikError {
    fn from(e: EntityIdError) -> Self { RabotnikError::IdError(e) }
}
impl From<NameError> for RabotnikError {
    fn from(e: NameError) -> Self { RabotnikError::FioError(e) }
}
impl From<SkidkaError> for RabotnikError {
    fn from(e: SkidkaError) -> Self { RabotnikError::SkidkaError(e) }
}
impl From<RecordLifetimeError> for RabotnikError {
    fn from(e: RecordLifetimeError) -> Self { RabotnikError::RecordLifetimeError(e) }
}

impl Rabotnik {
    pub fn create_new(
        fio: String,
        dolzhnost: String,
        skidka: f64,
        ceh_id: EntityId,
    ) -> Result<Self, RabotnikError> {
        let id = EntityId::new();
        let fio = Name::new_with_format(fio, vec![FormatOptions::Trim])?;
        let dolzhnost = Name::new_with_format(dolzhnost, vec![FormatOptions::Trim])?;
        let skidka = Skidka::new(skidka)?;
        let record_lifetime = RecordLifetime::create(time::UtcDateTime::now(), None, None)?;
        Ok(Self { id, fio, dolzhnost, skidka, ceh_id, record_lifetime })
    }

    pub fn create(
        id: EntityId,
        fio: Name,
        dolzhnost: Name,
        skidka: Skidka,
        ceh_id: EntityId,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self { id, fio, dolzhnost, skidka, ceh_id, record_lifetime }
    }
}