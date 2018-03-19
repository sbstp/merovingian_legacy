extern crate bincode;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod parse;
pub mod database;

fn main() {
    for tag in parse::metadata::ALL.iter() {
        println!("{}", tag);
    }
}
