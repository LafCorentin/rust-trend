use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The keyword is not set with the client !")]
    KeywordNotSet,

    #[error("The maximum is 5 keywords !")]
    KeywordMaxCapacity,

    #[error("At least one keyword is required !")]
    KeywordMinCapacity,

    #[error("Reqwest error: {0:?}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde url encoded error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Json parsing failed !")]
    JsonParsingFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
