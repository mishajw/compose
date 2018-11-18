use core::spec::{Spec, SpecMacro, Value};
use errors::*;

/// Macro to map a spec over a list
pub struct Map {}

impl SpecMacro for Map {
    fn name() -> &'static str { "map" }

    fn resolve(spec: &mut Spec) -> Result<Value> {
        let field: String = spec.consume("field")?;
        let fn_spec: Spec = spec.consume("fn")?;
        let list: Vec<Value> = spec.consume("list")?;
        Ok(Value::List(
            list.into_iter()
                .map(|v| Value::Spec(fn_spec.clone().with(field.clone(), v)))
                .collect(),
        ))
    }
}
