extern crate matching;

use std::fs::File;
use std::io::Write;

use matching::nfa::*;

fn main() {
    let m = Matcher::new(sequence([
        capture("series", many0(regex(r"^\w+$"))),
        or([
            capture("season_episode", regex(r"^s\d\d?e\d\d?$")),
            capture("season_episode", regex(r"^\d\d?x\d\d?$")),
            sequence([
                capture("season", regex(r"^s\d\d?$")),
                capture("episode", regex(r"^e\d\d?$")),
            ]),
            sequence([
                capture("season", regex(r"^\d\d?$")),
                capture("episode", regex(r"^\d\d?$")),
            ]),
            capture("episode", regex(r"^e\d\d?$")),
        ]),
    ]));

    let mut file = File::create("graph.dot").unwrap();
    write!(file, "{}", m.graphviz());
}
