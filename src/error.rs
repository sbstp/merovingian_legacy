use std::io;

use bincode;

#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    Io(io::Error),
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Error {
        Error::Bincode(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
