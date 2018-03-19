extern crate bincode;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod database;
pub mod error;
pub mod fs;
pub mod parse;

fn main() {
    let entry = fs::walk("src").expect("wtf");
    assert_eq!(entry, "src");
    println!("{:#?}", entry);
}
