//! Implementations of [`Player`](../core/trait.Player.html)

mod combiner;
mod keyboard;
mod play_input;
mod speed;
mod toggle;
mod volume;
mod wave;

pub use self::combiner::Combiner;
pub use self::keyboard::Keyboard;
pub use self::play_input::PlayInput;
pub use self::speed::Speed;
pub use self::toggle::Toggle;
pub use self::volume::Volume;
pub use self::wave::Wave;
