#![allow(missing_docs)]

use core::Consts;

use std::sync::Arc;

/// Used to keep track of the progress through a composition
pub struct State {
    /// How far through, in steps, we are through the composition
    pub tick: usize,
    /// Contants of the composition
    pub consts: Arc<Consts>,
}

impl State {
    #[allow(missing_docs)]
    pub fn initial(consts: Arc<Consts>) -> Self { State { tick: 0, consts } }

    /// Step to the next state in the composition
    pub fn increment(&mut self) { self.tick += 1; }

    /// Get a copy of the state with a custom tick value
    pub fn with_tick(&self, tick: usize) -> Self {
        State {
            tick,
            consts: self.consts.clone(),
        }
    }
}
