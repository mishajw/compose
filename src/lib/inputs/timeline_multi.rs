use consts;
use core::input;
use errors::*;
use inputs::Timeline;
use spec::{FromSpec, Value};

/// [`Timeline`](../strict.Timeline.html) for multiple inputs
pub struct TimelineMulti {}

impl FromSpec<Vec<Box<input::Bool>>> for TimelineMulti {
    fn name() -> &'static str { "timeline-multi" }
    fn from_spec(value: Value) -> Result<Vec<Box<input::Bool>>> {
        let mut spec = value.as_spec()?;
        let event_tick_sec: i32 = spec.consume("event-tick-sec")?;
        let event_tick_length =
            (event_tick_sec as f32 * consts::SAMPLE_HZ) as usize;
        let events: String = spec.consume("events")?;
        Ok(events
            .split("\n")
            .map(|l| -> Box<input::Bool> {
                Box::new(Timeline::from_string(l.into(), event_tick_length))
            })
            .collect())
    }
}
