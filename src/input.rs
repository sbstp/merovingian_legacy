use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

macro_rules! print_flush {
    ($($arg: tt)*) => {
        let stdout = stdout();
        let mut lock = stdout.lock();
        let _ = write!(lock, $($arg)*);
        let _ = lock.flush();
    }
}

// macro_rules! println_flush {
//     () => (print_flush!("\n"));
//     ($fmt:expr) => (print_flush!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (print_flush!(concat!($fmt, "\n"), $($arg)*));
// }

pub fn input() -> String {
    let mut answer = String::new();
    stdin()
        .read_line(&mut answer)
        .expect("could not read answer from stdin");
    let tlen = answer.trim_right().len();
    answer.truncate(tlen);
    answer
}

pub fn question(text: &str) -> String {
    print_flush!("{} ", text);
    input()
}

pub fn question_path(text: &str) -> PathBuf {
    loop {
        let answer = question(text);
        if let Ok(path) = Path::new(&answer).canonicalize() {
            return path;
        }
        println!("The path you have invented is not valid or does not exist.");
    }
}

pub fn question_bool(text: &str) -> bool {
    loop {
        print_flush!("{} [y/n] ", text);
        let answer = input();
        match &answer[..] {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => {}
        }
    }
}
