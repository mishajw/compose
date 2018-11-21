#![allow(missing_docs)]

use core::spec::{create, Spec, Value};
use core::Time;
use errors::*;

/// Used to keep track of the progress through a composition
pub struct CompositionState {
    /// How far through, in steps, we are through the composition
    pub tick: usize,
    /// Contants of the composition
    pub consts: CompositionConsts,
}

impl CompositionState {
    #[allow(missing_docs)]
    pub fn initial(consts: CompositionConsts) -> Self {
        CompositionState { tick: 0, consts }
    }

    /// Step to the next state in the composition
    pub fn increment(&mut self) { self.tick += 1; }

    /// Get a copy of the state with a custom tick value
    pub fn with_tick(&self, tick: usize) -> Self {
        CompositionState {
            tick,
            consts: self.consts.clone(),
        }
    }
}

#[derive(Clone)]
pub struct CompositionConsts {
    pub sample_hz: f32,
    pub beats_per_minute: f32,
    pub beats_per_bar: f32,
    pub loudness_factor: f32,
    pub reload_time: Time,
}

impl CompositionConsts {
    #[allow(missing_docs)]
    pub fn new(
        sample_hz: f32,
        beats_per_minute: f32,
        beats_per_bar: f32,
        loudness_factor: f32,
        reload_time: Time,
    ) -> Self
    {
        CompositionConsts {
            sample_hz,
            beats_per_minute,
            beats_per_bar,
            loudness_factor,
            reload_time,
        }
    }

    pub fn default() -> Self {
        CompositionConsts::new(44100.0, 120.0, 4.0, 0.3, Time::Ticks(0))
    }
}

impl create::FromSpec<CompositionConsts> for CompositionConsts {
    fn name() -> &'static str { "consts" }
    fn from_spec(
        value: Value,
        consts: &CompositionConsts,
    ) -> Result<CompositionConsts>
    {
        let mut spec: Spec = value.as_type()?;
        let defaults = CompositionConsts::default();
        let consts = CompositionConsts::new(
            spec.consume_with_default("sample-hz", consts.sample_hz)?,
            spec.consume_with_default(
                "beats-per-minute",
                consts.beats_per_minute,
            )?,
            spec.consume_with_default("beats-per-bar", consts.beats_per_bar)?,
            spec.consume_with_default(
                "loudness-factor",
                consts.loudness_factor,
            )?,
            Time::from_spec(
                spec.consume_with_default(
                    "reload-time",
                    Value::Str("0 ticks".to_string()),
                )?,
                &defaults,
            )?,
        );
        spec.ensure_all_used()?;
        Ok(consts)
    }
}
