use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use database::{Database, Movie};
use fingerprint;
use fs::{self, File};
use parse;
use tmdb::search;

fn build_movie_path(
    base: &Path,
    ext: &str,
    movie: &search::Movie,
    duplicate_index: i32,
) -> PathBuf {
    let version = if duplicate_index > 1 {
        format!(".v{}", duplicate_index)
    } else {
        format!("")
    };

    let dirname = fs::filter_filename(&format!("{} ({})", movie.title, movie.year()));
    let filename = fs::filter_filename(&format!(
        "{} ({}){}.{}",
        movie.title,
        movie.year(),
        version,
        ext
    ));

    let mut path = base.to_owned();
    path.push(dirname);
    path.push(filename);
    path
}

fn process_video_file(file: &File, stem: &str, ext: &str, db: &mut Database) {
    let hash = fingerprint::file(file).expect("failed to hash");
    if let Some(movie) = db.match_fingerprint(&hash) {
        println!(
            "{} is already in the library at {}",
            file.display(),
            movie.path.display()
        );
    } else {
        let (movie, year) = parse::movie::parse_movie(&stem);
        let mut paged = search::movie(&movie, year).expect("api fail");

        let api_movie = paged.results.remove(0);

        let duplicate_index = db.duplicates(api_movie.id)
            .last()
            .map(|m| m.duplicate_index)
            .unwrap_or(0) + 1;

        let path = build_movie_path(db.movies_path(), ext, &api_movie, duplicate_index);

        let movie = Movie {
            tmdb_id: api_movie.id,
            duplicate_index: duplicate_index,
            title: api_movie.title,
            original_title: api_movie.original_title,
            year: api_movie.release_date[..4].parse().unwrap(),
            overview: api_movie.overview,
            path: path.clone(),
            images: vec![],
            subtitles: vec![],
            fingerprint: hash,
        };

        let movie = db.add_movie(movie);

        fs::best_copy(&file, path).expect("bad copy");

        println!(
            "Added {} to database at {}",
            file.display(),
            movie.path.display()
        );
    }
}

pub fn import<A>(path: A, db: &mut Database)
where
    A: AsRef<Path>,
{
    let root = fs::walk(path).expect("failed to walk directory");
    for entry in root.iter() {
        if let Some(file) = entry.as_file() {
            if let Some(ext) = file.extension().map(OsStr::to_string_lossy) {
                if parse::metadata::VIDEO_FILES.contains(&ext.to_lowercase()[..]) {
                    if let Some(stem) = file.file_stem().map(OsStr::to_string_lossy) {
                        process_video_file(file, &stem, &ext, db);
                    }
                }
            }
        }
    }
}
