use thiserror::Error;

/// A unified error type for all TeaQL Tool operations.
#[derive(Debug, Error)]
pub enum TeaQLToolError {
    #[error("Parse Error: {0}")]
    ParseError(String),

    #[error("Invalid Argument: {0}")]
    InvalidArgument(String),

    #[error("Execution Error: {0}")]
    ExecutionError(String),

    #[error("Not Supported: {0}")]
    NotSupported(String),
}

/// A standard result type alias used throughout the TeaQL Tool ecosystem.
pub type Result<T> = std::result::Result<T, TeaQLToolError>;
