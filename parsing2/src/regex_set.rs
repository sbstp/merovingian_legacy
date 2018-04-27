use std::borrow::Cow;

use regex::{Captures, Regex};

pub fn regex(expr: &str) -> Regex {
    // This does automatic anchoring when anchors are not present.
    // We want the full token to be matched, not a substring of it.
    let anchored: Cow<str> = match expr.starts_with("^") {
        false => format!("^{}", expr).into(),
        _ => expr.into(),
    };
    Regex::new(&anchored).expect("invalid regex")
}

pub struct RegexSet<T> {
    set: Vec<Regex>,
    func: Box<Fn(Captures) -> T + Sync + 'static>,
}

impl<T> RegexSet<T> where {
    pub fn new<I, A, F>(set: I, func: F) -> Self
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

    pub fn apply(&self, text: &str) -> Option<T> {
        let func = &self.func;
        for rule in self.set.iter() {
            if let Some(caps) = rule.captures(text) {
                return Some(func(caps));
            }
        }
        None
    }
}

pub struct TieredSet<T> {
    tiers: Vec<RegexSet<T>>,
}

impl<T> TieredSet<T> {
    pub fn new(tiers: Vec<RegexSet<T>>) -> Self {
        TieredSet { tiers }
    }

    pub fn apply(&self, text: &str) -> Option<T> {
        for tier in self.tiers.iter() {
            if let Some(item) = tier.apply(text) {
                return Some(item);
            }
        }
        None
    }
}
