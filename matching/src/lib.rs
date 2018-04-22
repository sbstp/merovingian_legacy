#![feature(nll)]

extern crate petgraph;
extern crate regex;
extern crate rpds;

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::mem;
use std::rc::Rc;

use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};
use regex::Regex;
use rpds::{HashTrieMap, Vector};

type NFA = DiGraph<PatternInfo, PatternInfo>;

#[derive(Clone, Debug)]
struct PatternInfo {
    pattern: Rc<Pattern>,
    group: Option<&'static str>,
}

impl PatternInfo {
    fn dont_match() -> Self {
        PatternInfo {
            pattern: Rc::new(Pattern::DontMatch),
            group: None,
        }
    }
}

fn build_nfa_rec(
    nfa: &mut NFA,
    prev_nodes: &[NodeIndex],
    next_nodes: &mut Vec<NodeIndex>,
    exp: &Expr,
    group: Option<&'static str>,
) {
    match exp {
        Expr::Pattern(pattern) => {
            let new_node = nfa.add_node(PatternInfo {
                pattern: pattern.clone(),
                group,
            });
            next_nodes.push(new_node);
        }
        Expr::Or(left, right) => {
            build_nfa_rec(nfa, prev_nodes, next_nodes, left, group);
            build_nfa_rec(nfa, prev_nodes, next_nodes, right, group);
        }
        Expr::Many0(inner) => {
            let mut tmp_next_nodes = vec![];
            build_nfa_rec(nfa, prev_nodes, &mut tmp_next_nodes, inner, group);
            for &i in tmp_next_nodes.iter() {
                for &j in tmp_next_nodes.iter() {
                    if !nfa.contains_edge(i, j) {
                        nfa.add_edge(i, j, nfa[j].clone());
                    }
                }
            }
            next_nodes.extend(tmp_next_nodes.iter().cloned());
            // links previous nodes are also next nodes for many0
            for &prev_node in prev_nodes.iter() {
                next_nodes.push(prev_node);
            }
        }
        Expr::Many1(inner) => {
            let mut tmp_next_nodes = vec![];
            build_nfa_rec(nfa, prev_nodes, &mut tmp_next_nodes, inner, group);
            for &i in tmp_next_nodes.iter() {
                for &j in tmp_next_nodes.iter() {
                    if !nfa.contains_edge(i, j) {
                        nfa.add_edge(i, j, nfa[j].clone());
                    }
                }
            }
            next_nodes.extend(tmp_next_nodes.iter().cloned());
        }
        Expr::Capture(group, inner) => {
            build_nfa_rec(nfa, prev_nodes, next_nodes, inner, Some(group))
        }
    }
}

/// Creates a NFA from Expr expressions without epsilon transitions.
fn build_nfa(exprs: &[Expr]) -> (NFA, NodeIndex) {
    let mut nfa = NFA::new();
    let start = nfa.add_node(PatternInfo::dont_match());
    let mut prev_nodes = vec![start];
    let mut next_nodes = vec![];
    for exp in exprs {
        build_nfa_rec(&mut nfa, &prev_nodes, &mut next_nodes, exp, None);

        for &prev_node in prev_nodes.iter() {
            for &next_node in next_nodes.iter() {
                if prev_node != next_node && !nfa.contains_edge(prev_node, next_node) {
                    let weight = nfa[next_node].clone();
                    nfa.add_edge(prev_node, next_node, weight);
                }
            }
        }

        mem::swap(&mut prev_nodes, &mut next_nodes);
        next_nodes.clear();
    }

    (nfa, start)
}

#[derive(Debug)]
pub enum Pattern {
    DontMatch,
    String(String),
    Regex(Regex),
}

impl Pattern {
    fn matches(&self, token: &str) -> bool {
        match self {
            Pattern::DontMatch => false,
            Pattern::String(s) => s == token,
            Pattern::Regex(r) => r.is_match(token),
        }
    }
}

