//! Values that can be played

use std::i32;
use std::iter::Sum;
use std::ops;

/// A single data point in a sound wave
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Playable {
    value: i32,
}

impl Playable {
    #[allow(missing_docs)]
    pub fn new(value: i32) -> Self { Playable { value } }

    #[allow(missing_docs)]
    pub fn get_value(self) -> i32 { self.value }

    /// Get the zero value for the playable
    pub fn zero() -> Playable { Playable::new(0) }
}

impl ops::Add for Playable {
    type Output = Playable;
    fn add(self, other: Self) -> Self {
        // Perform addition in u64, clamp to i32 range, cast to i32
        // TODO: Can we make this faster?
        Playable::new(
            (i64::from(self.value) + i64::from(other.value))
                .max(i64::from(i32::MIN))
                .min(i64::from(i32::MAX)) as i32,
        )
    }
}

impl ops::Mul for Playable {
    type Output = Playable;
    fn mul(self, other: Self) -> Self {
        Playable::new(self.value * other.value)
    }
}

impl ops::Mul<f64> for Playable {
    type Output = Playable;
    fn mul(self, other: f64) -> Self {
        Playable::new((f64::from(self.value) * other) as i32)
    }
}

impl Sum for Playable {
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item = Self> {
        iter.fold(Playable::new(0), ops::Add::add)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_playable() {
        assert_eq!(
            Playable::new(6),
            vec![Playable::new(1), Playable::new(2), Playable::new(3)]
                .into_iter()
                .sum(),
        );
    }
}
