use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Input;
use core::State;
use error::*;

field_decl!(INPUT, Box<Input>, "The input to modify");
field_decl!(ADD, f64, "Add to the input");
field_decl!(MULT, f64, "Multiply the input");

/// Modifies an input
pub struct InputMod {
    input: Box<Input>,
    add: f64,
    mult: f64,
}

impl InputMod {
    #[allow(missing_docs)]
    pub fn new(input: Box<Input>, add: f64, mult: f64) -> InputMod {
        InputMod { input, add, mult }
    }
}

impl Input for InputMod {
    fn get(&mut self, state: &State) -> f64 {
        self.input.get(state) * self.mult + self.add
    }
}

impl Tree for InputMod {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.input.to_tree()]
    }
}

impl SpecType for InputMod {
    fn name() -> String { "input-mod".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![INPUT.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let input_mod = InputMod::new(
            INPUT.get(&mut spec, consts)?,
            ADD.get(&mut spec, consts)?,
            MULT.get(&mut spec, consts)?,
        );
        spec.ensure_all_used()?;
        Ok(input_mod)
    }
}
