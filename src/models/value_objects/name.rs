use std::fmt;

#[derive(Debug)]
pub struct Name(String);

#[derive(Debug)]
pub enum NameError {
    NameIsEmpty,
    NameLessThanMinLength,
    NameExceedsMaxLength,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NameError::NameIsEmpty => write!(f, "name is empty"),
            NameError::NameLessThanMinLength => write!(f, "name is too short"),
            NameError::NameExceedsMaxLength => write!(f, "name is too long"),
        }
    }
}

impl std::error::Error for NameError {}

pub enum FormatOptions {
    Trim,
    FirstLetterToUpper,
}

impl Name {
    pub fn new(value: String) -> Result<Self, NameError> {
        if value.trim().is_empty() {
            return Err(NameError::NameIsEmpty);
        }
        Ok(Name(value))
    }

    pub fn new_with_format(value: String, options: Vec<FormatOptions>) -> Result<Self, NameError> {
        let mut formatted = value.clone();
        for option in options {
            match option {
                FormatOptions::Trim => {
                    formatted = formatted.trim().to_string();
                }
                FormatOptions::FirstLetterToUpper => {
                    if let Some(first_char) = formatted.chars().next() {
                        let upper: String = first_char.to_uppercase().collect();
                        formatted = upper + &formatted[first_char.len_utf8()..];
                    }
                }
            }
        }
        Self::new(formatted)
    }

    pub fn new_with_length_check(
        value: String,
        min_length: usize,
        max_length: usize,
    ) -> Result<Self, NameError> {
        if value.len() < min_length {
            return Err(NameError::NameLessThanMinLength);
        }
        if value.len() > max_length {
            return Err(NameError::NameExceedsMaxLength);
        }
        Self::new(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}