use thiserror::Error;


#[derive(Error, Debug)]
pub enum McpError {
    #[error("Api error {0}")]
    ApiError(String),
    #[error("Fetch error {0}")]
    FetchError(String),
    #[error("Internal server error")]
    InternalError,
}

