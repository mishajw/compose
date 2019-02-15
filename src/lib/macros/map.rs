use core::spec::resolve_spec_value;
use core::spec::{Spec, SpecMacro, Value};
use core::Consts;
use error::*;

const DEFAULT_VAR: &str = "$1";

/// Macro to map a spec over a list
pub struct Map {}

impl SpecMacro for Map {
    fn name() -> String { "map".into() }

    fn resolve(spec: &mut Spec, consts: &Consts) -> Result<Value> {
        resolve_spec_value(spec, "list".into(), consts)?;
        let spec_fn: Spec = spec.consume("fn", consts)?;
        let var_name: String =
            spec.consume_with_default("var", DEFAULT_VAR.into(), consts)?;
        let list: Vec<Value> = spec.consume("list", consts)?;
        Ok(Value::List(
            list.into_iter()
                .map(|v| -> Result<_> {
                    let mut spec_fn: Spec = spec_fn.clone();
                    apply_spec_fn(&mut spec_fn, &v, &var_name, consts)?;
                    Ok(Value::Spec(spec_fn))
                })
                .collect::<Result<_>>()?,
        ))
    }
}

fn apply_spec_fn(
    spec_fn: &mut Spec,
    value: &Value,
    var_name: &str,
    consts: &Consts,
) -> Result<()>
{
    let value_names = spec_fn.value_names();

    {
        let found_name: Option<&String> =
            value_names.iter().find(|value_name| {
                if let Ok(s) = spec_fn.get::<String>(&value_name) {
                    s.contains(var_name)
                } else {
                    false
                }
            });

        if let Some(found_name) = found_name {
            // Unwrap, as we're sure it exists and is a string field
            let found_value =
                spec_fn.consume::<String>(found_name, consts).unwrap();
            if let Value::Str(value) = value {
                // If we're replacing with a string, replace the variable name
                // inside the string
                spec_fn.put(
                    found_name.clone(),
                    found_value.replace(var_name, value),
                )
            } else {
                // If we're replacing with another type, remove the old value
                // completely
                spec_fn.put(found_name.clone(), value.clone());
            }
            return Ok(());
        }
    }

    for value_name in value_names.iter() {
        if let Ok(s) = spec_fn.get_mut::<Spec>(&value_name) {
            if let Ok(()) = apply_spec_fn(s, value, var_name, consts) {
                return Ok(());
            }
        }
    }

    bail!(ErrorKind::SpecError(format!(
        "Map fn does not contain variable {}",
        var_name
    )))
}
