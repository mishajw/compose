//! Implementations of [`Output`](../core/trait.Output.html)

use core::Output;

mod speaker;

pub use self::speaker::Speaker;

impl_super_from_value!(dyn Output, "output", Speaker);
