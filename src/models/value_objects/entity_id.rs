use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub struct EntityId(Uuid);

#[derive(Debug)]
pub enum EntityIdError {
    EmptyId,
    InvalidId,
}

impl fmt::Display for EntityIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityIdError::EmptyId => write!(f, "id is empty"),
            EntityIdError::InvalidId => write!(f, "id is invalid"),
        }
    }
}

impl std::error::Error for EntityIdError {}

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn create(id: Uuid) -> Result<Self, EntityIdError> {
        if id.is_nil() {
            return Err(EntityIdError::EmptyId);
        }
        if id.get_variant() != uuid::Variant::RFC4122
            || id.get_version() != Some(uuid::Version::Random)
        {
            return Err(EntityIdError::InvalidId);
        }
        Ok(Self(id))
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}