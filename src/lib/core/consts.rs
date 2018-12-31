use core::spec::yaml;
use core::spec::{create, Spec, Value};
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
    pub sample_hz: f32,
    /// How many beats are in a minute (i.e. bpm)
    pub beats_per_minute: f32,
    /// How many beats are in a bar (i.e. time signature)
    pub beats_per_bar: f32,
    /// How loud sound is by default
    pub loudness_factor: f32,
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
        sample_hz: f32,
        beats_per_minute: f32,
        beats_per_bar: f32,
        loudness_factor: f32,
        reload_time: Time,
        scale_definition_path: String,
        chord_definition_path: String,
    ) -> Result<Self>
    {
        let scale_map = Self::create_scale_map(scale_definition_path)?;
        let chord_map = Self::create_chord_map(chord_definition_path)?;
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
            DEFAULT_SCALE_DEFINITION_PATH.into(),
            DEFAULT_CHORD_DEFINITION_PATH.into(),
        )
    }

    /// Get map from scale to list of step sizes
    fn create_scale_map(
        scale_definition_path: String,
    ) -> Result<HashMap<String, Vec<usize>>> {
        let mut scale_spec = yaml::read(Path::new(&scale_definition_path))?;
        scale_spec
            .value_names()
            .into_iter()
            .map(|name| {
                scale_spec
                    .consume_list::<i32>(&name)
                    .map(|list| list.into_iter().map(|i| i as usize).collect())
                    .map(|list| (name, list))
            })
            .collect::<Result<_>>()
    }

    /// Get map from chord to list of step sizes
    fn create_chord_map(
        chord_definition_path: String,
    ) -> Result<HashMap<String, Vec<ScaleIndex>>> {
        let mut chord_spec = yaml::read(Path::new(&chord_definition_path))?;
        chord_spec
            .value_names()
            .into_iter()
            .map(|name| {
                chord_spec
                    .consume_list::<String>(&name)
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

impl create::FromSpec<Consts> for Consts {
    fn name() -> &'static str { "consts" }
    fn from_spec(value: Value, consts: &Consts) -> Result<Consts> {
        let mut spec: Spec = value.as_type()?;
        let consts = Consts::new(
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
                &consts,
            )?,
            spec.consume_with_default(
                "scale-definition-path",
                DEFAULT_SCALE_DEFINITION_PATH.to_string(),
            )?,
            spec.consume_with_default(
                "chord-definition-path",
                DEFAULT_CHORD_DEFINITION_PATH.to_string(),
            )?,
        )?;
        spec.ensure_all_used()?;
        Ok(consts)
    }
}
