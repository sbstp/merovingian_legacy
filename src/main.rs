#![feature(nll)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
extern crate slab;
#[macro_use]
extern crate structopt;

pub mod database;
pub mod error;
pub mod fingerprint;
pub mod fs;
pub mod input;
pub mod parse;
pub mod tasks;
pub mod tmdb;
pub mod tree;

use structopt::StructOpt;

use database::Database;

#[derive(StructOpt, Debug)]
#[structopt(name = "mero", about = "Movie and tv library manager")]
pub enum Commands {
    /// Import movies from a directory, moving the files to the library.
    #[structopt(name = "import")]
    Import { path: String },

    /// Scan movies.
    #[structopt(name = "scan")]
    Scan { path: String },

    /// Cleanup the database.
    #[structopt(name = "sync")]
    Sync,

    /// Get the fingerprint of a file.
    #[structopt(name = "fingerprint")]
    Fingerprint { path: String },

    #[structopt(name = "test")]
    Test,
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

    let mut database = Database::open("database.json")
        .expect("unable to open database")
        .unwrap_or_else(|| {
            println!(
                "Your database has not been initialized. Please answer the following questions:"
            );
            let movies_path = input::question_path("Where do you want to store movies?");
            let tv_path = input::question_path("Where do you want to store tv?");
            Database::new(movies_path, tv_path)
        });

    // let args = Commands::from_args();
    match args {
        Commands::Import { path } => {
            tasks::import::import(path, &mut database);
        }
        Commands::Scan { path } => {
            let (tree, root) = fs::walk(path).unwrap();
            parse::patterns::scan(&tree, root);
        }
        Commands::Sync => {
            tasks::sync::sync(&mut database);
        }
        Commands::Fingerprint { path } => {
            let hash = fingerprint::file(path).expect("fail");
            println!("{}", hash);
        }
        Commands::Test => {
            let r = tmdb::search::movie("star wars empire strikes back", None).unwrap();
            println!("{:#?}", r);
        }
        _ => {}
    }

    database
        .save("database.json")
        .expect("unable to save database, this is bad");
}
