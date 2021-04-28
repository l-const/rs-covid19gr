//! Rust bindings  for the Covid19gr API!
pub mod cum;
pub mod daily;
pub mod hosp;
pub mod refug;

/// Define custom Result
type Result<T> = std::result::Result<T, CrateError>;

/// Represents different type of errors that can happen.
#[derive(Debug, Clone, PartialEq)]
enum CrateError {
    /// The error was cuased during serializing/deserializing data from JSON.
    SerialError,
    /// The error was caused during an HTTP GET request.
    HttpError,
}
