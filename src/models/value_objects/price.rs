use std::fmt;

#[derive(Debug)]
pub struct Price(f64);

#[derive(Debug)]
pub enum PriceError {
    PriceIsNegative,
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceError::PriceIsNegative => write!(f, "price is negative"),
        }
    }
}

impl std::error::Error for PriceError {}

impl Price {
    pub fn new(value: f64) -> Result<Self, PriceError> {
        if value < 0.0 {
            return Err(PriceError::PriceIsNegative);
        }
        Ok(Price(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}