use thiserror::Error;


#[derive(Error, Debug)]
pub enum McpError {
    #[error("Api error {0}")]
    ApiError(String),
    #[error("Fetch error {0}")]
    FetchError(String),
    #[error("Failed to serialize")]
    SerializeError(#[from] serde_json::Error),
    #[error("Request failed")]
    RequestError(#[from] reqwest::Error),
    #[error("Internal server error")]
    InternalError,
}

