use consts;
use core::input;
use core::CompositionState;
use errors::*;
use spec::{FromSpec, Value};

/// `input::Bool` defined by a list of booleans
pub struct Timeline {
    /// Events activation, indexed by `[time step][category]`
    events: Vec<bool>,
    event_tick_length: usize,
}

impl Timeline {
    #[allow(missing_docs)]
    pub fn new(events: Vec<bool>, event_tick_length: usize) -> Self {
        Timeline {
            events,
            event_tick_length,
        }
    }

    #[allow(missing_docs)]
    pub fn from_string(events_str: String, event_tick_length: usize) -> Self {
        Timeline {
            events: events_str.chars().map(|c| c == '_').collect(),
            event_tick_length,
        }
    }
}

impl input::Bool for Timeline {
    fn get(&mut self, state: &CompositionState) -> bool {
        let event_index = state.tick as usize
            % (self.events.len() * self.event_tick_length)
            / (self.event_tick_length);
        self.events[event_index]
    }
}

impl FromSpec<Box<input::Bool>> for Timeline {
    fn name() -> &'static str { "timeline" }
    fn from_spec(value: Value) -> Result<Box<input::Bool>> {
        let mut spec = value.as_spec()?;
        let event_tick_sec: i32 = spec.consume("event-tick-sec")?;
        let events: String = spec.consume("events")?;

        Ok(Box::new(Timeline::from_string(
            events,
            (event_tick_sec as f32 * consts::SAMPLE_HZ) as usize,
        )))
    }
}
