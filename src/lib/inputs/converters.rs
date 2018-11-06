/// Inputs that convert between input types
use core::input;
use core::CompositionState;
use errors::*;
use spec::{create_bool_input, create_bounded_input, FromSpec, Value};

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
    fn get(&mut self, state: &CompositionState) -> bool {
        self.bounded.get_with_bounds(state, -1.0, 1.0) >= 0.0
    }
}

impl FromSpec<Box<input::Bool>> for BoundedToBool {
    fn name() -> &'static str { "bounded-to-bool" }
    fn from_spec(value: Value) -> Result<Box<input::Bool>> {
        let mut spec = value.as_spec()?;
        let mut bounded_spec = spec.consume("bounded")?;
        spec.ensure_all_used()?;
        Ok(Box::new(BoundedToBool::new(create_bounded_input(
            &mut bounded_spec,
        )?)))
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
    fn get(&mut self, state: &CompositionState) -> f32 {
        if self.boolean.get(state) {
            1.0
        } else {
            0.0
        }
    }

    fn get_bounds(&self) -> (f32, f32) { (0.0, 1.0) }
}

impl FromSpec<Box<input::Bounded>> for BoolToBounded {
    fn name() -> &'static str { "bool-to-bounded" }
    fn from_spec(value: Value) -> Result<Box<input::Bounded>> {
        let mut spec = value.as_spec()?;
        let mut bool_spec = spec.consume("bool")?;
        spec.ensure_all_used()?;
        Ok(Box::new(BoolToBounded::new(create_bool_input(
            &mut bool_spec,
        )?)))
    }
}
