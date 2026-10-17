use std::fmt;

use crate::models::value_objects::{
    EntityId, EntityIdError, FormatOptions, Name, NameError, Price, PriceError,
    RecordLifetime, RecordLifetimeError, SrokNoski, SrokNoskiError,
};

#[derive(Debug)]
pub struct Specodezhda {
    pub(crate) id: EntityId,
    pub(crate) vid: Name,
    pub(crate) srok_noski: SrokNoski,
    pub(crate) stoimost: Price,
    pub(crate) record_lifetime: RecordLifetime,
}

#[derive(Debug)]
pub enum SpecodezhdaError {
    IdError(EntityIdError),
    VidError(NameError),
    SrokNoskiError(SrokNoskiError),
    PriceError(PriceError),
    RecordLifetimeError(RecordLifetimeError),
}

impl fmt::Display for SpecodezhdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpecodezhdaError::IdError(e) => write!(f, "id error: {}", e),
            SpecodezhdaError::VidError(e) => write!(f, "vid error: {}", e),
            SpecodezhdaError::SrokNoskiError(e) => write!(f, "srok_noski error: {}", e),
            SpecodezhdaError::PriceError(e) => write!(f, "price error: {}", e),
            SpecodezhdaError::RecordLifetimeError(e) => write!(f, "record_lifetime error: {}", e),
        }
    }
}

impl std::error::Error for SpecodezhdaError {}

impl From<EntityIdError> for SpecodezhdaError {
    fn from(e: EntityIdError) -> Self { SpecodezhdaError::IdError(e) }
}
impl From<NameError> for SpecodezhdaError {
    fn from(e: NameError) -> Self { SpecodezhdaError::VidError(e) }
}
impl From<SrokNoskiError> for SpecodezhdaError {
    fn from(e: SrokNoskiError) -> Self { SpecodezhdaError::SrokNoskiError(e) }
}
impl From<PriceError> for SpecodezhdaError {
    fn from(e: PriceError) -> Self { SpecodezhdaError::PriceError(e) }
}
impl From<RecordLifetimeError> for SpecodezhdaError {
    fn from(e: RecordLifetimeError) -> Self { SpecodezhdaError::RecordLifetimeError(e) }
}

impl Specodezhda {
    pub fn create_new(
        vid: String,
        srok_noski: i64,
        stoimost: f64,
    ) -> Result<Self, SpecodezhdaError> {
        let id = EntityId::new();
        let vid = Name::new_with_format(vid, vec![FormatOptions::Trim])?;
        let srok_noski = SrokNoski::new(srok_noski)?;
        let stoimost = Price::new(stoimost)?;
        let record_lifetime = RecordLifetime::create(time::UtcDateTime::now(), None, None)?;
        Ok(Self { id, vid, srok_noski, stoimost, record_lifetime })
    }

    pub fn create(
        id: EntityId,
        vid: Name,
        srok_noski: SrokNoski,
        stoimost: Price,
        record_lifetime: RecordLifetime,
    ) -> Self {
        Self { id, vid, srok_noski, stoimost, record_lifetime }
    }
}