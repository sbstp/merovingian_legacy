use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use bincode;

use error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Database {
    pub movies: Vec<Movie>,
    pub movies_path: PathBuf,
    pub tv_path: PathBuf,
}

impl Database {
    pub fn open<A>(path: A) -> Result<Option<Database>, error::Error>
    where
        A: AsRef<Path>,
    {
        let path = path.as_ref();
        if !path.exists() {
            Ok(None)
        } else {
            let file = BufReader::new(File::open(path)?);
            Ok(Some(bincode::deserialize_from(file)?))
        }
    }

    pub fn save<A>(&self, path: A) -> Result<(), error::Error>
    where
        A: AsRef<Path>,
    {
        let file = BufWriter::new(File::create(path)?);
        bincode::serialize_into(file, &self)?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    pub tmdb_id: i64,
    pub name: String,
    pub original_title: String,
    pub year: i32,
    pub overview: String,
    pub path: PathBuf,
    pub subtitles: Vec<Subtitle>,
    pub images: Vec<Image>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Subtitle {
    pub lang: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ImageKind {
    Poster,
    Backdrop,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Image {
    pub kind: ImageKind,
    pub path: PathBuf,
}
