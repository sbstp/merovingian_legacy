use super::parse_filename;

pub fn parse_movie(filename: &str) -> (String, Option<i32>) {
    let tokens = parse_filename(filename);
    let mut first_word_index = None;
    let mut year_candidates = vec![];

    for (idx, token) in tokens.iter().enumerate() {
        if first_word_index.is_none() && token.is_word() {
            first_word_index = Some(idx);
        }
        if token.is_year() {
            year_candidates.push(idx);
        }
    }

    let first_word_index = first_word_index.unwrap_or(0); // Error maybe?

    let mut title_tokens = &tokens[first_word_index..];
    let mut year = None;

    if let Some(&year_idx) = year_candidates.last() {
        title_tokens = &tokens[first_word_index..year_idx];
        year = Some(tokens[year_idx].parse().unwrap())
    }

    (
        title_tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" "),
        year,
    )
}

#[test]
fn test_simple() {
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
}

#[test]
fn test_skip_nonwords() {
    assert_eq!(
        parse_movie("[psycho] Snatch! 2005"),
        ("snatch!".into(), Some(2005))
    );
}
