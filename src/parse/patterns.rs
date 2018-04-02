use fs::Entry;
use parse::episode::{parse_episode, Episode};
use parse::movie::parse_movie;
use tree::{Node, Tree};

// pub struct Episode {
//     pub series_name: Option<String>,
//     pub season: Option<i32>,
//     pub number: i32,
// }

#[derive(Debug)]
pub struct Season {
    pub series_name: String,
    pub episodes: Vec<Episode>,
    pub number: i32,
}

#[derive(Debug)]
pub struct Series {
    pub name: String,
    pub seasons: Vec<Season>,
}

#[derive(Debug)]
pub struct Movie {
    pub name: String,
    pub year: Option<i32>,
}

#[derive(Debug)]
pub enum Media {
    Episode(Episode),
    Season(Season),
    Series(Series),
    Movie(Movie),
}

fn is_garbage_dir(stem: &str) -> bool {
    stem.contains("sample") || stem.contains("extra")
}

pub fn try_episode(tree: &Tree<Entry>, node: Node) -> Option<Episode> {
    let entry = tree.data(node);
    let stem = entry.stem().expect("invalid stem");

    if entry.is_dir() && !is_garbage_dir(stem) {
        // TODO try parse dir

        let subvids: Vec<_> = tree.children(node)
            .map(|n| tree.data(n))
            .filter(|e| e.is_video() && !e.is_ignored())
            .collect(); // TODO: filter out samples, extras

        return match subvids.len() {
            1 => parse_episode(subvids[0].stem().unwrap()),
            _ => None,
        };
    } else if entry.is_video() {
        return parse_episode(stem);
    }

    None
}

pub fn try_season(tree: &Tree<Entry>, node: Node) -> Option<Season> {
    let mut season: Option<i32> = None;
    let mut episodes = vec![];
    for child in tree.children(node) {
        if let Some(ep) = try_episode(tree, child) {
            if let Some(seas) = ep.season {
                season = Some(seas);
            }
            episodes.push(ep);
        }
    }

    episodes.sort_by_key(|ep| ep.episode);

    if let Some(season) = season {
        return Some(Season {
            number: season,
            series_name: episodes[0].series_name.clone(), // TODO: better way of picking series name
            episodes,
        });
    }

    return None;
}

pub fn try_series(tree: &Tree<Entry>, node: Node) -> Option<Series> {
    let mut seasons = vec![];
    for child in tree.children(node) {
        if let Some(season) = try_season(tree, child) {
            seasons.push(season);
        }
    }
    seasons.sort_by_key(|s| s.number);
    if !seasons.is_empty() {
        return Some(Series {
            name: seasons[0].series_name.clone(), // TODO: better way of picking series name
            seasons: seasons,
        });
    }

    None
}

pub fn try_movie(tree: &Tree<Entry>, node: Node) -> Option<Movie> {
    let entry = tree.data(node);
    if entry.is_video() {
        let (name, year) = parse_movie(entry.stem().unwrap());
        return Some(Movie { name, year });
    } else if entry.is_dir() {
        // Collect all the videos inside the directory.
        // TODO: filter extras, samples and release videos.
        let entries: Vec<_> = tree.children(node)
            .map(|child| tree.data(child))
            .filter(|&entry| entry.is_video() && !entry.is_ignored())
            .collect();

        return match entries.len() {
            1 => {
                let (name, year) = parse_movie(entries[0].stem().unwrap());
                Some(Movie { name, year })
            }
            _ => None,
        };
    }
    None
}

fn try_movie_pack(tree: &Tree<Entry>, node: Node) -> Vec<Movie> {
    let mut movies = vec![];
    for child in tree.children(node) {
        if let Some(movie) = try_movie(tree, child) {
            movies.push(movie);
        }
    }
    movies
}

pub fn scan(tree: &Tree<Entry>, root: Node) {
    for child in tree.children(root) {
        if let Some(episode) = try_episode(tree, child) {
            println!("{:#?}", episode);
        } else if let Some(season) = try_season(tree, child) {
            println!("{:#?}", season);
        } else if let Some(series) = try_series(tree, child) {
            println!("{:#?}", series);
        } else if let Some(movie) = try_movie(tree, child) {
            println!("{:#?}", movie);
        } else {
            let movies = try_movie_pack(tree, child);
            if movies.len() > 0 {
                println!("moviepack {:#?}", movies);
            }
        }
    }
}
