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
pub mod util;

pub mod movie;
pub mod tv;
