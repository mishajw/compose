use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::CompositionConsts;
use core::CompositionState;
use core::Time;
use errors::*;

/// `input::Bool` defined by a list of booleans
pub struct Timeline {
    /// Events activation, indexed by `[time step][category]`
    events: Vec<bool>,
    event_duration: Time,
}

impl Timeline {
    #[allow(missing_docs)]
    pub fn new(events: Vec<bool>, event_duration: Time) -> Box<input::Bool> {
        Box::new(Timeline {
            events,
            event_duration,
        })
    }

    #[allow(missing_docs)]
    pub fn from_string(
        events_str: String,
        event_duration: Time,
    ) -> Box<input::Bool>
    {
        Box::new(Timeline {
            events: events_str.chars().map(|c| c != '_').collect(),
            event_duration,
        })
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

impl create::FromSpec<Box<input::Bool>> for Timeline {
    fn name() -> &'static str { "timeline" }
    fn from_spec(
        value: Value,
        consts: &CompositionConsts,
    ) -> Result<Box<input::Bool>>
    {
        let mut spec: Spec = value.as_type()?;
        let event_duration =
            Time::from_spec(spec.consume("event-duration")?, consts)?;
        let events: String = spec.consume("events")?;
        spec.ensure_all_used()?;

        Ok(Timeline::from_string(events, event_duration))
    }
}
