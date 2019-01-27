use core::spec::FromValue;
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
    pub fn player() -> Empty { Empty {} }
}

impl Player for Empty {
    fn play(&mut self, _state: &State) -> Playable { Playable::new(0) }
}

impl Tree for Empty {
    fn to_tree(&self) -> &Tree { self as &Tree }
}

impl FromValue for Empty {
    fn name() -> &'static str { "empty" }

    fn from_value(_value: Value, _consts: &Consts) -> Result<Self> {
        Ok(Empty::player())
    }
}
