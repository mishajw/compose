use core::CompositionConsts;
use core::Note;
use errors::*;

const NUM_OCTAVE_STEPS: usize = 12;

/// Represents a scale around a note
pub struct Scale {
    base: Note,
    steps: Vec<usize>,
}

impl Scale {
    #[allow(missing_docs)]
    pub fn new(
        base: Note,
        scale_name: &str,
        consts: &CompositionConsts,
    ) -> Result<Self>
    {
        let mut steps: Vec<usize> = consts
            .scale_map
            .get(scale_name)
            .ok_or_else(|| -> Error {
                ErrorKind::SpecUnknownName(scale_name.into()).into()
            })?
            .clone();
        let steps_total: usize = steps.iter().sum();
        if steps_total != NUM_OCTAVE_STEPS {
            // Complete to the next octave
            steps.push(NUM_OCTAVE_STEPS - (steps_total % NUM_OCTAVE_STEPS));
        }
        Ok(Scale { base, steps })
    }

    /// Get the default scale size, typically ranging across an octave
    pub fn default_size(&self) -> usize { self.steps.len() }

    /// Get notes of the scale
    pub fn get_notes(&self, num: usize) -> Vec<Note> {
        let mut note = self.base.clone();
        let mut result = vec![note.clone()];
        for step in self.steps.iter().cycle().take(num - 1) {
            note = note.increment(*step as u32);
            result.push(note.clone());
        }
        result
    }
}