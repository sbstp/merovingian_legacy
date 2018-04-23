use std::cmp::Ordering;

use nfa::*;
use tokens::parse_filename_clean;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Episode {
    series: String,
    number: i32,
    season: Option<i32>,
}

pub fn parse_episode(filename: &str) -> Option<Episode> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text).collect();

    let m = Matcher::new(sequence([
        capture("series", many0(regex(r"^\w+$"))),
        or([
            capture("season_episode", regex(r"^s\d\d?e\d\d?$")),
            capture("season_episode", regex(r"^\d\d?x\d\d?$")),
            sequence([
                capture("season", regex(r"^s\d\d?$")),
                capture("episode", regex(r"^e\d\d?$")),
            ]),
            sequence([
                capture("season", regex(r"^\d\d?$")),
                capture("episode", regex(r"^\d\d?$")),
            ]),
            capture("episode", regex(r"^e\d\d?$")),
        ]),
    ]));

    let mut parses: Vec<Episode> = vec![];

    let caps = m.captures(&tokens);

    for cap in caps {
        let series = cap.concat("series");
        if let Some(season_episode) = cap.first("season_episode") {
            println!("season_episode={}, series={}", season_episode, series);
        } else if let (Some(season), Some(episode)) = (cap.first("season"), cap.first("episode")) {
            println!("season={}, episode={}, series={}", season, episode, series);
        } else if let Some(episode) = cap.first("episode") {
            println!("episode={}, series={}", episode, series);
        }
    }

    parses.sort_by(|lhs, rhs| Ordering::Equal);

    println!("sorted parses {:#?}", parses);

    parses.into_iter().next()
}
