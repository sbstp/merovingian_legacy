use regex_set::{RegexSet, TieredSet};
use tokens;
use util::parse;

lazy_static! {
    static ref MOVIE: TieredSet<Movie> = TieredSet::new(vec![
        // try finding series + season + episode number
        RegexSet::new(
            &[
                r"(.+)\s(\d{4})",
            ],
            |caps| Movie {
                title: caps[1].into(),
                year: Some(parse(&caps[2])),
            },
        ),
        // try finding series + episode number
        RegexSet::new(&[
                r"(.+)",
            ],
            |caps| Movie {
                title: caps[1].into(),
                year: None,
            },
        ),
    ]);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Movie {
    title: String,
    year: Option<u32>,
}

impl Movie {
    pub fn new<T, Y>(title: T, year: Y) -> Movie
    where
        T: Into<String>,
        Y: Into<Option<u32>>,
    {
        Movie {
            title: title.into(),
            year: year.into(),
        }
    }
}
pub fn parse_movie(filename: &str) -> Option<Movie> {
    let converted = tokens::convert_filename(filename);
    MOVIE.apply(&converted)
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
