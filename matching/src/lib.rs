#![feature(nll)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate petgraph;
extern crate regex;
extern crate rpds;

pub mod metadata;
pub mod nfa;
pub mod tokens;

pub mod movie;

// pub use nfa::*;

// fn main() {
//     let m = Matcher::new(sequence([
//         capture("title", many1(regex("\\w+"))),
//         capture("year", year()),
//     ]));

//     // let m = Matcher::new(sequence(vec![maybe(string("hello")), string("world")]));

//     let caps = m.captures(&["stranger", "things", "s02", "1999", "1998"]);
//     for cap in caps {
//         for group in cap.groups() {
//             let tokens: Vec<_> = cap.tokens(group).collect();
//             println!("{} {:?}", group, tokens);
//         }
//         println!("---");
//     }
//     use std::fs::File;
//     use std::io::Write;
//     let mut file = File::create("graph.dot").unwrap();
//     write!(file, "{}", m.graphviz());
// }