pub enum Expr {
    Pattern(Rc<Pattern>),
    Or(Box<Expr>, Box<Expr>),
    Many0(Box<Expr>),
    Many1(Box<Expr>),
    Capture(&'static str, Box<Expr>),
}

/// Create an expression that matches the given string.
pub fn string<A: AsRef<str>>(s: A) -> Expr {
    Expr::Pattern(Rc::new(Pattern::String(s.as_ref().into())))
}

/// Create an expression that matches the given Regex.
pub fn regex(s: &'static str) -> Expr {
    Expr::Pattern(Rc::new(Pattern::Regex(Regex::new(s).unwrap())))
}

/// Create an expression that matches a year.
pub fn year() -> Expr {
    regex(r"\d{4}")
}

/// Create an expression that will match either the left or the right expression.
pub fn or(left: Expr, right: Expr) -> Expr {
    Expr::Or(Box::new(left), Box::new(right))
}

/// Create an expression that matches zero or more times the inner expression.
pub fn many0(inner: Expr) -> Expr {
    Expr::Many0(Box::new(inner))
}

/// Create an expression that matches one or more times the inner expression.
pub fn many1(inner: Expr) -> Expr {
    Expr::Many1(Box::new(inner))
}

/// Create a capture group, every token matched inside of this expression will be added to the capture group tokens.
pub fn capture(group: &'static str, inner: Expr) -> Expr {
    Expr::Capture(group, Box::new(inner))
}

#[derive(Clone)]
pub struct Captures<'token> {
    inner: HashTrieMap<&'static str, Vector<&'token str>>,
}

impl<'token> Captures<'token> {
    fn new() -> Captures<'token> {
        Captures {
            inner: HashTrieMap::new(),
        }
    }

    fn add_to_group(&self, group: &'static str, token: &'token str) -> Captures<'token> {
        let dummy = Vector::new();
        let list = self.inner.get(group).unwrap_or(&dummy);
        let list = list.push_back(token);
        Captures {
            inner: self.inner.insert(group, list),
        }
    }

    pub fn groups(&self) -> rpds::map::hash_trie_map::IterKeys<&'static str, Vector<&str>> {
        self.inner.keys()
    }

    pub fn tokens(&self, group: &'static str) -> rpds::vector::Iter<&str> {
        self.inner.get(group).expect("invalid group").iter()
    }
}

#[derive(Clone)]
struct MatchPair<'a> {
    node: NodeIndex,
    caps: Captures<'a>,
}

impl<'a> PartialEq for MatchPair<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl<'a> Eq for MatchPair<'a> {}

impl<'a> PartialOrd for MatchPair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.node.partial_cmp(&other.node)
    }
}

impl<'a> Ord for MatchPair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node.cmp(&other.node)
    }
}

impl<'a> MatchPair<'a> {
    fn new(node: NodeIndex, caps: Captures) -> MatchPair {
        MatchPair { node, caps }
    }
}

fn match_nfa<'token>(start: NodeIndex, nfa: &NFA, tokens: &'token [&str]) -> Vec<Captures<'token>> {
    let mut nodes = BTreeSet::new();
    let mut next_nodes = BTreeSet::new();
    nodes.insert(MatchPair::new(start, Captures::new()));
    for token in tokens {
        for pair in nodes.iter() {
            for candidate in nfa.neighbors(pair.node) {
                let info = &nfa[candidate];
                if info.pattern.matches(token) {
                    if let Some(group) = info.group {
                        next_nodes.insert(MatchPair::new(
                            candidate,
                            pair.caps.add_to_group(group, token),
                        ));
                    } else {
                        next_nodes.insert(MatchPair::new(candidate, pair.caps.clone()));
                    }
                }
            }
        }
        mem::swap(&mut nodes, &mut next_nodes);
        next_nodes.clear();
    }
    nodes.iter().map(|pair| pair.caps.clone()).collect()
}

pub struct Matcher {
    start: NodeIndex,
    nfa: NFA,
}

impl Matcher {
    /// Create a new matcher from the sequence of expressions.
    pub fn new(exprs: &[Expr]) -> Matcher {
        let (nfa, start) = build_nfa(exprs);
        Matcher { nfa, start }
    }

    /// Match the sequence of tokens with this Matcher.
    ///
    /// A list of captures that matched this Matcher is returned.
    pub fn captures<'tokens>(&self, tokens: &'tokens [&str]) -> Vec<Captures<'tokens>> {
        match_nfa(self.start, &self.nfa, &tokens)
    }

    /// Get the graphviz code for this Matcher.
    pub fn graphviz(&self) -> String {
        format!("{:?}", Dot::new(&self.nfa))
    }
}

fn main() {
    let (nfa, start) = build_nfa(&[
        capture("title", many1(regex("\\w+"))),
        capture("year", year()),
    ]);
    let caps = match_nfa(start, &nfa, &["2001", "a", "space", "odyssey", "1968"]);
    for cap in caps {
        for group in cap.groups() {
            let tokens: Vec<_> = cap.tokens(group).collect();
            println!("{} {:?}", group, tokens);
        }
        println!("---");
    }
    // println!("{:?}", Dot::new(&nfa));
}
