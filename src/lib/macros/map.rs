use core::spec::{Spec, SpecMacro, Value};
use core::Consts;
use error::*;

const DEFAULT_VAR: &str = "$1";

/// Macro to map a spec over a list
pub struct Map {}

impl SpecMacro for Map {
    fn name() -> &'static str { "map" }

    fn resolve(spec: &mut Spec, _consts: &Consts) -> Result<Value> {
        let spec_fn: Spec = spec.consume("fn")?;
        let var_name: String =
            spec.consume_with_default("var", DEFAULT_VAR.into())?;
        let list: Vec<Value> = spec.consume("list")?;
        Ok(Value::List(
            list.into_iter()
                .map(|v| -> Result<_> {
                    let mut spec_fn: Spec = spec_fn.clone();
                    apply_spec_fn(&mut spec_fn, &v, &var_name)?;
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
) -> Result<()>
{
    let value_names = spec_fn.value_names();

    {
        let field_name: Option<&String> =
            value_names.iter().find(|value_name| {
                if let Ok(s) = spec_fn.get::<String>(&value_name) {
                    s == var_name
                } else {
                    false
                }
            });

        if let Some(field_name) = field_name {
            // Unwrap, as we're sure it exists and is a string field
            spec_fn.consume::<String>(field_name).unwrap();
            spec_fn.put(field_name.clone(), value.clone());
            return Ok(());
        }
    }

    for value_name in value_names.iter() {
        if let Ok(s) = spec_fn.get_mut::<Spec>(&value_name) {
            if let Ok(()) = apply_spec_fn(s, value, var_name) {
                return Ok(());
            }
        }
    }

    bail!(ErrorKind::BadInput(format!(
        "Map fn does not contain variable {}",
        var_name
    )))
}
