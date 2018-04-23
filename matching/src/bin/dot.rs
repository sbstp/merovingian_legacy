extern crate matching;

use std::fs::File;
use std::io::Write;

use matching::nfa::*;

fn main() {
    let m = Matcher::new(sequence([
        capture("series", many1(regex(r"\w+"))),
        or([
            capture("season_episode", regex(r"^s\d\d?e\d\d?$")),
            sequence([
                capture("season", regex("s\\d\\d?")),
                capture("episode", regex("e\\d\\d?")),
            ]),
            sequence([
                capture("season", regex("\\d\\d?")),
                capture("episode", regex("\\d\\d?")),
            ]),
        ]),
    ]));

    let mut file = File::create("graph.dot").unwrap();
    write!(file, "{}", m.graphviz());
}
