use std::path::{Path, PathBuf};
use std::fmt;
use std::ops::Deref;

use error;

macro_rules! pathlike {
    ($type:ty) => {
        impl Deref for $type {
            type Target = Path;

            #[inline]
            fn deref(&self) -> &Path {
                self.path()
            }
        }

        impl AsRef<Path> for $type {
            #[inline]
            fn as_ref(&self) -> &Path {
                self.path()
            }
        }

        impl<A> PartialEq<A> for $type
        where
            A: AsRef<Path>,
        {
            #[inline]
            fn eq(&self, rhs: &A) -> bool {
                match rhs.as_ref().canonicalize() {
                    Ok(path) => self.path() == path,
                    Err(_) => {
                        // debug!("canonicalize failed");
                        false
                    }
                }
            }
        }
    };
}

pub enum Entry {
    File(File),
    Directory(Directory),
}

impl Entry {
    #[inline]
    pub fn path(&self) -> &Path {
        match *self {
            Entry::File(ref file) => file.path(),
            Entry::Directory(ref dir) => dir.path(),
        }
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Entry::File(ref file) => file.fmt(f),
            Entry::Directory(ref dir) => dir.fmt(f),
        }
    }
}

pathlike!(Entry);

pub struct File {
    path: PathBuf,
}

impl File {
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File(\"{}\")", self.display())
    }
}

pathlike!(File);

pub struct Directory {
    path: PathBuf,
    children: Vec<Entry>,
}

impl Directory {
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[inline]
    pub fn children(&self) -> &[Entry] {
        &self.children
    }
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Directory(\"{}\", ", self.display())?;
        {
            let mut list = f.debug_list();
            for entry in self.children().iter() {
                list.entry(entry);
            }
            list.finish()?;
        }
        write!(f, ")")
    }
}

pathlike!(Directory);

fn walk_rec(path: &Path, parent: &mut Directory) -> Result<(), error::Error> {
    for item in path.read_dir()? {
        let item = item?;
        let path = item.path().canonicalize()?;
        let metadata = item.metadata()?;
        if metadata.is_dir() {
            let mut dir = Directory {
                path: path.clone(),
                children: Vec::new(),
            };
            walk_rec(&path, &mut dir)?;
            parent.children.push(Entry::Directory(dir));
        } else {
            parent.children.push(Entry::File(File { path }))
        }
    }
    Ok(())
}

pub fn walk<A: AsRef<Path>>(path: A) -> Result<Entry, error::Error> {
    let path = path.as_ref().canonicalize()?;
    let metadata = path.metadata()?;
    if metadata.is_dir() {
        let mut dir = Directory {
            path: path.clone(),
            children: Vec::new(),
        };
        walk_rec(&path, &mut dir)?;
        Ok(Entry::Directory(dir))
    } else {
        Ok(Entry::File(File { path }))
    }
}
