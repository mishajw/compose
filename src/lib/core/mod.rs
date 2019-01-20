//! The core concepts in a composition.
//!
//! The [`compose`](fn.compose.html) function takes a tree of
//! [`Player`](trait.Player.html)s that creates music as a series of
//! [`Playable`](struct.Playable.html)s, and writes them to some
//! [`Output`](trait.Output.html)s. The progress of the composition is tracked
//! through [`State`](struct.State.html)s. `Player`s can
//! be controlled through [`input`](input/) traits.

mod chord;
pub mod composer;
mod composition;
mod consts;
pub mod input;
mod note;
mod output;
mod playable;
mod player;
pub mod regex;
mod reload_composition;
mod scale;
mod scale_index;
pub mod spec;
mod state;
mod time;
pub mod tree;

pub use self::chord::Chord;
pub use self::composition::Composition;
pub use self::consts::Consts;
pub use self::note::Note;
pub use self::output::Output;
pub use self::playable::Playable;
pub use self::player::Player;
pub use self::reload_composition::ReloadingComposition;
pub use self::scale::Scale;
pub use self::scale_index::ScaleIndex;
pub use self::state::State;
pub use self::time::Time;
