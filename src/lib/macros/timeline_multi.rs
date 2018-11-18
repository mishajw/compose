use errors::*;
use spec::{Spec, SpecMacro, Value, ValueType};

/// [`Timeline`](../strict.Timeline.html) for multiple inputs
pub struct TimelineMulti {}

impl SpecMacro for TimelineMulti {
    fn name() -> &'static str { "timeline-multi" }
    fn resolve(spec: &mut Spec) -> Result<Value> {
        let event_duration: String = spec.consume("event-duration")?;
        let events: String = spec.consume("events")?;
        spec.ensure_all_used()?;

        Ok(Value::List(
            events
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|event_line| {
                    Value::Spec(
                        Spec::empty()
                            .with("name".into(), "timeline".to_string())
                            .with(
                                "event-duration".into(),
                                event_duration.clone().into_value(),
                            )
                            .with("events".into(), event_line.to_string()),
                    )
                })
                .collect(),
        ))
    }
}
