use std::cmp::Ordering;

use nfa::*;
use tokens::parse_filename_clean;
use util::parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Episode {
    series: String,
    number: i32,
    season: Option<i32>,
}

impl Episode {
    pub fn new<T, S>(series: T, number: i32, season: S) -> Episode
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
            sequence([regex("ep"), capture("episode", regex(r"(\d\d?)"))]),
            // capture("episode", regex(r"(\d\d?)")), TODO not expr?
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
            _ => lhs.series.cmp(&rhs.series),
        },
    );

    parses.pop()
}

#[test]
fn test_season_episode() {
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
        assert_eq!(ep.series, "southpark".to_string());
        println!("ok {}", stem);
    }
}

#[test]
fn test_episode() {
    let stems = ["southpark ep2", "southpark ep_2", "southpark e02"];
    for stem in stems.iter() {
        let ep = parse_episode(stem).unwrap();
        assert_eq!(ep.season, None);
        assert_eq!(ep.number, 2);
        assert_eq!(ep.series, "southpark".to_string());
        println!("ok {}", stem);
    }
}

#[test]
fn test_ambiguous() {
    let ep = parse_episode("19-2 s01e01").unwrap();
    assert_eq!(ep.season, Some(1));
    assert_eq!(ep.number, 1);
    assert_eq!(ep.series, "19 2".to_string());
}

#[test]
fn test_invalid() {
    assert_eq!(parse_episode("blade runner 2049 (2017)"), None);
}
