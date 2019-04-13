use core::spec::Spec;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Input;
use core::State;
use error::*;

use rand::Rng;

/// Supply random input in [0, 1]
pub struct Random {}

// impl Random {
//     pub fn new() -> Random {
//         Random {
//             rng: rand::thread_rng()
//         }
//     }
// }

impl Input for Random {
    fn get(&mut self, _state: &State) -> f64 {
        // TODO: Don't create nww RNG on every call
        rand::thread_rng().gen_range(0.0, 1.0)
    }
}

impl Tree for Random {
    fn to_tree(&self) -> &Tree { self }
}

impl SpecType for Random {
    fn name() -> String { "random".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> { vec![] }

    fn from_spec(_spec: Spec, _consts: &Consts) -> Result<Self> {
        Ok(Random {})
    }
}
