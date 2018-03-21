use std::path::Path;
use std::ffi::OsStr;

use fs;
use parse;
use tmdb::search;

pub fn import<A>(path: A)
where
    A: AsRef<Path>,
{
    let root = fs::walk(path).expect("failed to walk directory");
    for entry in root.iter() {
        if let Some(file) = entry.as_file() {
            if let Some(ext) = file.extension().map(OsStr::to_string_lossy) {
                if parse::metadata::VIDEO_FILES.contains(&ext.to_lowercase()[..]) {
                    if let Some(name) = file.file_stem().map(OsStr::to_string_lossy) {
                        println!("doing {}", name);
                        let (movie, year) = parse::movie::parse_movie(&name);
                        println!("parse {} ({:?})", movie, year);
                        let results = search::movie(&movie, year).expect("api fail");
                        println!("{:#?}", results);
                        let movie = results.results.get(0).expect("no results");
                        println!("{} - {} ({})", name, movie.title, movie.release_date);
                        println!("-----------");
                    }
                }
            }
        }
    }
}
