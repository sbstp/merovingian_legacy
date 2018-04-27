#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate regex;

mod metadata;
mod tokens;

use std::borrow::Cow;

use regex::{Captures, Regex};

fn regex(expr: &str) -> Regex {
    // This does automatic anchoring when anchors are not present.
    // We want the full token to be matched, not a substring of it.
    let anchored: Cow<str> = match expr.starts_with("^") {
        false => format!("^{}", expr).into(),
        _ => expr.into(),
    };
    Regex::new(&anchored).expect("invalid regex")
}

struct RegexSet<T> {
    set: Vec<Regex>,
    func: Box<Fn(Captures) -> T + Sync + 'static>,
}

impl<T> RegexSet<T> where {
    fn new<I, A, F>(set: I, func: F) -> Self
    where
        I: IntoIterator<Item = A>,
        A: AsRef<str>,
        F: Fn(Captures) -> T + Sync + 'static,
    {
        RegexSet {
            set: set.into_iter().map(|r| regex(r.as_ref())).collect(),
            func: Box::new(func),
        }
    }

    fn apply(&self, text: &str) -> Option<T> {
        let func = &self.func;
        for rule in self.set.iter() {
            if let Some(caps) = rule.captures(text) {
                return Some(func(caps));
            }
        }
        None
    }
}

struct TieredSet<T> {
    tiers: Vec<RegexSet<T>>,
}

impl<T> TieredSet<T> {
    fn new(tiers: Vec<RegexSet<T>>) -> Self {
        TieredSet { tiers }
    }

    fn apply(&self, text: &str) -> Option<T> {
        for tier in self.tiers.iter() {
            if let Some(item) = tier.apply(text) {
                return Some(item);
            }
        }
        None
    }
}

struct Episode {
    series: String,
    number: u32,
    season: Option<u32>,
}

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
                season: Some(caps[2].parse().unwrap()),
                number: caps[3].parse().unwrap(),
            },
        ),
        // try finding series + episode number
        RegexSet::new(&[
            r"(.+)\s(?:e|ep|episode)\s*(\d\d?)",
            r"(.+)\s(\d\d?)"
        ], |caps| Episode {
            series: caps[1].into(),
            season: None,
            number: caps[2].parse().unwrap(),
        }),
    ]);
    // static ref EPISODE: RegexSet<Episode, fn(Captures) -> Episode> =
}

fn parse_episode(filename: &str) -> Option<Episode> {
    let converted = tokens::convert_filename(filename);
    EPISODE.apply(&converted)
}

fn main() {}

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
