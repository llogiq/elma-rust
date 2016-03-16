#![feature(slice_patterns)]
extern crate byteorder;

use std::io::Read;

pub mod lev;
pub mod rec;

/// Position struct.
pub struct Position<T> {
    /// X-position.
    x: T,
    /// Y-position.
    y: T
}

/// Read n bytes function.
fn read_n<R> (reader: R, bytes: u64) -> Vec<u8> where R: Read {
    let mut buffer = vec![];
    let mut chunk = reader.take(bytes);
    chunk.read_to_end(&mut buffer).unwrap();
    buffer
}
