use std::ops::Deref;

pub mod metadata;
pub mod movie;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Scope {
    Normal,
    Parens,
    Square,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub text: &'a str,
    pub scope: Scope,
}

impl<'a> Token<'a> {
    pub fn new(text: &str, scope: Scope) -> Token {
        Token { text, scope }
    }

    pub fn normal(text: &str) -> Token {
        Token::new(text, Scope::Normal)
    }

    pub fn parens(text: &str) -> Token {
        Token::new(text, Scope::Parens)
    }

    pub fn square(text: &str) -> Token {
        Token::new(text, Scope::Square)
    }
}

impl<'a> Deref for Token<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        self.text
    }
}

pub fn parse_filename(name: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_scope = Scope::Normal;
    let mut pos = 0;

    for (idx, car) in name.char_indices() {
        match car {
            ' ' | '.' | '_' | '-' | ':' | '(' | ')' | '[' | ']' => {
                let text = &name[pos..idx];
                if !text.is_empty() {
                    tokens.push(Token::new(text, current_scope));
                }

                current_scope = match car {
                    '(' => Scope::Parens,
                    ')' => Scope::Normal,
                    '[' => Scope::Square,
                    ']' => Scope::Normal,
                    _ => current_scope,
                };

                pos = idx + car.len_utf8();
            }
            _ => {}
        }
    }

    let text = &name[pos..];
    if !text.is_empty() {
        tokens.push(Token::new(text, current_scope));
    }

    tokens
}

pub fn is_year(token: &str) -> bool {
    return token.len() == 4 && token.chars().all(|c| char::is_digit(c, 10));
}

#[test]
fn test_is_year() {
    assert!(is_year("2009"));
    assert!(!is_year("1080p"));
}

#[test]
fn test_split_tokens() {
    assert_eq!(
        parse_filename("this.file_name-uses:every separator"),
        vec![
            Token::normal("this"),
            Token::normal("file"),
            Token::normal("name"),
            Token::normal("uses"),
            Token::normal("every"),
            Token::normal("separator"),
        ]
    );

    assert_eq!(
        parse_filename("foo.-_ .:bar"),
        vec![Token::normal("foo"), Token::normal("bar")]
    );
}

#[test]
fn test_parse_filename_simple() {
    let tokens = parse_filename("american psycho");
    assert_eq!(
        tokens,
        vec![Token::normal("american"), Token::normal("psycho")]
    );
}

#[test]
fn test_parse_filename_parens_square() {
    let tokens = parse_filename("American.Psycho.(2000).[1080p]");
    assert_eq!(
        tokens,
        vec![
            Token::normal("American"),
            Token::normal("Psycho"),
            Token::parens("2000"),
            Token::square("1080p"),
        ]
    );
}

#[test]
fn test_parse_filename_ambiguous() {
    let tokens = parse_filename("[release name] foobar (1999)");
    assert_eq!(
        tokens,
        vec![
            Token::square("release"),
            Token::square("name"),
            Token::normal("foobar"),
            Token::parens("1999"),
        ]
    );
}

// #[test]
// fn test_parse_filename_incomplete() {
//     let tokens = parse_filename("foo (bar");
//     assert_eq!(
//         tokens,
//         vec![Token::normal("foo"), Token::Word("bar".into())]
//     );

//     let tokens = parse_filename("foo [bar");
//     assert_eq!(
//         tokens,
//         vec![Token::Word("foo".into()), Token::Word("bar".into())]
//     );
// }
