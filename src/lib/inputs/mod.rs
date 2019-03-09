//! Implementations of [`input`](../core/input) traits

use core::Input;

mod buffer;
mod constant;
mod function;
mod input_mod;
mod smooth_bool;
mod timeline;

pub use self::buffer::Buffer;
pub use self::constant::Constant;
pub use self::function::Function;
pub use self::input_mod::InputMod;
pub use self::smooth_bool::SmoothBool;
pub use self::timeline::Timeline;

impl_super_from_value!(
    Input,
    "bounded-input",
    Function,
    SmoothBool,
    Constant,
    Timeline,
    InputMod
);
