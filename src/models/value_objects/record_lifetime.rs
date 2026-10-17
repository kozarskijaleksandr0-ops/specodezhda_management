use std::fmt;
use time::UtcDateTime;

#[derive(Debug)]
pub struct RecordLifetime {
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

#[derive(Debug)]
pub enum RecordLifetimeError {
    UpdatedBeforeCreated,
    DeletedBeforeCreated,
    DateTimeOutOfRange,
}

impl fmt::Display for RecordLifetimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordLifetimeError::UpdatedBeforeCreated => write!(f, "updated_at before created_at"),
            RecordLifetimeError::DeletedBeforeCreated => write!(f, "deleted_at before created_at"),
            RecordLifetimeError::DateTimeOutOfRange => write!(f, "datetime out of range"),
        }
    }
}

impl std::error::Error for RecordLifetimeError {}

impl RecordLifetime {
    pub fn create(
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, RecordLifetimeError> {
        use time::macros::datetime;
        let max_valid = datetime!(2060-01-01 00:00:00 UTC);
        let min_valid = datetime!(2000-01-01 00:00:00 UTC);

        if let Some(updated_at) = updated_at {
            if updated_at < created_at {
                return Err(RecordLifetimeError::UpdatedBeforeCreated);
            }
        }
        if let Some(deleted_at) = deleted_at {
            if deleted_at < created_at {
                return Err(RecordLifetimeError::DeletedBeforeCreated);
            }
        }
        if created_at < min_valid || created_at > max_valid {
            return Err(RecordLifetimeError::DateTimeOutOfRange);
        }

        Ok(RecordLifetime {
            created_at,
            updated_at,
            deleted_at,
        })
    }

    pub fn created_at(&self) -> UtcDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<UtcDateTime> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<UtcDateTime> {
        self.deleted_at
    }
}