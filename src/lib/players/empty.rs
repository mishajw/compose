use core::spec::create::FromSpec;
use core::spec::Value;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

/// Plays nothing
pub struct Empty;

impl Empty {
    #[allow(missing_docs)]
    pub fn new() -> Box<Player> { Box::new(Empty {}) }
}

impl Player for Empty {
    fn play(&mut self, _state: &State) -> Playable { Playable::new(0) }
}

impl Tree for Empty {
    fn to_tree<'a>(&'a self) -> &'a Tree { self as &Tree }
}

impl FromSpec<Box<Player>> for Empty {
    fn name() -> &'static str { "empty" }

    fn from_spec(_value: Value, _consts: &Consts) -> Result<Box<Player>> {
        Ok(Empty::new())
    }
}
