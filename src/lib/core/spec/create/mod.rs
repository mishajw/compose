//! Creating components from [`core`](../../core/)

// TODO: Consume spec values in creation functions, instead of passing by
// reference

use core::spec::{Spec, Value};
use error::*;

mod bool_input;
mod bounded_input;
mod output;
mod player;
mod spec_macro;

pub use self::bool_input::create_bool_input;
pub use self::bounded_input::create_bounded_input;
pub use self::output::create_outputs;
pub use self::player::create_player;
pub use self::spec_macro::resolve_macros;
use core::Consts;

/// Implementors can be created from a spec
pub trait FromSpec<T> {
    /// The name of the value to be created, used to find the type of the
    /// definition
    fn name() -> &'static str;
    /// Create the value from a spec
    fn from_spec(value: Value, consts: &Consts) -> Result<T>;
}

/// Create a type `T` from a spec
pub fn create_with_type<T: 'static + FromSpec<S>, S>(
    name: &str,
    spec: &mut Spec,
    consts: &Consts,
) -> Option<Result<S>>
{
    if name == T::name() {
        Some(T::from_spec(Value::Spec(spec.clone()), consts))
    } else {
        None
    }
}
