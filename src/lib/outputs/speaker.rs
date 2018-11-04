//! Play music to the device speaker

use core::Output;
use core::Playable;
use errors::*;
use spec::{FromSpec, Spec};

/// Play music to the device speaker
pub struct Speaker {}

impl Speaker {
    #[allow(missing_docs)]
    fn new() -> Self { Speaker {} }
}

impl Output for Speaker {
    fn write(&self, _playable: Playable) {}
}

impl FromSpec for Speaker {
    fn name() -> &'static str { "speaker" }
    fn from_spec(spec: &mut Spec) -> Result<Box<Self>> {
        spec.ensure_all_used()?;
        Ok(Box::new(Speaker::new()))
    }
}
