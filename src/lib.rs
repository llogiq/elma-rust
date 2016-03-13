#[warn(missing_docs)]
pub mod lev;

#[warn(missing_docs)]
pub mod rec;

/// Position struct.
pub struct Position<T> {
    /// X-position.
    x: T,
    /// Y-position.
    y: T
}
