use std::cmp::Ordering;

use nfa::*;
use tokens::parse_filename_clean;
use util::parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Episode {
    pub series: String,
    pub number: i32,
    pub season: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Season {
    pub series: String,
    pub number: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Series {
    pub name: String,
}

impl Episode {
    pub fn new<T, S>(series: T, number: i32, season: S) -> Self
    where
        T: Into<String>,
        S: Into<Option<i32>>,
    {
        Episode {
            series: series.into(),
            number,
            season: season.into(),
        }
    }
}

impl Season {
    pub fn new<T>(series: T, number: i32) -> Self
    where
        T: Into<String>,
    {
        Season {
            series: series.into(),
            number,
        }
    }
}

impl Series {
    pub fn new<T>(series: T) -> Self
    where
        T: Into<String>,
    {
        Series {
            name: series.into(),
        }
    }
}

lazy_static! {
    pub static ref EPISODE_MATCHER: Matcher = Matcher::new(sequence([
        capture("series", many0(regex(r".+"))),
        or([
            capture("season_episode", regex(r"s(\d\d?)e(\d\d?)")),
            capture("season_episode", regex(r"(\d\d?)x(\d\d?)$")),
            capture("season_episode", regex(r"(\d)(\d\d)")),
            sequence([
                capture("season", regex(r"s(\d\d?)")),
                capture("episode", regex(r"e(\d\d?)")),
            ]),
            sequence([
                capture("season", regex(r"(\d\d?)")),
                capture("episode", regex(r"(\d\d?)")),
            ]),
            capture("episode", regex(r"ep?(\d\d?)")),
            sequence([regex("ep(?:isode)?"), capture("episode", regex(r"(\d\d?)"))]),
        ]),
        many0(regex(r".+")),
    ]));
    pub static ref SEASON_MATCHER: Matcher = Matcher::new(sequence([
        capture("series", many0(regex(r".+"))),
        or([
            sequence([regex("season|saison"), capture("season", regex(r"(\d\d?)"))]),
            capture("season", regex(r"s(\d\d?)")),
        ]),
        many0(regex(r".+")),
    ]));
    pub static ref SERIES_MATCHER: Matcher = Matcher::new(sequence([
        capture("series", many1(regex(r".+"))),
        or([
            regex("complete"),
            sequence([regex(r"s(\d\d?)"), regex(r"s(\d\d?)")]),
        ]),
        many0(regex(r".+")),
    ]));
}

pub fn parse_episode(filename: &str) -> Option<Episode> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text.as_str()).collect();

    let mut parses: Vec<Episode> = vec![];

    let caps = EPISODE_MATCHER.captures(&tokens);

    for cap in caps {
        let series = cap.concat("series");
        if let Some(season_episode) = cap.first("season_episode") {
            // println!("season_episode={}, series={}", season_episode, series);
            parses.push(Episode::new(
                series,
                parse(season_episode.group(2)),
                Some(parse(season_episode.group(1))),
            ));
        } else if let (Some(season), Some(episode)) = (cap.first("season"), cap.first("episode")) {
            // println!("season={}, episode={}, series={}", season, episode, series);
            parses.push(Episode::new(
                series,
                parse(episode.group(1)),
                Some(parse(season.group(1))),
            ));
        } else if let Some(episode) = cap.first("episode") {
            // println!("episode={}, series={}", episode, series);
            parses.push(Episode::new(series, parse(episode.group(1)), None));
        }
    }

    // println!("sorted parses {:#?}", parses);

    parses.sort_by(
        |lhs, rhs| match (lhs.season.is_some(), rhs.season.is_some()) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => lhs.series.len().cmp(&rhs.series.len()),
        },
    );

    parses.pop()
}

pub fn parse_season(filename: &str) -> Option<Season> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text.as_str()).collect();

    let mut parses: Vec<Season> = vec![];

    let caps = SEASON_MATCHER.captures(&tokens);

    for cap in caps {
        let series = cap.concat("series");
        if let Some(season) = cap.first("season") {
            parses.push(Season::new(series, parse(season.group(1))));
        }
    }

    // println!("sorted parses {:#?}", parses);

    parses.sort_by(|lhs, rhs| lhs.series.len().cmp(&rhs.series.len()));

    parses.pop()
}

pub fn parse_series(filename: &str) -> Option<Series> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text.as_str()).collect();

    let mut parses: Vec<Series> = vec![];

    let caps = SERIES_MATCHER.captures(&tokens);

    for cap in caps {
        let name = cap.concat("series");
        parses.push(Series::new(name));
    }

    // println!("sorted parses {:#?}", parses);

    parses.sort_by(|lhs, rhs| lhs.name.len().cmp(&rhs.name.len()));

    // take first, shortest series name
    if !parses.is_empty() {
        Some(parses.remove(0))
    } else {
        None
    }
}

#[test]
fn test_episode_with_season() {
    let stems = [
        "southpark s01e02",
        "southpark s1e2",
        "southpark s01.e02",
        "southpark s01_e02",
        "southpark s01 e02",
        "southpark s1 e2",
        "southpark 1x2",
        "southpark 1x02",
        "southpark 01x02",
        "southpark 1_02",
        "southpark 1.02",
        "southpark 102",
    ];
    for stem in stems.iter() {
        let ep = parse_episode(stem).unwrap();
        assert_eq!(ep.season, Some(1));
        assert_eq!(ep.number, 2);
        assert_eq!(ep.series, "southpark");
        println!("ok {}", stem);
    }
}

#[test]
fn test_episode() {
    let stems = [
        "southpark ep2",
        "southpark ep_2",
        "southpark e02",
        "southpark episode 2",
    ];
    for stem in stems.iter() {
        let ep = parse_episode(stem).unwrap();
        assert_eq!(ep.season, None);
        assert_eq!(ep.number, 2);
        assert_eq!(ep.series, "southpark");
        println!("ok {}", stem);
    }
}

#[test]
fn test_episode_ambiguous() {
    let ep = parse_episode("19-2 s01e01").unwrap();
    assert_eq!(ep.season, Some(1));
    assert_eq!(ep.number, 1);
    assert_eq!(ep.series, "19 2");
}

#[test]
fn test_episode_invalid() {
    assert_eq!(parse_episode("blade runner 2049 (2017)"), None);
}

#[test]
fn test_season() {
    let stems = ["southpark saison 2", "southpark season 2", "southpark s02"];
    for stem in stems.iter() {
        let season = parse_season(stem).unwrap();
        assert_eq!(season.number, 2);
        assert_eq!(season.series, "southpark");
        println!("ok {}", stem);
    }
}

#[test]
fn test_series() {
    let stems = ["southpark s01-s02", "southpark complete"];
    for stem in stems.iter() {
        let series = parse_series(stem).unwrap();
        assert_eq!(series.name, "southpark");
        println!("ok {}", stem);
    }
}
