use std::cmp::Ordering;

use nfa::*;
use tokens::parse_filename_clean;
use util::parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Movie {
    title: String,
    year: Option<i32>,
}

impl Movie {
    pub fn new<T, Y>(title: T, year: Y) -> Movie
    where
        T: Into<String>,
        Y: Into<Option<i32>>,
    {
        Movie {
            title: title.into(),
            year: year.into(),
        }
    }
}

lazy_static! {
    pub static ref MOVIE_MATCHER: Matcher = Matcher::new(sequence([
        capture("title", many1(regex(r".+"))),
        capture("year", maybe(year())),
        many0(regex(r".+")),
    ]));
}

pub fn parse_movie(filename: &str) -> Option<Movie> {
    let tokens = parse_filename_clean(filename);
    let tokens: Vec<&str> = tokens.iter().map(|t| t.text.as_str()).collect();

    // println!("tokens {:?}", tokens);

    let mut parses: Vec<Movie> = vec![];

    for cap in MOVIE_MATCHER.captures(&tokens) {
        let title: String = cap.concat("title");
        let year = cap.tokens("year").next().map(|s| parse(s));
        parses.push(Movie::new(title, year));
    }

    parses.sort_by(|lhs, rhs| {
        if lhs.year.is_some() && rhs.year.is_none() {
            Ordering::Greater
        } else if lhs.year.is_none() && rhs.year.is_some() {
            Ordering::Less
        } else {
            lhs.title.len().cmp(&rhs.title.len())
        }
    });

    // println!("sorted parses {:#?}", parses);

    parses.pop()
}

#[cfg(test)]
mod tests {
    use super::Movie;

    fn parse_movie(filename: &str) -> Movie {
        super::parse_movie(filename).unwrap()
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            parse_movie("Groundhog Day"),
            Movie::new("groundhog day", None)
        );
        assert_eq!(parse_movie("Snatch! 2005"), Movie::new("snatch!", 2005));
        assert_eq!(parse_movie("Snatch! (2005)"), Movie::new("snatch!", 2005));
        assert_eq!(parse_movie("Snatch! [2005]"), Movie::new("snatch!", 2005));
    }

    #[test]
    fn test_ambiguous_year() {
        assert_eq!(parse_movie("2011 1968"), Movie::new("2011", 1968));
        assert_eq!(parse_movie("2011"), Movie::new("2011", None));
    }

    #[test]
    fn test_metadata() {
        assert_eq!(
            parse_movie("Truman Show 1998 1080p.mkv"),
            Movie::new("truman show", 1998),
        );
        assert_eq!(
            parse_movie("Truman Show 1080p.mkv"),
            Movie::new("truman show", None),
        );
    }

    #[test]
    fn test_skip_nonwords() {
        assert_eq!(
            parse_movie("[psycho] Snatch! 2005"),
            Movie::new("snatch!", 2005)
        );
    }

    #[test]
    fn test_year_within_scope() {
        assert_eq!(
            parse_movie("Night Of The Living Dead (1968 - Widescreen)"),
            Movie::new("night of the living dead", 1968)
        )
    }

}
