extern crate matching;
extern crate regex;

use matching::tv::EPISODE_MATCHER;
use regex::{Captures, Regex};

// TODO next release of petgraph
fn fix_escape(src: &str) -> String {
    let re = Regex::new("\\\\([^\"])").unwrap();
    re.replace_all(src, |caps: &Captures| format!("\\\\{}", &caps[1]))
        .into()
}

fn main() {
    println!("{}", fix_escape(&EPISODE_MATCHER.graphviz()));
}
