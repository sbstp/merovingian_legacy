use std::ffi::OsStr;
use std::path::Path;

use database::{Database, Movie};
use fingerprint;
use fs;
use parse;
use tmdb::search;

pub fn import<A>(path: A, db: &mut Database)
where
    A: AsRef<Path>,
{
    let root = fs::walk(path).expect("failed to walk directory");
    for entry in root.iter() {
        if let Some(file) = entry.as_file() {
            if let Some(ext) = file.extension().map(OsStr::to_string_lossy) {
                if parse::metadata::VIDEO_FILES.contains(&ext.to_lowercase()[..]) {
                    if let Some(name) = file.file_stem().map(OsStr::to_string_lossy) {
                        let hash = fingerprint::file(file).expect("failed to hash");
                        if let Some(movie) = db.match_fingerprint(&hash) {
                            println!(
                                "{} is already in the library as {}",
                                file.file_name().unwrap().to_string_lossy(),
                                movie.title
                            );
                        } else {
                            let (movie, year) = parse::movie::parse_movie(&name);
                            let mut results = search::movie(&movie, year).expect("api fail");
                            println!("{:#?}", results);
                            let api_movie = results.results.remove(0);
                            println!("added movie {}", movie);
                            let movie = Movie {
                                tmdb_id: api_movie.id,
                                title: api_movie.title,
                                original_title: api_movie.original_title,
                                year: api_movie.release_date[..4].parse().unwrap(),
                                overview: api_movie.overview,
                                path: "".into(),
                                images: vec![],
                                subtitles: vec![],
                                fingerprint: hash,
                            };
                            db.add_movie(movie);

                            // let movie = results.results.get(0).expect("no results");
                            // println!("{} - {} ({})", name, movie.title, movie.release_date);
                            // println!("-----------");
                        }
                    }
                }
            }
        }
    }
}
