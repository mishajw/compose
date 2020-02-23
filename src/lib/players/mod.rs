//! Implementations of [`Player`](../core/trait.Player.html)

use core::Player;

mod combiner;
mod empty;
mod fourier_drawer;
mod keyboard;
mod linear;
mod one_off;
mod play_input;
mod sample;
mod speed;
mod volume;
mod wave;
mod wave_drawer;

pub use self::combiner::Combiner;
pub use self::empty::Empty;
pub use self::fourier_drawer::FourierDrawer;
pub use self::keyboard::Keyboard;
pub use self::linear::Linear;
pub use self::one_off::OneOff;
pub use self::play_input::PlayInput;
pub use self::sample::Sample;
pub use self::speed::Speed;
pub use self::volume::Volume;
pub use self::wave::Wave;
pub use self::wave_drawer::WaveDrawer;

impl_super_from_value!(
    dyn Player,
    "player",
    Wave,
    Volume,
    Combiner,
    Keyboard,
    Sample,
    WaveDrawer,
    Speed,
    Empty,
    Linear,
    OneOff,
    FourierDrawer
);
