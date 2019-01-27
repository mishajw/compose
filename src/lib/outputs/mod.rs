//! Implementations of [`Output`](../core/trait.Output.html)

use core::Output;

mod speaker;

pub use self::speaker::Speaker;

impl_from_value_switch!(Output, "output", Speaker);
