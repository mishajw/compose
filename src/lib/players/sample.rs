use core::spec::create::FromSpec;
use core::spec::Spec;
use core::spec::Value;
use core::Consts;
use core::Player;
use core::Time;
use errors::*;
use hound;
use inputs::Buffer;
use players::PlayInput;
use players::Speed;

/// Sample music from a .wav file
pub struct Sample {}

impl Sample {
    #[allow(missing_docs)]
    pub fn new(
        wav_path: String,
        start: Time,
        duration: Time,
        consts: &Consts,
    ) -> Result<Box<Player>>
    {
        let mut reader = hound::WavReader::open(&wav_path)
            .chain_err(|| format!("Failed to open .wav file: {}", wav_path))?;
        let start_seconds = start.to_seconds(consts);
        let duration_seconds = duration.to_seconds(consts);
        let sample_hz = reader.spec().sample_rate;

        // Skip to the part of the sample we want
        reader
            .seek((sample_hz as f32 * start_seconds) as u32)
            .chain_err(|| {
                format!("Failed to get seek to {} seconds", start_seconds)
            })?;

        // Extract the samples we need
        let buffer: Vec<f32> = reader
            .samples::<i32>()
            .take((sample_hz as f32 * duration_seconds) as usize)
            .map(|r| r.map(|i| i as f32))
            .collect::<std::result::Result<_, _>>()
            .chain_err(|| "Failed to read sample")?;

        Ok(Speed::new(
            PlayInput::new(Buffer::new(buffer)),
            sample_hz as f32 / consts.sample_hz,
        ))
    }
}

impl FromSpec<Box<Player>> for Sample {
    fn name() -> &'static str { "sample" }

    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let wav_path: String = spec.consume("path")?;
        let start: Time = Time::from_spec(spec.consume("start")?, consts)?;
        let duration: Time =
            Time::from_spec(spec.consume("duration")?, consts)?;
        spec.ensure_all_used()?;
        Ok(Sample::new(wav_path, start, duration, consts)?)
    }
}
