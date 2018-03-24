use std::cmp;

use super::metadata;
use super::{is_year, parse_filename, Scope};

/// Try to extract title and year from filename.
///
/// Usually, the title is placed before the year. There are cases where the movie's name has a year.
/// In that case, use right most year found is assumed to be the movie's release year. If nothing
/// occurs before the year found, the year is assumed to be the movie's title, such as
/// '2001: A Space Odyssey.mp4'.
///
/// If a metadata token is found, the title is assumed to stop before the metadata token. So the title
/// is everything before the year or the first metadata token.
///
/// There are also cases where a releases' name shows up before the title, such as '[foobar] The Matrix.mp4',
/// everything inside square brackets or parens before any normal word is ignored.
pub fn parse_movie(filename: &str) -> (String, Option<i32>) {
    let filename = filename.to_lowercase();
    let tokens = parse_filename(&filename);

    let mut first_word_index = None;
    let mut year_candidates = vec![];
    let mut first_metadata_token = None;

    for (idx, token) in tokens.iter().enumerate() {
        if first_word_index.is_none() && token.scope == Scope::Normal {
            first_word_index = Some(idx);
        }
        if is_year(token) {
            year_candidates.push(idx);
        }
        if first_metadata_token.is_none() {
            if metadata::ALL.contains(&token.text) {
                first_metadata_token = Some(idx);
            }
        }
    }

    let first_word_index = first_word_index.unwrap_or(0); // Error maybe?
    let first_metadata_token = first_metadata_token.unwrap_or(tokens.len());

    let mut title_tokens = &tokens[first_word_index..first_metadata_token];
    let mut year = None;

    if let Some(&year_idx) = year_candidates.last() {
        let min_idx = cmp::min(year_idx, first_metadata_token);
        let new_title_tokens = &tokens[first_word_index..min_idx];
        if !new_title_tokens.is_empty() {
            title_tokens = new_title_tokens;
            year = Some(tokens[year_idx].parse().unwrap());
        }
    }

    (
        title_tokens
            .iter()
            .map(|t| t.text)
            .collect::<Vec<_>>()
            .join(" "),
        year,
    )
}

#[test]
fn test_simple() {
    assert_eq!(parse_movie("Groundhog Day"), ("groundhog day".into(), None));
    assert_eq!(parse_movie("Snatch! 2005"), ("snatch!".into(), Some(2005)));
    assert_eq!(
        parse_movie("Snatch! (2005)"),
        ("snatch!".into(), Some(2005))
    );
    assert_eq!(
        parse_movie("Snatch! [2005]"),
        ("snatch!".into(), Some(2005))
    );
}

#[test]
fn test_ambiguous_year() {
    assert_eq!(parse_movie("2011 1968"), ("2011".into(), Some(1968)));
    assert_eq!(parse_movie("2011"), ("2011".into(), None));
}

#[test]
fn test_metadata() {
    assert_eq!(
        parse_movie("Truman Show 1080p 1998.mkv"),
        ("truman show".into(), Some(1998))
    );
    assert_eq!(
        parse_movie("Truman Show 1080p.mkv"),
        ("truman show".into(), None)
    );
}

#[test]
fn test_skip_nonwords() {
    assert_eq!(
        parse_movie("[psycho] Snatch! 2005"),
        ("snatch!".into(), Some(2005))
    );
}

#[test]
fn test_year_within_scope() {
    assert_eq!(
        parse_movie("Night Of The Living Dead (1968 - Widescreen)"),
        ("night of the living dead".into(), Some(1968))
    )
}
