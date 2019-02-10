use core::spec::FieldDescription;
use core::spec::Spec;
use core::spec::SpecType;
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

impl SpecType for Empty {
    fn name() -> &'static str { "empty" }

    fn field_descriptions() -> Vec<FieldDescription> { Vec::new() }

    fn from_spec(_spec: Spec, _consts: &Consts) -> Result<Self> {
        Ok(Empty::player())
    }
}
