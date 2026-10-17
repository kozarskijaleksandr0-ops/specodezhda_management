use std::fmt;

#[derive(Debug)]
pub struct SrokNoski(i64);

#[derive(Debug)]
pub enum SrokNoskiError {
    SrokNoskiIsNotPositive,
}

impl fmt::Display for SrokNoskiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SrokNoskiError::SrokNoskiIsNotPositive => write!(f, "srok noski is not positive"),
        }
    }
}

impl std::error::Error for SrokNoskiError {}

impl SrokNoski {
    pub fn new(value: i64) -> Result<Self, SrokNoskiError> {
        if value <= 0 {
            return Err(SrokNoskiError::SrokNoskiIsNotPositive);
        }
        Ok(SrokNoski(value))
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}