use std::fmt::Write;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

use sha2::{Digest, Sha256};

const BLOCK_SIZE: usize = 1024 * 64; // 64 KiB
const MIN_SIZE: u64 = 1024 * 1024;

/// Very quick file fingerprinting. Takes 64KiB from the start, middle and end of the file.
/// There are 256^192KiB possibilities for the sample, collisions should be very low hopefully.
/// The hash itself is Sha256, it produces 32 bytes that are hexed to a 64 character string.
pub fn file<A>(path: A) -> io::Result<String>
where
    A: AsRef<Path>,
{
    let mut buf = [0u8; BLOCK_SIZE as usize];
    let mut hasher = Sha256::default();
    let mut file = File::open(path)?;
    let len = file.metadata()?.len();

    if len == 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "file is empty"));
    }

    if len < MIN_SIZE {
        loop {
            match file.read(&mut buf[..])? {
                0 => break,
                n => {
                    hasher.input(&buf[..n]);
                }
            }
        }
    } else {
        // begin block
        file.seek(SeekFrom::Start(0))?;
        let n = file.read(&mut buf[..])?;
        hasher.input(&buf[..n]);

        // middle block
        let pos = (len / 2)
            .checked_sub(BLOCK_SIZE as u64 / 2)
            .expect("middle position overflowed");
        file.seek(SeekFrom::Start(pos))?;
        let n = file.read(&mut buf[..])?;
        hasher.input(&buf[..n]);

        // end block
        file.seek(SeekFrom::End(-(BLOCK_SIZE as i64)))?;
        let n = file.read(&mut buf[..])?;
        hasher.input(&buf[..n]);
    }

    let mut hash = String::with_capacity(64);
    let output = &hasher.result()[..];
    for byte in output {
        let _ = write!(hash, "{:02x}", byte);
    }
    Ok(hash)
}
