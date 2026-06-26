use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynapseError {
    /// The requested resource does not exist (HTTP 404).
    #[error("not found: {0}")]
    NotFound(String),
    /// The provided pagination cursor is invalid or expired (HTTP 400).
    #[error("invalid or expired cursor: {0}")]
    InvalidCursor(String),
    /// The API returned a non-success status code.
    #[error("api error {status}: {message}")]
    Api { status: u16, message: String },
    /// A network-level error from the HTTP client.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    /// The response body could not be decoded as JSON.
    #[error("json decode error: {0}")]
    Decode(#[from] serde_json::Error),
}
