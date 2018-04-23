use std::cmp::Ordering;

use nfa::*;
use tokens::parse_filename_clean;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Episode {
    series: String,
    number: i32,
    season: i32,
}

pub fn parse_episode(filename: &str) -> Option<Episode> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text).collect();

    let m = Matcher::new(sequence([
        capture("series", many1(regex(r"\w+"))),
        or([
            capture("season_episode", regex(r"^s\d\d?e\d\d?$")),
            sequence([
                capture("season", regex("s\\d\\d?")),
                capture("episode", regex("e\\d\\d?")),
            ]),
            sequence([
                capture("season", regex("\\d\\d?")),
                capture("episode", regex("\\d\\d?")),
            ]),
        ]),
    ]));

    let mut parses: Vec<Episode> = vec![];

    for cap in m.captures(&tokens) {
        // println!("{:#?}", cap);
        let title: String = cap.tokens("series")
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("tit {}", title);
        if let Some(tokens) = cap.tokens("season_episode").next() {
            println!("season_episode {:?}", tokens);
        } else if let Some(tokens) = cap.tokens("season").next() {
            println!("season {:?}", tokens);
        } else if let Some(tokens) = cap.tokens("episode").next() {
            println!("episode {:?}", tokens);
        }
        println!("-----");
    }

    parses.sort_by(|lhs, rhs| Ordering::Equal);

    println!("sorted parses {:#?}", parses);

    parses.into_iter().next()
}
