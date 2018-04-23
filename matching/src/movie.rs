use std::cmp::Ordering;

use nfa::*;
use tokens::parse_filename_clean;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Movie {
    title: String,
    year: Option<i32>,
}

pub fn parse_movie(filename: &str) -> Option<Movie> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text).collect();

    let m = Matcher::new(sequence([
        capture("title", many1(regex("\\w+"))),
        capture("year", maybe(year())),
    ]));

    let mut parses: Vec<Movie> = vec![];

    for cap in m.captures(&tokens) {
        let title: String = cap.tokens("title")
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let year = cap.tokens("year").next().map(|s| s.parse().unwrap());
        parses.push(Movie { title, year });
    }

    parses.sort_by(|lhs, rhs| {
        if lhs.year.is_some() && rhs.year.is_none() {
            Ordering::Less
        } else if lhs.year.is_none() && rhs.year.is_some() {
            Ordering::Greater
        } else {
            lhs.title.len().cmp(&rhs.title.len())
        }
    });

    println!("sorted parses {:#?}", parses);

    parses.into_iter().next()
}
