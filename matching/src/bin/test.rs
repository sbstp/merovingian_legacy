extern crate matching;

use matching::tv::parse_episode;

fn main() {
    println!("result {:?}", parse_episode("boondocks s02e01"));
    println!("result {:?}", parse_episode("boondocks 12x01"));
    println!("result {:?}", parse_episode("boondocks s01 e02"));
    println!("result {:?}", parse_episode("boondocks 01.02"));
    println!("result {:?}", parse_episode("boondocks e02"));
}
