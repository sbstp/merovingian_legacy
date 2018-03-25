use std::fmt;

use regex::{Captures, Regex};

use super::parse_ok;
use super::{parse_clean, Token};

lazy_static! {
    static ref SEASON_EPISODE_1: Regex = Regex::new(r"^s(\d\d?)e(\d\d?)$").unwrap();
    static ref SEASON_EPISODE_2: Regex = Regex::new(r"^(\d\d?)x(\d\d?)$").unwrap();
    static ref SEASON: Regex = Regex::new(r"^s(\d\d)$").unwrap();
    static ref EPISODE: Regex = Regex::new(r"^e(\d\d)$").unwrap();
    static ref TWO_DIGITS: Regex = Regex::new(r"^(\d\d?)$").unwrap();
    static ref THREE_DIGITS: Regex = Regex::new(r"^(\d)(\d\d)$").unwrap();
    static ref EPISODE_ONLY: Regex = Regex::new(r"^ep?(\d\d?)$").unwrap();
    static ref EPISODE_TAG: Regex = Regex::new(r"^(ep|episode)$").unwrap();
}

#[derive(PartialEq)]
pub struct Episode {
    pub series_name: String,
    pub season: Option<i32>,
    pub episode: i32,
}

impl Episode {
    fn new(season: Option<i32>, episode: i32, name: String) -> Episode {
        Episode {
            series_name: name,
            season,
            episode,
        }
    }
}

impl fmt::Debug for Episode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Episode(number={}, season={:?}, series={})",
            self.episode, self.season, self.series_name
        )
    }
}

fn match_single<'t>(tokens: &'t [Token], regex: &Regex) -> Option<(usize, Captures<'t>)> {
    for (idx, token) in tokens.iter().enumerate() {
        if let Some(caps) = regex.captures(token) {
            return Some((idx, caps));
        }
    }

    None
}

fn match_single_multi<'t>(
    tokens: &'t [Token],
    regexes: &[&Regex],
) -> Option<(usize, Captures<'t>)> {
    for regex in regexes {
        if let Some(result) = match_single(tokens, regex) {
            return Some(result);
        }
    }

    None
}
fn match_sequence<'t>(
    tokens: &'t [Token],
    first: &Regex,
    second: &Regex,
) -> Option<(usize, Captures<'t>, Captures<'t>)> {
    let mut iter = tokens.iter().enumerate().peekable();
    while let Some((idx, token)) = iter.next() {
        if let Some(caps) = first.captures(token) {
            if let Some(&(_, next_token)) = iter.peek() {
                if let Some(next_caps) = second.captures(next_token) {
                    return Some((idx, caps, next_caps));
                }
            }
        }
    }
    None
}

fn match_sequence_multi<'t>(
    tokens: &'t [Token],
    pairs: &[(&Regex, &Regex)],
) -> Option<(usize, Captures<'t>, Captures<'t>)> {
    for &(first, second) in pairs {
        if let Some(result) = match_sequence(tokens, first, second) {
            return Some(result);
        }
    }
    None
}

fn tokens_to_string(tokens: &[Token]) -> String {
    tokens.iter().map(|t| t.text).collect::<Vec<_>>().join(" ")
}

/// The stem must be lowercase.
pub fn parse_episode(stem: &str) -> Option<Episode> {
    let stem = stem.to_lowercase();
    let tokens: Vec<_> = parse_clean(&stem);

    // Try to match
    // s01e02
    // 01x02
    if let Some((idx, caps)) = match_single_multi(&tokens, &[&SEASON_EPISODE_1, &SEASON_EPISODE_2])
    {
        let season = parse_ok(&caps[1]);
        let episode = parse_ok(&caps[2]);
        let name = tokens_to_string(&tokens[..idx]);
        return Some(Episode::new(Some(season), episode, name));
    }

    // Try to match
    // 01.01
    if let Some((idx, caps, caps_next)) =
        match_sequence_multi(&tokens, &[(&SEASON, &EPISODE), (&TWO_DIGITS, &TWO_DIGITS)])
    {
        let season = parse_ok(&caps[1]);
        let episode = parse_ok(&caps_next[1]);
        let name = tokens_to_string(&tokens[..idx]);
        return Some(Episode::new(Some(season), episode, name));
    }

    // Try to match
    // 102
    if let Some((idx, caps)) = match_single(&tokens, &THREE_DIGITS) {
        let season = parse_ok(&caps[1]);
        let episode = parse_ok(&caps[2]);
        let name = tokens_to_string(&tokens[..idx]);
        return Some(Episode::new(Some(season), episode, name));
    }

    // Try to match
    // ep02
    if let Some((idx, caps)) = match_single(&tokens, &EPISODE_ONLY) {
        let episode = parse_ok(&caps[1]);
        let name = tokens_to_string(&tokens[..idx]);
        return Some(Episode::new(None, episode, name));
    }

    // Try to match
    // ep.02
    if let Some((idx, _, caps)) = match_sequence(&tokens, &EPISODE_TAG, &TWO_DIGITS) {
        let episode = parse_ok(&caps[1]);
        let name = tokens_to_string(&tokens[..idx]);
        return Some(Episode::new(None, episode, name));
    }

    None
}

#[test]
fn test_season_episode() {
    let stems = [
        "southpark s01e02",
        "southpark s1e2",
        "southpark s01.e02",
        "southpark s01_e02",
        "southpark s01 e02",
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
        assert_eq!(ep.episode, 2);
        assert_eq!(ep.series_name, "southpark".to_string());
        println!("ok {}", stem);
    }
}

#[test]
fn test_episode() {
    let stems = ["southpark ep2", "southpark ep_2", "southpark e02"];
    for stem in stems.iter() {
        let ep = parse_episode(stem).unwrap();
        assert_eq!(ep.season, None);
        assert_eq!(ep.episode, 2);
        assert_eq!(ep.series_name, "southpark".to_string());
        println!("ok {}", stem);
    }
}

#[test]
fn test_ambiguous() {
    let ep = parse_episode("19-2 s01e01").unwrap();
    assert_eq!(ep.season, Some(1));
    assert_eq!(ep.episode, 1);
    assert_eq!(ep.series_name, "19 2".to_string());
}

#[test]
fn test_invalid() {
    assert_eq!(parse_episode("blade runner 2049 (2017)"), None);
}
