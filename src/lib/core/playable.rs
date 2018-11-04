//! Values that can be played

use std::ops;

/// A single data point in a sound wave
#[derive(Clone, Copy)]
pub struct Playable {
    value: i32,
}

impl Playable {
    #[allow(missing_docs)]
    pub fn new(value: i32) -> Self { Playable { value } }

    #[allow(missing_docs)]
    pub fn get_value(self) -> i32 { self.value }
}

impl ops::Mul for Playable {
    type Output = Playable;
    fn mul(self, other: Self) -> Self {
        Playable::new(self.value * other.value)
    }
}

impl ops::Mul<f32> for Playable {
    type Output = Playable;
    fn mul(self, other: f32) -> Self {
        Playable::new((self.value as f32 * other) as i32)
    }
}
