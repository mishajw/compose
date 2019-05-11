use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use core::Time;
use error::*;
use gui::Drawable;
use SampleBucketer;

use std::sync::Mutex;

use sfml::graphics::{Color, RenderWindow};

field_decl!(CHILD, Box<Player>, "Wave of this child is drawn");
field_decl!(DISPLAY_TIME, Time, "How much time is displayed on screen");

/// Visualize the sound wave of a player
pub struct WaveDrawer {
    child: Box<Player>,
    display_ticks: usize,
    /// Initialized once `window_width` is filled
    sample_bucketer: Option<SampleBucketer>,
    /// Filled in by the drawing thread
    window_width: Mutex<Option<u32>>,
}

impl WaveDrawer {
    fn new(
        child: Box<Player>,
        display_time: Time,
        consts: &Consts,
    ) -> WaveDrawer
    {
        let display_ticks = display_time.to_ticks(consts);
        WaveDrawer {
            child,
            display_ticks,
            sample_bucketer: None,
            window_width: Mutex::new(None),
        }
    }
}

impl Player for WaveDrawer {
    fn play(&mut self, state: &State) -> Playable {
        let played = self.child.play(state);
        if let Some(ref mut sample_bucketer) = self.sample_bucketer {
            sample_bucketer.add_sample(played.get_value(), state.tick());
        } else {
            // If the sample bucketer has not been initialized, try and
            // initialize it using the window width from the draw thread
            if let Some(window_width) = *self.window_width.lock().unwrap() {
                self.sample_bucketer = Some(SampleBucketer::new(
                    self.display_ticks,
                    window_width as usize,
                ));
            }
        }
        played
    }
}

impl Tree for WaveDrawer {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children(&self) -> Vec<&Tree> { vec![self.child.to_tree()] }

    fn get_drawables(&self) -> Vec<&Drawable> { vec![self] }
}

impl Drawable for WaveDrawer {
    fn draw(
        &self,
        window: &mut RenderWindow,
        color: Color,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<()>
    {
        match &self.sample_bucketer {
            Some(sample_bucketer) => {
                sample_bucketer.draw(window, color, height, offset_x, offset_y)
            }
            None => {
                *self.window_width.lock().unwrap() = Some(width);
                return Ok(());
            }
        }
    }
}

impl SpecType for WaveDrawer {
    fn name() -> String { "wave-drawer".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILD.to_description(), DISPLAY_TIME.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let child = CHILD.get(&mut spec, consts)?;
        let display_time = DISPLAY_TIME.get(&mut spec, consts)?;
        Ok(WaveDrawer::new(child, display_time, consts))
    }
}
