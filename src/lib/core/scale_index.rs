use core::regex;
use error::*;

use std::str::FromStr;

/// Represents a note in a scale
pub struct ScaleIndex {
    /// Index in the scale, one-based
    pub index: usize,
    /// Half step adjustment on the indexed note
    pub step_adjustment: i8,
}

impl ScaleIndex {
    #[allow(missing_docs)]
    pub fn new(index: usize, step_adjustment: i8) -> Self {
        ScaleIndex {
            index,
            step_adjustment,
        }
    }

    /// Get the index offsetted by some number
    pub fn offset(&self, n: usize) -> Self {
        ScaleIndex::new(self.index + n, self.step_adjustment)
    }
}

impl FromStr for ScaleIndex {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let captures =
            regex::SCALE_INDEX_REGEX.captures(s).ok_or_else(|| {
                ErrorKind::SpecBadValue("scale_index".into(), s.into())
            })?;
        let step_adjustment_str = captures.get(1).unwrap().as_str();
        let index: usize = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .chain_err(|| "Failed to parse scale index as integer")?;

        Ok(ScaleIndex::new(
            index,
            get_step_adjustment(step_adjustment_str),
        ))
    }
}

fn get_step_adjustment(s: &str) -> i8 {
    let mut result = 0;
    for c in s.chars() {
        match c {
            's' => result += 1,
            'b' => result -= 1,
            _ => {}
        }
    }
    result
}
