use core::input;
use core::spec::FieldDeclaration;
use core::spec::FieldDescription;
use core::spec::SpecType;
use core::spec::Spec;
use core::tree::Tree;
use core::Consts;
use core::State;
use error::*;

field_decl!(
    INPUT,
    Box<input::Bool>,
    "Bool input to convert to a bounded"
);

/// Convert a `Bool` input to a `Bounded` input
pub struct BoolToBounded {
    boolean: Box<input::Bool>,
}

impl BoolToBounded {
    #[allow(missing_docs)]
    pub fn new(boolean: Box<input::Bool>) -> Self { BoolToBounded { boolean } }
}

impl input::Bounded for BoolToBounded {
    fn get(&mut self, state: &State) -> f64 {
        if self.boolean.get(state) {
            1.0
        } else {
            0.0
        }
    }

    fn get_bounds(&self) -> (f64, f64) { (0.0, 1.0) }
}

impl Tree for BoolToBounded {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.boolean.to_tree()]
    }
}

impl SpecType for BoolToBounded {
    fn name() -> &'static str { "bool-to-bounded" }

    fn field_descriptions() -> Vec<FieldDescription> {
        vec![INPUT.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let input = INPUT.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(BoolToBounded::new(input))
    }
}
