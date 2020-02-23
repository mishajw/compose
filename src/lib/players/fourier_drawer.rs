use sfml::graphics::{Color, RenderWindow};

use core::spec::{Spec, SpecField, SpecFieldDescription, SpecType};
use core::tree::Tree;
use core::Time;
use core::{Consts, Playable, Player, State};
use error::*;
use gui::Drawable;
use {fourier, SampleBucketer};

field_decl!(CHILD, Box<dyn Player>, "Fourier of this child is drawn");
field_decl!(
    BUFFER_SIZE,
    Time,
    "How much time is used to make the fourier transform",
    |_| Time::Seconds(0.1)
);

/// Displays the fourier transform of an input child
pub struct FourierDrawer {
    player: Box<dyn Player>,
    buffer_size: usize,
    buffer: Vec<i32>,
    sample_bucketer: SampleBucketer,
}

impl FourierDrawer {
    #[allow(missing_docs)]
    pub fn new(player: Box<dyn Player>, buffer_size: usize) -> FourierDrawer {
        FourierDrawer {
            player,
            buffer_size,
            buffer: Vec::with_capacity(buffer_size),
            sample_bucketer: SampleBucketer::new(buffer_size, 600),
        }
    }

    fn set_up_sample_bucketer(&mut self) {
        assert_eq!(self.buffer.len(), self.buffer_size);
        let fourier = fourier(&self.buffer);
        fourier
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| self.sample_bucketer.add_sample(v, i));
    }
}

impl Player for FourierDrawer {
    fn play(&mut self, state: &State) -> Playable {
        let playable = self.player.play(state);
        if self.buffer.len() < self.buffer_size {
            self.buffer.push(playable.get_value());
        } else {
            self.set_up_sample_bucketer();
            self.buffer.clear();
        }
        playable
    }
}

impl Tree for FourierDrawer {
    fn to_tree(&self) -> &dyn Tree {
        self
    }

    fn get_children(&self) -> Vec<&dyn Tree> {
        vec![self.player.to_tree()]
    }

    fn get_drawables(&self) -> Vec<&dyn Drawable> {
        vec![self]
    }
}

impl Drawable for FourierDrawer {
    fn draw(
        &self,
        window: &mut RenderWindow,
        color: Color,
        _width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<()> {
        self.sample_bucketer
            .draw(window, color, height, offset_x, offset_y)
    }
}

impl SpecType for FourierDrawer {
    fn name() -> String {
        "fourier-drawer".into()
    }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILD.to_description(), BUFFER_SIZE.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<FourierDrawer> {
        let child = CHILD.get(&mut spec, consts)?;
        let buffer_size = BUFFER_SIZE.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(FourierDrawer::new(child, buffer_size.to_ticks(consts)))
    }
}
