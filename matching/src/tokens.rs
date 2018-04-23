use std::ops::Deref;

use metadata;

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

pub fn parse_filename_clean(name: &str) -> Vec<Token> {
    let tokens = parse_filename(name);

    let first_normal = tokens.iter().position(|token| token.scope == Scope::Normal);
    let first_metadata = tokens
        .iter()
        .position(|token| metadata::ALL.contains(token.text));

    let first_normal = first_normal.unwrap_or(0);
    let first_metadata = first_metadata.unwrap_or(tokens.len());

    tokens[first_normal..first_metadata].to_vec()
}
