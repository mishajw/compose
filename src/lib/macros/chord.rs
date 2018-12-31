use core::spec::{Spec, SpecMacro, Value};
use core::Consts;
use core::Note;
use core::Scale;
use error::*;

const DEFAULT_SCALE: &str = "major";

/// Resolves to a list of frequencies to make a chord
pub struct Chord {}

impl SpecMacro for Chord {
    fn name() -> &'static str { "chord" }

    fn resolve(spec: &mut Spec, consts: &Consts) -> Result<Value> {
        let note: Note = spec.consume::<String>("note")?.parse()?;
        let scale_name: String =
            spec.consume_with_default("scale", DEFAULT_SCALE.into())?;
        let chord_name: String = spec.consume("chord")?;
        let chord_indices =
            consts.chord_map.get(&chord_name).ok_or_else(|| -> Error {
                ErrorKind::SpecBadValue("chord".into(), chord_name.into())
                    .into()
            })?;
        let scale = Scale::new(note, &scale_name, consts)?;
        spec.ensure_all_used()?;
        Ok(Value::List(
            chord_indices
                .into_iter()
                .map(|i| scale.at_index(i))
                .map(|n| Note::to_frequency(&n))
                .map(Value::Float)
                .collect(),
        ))
    }
}
