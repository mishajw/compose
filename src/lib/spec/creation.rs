//! Tools for creating components from a [`Spec`](../struct.Spec.html)

use core::input;
use core::Output;
use core::Player;
use errors::*;
use inputs;
use macros;
use outputs;
use players;
use spec::SpecMacro;
use spec::{Spec, Value};

/// Implementors can be created from a spec
pub trait FromSpec<T> {
    /// The name of the value to be created, used to find the type of the
    /// definition
    fn name() -> &'static str;
    /// Create the value from a spec
    fn from_spec(value: Value) -> Result<T>;
}

/// Create a player from the spec. Every creatable player has to be added to
/// this function
pub fn create_player(spec: &mut Spec) -> Result<Box<Player>> {
    let name: String = spec.consume("name")?;
    create_with_type::<players::Wave, _>(&name, spec)
        .or_else(|| create_with_type::<players::Volume, _>(&name, spec))
        .or_else(|| create_with_type::<players::Combiner, _>(&name, spec))
        .or_else(|| create_with_type::<players::Toggle, _>(&name, spec))
        .or_else(|| create_with_type::<players::Keyboard, _>(&name, spec))
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create player {}", name))
}

/// Create an input from the spec. Every creatable input has to be added to
/// this function
pub fn create_bounded_input(spec: &mut Spec) -> Result<Box<input::Bounded>> {
    let name: String = spec.consume("name")?;
    create_with_type::<inputs::Wave, _>(&name, spec)
        .or_else(|| create_with_type::<inputs::BoolToBounded, _>(&name, spec))
        .or_else(|| create_with_type::<inputs::SmoothBool, _>(&name, spec))
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create bounded input {}", name))
}

/// Create an bool input from the spec. Every creatable bool input has to be
/// added to this function
pub fn create_bool_input(spec: &mut Spec) -> Result<Box<input::Bool>> {
    let name: String = spec.consume("name")?;
    create_with_type::<inputs::BoundedToBool, _>(&name, spec)
        .or_else(|| create_with_type::<inputs::Timeline, _>(&name, spec))
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create bool input {}", name))
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
    let name: String = spec.consume("name")?;
    create_with_type::<outputs::Speaker, _>(&name, spec)
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create output {}", name))
}

fn create_with_type<T: 'static + FromSpec<S>, S>(
    name: &str,
    spec: &mut Spec,
) -> Option<Result<S>>
{
    if name == T::name() {
        Some(T::from_spec(Value::Spec(spec.clone())))
    } else {
        None
    }
}

/// Resolve all the macros in a spec
pub fn resolve_macros(spec: Spec) -> Result<Spec> {
    fn resolve_single_macro<T: SpecMacro>(
        name: &str,
        spec: &mut Spec,
    ) -> Option<Result<Value>>
    {
        if name == T::name() {
            Some(T::resolve(spec))
        } else {
            None
        }
    }

    fn resolve_single_spec(spec: &mut Spec) -> Result<Option<Value>> {
        let name: String = spec.consume("name")?;
        match resolve_single_macro::<macros::TimelineMulti>(&name, spec)
            .or_else(|| resolve_single_macro::<macros::Map>(&name, spec))
        {
            None => {
                spec.put("name".into(), name);
                Ok(None)
            }
            Some(value) => value.map(Some),
        }
    }

    fn resolve_entry(entry: (String, Value)) -> Result<(String, Value)> {
        let (value_name, value) = entry;
        resolve_children(value)
            .chain_err(|| format!("Error resolving macros for {}", value_name))
            .map(|resolved_value| (value_name, resolved_value))
    }

    fn resolve_children(value: Value) -> Result<Value> {
        let spec = match value {
            // If resolving a spec, continue
            Value::Spec(spec) => spec,
            // If resolving a list, run for each element in the list
            Value::List(list) => {
                return list
                    .into_iter()
                    .map(resolve_children)
                    .collect::<Result<_>>()
                    .map(Value::List)
            }
            // If resolving anything else, there's nothing we can do
            value => return Ok(value),
        };

        // Resolve all children first
        let mut spec = Spec::new(
            spec.values
                .into_iter()
                .map(resolve_entry)
                .collect::<Result<_>>()?,
        );

        // Resolve this spec
        let resolved_value = resolve_single_spec(&mut spec)?
            .unwrap_or_else(|| Value::Spec(spec));

        // TODO Resolve children again, in case resolving this spec created more
        // macros

        Ok(resolved_value)
    }

    let resolved = resolve_children(Value::Spec(spec))
        .chain_err(|| "Failed to resolve macros")?;
    match resolved {
        Value::Spec(spec) => Ok(spec),
        _ => Err(ErrorKind::SpecBadValue(
            "root".into(),
            "Root value for players was not an object".into(),
        )
        .into()),
    }
}
