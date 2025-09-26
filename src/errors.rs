use thiserror::Error;

#[derive(Error, Debug)]
pub enum KrakenError {
    #[error("IO Error")]
    IO,

    #[error("Invalid enum value error: {0}")]
    Enum(String),

    #[error("Error")]
    Error,
}