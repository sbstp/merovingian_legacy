extern crate bincode;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

pub mod database;
pub mod error;
pub mod fs;
pub mod parse;

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
    // // let args = Commands::from_args();
    // match args {
    //     Commands::Import { path } => {
    //         println!("importing");
    //     }
    //     _ => {}
    // }

    // let entry = fs::walk("src/main.rs").expect("wtf");
    // assert_eq!(entry, "src/main.rs");
    // println!("{:#?}", entry);

    // for entry in entry.iter() {
    //     println!("{}", entry.path().display());
    // }
}
