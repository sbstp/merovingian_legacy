use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use serde_json;

use error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Database {
    movies_path: PathBuf,
    tv_path: PathBuf,
    movies: Vec<Movie>,
    movies_index: HashMap<String, usize>,
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
            movies_index: HashMap::new(),
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

    pub fn movies_path(&self) -> &Path {
        &self.movies_path
    }

    pub fn tv_path(&self) -> &Path {
        &self.tv_path
    }

    pub fn add_movie(&mut self, movie: Movie) -> &Movie {
        let fingerprint = movie.fingerprint.clone();
        self.movies.push(movie);
        let idx = self.movies.len() - 1;
        self.movies_index.insert(fingerprint, idx);
        &self.movies[idx]
    }

    pub fn match_fingerprint<'db>(&'db self, fingerprint: &str) -> Option<&'db Movie> {
        self.movies_index
            .get(fingerprint)
            .and_then(|&idx| self.movies.get(idx))
    }

    pub fn duplicates(&self, tmdb_id: i64) -> Vec<&Movie> {
        self.movies
            .iter()
            .filter(|m| m.tmdb_id == tmdb_id)
            .collect()
    }

    pub fn rebuild_index(&mut self) {
        self.movies_index.clear();
        for (idx, movie) in self.movies.iter().enumerate() {
            self.movies_index.insert(movie.fingerprint.clone(), idx);
        }
    }

    pub fn retain_movies<F>(&mut self, func: F)
    where
        F: FnMut(&Movie) -> bool,
    {
        self.movies.retain(func);
        self.rebuild_index();
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Movie {
    pub tmdb_id: i64,
    pub duplicate_index: i32, // Used when there are duplicate copies of a movie.
    pub fingerprint: String,  // Unique fingerprint of the file, see fingerprint::file.

    pub title: String,
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
