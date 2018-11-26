use core::spec::Spec;
use core::spec::SpecMacro;
use core::spec::Value;
use core::CompositionConsts;
use core::Note;
use errors::*;

/// Resolve a scale name into a list of frequencies
pub struct Scale {}

impl SpecMacro for Scale {
    fn name() -> &'static str { "scale" }

    fn resolve(spec: &mut Spec, consts: &CompositionConsts) -> Result<Value> {
        let mut note: Note = spec.consume::<String>("note")?.parse()?;
        let scale_name: String = spec.consume("scale")?;
        let steps: Vec<usize> = {
            let map_result: Result<_> = consts
                .scale_map
                .get(&scale_name)
                .ok_or_else(|| ErrorKind::SpecUnknownName(scale_name).into());
            let mut steps = map_result?.clone();
            let steps_total: usize = steps.iter().sum();
            if steps_total != 12 {
                // Complete to the next octave
                steps.push(12 - (steps_total % 12));
            }
            steps
        };

        let num_notes = spec.consume_with_default("num-notes", steps.len() as i32)? as usize;

        let mut frequencies = vec![note.to_frequency()];
        for step in steps.into_iter().cycle().take(num_notes - 1) {
            for _ in 0..step {
                note = note.next();
            }
            frequencies.push(note.to_frequency());
        }

        Ok(Value::List(
            frequencies.into_iter().map(Value::Float).collect(),
        ))
    }
}
