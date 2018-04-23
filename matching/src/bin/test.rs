extern crate matching;

use matching::movie::parse_movie;

fn main() {
    println!("result {:?}", parse_movie("2001 a space odyssey 2009"));
}
