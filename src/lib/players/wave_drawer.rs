use core::spec::create::create_player;
use core::spec::create::FromSpec;
use core::spec::Spec;
use core::spec::Value;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use core::Time;
use error::*;
use gui::Drawable;

use std::sync::Mutex;

use sfml::graphics::{
    RectangleShape, RenderStates, RenderTarget, RenderWindow,
};
use sfml::system::Vector2f;

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
    ) -> Box<WaveDrawer>
    {
        let display_ticks = display_time.to_ticks(consts);
        Box::new(WaveDrawer {
            child,
            display_ticks,
            sample_bucketer: None,
            window_width: Mutex::new(None),
        })
    }
}

impl Player for WaveDrawer {
    fn play(&mut self, state: &State) -> Playable {
        let played = self.child.play(state);
        if let Some(ref mut sample_bucketer) = self.sample_bucketer {
            sample_bucketer.add_sample(played.get_value(), state.tick);
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

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.child.to_tree()]
    }

    fn get_drawables<'a>(&'a self) -> Vec<&'a Drawable> { vec![self] }
}

impl Drawable for WaveDrawer {
    fn draw(
        &self,
        window: &mut RenderWindow,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<()>
    {
        if self.sample_bucketer.is_none() {}
        let sample_bucketer = match &self.sample_bucketer {
            Some(sample_bucketer) => sample_bucketer,
            None => {
                *self.window_width.lock().unwrap() = Some(width);
                return Ok(());
            }
        };

        let scale_to_window = |x: i32| {
            (i64::from(x) - i64::from(sample_bucketer.all_min_max.0)) as f32
                / (i64::from(sample_bucketer.all_min_max.1)
                    - i64::from(sample_bucketer.all_min_max.0))
                    as f32
                * height as f32
        };

        for (i, (range_min, range_max)) in sample_bucketer.iter().enumerate() {
            let window_min = scale_to_window(*range_min);
            let window_max = scale_to_window(*range_max);
            let range_height = (window_max - window_min).max(1.0);
            window.draw_with_renderstates(
                &RectangleShape::with_size(Vector2f::new(1.0, range_height)),
                {
                    let mut state = RenderStates::default();
                    state.transform.translate(
                        (offset_x + i as u32) as f32,
                        offset_y as f32 + window_min,
                    );
                    state
                },
            );
        }

        Ok(())
    }
}

impl FromSpec<Box<Player>> for WaveDrawer {
    fn name() -> &'static str { "wave-drawer" }

    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.into_type()?;
        let child = create_player(&mut spec.consume("child")?, consts)?;
        let display_time =
            Time::from_spec(spec.consume("display-time")?, consts)?;
        Ok(WaveDrawer::new(child, display_time, consts))
    }
}

/// Takes samples, and puts them into lower resolution buckets, only storing the
/// minimum and the maximum values
struct SampleBucketer {
    bucket_min_max: Vec<(i32, i32)>,
    all_min_max: (i32, i32),
    num_samples: usize,
    last_bucket_index: usize,
}

impl SampleBucketer {
    fn new(num_samples: usize, num_buckets: usize) -> SampleBucketer {
        SampleBucketer {
            bucket_min_max: vec![(0, 0); num_buckets],
            all_min_max: (0, 0),
            num_samples,
            last_bucket_index: 0,
        }
    }

    fn add_sample(&mut self, sample: i32, index: usize) {
        let bucket_index = ((index % self.num_samples) as f32
            * (self.bucket_min_max.len() as f32 / self.num_samples as f32))
            as usize;
        debug_assert!(bucket_index < self.bucket_min_max.len());
        let bucket = &mut self.bucket_min_max[bucket_index];
        if bucket_index != self.last_bucket_index {
            // If we've just moved into the bucket, overwrite the current values
            *bucket = (sample, sample);
        } else {
            // If we're continuing to write in the same bucket, change the
            // min/max
            bucket.0 = bucket.0.min(sample);
            bucket.1 = bucket.1.max(sample);
        }
        // Update the overall min/max
        self.all_min_max.0 = self.all_min_max.0.min(sample);
        self.all_min_max.1 = self.all_min_max.1.max(sample);
        self.last_bucket_index = bucket_index;
    }

    fn iter(&self) -> impl Iterator<Item = &(i32, i32)> + '_ {
        let first_iter =
            self.bucket_min_max.iter().skip(self.last_bucket_index);
        let second_iter =
            self.bucket_min_max.iter().take(self.last_bucket_index);
        first_iter.chain(second_iter)
    }
}
