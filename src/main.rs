extern crate bincode;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
#[macro_use]
extern crate structopt;

pub mod database;
pub mod error;
pub mod fs;
pub mod parse;
pub mod tasks;
pub mod tmdb;
pub mod fingerprint;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mero", about = "Movie and tv library manager")]
pub enum Commands {
    /// Import movies from a directory, moving the files to the library.
    #[structopt(name = "import")]
    Import { path: String },

    /// Cleanup the database, detect new files, remove references deleted files, update images.
    #[structopt(name = "update")]
    Update,

    /// Get the fingerprint of a file.
    #[structopt(name = "fingerprint")]
    Fingerprint { path: String },
}

static TEMPLATE: &'static str = "\
USAGE:
    {usage}

{all-args}
";

fn main() {
    let app = Commands::clap();
    let app = app.template(TEMPLATE);
    let args = Commands::from_clap(&app.get_matches());
    // let args = Commands::from_args();
    match args {
        Commands::Import { path } => {
            tasks::import(path);
        }
        Commands::Fingerprint { path } => {
            let hash = fingerprint::file(path).expect("fail");
            println!("{}", hash);
        }
        _ => {}
    }

    // let entry = fs::walk("src/main.rs").expect("wtf");
    // assert_eq!(entry, "src/main.rs");
    // println!("{:#?}", entry);

    // for entry in entry.iter() {
    //     println!("{}", entry.path().display());
    // }
}
