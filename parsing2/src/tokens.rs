use std::ops::Deref;

use metadata;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Scope {
    Normal,
    Parens,
    Square,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub text: String,
    pub scope: Scope,
}

impl Token {
    pub fn new<T>(text: T, scope: Scope) -> Token
    where
        T: Into<String>,
    {
        Token {
            text: text.into(),
            scope,
        }
    }

    pub fn normal<T>(text: T) -> Token
    where
        T: Into<String>,
    {
        Token::new(text, Scope::Normal)
    }

    pub fn parens<T>(text: T) -> Token
    where
        T: Into<String>,
    {
        Token::new(text, Scope::Parens)
    }

    pub fn square<T>(text: T) -> Token
    where
        T: Into<String>,
    {
        Token::new(text, Scope::Square)
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.text
    }
}

impl Deref for Token {
    type Target = str;

    fn deref(&self) -> &str {
        &self.text
    }
}

/// Convert a file name into a list of tokens.
///
/// Name is lowercased.
fn parse_filename(name: &str) -> Vec<Token> {
    let name = name.to_lowercase(); // TODO opt
    let mut tokens = Vec::new();
    let mut current_scope = Scope::Normal;
    let mut pos = 0;

    for (idx, car) in name.char_indices() {
        match car {
            ' ' | '.' | '_' | '-' | ':' | '(' | ')' | '[' | ']' => {
                // TODO: remove -
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

fn parse_filename_clean(name: &str) -> Vec<Token> {
    let tokens = parse_filename(name);

    let first_normal = tokens.iter().position(|token| token.scope == Scope::Normal);
    let first_metadata = tokens
        .iter()
        .position(|token| metadata::ALL.contains(token.text.as_str()));

    let first_normal = first_normal.unwrap_or(0);
    let first_metadata = first_metadata.unwrap_or(tokens.len());

    tokens[first_normal..first_metadata].to_vec()
}

pub fn join<I, G, P>(pieces: I, glue: G) -> String
where
    I: IntoIterator<Item = P>,
    P: AsRef<str>,
    G: AsRef<str>,
{
    let mut buff = String::new();
    let mut pieces = pieces.into_iter();
    let glue = glue.as_ref();
    if let Some(first) = pieces.next() {
        buff.push_str(first.as_ref());
        for piece in pieces {
            buff.push_str(glue);
            buff.push_str(piece.as_ref());
        }
    }
    buff
}

pub fn convert_filename(filename: &str) -> String {
    let tokens = parse_filename_clean(filename);
    join(tokens, " ")
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
            Token::normal("american"),
            Token::normal("psycho"),
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

#[test]
fn test_parse_clean() {
    let tokens = parse_filename_clean("[foo].bar.1080p");
    assert_eq!(tokens, vec![Token::normal("bar")]);
}
