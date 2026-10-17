use std::fmt;

#[derive(Debug)]
pub struct Skidka(f64);

#[derive(Debug)]
pub enum SkidkaError {
    SkidkaIsNegative,
    SkidkaExceedsMax,
}

impl fmt::Display for SkidkaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkidkaError::SkidkaIsNegative => write!(f, "skidka is negative"),
            SkidkaError::SkidkaExceedsMax => write!(f, "skidka exceeds 100"),
        }
    }
}

impl std::error::Error for SkidkaError {}

impl Skidka {
    pub fn new(value: f64) -> Result<Self, SkidkaError> {
        if value < 0.0 {
            return Err(SkidkaError::SkidkaIsNegative);
        }
        if value > 100.0 {
            return Err(SkidkaError::SkidkaExceedsMax);
        }
        Ok(Skidka(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}