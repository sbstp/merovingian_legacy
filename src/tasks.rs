use std::path::Path;
use std::ffi::OsStr;

use fs;
use parse;

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
                        let (movie, year) = parse::movie::parse_movie(&name);
                        println!("{} - ({:?}) --- {}", movie, year, name);
                    }
                }
            }
        }
    }
}
