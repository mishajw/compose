use core::input;
use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::State;
use core::Time;
use error::*;

field_decl!(
    EVENTS,
    String,
    "The events, where a space means false and other characters mean true"
);
field_decl!(EVENT_DURATION, Time, "The duration of an event");

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
        let event_index = state.tick() as usize
            % (self.events.len() * self.event_duration.to_ticks(&state.consts))
            / (self.event_duration.to_ticks(&state.consts));
        self.events[event_index]
    }
}

impl Tree for Timeline {
    fn to_tree(&self) -> &Tree { self as &Tree }
}

impl SpecType for Timeline {
    fn name() -> String { "timeline".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![EVENTS.to_description(), EVENT_DURATION.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let event_duration = EVENT_DURATION.get(&mut spec, consts)?;
        let events = EVENTS.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(Timeline::from_string(events, event_duration))
    }
}
