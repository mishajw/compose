use core;
use core::spec::{Spec, SpecMacro, Value};
use core::Consts;
use error::*;

/// Resolves to a list of frequencies to make a chord
pub struct Chord {}

impl SpecMacro for Chord {
    fn name() -> &'static str { "chord" }

    fn resolve(spec: &mut Spec, consts: &Consts) -> Result<Value> {
        let chord =
            core::Chord::from_str(&spec.consume::<String>("chord")?, consts)?;
        spec.ensure_all_used()?;
        Ok(Value::List(
            chord
                .into_notes()
                .into_iter()
                .map(|n| n.to_frequency())
                .map(Value::Float)
                .collect(),
        ))
    }
}
