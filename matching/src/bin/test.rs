extern crate matching;

use matching::tv::parse_episode;

fn main() {
    println!("result {:?}", parse_episode("boondocks s02e01"));
    println!("result {:?}", parse_episode("boondocks s01 e02"));
}
