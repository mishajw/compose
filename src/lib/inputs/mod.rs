//! Implementations of [`input`](../core/input) traits

use core::input;

mod bool_to_bounded;
mod bounded_to_bool;
mod buffer;
mod function;
mod smooth_bool;
mod timeline;

pub use self::bool_to_bounded::BoolToBounded;
pub use self::bounded_to_bool::BoundedToBool;
pub use self::buffer::Buffer;
pub use self::function::Function;
pub use self::smooth_bool::SmoothBool;
pub use self::timeline::Timeline;

mod bool_impl {
    use super::*;
    impl_super_from_value!(input::Bool, "bool-input", BoundedToBool, Timeline);
}

mod bounded_impl {
    use super::*;
    impl_super_from_value!(
        input::Bounded,
        "bounded-input",
        Function,
        BoolToBounded,
        SmoothBool
    );
}
