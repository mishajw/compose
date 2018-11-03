#![allow(missing_docs)]

use std::time::Duration;

/// Used to keep track of the progress through a composition
pub struct CompositionState {
    /// How far through, in time, we are through the composition
    pub time: Duration,
    /// How far through, in steps, we are through the composition
    pub tick: u64,
    /// The frequency of the composition
    pub frequency: u64,
}

impl CompositionState {
    #[allow(missing_docs)]
    pub fn initial(frequency: u64) -> Self {
        CompositionState {
            time: Duration::from_secs(0),
            tick: 0,
            frequency,
        }
    }

    /// Step to the next state in the composition
    pub fn increment(&mut self) {
        self.tick += 1;
        self.time =
            Duration::from_nanos((self.tick * 1_000_000) / self.frequency);
    }
}
