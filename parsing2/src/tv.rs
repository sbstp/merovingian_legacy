use regex_set::{RegexSet, TieredSet};
use tokens;
use util::parse;

lazy_static! {
    static ref EPISODE: TieredSet<Episode> = TieredSet::new(vec![
        // try finding series + season + episode number
        RegexSet::new(
            &[
                r"(.+)\ss(\d\d?)\s*e(\d\d?)",
                r"(.+)\s(\d\d?)[x\s](\d\d?)",
                r"(.+)\s(\d)(\d\d)",
            ],
            |caps| Episode {
                series: caps[1].into(),
                season: Some(parse(&caps[2])),
                number: parse(&caps[3]),
            },
        ),
        // try finding series + episode number
        RegexSet::new(&[
            r"(.+)\s(?:e|ep|episode)\s*(\d\d?)",
            r"(.+)\s(\d\d?)"
        ], |caps| Episode {
            series: caps[1].into(),
            season: None,
            number: parse(&caps[2]),
        }),
    ]);
}

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

pub fn parse_episode(filename: &str) -> Option<Episode> {
    let converted = tokens::convert_filename(filename);
    EPISODE.apply(&converted)
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
        println!("trying {}", stem);
        let ep = parse_episode(stem).unwrap();
        assert_eq!(ep.season, Some(1));
        assert_eq!(ep.number, 2);
        assert_eq!(ep.series, "southpark");
    }
}

#[test]
fn test_episode() {
    let stems = [
        "southpark ep2",
        "southpark ep_2",
        "southpark e02",
        "southpark episode 2",
        "southpark 02",
    ];
    for stem in stems.iter() {
        println!("trying {}", stem);
        let ep = parse_episode(stem).unwrap();
        assert_eq!(ep.season, None);
        assert_eq!(ep.number, 2);
        assert_eq!(ep.series, "southpark");
    }
}
