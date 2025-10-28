use crate::belt::FieldError;

/// Result alias for math operations within this crate.
pub type MathResult<T> = Result<T, MathError>;

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    #[error("division by zero")]
    DivisionByZero,
    #[error("field operation failed: {0:?}")]
    Field(FieldError),
}

impl From<FieldError> for MathError {
    fn from(value: FieldError) -> Self {
        MathError::Field(value)
    }
}
