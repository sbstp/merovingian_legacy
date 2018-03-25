use database::Database;

pub fn sync(db: &mut Database) {
    db.retain_movies(|movie| {
        let exists = movie.path.exists();
        if !exists {
            println!(
                "{} is missing, removing from database.",
                movie.path.display()
            );
        }
        exists
    })
}
