use matching::movie::{parse_movie, Movie};
use matching::tv::Episode;
use matching::tv::{parse_episode, parse_season, parse_series};

use fs::Entry;
use tree::{Node, Tree};

#[derive(Debug)]
pub struct Season {
    pub series: String,
    pub episodes: Vec<Episode>,
    pub number: i32,
}

#[derive(Debug)]
pub struct Series {
    pub name: String,
    pub seasons: Vec<Season>,
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
    let mut episodes = vec![];
    for child in tree.children(node) {
        if let Some(ep) = try_episode(tree, child) {
            episodes.push(ep);
        }
    }

    episodes.sort_by_key(|ep| ep.number);

    if let (Some(season), false) = (
        parse_season(tree.data(node).stem().unwrap()),
        episodes.is_empty(),
    ) {
        return Some(Season {
            number: season.number,
            series: season.series, // TODO: better way of picking series name
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

    if let (Some(series), false) = (
        parse_series(tree.data(node).stem().unwrap()),
        seasons.is_empty(),
    ) {
        return Some(Series {
            name: series.name, // TODO: better way of picking series name
            seasons: seasons,
        });
    }
    None
}

pub fn try_movie(tree: &Tree<Entry>, node: Node) -> Option<Movie> {
    let entry = tree.data(node);
    if entry.is_video() {
        return parse_movie(entry.stem().unwrap());
    } else if entry.is_dir() {
        // Collect all the videos inside the directory.
        // TODO: filter extras, samples and release videos.
        let entries: Vec<_> = tree.children(node)
            .map(|child| tree.data(child))
            .filter(|&entry| entry.is_video() && !entry.is_ignored())
            .collect();

        return match entries.len() {
            1 => parse_movie(entries[0].stem().unwrap()),
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
