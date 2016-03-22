#![doc(html_root_url = "https://hexjelly.github.io/elma-rust/")]
#![feature(slice_patterns)]
#[doc(no_inline)]
extern crate byteorder;
extern crate rand;

use std::io::Read;
use std::ffi::CString;

pub mod lev;
pub mod rec;

/// Position struct.
#[derive(Debug, PartialEq)]
pub struct Position<T> {
    /// X-position.
    pub x: T,
    /// Y-position.
    pub y: T
}

/// Read n bytes function.
fn read_n<R> (reader: R, bytes: u64) -> Vec<u8>
    where R: Read {
        let mut buffer = vec![];
        let mut chunk = reader.take(bytes);
        chunk.read_to_end(&mut buffer).unwrap();
        buffer
}

/// Read CString data from byte vector and stop at null byte.
fn cstring_read (buffer: Vec<u8>) -> CString {
    let mut name :Vec<u8> = vec![];
    for n in 0..buffer.len() {
        if buffer[n] != 0 { name.push(buffer[n]); }
        else { break };
    }
    CString::new(name).unwrap()
}
