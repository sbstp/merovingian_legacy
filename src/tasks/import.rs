use std::path::{Path, PathBuf};

use database::{Database, Movie, Subtitle};
use fingerprint;
use fs::{self, Entry};
use parse;
use tmdb::search;
use tree::{Node, Tree};

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

fn process_video_file(
    tree: &Tree<Entry>,
    node: Node,
    file: &Entry,
    stem: &str,
    ext: &str,
    db: &mut Database,
) {
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

        let sub_entries: Vec<&Entry> = scan_subtitles(&tree, node)
            .iter()
            .map(|&n| tree.data(n))
            .collect();

        let path = build_movie_path(db.movies_path(), ext, &api_movie, duplicate_index);
        let subtitles = sub_entries
            .iter()
            .map(|&sub| Subtitle {
                lang: None,
                path: build_movie_path(
                    db.movies_path(),
                    sub.extension().expect("subtitle has no extension"),
                    &api_movie,
                    duplicate_index,
                ),
            })
            .collect();

        let movie = Movie {
            tmdb_id: api_movie.id,
            duplicate_index: duplicate_index,
            title: api_movie.title,
            original_title: api_movie.original_title,
            year: api_movie.release_date[..4].parse().unwrap(),
            overview: api_movie.overview,
            path: path.clone(),
            images: vec![],
            subtitles: subtitles,
            fingerprint: hash,
        };

        let movie = db.add_movie(movie);

        fs::best_copy(&file, path).expect("failed to copy movie");
        for (&entry, sub) in sub_entries.iter().zip(movie.subtitles.iter()) {
            fs::best_copy(&entry, &sub.path);
        }

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
    let (tree, root) = fs::walk(path).expect("failed to walk directory");
    for node in tree.recursive_iter(root) {
        let entry = tree.data(node);
        if entry.is_file() && entry.is_video() {
            if let (Some(stem), Some(ext)) = (entry.stem(), entry.extension()) {
                process_video_file(&tree, node, entry, stem, ext, db);
            }
        }
    }
}

pub fn scan_subtitles(tree: &Tree<Entry>, video: Node) -> Vec<Node> {
    let mut subtitles = vec![];
    let video_entry = tree.data(video);

    let siblings: Vec<Node> = tree.siblings(video);
    let other_videos: Vec<&Entry> = siblings
        .iter()
        .map(|&n| tree.data(n))
        .filter(|e| e.is_video())
        .collect();

    // If there's only one video file inside the directory that contains this video
    if other_videos.is_empty() {
        if let Some(parent) = tree.parent(video) {
            // Find every subtitle file recursively from the video file's parent.
            for node in tree.recursive_iter(parent) {
                let entry = tree.data(node);
                if entry.is_subtitle() {
                    subtitles.push(node);
                }
            }
        }
    } else {
        if let Some(stem) = video_entry.stem() {
            let stem = stem.to_lowercase();
            // Find subtitles that have the same file name, with a subtitle extension.
            let same_name_subs: Vec<Node> = siblings
                .iter()
                .filter(|&&n| {
                    let entry = tree.data(n);
                    if entry.is_subtitle() {
                        if let Some(sub_stem) = entry.stem() {
                            return stem == sub_stem.to_lowercase();
                        }
                    }
                    false
                })
                .cloned()
                .collect();
            subtitles.extend(same_name_subs);
        }
    }

    subtitles
}
