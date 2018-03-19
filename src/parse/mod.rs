use std::ops::Deref;

pub mod metadata;
pub mod movie;

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    Parens(String),
    Square(String),
}

impl<'t> Token {
    pub fn is_word(&self) -> bool {
        match *self {
            Token::Word(_) => true,
            _ => false,
        }
    }

    pub fn is_year(&self) -> bool {
        return self.len() == 4 && self.chars().all(|c| char::is_digit(c, 10));
    }
}

impl Deref for Token {
    type Target = str;

    fn deref(&self) -> &str {
        match *self {
            Token::Word(ref s) => s,
            Token::Parens(ref s) => s,
            Token::Square(ref s) => s,
        }
    }
}

pub fn parse_filename(name: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut in_parens = false;
    let mut in_square = false;
    let mut pos = 0;

    for (idx, car) in name.char_indices() {
        let mut move_pos = true;
        let mut token = None;

        match car {
            ' ' | '.' | '_' | '-' if !in_parens && !in_square => {
                token = Some(Token::Word(name[pos..idx].to_lowercase()));
            }
            '(' => {
                in_parens = true;
            }
            ')' if in_parens => {
                in_parens = false;
                token = Some(Token::Parens(name[pos..idx].to_lowercase()));
            }
            '[' => {
                in_square = true;
            }
            ']' if in_square => {
                in_square = false;
                token = Some(Token::Square(name[pos..idx].to_lowercase()))
            }
            _ => {
                move_pos = false;
            }
        }

        if move_pos {
            pos = idx + car.len_utf8();
        }

        if let Some(token) = token {
            if !token.is_empty() {
                tokens.push(token);
            }
        }
    }

    let tok = name[pos..].to_lowercase();
    if !tok.is_empty() {
        tokens.push(Token::Word(tok));
    }

    tokens
}

#[test]
fn test_token_is_year() {
    assert!(Token::Square("2009".into()).is_year());
    assert!(!Token::Word("1080p".into()).is_year());
}

#[test]
fn test_parse_filename_simple() {
    let tokens = parse_filename("American Psycho");
    assert_eq!(
        tokens,
        vec![Token::Word("american".into()), Token::Word("psycho".into())]
    );
}

#[test]
fn test_parse_filename_parens_square() {
    let tokens = parse_filename("American.Psycho.(2000).[1080p]");
    assert_eq!(
        tokens,
        vec![
            Token::Word("american".into()),
            Token::Word("psycho".into()),
            Token::Parens("2000".into()),
            Token::Square("1080p".into()),
        ]
    );
}

#[test]
fn test_parse_filename_ambiguous() {
    let tokens = parse_filename("[release name] foobar (1999)");
    assert_eq!(
        tokens,
        vec![
            Token::Square("release name".into()),
            Token::Word("foobar".into()),
            Token::Parens("1999".into()),
        ]
    );
}

#[test]
fn test_parse_filename_incomplete() {
    let tokens = parse_filename("foo (bar");
    assert_eq!(
        tokens,
        vec![Token::Word("foo".into()), Token::Word("bar".into())]
    );

    let tokens = parse_filename("foo [bar");
    assert_eq!(
        tokens,
        vec![Token::Word("foo".into()), Token::Word("bar".into())]
    );
}
