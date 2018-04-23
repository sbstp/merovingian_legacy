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
    pub fn new(series: String, number: i32, season: Option<i32>) -> Episode {
        Episode {
            series,
            number,
            season,
        }
    }
}

lazy_static! {
    pub static ref EPISODE_MATCHER: Matcher = Matcher::new(sequence([
        capture("series", many0(regex(r"^\w+$"))),
        or([
            capture("season_episode", regex(r"s(\d\d?)e(\d\d?)")),
            capture("season_episode", regex(r"(\d\d?)x(\d\d?)$")),
            sequence([
                capture("season", regex(r"s(\d\d?)")),
                capture("episode", regex(r"e(\d\d?)")),
            ]),
            sequence([
                capture("season", regex(r"(\d\d?)")),
                capture("episode", regex(r"(\d\d?)")),
            ]),
            capture("episode", regex(r"e(\d\d?)")),
            capture("episode", regex(r"(\d\d?)")),
        ]),
        many0(regex(r"\w+")),
    ]));
}

pub fn parse_episode(filename: &str) -> Option<Episode> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text).collect();

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
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => lhs.series.cmp(&rhs.series),
        },
    );

    parses.into_iter().next()
}
