use core::spec::FromValue;
use core::spec::Spec;
use core::spec::Value;
use core::Consts;
use core::Time;
use error::*;
use hound;
use inputs::Buffer;
use players::PlayInput;
use players::Speed;

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
            PlayInput::player(Buffer::bounded(buffer)),
            f64::from(sample_hz) / consts.sample_hz,
        )?)
    }
}

impl FromValue<Speed> for Sample {
    fn name() -> &'static str { "sample" }

    fn from_value(value: Value, consts: &Consts) -> Result<Speed> {
        let mut spec: Spec = value.into_type(consts)?;
        let wav_path: String = spec.consume("path", consts)?;
        let start: Time = spec.consume("start", consts)?;
        let duration: Time = spec.consume("duration", consts)?;
        spec.ensure_all_used()?;
        Sample::player(wav_path, start, duration, consts)
    }
}
