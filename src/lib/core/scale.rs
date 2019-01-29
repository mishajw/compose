use core::regex;
use core::Consts;
use core::Note;
use core::ScaleIndex;
use error::*;

const NUM_OCTAVE_STEPS: usize = 12;

/// Represents a scale around a note
pub struct Scale {
    base: Note,
    steps: Vec<usize>,
}

impl Scale {
    #[allow(missing_docs)]
    pub fn new(base: Note, scale_name: &str, consts: &Consts) -> Result<Self> {
        let mut steps: Vec<usize> = consts
            .scale_map
            .get(scale_name)
            .ok_or_else(|| -> Error {
                ErrorKind::SpecError(format!(
                    "Unrecongnized scale name: {}",
                    scale_name
                ))
                .into()
            })?
            .clone();
        let steps_total: usize = steps.iter().sum();
        if steps_total != NUM_OCTAVE_STEPS {
            // Complete to the next octave
            steps.push(NUM_OCTAVE_STEPS - (steps_total % NUM_OCTAVE_STEPS));
        }
        Ok(Scale { base, steps })
    }

    #[allow(missing_docs)]
    pub fn from_str(s: &str, consts: &Consts) -> Result<Self> {
        let captures = regex::SCALE_REGEX
            .captures(s)
            .chain_err(|| "Failed to match scale string")?;
        let note_str = captures.get(1).unwrap().as_str();
        let scale_name = captures.get(2).unwrap().as_str();
        Scale::new(note_str.parse()?, scale_name, consts)
    }

    /// Get the default scale size, typically ranging across an octave
    pub fn default_size(&self) -> usize { self.steps.len() }

    /// Get notes of the scale
    pub fn get_notes(&self, num: usize) -> Vec<Note> {
        let mut note = self.base.clone();
        let mut result = vec![note.clone()];
        for step in self.steps.iter().cycle().take(num - 1) {
            note = note.increment(*step);
            result.push(note.clone());
        }
        result
    }

    /// Get the note defined by a `ScaleIndex`
    pub fn at_index(&self, scale_index: &ScaleIndex) -> Note {
        let increment: usize =
            self.steps.iter().take(scale_index.index - 1).sum();
        let increment_adjusted =
            (increment as i8 + scale_index.step_adjustment) as usize;
        self.base.increment(increment_adjusted)
    }
}
