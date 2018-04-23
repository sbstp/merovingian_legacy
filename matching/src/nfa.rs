use std::rc::Rc;

use petgraph::dot::Dot;
use petgraph::graph::{DiGraph, NodeIndex};
use regex::Regex;
use rpds::{self, HashTrieMap, Vector};

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

fn connect_node_pairs(
    nfa: &mut NFA,
    list1: &[NodeIndex],
    list2: &[NodeIndex],
    connect_to_self: bool,
) {
    for &node1 in list1.iter() {
        for &node2 in list2.iter() {
            if (connect_to_self || node1 != node2) && !nfa.contains_edge(node1, node2) {
                let weight = nfa[node2].clone();
                nfa.add_edge(node1, node2, weight);
            }
        }
    }
}

fn build_nfa_rec(
    nfa: &mut NFA,
    prev_nodes: &[NodeIndex],
    exp: &Expr,
    group: Option<&'static str>,
) -> Vec<NodeIndex> {
    match exp {
        Expr::Pattern(pattern) => {
            let new_node = nfa.add_node(PatternInfo {
                pattern: pattern.clone(),
                group,
            });
            connect_node_pairs(nfa, prev_nodes, &[new_node], false);
            vec![new_node]
        }
        Expr::Sequence(seq) => {
            let mut prev_nodes = prev_nodes.to_vec();
            for exp in seq {
                prev_nodes = build_nfa_rec(nfa, &prev_nodes, exp, group);
            }
            prev_nodes
        }
        Expr::Or(seq) => {
            let mut term_nodes = vec![];
            for exp in seq {
                let mut sub_term_nodes = build_nfa_rec(nfa, prev_nodes, exp, group);
                term_nodes.append(&mut sub_term_nodes);
            }
            term_nodes
        }
        Expr::Many0(inner) => {
            let mut sub_term_nodes = build_nfa_rec(nfa, prev_nodes, inner, group);
            connect_node_pairs(nfa, &sub_term_nodes, &sub_term_nodes, true);
            let mut term_nodes = vec![];
            term_nodes.append(&mut sub_term_nodes);
            term_nodes.extend(prev_nodes.iter().cloned());
            term_nodes
        }
        Expr::Many1(inner) => {
            let mut sub_term_nodes = build_nfa_rec(nfa, prev_nodes, inner, group);
            connect_node_pairs(nfa, &sub_term_nodes, &sub_term_nodes, true);
            let mut term_nodes = vec![];
            term_nodes.append(&mut sub_term_nodes);
            term_nodes
        }
        Expr::Maybe(inner) => {
            let mut sub_term_nodes = build_nfa_rec(nfa, prev_nodes, inner, group);
            sub_term_nodes.extend(prev_nodes.iter().cloned());
            sub_term_nodes
        }
        Expr::Capture(group, inner) => build_nfa_rec(nfa, prev_nodes, inner, Some(group)),
    }
}

/// Creates a NFA from Expr expressions without epsilon transitions.
fn build_nfa(root: Expr) -> (NFA, NodeIndex) {
    let mut nfa = NFA::new();
    let start = nfa.add_node(PatternInfo::dont_match());
    build_nfa_rec(&mut nfa, &[start], &root, None);
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

#[derive(Clone, Debug)]
pub enum Expr {
    Pattern(Rc<Pattern>),
    Sequence(Vec<Expr>),
    /// |
    Or(Vec<Expr>),
    /// *
    Many0(Box<Expr>),
    /// +
    Many1(Box<Expr>),
    /// ?
    Maybe(Box<Expr>),
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

pub fn sequence<E: AsRef<[Expr]>>(exprs: E) -> Expr {
    Expr::Sequence(exprs.as_ref().to_vec())
}

/// Create an expression that will match either the left or the right expression.
pub fn or<E: AsRef<[Expr]>>(exprs: E) -> Expr {
    Expr::Or(exprs.as_ref().to_vec())
}

/// Create an expression that matches zero or more times the inner expression.
pub fn many0(inner: Expr) -> Expr {
    Expr::Many0(Box::new(inner))
}

/// Create an expression that matches one or more times the inner expression.
pub fn many1(inner: Expr) -> Expr {
    Expr::Many1(Box::new(inner))
}

/// Create an expression that matches zero or one times the inner expression.
pub fn maybe(inner: Expr) -> Expr {
    Expr::Maybe(Box::new(inner))
}

/// Create a capture group, every token matched inside of this expression will be added to the capture group tokens.
pub fn capture(group: &'static str, inner: Expr) -> Expr {
    Expr::Capture(group, Box::new(inner))
}

/// A structure that holds the captured tokens by group name and inside a vector.
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

    /// Create an iterator of the names of the `Capture` groups.
    pub fn groups(&self) -> impl Iterator<Item = &&str> {
        self.inner.keys()
    }

    /// Create an iterator over the tokens in the given `Capture` group.
    pub fn tokens(&self, group: &'static str) -> CapturesGroupIter {
        match self.inner.get(group) {
            None => CapturesGroupIter { caps: None },
            Some(vec) => CapturesGroupIter {
                caps: Some(vec.iter()),
            },
        }
    }
}

pub struct CapturesGroupIter<'vec, 'tok: 'vec> {
    caps: Option<rpds::vector::Iter<'vec, &'tok str>>,
}

impl<'vec, 'tok> Iterator for CapturesGroupIter<'vec, 'tok> {
    type Item = &'vec &'tok str;
    fn next(&mut self) -> Option<&'vec &'tok str> {
        match self.caps {
            None => None,
            Some(ref mut iter) => iter.next(),
        }
    }
}

/// A backtracking search through the NFA.
fn match_nfa_backtrack<'token>(
    current_node: NodeIndex,
    nfa: &NFA,
    caps: Captures<'token>,
    remaining_tokens: &[&'token str],
    results: &mut Vec<Captures<'token>>,
) {
    if remaining_tokens.len() == 0 {
        results.push(caps.clone());
    } else {
        let token = remaining_tokens[0];
        for neighbor in nfa.neighbors(current_node) {
            let info = &nfa[neighbor];
            if info.pattern.matches(token) {
                if let Some(group) = info.group {
                    match_nfa_backtrack(
                        neighbor,
                        nfa,
                        caps.add_to_group(group, token),
                        &remaining_tokens[1..],
                        results,
                    );
                } else {
                    match_nfa_backtrack(
                        neighbor,
                        nfa,
                        caps.clone(),
                        &remaining_tokens[1..],
                        results,
                    );
                }
            }
        }
    }
}

fn match_nfa<'token>(start: NodeIndex, nfa: &NFA, tokens: &'token [&str]) -> Vec<Captures<'token>> {
    let mut results = Vec::new();
    match_nfa_backtrack(start, nfa, Captures::new(), tokens, &mut results);
    results
}

pub struct Matcher {
    start: NodeIndex,
    nfa: NFA,
}

impl Matcher {
    /// Create a new matcher from the sequence of expressions.
    pub fn new(root: Expr) -> Matcher {
        let (nfa, start) = build_nfa(root);
        Matcher { nfa, start }
    }

    /// Match the sequence of tokens with this Matcher.
    ///
    /// A list of captures that matched this Matcher is returned.
    /// If no capture group was created, the `Captures` object will be empty.
    pub fn captures<'tokens>(&self, tokens: &'tokens [&str]) -> Vec<Captures<'tokens>> {
        match_nfa(self.start, &self.nfa, &tokens)
    }

    /// Get the graphviz code for this Matcher.
    pub fn graphviz(&self) -> String {
        format!("{:?}", Dot::new(&self.nfa))
    }
}
