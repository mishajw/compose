//! Regular expressions used for parsing items from core

use regex::Regex;

const NOTE_REGEX_STR: &str = r"([a-g]#?)([0-9]*)";

lazy_static! {

    /// Represents a note, e.g. a#4, b, f#5
    pub static ref NOTE_REGEX: Regex =
        Regex::new(&NOTE_REGEX_STR).unwrap();

    /// Represents a scale index, e.g. 1, 2, b3, s6
    pub static ref SCALE_INDEX_REGEX: Regex =
        Regex::new(r"([sb]*)([0-9]+)").unwrap();
}
