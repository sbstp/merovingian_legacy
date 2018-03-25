use std::fmt;
use std::fs::{self, Metadata};
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use error;
use tree::{Node, Tree};

pub struct Entry {
    path: PathBuf,
    metadata: Metadata,
}

impl Entry {
    #[inline]
    pub fn new(path: PathBuf, metadata: Metadata) -> Entry {
        Entry { path, metadata }
    }

    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[inline]
    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Entry({})", self.path.display())
    }
}

impl Deref for Entry {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Path {
        self.path()
    }
}

impl AsRef<Path> for Entry {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

impl<A> PartialEq<A> for Entry
where
    A: AsRef<Path>,
{
    #[inline]
    fn eq(&self, rhs: &A) -> bool {
        match rhs.as_ref().canonicalize() {
            Ok(path) => self.path() == path,
            Err(_) => false,
        }
    }
}

fn walk_rec(path: &Path, tree: &mut Tree<Entry>, parent: Node) -> Result<(), error::Error> {
    for item in path.read_dir()? {
        let item = item?;
        let abs_path = item.path().canonicalize()?;
        let metadata = item.metadata()?;
        let is_dir = metadata.is_dir();

        let node = tree.node(Entry::new(abs_path.clone(), metadata));
        tree.append_to(node, parent);

        if is_dir {
            walk_rec(&abs_path, tree, node)?;
        }
    }
    Ok(())
}

pub fn walk<A: AsRef<Path>>(path: A) -> Result<(Tree<Entry>, Node), error::Error> {
    let abs_path = path.as_ref().canonicalize()?;
    let metadata = abs_path.metadata()?;
    let is_dir = metadata.is_dir();

    let mut tree = Tree::new();
    let root = tree.node(Entry::new(abs_path, metadata));

    if is_dir {
        walk_rec(path.as_ref(), &mut tree, root)?;
    }

    Ok((tree, root))
}

pub fn best_copy<A1, A2>(src: A1, dst: A2) -> io::Result<()>
where
    A1: AsRef<Path>,
    A2: AsRef<Path>,
{
    fs::DirBuilder::new()
        .recursive(true)
        .create(dst.as_ref().parent().expect("destination has no directory"))?;
    fs::hard_link(&src, &dst).or_else(|_| fs::copy(&src, &dst).map(|_| ()))
}

pub fn filter_filename(source: &str) -> String {
    let mut dest = String::with_capacity(source.len());
    for car in source.chars() {
        dest.push(match car {
            '/' | '<' | '>' | ':' | '"' | '\\' | '|' | '?' | '*' => '_',
            c if c.is_ascii_control() => '_',
            _ => car,
        });
    }
    let tlen = dest.trim_right_matches(&[' ', '.'][..]).len();
    dest.truncate(tlen);
    dest
}

#[test]
fn test_filter_filename() {
    assert_eq!(filter_filename("2001: A Space"), "2001_ A Space");
    assert_eq!(filter_filename("file ends with . "), "file ends with");
}
