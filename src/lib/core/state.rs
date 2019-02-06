#![allow(missing_docs)]

use core::Consts;

use std::sync::Arc;

/// Used to keep track of the progress through a composition
pub struct State {
    /// How far through, in 1000ths of a step, we are through the composition
    pub milli_tick: usize,
    /// Contants of the composition
    pub consts: Arc<Consts>,
}

impl State {
    #[allow(missing_docs)]
    pub fn initial(consts: Arc<Consts>) -> Self {
        State {
            milli_tick: 0,
            consts,
        }
    }

    /// Get progress through the composition in ticks
    pub fn tick(&self) -> usize { self.milli_tick / 1000 }

    /// Step to the next state in the composition
    pub fn increment(&mut self) { self.milli_tick += 1000; }

    /// Get a copy of the state with a custom tick value
    pub fn with_tick(&self, tick: usize) -> Self {
        self.with_milli_tick(tick * 1000)
    }

    /// Get a copy of the state with a custom milli tick value
    pub fn with_milli_tick(&self, milli_tick: usize) -> Self {
        State {
            milli_tick,
            consts: self.consts.clone(),
        }
    }
}
