use core;
use core::spec::Spec;
use core::spec::SpecMacro;
use core::spec::Value;
use core::Consts;
use core::Note;
use error::*;

/// Resolve a scale name into a list of frequencies
pub struct Scale {}

impl SpecMacro for Scale {
    fn name() -> String { "scale".into() }

    fn resolve(spec: &mut Spec, consts: &Consts) -> Result<Value> {
        let scale_str: String = spec.consume("scale", consts)?;
        let scale = core::Scale::from_str(&scale_str, consts)?;
        let num_notes = spec.consume_with_default(
            "num-notes",
            scale.default_size() as i32,
            consts,
        )? as usize;
        spec.ensure_all_used()?;

        Ok(Value::List(
            scale
                .to_notes(num_notes)
                .iter()
                .map(Note::to_frequency)
                .map(Value::Float)
                .collect(),
        ))
    }
}
