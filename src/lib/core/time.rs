use core::spec::{create, Value};
use core::CompositionConsts;
use errors::*;
use std::time::Duration;

/// Amount of time in different measurements
#[derive(Clone)]
pub enum Time {
    /// Amount of ticks
    Ticks(usize),
    /// Seconds in real time
    Seconds(f32),
    /// Beats in a bar
    Beats(f32),
    /// Composed of beats
    Bars(f32),
}

impl Time {
    #[allow(missing_docs)]
    pub fn to_ticks(&self, consts: &CompositionConsts) -> usize {
        match self {
            Time::Ticks(ticks) => ticks.clone(),
            Time::Seconds(seconds) => (seconds * consts.sample_hz) as usize,
            other => Time::Seconds(other.to_seconds(consts)).to_ticks(consts),
        }
    }

    #[allow(missing_docs)]
    pub fn to_seconds(&self, consts: &CompositionConsts) -> f32 {
        match self {
            Time::Seconds(seconds) => seconds.clone(),
            Time::Ticks(ticks) => ticks.clone() as f32 / consts.sample_hz,
            Time::Beats(beats) => (beats * 60.0) / consts.beats_per_minute,
            bars => Time::Beats(bars.to_beats(consts)).to_seconds(consts),
        }
    }

    #[allow(missing_docs)]
    pub fn to_beats(&self, consts: &CompositionConsts) -> f32 {
        match self {
            Time::Beats(beats) => beats.clone(),
            Time::Bars(bars) => bars * consts.beats_per_bar,
            Time::Seconds(seconds) => {
                (seconds * consts.beats_per_minute) / 60.0
            }
            ticks => Time::Seconds(ticks.to_seconds(consts)).to_beats(consts),
        }
    }

    #[allow(missing_docs)]
    pub fn to_duration(&self, consts: &CompositionConsts) -> Duration {
        return Duration::from_nanos((self.to_seconds(consts) * 1e9) as u64);
    }

    /// Check if represents no time
    pub fn is_zero(&self) -> bool {
        match self {
            Time::Ticks(ticks) => ticks == &0,
            Time::Seconds(seconds) => seconds == &0.0,
            Time::Beats(beats) => beats == &0.0,
            Time::Bars(bars) => bars == &0.0,
        }
    }
}

impl create::FromSpec<Time> for Time {
    fn name() -> &'static str { "time" }
    fn from_spec(value: Value, _consts: &CompositionConsts) -> Result<Time> {
        let string: String = value.as_type()?;
        match string.trim().split(" ").collect::<Vec<_>>().as_slice() {
            [number, "ticks"] => Ok(Time::Ticks(
                number.parse().chain_err(|| "Failed to parse tick number")?,
            )),
            [number, "seconds"] => Ok(Time::Seconds(
                number
                    .parse()
                    .chain_err(|| "Failed to parse seconds number")?,
            )),
            [number, "beats"] => Ok(Time::Beats(
                number
                    .parse()
                    .chain_err(|| "Failed to parse beats number")?,
            )),
            [number, "bars"] => Ok(Time::Bars(
                number.parse().chain_err(|| "Failed to parse bars number")?,
            )),
            _ => Err(ErrorKind::SpecBadValue(
                "time".into(),
                format!("Didn't recognize qualifier in {}", string),
            )
            .into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_ticks() {
        let consts = &CompositionConsts::default().unwrap();
        assert_eq!(Time::Ticks(1000).to_ticks(consts), 1000);
        assert_eq!(Time::Seconds(3.0).to_ticks(consts), 44100 * 3);
        assert_eq!(Time::Bars(2.0).to_ticks(consts), 44100 * 4);
        assert_eq!(Time::Beats(1.0).to_ticks(consts), 44100 / 2);
    }

    #[test]
    fn test_to_seconds() {
        let consts = &CompositionConsts::default().unwrap();
        assert_eq!(Time::Ticks(44100).to_seconds(consts), 1.0);
        assert_eq!(Time::Seconds(3.0).to_seconds(consts), 3.0);
        assert_eq!(Time::Bars(2.0).to_seconds(consts), 4.0);
        assert_eq!(Time::Beats(1.0).to_seconds(consts), 0.5);
    }

    #[test]
    fn test_to_beats() {
        let consts = &CompositionConsts::default().unwrap();
        assert_eq!(Time::Ticks(44100).to_beats(consts), 2.0);
        assert_eq!(Time::Seconds(3.0).to_beats(consts), 6.0);
        assert_eq!(Time::Bars(2.0).to_beats(consts), 8.0);
        assert_eq!(Time::Beats(1.0).to_beats(consts), 1.0);
    }
}
