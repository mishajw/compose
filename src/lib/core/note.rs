use error::*;

use regex::Regex;

/// Notes in an octave
#[derive(Clone)]
pub struct Note {
    note: AbstractNote,
    octave: usize,
}

impl Note {
    #[allow(missing_docs)]
    pub fn to_frequency(&self) -> f32 {
        self.note.to_frequency() * 2f32.powi(self.octave as i32 - 5)
    }

    /// Get the next note
    pub fn next(&self) -> Note {
        match self.note.next() {
            AbstractNote::C => Note {
                note: AbstractNote::C,
                octave: self.octave + 1,
            },
            note => Note {
                note,
                octave: self.octave,
            },
        }
    }

    /// Get the nth next note
    pub fn increment(&self, increment: usize) -> Note {
        let mut note = self.clone();
        for _ in 0..increment {
            note = note.next();
        }
        note
    }
}

impl std::str::FromStr for Note {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref NOTE_REGEX: Regex =
                Regex::new(r"([a-gA-G]#?)([0-9]*)").unwrap();
        }

        let captures = NOTE_REGEX
            .captures(s)
            .ok_or_else(|| ErrorKind::SpecBadValue("note".into(), s.into()))?;
        let abstract_note_str = captures.get(1).unwrap().as_str();
        let octave_str = captures.get(2).unwrap().as_str();

        Ok(Note {
            note: abstract_note_str.parse().chain_err(|| {
                format!("Failed to parse abstract note: {}", abstract_note_str)
            })?,
            octave: octave_str.parse().chain_err(|| {
                format!("Failed to parse note octave: {}", octave_str)
            })?,
        })
    }
}

/// Notes in scale, with no assigned octave
#[derive(Clone)]
#[allow(missing_docs)]
enum AbstractNote {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl AbstractNote {
    /// Return the frequency of the key in the 5th octave
    fn to_frequency(&self) -> f32 {
        match *self {
            AbstractNote::C => 523.25,
            AbstractNote::Cs => 554.37,
            AbstractNote::D => 587.33,
            AbstractNote::Ds => 622.25,
            AbstractNote::E => 659.25,
            AbstractNote::F => 698.46,
            AbstractNote::Fs => 739.99,
            AbstractNote::G => 783.99,
            AbstractNote::Gs => 830.81,
            AbstractNote::A => 880.0,
            AbstractNote::As => 932.33,
            AbstractNote::B => 987.77,
        }
    }

    fn next(&self) -> AbstractNote {
        match *self {
            AbstractNote::C => AbstractNote::Cs,
            AbstractNote::Cs => AbstractNote::D,
            AbstractNote::D => AbstractNote::Ds,
            AbstractNote::Ds => AbstractNote::E,
            AbstractNote::E => AbstractNote::F,
            AbstractNote::F => AbstractNote::Fs,
            AbstractNote::Fs => AbstractNote::G,
            AbstractNote::G => AbstractNote::Gs,
            AbstractNote::Gs => AbstractNote::A,
            AbstractNote::A => AbstractNote::As,
            AbstractNote::As => AbstractNote::B,
            AbstractNote::B => AbstractNote::C,
        }
    }
}

impl std::str::FromStr for AbstractNote {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.as_ref() {
            "c" => Ok(AbstractNote::C),
            "c#" => Ok(AbstractNote::Cs),
            "d" => Ok(AbstractNote::D),
            "d#" => Ok(AbstractNote::Ds),
            "e" => Ok(AbstractNote::E),
            "f" => Ok(AbstractNote::F),
            "f#" => Ok(AbstractNote::Fs),
            "g" => Ok(AbstractNote::G),
            "g#" => Ok(AbstractNote::Gs),
            "a" => Ok(AbstractNote::A),
            "a#" => Ok(AbstractNote::As),
            "b" => Ok(AbstractNote::B),
            s => Err(ErrorKind::SpecBadValue("abstract note".into(), s.into())
                .into()),
        }
    }
}
