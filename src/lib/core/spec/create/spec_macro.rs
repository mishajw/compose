use core::spec::Spec;
use core::spec::{SpecMacro, Value};
use core::Consts;
use errors::*;
use macros;

/// Resolve all the macros in a spec
pub fn resolve_macros(spec: Spec, consts: &Consts) -> Result<Spec> {
    let resolved = resolve_children(Value::Spec(spec), consts)
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

fn resolve_single_macro<T: SpecMacro>(
    name: &str,
    spec: &mut Spec,
    consts: &Consts,
) -> Option<Result<Value>>
{
    if name == T::name() {
        Some(T::resolve(spec, consts))
    } else {
        None
    }
}

fn resolve_single_spec(
    spec: &mut Spec,
    consts: &Consts,
) -> Result<Option<Value>>
{
    let name: String = match spec.consume_optional("name")? {
        Some(name) => name,
        None => return Ok(None),
    };
    match resolve_single_macro::<macros::TimelineMulti>(&name, spec, consts)
        .or_else(|| resolve_single_macro::<macros::Map>(&name, spec, consts))
        .or_else(|| resolve_single_macro::<macros::Scale>(&name, spec, consts))
        .or_else(|| resolve_single_macro::<macros::Chord>(&name, spec, consts))
    {
        None => {
            spec.put("name".into(), name);
            Ok(None)
        }
        Some(value) => value.map(Some),
    }
}

fn resolve_entry(
    entry: (String, Value),
    consts: &Consts,
) -> Result<(String, Value)>
{
    let (value_name, value) = entry;
    resolve_children(value, consts)
        .chain_err(|| format!("Error resolving macros for {}", value_name))
        .map(|resolved_value| (value_name, resolved_value))
}

fn resolve_children(value: Value, consts: &Consts) -> Result<Value> {
    let spec = match value {
        // If resolving a spec, continue
        Value::Spec(spec) => spec,
        // If resolving a list, run for each element in the list
        Value::List(list) => {
            return list
                .into_iter()
                .map(|v| resolve_children(v, consts))
                .collect::<Result<_>>()
                .map(Value::List);
        }
        // If resolving anything else, there's nothing we can do
        value => return Ok(value),
    };

    // Resolve all children first
    let mut spec = Spec::new(
        spec.values
            .into_iter()
            .map(|e| resolve_entry(e, consts))
            .collect::<Result<_>>()?,
    );

    // Resolve this spec
    let resolved_value = resolve_single_spec(&mut spec, consts)?
        .unwrap_or_else(|| Value::Spec(spec));

    // TODO Resolve children again, in case resolving this spec created more
    // macros

    Ok(resolved_value)
}
