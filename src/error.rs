use std::io;

use bincode;
use reqwest;

use tmdb;

#[derive(Debug)]
pub enum Error {
    Bincode(bincode::Error),
    Io(io::Error),
    Http(reqwest::Error),
    TMDB(tmdb::search::Error),
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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl From<tmdb::search::Error> for Error {
    fn from(err: tmdb::search::Error) -> Error {
        Error::TMDB(err)
    }
}
