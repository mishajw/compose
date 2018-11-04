//! Tools for creating components from a [`Spec`](../struct.Spec.html)

use core::input;
use core::Output;
use core::Player;
use errors::*;
use inputs;
use outputs;
use players;
use spec::{Spec, Value};

/// Implementors can be created from a spec
pub trait FromSpec<T> {
    /// The name of the value to be created, used to find the type of the
    /// definition
    fn name() -> &'static str;
    /// Create the value from a spec
    fn from_spec(spec: &mut Spec) -> Result<T>;
}

/// Create a player from the spec. Every creatable player has to be added to
/// this function
pub fn create_player(spec: &mut Spec) -> Result<Box<Player>> {
    let name = spec.use_str("name")?;
    create_with_type::<players::Wave, _>(&name, spec)
        .chain_err(|| format!("Failed to create player {}", name))?
        .ok_or_else(|| ErrorKind::SpecUnknownName(name).into())
}

/// Create an input from the spec. Every creatable input has to be added to
/// this function
pub fn create_continuous_input(
    spec: &mut Spec,
) -> Result<Box<input::Continuous>> {
    let name = spec.use_str("name")?;
    create_with_type::<inputs::Wave, _>(&name, spec)
        .chain_err(|| format!("Failed to create continuous input {}", name))?
        .ok_or_else(|| ErrorKind::SpecUnknownName(name).into())
}

/// Create outputs from the spec.
pub fn create_outputs(values: Vec<Value>) -> Result<Vec<Box<Output>>> {
    let mut outputs = Vec::new();
    for value in values {
        if let Value::Spec(mut spec) = value {
            outputs.push(create_output(&mut spec)?);
        } else {
            return Err(ErrorKind::SpecTypeError(
                "outputs[]".into(),
                "object".into(),
            )
            .into());
        }
    }
    Ok(outputs)
}

/// Create an output from the spec. Every creatable output has to be added to
/// this function
fn create_output(spec: &mut Spec) -> Result<Box<Output>> {
    let name = spec.use_str("name")?;
    create_with_type::<outputs::Speaker, _>(&name, spec)
        .chain_err(|| format!("Failed to create output {}", name))?
        .ok_or_else(|| ErrorKind::SpecUnknownName(name).into())
}

fn create_with_type<T: 'static + FromSpec<S>, S>(
    name: &str,
    spec: &mut Spec,
) -> Result<Option<S>>
{
    if name == T::name() {
        Ok(Some(T::from_spec(spec)?))
    } else {
        Ok(None)
    }
}
