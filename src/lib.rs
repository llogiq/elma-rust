#![feature(slice_patterns)]
extern crate byteorder;
extern crate rand;

use std::io::Read;
use std::ffi::CString;

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
fn read_n<R> (reader: R, bytes: u64) -> Vec<u8>
    where R: Read {
        let mut buffer = vec![];
        let mut chunk = reader.take(bytes);
        chunk.read_to_end(&mut buffer).unwrap();
        buffer
}

/// Read CString data from n bytes, stop at null byte.
fn cstring_read<R> (mut reader: R, bytes: u64) -> CString
    where R: Read {
        let mut temp :Vec<u8> = vec![];
        let mut finished = false;
        for n in 0..bytes {
            let byte = read_n(&mut reader, 1);
            if !finished && byte[0] != 0u8 {
                temp.push(byte[0]);
            } else if byte[0] == 0u8 {
                finished = true;
            }
        }
        CString::new(temp).unwrap()
}
