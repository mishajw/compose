use core::spec::yaml;
use core::spec::{FromValue, Spec, Value};
use core::ScaleIndex;
use core::Time;
use error::*;

use std::collections::HashMap;
use std::path::Path;

const DEFAULT_SCALE_DEFINITION_PATH: &str = "resources/scales.yaml";
const DEFAULT_CHORD_DEFINITION_PATH: &str = "resources/chords.yaml";

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
    /// Maps scale names to scales
    pub scale_map: HashMap<String, Vec<usize>>,
    /// Maps chord names to chords
    pub chord_map: HashMap<String, Vec<ScaleIndex>>,
}

impl Consts {
    #[allow(missing_docs)]
    pub fn new(
        sample_hz: f64,
        beats_per_minute: f64,
        beats_per_bar: f64,
        loudness_factor: f64,
        reload_time: Time,
        scale_map: HashMap<String, Vec<usize>>,
        chord_map: HashMap<String, Vec<ScaleIndex>>,
    ) -> Result<Self>
    {
        Ok(Consts {
            sample_hz,
            beats_per_minute,
            beats_per_bar,
            loudness_factor,
            reload_time,
            scale_map,
            chord_map,
        })
    }

    /// The default values for the constants
    pub fn default() -> Result<Self> {
        Consts::new(
            44100.0,
            120.0,
            4.0,
            0.3,
            Time::Ticks(0),
            HashMap::new(),
            HashMap::new(),
        )
    }

    /// Get map from scale to list of step sizes
    fn create_scale_map(
        scale_definition_path: String,
        consts: &Consts,
    ) -> Result<HashMap<String, Vec<usize>>>
    {
        let mut scale_spec = yaml::read(Path::new(&scale_definition_path))?;
        scale_spec
            .value_names()
            .into_iter()
            .map(|name| {
                scale_spec
                    .consume_list::<i32>(&name, consts)
                    .map(|list| list.into_iter().map(|i| i as usize).collect())
                    .map(|list| (name, list))
            })
            .collect::<Result<_>>()
    }

    /// Get map from chord to list of step sizes
    fn create_chord_map(
        chord_definition_path: String,
        consts: &Consts,
    ) -> Result<HashMap<String, Vec<ScaleIndex>>>
    {
        let mut chord_spec = yaml::read(Path::new(&chord_definition_path))?;
        chord_spec
            .value_names()
            .into_iter()
            .map(|name| {
                chord_spec
                    .consume_list::<String>(&name, consts)
                    .and_then(|list| {
                        list.into_iter()
                            .map(|s| s.parse())
                            .collect::<Result<_>>()
                    })
                    .map(|list| (name, list))
            })
            .collect::<Result<_>>()
    }
}

impl FromValue for Consts {
    fn name() -> String { "consts".into() }
    fn from_value(value: Value, consts: &Consts) -> Result<Consts> {
        let mut spec: Spec = value.into_type(consts)?;
        let consts = Consts::new(
            spec.consume_with_default("sample-hz", consts.sample_hz, consts)?,
            spec.consume_with_default(
                "beats-per-minute",
                consts.beats_per_minute,
                consts,
            )?,
            spec.consume_with_default(
                "beats-per-bar",
                consts.beats_per_bar,
                consts,
            )?,
            spec.consume_with_default(
                "loudness-factor",
                consts.loudness_factor,
                consts,
            )?,
            spec.consume_with_default("reload-time", Time::zero(), consts)?,
            Consts::create_scale_map(
                spec.consume_with_default(
                    "scale-definition-path",
                    DEFAULT_SCALE_DEFINITION_PATH.to_string(),
                    consts,
                )?,
                consts,
            )?,
            Consts::create_chord_map(
                spec.consume_with_default(
                    "chord-definition-path",
                    DEFAULT_CHORD_DEFINITION_PATH.to_string(),
                    consts,
                )?,
                consts,
            )?,
        )?;
        spec.ensure_all_used()?;
        Ok(consts)
    }
}
