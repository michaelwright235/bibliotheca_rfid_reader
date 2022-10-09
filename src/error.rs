use std::{fmt::Display, error::Error};
use libftd2xx::FtStatus;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ReaderError {
    /// UsbError is returned if there was some kind of connection problem.
    UsbError(FtStatus),
    WrongChecksum,
    WrongResponse,
    EmptyResponse,
    WrongCardId,
    NotImplemented,
    NoCard
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::UsbError(e) => f.write_str(e.to_string().as_str()),
            ReaderError::WrongChecksum => f.write_str("Wrong checksum in the response from the reader"),
            ReaderError::WrongResponse => f.write_str("Response cointains unrecognizable data"),
            ReaderError::EmptyResponse => f.write_str("Reader returned an empty response"),
            ReaderError::WrongCardId => f.write_str("Wrong card id. Card id should contain exactly 8 bytes"),
            ReaderError::NotImplemented => f.write_str("Not implemented yet"),
            ReaderError::NoCard => f.write_str("No cards found")
        }
    }
}

impl From<FtStatus> for ReaderError {
    fn from(e: FtStatus) -> Self {
        Self::UsbError(e)
    }
}

impl Error for ReaderError {}