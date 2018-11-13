#![allow(missing_docs)]

use errors::*;
use spec::{FromSpec, Spec, Value};

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
}

pub struct CompositionConsts {
    pub sample_hz: f32,
    pub beats_per_minute: f32,
    pub beats_per_bar: f32,
    pub loudness_factor: f32,
}

impl CompositionConsts {
    #[allow(missing_docs)]
    pub fn new(
        sample_hz: f32,
        beats_per_minute: f32,
        beats_per_bar: f32,
        loudness_factor: f32,
    ) -> Self
    {
        CompositionConsts {
            sample_hz,
            beats_per_minute,
            beats_per_bar,
            loudness_factor,
        }
    }
}

impl FromSpec<CompositionConsts> for CompositionConsts {
    fn name() -> &'static str { "consts" }
    fn from_spec(value: Value) -> Result<CompositionConsts> {
        let mut spec: Spec = value.as_type()?;
        let consts = CompositionConsts::new(
            spec.consume_with_default("sample-hz", 44100.0)?,
            spec.consume_with_default("beats-per-minute", 120.0)?,
            spec.consume_with_default("beats-per-bar", 4.0)?,
            spec.consume_with_default("loudness-factor", 0.3)?,
        );
        spec.ensure_all_used()?;
        Ok(consts)
    }
}
