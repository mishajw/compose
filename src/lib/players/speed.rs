use core::spec::create::create_player;
use core::spec::create::FromSpec;
use core::spec::Spec;
use core::spec::Value;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use errors::*;

/// Adjust the speed of a child player
pub struct Speed {
    child: Box<Player>,
    scale: f32,
}

impl Speed {
    #[allow(missing_docs)]
    pub fn new(child: Box<Player>, scale: f32) -> Box<Player> {
        Box::new(Speed { child, scale })
    }
}

impl Player for Speed {
    fn play(&mut self, state: &State) -> Playable {
        // TODO: Handle speed decreases
        self.child
            .play(&state.with_tick((state.tick * self.scale as usize) as usize))
    }
}

impl FromSpec<Box<Player>> for Speed {
    fn name() -> &'static str { "speed" }

    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.as_type()?;
        let child = create_player(&mut spec.consume("child")?, consts)?;
        let speed: f32 = spec.consume("speed")?;
        Ok(Speed::new(child, speed))
    }
}
