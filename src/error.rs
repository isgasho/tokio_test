use std::io;
use std::net;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    AddrParseError(net::AddrParseError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::AddrParseError(err)
    }
}
