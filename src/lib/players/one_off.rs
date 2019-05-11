use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Input;
use core::Playable;
use core::Player;
use core::State;
use error::*;

field_decl!(CHILD, Box<Player>, "Child to play");
field_decl!(
    INPUT,
    Box<Input>,
    "When turns to positive, the child is played from the beginning"
);

/// Plays the beginning of a player every time it's triggered
pub struct OneOff {
    child: Box<Player>,
    input: Box<Input>,
    child_play_history: Vec<Playable>,
    play_index: usize,
}

impl OneOff {
    #[allow(missing_docs)]
    pub fn new(child: Box<Player>, input: Box<Input>) -> OneOff {
        OneOff {
            child,
            input,
            child_play_history: Vec::new(),
            play_index: 0,
        }
    }
}

impl Player for OneOff {
    fn play(&mut self, state: &State) -> Playable {
        if self.input.get(state) <= 0.0 {
            self.play_index = 0;
            return Playable::zero();
        } else {
            self.play_index += 1;
        }

        while self.play_index >= self.child_play_history.len() {
            self.child_play_history
                .push(self.child.play(&state.with_tick(self.play_index)));
        }

        self.child_play_history[self.play_index]
    }
}

impl Tree for OneOff {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children(&self) -> Vec<&Tree> {
        vec![self.child.to_tree(), self.input.to_tree()]
    }
}

impl SpecType for OneOff {
    fn name() -> String { "one-off".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILD.to_description(), INPUT.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let child = CHILD.get(&mut spec, consts)?;
        let input = INPUT.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(OneOff::new(child, input))
    }
}
