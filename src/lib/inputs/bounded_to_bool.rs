use core::input;
use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::State;
use error::*;

field_decl!(
    INPUT,
    Box<input::Bounded>,
    "Bounded input to convert to a bool"
);

/// Convert a `Bounded` input to a `Bool` input
pub struct BoundedToBool {
    bounded: Box<input::Bounded>,
}

impl BoundedToBool {
    #[allow(missing_docs)]
    pub fn new(bounded: Box<input::Bounded>) -> Self {
        BoundedToBool { bounded }
    }
}

impl input::Bool for BoundedToBool {
    fn get(&mut self, state: &State) -> bool {
        self.bounded.get_with_bounds(state, -1.0, 1.0) >= 0.0
    }
}

impl Tree for BoundedToBool {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.bounded.to_tree()]
    }
}

impl SpecType for BoundedToBool {
    fn name() -> String { "bounded-to-bool".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![INPUT.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let input = INPUT.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(BoundedToBool::new(input))
    }
}
