//! Implementations of [`Player`](../core/trait.Player.html)

mod combiner;
mod keyboard;
mod smooth_toggle;
mod toggle;
mod volume;
mod wave;

pub use self::combiner::Combiner;
pub use self::keyboard::Keyboard;
pub use self::smooth_toggle::SmoothToggle;
pub use self::toggle::Toggle;
pub use self::volume::Volume;
pub use self::wave::Wave;
