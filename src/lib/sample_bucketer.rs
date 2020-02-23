use error::*;

use sfml::graphics::{Color, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape};
use sfml::system::Vector2f;

const PADDING_PERC: f64 = 0.1;

/// Takes samples, and puts them into lower resolution buckets, only storing the
/// minimum and the maximum values
pub struct SampleBucketer {
    bucket_min_max: Vec<(i32, i32)>,
    all_min_max: (i32, i32),
    num_samples: usize,
    last_bucket_index: usize,
}

impl SampleBucketer {
    /// Creates with the number of samples to hold at any time, and the number
    /// of buckets to put them in
    pub fn new(num_samples: usize, num_buckets: usize) -> SampleBucketer {
        SampleBucketer {
            bucket_min_max: vec![(0, 0); num_buckets],
            all_min_max: (0, 0),
            num_samples,
            last_bucket_index: 0,
        }
    }

    /// Adds a sample to the bucketer
    pub fn add_sample(&mut self, sample: i32, index: usize) {
        let bucket_index = ((index % self.num_samples) as f64
            * (self.bucket_min_max.len() as f64 / self.num_samples as f64))
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

    /// Create an iterator over the mins and maxes of the buckets
    pub fn iter(&self) -> impl Iterator<Item = &(i32, i32)> + '_ {
        let first_iter = self.bucket_min_max.iter().skip(self.last_bucket_index);
        let second_iter = self.bucket_min_max.iter().take(self.last_bucket_index);
        first_iter.chain(second_iter)
    }

    /// Draw the samples to a window
    pub fn draw(
        &self,
        window: &mut RenderWindow,
        color: Color,
        height: u32,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<()> {
        let (all_min, all_max) = self.all_min_max;
        let scale_to_window = |x: i32| {
            let scaled = (i64::from(x) - i64::from(all_min)) as f64
                / (i64::from(all_max) - i64::from(all_min)) as f64;
            scaled * f64::from(height) * (1.0 - 2.0 * PADDING_PERC)
                + f64::from(height) * PADDING_PERC
        };

        for (i, (range_min, range_max)) in self.iter().enumerate() {
            let window_min = scale_to_window(*range_min);
            let window_max = scale_to_window(*range_max);
            let range_height = (window_max - window_min).max(1.0);
            let mut shape = RectangleShape::with_size(Vector2f::new(1.0, range_height as f32));
            shape.set_fill_color(&color);
            window.draw_with_renderstates(&shape, {
                let mut state = RenderStates::default();
                state.transform.translate(
                    (offset_x + i as u32) as f32,
                    offset_y as f32 + window_min as f32,
                );
                state
            });
        }

        Ok(())
    }
}
