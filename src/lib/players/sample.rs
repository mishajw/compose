use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::Consts;
use core::Time;
use error::*;
use hound;
use inputs::Buffer;
use players::PlayInput;
use players::Speed;

field_decl!(PATH, String, "Path of the .wav file to sample from");
field_decl!(START, Time, "Start of the sample in the .wav file");
field_decl!(DURATION, Time, "Duration of the sample in the .wav file");

/// Sample music from a .wav file
pub struct Sample {}

impl Sample {
    #[allow(missing_docs)]
    pub fn player(
        wav_path: String,
        start: Time,
        duration: Time,
        consts: &Consts,
    ) -> Result<Speed>
    {
        let mut reader = hound::WavReader::open(&wav_path)
            .chain_err(|| format!("Failed to open .wav file: {}", wav_path))?;
        let start_seconds = start.to_seconds(consts);
        let duration_seconds = duration.to_seconds(consts);
        let sample_hz = reader.spec().sample_rate;

        // Skip to the part of the sample we want
        reader
            .seek((f64::from(sample_hz) * start_seconds) as u32)
            .chain_err(|| {
                format!("Failed to get seek to {} seconds", start_seconds)
            })?;

        // Extract the samples we need
        let buffer: Vec<f64> = reader
            .samples::<i32>()
            .take((f64::from(sample_hz) * duration_seconds) as usize)
            .map(|r| r.map(f64::from))
            .collect::<std::result::Result<_, _>>()
            .chain_err(|| "Failed to read sample")?;

        Ok(Speed::player(
            PlayInput::new(Buffer::new(buffer), consts),
            f64::from(sample_hz) / consts.sample_hz,
        )?)
    }
}

impl SpecType<Speed> for Sample {
    fn name() -> String { "sample".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![
            PATH.to_description(),
            START.to_description(),
            DURATION.to_description(),
        ]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Speed> {
        let wav_path = PATH.get(&mut spec, consts)?;
        let start = START.get(&mut spec, consts)?;
        let duration = DURATION.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Sample::player(wav_path, start, duration, consts)
    }
}
