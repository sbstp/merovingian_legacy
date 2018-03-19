use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Database {
    movies: Vec<Movie>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Movie {
    tmdb_id: i64,
    name: String,
    original_title: String,
    year: i32,
    overview: String,
    path: PathBuf,
    subtitles: Vec<Subtitle>,
    images: Vec<Image>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Subtitle {
    lang: String,
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
enum ImageKind {
    Poster,
    Backdrop,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Image {
    kind: ImageKind,
    path: PathBuf,
}
