use core::spec::{Spec, Value};
use core::Consts;
use error::*;
use macros;

/// Resolvable macro in the spec
pub trait SpecMacro {
    /// The name of the macro, checked when invoking
    fn name() -> String;

    /// Resolve the macro on a spec
    fn resolve(spec: &mut Spec, consts: &Consts) -> Result<Value>;
}

/// Resolve macros in the root player
pub fn resolve_root_macros(spec: Spec, consts: &Consts) -> Result<Spec> {
    let resolved =
        resolve_macros(Value::Spec(spec), consts).chain_err(|| "Failed to resolve macros")?;
    match resolved {
        Value::Spec(spec) => Ok(spec),
        _ => Err(ErrorKind::SpecError("Macro-resolved spec was not an object".into()).into()),
    }
}

/// Resolve a value in a spec
pub fn resolve_spec_value(spec: &mut Spec, value_name: String, consts: &Consts) -> Result<()> {
    let value: Value = spec.consume(&value_name, consts)?;
    let value = resolve_macros(value, consts)?;
    spec.put(value_name, value);
    Ok(())
}

/// Resolve macros in a spec
fn resolve_macros(value: Value, consts: &Consts) -> Result<Value> {
    let mut spec = match value {
        // If resolving a spec, continue
        Value::Spec(spec) => spec,
        // If resolving a list, run for each element in the list
        Value::List(list) => {
            return list
                .into_iter()
                .map(|v| resolve_macros(v, consts))
                .collect::<Result<_>>()
                .map(Value::List);
        }
        // If resolving anything else, there's nothing we can do
        value => return Ok(value),
    };

    // Resolve this spec
    let resolved_value =
        resolve_single_spec(&mut spec, consts)?.unwrap_or_else(|| Value::Spec(spec));

    // If resolved to another spec or list, resolve all children
    let resolved_value = match resolved_value {
        Value::Spec(spec) => {
            // Resolve all children
            Value::Spec(Spec::new(
                spec.values
                    .into_iter()
                    .map(|e| resolve_entry(e, consts))
                    .collect::<Result<_>>()?,
            ))
        }
        value => resolve_macros(value, consts)?,
    };

    Ok(resolved_value)
}

fn resolve_entry(entry: (String, Value), consts: &Consts) -> Result<(String, Value)> {
    let (value_name, value) = entry;
    resolve_macros(value, consts)
        .chain_err(|| format!("Error resolving macros for {}", value_name))
        .map(|resolved_value| (value_name, resolved_value))
}

fn resolve_single_spec(spec: &mut Spec, consts: &Consts) -> Result<Option<Value>> {
    let name: String = match spec.consume_optional("name", consts)? {
        Some(name) => name,
        None => return Ok(None),
    };
    match resolve_single_macro::<macros::TimelineMulti>(&name, spec, consts)
        .or_else(|| resolve_single_macro::<macros::Map>(&name, spec, consts))
    {
        None => {
            spec.put("name".into(), name);
            Ok(None)
        }
        Some(value) => value.map(Some),
    }
}

fn resolve_single_macro<T: SpecMacro>(
    name: &str,
    spec: &mut Spec,
    consts: &Consts,
) -> Option<Result<Value>> {
    if name == T::name() {
        Some(T::resolve(spec, consts))
    } else {
        None
    }
}
