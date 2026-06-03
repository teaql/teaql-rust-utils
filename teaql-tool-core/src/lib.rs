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

/// A zero-cost wrapper that forces the developer (or AI) to provide a business comment.
/// Without calling `.comment()`, the internal value cannot be extracted or used.
/// This prevents un-annotated logic in the application layer.
#[repr(transparent)]
pub struct MustComment<T> {
    value: T,
}

impl<T> MustComment<T> {
    /// Internal constructor, used by the framework to wrap returned values.
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// The ONLY way to extract the value. Enforces providing a business intent string.
    /// In release builds, this is completely optimized away (Zero-Cost).
    #[inline(always)]
    pub fn comment(self, _desc: impl Into<String>) -> T {
        self.value
    }
}
