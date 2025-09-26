use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum KrakenError {
    #[error("IO Error")]
    IoError,

    #[error("Enum Conversion Error")]
    EnumError,
}