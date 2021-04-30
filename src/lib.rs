//! Rust bindings  for the Covid19gr API!
mod macros;

pub mod cum;
pub mod daily;
pub mod hosp;
pub mod refug;

/// Define custom Result
type Result<T> = std::result::Result<T, CrateError>;

// API BASE_URL
const BASE_URL: &'static str = "https://covid-19-greece.herokuapp.com";

/// Represents different type of errors that can happen.
#[derive(Debug, Clone, PartialEq)]
enum CrateError {
    /// The error was caused during serializing/deserializing data from JSON.
    SerialError,
    /// The error was caused during an HTTP GET request.
    HttpError,
}

pub(crate) fn build_request(url_suffix: &str) -> String {
    let body: String = ureq::get(format!("{}/{}", BASE_URL, url_suffix).as_str())
        .set("Accept", "application/json")
        .call()
        .expect("http error")
        .into_string()
        .expect("serializaton error");
    body
}
