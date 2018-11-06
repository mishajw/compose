//! Implementations of [`input`](../core/input) traits

mod converters;
mod wave;

pub use self::converters::{BoolToBounded, BoundedToBool};
pub use self::wave::Wave;
