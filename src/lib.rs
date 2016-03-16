extern crate byteorder;

pub mod lev;
pub mod rec;

/// Position struct.
pub struct Position<T> {
    /// X-position.
    x: T,
    /// Y-position.
    y: T
}
