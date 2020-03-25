use core::spec::{FromValue, Spec, Value};
use core::Time;
use error::*;

/// Constants in the composition
pub struct Consts {
    /// How many samples are in a second in the output audio
    pub sample_hz: f64,
    /// How many beats are in a minute (i.e. bpm)
    pub beats_per_minute: f64,
    /// How many beats are in a bar (i.e. time signature)
    pub beats_per_bar: f64,
    /// How loud sound is by default
    pub loudness_factor: f64,
    /// How often to reload the input configuration file
    pub reload_time: Time,
}

impl Consts {
    #[allow(missing_docs)]
    pub fn new(
        sample_hz: f64,
        beats_per_minute: f64,
        beats_per_bar: f64,
        loudness_factor: f64,
        reload_time: Time,
    ) -> Result<Self> {
        Ok(Consts {
            sample_hz,
            beats_per_minute,
            beats_per_bar,
            loudness_factor,
            reload_time,
        })
    }

    /// The default values for the constants
    pub fn default() -> Result<Self> {
        Consts::new(44100.0, 120.0, 4.0, 0.3, Time::Ticks(0))
    }
}

impl FromValue for Consts {
    fn name() -> String {
        "consts".into()
    }
    fn from_value(value: Value, consts: &Consts) -> Result<Consts> {
        let mut spec: Spec = value.into_type(consts)?;
        let consts = Consts::new(
            spec.consume_with_default("sample-hz", consts.sample_hz, consts)?,
            spec.consume_with_default("beats-per-minute", consts.beats_per_minute, consts)?,
            spec.consume_with_default("beats-per-bar", consts.beats_per_bar, consts)?,
            spec.consume_with_default("loudness-factor", consts.loudness_factor, consts)?,
            spec.consume_with_default("reload-time", Time::zero(), consts)?,
        )?;
        spec.ensure_all_used()?;
        Ok(consts)
    }
}
