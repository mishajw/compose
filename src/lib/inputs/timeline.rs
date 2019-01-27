use core::input;
use core::spec::FromValue;
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
    pub fn bool(events: Vec<bool>, event_duration: Time) -> Timeline {
        Timeline {
            events,
            event_duration,
        }
    }

    #[allow(missing_docs)]
    pub fn from_string(events_str: String, event_duration: Time) -> Timeline {
        Timeline {
            events: events_str.chars().map(|c| c != '_').collect(),
            event_duration,
        }
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

impl FromValue for Timeline {
    fn name() -> &'static str { "timeline" }
    fn from_value(value: Value, consts: &Consts) -> Result<Self> {
        let mut spec: Spec = value.into_type(consts)?;
        let event_duration = spec.consume("event-duration", consts)?;
        let events: String = spec.consume("events", consts)?;
        spec.ensure_all_used()?;
        Ok(Timeline::from_string(events, event_duration))
    }
}
