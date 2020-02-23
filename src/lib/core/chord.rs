use core::regex;
use core::Consts;
use core::Note;
use core::Scale;
use core::ScaleIndex;
use error::*;

/// A collection of notes that can be created using chord names and scales
pub struct Chord {
    notes: Vec<Note>,
}

impl Chord {
    /// Create a chord from a note, e.g. "a4 dim"
    pub fn from_note_chord(note: Note, chord_name: &str, consts: &Consts) -> Result<Self> {
        let scale = Scale::new(note, "maj", consts)?;

        let chord_indices = consts.chord_map.get(chord_name).ok_or_else(|| -> Error {
            ErrorKind::SpecError(format!("Unrecognized chord name: {}", chord_name)).into()
        })?;

        let notes = chord_indices.iter().map(|i| scale.at_index(&i)).collect();

        Ok(Chord { notes })
    }

    /// Create a chord from a scale index, e.g. "a major IV"
    pub fn from_scale_index(scale: Scale, index: usize) -> Self {
        let notes = [index, index + 2, index + 4]
            .iter()
            .map(|i| ScaleIndex::new(*i, 0))
            .map(|i| scale.at_index(&i))
            .collect();
        Chord { notes }
    }

    #[allow(missing_docs)]
    pub fn from_str(s: &str, consts: &Consts) -> Result<Self> {
        if let Some(capture) = regex::NOTE_CHORD_REGEX.captures(s) {
            let note: Note = capture.get(1).unwrap().as_str().parse()?;
            let chord_name = capture.get(2).unwrap().as_str();
            Chord::from_note_chord(note, chord_name, consts)
        } else if let Some(capture) = regex::SCALE_INDEX_CHORD_REGEX.captures(s) {
            let scale = Scale::from_str(capture.get(1).unwrap().as_str(), consts)?;
            let index = capture
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .chain_err(|| "Failed to parse index for scale index chord")?;
            Ok(Chord::from_scale_index(scale, index))
        } else {
            bail!(ErrorKind::SpecError(format!(
                "Failed to recognize chord format: {}",
                s
            )));
        }
    }

    /// Get the notes of the chord
    pub fn into_notes(self) -> Vec<Note> {
        self.notes
    }
}
