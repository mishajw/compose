use core::input;
use core::spec::create;
use core::spec::{Spec, Value};
use core::tree::Tree;
use core::Consts;
use core::State;
use core::Time;
use error::*;

/// `input::Bool` defined by a list of booleans
pub struct Timeline {
    /// Events activation, indexed by `[time step][category]`
    events: Vec<bool>,
    event_duration: Time,
}

impl Timeline {
    #[allow(missing_docs)]
    pub fn bool(events: Vec<bool>, event_duration: Time) -> Box<input::Bool> {
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
    fn get(&mut self, state: &State) -> bool {
        let event_index = state.tick as usize
            % (self.events.len() * self.event_duration.to_ticks(&state.consts))
            / (self.event_duration.to_ticks(&state.consts));
        self.events[event_index]
    }
}

impl Tree for Timeline {
    fn to_tree(&self) -> &Tree { self as &Tree }
}

impl create::FromSpec<Box<input::Bool>> for Timeline {
    fn name() -> &'static str { "timeline" }
    fn from_spec(value: Value, consts: &Consts) -> Result<Box<input::Bool>> {
        let mut spec: Spec = value.into_type()?;
        let event_duration =
            Time::from_spec(spec.consume("event-duration")?, consts)?;
        let events: String = spec.consume("events")?;
        spec.ensure_all_used()?;

        Ok(Timeline::from_string(events, event_duration))
    }
}
