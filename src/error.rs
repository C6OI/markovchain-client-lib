use thiserror::Error;
use crate::content_string::ContentStringError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid payload text: {0}")]
    InvalidContentString(#[from] ContentStringError),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("failed to (de)serialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("unexpected API error: status {status}, body = {body}")]
    Api {
        status: reqwest::StatusCode,
        body: String,
    },
}
