//! The core concepts in a composition.
//!
//! The [`compose`](fn.compose.html) function takes a tree of
//! [`Player`](trait.Player.html)s that creates music as a series of
//! [`Playable`](struct.Playable.html)s, and writes them to some
//! [`Output`](trait.Output.html)s. The progress of the composition is tracked
//! through [`CompositionState`](struct.CompositionState.html)s. `Player`s can
//! be controlled through [`input`](input/) traits.

pub mod composer;
mod composition_state;
pub mod input;
mod output;
mod playable;
mod player;
pub mod spec;
mod time;

pub use self::composition_state::{CompositionConsts, CompositionState};
pub use self::output::Output;
pub use self::playable::Playable;
pub use self::player::Player;
pub use self::time::Time;
