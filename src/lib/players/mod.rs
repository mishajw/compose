//! Implementations of [`Player`](../core/trait.Player.html)

mod combiner;
mod volume;
mod wave;

pub use self::combiner::Combiner;
pub use self::volume::Volume;
pub use self::wave::Wave;
