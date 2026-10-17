use std::fmt;

use crate::models::value_objects::{
    EntityId, EntityIdError, FormatOptions, Name, NameError, RecordLifetime, RecordLifetimeError,
};

#[derive(Debug)]
pub struct Ceh {
    pub(crate) id: EntityId,
    pub(crate) name: Name,
    pub(crate) nachalnik: Name,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug)]
pub enum CehError {
    IdError(EntityIdError),
    NameError(NameError),
    NachalnikError(NameError),
    RecordLifetimeError(RecordLifetimeError),
}

impl fmt::Display for CehError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CehError::IdError(e) => write!(f, "id error: {}", e),
            CehError::NameError(e) => write!(f, "name error: {}", e),
            CehError::NachalnikError(e) => write!(f, "nachalnik error: {}", e),
            CehError::RecordLifetimeError(e) => write!(f, "record_lifetime error: {}", e),
        }
    }
}

impl std::error::Error for CehError {}

impl From<EntityIdError> for CehError {
    fn from(e: EntityIdError) -> Self { CehError::IdError(e) }
}
impl From<NameError> for CehError {
    fn from(e: NameError) -> Self { CehError::NameError(e) }
}
impl From<RecordLifetimeError> for CehError {
    fn from(e: RecordLifetimeError) -> Self { CehError::RecordLifetimeError(e) }
}

impl Ceh {
    pub fn create_new(name: String, nachalnik: String) -> Result<Self, CehError> {
        let id = EntityId::new();
        let name = Name::new_with_format(name, vec![FormatOptions::Trim])?;
        let nachalnik = Name::new_with_format(nachalnik, vec![FormatOptions::Trim])?;
        let record_lifetime = RecordLifetime::create(time::UtcDateTime::now(), None, None)?;
        Ok(Self { id, name, nachalnik, record_lifetime })
    }

    pub fn create(
        id: EntityId,
        name: Name,
        nachalnik: Name,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self { id, name, nachalnik, record_lifetime }
    }
}