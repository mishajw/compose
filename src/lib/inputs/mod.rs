//! Implementations of [`input`](../core/input) traits

use core::input;

mod buffer;
mod converters;
mod function;
mod smooth_bool;
mod timeline;

pub use self::buffer::Buffer;
pub use self::converters::{BoolToBounded, BoundedToBool};
pub use self::function::Function;
pub use self::smooth_bool::SmoothBool;
pub use self::timeline::Timeline;

mod bool_impl {
    use super::*;
    impl_from_value_switch!(input::Bool, "bool-input", BoundedToBool, Timeline);
}

mod bounded_impl {
    use super::*;
    impl_from_value_switch!(
        input::Bounded,
        "bounded-input",
        Function,
        BoolToBounded,
        SmoothBool
    );
}
