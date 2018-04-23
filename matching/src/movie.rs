use std::cmp::Ordering;

use nfa::*;
use tokens::parse_filename_clean;
use util::parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Movie {
    title: String,
    year: Option<i32>,
}

lazy_static! {
    pub static ref MOVIE_MATCHER: Matcher = Matcher::new(sequence([
        capture("title", many1(regex(r"\w+"))),
        capture("year", maybe(year())),
        many0(regex(r"\w+")),
    ]));
}

pub fn parse_movie(filename: &str) -> Option<Movie> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text).collect();

    let mut parses: Vec<Movie> = vec![];

    for cap in MOVIE_MATCHER.captures(&tokens) {
        let title: String = cap.concat("title");
        let year = cap.tokens("year").next().map(|s| parse(s));
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

    // println!("sorted parses {:#?}", parses);

    parses.into_iter().next()
}
