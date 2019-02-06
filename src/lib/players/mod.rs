//! Implementations of [`Player`](../core/trait.Player.html)

use core::Player;

mod combiner;
mod empty;
mod keyboard;
mod linear;
mod play_input;
mod sample;
mod speed;
mod toggle;
mod volume;
mod wave;
mod wave_drawer;

pub use self::combiner::Combiner;
pub use self::empty::Empty;
pub use self::keyboard::Keyboard;
pub use self::linear::Linear;
pub use self::play_input::PlayInput;
pub use self::sample::Sample;
pub use self::speed::Speed;
pub use self::toggle::Toggle;
pub use self::volume::Volume;
pub use self::wave::Wave;
pub use self::wave_drawer::WaveDrawer;

impl_from_value_switch!(
    Player, "player", Wave, Volume, Combiner, Toggle, Keyboard, Sample,
    WaveDrawer, Speed, Empty, Linear
);
