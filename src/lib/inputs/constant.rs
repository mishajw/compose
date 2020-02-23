use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Input;
use core::State;
use error::*;

field_decl!(VALUE, f64, "Value of the constant");

/// Provides a constant value input
pub struct Constant {
    value: f64,
}

impl Constant {
    fn new(value: f64) -> Constant {
        Constant { value }
    }
}

impl Input for Constant {
    fn get(&mut self, _state: &State) -> f64 {
        self.value
    }
}

impl Tree for Constant {
    fn to_tree(&self) -> &dyn Tree {
        self as &dyn Tree
    }
}

impl SpecType for Constant {
    fn name() -> String {
        "constant".into()
    }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![VALUE.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let value = VALUE.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(Constant::new(value))
    }
}
