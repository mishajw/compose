//! Values that can be played

/// A single data point in a sound wave
#[derive(Clone, Copy)]
pub struct Playable {
    value: i32,
}

impl Playable {
    #[allow(missing_docs)]
    pub fn new(value: i32) -> Self { Playable { value } }

    #[allow(missing_docs)]
    pub fn get_value(&self) -> i32 { self.value }
}
