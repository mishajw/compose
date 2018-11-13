use core::input;
use core::Time;
use errors::*;
use inputs::Timeline;
use spec::{FromSpec, Spec, Value};

/// [`Timeline`](../strict.Timeline.html) for multiple inputs
pub struct TimelineMulti {}

impl FromSpec<Vec<Box<input::Bool>>> for TimelineMulti {
    fn name() -> &'static str { "timeline-multi" }
    fn from_spec(value: Value) -> Result<Vec<Box<input::Bool>>> {
        let mut spec: Spec = value.as_type()?;
        let event_duration = Time::from_spec(spec.consume("event-duration")?)?;
        let events: String = spec.consume("events")?;
        Ok(events
            .split("\n")
            .map(|l| -> Box<input::Bool> {
                Box::new(Timeline::from_string(
                    l.into(),
                    event_duration.clone(),
                ))
            })
            .collect())
    }
}
