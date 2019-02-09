use core::spec::FieldDeclaration;
use core::spec::FieldDescription;
use core::spec::FromSpec;
use core::spec::Spec;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

lazy_static! {
    static ref SCALE: FieldDeclaration<i32> =
        FieldDeclaration::new("scale", "Scale of linear player",);
}

/// Plays the step it's played on
pub struct Linear {
    scale: i32,
}

impl Linear {
    #[allow(missing_docs)]
    pub fn player(scale: i32) -> Linear { Linear { scale } }
}

impl Player for Linear {
    fn play(&mut self, state: &State) -> Playable {
        Playable::new(state.tick() as i32 * self.scale)
    }
}

impl Tree for Linear {
    fn to_tree(&self) -> &Tree { self as &Tree }
}

impl FromSpec for Linear {
    fn name() -> &'static str { "linear" }

    fn field_descriptions() -> Vec<FieldDescription> {
        vec![SCALE.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let scale = SCALE.get(&mut spec, consts)?;
        Ok(Linear::player(scale))
    }
}
