//! Regular expressions used for parsing items from core

use regex::Regex;

const NOTE_REGEX_STR: &str = r"^([a-g]#?)([0-9]*)$";

/// Strip groups in a regular expression, and make the entire regex a group
///
/// Used for putting regex in other regex
fn change_groups(s: &str) -> String {
    let mut s = s.to_string();
    s = s.replace("(", "").replace(")", "");
    if s.starts_with('^') {
        s.remove(0);
    }
    if s.ends_with('$') {
        let len = s.len();
        s.remove(len - 1);
    }
    format!("({})", s)
}

lazy_static! {
    static ref SCALE_REGEX_STR: String = format!(
        r"^{} +([a-z0-9-]+)$",
        change_groups(NOTE_REGEX_STR));

    /// Represents a note, e.g. a#4, b, f#5
    pub static ref NOTE_REGEX: Regex =
        Regex::new(&NOTE_REGEX_STR).unwrap();

    /// Represents a scale, e.g: a min, c maj, f#3 dim
    pub static ref SCALE_REGEX: Regex =
        Regex::new(&SCALE_REGEX_STR).unwrap();

    /// Represents a scale, e.g: a min, c maj, f#3 dim
    pub static ref NOTE_CHORD_REGEX: Regex =
        Regex::new(&format!(
            r"^{} +([a-z0-9-]+)$",
            change_groups(NOTE_REGEX_STR))).unwrap();

    /// Represents a scale, e.g: a min, c maj, f#3 dim
    pub static ref SCALE_INDEX_CHORD_REGEX: Regex =
        Regex::new(&format!(
            r"^{} +([0-9]+)$",
            change_groups(&SCALE_REGEX_STR))).unwrap();

    /// Represents a scale index, e.g. 1, 2, b3, s6
    pub static ref SCALE_INDEX_REGEX: Regex =
        Regex::new(r"^([sb]*)([0-9]+)$").unwrap();
}
