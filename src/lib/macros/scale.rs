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
    fn name() -> &'static str { "scale" }

    fn resolve(spec: &mut Spec, consts: &Consts) -> Result<Value> {
        let note: Note = spec.consume::<String>("note")?.parse()?;
        let scale_name: String = spec.consume("scale")?;
        let scale = core::Scale::new(note, &scale_name, consts)?;
        let num_notes = spec
            .consume_with_default("num-notes", scale.default_size() as i32)?
            as usize;
        spec.ensure_all_used()?;

        Ok(Value::List(
            scale
                .get_notes(num_notes)
                .iter()
                .map(Note::to_frequency)
                .map(Value::Float)
                .collect(),
        ))
    }
}
