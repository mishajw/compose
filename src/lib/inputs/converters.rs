/// Inputs that convert between input types
use core::input;
use core::spec::FromValue;
use core::spec::{Spec, Value};
use core::tree::Tree;
use core::Consts;
use core::State;
use error::*;

/// Convert a `Bounded` input to a `Bool` input
pub struct BoundedToBool {
    bounded: Box<input::Bounded>,
}

impl BoundedToBool {
    #[allow(missing_docs)]
    pub fn bool(bounded: Box<input::Bounded>) -> Self {
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

impl FromValue for BoundedToBool {
    fn name() -> &'static str { "bounded-to-bool" }
    fn from_value(value: Value, consts: &Consts) -> Result<Self> {
        let mut spec: Spec = value.into_type(consts)?;
        let bounded: Box<input::Bounded> = spec.consume("bounded", consts)?;
        spec.ensure_all_used()?;
        Ok(BoundedToBool::bool(bounded))
    }
}

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

impl FromValue for BoolToBounded {
    fn name() -> &'static str { "bool-to-bounded" }
    fn from_value(value: Value, consts: &Consts) -> Result<Self> {
        let mut spec: Spec = value.into_type(consts)?;
        let bool_input: Box<input::Bool> = spec.consume("bool", consts)?;
        spec.ensure_all_used()?;
        Ok(BoolToBounded::new(bool_input))
    }
}
