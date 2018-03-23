use std::io;

use reqwest;
use serde_json;

use tmdb;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Http(reqwest::Error),
    TMDB(tmdb::search::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
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
