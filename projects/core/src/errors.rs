#[derive(Debug, Copy, Clone)]
/// Error type for the sub_projects crate
pub enum Error {
    /// Error when the project is not found
    UnknownError
}

/// Result type for the sub_projects crate
pub type Result<T> = std::result::Result<T, Error>;
