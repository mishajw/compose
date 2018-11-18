use core::input;
use core::CompositionState;
use core::Time;
use errors::*;
use spec::{FromSpec, Spec, Value};

/// `input::Bool` defined by a list of booleans
pub struct Timeline {
    /// Events activation, indexed by `[time step][category]`
    events: Vec<bool>,
    event_duration: Time,
}

impl Timeline {
    #[allow(missing_docs)]
    pub fn new(events: Vec<bool>, event_duration: Time) -> Self {
        Timeline {
            events,
            event_duration,
        }
    }

    #[allow(missing_docs)]
    pub fn from_string(events_str: String, event_duration: Time) -> Self {
        Timeline {
            events: events_str.chars().map(|c| c != '_').collect(),
            event_duration,
        }
    }
}

impl input::Bool for Timeline {
    fn get(&mut self, state: &CompositionState) -> bool {
        let event_index = state.tick as usize
            % (self.events.len() * self.event_duration.to_ticks(&state.consts))
            / (self.event_duration.to_ticks(&state.consts));
        self.events[event_index]
    }
}

impl FromSpec<Box<input::Bool>> for Timeline {
    fn name() -> &'static str { "timeline" }
    fn from_spec(value: Value) -> Result<Box<input::Bool>> {
        let mut spec: Spec = value.as_type()?;
        let event_duration = Time::from_spec(spec.consume("event-duration")?)?;
        let events: String = spec.consume("events")?;
        spec.ensure_all_used()?;

        Ok(Box::new(Timeline::from_string(events, event_duration)))
    }
}
