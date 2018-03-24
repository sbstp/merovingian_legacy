use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use serde_json;

use error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Database {
    pub movies_path: PathBuf,
    pub tv_path: PathBuf,
    movies: Vec<Movie>,
    fingerprint_map: HashMap<String, usize>,
}

impl Database {
    pub fn new<A1, A2>(movies_path: A1, tv_path: A2) -> Database
    where
        A1: AsRef<Path>,
        A2: AsRef<Path>,
    {
        Database {
            movies: vec![],
            movies_path: movies_path.as_ref().to_owned(),
            tv_path: tv_path.as_ref().to_owned(),
            fingerprint_map: HashMap::new(),
        }
    }

    pub fn open<A>(path: A) -> Result<Option<Database>, error::Error>
    where
        A: AsRef<Path>,
    {
        let path = path.as_ref();
        if !path.exists() {
            Ok(None)
        } else {
            let file = BufReader::new(File::open(path)?);
            Ok(Some(serde_json::from_reader(file)?))
        }
    }

    pub fn save<A>(&self, path: A) -> Result<(), error::Error>
    where
        A: AsRef<Path>,
    {
        let file = BufWriter::new(File::create(path)?);
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn add_movie(&mut self, movie: Movie) {
        let fingerprint = movie.fingerprint.clone();
        self.movies.push(movie);
        let idx = self.movies.len() - 1;
        self.fingerprint_map.insert(fingerprint, idx);
    }

    pub fn match_fingerprint<'db>(&'db self, fingerprint: &str) -> Option<&'db Movie> {
        self.fingerprint_map
            .get(fingerprint)
            .and_then(|&idx| self.movies.get(idx))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    pub tmdb_id: i64,
    pub title: String,
    pub original_title: String,
    pub year: i32,
    pub overview: String,
    pub path: PathBuf,
    pub subtitles: Vec<Subtitle>,
    pub images: Vec<Image>,
    pub fingerprint: String,
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
